// © 2025 Maximilian Marx
// SPDX-FileContributor: Maximilian Marx
//
// SPDX-License-Identifier: EUPL-1.2

pub mod aktool {
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct EventId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct AKId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct CategoryId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct OwnerId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct TypeId(u64);

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
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

    mod test {
        use super::*;
        use test_log::test;

        #[test]
        fn parse_ak() {
            let result = serde_json::from_str::<AK>(
                r#"{"id":1289,"name":"Tarifflucht und Massenentlassung","short_name":"Tarifflucht","description":"An Universitäten sind viele Verwaltungskräfte rechtswidrig befristet und vom Tarifvertrag und der Personalvertretung ausgeschlossen.\r\nGewerkschaften versuchen dagegen vorzugehen, Unis versuchen gegen Gewerkschaften vorzugehen: In Dresden hat die TU allein zum Jahreswechsel 70 SHK/WHK defacto-entlassen.\r\nAuch in Passau kam es zu ähnlichen Vorgängen.\r\nWas kann man dagegen tun? Wie bekommt man die Unileitung dazu, Anwaltspost zu lesen? Und was passiert, wenn man Elon Musk auf Whish bestellt und zum Uni-Kanzler macht?","link":"https://wiki.kif.rocks/wiki/KIF530:Tarifflucht_und_Massenentlassung","protocol_link":"","reso":true,"present":null,"notes":"","interest":-1,"interest_counter":0,"include_in_export":true,"category":64,"track":null,"event":16,"owners":[764],"types":[1,2],"requirements":[],"conflicts":[1242,1250,1271,1277,1280],"prerequisites":[]}"#,
            );

            log::debug!("{result:?}");
            assert!(result.is_ok());
        }
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
