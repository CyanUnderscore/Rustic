extern crate rodio;
extern crate dirs;

use std::fs::File;
use std::fs;
use std::process::Command;
use egui::util::cache::CacheTrait;
use native_dialog::FileDialog;
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

mod audio_player;
use audio_player::AudioPlayer;

use self::file_managment::get_music_dir;

pub struct rustic {
    queue: Vec<Song>,
    repeat: bool,
    player: AudioPlayer,
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
            let audio_file ;
            if self.queue[0].path.starts_with("https://"){
                Command::new("yt-dlp").arg("-o").arg(file_managment::get_music_dir()).arg(self.queue[0].path);
                audio_file = File::open(&self.queue[self.queue.len()-1].path).unwrap();
            }else {
                audio_file= File::open(&self.queue[self.queue.len()-1].path).unwrap()
            }
            if  !self.repeat {
                self.queue.pop();
            }
            println!("heya");
            let source = rodio::Decoder::new(BufReader::new(audio_file)).unwrap();
            print!("hey");
            // Jouer le fichier audio
            self.player.sink.append(source);
            
            
    
    }

    async fn get_song_name(link: String) -> Result<String, reqwest::Error> {
        web_scraper::get_name(&link).await
    }
    
    fn link_queue<'s>(&'s mut self, link: String) {
        if link.starts_with("https://"){
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
        } else {
            let parsed_link: Vec<&str> = link.split("/").collect();
            let last_part = parsed_link[parsed_link.len()-1].to_owned();
            let song_name: &str = last_part.split(".").collect::<Vec<&str>>()[0];
            let new_song = Song {
                path: link.clone(),
                name: song_name.to_owned(),
            };
            println!("{}", &new_song.name);
            self.queue.push(new_song);
        }
        
    }
}

impl Default for rustic {
    fn default() -> Self{
        Self{
            queue: Vec::new(),
            repeat: false,
            player: AudioPlayer::new(),
            downloading: false,
            current_link: "".to_owned(),
            downloading_location: file_managment::get_music_dir().unwrap(),
            custom_downloading: false
        }
    }
}

impl eframe::App for rustic {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //if self.queue.len() != 0 && self.queue.len() > self.player.sink.len(){
        //    self.queue.remove(0);
        //} 
        
        egui::SidePanel::right("song list").show(ctx, |ui| {
            ui.heading("Library");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Ok(song_files) = fs::read_dir(get_music_dir().unwrap()) {
                    for song_file in song_files {
                        if let Ok(entry) = song_file {
                            let file_name = entry.file_name();
                            ui.label(format!("{}", file_name.to_str().unwrap()));
                        } else {
                            eprintln!("Erreur lors de la lecture du fichier.");
                        }
                    }
                } else {
                    eprintln!("Erreur lors de l'ouverture du répertoire.");
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
                ui.set_enabled(self.downloading);
                ui.group(|ui| {
                    ui.label("download location");
                    ui.label(&self.downloading_location);
                    ui.checkbox(&mut self.custom_downloading, "change the file location");
                    ui.set_enabled(self.custom_downloading);
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
                    if self.queue.len() != 0{
                        print!("playing {}", &self.queue[0].name);
                        self.play();
                    }
                    println!("nothing in the queue");
                }
                if ui.button("add to queue").clicked() {
                    self.link_queue(self.current_link.clone());
                    println!("added to the queue");
                }
            });
       });
   }
}