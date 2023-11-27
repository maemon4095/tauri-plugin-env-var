mod env_var_scope;
mod error;
mod permission;

use error::Error;
use glob::Pattern;
use std::collections::HashMap;
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
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
fn set(scope: tauri::State<EnvVarScope>, name: &str, value: &str) -> Result<(), Error> {
    let permission = scope.get_permission(name);
    if permission.can_write() {
        std::env::set_var(name, value);
        Ok(())
    } else {
        Err(Error::PermissionDenied(name.to_string(), permission))
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::new("env-var")
        .setup_with_config(|app_handle, config: Config| {
            let scope = EnvVarScope::new();
            for (raw_pat, p) in config.scope {
                let pat = Pattern::new(&raw_pat)?;
                scope.allow(pat, p);
            }
            app_handle.manage(scope);
            Ok(())
        })
        .invoke_handler(generate_handler![get, set])
        .build()
}
