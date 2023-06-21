pub mod distort;

use eframe::egui::{Context, Ui};

/* These are currently temporary, the way that algorithms-
are categorised in CDP is inconsistent and confusing and-
thus we'll have to sit down soon and re-categorise them */
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlgoCategory {
	Distort,
	Dynamics, // Envel + Envnu
	Filter,
	Grain,
	Utility, // Housekeep + SndInfo
	Reverb,
}

pub trait Algorithm {
	fn process(&mut self, channels: &mut Vec<Vec<f32>>);
	fn draw(&mut self, ctx: &Context, ui: &mut Ui);
}
