use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("input the miner address: ");
    io::stdout().flush().expect("fail to flush");
    io::stdin()
        .read_line(&mut miner_addr)
        .expect("fail to read line");
    println!("input the difficulty: ");
    io::stdout().flush().expect("fail to flush");
    io::stdin()
        .read_line(&mut difficulty)
        .expect("fail to read line");
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block! ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("5) Print specific block");
        println!("6) Print all transactions");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("fail to flush");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("fail to read line");
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut sender)
                    .expect("fail to read line");
                println!("Enter receiver address: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut receiver)
                    .expect("fail to read line");
                println!("Enter amount: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut amount)
                    .expect("fail to read line");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();

                println!("Enter new difficulty: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut new_diff)
                    .expect("fail to read line");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Difficulty updated"),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                let mut reward = String::new();

                println!("Enter new reward: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut reward)
                    .expect("fail to read line");
                let res = chain.update_reward(reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward updated"),
                    false => println!("Failed to update reward"),
                }
            }
            5 => {
                let mut index = String::new();

                println!("Enter number of the block: ");
                io::stdout().flush().expect("fail to flush");
                io::stdin()
                    .read_line(&mut index)
                    .expect("fail to read line");
                chain.print_specific_block(index.trim().parse::<usize>().unwrap());
            }
            6 => chain.print_chain_transactions(),
            _ => println!("\tinvalid option, retry please\t"),
        }
    }
}
