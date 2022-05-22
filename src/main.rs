use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;

use tokio::time::{sleep, Duration};
use tokio::{task, time};

use std::collections::HashMap;

use parking_lot::Mutex;
use std::sync::Arc;

use mouse_rs::{Mouse, types::keys::Keys};

mod voting;

const SCREEN_X: i32 = 1920;
const SCREEN_Y: i32 = 1080;

#[tokio::main]
pub async fn main() {
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
    forever.await.unwrap();
    //mouse_derp.await.unwrap();
}