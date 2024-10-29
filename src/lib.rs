use tauri::{
    Runtime, Manager,
    plugin::{TauriPlugin, Builder},
};
pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Shared;
#[cfg(mobile)]
use mobile::Shared;


/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the shared APIs.
pub trait SharedExt<R: Runtime> {
    fn shared(&self) -> &Shared<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SharedExt<R> for T {
    fn shared(&self) -> &Shared<R> {
        self.state::<Shared<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("shared")
        .invoke_handler(tauri::generate_handler![commands::ping])
        .setup(|app, api| {
            #[cfg(mobile)]
            let shared = mobile::init(app, api)?;
            #[cfg(desktop)]
            let shared = desktop::init(app, api)?;
            app.manage(shared);
            Ok(())
        })
        .build()
}
