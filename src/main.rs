use std::{io, string};
use combat_types::{Damage, DamageCategory, Entity};
mod combat_types;

fn main() 
{
    //let mut buffer = String::new();
    //io::stdin().read_line(&mut buffer);
    let mut console_input: String = String::new();
    let mut continue_running: bool = true;
    while continue_running
    {
        io::stdin().read_line(&mut console_input);

        if console_input == "create entity"
        {

        }

    }

}

fn create_entity() -> Entity
{
    let mut user_input = String::new();
    println!("Is the entity a player?: y/n");
    io::stdin().read_line(&mut user_input);
    let is_player: bool = user_input == "y";


}