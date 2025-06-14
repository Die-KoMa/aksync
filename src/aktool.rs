// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::{Result, anyhow};
use reqwest::{
    Client, Response,
    header::{HeaderMap, HeaderValue},
};

use crate::{
    AKSYNC_USER_AGENT,
    model::{Event, EventId, aktool},
};

pub struct AKToolApi {
    client: Client,
    iri: String,
}

enum Endpoint {
    AK,
    Category,
    Owner,
}

impl Endpoint {
    fn iri(&self, iri: String) -> String {
        format!("{iri}/{self}/")
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::AK => write!(f, "ak"),
            Endpoint::Category => write!(f, "akcategory"),
            Endpoint::Owner => write!(f, "akowner"),
        }
    }
}

impl AKToolApi {
    pub fn new(iri: String) -> Result<Self> {
        let headers = HeaderMap::from_iter([(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        )]);
        let client = Client::builder()
            .user_agent(AKSYNC_USER_AGENT)
            .default_headers(headers)
            .build()?;

        Ok(Self { client, iri })
    }

    async fn get(&self, endpoint: Endpoint) -> reqwest::Result<Response> {
        self.client.get(endpoint.iri(self.iri.clone())).send().await
    }

    pub async fn events(&self) -> Result<HashMap<EventId, Event>> {
        let categories = self
            .get(Endpoint::Category)
            .await?
            .json::<Vec<aktool::Category>>()
            .await?;
        let owners = self
            .get(Endpoint::Owner)
            .await?
            .json::<Vec<aktool::Owner>>()
            .await?;
        let aks = self
            .get(Endpoint::AK)
            .await?
            .json::<Vec<aktool::AK>>()
            .await?;

        let mut events = HashSet::new();
        let mut categories_by_event = HashMap::<_, Vec<_>>::new();
        for category in categories {
            let event = category.event;
            events.insert(event);
            categories_by_event
                .entry(event)
                .and_modify(|categories| categories.push(category.clone()))
                .or_insert_with(|| vec![category]);
        }

        let mut owners_by_event = HashMap::<_, Vec<_>>::new();
        for owner in owners {
            let event = owner.event;
            events.insert(event);
            owners_by_event
                .entry(event)
                .and_modify(|owners| owners.push(owner.clone()))
                .or_insert_with(|| vec![owner]);
        }

        let mut aks_by_event = HashMap::<_, Vec<_>>::new();
        for ak in aks {
            let event = ak.event;
            events.insert(event);
            aks_by_event
                .entry(event)
                .and_modify(|aks| aks.push(ak.clone()))
                .or_insert_with(|| vec![ak]);
        }

        events
            .into_iter()
            .map(|id| {
                let categories = categories_by_event
                    .get(&id)
                    .ok_or(anyhow!("unknown event {id:?}"))?
                    .iter()
                    .cloned();
                let owners = owners_by_event
                    .get(&id)
                    .ok_or(anyhow!("unknown event {id:?}"))?
                    .iter()
                    .cloned();
                let mut event = Event::new(categories, owners);
                for ak in aks_by_event
                    .get(&id)
                    .ok_or(anyhow!("unknown event {id:?}"))?
                {
                    event.add_ak(ak.clone())?;
                }

                Ok((id, event))
            })
            .collect()
    }
}
