use std::collections::HashMap;
use mki::Keyboard;
use mouse_rs::{Mouse, types::keys::Keys};

use std::thread;
use std::time::Duration;

pub enum VotingPhase{
    Regular,
    Placement
}

// Validate this is proper vote syntax
pub fn validate_vote(message_text: &str, phase: &VotingPhase) -> bool{
    let message_text: &str = &message_text.to_lowercase();

    // Choose a tower (ex: tower bomb)
    if matches!(phase, VotingPhase::Regular) && message_text.starts_with("tower "){
        let second_word = message_text.split(" ").nth(1).unwrap();
        match second_word{
            "hero" | "dart" | "boomerang" | "bomb" | "tack" | "ice"
            | "glue" | "sniper" | "sub" | "buccaneer" | "ace" | "heli"
            | "mortar" | "gunner" | "wizard" | "super" | "ninja"
            | "alchemist" | "druid" | "farm" | "factory" | "village"
            | "engineer" => { return true; },
            _ => {}
        }
    }

    // Place a tower (ex: a24)
    else if matches!(phase, VotingPhase::Placement) && validate_location(message_text){
        return true;
    }

    // Upgrade a tower (ex: upgrade f17)
    else if matches!(phase, VotingPhase::Regular) && message_text.starts_with("upgrade "){
        let second_word = message_text.split(" ").nth(1).unwrap();
        
        match second_word{
            "1" | "2" | "3" => { return true; }
            _ => {}
        }
    }

    // Select
    else if matches!(phase, VotingPhase::Regular) && message_text.starts_with("select "){
        let second_word = message_text.split(" ").nth(1).unwrap();

        if validate_location(second_word){
            return true;
        }
    }

    // Start / Speed up and down, sell, targeting
    else if matches!(phase, VotingPhase::Regular) && (message_text == "start" || message_text == "speed" 
    || message_text == "sell" || message_text == "targeting"){
        return true;
    }

    else if matches!(phase, VotingPhase::Regular) && message_text.starts_with("ability "){
        let second_word = message_text.split(" ").nth(1).unwrap();

        if second_word.chars().next().unwrap().is_numeric(){
            return true;
        }
    }
    return false;
}

pub fn add_vote(votes: &mut HashMap<String, String>, user_id: String, message_text: String){
    println!("VOTE: {}", message_text.to_lowercase());
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

        match second_word{
            "hero" => { Keyboard::U.click(); },
            "dart" => { Keyboard::Q.click(); },
            "boomerang" => { Keyboard::W.click(); },
            "bomb" => { Keyboard::E.click(); },
            "tack" => { Keyboard::R.click(); },
            "ice" => { Keyboard::T.click(); },
            "glue" => { Keyboard::Y.click(); },
            "sniper" => { Keyboard::Z.click(); },
            "sub" => { Keyboard::X.click(); },
            "buccaneer" => { Keyboard::C.click(); },
            "ace" => { Keyboard::V.click(); },
            "heli" => { Keyboard::B.click(); },
            "mortar" => { Keyboard::N.click(); },
            "gunner" => { Keyboard::M.click(); },
            "wizard" => { Keyboard::A.click(); },
            "super" => { Keyboard::S.click(); },
            "ninja" => { Keyboard::D.click(); },
            "alchemist" => { Keyboard::F.click(); },
            "druid" => { Keyboard::G.click(); },
            "farm" => { Keyboard::H.click(); },
            "engineer" => { Keyboard::L.click(); },
            "factory" => { Keyboard::J.click(); },
            "village" => { Keyboard::K.click(); },
            _ => {}
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

    // Upgrade a tower
    else if matches!(phase, VotingPhase::Regular) && first_word == "upgrade"{
        // The code is the same as selecting a tower, but we change the phase
        let second_word = final_vote.split(" ").nth(1).unwrap();

        match second_word{
            "1" => { Keyboard::Comma.click(); *phase = VotingPhase::Regular; },
            "2" => { Keyboard::Period.click(); *phase = VotingPhase::Regular; },
            "3" => { Keyboard::Slash.click(); *phase = VotingPhase::Regular; },
            _ => {}
        }
    }

    // Selecting a tower
    else if matches!(phase, VotingPhase::Regular) && first_word == "select"{
        let second_word = final_vote.split(" ").nth(1).unwrap();

        let (x_dist, y_dist) = calculate_location(second_word);

        select_logic(x_dist, y_dist);
    }

    else if matches!(phase, VotingPhase::Regular) && (first_word == "start" || first_word == "speed"){
        Keyboard::Space.click();
    }

    else if matches!(phase, VotingPhase::Regular) && first_word == "sell"{
        Keyboard::BackSpace.click();
    }

    else if matches!(phase, VotingPhase::Regular) && first_word == "targeting"{
        Keyboard::Tab.click();
    }

    else if matches!(phase, VotingPhase::Regular) && first_word == "ability"{
        let second_word = final_vote.split(" ").nth(1).unwrap();

        match second_word{
            "1" => { Keyboard::Number1.click(); },
            "2" => { Keyboard::Number2.click(); },
            "3" => { Keyboard::Number3.click(); },
            "4" => { Keyboard::Number4.click(); },
            "5" => { Keyboard::Number5.click(); },
            "6" => { Keyboard::Number6.click(); },
            "7" => { Keyboard::Number7.click(); },
            "8" => { Keyboard::Number8.click(); },
            "9" => { Keyboard::Number9.click(); },
            _ => {}
        }
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