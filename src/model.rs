// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

pub mod aktool {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct EventId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct AKId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct CategoryId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct OwnerId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct TypeId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    #[serde(transparent)]
    pub struct RequirementId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct AK {
        id: AKId,
        name: String,
        short_name: String,
        description: String,
        link: String,
        protocol_link: String,
        reso: bool,
        present: Option<bool>,
        notes: String,
        interest: i64,
        interest_counter: u64,
        include_in_export: bool,
        category: CategoryId,
        track: Option<u64>,
        event: EventId,
        owners: HashSet<OwnerId>,
        types: HashSet<TypeId>,
        requirements: HashSet<RequirementId>,
        conflicts: HashSet<AKId>,
        prerequisites: HashSet<AKId>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Category {
        id: CategoryId,
        name: String,
        color: String,
        description: String,
        present_by_default: bool,
        event: EventId,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
    pub struct Owner {
        id: OwnerId,
        name: String,
        slug: String,
        institution: String,
        link: String,
        event: EventId,
    }

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

pub use aktool::AKId;

#[derive(Debug)]
pub struct AK {}

impl AK {
    fn from_aktool(ak: aktool::AK) -> Self {
        todo!()
    }
}
