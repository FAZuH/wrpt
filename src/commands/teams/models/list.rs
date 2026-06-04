use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TeamList {
    pub(crate) id: u32,
    pub(crate) name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn team_list_deserialize() {
        let json = r#"{"Id": 1, "Name": "developers"}"#;
        let team: TeamList = serde_json::from_str(json).unwrap();
        assert_eq!(team.id, 1);
        assert_eq!(team.name, "developers");
    }
}
