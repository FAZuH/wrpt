pub(crate) mod deploy;
pub(crate) mod list;
pub(crate) mod resource_control;

use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum StackStatus {
    Active = 1,
    Inactive = 2,
    #[serde(other)]
    Unknown = 0,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum StackType {
    Swarm = 1,
    Compose = 2,
    #[serde(other)]
    Unknown = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_status_deserialize_known() {
        let active: StackStatus = serde_json::from_str("1").unwrap();
        assert_eq!(active, StackStatus::Active);
        let inactive: StackStatus = serde_json::from_str("2").unwrap();
        assert_eq!(inactive, StackStatus::Inactive);
    }

    #[test]
    fn stack_status_deserialize_unknown() {
        let unknown: StackStatus = serde_json::from_str("99").unwrap();
        assert_eq!(unknown, StackStatus::Unknown);
    }

    #[test]
    fn stack_type_deserialize_known() {
        let swarm: StackType = serde_json::from_str("1").unwrap();
        assert_eq!(swarm, StackType::Swarm);
        let compose: StackType = serde_json::from_str("2").unwrap();
        assert_eq!(compose, StackType::Compose);
    }

    #[test]
    fn stack_type_deserialize_unknown() {
        let unknown: StackType = serde_json::from_str("99").unwrap();
        assert_eq!(unknown, StackType::Unknown);
    }
}
