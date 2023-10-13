use std::{io, string};
use std::io::Read;
use combat_types::{Damage, DamageCategory, Entity};
use log::debug;
use crate::combat_types::{Attack, DamageType, string_to_damage_type};

#[allow(unused)]

mod combat_types;
mod round_tracker;

fn main() 
{
    let mut running = true;
    let mut entities: Vec<combat_types::Entity> = vec![];


    while running
    {
        if !&entities.is_empty()
        {
            for i in &mut entities
            {
                println!("{}:", i.name);
                println!("   Current HP:{}", i.hitpoints);
                println!("   Max hp:{}", i.maximum_hitpoints);
                println!("   Armor Class:{}", i.armour_class);
                println!();
            }
        }
        println!("What do?");
        let mut invalid_choice = true;

        while invalid_choice
        {
            invalid_choice = false;
            match read_from_console().to_lowercase().as_ref()
            {
                "create entity" => entities.push(create_entity()),
                "quit" => running = false,
                _ => {
                    println!("please enter a valid choice");
                    invalid_choice = false;
                },
            }
        }
    }
}


fn create_entity() -> Entity
{
    let mut user_input = String::new();

    println!("What is the entity's name?: ");
    let name = read_from_console();

    println!("Is the entity a player?: y/n");
    user_input = read_from_console();
    let is_player: bool = user_input.to_uppercase() == "Y";

    println!("How many hit points does it currently have?: enter a positive integer");
    let current_hit_points: i32 = read_integer_from_console();

    println!("What is its maximum hit points?: enter a positive integer");
    let maximum_hit_points: i32 = read_integer_from_console();

    println!("What are its temporary hit points?: enter a positive integer");
    let temporary_hit_points: i32 = read_integer_from_console();

    println!("What is its armor class?: enter a positive integer");
    let ac: i32 = read_integer_from_console();

    println!("Creating attacks...");
    let attacks = create_attack_vec();

    println!("Loading resistances, enter them one at a time...");
    let resistances = read_resistances_from_console();

    return combat_types::Entity{name, is_player, hitpoints: current_hit_points, maximum_hitpoints: maximum_hit_points,
    temporary_hitpoints: temporary_hit_points, armour_class: ac, attacks, resistances, intitiative: 0};
}

fn create_attack_vec() -> Vec<Attack>
{
    let mut user_input = String::new();

    let mut more_attacks: bool = true;

    let mut attacks: Vec<combat_types::Attack> = vec![];
    while more_attacks
    {
        println!("What is the name of this attack?:");
        let attack_name = read_from_console();
        //dbg!(attack_name);

        println!("What is the to hit bonus of this attack?");
        let hit_bonus: i32 = read_integer_from_console();

        let attack_damage_categories   = create_damage_categories();

        println!("Is there another attack?: y/n");
        user_input = read_from_console();
        more_attacks = (user_input.to_uppercase() == "Y");




        attacks.push(Attack{name: attack_name,
            to_hit_bonus: hit_bonus, damage_categories: attack_damage_categories})

    }
    attacks
}

fn read_damage_type_from_console() -> DamageType
{
    let mut user_input: String;
    let mut is_valid: bool = false;

    user_input = "".to_string();
    let mut damage_type: DamageType = DamageType::Slashing;
    while !is_valid
    {
        user_input = read_from_console();
        match string_to_damage_type(&user_input)
        {
            Ok(type_of_damage) => {is_valid = true; damage_type = type_of_damage},
            Err(e) => println!("Invalid damage type, please enter a damage type:"),
        };
    }
    return damage_type;
}

fn create_damage_categories() -> Vec<DamageCategory>
{
    let mut another_damage_category = true;
    let mut damage_categories: Vec<DamageCategory> = vec![];
    let mut user_input = String::new();

    while another_damage_category
    {
        println!("What is the damage bonus in this category?:");
        let damage_bonus: i32 = read_integer_from_console();

        println!("What is the die of this damage category?:");
        let damage_die: i32 = read_integer_from_console();

        println!("What is the number of dice in this category?:");
        let num_dice = read_integer_from_console();

        println!("What is the damage type in this category?:");
        let damage_type = read_damage_type_from_console();

        println!("Is there another damage category in this attack? y/n:");
        another_damage_category = read_from_console().to_uppercase() == "Y";

        damage_categories.push(DamageCategory{bonus: damage_bonus, die: damage_die,
            num_dice, category_type: damage_type})

    }
    damage_categories

}

fn read_integer_from_console() -> i32
{

    let mut input_integer: i32;
    let mut valid_result = false;
    let mut validated_integer: i32 = 0;

    while !valid_result
    {
        let input = read_from_console();
        match input.trim().parse::<i32>()
        {
            Ok(int) => {validated_integer = int; valid_result = true},
            Err(E) => println!("Please enter a valid integer: "),
        }
    }
    validated_integer
}

fn read_from_console() -> String {
    let mut user_input = String::new();
    let mut validated_input = String::new();
    let mut valid_result = false;

    while !valid_result
    {
        match io::stdin().read_line(&mut user_input)
        {
            Ok(input) => { validated_input = String::from(&user_input[0..user_input.len()-2]); valid_result = true; },
            Err(n) => println!("Invalid input: please enter a valid value:")
        }
    }
    validated_input
}

fn read_resistances_from_console() -> Vec<combat_types::DamageType>
{
    let mut another_damage_type:bool;
    let mut resistances:Vec<DamageType> = vec![];

    println!("Are there resistances? y/n:");
    let user_input = read_from_console();
    another_damage_type = user_input.to_uppercase() == "Y";

    while another_damage_type
    {
        println!("What is the damage type?: ");
        resistances.push(read_damage_type_from_console());

        println!("Is there another damage type? y/n:");
        let user_input = read_from_console();
        another_damage_type = user_input.to_uppercase() == "Y";
    }
    resistances
}