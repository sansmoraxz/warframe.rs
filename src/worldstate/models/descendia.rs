use warframe_macros::model;

#[model]
pub struct DescendiaNamedItem {
    pub unique_name: String,
    pub name: String,
}

#[model]
pub struct DescendiaChallenge {
    pub index: i32,
    #[serde(rename = "type")]
    pub challenge_type: String,
    pub type_key: String,
    pub challenge: String,
    pub challenge_key: String,
    pub level: String,
    pub level_unique_name: String,
    pub specs: Vec<DescendiaNamedItem>,
    pub auras: Vec<DescendiaNamedItem>,
}

/// The current Descendia (Circuit of Heroes) rotation
#[model(endpoint = "/descendia", return_style = Object, timed)]
pub struct Descendia {
    pub id: String,
    pub seed: u64,
    pub challenges: Vec<DescendiaChallenge>,
}

#[cfg(test)]
mod test_descendia {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Descendia;
    use crate::worldstate::Queryable;

    type R = <Descendia as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/descendia.json")]
        #[mode = str]
        descendia_en: &str,
    ) {
        from_str::<R>(descendia_en).unwrap();
    }
}
