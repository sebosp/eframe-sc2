//! Main app

use crate::meta::SnapshotStats;
use eframe::egui;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct SC2ReplayExplorer {
    /// Contains the metadata related to the backend snapshot.
    #[serde(skip)]
    analyzed_snapshot_meta: Option<poll_promise::Promise<SnapshotStats>>,

    /// The Map selection UI
    #[serde(skip)]
    map_picker: crate::api::v1::details::maps::SC2MapPicker,

    /// A filter in the future
    #[serde(skip)]
    value: f32,

    /// A list of files drag and dropped.
    dropped_files: Vec<egui::DroppedFile>,

    /// The path of the picked file
    picked_path: Option<String>,

    #[serde(skip)]
    file_request_future: Option<poll_promise::Promise<Option<Vec<u8>>>>,

    /// The details of the SC2Replay
    #[serde(skip)]
    replay_details: Option<s2protocol::details::Details>,

    /// The details status
    #[serde(skip)]
    replay_details_status_color: egui::Color32,
}

impl Default for SC2ReplayExplorer {
    fn default() -> Self {
        Self {
            map_picker: Default::default(),
            value: 2.7,
            dropped_files: Default::default(),
            picked_path: None,
            analyzed_snapshot_meta: Default::default(),
            file_request_future: None,
            replay_details: None,
            replay_details_status_color: egui::Color32::GREEN,
        }
    }
}

impl SC2ReplayExplorer {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    /// Loads a file Using rfd file open dialog
    async fn load_file() -> Option<Vec<u8>> {
        let res = rfd::AsyncFileDialog::new().pick_file().await;

        match res {
            Some(file) => Some(file.read().await),
            None => None,
        }
    }

    /// Loads basic information about the analyzed metadata
    async fn load_analyzed_snapshot_meta() -> SnapshotStats {
        let request = ehttp::Request::get("/api/v1/analyzed_snapshot_meta");
        ehttp::fetch_async(request)
            .await
            .map(|response| serde_json::from_slice(&response.bytes).unwrap_or_default())
            .unwrap_or_default()
    }
}

impl eframe::App for SC2ReplayExplorer {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("Main", |ui| {
                    if ui.button("About").clicked() {
                        // TODO: open a modal window with the about info
                    }
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                    {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    }
                });
                ui.add_space(16.0);

                ui.horizontal(|ui| {
                    if ui.button("Reload Stats").clicked() {
                        #[cfg(target_arch = "wasm32")]
                        {
                            self.analyzed_snapshot_meta = Some(poll_promise::Promise::spawn_local(
                                Self::load_analyzed_snapshot_meta(),
                            ));
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            self.analyzed_snapshot_meta = Some(poll_promise::Promise::spawn_async(
                                Self::load_analyzed_snapshot_meta(),
                            ));
                        }
                    }
                    if let Some(analyzed_snapshot_meta) = &self.analyzed_snapshot_meta {
                        if let Some(analyzed_snapshot_meta) = analyzed_snapshot_meta.ready() {
                            // Crete a floating panel with the stats
                            ui.label(format!("Total files: {}", analyzed_snapshot_meta.num_files));
                            ui.label(format!("Total maps: {}", analyzed_snapshot_meta.num_maps));
                            ui.label(format!("Min date: {:?}", analyzed_snapshot_meta.min_date));
                            ui.label(format!("Max date: {:?}", analyzed_snapshot_meta.max_date));
                            ui.label(format!(
                                "Total players: {}",
                                analyzed_snapshot_meta.num_players
                            ));
                        } else {
                            ui.label("Loading snapshot metadata...");
                        }
                    } else {
                        ui.label("N/A");
                    }
                });
                //egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("SC2Replays Batch Analyser");

            ui.label("Drag-and-drop SC2Replay file onto the window!");

            if ui.button("Open map selection").clicked() {
                self.map_picker.is_open_map_selection = true;
            }
            if ui.button("Open file...").clicked() {
                #[cfg(target_arch = "wasm32")]
                {
                    self.file_request_future =
                        Some(poll_promise::Promise::spawn_local(Self::load_file()));
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.file_request_future =
                        Some(poll_promise::Promise::spawn_async(Self::load_file()));
                }
            }

            self.map_picker.update(ctx, frame);

            if let Some(file_async) = &self.file_request_future {
                if let Some(Some(file_contents)) = file_async.ready() {
                    self.replay_details = match s2protocol::parser::parse(file_contents) {
                        Ok((_input, mpq)) => {
                            self.replay_details_status_color = egui::Color32::GREEN;
                            s2protocol::details::Details::new("TEST", &mpq, file_contents).ok()
                        }
                        Err(e) => {
                            println!("Error parsing replay: {}", e);
                            self.replay_details_status_color = egui::Color32::RED;
                            None
                        }
                    };
                }
            }

            if let Some(replay_details) = &self.replay_details {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        self.replay_details_status_color,
                        format!("Status: {:?}", self.replay_details_status_color),
                    );
                    ui.monospace(format!("{:?}", replay_details));
                });
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Matching Replays: ");
            });

            ui.add(egui::github_link_file!(
                "https://github.com/sebosp/eframe-sc2/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
