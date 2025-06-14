// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

pub mod aktool {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct EventId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct AKId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct CategoryId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct OwnerId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct TypeId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
    #[serde(transparent)]
    pub struct RequirementId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct AK {
        pub(crate) id: AKId,
        pub(crate) name: String,
        pub(crate) short_name: String,
        pub(crate) description: String,
        pub(crate) link: String,
        pub(crate) protocol_link: String,
        pub(crate) reso: bool,
        pub(crate) present: Option<bool>,
        pub(crate) notes: String,
        pub(crate) interest: i64,
        pub(crate) interest_counter: u64,
        pub(crate) include_in_export: bool,
        pub(crate) category: CategoryId,
        pub(crate) track: Option<u64>,
        pub(crate) event: EventId,
        pub(crate) owners: HashSet<OwnerId>,
        pub(crate) types: HashSet<TypeId>,
        pub(crate) requirements: HashSet<RequirementId>,
        pub(crate) conflicts: HashSet<AKId>,
        pub(crate) prerequisites: HashSet<AKId>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Category {
        pub(crate) id: CategoryId,
        pub(crate) name: String,
        pub(crate) color: String,
        pub(crate) description: String,
        pub(crate) present_by_default: bool,
        pub(crate) event: EventId,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Owner {
        pub(crate) id: OwnerId,
        pub(crate) name: String,
        pub(crate) slug: String,
        pub(crate) institution: String,
        pub(crate) link: String,
        pub(crate) event: EventId,
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use test_log::test;

        #[test]
        fn parse_ak() {
            let result = serde_json::from_str::<AK>(
                r#"{"id":1289,"name":"Testwursttesting","short_name":"Testwurst","description":"Wir testen das Verspeisen leckerer Testwürste","link":"https://de.komapedia.org/wiki/KoMa_92/AK_Testwurst","protocol_link":"","reso":true,"present":null,"notes":"","interest":-1,"interest_counter":0,"include_in_export":true,"category":64,"track":null,"event":16,"owners":[1312],"types":[1,2],"requirements":[],"conflicts":[1242,1250,1271,1277,1280],"prerequisites":[]}"#,
            );

            log::debug!("{result:?}");
            assert!(result.is_ok());
        }
    }

    #[test]
    fn parse_category() {
        let result = serde_json::from_str::<Category>(
            r##"{"id":64,"name":"Inhalt/Arbeit","color":"#487eb0","description":"Inhalt- und Arbeits-AKs: Für eher ernsthafte und inhaltliche (HoPo, FS-Arbeit, Mathematik, Informatik...) Arbeitskreise ist diese Kategorie da.","present_by_default":false,"event":16}"##,
        );

        log::debug!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn parse_owner() {
        let result = serde_json::from_str::<Owner>(
            r#"{"id":1312,"name":"mmarx","slug":"mmarx","institution":"TU Dresden","link":"https://de.komapedia.org/wiki/Benutzer:Mmarx","event":16}"#,
        );

        log::debug!("{result:?}");
        assert!(result.is_ok());
    }
}

use std::collections::{HashMap, HashSet};

pub use aktool::{AKId, CategoryId, EventId, OwnerId};
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Event<'event> {
    owners: HashMap<OwnerId, Owner>,
    categories: HashMap<CategoryId, Category>,
    aks: HashMap<AKId, AK<'event>>,
}

impl<'event> Event<'event> {
    fn new<C, O>(categories: C, owners: O) -> Self
    where
        C: IntoIterator<Item = aktool::Category>,
        O: IntoIterator<Item = aktool::Owner>,
    {
        Self {
            categories: HashMap::from_iter(
                categories
                    .into_iter()
                    .map(|category| (category.id, Category::from(category))),
            ),
            owners: HashMap::from_iter(
                owners
                    .into_iter()
                    .map(|owner| (owner.id, Owner::from(owner))),
            ),
            aks: HashMap::new(),
        }
    }

    fn add_ak(&'event mut self, id: AKId, ak: aktool::AK) -> Result<()> {
        let category = self
            .categories
            .get(&ak.category)
            .ok_or_else(|| anyhow!("unknown category {:?}", ak.category))?;
        let owners = ak
            .owners
            .iter()
            .map(|owner_id| {
                self.owners
                    .get(owner_id)
                    .ok_or_else(|| anyhow!("unknown owner {:?}", owner_id))
            })
            .collect::<Result<_>>()?;

        let ak = AK::from_aktool(ak, category, owners);
        let _ = self.aks.insert(id, ak);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Category {
    name: String,
    description: String,
}

impl From<aktool::Category> for Category {
    fn from(value: aktool::Category) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Owner {
    name: String,
    institution: String,
    link: String,
}

impl From<aktool::Owner> for Owner {
    fn from(value: aktool::Owner) -> Self {
        Self {
            name: value.name,
            institution: value.institution,
            link: value.link,
        }
    }
}

#[derive(Debug)]
pub struct AK<'event> {
    name: String,
    description: String,
    owners: HashSet<&'event Owner>,
    category: &'event Category,
}

impl<'event> AK<'event> {
    fn from_aktool(
        ak: aktool::AK,
        category: &'event Category,
        owners: HashSet<&'event Owner>,
    ) -> Self {
        Self {
            name: ak.name,
            description: ak.description,
            category,
            owners,
        }
    }
}
