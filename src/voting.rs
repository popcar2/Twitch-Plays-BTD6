use std::collections::HashMap;
use mki::Keyboard;
use mouse_rs::{Mouse, types::keys::Keys};

use std::thread;
use std::time::Duration;

pub enum VotingPhase{
    Regular,
    Placement
}

// Used in two places: placement mode (ex: b23) and selection in regular mode (ex: select b23)
fn validate_location(location_text: &str) -> bool{

    // Range is from a-w 1-32 because these are usable locations in the play area
    // Checks in order: is in placement phase, first letter between a and x, 2nd char is a number
    if location_text.len() > 1 
    && "abcdefghijklmnopqrstuvw".contains(location_text.to_lowercase().chars().next().unwrap())
    && location_text.chars().nth(1).unwrap().is_numeric(){
        // has 2 numbers (ex: a17). Can't go higher than 32.
        if location_text.len() > 2{
            if location_text.chars().nth(2).unwrap().is_numeric(){
                let tile_num = String::from(location_text)[1..3].parse::<u8>().unwrap();
                if tile_num > 0 && tile_num < 33{
                    return true;
                }
                return false;
            }
            return false;
        }
    
        // only 1 number (ex: a7). This can't be 0 (ex: a0)
        if location_text.chars().nth(1).unwrap() != '0'{
            return true;
        }
    }
    return false;
}

fn calculate_location(location_text: &str) -> (i32, i32){
    // Adding 1 to row and column's defaults so it doesn't click the very edge of the screen and instead starts a little inwards.
    let row: u32 = "abcdefghijklmnopqrstuvw".find(location_text.chars().next().unwrap()).unwrap() as u32 + 1;
    let column: u32;
    
    // TODO: Get resolution and scaling % and left padding
    if location_text.len() > 2{
        column = String::from(location_text)[1..3].parse::<u32>().unwrap();
    }
    else{
        column = String::from(location_text)[1..2].parse::<u32>().unwrap();
    }

    let x_dist = (f64::from(1920 / 40 * column + 25) / 1.25).floor() as i32;
    let y_dist = (f64::from(1080 / 24 * row) / 1.25).floor() as i32;

    (x_dist, y_dist)
}

// Validate this is proper vote syntax
// The only exception which doesn't use this function is "select" which is handled by validate_vote_select()
// since that uses a different timer and hashmap.
pub fn validate_vote(message_text: &str, phase: &VotingPhase) -> bool{
    // Choose a tower (ex: tower bomb)
    if matches!(phase, VotingPhase::Regular) && message_text.to_lowercase().starts_with("tower "){
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

    else if matches!(phase, VotingPhase::Placement) && validate_location(message_text){
        return true;
    }
    return false;
}

pub fn validate_vote_select(message_text: &str, phase: &VotingPhase) -> bool{
    if matches!(phase, VotingPhase::Regular) && message_text.to_lowercase().starts_with("select "){
        let second_word = message_text.split(" ").nth(1).unwrap().trim_end().to_lowercase();

        if validate_location(second_word.as_str()){
            return true;
        }
    }
    return false;
}

pub fn add_vote(votes: &mut HashMap<String, String>, user_id: String, message_text: String){
    println!("{}", message_text.to_lowercase());
    votes.insert(user_id, message_text.to_lowercase());

    /*for (key, value) in &*votes{
        println!("{} / {}", key, value);
    }*/
}

// Chooses the highest winning vote in the votes hashmap then resets it and passes the final vote to activate_vote()
pub fn collect_votes(votes: &mut HashMap<String, String>, phase: &mut VotingPhase){
    let mut vote_results: HashMap<String, i32> = HashMap::new();

    // Count votes and keep the highest one
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

    votes.clear();

    if highest_count > 0{
        println!("{} won at {} votes!", highest_vote, highest_count);
        activate_vote(highest_vote, phase);
    }
    else{
        //println!("Nobody voted, nothing happened...");
    }
}

fn activate_vote(final_vote: String, phase: &mut VotingPhase){
    let first_word = final_vote.split(" ").next().unwrap();

    if matches!(phase, VotingPhase::Regular) && first_word == "tower"{
        let second_word = final_vote.split(" ").nth(1).unwrap();
        // TODO: match this...
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

    // Placing a tower. Validating location because it could clash with the select command queuing into placement phase (I hate multithreading)
    else if matches!(phase, VotingPhase::Placement) && validate_location(final_vote.as_str()){
        let mouse = Mouse::new();
        let (x_dist, y_dist) = calculate_location(final_vote.as_str());
        mouse.move_to(x_dist, y_dist).expect("Unable to move mouse");

        // Sometimes it would click before moving, so I'll move the mouse first then click after a small delay

        // TODO: check if it got placed???
        // Force cancel by pushing the red X in C33 if the tower failed to get placed properly
        thread::spawn(|| {
            let mouse = Mouse::new();

            thread::sleep(Duration::from_millis(100));

            mouse.press(&Keys::LEFT).expect("Unable to press button");
            mouse.release(&Keys::LEFT).expect("Unable to release button");

            thread::sleep(Duration::from_millis(100));
            let x_dist = (f64::from(1920 / 40 * 33 + 25) / 1.25).floor() as i32;
            let y_dist = (f64::from(1080 / 24 * 3) / 1.25).floor() as i32;
            mouse.move_to(x_dist, y_dist).expect("Unable to move mouse");
            mouse.press(&Keys::LEFT).expect("Unable to press button");
            mouse.release(&Keys::LEFT).expect("Unable to release button");
        });

        *phase = VotingPhase::Regular;
        println!("Entering regular phase...");
    }

    // Shorter votes only
    else if matches!(phase, VotingPhase::Regular) && first_word == "select"{
        let second_word = final_vote.split(" ").nth(1).unwrap();

        let (x_dist, y_dist) = calculate_location(second_word);

        select_logic(x_dist, y_dist);
    }
}

// Need to de-select before selecting anything, needs a delay so I made a new thread.
fn select_logic(x_dist: i32, y_dist: i32){
    thread::spawn(move || {
        let mouse = Mouse::new();

        mouse.move_to(1920/2, 0).expect("Unable to move mouse");

        thread::sleep(Duration::from_millis(50));
        mouse.press(&Keys::LEFT).expect("Unable to press button");
        mouse.release(&Keys::LEFT).expect("Unable to release button");

        thread::sleep(Duration::from_millis(50));
        mouse.move_to(x_dist, y_dist).expect("Unable to move mouse");

        thread::sleep(Duration::from_millis(200));
        mouse.press(&Keys::LEFT).expect("Unable to press button");
        mouse.release(&Keys::LEFT).expect("Unable to release button");
    });
}