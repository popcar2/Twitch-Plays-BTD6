use std::collections::HashMap;
use mki::Keyboard;
use mouse_rs::{types::keys::Keys, Mouse};

pub enum VotingPhase{
    Regular,
    Placement
}

// Validate this is a vote syntax
pub fn validate_vote(message_text: &str, phase: &VotingPhase) -> bool{
    if message_text.to_lowercase().starts_with("tower ") && matches!(phase, VotingPhase::Regular){
        let second_word = message_text.split(" ").nth(1).unwrap().trim_end().to_lowercase();
        match second_word.as_str(){
            "hero" | "dart" | "boomerang" | "bomb" | "tack" | "ice"
            | "glue" | "sniper" | "sub" | "buccaneer" | "ace" | "heli"
            | "mortar" | "gunner" | "wizard" | "super" | "ninja"
            | "alchemist" | "druid" | "farm" | "factory" | "village"
            | "engineer" => { return true; },
            _ => {}
        }
    }
    return false;
}

pub fn add_vote(votes: &mut HashMap<String, String>, user_id: String, message_text: String){
    let mut word_iter = message_text.split(" ");
    let mut test = word_iter.next().unwrap().trim().to_lowercase();
    test += " ";
    test += word_iter.next().unwrap().trim().to_lowercase().as_str();
    println!("{}", test);
    votes.insert(user_id, test);

    /*for (key, value) in &*votes{
        println!("{} / {}", key, value);
    }*/
}

pub fn collect_votes(votes: &mut HashMap<String, String>, phase: &mut VotingPhase){
    let mut vote_results: HashMap<String, i32> = HashMap::new();
    for (_user_id, message_text) in votes.into_iter(){
        if vote_results.contains_key(message_text){
            let value = vote_results.get(message_text).unwrap().to_owned();
            vote_results.insert(message_text.to_string(), value + 1);
        }
        else{
            vote_results.insert(message_text.to_string(), 1);
        }
    }

    let mut highest_vote = String::new();
    let mut highest_count = 0;

    for (message_text, value) in vote_results.into_iter(){
        println!("{} / {}", message_text, value);
        if value > highest_count{
            highest_vote = message_text;
            highest_count = value;
        }
    }
    println!("{} won at {} votes!", highest_vote, highest_count);
    votes.clear();

    if highest_count > 0{
        activate_vote(highest_vote, phase);
    }
}

fn activate_vote(final_vote: String, phase: &mut VotingPhase){
    let first_word = final_vote.split(" ").next().unwrap();
    let second_word = final_vote.split(" ").nth(1).unwrap();

    if first_word == "tower"{
        if second_word == "hero"{
            Keyboard::U.click();
        }
        else if second_word == "dart"{
            Keyboard::Q.click();
        }
        else if second_word == "boomerang"{
            Keyboard::W.click();
        }
        else if second_word == "bomb"{
            Keyboard::E.click();
        }
        else if second_word == "tack"{
            Keyboard::R.click();
        }
        else if second_word == "ice"{
            Keyboard::T.click();
        }
        else if second_word == "glue"{
            Keyboard::Y.click();
        }
        else if second_word == "sniper"{
            Keyboard::Z.click();
        }
        else if second_word == "sub"{
            Keyboard::X.click();
        }
        else if second_word == "buccaneer"{
            Keyboard::C.click();
        }
        else if second_word == "ace"{
            Keyboard::V.click();
        }
        else if second_word == "heli"{
            Keyboard::B.click();
        }
        else if second_word == "mortar"{
            Keyboard::N.click();
        }
        else if second_word == "gunner"{
            Keyboard::M.click();
        }
        else if second_word == "wizard"{
            Keyboard::A.click();
        }
        else if second_word == "super"{
            Keyboard::S.click();
        }
        else if second_word == "ninja"{
            Keyboard::D.click();
        }
        else if second_word == "alchemist"{
            Keyboard::F.click();
        }
        else if second_word == "druid"{
            Keyboard::G.click();
        }
        else if second_word == "farm"{
            Keyboard::H.click();
        }
        else if second_word == "engineer"{
            Keyboard::L.click();
        }
        else if second_word == "factory"{
            Keyboard::J.click();
        }
        else if second_word == "village"{
            Keyboard::K.click();
        }
        *phase = VotingPhase::Placement;
        println!("Entering placement phase...");
    }
}