
// Nothing is implemented yet this is just a base for a quick rust cli I want to make for my
// private website

mod upload;
use std::env;
use std::path::Path;

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
                 cmcli status - checks the status of the cameron.media api\n
                 cmcli statusraw - checks the status of the cameron.media api but returns either 1 for online or 0 for offline\n
                 cmcli upload <file> <key> - uploads a file to the cameron.media api\n")
    }
    else if command == "version" {
        println!("You are running version {}", env!("CARGO_PKG_VERSION")) 
    } else if command == "status" {
        match reqwest::blocking::get("http://cameron.media/api/status") {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(text) => println!("Status request responded with {}", text),
                        Err(_) => println!("Api is down"),
                    }
                } else {
                    println!("Api is down");
                }
            }
            Err(_) => println!("Api is offline")
        }
    } else if command == "statusraw" {
        match reqwest::blocking::get("http://cameron.media/api/status") {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.text() {
                        Ok(_) => println!("Online"),
                        Err(_) => println!("Offline"),
                    }
                } else {
                    println!("Offline");
                }
            }
            Err(_) => println!("Offline")
        }
    } else if command == "upload" {
        
        if args.len() < 4 {
            println!("Invalid usage, specify a file and a key");
            return;
        }

        if !Path::new(&args[2]).exists() {
            println!("Unknown file {}", args[2]);
            return;
        }
        
        upload::upload_file(&args[3], &args[2]);

    } else {
        println!("Invalid command please use the help command to see available commands");
    }
    
}
