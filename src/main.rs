#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod algorithms;
mod file_io;
mod state;

struct App {
	state: state::CdpState,
}

impl Default for App {
	fn default() -> Self {
		Self {
			state: Default::default(),
		}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		self.state.draw(ctx, frame);
	}
}

fn main() -> Result<(), eframe::Error> {
	env_logger::init();
	let frame_options = eframe::NativeOptions {
		initial_window_size: Some(egui::vec2(500f32, 500f32)),
		..Default::default()
	};

	eframe::run_native(
		&format!("Banshee [v{}]", env!("CARGO_PKG_VERSION")),
		frame_options,
		Box::new(|_cc| Box::<App>::default()),
	)
}
