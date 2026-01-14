use rand::Rng; 
use std::io; 

// send a specified number of bitcoin
fn send_bitcoin() {
     println!("We will send some Bitcoin!\n" ); 

     // creating a vec! to parse through 
     let clients = vec!["Jill", "Mike", "Ike", "Moe"]; 

     for client in &clients{
        print!("{} ", client); 
     }
     // allowing user to pick from a person from vector 

     println!("\nWho would you like to send some bitcoin to?\n"); 

     let mut recipient = String::new(); 
     let _user_input = io::stdin().read_line(&mut recipient); 
     
     // parsing through the vector if the user writes a name from the list 
     if clients.contains(&recipient.trim()){
        println!("\nHow much coin do you want to send?"); 

     let mut amount = String::new(); 
     io::stdin().read_line(&mut amount);

     println!("You have sent {} bitcoin to recipient {:?}", amount, recipient);

     }else {
        println!("Please select a recipient within your client list!");
     }
}

// receive a random number of bitcoin and output amount of bitcoin received
fn receive_bitcoin() {
    let amount = rand::rng().random_range(1..20); 

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
