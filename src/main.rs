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

    join_handle.await.unwrap();
    forever.await.unwrap();
}