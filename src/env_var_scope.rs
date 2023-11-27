use std::sync::{RwLock, RwLockReadGuard};

use glob::Pattern;

use crate::Permission;

#[derive(Debug)]
pub struct EnvVarScope {
    allowed_patterns: RwLock<Vec<(Pattern, Permission)>>,
    forbidden_patterns: RwLock<Vec<(Pattern, Permission)>>,
}

impl EnvVarScope {
    pub(crate) fn new() -> Self {
        Self {
            allowed_patterns: Default::default(),
            forbidden_patterns: Default::default(),
        }
    }

    pub fn allowed_patterns(&self) -> RwLockReadGuard<'_, Vec<(Pattern, Permission)>> {
        self.allowed_patterns.read().unwrap()
    }
    pub fn forbidden_patterns(&self) -> RwLockReadGuard<'_, Vec<(Pattern, Permission)>> {
        self.forbidden_patterns.read().unwrap()
    }

    pub fn allow(&self, pattern: Pattern, permission: Permission) {
        let mut pats = self.allowed_patterns.write().unwrap();
        pats.push((pattern, permission));
    }

    pub fn forbid(&self, pattern: Pattern, permission: Permission) {
        let mut pats = self.forbidden_patterns.write().unwrap();
        pats.push((pattern, permission));
    }

    pub fn get_permission(&self, name: &str) -> Permission {
        let allowed = self.allowed_patterns();
        let forbidden = self.forbidden_patterns();

        let mut permission = Permission::None;
        for (pat, p) in allowed.iter() {
            if !pat.matches(name) {
                continue;
            }
            permission = p.allow(permission);
            if permission.is_all() {
                break;
            }
        }

        for (pat, p) in forbidden.iter() {
            if !pat.matches(name) {
                continue;
            }
            permission = p.revoke(permission);
            if permission.is_none() {
                break;
            }
        }

        permission
    }
}
