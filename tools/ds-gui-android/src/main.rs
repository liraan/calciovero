//! Desktop preview for the Android GUI.

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([430.0, 820.0])
            .with_min_inner_size([360.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calcio Vero Android Preview",
        options,
        Box::new(|_cc| Ok(Box::new(ds_gui_android::AndroidApp::default()))),
    )
}

#[cfg(target_os = "android")]
fn main() {}
