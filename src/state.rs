use eframe::egui::{self, TextBuffer, ScrollArea};

use crate::{algorithms::{AlgoCategory, Algorithm, distort::{self, distort_algo_menu}}, file_io::{Buffer, audio_import, wav_export}};

pub struct CdpState {
	buffers: Vec<Buffer>,
	algo_category: AlgoCategory,
	algorithm: Box<dyn Algorithm>,
	export_suffix: String,
}

impl Default for CdpState {
	fn default() -> Self {
		Self {
			buffers: Vec::<Buffer>::default(),
			algo_category: AlgoCategory::Distort,
			algorithm: Box::new(distort::reform::Reform::default()),
			export_suffix: "_reform".to_string(),
		}
	}
}

impl CdpState {
	pub fn draw(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
			ui.heading("Banshee");
		});

		egui::TopBottomPanel::bottom("bottom-panel").show(ctx, |ui| {
			ui.add_space(5f32);
			ui.horizontal(|ui| {
				ui.label("Export Suffix");
				ui.text_edit_singleline(&mut self.export_suffix);
			});
			ui.add_space(1f32);
		});

		egui::SidePanel::left("category-panel").resizable(false).max_width(1f32).show(ctx, |ui| {
			ui.add_space(3f32);
			egui::ComboBox::from_label("Category")
				.selected_text(format!("{:?}", self.algo_category))
				.show_ui(ui, |ui| {
					const ALL_CATEGORIES: [AlgoCategory; 6] = [
						AlgoCategory::Distort,
						AlgoCategory::Dynamics,
						AlgoCategory::Filter,
						AlgoCategory::Grain,
						AlgoCategory::Utility,
						AlgoCategory::Reverb,
					];
					for category in ALL_CATEGORIES {
						ui.selectable_value(
							&mut self.algo_category,
							category,
							format!("{:?}", category),
						);
					}
				});
			ui.separator();
			egui::ScrollArea::new([false, true]).show(ui, |ui| {match self.algo_category {
				AlgoCategory::Distort => { distort_algo_menu(ui, &mut self.algorithm, &mut self.export_suffix) },
				_ => {}
			}});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			self.algorithm.draw(ctx, ui);
		});

		preview_files_being_dropped(ctx);

		ctx.input(|i| {
			if !i.raw.dropped_files.is_empty() {
				let dropped_files = i.raw.dropped_files.clone();
				for file in dropped_files {
					match file.path {
						Some(path) => match audio_import(path.as_os_str().to_str().unwrap_or_default()) {
							Some(buffer) => self.buffers.push(buffer),
							None => continue,
						},
						None => continue,
					}

					for buffer in self.buffers.iter_mut() {
						self.algorithm.process(&mut buffer.channels);
						wav_export(&self.export_suffix, buffer);
					}

					self.buffers.clear();
				}
			}
		});
	}
}

// Placeholder
fn preview_files_being_dropped(ctx: &egui::Context) {
	use egui::*;
	use std::fmt::Write as _;

	if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
		let text = ctx.input(|i| {
			let mut text = "Dropping files:\n".to_owned();
			for file in &i.raw.hovered_files {
				if let Some(path) = &file.path {
					write!(text, "\n{}", path.display()).ok();
				} else if !file.mime.is_empty() {
					write!(text, "\n{}", file.mime).ok();
				} else {
					text += "\n???";
				}
			}
			text
		});

		let painter =
			ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

		let screen_rect = ctx.screen_rect();
		painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
		painter.text(
			screen_rect.center(),
			Align2::CENTER_CENTER,
			text,
			TextStyle::Heading.resolve(&ctx.style()),
			Color32::WHITE,
		);
	}
}