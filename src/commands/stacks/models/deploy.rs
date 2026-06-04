use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds_option::deserialize as from_ts_option;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::commands::stacks::models::resource_control::ResourceControl;
use crate::commands::stacks::models::StackStatus;
use crate::commands::stacks::models::StackType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EnvVar {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StackDeploySwarmCreatePayload {
    pub(crate) env: Vec<EnvVar>,
    pub(crate) from_app_template: bool,
    pub(crate) name: String,
    pub(crate) stack_file_content: String,
    pub(crate) swarm_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StackDeployStandaloneCreatePayload {
    pub(crate) env: Vec<EnvVar>,
    pub(crate) from_app_template: bool,
    pub(crate) name: String,
    pub(crate) stack_file_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StackDeployUpdatePayload {
    pub(crate) env: Vec<EnvVar>,
    pub(crate) stack_file_content: String,
    pub(crate) pull_image: bool,
    pub(crate) prune: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Stack {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) env: Vec<EnvVar>,
    pub(crate) from_app_template: bool,
    pub(crate) swarm_id: String,
    pub(crate) r#type: StackType,
    pub(crate) status: StackStatus,
    pub(crate) endpoint_id: u32,
    #[serde(deserialize_with = "from_ts")]
    pub(crate) creation_date: DateTime<Utc>,
    pub(crate) created_by: String,
    #[serde(deserialize_with = "from_ts_option")]
    pub(crate) update_date: Option<DateTime<Utc>>,
    pub(crate) updated_by: Option<String>,
    pub(crate) resource_control: Option<ResourceControl>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_deserialize_with_resource_control() {
        let json = r#"{
            "Id": 1,
            "Name": "app",
            "Env": [],
            "FromAppTemplate": false,
            "SwarmId": "",
            "Type": 2,
            "Status": 1,
            "EndpointId": 3,
            "CreationDate": 1700000000,
            "CreatedBy": "admin",
            "UpdateDate": null,
            "UpdatedBy": null,
            "ResourceControl": {
                "Id": 1,
                "ResourceId": "1_app",
                "Type": 6,
                "UserAccesses": [],
                "TeamAccesses": [],
                "Public": true
            }
        }"#;
        let stack: Stack = serde_json::from_str(json).unwrap();
        assert_eq!(stack.id, 1);
        assert_eq!(stack.name, "app");
        assert!(stack.resource_control.is_some());
    }

    #[test]
    fn stack_deserialize_without_resource_control() {
        let json = r#"{
            "Id": 2,
            "Name": "test",
            "Env": [{"name": "KEY", "value": "val"}],
            "FromAppTemplate": false,
            "SwarmId": "swarm-123",
            "Type": 1,
            "Status": 1,
            "EndpointId": 1,
            "CreationDate": 1700000000,
            "CreatedBy": "user",
            "UpdateDate": 1700001000,
            "UpdatedBy": "user",
            "ResourceControl": null
        }"#;
        let stack: Stack = serde_json::from_str(json).unwrap();
        assert_eq!(stack.id, 2);
        assert!(stack.resource_control.is_none());
    }

    #[test]
    fn env_var_serialize() {
        let env = EnvVar {
            name: "KEY".to_string(),
            value: "value".to_string(),
        };
        let json = serde_json::to_string(&env).unwrap();
        assert!(json.contains("\"name\":\"KEY\""));
        assert!(json.contains("\"value\":\"value\""));
    }
}
