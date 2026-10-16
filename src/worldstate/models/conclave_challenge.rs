use warframe_macros::model;

/// The category of a Conclave challenge
#[derive(
    Debug, PartialEq, PartialOrd, Clone, Eq, Hash, derive_more::Display, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ConclaveChallengeCategory {
    /// Daily challenge
    Day,
    /// Daily root challenge
    DailyRoot,
    /// Weekly challenge
    Week,
    /// Weekly root challenge
    WeekRoot,
    /// Mode affector challenge
    ModeAffector,
    /// An unrecognized category
    #[serde(untagged)]
    Unknown(String),
}

/// A Conclave challenge
#[model(endpoint = "/conclaveChallenges", return_style = Array, timed)]
pub struct ConclaveChallenge {
    pub id: String,
    pub amount: i32,
    pub mode: String,
    pub category: String,
    pub category_key: ConclaveChallengeCategory,
    pub description: String,
    pub title: String,
    pub standing: i32,
    pub daily: bool,
    pub root_challenge: bool,
}

#[cfg(test)]
mod test_conclave_challenge {
    use rstest::rstest;
    use serde_json::from_str;

    use super::ConclaveChallenge;
    use crate::worldstate::Queryable;

    type R = <ConclaveChallenge as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/conclave_challenge.json")]
        #[mode = str]
        conclave_challenge_en: &str,
    ) {
        from_str::<R>(conclave_challenge_en).unwrap();
    }
}
