const COMMANDS: &[&str] = &["ping", "registerListener"];

fn main() {
    // Do not build when processed by docs.rs.
    if std::env::var("DOCS_RS").is_err() {
        tauri_plugin::Builder::new(COMMANDS)
            .android_path("android")
            .ios_path("ios")
            .build();
    }
}
