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

mod voting;
mod config;

#[tokio::main]
pub async fn main() {
    //println!("{} {} {}", config::CONFIG_VARS.twitch_username, config::CONFIG_VARS.timer, config::CONFIG_VARS.screen_scaling);
    if config::CONFIG_VARS.twitch_username == "username"{
        println!("You forgot to set your Twitch username in the config file!");
    }
    let votes: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let phase: Arc<Mutex<voting::VotingPhase>> = Arc::new(Mutex::new(voting::VotingPhase::Regular));
    
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let votes_arc = votes.clone();
    let phase_arc = phase.clone();
    let join_handle = tokio::spawn(async move{
        while let Some(message) = incoming_messages.recv().await {
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

    
    client.join(config::CONFIG_VARS.twitch_username.to_owned()).unwrap();

    let votes_arc = votes.clone();
    let phase_arc = phase.clone();
    let timer = task::spawn(async move{
        let mut interval = time::interval(Duration::from_secs(1));
        let mut countdown = config::CONFIG_VARS.timer;

        loop {
            interval.tick().await;
            countdown -= 1;
            let mut phase_arc = phase_arc.lock();
            if countdown <= 0{
                let mut votes_arc = votes_arc.lock();
                voting::collect_votes(&mut votes_arc, &mut phase_arc);
                countdown = config::CONFIG_VARS.timer;
            }
        }
    });

    // This is code used for testing the mouse boundries. DO NOT RUN THIS IF YOU DON'T KNOW WHAT YOU'RE DOING.
    // THIS FUNCTION WILL HIJACK YOUR MOUSE AND CLICK A TON OF PLACES. BE CAREFUL.
    /*let mouse_derp = task::spawn(async move{
        for i in 1..40{
            for j in 1..24{
                if i > 32{ continue; }
                let mouse = Mouse::new();
                let x_dist = (f64::from(SCREEN_X / 40 * i + 25) / 1.25).floor() as i32;
                let y_dist = (f64::from(SCREEN_Y / 24 * j) / 1.25).floor() as i32;
                mouse.move_to(x_dist, y_dist);
                mouse.press(&Keys::LEFT).expect("Unable to press button");
                mouse.release(&Keys::LEFT).expect("Unable to release button");
                sleep(Duration::from_millis(20)).await;
                //println!("x: {}, y: {}", SCREEN_X / 40 * i + 25, SCREEN_Y / 24 * j);
                println!("Location: {}{}", "abcdefghijklmnopqrstuvwxyz".chars().nth(j as usize - 1).unwrap(), i);
            }
        }
    });*/

    join_handle.await.unwrap();
    timer.await.unwrap();
    //mouse_derp.await.unwrap();
}