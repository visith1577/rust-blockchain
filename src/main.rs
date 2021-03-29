#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_adr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input miner address : ");
    io::stdout().flush().expect("error occurred while @miner_address");
    io::stdin().read_line(&mut miner_adr).expect("unable to read Input");
    println!("Input difficulty : ");
    io::stdout().flush().expect("error occurred while @difficulty");
    io::stdin().read_line(&mut difficulty).expect("unable to read Input");
    let diff = difficulty.trim().parse::<u32>().expect("difficulty should be a integer");
    println!("Generating genesis block...");
    let mut chain = blockchain::Chain::new(miner_adr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New transaction");
        println!("2) New block");
        println!("3) Change difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter new Choice : ");
        io::stdout().flush().expect("error @menu");
        choice.clear();
        io::stdin().read_line(&mut choice).expect("unable to read Input");
        println!();


        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting process");
                process::exit(0);
            }

            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("enter sender address: ");
                io::stdout().flush().expect("error @sender_addr");
                io::stdin().read_line(&mut sender).expect("unable to read Input");
                print!("enter receiver address: ");
                io::stdout().flush().expect("error @receiver_addr");
                io::stdin().read_line(&mut receiver).expect("unable to read Input");
                print!("Enter amount: ");
                io::stdout().flush().expect("error @amount");
                io::stdin().read_line(&mut amount).expect("unable to read Input");
                let res = chain.new_transaction(sender.trim().to_string(),
                                                receiver.trim().to_string(),
                                                amount.trim().parse::<f32>().unwrap());
                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }

            2 => {
                println!("Generating new block!");
                let res = chain.generate_new_block();
                match res {
                    true => println!("New block added"),
                    false => println!("New block failed"),
                }
            }

            3 => {
                let mut new_diff = String::new();
                println!("Enter new difficulty: ");
                io::stdout().flush().expect("error @difficulty");
                io::stdin().read_line(&mut new_diff).expect("unable to read input");
                let  res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("New difficulty updated"),
                    false => println!("failed to update difficulty"),
                }
            }

            4 => {
                let mut new_reward = String::new();
                println!("Enter new reward: ");
                io::stdout().flush().expect("error @reward");
                io::stdin().read_line(&mut new_reward).expect("unable to read Input");
                let  res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated Reward"),
                    false => println!("failed to update Reward"),
                }
            }
            _ => println!("\tInvalid option please try again\t")
        }
    }
}
