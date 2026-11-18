fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(
            tauri_build::AppManifest::new().commands(&[
                "get_display",
                "input_digit",
                "input_decimal",
                "set_operation",
                "evaluate",
                "clear_all",
                "backspace",
                "toggle_sign",
            ]),
        ),
    )
    .expect("failed to run tauri-build");
}
