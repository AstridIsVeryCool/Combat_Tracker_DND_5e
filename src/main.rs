use std::io;
use combat_types::{Damage, DamageCategory};
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


    let a: combat_types::Attack = combat_types::Attack
    {
        to_hit_bonus: 3, 
        damage_categories: vec![z, g]
    };

    let b: Vec<Damage> = a.roll_damage();

    for i in 0..b.len()
    {
        println!("{:?}", b[i].damage_type_for_damage);
        println!("{}", b[i].amount);
    }

}
