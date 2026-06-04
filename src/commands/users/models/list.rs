use serde::Deserialize;
use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) username: String,
    pub(crate) role: UserRole,
}

#[derive(Debug, Serialize, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub(crate) enum UserRole {
    Administrator = 1,
    User = 2,
    #[serde(other)]
    Unknown = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_deserialize() {
        let json = r#"{"Id": 1, "Username": "admin", "Role": 1}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "admin");
        assert_eq!(user.role, UserRole::Administrator);
    }

    #[test]
    fn user_role_deserialize_unknown() {
        let unknown: UserRole = serde_json::from_str("99").unwrap();
        assert_eq!(unknown, UserRole::Unknown);
    }

    #[test]
    fn user_role_deserialize_known() {
        let admin: UserRole = serde_json::from_str("1").unwrap();
        assert_eq!(admin, UserRole::Administrator);
        let user: UserRole = serde_json::from_str("2").unwrap();
        assert_eq!(user, UserRole::User);
    }
}
