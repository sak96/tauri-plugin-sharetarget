const COMMANDS: &[&str] = &["ping", "register_listener"];

fn main() {
    // Do not build when processed by docs.rs.
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
