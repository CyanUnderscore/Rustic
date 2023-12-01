extern crate rodio;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct AudioPlayer{
    pub sink: rodio::Sink,
}

impl AudioPlayer {
    pub fn new()-> Self{
        let (_stream, endpoint) = rodio::OutputStream::try_default().unwrap();
        let the_sink = rodio::Sink::try_new(&endpoint).unwrap();
        AudioPlayer{ sink:the_sink}
    }
    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }
}
