mod combat_types;

fn main() 
{
    let a: combat_types::Attack = combat_types::Attack
    {
        to_hit_bonus: 3, 
        damage_categories: vec![]
    };

    //let b: Damage = a.roll_damage();

    //println!("{:?}", b.damage_type_for_damage);
    //println!("{}", b.amount);
}
