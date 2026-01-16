// Build a Rock, Paper, Scissors game against the computer. 
// Play multiple rounds, keep score, and let the player decide 
// when to quit. 
 
use std::io; 
use rand::Rng;


// this function will be for the computer to randomly pick Rock, Paper, Scissors
fn opponent() {

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

println!("Your opponent picked {}!", opp_select); 
}

// this function is to keep score of who is winning
fn score() {
    let comp_score = opponent(&opp_select); 
    let user_score = console(&user_select); 

}
  
// this function is used to say who won
fn winner() {

}

fn console() {
    println!("Let's play a game of Rock, Paper, Scissors!\n"); 
    println!("Ready?\n Rock\n Paper\n Scissors\n Shoot!\n"); 


// this is the user's guess "Rock, paper or scissors!"
    let mut user_input = String::new(); 
    io::stdin().read_line(&mut user_input).unwrap(); 

//match on the user's answer
    let user_select = match user_input.trim() {
            "Rock" => "Rock",
            "Paper" => "Paper",
            "Scissors" => "Scissors!",
            _ => "Invalid",
    };
    println!("User picked {}!", answer); 
}

fn main(){
console();
opponent();

}