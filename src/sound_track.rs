use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub fn sound_track() {    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mp3_files = vec![
        "assets/sound_track/2019-08-25_-_8bit-Smooth_Presentation_-_David_Fesliyan.mp3",
        "assets/sound_track/slow-2021-08-17_-_8_Bit_Nostalgia_-_www.FesliyanStudios.com.mp3",
        "assets/sound_track/SLOWER-TEMPO2019-12-09_-_Retro_Forest_-_David_Fesliyan.mp3",
        "assets/sound_track/2019-01-10_-_Land_of_8_Bits_-_Stephen_Bennett_-_FesliyanStudios.com.mp3",
        
    ];

    loop {
        for file_path in &mp3_files {
            let file = File::open(file_path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
        }
    }

}