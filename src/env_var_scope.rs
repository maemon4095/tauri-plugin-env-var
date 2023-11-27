use std::sync::{RwLock, RwLockReadGuard};

use regex::Regex;

use crate::Permission;

#[derive(Debug)]
pub struct EnvVarScope {
    allowed_patterns: RwLock<Vec<(Regex, Permission)>>,
    forbidden_patterns: RwLock<Vec<(Regex, Permission)>>,
}

impl EnvVarScope {
    pub(crate) fn new() -> Self {
        Self {
            allowed_patterns: Default::default(),
            forbidden_patterns: Default::default(),
        }
    }

    pub fn allowed_patterns(&self) -> RwLockReadGuard<'_, Vec<(Regex, Permission)>> {
        self.allowed_patterns.read().unwrap()
    }
    pub fn forbidden_patterns(&self) -> RwLockReadGuard<'_, Vec<(Regex, Permission)>> {
        self.forbidden_patterns.read().unwrap()
    }

    pub fn allow(&self, pattern: Regex, permission: Permission) {
        let mut pats = self.allowed_patterns.write().unwrap();
        pats.push((pattern, permission));
    }

    pub fn forbid(&self, pattern: Regex, permission: Permission) {
        let mut pats = self.forbidden_patterns.write().unwrap();
        pats.push((pattern, permission));
    }

    pub fn get_permission(&self, name: &str) -> Permission {
        let allowed = self.allowed_patterns();
        let forbidden = self.forbidden_patterns();

        let mut permission = Permission::None;
        for (pat, p) in allowed.iter() {
            if !pat.is_match(name) {
                continue;
            }
            permission = p.union(permission);
            if permission.is_all() {
                break;
            }
        }

        for (pat, p) in forbidden.iter() {
            if !pat.is_match(name) {
                continue;
            }
            permission = p.intersect(permission);
            if permission.is_none() {
                break;
            }
        }

        permission
    }
}
