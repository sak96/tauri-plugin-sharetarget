#[cfg(not(docsrs))]
const COMMANDS: &[&str] = &["ping", "registerListener"];

fn main() {
    // Do not build when processed by docs.rs.
    #[cfg(not(docsrs))]
    {
        tauri_plugin::Builder::new(COMMANDS)
            .android_path("android")
            .ios_path("ios")
            .build();
    }
}
