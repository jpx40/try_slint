// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::{random, Rng};
use std::error::Error;
slint::include_modules!();





fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let mut rng = rand::thread_rng();

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + { let u = rng.gen::<f32>() * (6.0 -1.0) + 1.0;
            
                u as i32  } );
        }
    });

    ui.run()?;

    Ok(())
}
