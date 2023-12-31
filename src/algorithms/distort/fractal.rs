use crate::algorithms::Algorithm;
use eframe::egui::{self, Context, Ui, Widget};

const MAX_ITERATIONS: usize = 15usize;

pub struct Fractal {
	iterations: usize,
	dry_gain: f32,
	gains: Vec<f32>,
}

impl Default for Fractal {
	fn default() -> Self {
		Self {
			iterations: 5usize,
			dry_gain: 1f32,
			gains: vec![0f32; MAX_ITERATIONS],
		}
	}
}

impl Algorithm for Fractal {
	fn process(&mut self, channels: &mut Vec<Vec<f32>>) {
		for channel in channels.iter_mut() {
			let mut zero_cross_counter: usize = 0usize;
			let mut prev: f32 = 0f32;
			let mut wavecycle_begin_index: usize = 0usize;
			let mut wavecycle = Vec::<f32>::new();

			for i in 0..channel.len() {
				let sample = channel[i];

				if (sample.signum() != prev.signum()) && (zero_cross_counter == 1) {
					for cycle_idx in 0..wavecycle.len() {
						channel[wavecycle_begin_index + cycle_idx] *= self.dry_gain;
					}
					for iteration in 0..self.iterations {
						for cycle_idx in 0..wavecycle.len() {
							let f_idx: f32 = ((cycle_idx as f32 / wavecycle.len() as f32)
								* (iteration + 2) as f32) % 1f32;
							channel[wavecycle_begin_index + cycle_idx] +=
								f_index(&f_idx, &wavecycle) * self.gains[iteration] as f32;
						}
					}

					zero_cross_counter = 0usize;
					wavecycle.clear();
					wavecycle_begin_index = i;
				} else if sample.signum() != prev.signum() {
					zero_cross_counter += 1usize;
				}

				prev = sample;
				wavecycle.push(sample);
			}
		}
	}

	fn draw(&mut self, _ctx: &Context, ui: &mut Ui) {
		ui.heading("Fractal");
		ui.spacing_mut().item_spacing.y = 8f32;
		ui.separator();
		ui.horizontal(|ui| {
			ui.label("Harmonics");
			egui::widgets::Slider::new(&mut self.iterations, 1..=MAX_ITERATIONS).ui(ui);
		});
		egui::ScrollArea::new([true, false]).show(ui, |ui| {
			ui.horizontal(|ui| {
				ui.vertical(|ui| {
					egui::widgets::Slider::new(&mut self.dry_gain, 0.0..=1.0)
						.vertical()
						.ui(ui);
					ui.label("Dry");
				});
				for i in 0..self.iterations {
					ui.vertical(|ui| {
						egui::widgets::Slider::new(&mut self.gains[i], 0.0..=1.0)
							.vertical()
							.ui(ui);
						ui.label(format!("{}", i+2));
					});
				}
			});
		});
	}
}

#[inline]
fn f_index(i: &f32, vec: &Vec<f32>) -> f32 {
	let i_scaled = i * vec.len() as f32;
	let idx: usize = i_scaled.trunc() as usize;
	let v1: f32 = vec[idx];
	let v2: f32 = vec[(idx + 1).min(vec.len() - 1)];
	let t: f32 = i.fract();
	let t_inv: f32 = 1f32 - t;
	(v1 * t_inv) + (v2 * t)
}
