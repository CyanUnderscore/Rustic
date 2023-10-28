extern crate rodio;
extern crate dirs;

use std::fs::File;
use native_dialog::FileDialog;
use std::fs;
use std::io::BufReader;
use std::path;

pub struct rustic {
    queue: Vec<String>,
    repeat: bool,
    current_link: String,
    downloading_location: String,
    custom_downloading: bool,
}


impl rustic {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.


        
        Self::default()
    }
    fn play(&mut self){
        let audio_file = File::open(&self.queue[self.queue.len()-1]).expect("nothing in the queue");
        if  !self.repeat {
            self.queue.pop();
        }
        let source = rodio::Decoder::new(BufReader::new(audio_file)).unwrap();

        // Créer un gestionnaire audio
        let (_stream, endpoint) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&endpoint).unwrap();

        // Jouer le fichier audio
        sink.append(source);
        sink.sleep_until_end();

    }
}

impl Default for rustic {
    fn default() -> Self{
        Self{
            queue: Vec::new(),
            repeat: false,
            current_link: "".to_owned(),
            downloading_location: "".to_owned(),
            custom_downloading: false
        }
    }
}

impl eframe::App for rustic {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::SidePanel::right("song list").show(ctx, |ui| {
            ui.heading("Library");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..100 {
                    ui.label(format!("Item {}", i));
                    ui.label("yeah");
                    ui.separator();
                }
            });
        });
        
        egui::SidePanel::left("queue list").show(ctx, |ui| {
            ui.heading("Queue");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..100 {
                    ui.label(format!("Item {}", i));
                    ui.label("yeah");
                    ui.separator();
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("Hi and welcome to rustic, its a music reader");
            ui.group(|ui| {
                ui.checkbox(&mut self.custom_downloading, "change the file location");
                ui.set_visible(self.custom_downloading);
                if ui.button("file browsing").clicked() {
                    println!("button cliked");
                    let file = FileDialog::new().set_location("~/Document").add_filter("texte ", &["txt"]).show_open_single_file().unwrap();
                    self.downloading_location = match file {
                        Some(file) => file.to_str().unwrap_or_default().to_string(),
                        None => panic!("wrong path"),
                    };}
            });
            ui.text_edit_singleline(&mut self.current_link);
            ui.horizontal(|ui| {
                if ui.button("play").clicked() {
                    self.play();
                }
                if ui.button("add to queue").clicked() {
                    if self.current_link != "" {
                        self.queue.push((&self.current_link).to_owned())
                    }
                }
            });
       });
   }
}