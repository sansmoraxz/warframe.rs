use warframe_macros::model;

/// A Conclave challenge
#[model(endpoint = "/conclaveChallenges", return_style = Array, timed)]
pub struct ConclaveChallenge {
    pub id: String,
    pub amount: i32,
    pub mode: String,
    pub category: String,
    pub category_key: String,
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
