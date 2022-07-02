// Functionality for the application

use colored::Colorize;
fn prompt(){
    println!("{}{}{} # ","[".red(),"pmngr".green(),"]".red());
}
pub fn run(){
    use std::io;
    // main function that runs
    let mut cmd : String = String::new();
    let mut cmd : String = io::stdin().read_line(&mut cmd).unwrap().to_string();

    if cmd.as_str() == "exit" {
        println!("Hurray")
    }
}