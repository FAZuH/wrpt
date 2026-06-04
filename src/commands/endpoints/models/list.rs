use serde::Deserialize;
use serde::Serialize;

use crate::commands::endpoints::models::EndpointStatus;
use crate::commands::endpoints::models::EndpointType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct EndpointList {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) r#type: EndpointType,
    pub(crate) status: EndpointStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_list_deserialize() {
        let json = r#"{"Id": 1, "Name": "local", "Type": 1, "Status": 1}"#;
        let endpoint: EndpointList = serde_json::from_str(json).unwrap();
        assert_eq!(endpoint.id, 1);
        assert_eq!(endpoint.name, "local");
    }
}
