// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use std::env;

use anyhow::{Result, anyhow};
use mediawiki::Api;
use serde_json::Value;

use crate::model::{Event, EventId, aktool::EVENT_KOMA92};

const KOMAPEDIA_DOMAINS: &[&str] = &[
    "https://de.komapedia.org",
    "https://komapedia.org",
    "https://www.komapedia.org",
];

const KOMAPEDIA_ENDPOINT: &str = "https://de.komapedia.org/api.php";
const KOMAPEDIA_BOT_USERNAME: &str = "AKsync";

const KOMAPEDIA_PAGE_PREFIXES: &[&str] = &["/wiki/", "/index.php?title="];
const KOMAPEDIA_IMPORT_SUBPAGE: &str = "Importiert_aus_aktool";

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
        .find_map(|&(id, page)| (id == event).then(|| format!("{page}/{KOMAPEDIA_IMPORT_SUBPAGE}")))
        .ok_or(anyhow!("unknown event {event:?}"))
}

pub(crate) async fn update_event(id: EventId, event: &Event) -> Result<Value> {
    let mut api = Api::new(KOMAPEDIA_ENDPOINT).await?;
    api.login(KOMAPEDIA_BOT_USERNAME, &env::var("AKSYNC_BOT_PASSWORD")?)
        .await?;
    let token = api.get_edit_token().await?;

    let parameters = api.params_into(&[
        ("action", "edit"),
        ("title", &wikipage(id)?),
        ("text", &event.wikitext()),
        ("summary", AKSYNC_SUMMARY),
        ("bot", "true"),
        ("watchlist", "unwatch"),
        ("token", &token),
    ]);

    Ok(api.post_query_api_json(&parameters).await?)
}
