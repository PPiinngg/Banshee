use eframe::egui::Ui;

pub mod fractal;
pub mod reform;

pub fn distort_algo_menu(ui: &mut Ui, algo: &mut Box<dyn super::Algorithm>) {
	if ui.button("Fractal").clicked() { *algo = Box::new(fractal::Fractal::default()) };
	if ui.button("Re-form").clicked() { *algo = Box::new(reform::Reform::default()) };
}