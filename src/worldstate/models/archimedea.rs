use warframe_macros::model;

use super::faction::Faction;
use crate::worldstate::MissionType;

#[model]
pub struct ArchimedeaModifier {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[model]
pub struct ArchimedeaRisk {
    pub key: String,
    pub name: String,
    pub description: String,
    pub is_hard: bool,
}

#[model]
pub struct ArchimedeaMission {
    pub faction: Faction,
    pub faction_key: Faction,
    pub mission_type: String,
    pub mission_type_key: MissionType,
    pub deviation: ArchimedeaModifier,
    pub risks: Vec<ArchimedeaRisk>,
}

#[model(endpoint = "/archimedeas", return_style = Array, timed)]
pub struct Archimedea {
    pub id: String,
    #[serde(rename = "type")]
    pub archimedea_type: String,
    pub type_key: String,
    pub missions: Vec<ArchimedeaMission>,
    pub personal_modifiers: Vec<ArchimedeaModifier>,
}

#[cfg(test)]
mod test_archimedea {
    use rstest::rstest;
    use serde_json::from_str;

    use super::Archimedea;
    use crate::worldstate::Queryable;

    type R = <Archimedea as Queryable>::Return;

    #[rstest]
    fn test(
        #[files("src/worldstate/models/fixtures/archimedea.json")]
        #[mode = str]
        archimedea_en: &str,
    ) {
        from_str::<R>(archimedea_en).unwrap();
    }
}
