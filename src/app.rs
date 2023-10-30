extern crate rodio;
extern crate dirs;

use std::fs::File;
use egui::util::cache::CacheTrait;
use native_dialog::FileDialog;
use std::fs;
use std::io::BufReader;
use std::path;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use futures_util::FutureExt;
use futures;

mod song;
use song::Song;

mod file_managment;

mod web_scraper;

pub struct rustic {
    queue: Vec<Song>,
    repeat: bool,
    downloading: bool,
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
        if self.queue.len() != 0 {
            let audio_file = File::open(&self.queue[self.queue.len()-1].path).unwrap();
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

    async fn get_song_name(link: String) -> Result<String, reqwest::Error> {
        web_scraper::get_name(&link).await
    }
    
    fn link_queue<'s>(&'s mut self, link: String) {
        let song_name = "unknown".to_owned();
        let (tx, mut rx) = mpsc::channel(32); // Adjust the buffer size as needed
    
        let link_clone = link.clone();
        let rt = Runtime::new().unwrap();
        let enter = rt.enter();
        tokio::spawn(async move {
            let result = Self::get_song_name(link_clone).await;
    
            match result {
                Ok(name) => {
                    let new_song = Song {
                        path: link.clone(),
                        name,
                    };
                    println!("{}", &new_song.name);
                    tx.send(new_song).await.expect("Channel send error");
                }
                Err(err) => {
                    println!("Couldn't get the song name: {}", err);
                    let new_song = Song {
                        path: link.clone(),
                        name: "Unknown".to_owned(),
                    };
                    println!("{}", &new_song.name);
                    tx.send(new_song).await.expect("Channel send error");
                }
            }
        });
    
        // Now, you can receive the result from the asynchronous task
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                if let Some(new_song) = rx.recv().await {
                    self.queue.push(new_song);
                }
            });
        drop(enter);
    }
}

impl Default for rustic {
    fn default() -> Self{
        Self{
            queue: Vec::new(),
            repeat: false,
            downloading: false,
            current_link: "".to_owned(),
            downloading_location: file_managment::get_music_dir().unwrap(),
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
                for i in 0..self.queue.len() {
                    ui.label(format!("Item {}", i));
                    ui.label(&self.queue[i].name);
                    ui.separator();
                }
            });
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("Hi and welcome to rustic, its a music reader");
            ui.group(|ui| {
                ui.checkbox(&mut self.downloading, "download the songs");
                ui.set_visible(self.downloading);
                ui.group(|ui| {
                    ui.label(&self.downloading_location);
                    ui.checkbox(&mut self.custom_downloading, "change the file location");
                    ui.set_visible(self.custom_downloading);
                    if ui.button("file browsing").clicked() {
                        println!("button cliked");
                        let file = FileDialog::new().set_location("~/Document").add_filter("texte ", &["txt"]).show_open_single_file().unwrap();
                        self.downloading_location = match file {
                            Some(file) => file.to_str().unwrap_or_default().to_string(),
                            None => file_managment::get_music_dir().unwrap(),
                        };}
                })
            });
            
            ui.text_edit_singleline(&mut self.current_link);
            ui.horizontal(|ui| {
                if ui.button("play").clicked() {
                    self.link_queue(self.current_link.clone());
                    self.play();
                }
                if ui.button("add to queue").clicked() {
                    self.link_queue(self.current_link.clone());
                    println!("a");
                }
            });
       });
   }
}