mod env_var_scope;
mod error;
mod permission;

use error::Error;
use regex::Regex;
use std::collections::HashMap;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub use env_var_scope::EnvVarScope;
pub use permission::Permission;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    scope: HashMap<String, Permission>,
}

#[tauri::command]
fn get(scope: tauri::State<EnvVarScope>, name: &str) -> Result<String, Error> {
    let permission = scope.get_permission(name);
    if permission.can_read() {
        Ok(std::env::var(name)?)
    } else {
        Err(Error::PermissionDenied(name.to_string(), permission))
    }
}

#[tauri::command]
fn set(scope: tauri::State<EnvVarScope>, name: &str) -> Result<String, Error> {
    let permission = scope.get_permission(name);
    if permission.can_write() {
        Ok(std::env::var(name)?)
    } else {
        Err(Error::PermissionDenied(name.to_string(), permission))
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::new("env-var")
        .setup_with_config(|_, config: Config| {
            let scope = EnvVarScope::new();
            for (raw_pat, p) in config.scope {
                let pat = Regex::new(&raw_pat)?;
                scope.allow(pat, p);
            }
            Ok(())
        })
        .invoke_handler(generate_handler![get, set])
        .build()
}
