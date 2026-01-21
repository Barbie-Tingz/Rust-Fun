// Build a Rock, Paper, Scissors game against the computer. 
// Play multiple rounds, keep score, and let the player decide 
// when to quit. 
 
use std::io; 
use rand::Rng;


// this function will be for the computer to randomly pick Rock, Paper, Scissors
fn opponent() -> &'static str {

    let comp_selection: u8 = rand::thread_rng().gen_range(1..=3); 

    let opp_select = 
    if comp_selection == 1 {
        "Rock"
    }else if comp_selection == 2 {
        "Paper"
    }else if comp_selection == 3 {
        "Scissors"
    }else{
        "Invalid option"
    };
    println!("\nYour opponent picked {}!", opp_select);
    opp_select
}

// this function is to keep score of who is winning
fn score() -> &'static str{
    //bring the other functions into scope and assign a variable
    let user_choice = console();
    let comp_choice = opponent();


    if comp_choice == user_choice {
        "Tie"
    }else if comp_choice == "Rock" && user_choice == "Paper"{
        "User Wins"
    }else if comp_choice == "Rock" && user_choice == "Scissors"{
        "Comp Wins"
    }else if comp_choice == "Paper" && user_choice == "Rock"{
        "Comp Wins"
    }else if comp_choice == "Paper" && user_choice == "Scissors"{
        "User Wins"
    }else if comp_choice == "Scissors" && user_choice == "Rock"{
        "User Wins"
    }else if comp_choice == "Scissors" && user_choice == "Paper"{
        "Comp Wins"
    }else{
        "Invalid"
    }
}
  
// this function is used to say who won
fn winner() {
}

fn console() -> &'static str {

    println!("\nReady?\n Rock\n Paper\n Scissors\n Shoot!\n"); 

// this is the user's guess "Rock, paper or scissors!"
    let mut user_input = String::new(); 
    io::stdin().read_line(&mut user_input).unwrap(); 

//match on the user's answer
    let user_select = match user_input.trim() {
            "Rock" => "Rock",
            "Paper" => "Paper",
            "Scissors" => "Scissors",
            _ => "Invalid",
    };
    println!("User picked {}!", user_select); 
    user_select
}

fn main(){

    println!("Let's play a game of Rock, Paper, Scissors!\n"); 


    for i in 1..=3 {
    println!("{}", score());
    }

}