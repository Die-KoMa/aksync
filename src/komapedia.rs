// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

const KOMAPEDIA_DOMAINS: &[&str] = &[
    "https://de.komapedia.org",
    "https://komapedia.org",
    "https://www.komapedia.org",
];

const KOMAPEDIA_PAGE_PREFIXES: &[&str] = &["/wiki/", "/index.php?title="];

pub(crate) const AKSYNC_TEMPLATE: &str = "KoMa Externer AK aus aktool";

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
