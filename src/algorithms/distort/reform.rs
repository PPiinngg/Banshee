use crate::algorithms::Algorithm;
use std::f32::consts::PI;
use eframe::egui::{self, Context, Ui};

#[derive(Debug, PartialEq)]
enum ReformMode {
	Sine,
	Saw,
	Square,
}

pub struct Reform {
	mode: ReformMode,
}

impl Default for Reform {
	fn default() -> Self {
		Self {
			mode: ReformMode::Sine,
		}
	}
}

impl Algorithm for Reform {
	fn process(&mut self, channels: &mut Vec<Vec<f32>>) {
		for channel in channels.iter_mut() {
			let mut new_channel = Vec::<f32>::new();
			reform(channel, &mut new_channel, &self.mode);
			*channel = new_channel;
		}
	}

	fn draw(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.heading("Reform");
		ui.spacing_mut().item_spacing.y = 8f32;
		ui.separator();
		egui::ComboBox::from_label("Mode")
			.selected_text(format!("{:?}", self.mode))
			.show_ui(ui, |ui| {
				ui.selectable_value(&mut self.mode, ReformMode::Sine, "Sine");
				ui.selectable_value(&mut self.mode, ReformMode::Saw, "Saw");
				ui.selectable_value(&mut self.mode, ReformMode::Square, "Square");
			});
	}
}

// nicked from old impl, probably could use a slight overhaul
fn reform(in_vec: &Vec<f32>, out_vec: &mut Vec<f32>, mode: &ReformMode) {
	let mut prev: f32 = 0f32;
	let mut peak: f32 = 0f32;
	let mut sign: f32 = in_vec[0].signum();
	let mut length: usize = 0usize;

	for in_smp in in_vec {
		if in_smp.signum() != prev.signum() {
			length += 1;

			for i in 0..length {
				match mode {
					ReformMode::Sine => out_vec.push(((PI / length as f32) * i as f32).sin() * sign * peak),
					ReformMode::Saw => {
						let phasor = (1f32 / length as f32) * i as f32;
						let offset = (peak * sign).min(0f32);
						out_vec.push((phasor * peak) + offset)
					},
					ReformMode::Square => out_vec.push(peak * sign),
				}
			}
			prev = *in_smp;
			peak = 0f32;
			sign = in_smp.signum();
			length = 0;
			continue;
		}

		prev = *in_smp;
		peak = peak.max(in_smp.abs());
		length += 1;
	}

	let step = (std::f32::consts::PI / length as f32) * sign;
	for i in 0..length {
		match mode {
			ReformMode::Sine => out_vec.push((step * i as f32).sin() * peak),
			ReformMode::Saw => out_vec.push(((step / std::f32::consts::PI) * i as f32) * peak),
			ReformMode::Square => out_vec.push(peak * sign),
		}
	}
}
