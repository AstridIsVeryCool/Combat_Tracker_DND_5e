use std::io;
use rand::Rng;
mod Combat_Types;

fn main() {
    let a: Combat_Types::Attack = Attack
    {
        to_hit_bonus: 3, 
    };

    let b: Damage = a.roll_damage();

    println!("{:?}", b.Combat_types::damage_type_for_damage);
    println!("{}", b.Combat_types::amount);
}
