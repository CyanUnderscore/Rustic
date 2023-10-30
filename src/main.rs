use eframe::egui;
use Rustic::rustic;
use tokio;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(rustic::new(cc))));
}