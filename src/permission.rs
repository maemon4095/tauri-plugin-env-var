#[derive(Debug, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    None,
    Read,
    Write,
    All,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Permission::None => "none",
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::All => "all",
        };

        f.write_str(str)
    }
}

impl Permission {
    pub fn is_none(&self) -> bool {
        match self {
            Permission::None => true,
            _ => false,
        }
    }

    pub fn is_read(&self) -> bool {
        match self {
            Permission::Read => true,
            _ => false,
        }
    }

    pub fn is_write(&self) -> bool {
        match self {
            Permission::Write => true,
            _ => false,
        }
    }

    pub fn is_all(&self) -> bool {
        match self {
            Permission::All => true,
            _ => false,
        }
    }

    pub fn can_read(&self) -> bool {
        use Permission::*;
        match self {
            All | Read => true,
            _ => false,
        }
    }

    pub fn can_write(&self) -> bool {
        use Permission::*;
        match self {
            Write | All => true,
            _ => false,
        }
    }
}

impl Permission {
    pub(crate) fn intersect(self, rhs: Permission) -> Permission {
        use Permission::*;
        match self {
            None => None,
            Read => match rhs {
                None => None,
                _ => Read,
            },
            Write => match rhs {
                None => None,
                Read => Read,
                _ => Write,
            },
            All => rhs,
        }
    }

    pub(crate) fn union(self, rhs: Permission) -> Permission {
        use Permission::*;
        match self {
            None => rhs,
            Read => match rhs {
                None | Read => Read,
                Write | All => All,
            },
            Write => match rhs {
                None | Write => Write,
                Read | All => All,
            },
            All => All,
        }
    }
}
