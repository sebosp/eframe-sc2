#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    // This will start the web server.
    // Then we can start the GUI to interact with it.
    // The web server will eventually serve also the wasm to interact
    // with the server as well.
    eframe_sc2::cli::process_cli_request().await.unwrap();
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_sc2::SC2ReplayAnalyser::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
