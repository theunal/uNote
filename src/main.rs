mod app;
mod crypto;
mod db;
mod models;
mod settings;
mod theme;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_min_inner_size([600.0, 400.0])
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "uNote",
        options,
        Box::new(|cc| {
            let dark = cc.egui_ctx.system_theme() == Some(egui::Theme::Dark);

            #[cfg(target_os = "windows")]
            {
                use raw_window_handle::HasWindowHandle;
                if let Ok(hwnd_handle) = cc.window_handle() {
                    if let raw_window_handle::RawWindowHandle::Win32(h) = hwnd_handle.as_raw() {
                        let hwnd_ptr = h.hwnd.get() as *mut std::ffi::c_void;

                        unsafe {
                            use windows::Win32::Graphics::Dwm::{
                                DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE,
                                DWMWCP_ROUND,
                            };
                            let corner = DWMWCP_ROUND;
                            let _ = DwmSetWindowAttribute(
                                windows::Win32::Foundation::HWND(hwnd_ptr),
                                DWMWA_WINDOW_CORNER_PREFERENCE,
                                &corner as *const _ as _,
                                std::mem::size_of::<i32>() as u32,
                            );
                        }
                    }
                }

                let alpha: u8 = if dark { 200 } else { 210 };
                let color = if dark {
                    (32u8, 32u8, 32u8, alpha)
                } else {
                    (243u8, 243u8, 243u8, alpha)
                };
                let _ = window_vibrancy::apply_acrylic(cc, Some(color));
            }

            Ok(Box::new(app::NoteApp::new()))
        }),
    )
}
