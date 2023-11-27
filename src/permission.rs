#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Copy)]
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
    /// revoke permissions specified by rhs from self.
    pub fn revoke(self, rhs: Permission) -> Permission {
        use Permission::*;
        match (self, rhs) {
            (None, _) => None,
            (Read, Read | All) => None,
            (Read, _) => Read,
            (Write, Write | All) => None,
            (Write, _) => Write,
            (All, All) => None,
            (All, Read) => Write,
            (All, Write) => Read,
            (All, None) => All,
        }
    }

    /// allow permissions specified by rhs to self.
    pub fn allow(self, rhs: Permission) -> Permission {
        use Permission::*;
        match (self, rhs) {
            (None, rhs @ _) => rhs,
            (Read, None | Read) => Read,
            (Read, Write | All) => All,
            (Write, None | Write) => Write,
            (Write, Read | All) => All,
            (All, _) => All,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Permission;

    #[test]
    fn none_must_forbid_read_write() {
        let p = Permission::None;
        assert!(!p.can_read());
        assert!(!p.can_write());
    }

    #[test]
    fn read_must_allow_read_and_forbid_write() {
        let p = Permission::Read;
        assert!(p.can_read());
        assert!(!p.can_write());
    }

    #[test]
    fn write_must_allow_write_and_forbid_read() {
        let p = Permission::Write;
        assert!(!p.can_read());
        assert!(p.can_write());
    }

    #[test]
    fn all_must_allow_write_and_read() {
        let p = Permission::All;
        assert!(p.can_read());
        assert!(p.can_write());
    }
}
