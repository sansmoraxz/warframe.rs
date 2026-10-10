use warframe_macros::model;

/// A choice category in the Duviri cycle
#[model]
pub struct DuviriChoice {
    /// The i18n category name
    pub category: String,

    /// The category key
    pub category_key: String,

    /// The available choices (Warframes or Weapons)
    pub choices: Vec<String>,
}

/// The emotional state of the current Duviri cycle
#[model]
#[serde(rename_all = "lowercase")]
pub enum DuviriState {
    /// Joy
    Joy,
    /// Anger
    Anger,
    /// Envy
    Envy,
    /// Sorrow
    Sorrow,
    /// Fear
    Fear,
}

/// The current Duviri cycle state
#[model(endpoint = "/duviriCycle", return_style = Object, timed)]
pub struct DuviriCycle {
    /// Unique identifier for the cycle
    pub id: String,

    /// The current emotional state
    pub state: DuviriState,

    /// The available choices for this cycle
    pub choices: Vec<DuviriChoice>,
}

#[cfg(test)]
mod test_duviri_cycle {
    use rstest::rstest;
    use serde_json::from_str;

    use super::DuviriCycle;
    use crate::worldstate::Queryable;

    type R = <DuviriCycle as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/duviri_cycle.json")]
        #[mode = str]
        duviri_cycle_en: &str,
    ) {
        from_str::<R>(duviri_cycle_en).unwrap();
    }
}
