pub(crate) mod list;

use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum EndpointStatus {
    Up = 1,
    Down = 2,
    #[serde(other)]
    Unknown = 0,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum EndpointType {
    Docker = 1,
    Agent = 2,
    Azure = 3,
    #[serde(other)]
    Unknown = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_status_deserialize() {
        let up: EndpointStatus = serde_json::from_str("1").unwrap();
        assert_eq!(up, EndpointStatus::Up);
        let down: EndpointStatus = serde_json::from_str("2").unwrap();
        assert_eq!(down, EndpointStatus::Down);
        let unknown: EndpointStatus = serde_json::from_str("99").unwrap();
        assert_eq!(unknown, EndpointStatus::Unknown);
    }

    #[test]
    fn endpoint_type_deserialize() {
        let docker: EndpointType = serde_json::from_str("1").unwrap();
        assert_eq!(docker, EndpointType::Docker);
        let agent: EndpointType = serde_json::from_str("2").unwrap();
        assert_eq!(agent, EndpointType::Agent);
        let azure: EndpointType = serde_json::from_str("3").unwrap();
        assert_eq!(azure, EndpointType::Azure);
        let unknown: EndpointType = serde_json::from_str("50").unwrap();
        assert_eq!(unknown, EndpointType::Unknown);
    }
}
