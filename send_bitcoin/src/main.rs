use rand::Rng; 
use std::io; 

// send a specified number of bitcoin
fn send_bitcoin() {
     println!("We will send {} Bitcoin!" ); 

     // creating a vec! to parse through 
     let clients = vec!["Jill", "Mike", "Ike", "Moe"]; 

     for client in &clients{
        print!("{}", client); 
     }
     // allowing user to pick from a person from vector 
     let mut recipient = String::new(); 
     io::stdin().read_line(&mut recipient); 
}

// receive a random number of bitcoin and output amount of bitcoin received
fn receive_bitcoin() {
    let amount = rand::thread_rng().random_range(1..20); 

    println!("You just received {} Bitcoin!", amount)
}

// exit console 
fn exit_console(){
    println!("You must write 's' or 'r' to receive bitcoin")
}

// create a function that allows user input

fn console() {

    println!("Would you like to send(s) or receive(r) bitcoin?\n"); 

    let mut user_input = String::new(); 

    io::stdin().read_line(&mut user_input).unwrap(); 

   if user_input.trim().eq("r") {
    receive_bitcoin();
   }
   else if user_input.trim().eq("s") {
    send_bitcoin();
   }
   else{
    exit_console(); 
   }
}


fn main() {
  console()  
}
