use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

use tokio::time::Duration;
use tokio::{task, time};

use std::collections::HashMap;

use parking_lot::Mutex;
use std::sync::Arc;

use eframe::egui;
use eframe::emath::{ Pos2, Vec2 };

mod voting;

fn main(){
    // Starts tokio runtime
    std::thread::spawn(tokio_main);

    // Starts egui runtime
    let mut native_options = eframe::NativeOptions::default();
    native_options.always_on_top = true;
    native_options.decorated = false;
    native_options.transparent = true;
    native_options.initial_window_pos = Some(Pos2::new(0.0, 0.0));
    //native_options.initial_window_size = Some(Vec2::new(1920.0, 1080.0));
    eframe::run_native("Twitch Plays BTD6", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[tokio::main]
pub async fn tokio_main() {
    let votes: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let phase: Arc<Mutex<voting::VotingPhase>> = Arc::new(Mutex::new(voting::VotingPhase::Regular));
    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let votes_arc = votes.clone();
    let phase_arc = phase.clone();
    let join_handle = tokio::spawn(async move{
        while let Some(message) = incoming_messages.recv().await {
            //println!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!("{}: {}", msg.sender.name, msg.message_text);
                    let phase_arc = &*phase_arc.lock();
                    if voting::validate_vote(msg.message_text.as_str(), phase_arc){
                        let mut votes_arc = votes_arc.lock();
                        voting::add_vote(&mut votes_arc, msg.sender.id, msg.message_text);
                    }
                },
                _ => {}
            }
        }
    });

    
    client.join("popcar2".to_owned()).unwrap();

    let votes_arc = votes.clone();
    let phase_arc = phase.clone();
    let forever = task::spawn(async move{
        let mut interval = time::interval(Duration::from_secs(8));

        loop {
            interval.tick().await;
            let mut votes_arc = votes_arc.lock();
            let mut phase_arc = phase_arc.lock();
            voting::collect_votes(&mut votes_arc, &mut phase_arc);
        }
    });

    join_handle.await.unwrap();
    forever.await.unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("My Window").show(ctx, |ui| {
            ui.label("Hello World!");
         });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        //_visuals.dark_mode = true;
        egui::Rgba::TRANSPARENT
    }
}