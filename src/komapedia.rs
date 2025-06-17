// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use std::{env, fs::read_to_string};

use anyhow::{Result, anyhow, bail};
use mediawiki::Api;
use serde_json::Value;

use crate::{
    AKSYNC_USER_AGENT,
    model::{AK, Event, EventId, aktool::EVENT_KOMA92},
};

const KOMAPEDIA_DOMAINS: &[&str] = &[
    "https://de.komapedia.org",
    "https://komapedia.org",
    "https://www.komapedia.org",
];

const KOMAPEDIA_ENDPOINT: &str = "https://de.komapedia.org/api.php";
const KOMAPEDIA_BOT_USERNAME: &str = "AKsync";

const KOMAPEDIA_PAGE_PREFIXES: &[&str] = &["/wiki/", "/index.php?title="];

pub(crate) const KOMAPEDIA_AK_PREFIX: &str = "AK ";
pub(crate) const KOMAPEDIA_EVENTS: &[(EventId, &str)] = &[(EVENT_KOMA92, "KoMa_92")];
pub(crate) const AKSYNC_AK_TEMPLATE: &str = "KoMa Externer AK aus aktool";
pub(crate) const AKSYNC_GENERATED_TEMPLATE: &str = "Seite automatisch erzeugt von aksync";
pub(crate) const AKSYNC_SUMMARY: &str = "AK-Liste aus aktool importiert";
pub(crate) const AKSYNC_DELETE_SUMMARY: &str = "AK wurde in aktool gelöscht";

fn is_pagelink(target: &str) -> Option<String> {
    for &domain in KOMAPEDIA_DOMAINS {
        if let Some(relative) = target.strip_prefix(domain) {
            for &prefix in KOMAPEDIA_PAGE_PREFIXES {
                if let Some(link) = relative.strip_prefix(prefix) {
                    let index = link.find('&').unwrap_or(link.len());

                    return Some(link[..index].to_string());
                }
            }
        }
    }

    None
}

pub(crate) fn is_subpage(target: &str, event: EventId, ak: &AK) -> Option<String> {
    let prefix = format!("{}/", ak.wikipage(event).ok()?).replace(' ', "_");

    is_pagelink(target).and_then(|t| t.strip_prefix(&prefix).map(|t| t.to_string()))
}

pub(crate) fn format_link(label: String, target: &Option<String>) -> String {
    match target {
        Some(link) => {
            if let Some(link) = is_pagelink(link) {
                format!("[[{link}|{label}]]")
            } else {
                format!("[{link} {label}]")
            }
        }
        None => label,
    }
}

pub(crate) fn escape(text: &str) -> String {
    let result = text.to_string();

    let result = result.replace("{{", "{&ZeroWidthSpace;{");
    let result = result.replace("}}", "}&ZeroWidthSpace;}");
    let result = result.replace('|', "{{!}}");
    result.replace('=', "{{=}}")
}

pub(crate) fn wikipage(event: EventId) -> Result<String> {
    KOMAPEDIA_EVENTS
        .iter()
        .find_map(|&(id, page)| (id == event).then(|| page.to_string()))
        .ok_or(anyhow!("unknown event {event:?}"))
}

