use eframe::egui::Ui;

pub mod fractal;
pub mod reform;

pub fn distort_algo_menu(ui: &mut Ui, algo: &mut Box<dyn super::Algorithm>, suffix: &mut String) {
	if ui.button("Fractal").clicked() { *algo = Box::new(fractal::Fractal::default()); *suffix = "_fractal".to_string(); };
	if ui.button("Re-form").clicked() { *algo = Box::new(reform::Reform::default()); *suffix = "_reform".to_string(); };
}