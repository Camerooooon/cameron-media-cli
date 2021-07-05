
// Nothing is implemented yet this is just a base for a quick rust cli I want to make for my
// private website

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("You didn't specify a command, use the help command to see available commands");
        return;
    }

    let command = args[1].clone();

    if command == "help" {
        println!("
                 cmcli help - opens this help page\n
                 cmcli version - gets the version\n
                 cmcli key <key> - checks the status of your key\n
                 cmcli status - checks the status of the cameron.media api\n
                 cmcli statusraw - checks the status of the cameron.media api but returns either 1 for online or 0 for offline\n
                 cmcli upload <file> <key> - uploads a file to the cameron.media api\n")
    }
    else if command == "version" {
        println!("You are running version {}", env!("CARGO_PKG_VERSION")) 
    } else {
        println!("Invalid command please use the help command to see available commands");
    }
    
}