async fn delete_old_pages(api: &mut Api, event: EventId, ak: &AK) -> Result<()> {
    let parameters = api.params_into(&[
        ("action", "ask"),
        ("query", &ak.semantic_query(event)),
        ("formatversion", "2"),
    ]);

    log::debug!("API request:\n{parameters:#?}");

    let result = api.get_query_api_json(&parameters).await?;
    log::debug!("{result:#?}");
    if let Value::Object(map) = result {
        if let Some(Value::Object(map)) = map.get("query") {
            if let Some(Value::Object(map)) = map.get("results") {
                for page in map.keys() {
                    let page = page.replace(' ', "_");
                    if ak.wikipage(event)? != page {
                        log::debug!("{page:?}, {:?}", ak.wikipage(event)?);
                        let token = api.get_edit_token().await?;
                        let parameters = api.params_into(&[
                            ("action", "delete"),
                            ("title", &page),
                            ("reason", AKSYNC_DELETE_SUMMARY),
                            ("bot", "true"),
                            ("token", &token),
                        ]);
                        log::debug!("API request:\n{parameters:#?}");
                        log::info!("Deleting obsolete page {page}");
                        let result = api.post_query_api_json(&parameters).await?;

                        if let Value::Object(map) = result {
                            if let Some(Value::Object(err)) = map.get("error") {
                                bail!(
                                    "got error {}: {}",
                                    err.get("code")
                                        .unwrap_or(&Value::String("<unknown>".to_string()))
                                        .to_string(),
                                    err.get("info")
                                        .unwrap_or(&Value::String(Default::default()))
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub(crate) async fn update_ak(api: &mut Api, event: EventId, ak: &AK) -> Result<()> {
    delete_old_pages(api, event, ak).await?;

    let token = api.get_edit_token().await?;
    let parameters = api.params_into(&[
        ("action", "edit"),
        ("title", &ak.wikipage(event)?),
        ("text", &ak.wikitext()),
        ("summary", AKSYNC_SUMMARY),
        ("bot", "true"),
        ("watchlist", "unwatch"),
        ("token", &token),
    ]);

    log::debug!("API request:\n{parameters:#?}");

    let result = api.post_query_api_json(&parameters).await?;

    if let Value::Object(map) = result {
        if let Some(Value::Object(err)) = map.get("error") {
            bail!(
                "got error {}: {}",
                err.get("code")
                    .unwrap_or(&Value::String("<unknown>".to_string()))
                    .to_string(),
                err.get("info")
                    .unwrap_or(&Value::String(Default::default()))
            )
        }
    }

    Ok(())
}

pub(crate) async fn update_event(id: EventId, event: &Event) -> Result<()> {
    let mut api = Api::new(KOMAPEDIA_ENDPOINT).await?;
    api.set_user_agent(AKSYNC_USER_AGENT);
    api.login(KOMAPEDIA_BOT_USERNAME, &bot_password_from_env()?)
        .await?;

    for (_, ak) in event.aks() {
        if ak.is_koma() {
            log::info!("processing {} ({})", ak.name(), ak.wikipage(id)?);
            update_ak(&mut api, id, ak).await?;
        }
    }

    Ok(())
}

fn bot_password_from_env() -> Result<String> {
    match env::var("AKSYNC_BOT_PASSWORD_FILE") {
        Ok(path) => Ok(read_to_string(path)?.trim().to_string()),
        Err(_) => Ok(env::var("AKSYNC_BOT_PASSWORD")?),
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use test_log::test;

    use crate::{
        komapedia::is_subpage,
        model::{
            AK,
            aktool::{self, EVENT_KOMA92},
        },
    };

    #[test]
    fn subpage() {
        let ak =
    AK::from_aktool(serde_json::from_str::<aktool::AK>(
    r#"{"id":1305,"name":"IT-Infrastruktur","short_name":"IT-Infrastruktur","description":"Test","link":"https://wiki.kif.rocks/wiki/KIF530:IT-Infrastruktur","protocol_link":"https://de.komapedia.org/wiki/KoMa_92/AK_IT-Infrastruktur/Ergebnis","reso":false,"present":null,"notes":"","interest":-1,"interest_counter":0,"include_in_export":true,"category":65,"track":null,"event":16,"owners":[],"types":[2],"requirements":[49],"conflicts":[],"prerequisites":[]}"#).unwrap(),
    serde_json::from_str::<aktool::Category>(
    r##"{"id":64,"name":"Inhalt/Arbeit","color":"#487eb0","description":"","present_by_default":false,"event":16}"##,
    ).unwrap().into(), HashSet::new(), );
        assert_eq!(
            is_subpage(
                "https://de.komapedia.org/wiki/KoMa_92/AK_IT-Infrastruktur/Ergebnis",
                EVENT_KOMA92,
                &ak
            ),
            Some("Ergebnis".to_string())
        );
    }
}
