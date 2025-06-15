// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

pub mod aktool {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    pub(crate) const DEFAULT_SLOT_IN_HOURS: f64 = 1.0;

    const TYPE_KOMA: TypeId = TypeId(2);
    pub(crate) const EVENT_KOMA92: EventId = EventId(16);
    const CATEGORY_WORK: CategoryId = CategoryId(64);
    const CATEGORY_META: CategoryId = CategoryId(65);
    const CATEGORY_CULTURE: CategoryId = CategoryId(66);
    const CATEGORY_FRAMING: CategoryId = CategoryId(67);

    const CATEGORIES_EXCHANGE: &[CategoryId] = &[CATEGORY_CULTURE, CATEGORY_META];
    const CATEGORIES_INPUT: &[CategoryId] = &[CATEGORY_CULTURE, CATEGORY_WORK];
    const CATEGORIES_OUTPUT: &[CategoryId] = &[CATEGORY_WORK, CATEGORY_META];
    const CATEGORIES_TALK: &[CategoryId] = &[CATEGORY_FRAMING];
    const CATEGORIES_FUN: &[CategoryId] = &[CATEGORY_FRAMING, CATEGORY_CULTURE];

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct EventId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct AKId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct CategoryId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct OwnerId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct TypeId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct RequirementId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct SlotId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
    #[serde(transparent)]
    pub struct RoomId(u64);

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

    impl AK {
        pub fn is_exchange(&self) -> bool {
            CATEGORIES_EXCHANGE.contains(&self.category)
        }

        pub fn is_input(&self) -> bool {
            CATEGORIES_INPUT.contains(&self.category)
        }

        pub fn is_output(&self) -> bool {
            CATEGORIES_OUTPUT.contains(&self.category)
        }

        pub fn is_reso(&self) -> bool {
            self.reso
        }

        pub fn is_talk(&self) -> bool {
            CATEGORIES_TALK.contains(&self.category)
        }

        pub fn is_fun(&self) -> bool {
            CATEGORIES_FUN.contains(&self.category)
        }

        pub fn is_koma(&self) -> bool {
            self.types.contains(&TYPE_KOMA)
        }
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Category {
        pub(crate) id: CategoryId,
        pub(crate) name: String,
        pub(crate) color: String,
        pub(crate) description: String,
        pub(crate) present_by_default: bool,
        pub(crate) event: EventId,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Owner {
        pub(crate) id: OwnerId,
        pub(crate) name: String,
        pub(crate) slug: String,
        pub(crate) institution: String,
        pub(crate) link: String,
        pub(crate) event: EventId,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    pub struct Slot {
        pub(crate) id: SlotId,
        pub(crate) start: Option<String>,
        #[serde(with = "f64_as_string")]
        pub(crate) duration: f64,
        pub(crate) fixed: bool,
        pub(crate) updated: String,
        pub(crate) ak: AKId,
        pub(crate) room: Option<RoomId>,
        pub(crate) event: EventId,
    }

    mod f64_as_string {
        use core::f64;
        use std::fmt;

        use serde::{Deserializer, Serializer, de::Visitor};

        pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct F64Visitor {}
            impl Visitor<'_> for F64Visitor {
                type Value = f64;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "a string containing an f64")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    value.parse().map_err(E::custom)
                }
            }

            deserializer.deserialize_str(F64Visitor {})
        }

        pub(super) fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&value.to_string())
        }
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

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
};

pub use aktool::{AKId, CategoryId, EventId, OwnerId};
use anyhow::{Result, anyhow};
use itertools::Itertools;

use crate::{
    komapedia::{
        AKSYNC_AK_TEMPLATE, AKSYNC_GENERATED_TEMPLATE, KOMAPEDIA_AK_PREFIX, escape, format_link,
    },
    wikipage,
};

#[derive(Debug)]
pub struct Event {
    owners: HashMap<OwnerId, Owner>,
    categories: HashMap<CategoryId, Category>,
    aks: HashMap<AKId, AK>,
}

impl Event {
    pub(crate) fn new<C, O>(categories: C, owners: O) -> Self
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

    pub(crate) fn aks(&self) -> impl Iterator<Item = (&AKId, &AK)> {
        self.aks
            .iter()
            .sorted_by(|&(id, _), &(other, _)| Ord::cmp(id, other))
    }

