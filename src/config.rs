// This file handles the config file for the program
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

pub struct Configs{
    pub twitch_username: String,
    pub timer: usize,
    pub screen_scaling: f32
}

impl Configs{
    pub fn new() -> Configs{
        let path = "config.cfg";
        let file = File::open(path).unwrap(); // TODO: error handle and create default cfg file
        let buf_reader = BufReader::new(file);

        let mut twitch_username: String = String::from("USERNAME");
        let mut timer: usize = 10;
        let mut screen_scaling: f32 = 1.0;

        for line in buf_reader.lines(){
            let var_name = line.as_ref().unwrap().split("=").next().unwrap().trim().to_lowercase();
            let var_value = line.unwrap().split("=").nth(1).unwrap().trim().to_lowercase();

            match var_name.as_str(){
                "twitch_username" => { twitch_username = var_value; },
                "timer" => { timer = var_value.parse::<usize>().unwrap().clone(); },
                "screen_scaling" => { screen_scaling = var_value.parse::<f32>().unwrap().clone(); }
                _ => {}
            }
        }
        Configs { twitch_username: String::from(twitch_username), timer: timer, screen_scaling: screen_scaling }
    }
}