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

pub(crate) async fn update_ak(api: &mut Api, event: EventId, ak: &AK) -> Result<()> {
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