    pub(crate) fn add_ak(&mut self, ak: aktool::AK) -> Result<&mut Self> {
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
                    .ok_or_else(|| anyhow!("unknown owner {owner_id:?}"))
                    .cloned()
            })
            .collect::<Result<_>>()?;

        let id = ak.id;
        let ak = AK::from_aktool(ak, category.clone(), owners);
        let _ = self.aks.insert(id, ak);
        Ok(self)
    }

    pub(crate) fn add_slot(&mut self, slot: &aktool::Slot) -> Result<&mut Self> {
        self.aks
            .get_mut(&slot.ak)
            .ok_or(anyhow!("unknown AK {:?}", slot.ak))?
            .duration += slot.duration * aktool::DEFAULT_SLOT_IN_HOURS;

        Ok(self)
    }
}

#[derive(Debug, Clone)]
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

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.description)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Owner {
    name: String,
    institution: Option<String>,
    link: Option<String>,
}

impl From<aktool::Owner> for Owner {
    fn from(value: aktool::Owner) -> Self {
        Self {
            name: value.name,
            institution: (!value.institution.is_empty()).then_some(value.institution),
            link: (!value.link.is_empty()).then_some(value.link),
        }
    }
}

impl Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match &self.institution {
            None => escape(&self.name),
            Some(institution) => format!("{} ({})", escape(&self.name), escape(institution)),
        };

        write!(f, "{}", format_link(label, &self.link))
    }
}

#[derive(Debug, Clone)]
pub struct AK {
    name: String,
    short_name: String,
    description: String,
    result: String,
    owners: HashSet<Owner>,
    #[allow(unused)]
    category: Category,
    duration: f64,

    exchange: bool,
    input: bool,
    output: bool,
    reso: bool,
    talk: bool,
    fun: bool,

    koma: bool,
}

impl AK {
    pub(crate) fn from_aktool(ak: aktool::AK, category: Category, owners: HashSet<Owner>) -> Self {
        fn with_prefix(name: String) -> String {
            if name.starts_with(KOMAPEDIA_AK_PREFIX) {
                name
            } else {
                format!("{KOMAPEDIA_AK_PREFIX}{name}")
            }
        }

        let exchange = ak.is_exchange();
        let input = ak.is_input();
        let output = ak.is_output();
        let reso = ak.is_reso();
        let talk = ak.is_talk();
        let fun = ak.is_fun();
        let koma = ak.is_koma();

        Self {
            name: with_prefix(ak.name),
            short_name: with_prefix(ak.short_name),
            description: ak.description,
            result: ak.protocol_link,
            category,
            owners,
            duration: 0.0,

            exchange,
            input,
            output,
            reso,
            talk,
            fun,

            koma,
        }
    }

    pub(crate) fn is_koma(&self) -> bool {
        self.koma
    }

    pub(crate) fn wikipage(&self, event: EventId) -> Result<String> {
        Ok(format!("{}/{}", wikipage(event)?, self.short_name))
    }

    pub(crate) fn wikitext(&self) -> String {
        format!("{{{{{AKSYNC_GENERATED_TEMPLATE}}}}}\n{self}")
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    fn format_type(&self) -> String {
        Itertools::intersperse(
            [
                (self.exchange, "Austausch"),
                (self.input, "Input"),
                (self.output, "Output"),
                (self.reso, "Reso"),
                (self.talk, "Vortrag"),
                (self.fun, "Spaß"),
            ]
            .into_iter()
            .filter_map(|(predicate, value)| if predicate { Some(value) } else { None }),
            ",",
        )
        .collect()
    }

    fn format_owners(&self) -> String {
        Itertools::intersperse(
            self.owners.iter().map(|owner| owner.to_string()).sorted(),
            ", ".to_string(),
        )
        .collect()
    }
}

impl Display for AK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! attribute {
            ($key:expr=>$value:expr) => {
                writeln!(f, "|{}={}", $key, $value)?;
            };
        }

        writeln!(f, "{{{{{AKSYNC_AK_TEMPLATE}")?;

        attribute!("Name" => escape(&self.name));

        let types = self.format_type();
        if !types.is_empty() {
            attribute!("Typ" => self.format_type());
        }

        let owners = self.format_owners();
        if !owners.is_empty() {
            attribute!("Leitung" => self.format_owners());
        }

        attribute!("Dauer" => self.duration);

        if !self.description.is_empty() {
            attribute!("Beschreibung" => escape(&self.description));
        }

        if !self.result.is_empty() {
            attribute!("Ergebnis" => escape(&self.result));
        }

        writeln!(f, "}}}}")
    }
}
