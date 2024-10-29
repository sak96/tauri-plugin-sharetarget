use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    ipc::Channel,
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_shared);

/// Access to the shared APIs.
pub struct Shared<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Shared<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
}

//pub struct ShareWatcher<R: Runtime>(PluginHandle<R>);
#[derive(serde::Serialize)]
pub struct ShareWatcher {
    pub channel: Channel,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct ShareEvent {
    data: String,
}

#[derive(serde::Deserialize)]
pub struct Config {}

fn _init_share_watcher<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Shared<R>> {
    #[cfg(target_os = "android")]
    {
        use tauri::ipc::InvokeResponseBody;
        let handle = api.register_android_plugin("app.tauri.shared", "SharedPlugin")?;
        let app_handle = app.clone();
        if let Err(error) = handle.run_mobile_plugin::<()>(
            "setShareEventsHandler",
            ShareWatcher {
                channel: Channel::new(move |event| {
                    eprintln!("plugin: received pre-event");
                    let payload = match event {
                        InvokeResponseBody::Json(payload) =>
                            serde_json::from_str::<ShareEvent>(&payload).ok(),
                        InvokeResponseBody::Raw(_) => None
                    };
                    use tauri::Emitter;

                    //trigger("share-event", vec![payload]);
                    eprintln!("plugin: received event, emitting it");
                    let _res = app_handle.emit("share-event", vec![payload]);
                    eprintln!("sent share_event: ${_res:?}");

                    Ok(())
                })
            }
        ) {
            eprintln!("cannot watch share events: {error:?}");
        }

        return Ok(Shared(handle));
    }

    #[cfg(target_os = "ios")]
    {
        let handle = api.register_ios_plugin(init_plugin_shared)?;
        unimplemented!();
    }
}

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Shared<R>> {
    let handle = _api.register_android_plugin("app.tauri.shared", "SharedPlugin")?;
    Ok(Shared(handle))

    // Alternatively, deploy a Channel :
    //init_share_watcher(_app, _api)
}

/*
//pub fn init<R: Runtime>() -> ShareWatcher<R> {}
//pub fn init<R: Runtime>() -> TauriPlugin<R, Option<crate::plugin::Config>> {
pub fn init<R: Runtime>() -> crate::Result<<TauriPlugin<R, Option<crate::plugin::Config>>> {
    PluginBuilder::new("window")
        .setup(|_app, _api| {
            use crate::plugin::init_share_watcher;
            let handler = init_share_watcher(&_app, _api);
            eprintln!("handler is set up: {handler:#?}");

            Ok(())
        })
        .on_event(|_app, event| {
            match event {
                RunEvent::Ready => {
                    eprintln!("app is ready");
                }
                RunEvent::WindowEvent { label, event, .. } => {
                    eprintln!("window {} received an event: {:?}", label, event);
                }
                _ => (),
            }
        })
        .build()
        /*
        .invoke_handler(tauri::generate_handler![add_share_watcher])
        */
}
*/





/*
impl<R: Runtime> Shared<R> {
    fn create_watcher(&self) -> Channel {
        let channel: Channel<InvokeResponseBody> = Channel::new(move |event| {
            let payload = match event {
                InvokeResponseBody::Json(payload) => serde_json::from_str::<ShareEvent>(&payload).ok(),
                InvokeResponseBody::Raw(_) => None
            };

            Ok(())
        });
        self.handle.run_mobile_plugin("watchShareEvents");

        channel
    }
}

// this command can be called in the frontend using `invoke('plugin:window|add_share_watcher')`.
#[tauri::command]
async fn add_share_watcher<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>
) -> Result<(), String> {
    eprintln!("command called");

    Ok(())
}
*/

