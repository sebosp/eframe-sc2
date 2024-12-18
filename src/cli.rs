use clap::Parser;
use eframe::egui;
use tokio::signal;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets the source of the data, can be a file or directory.
    #[arg(short, long, value_name = "PATH")]
    pub source_dir: String,

    /// Turn debugging information on
    #[arg(short, long, default_value = "info")]
    pub verbosity_level: String,

    /// Sets the IP to listen for the web server
    #[arg(short, long, default_value = "0.0.0.0")]
    pub ip: String,

    /// Sets the port to listen for the web server
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    /// Disables the native server in case we want to run only the web server
    #[arg(short, long, default_value = "false")]
    pub disable_native: bool,
}

/// Handles the request from the CLI to start the server
/// And initializes the logging system
pub async fn process_cli_request() -> Result<(), crate::error::Error> {
    let cli = Cli::parse();
    // use the cli verbosity level to set the tracing level
    let level = match cli.verbosity_level.as_str() {
        "error" => tracing::Level::ERROR,
        "warn" => tracing::Level::WARN,
        "info" => tracing::Level::INFO,
        "debug" => tracing::Level::DEBUG,
        "trace" => tracing::Level::TRACE,
        _ => tracing::Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_env_filter("info")
        .init();

    crate::server::start_server(&cli).await;

    if !cli.disable_native {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
            ..Default::default()
        };
        eframe::run_native(
            "eframe sc2",
            native_options,
            Box::new(|cc| Ok(Box::new(crate::SC2ReplayExplorer::new(cc)))),
        )
        .unwrap();
    }
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }
    Ok(())
}
