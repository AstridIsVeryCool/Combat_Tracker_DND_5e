use std::{io, string};
use combat_types::{Damage, DamageCategory, Entity};
mod combat_types;

fn main() 
{
    

    let z: DamageCategory = DamageCategory
    {
        category_type: combat_types::DamageType::Piercing,
        die: 8,
        bonus: 2,
        num_dice: 1
    };

    let g: DamageCategory = DamageCategory
    {
        category_type: combat_types::DamageType::Fire,
        die: 6,
        bonus: 0,
        num_dice: 2
    };

    let w: String = "big nae nae whip and dab moment".to_string();

    let a: combat_types::Attack = combat_types::Attack
    {
        name: w,
        to_hit_bonus: 3, 
        damage_categories: vec![z, g]
    };

    let mut test_entity: Entity = Entity 
    { 
        is_player: true,
        hitpoints: 12,
        maximum_hitpoints: 12,
        temporary_hitpoints: 0,
        armour_class: 13,
        attacks: (vec![a]),
        resistances: (vec![]) 
    };

    let q: String = "big nae nae whip and dab moment".to_string();

    let mut sex_mode: Vec<Damage> = test_entity.roll_damage(&q);

    println!("{:?}", sex_mode[1].damage_type_for_damage);

    println!("{}", sex_mode[1].amount);   

    println!("{:?}", sex_mode[0].damage_type_for_damage);

    println!("{}", sex_mode[0].amount);  
}