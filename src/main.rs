use std::{io, string};
use combat_types::{Damage, DamageCategory, Entity};
use log::debug;
#[allow(unused)]

mod combat_types;

fn main() 
{
    //let mut buffer = String::new();
    //io::stdin().read_line(&mut buffer);
    let mut console_input: String = String::new();
    let mut continue_running: bool = true;
    /*
    while continue_running
    {
        io::stdin().read_line(&mut console_input);

        if console_input == "create entity"
        {

        }

    }
    */
    create_entity();
}

//Add error handling, this shit will hard panic the instant anything goes wrong
//THIS IS EXREMELY UNSAFE
//BAD CODE AHEAD
fn create_entity() //-> Entity
{
    let mut user_input = String::new();

    println!("Is the entity a player?: y/n");
    io::stdin().read_line(&mut user_input);
    let is_player: bool = (user_input == "y\r\n") || (user_input == "Y\r\n");

    println!("How many hit points does it currently have?: enter a positive integer");
    io::stdin().read_line(&mut user_input);
    let current_hit_points: i32 = user_input.parse().unwrap();

    println!("What is its maximum hit points?: enter a positive integer");
    io::stdin().read_line(&mut user_input);
    let maximum_hit_points: i32 = user_input.parse().unwrap();

    println!("What are its maximum hit points?: enter a positive integer");
    io::stdin().read_line(&mut user_input);
    let maximum_hit_points: i32 = user_input.parse().unwrap();

    println!("What are its temporary hit points?: enter a positive integer");
    io::stdin().read_line(&mut user_input);
    let temporary_hit_points: i32 = user_input.parse().unwrap();

    println!("What is its armor class?: enter a positive integer");
    io::stdin().read_line(&mut user_input);
    let ac: i32 = user_input.parse().unwrap();


}