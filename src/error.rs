use crate::Permission;

#[derive(Debug)]
pub enum Error {
    PermissionDenied(String, Permission),
    Var(std::env::VarError),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PermissionDenied(name, permission) => {
                write!(
                    f,
                    "Permission Denied: The permission is insufficient to carry out the operation on the variable. variable: {}, permission: {}",
                    name, permission
                )
            }
            Error::Var(e) => e.fmt(f),
        }
    }
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl From<std::env::VarError> for Error {
    fn from(value: std::env::VarError) -> Self {
        Self::Var(value)
    }
}
