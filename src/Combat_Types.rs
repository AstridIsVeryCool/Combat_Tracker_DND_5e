use std::vec;
use rand::Rng;
#[allow(unused)]
//Enumeration representing the damage type of damage
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DamageType
{
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder
}

impl DamageType
{
    fn string_to_damage_type(input: &String) -> Result<DamageType,Err()>
    {
        let input_lowercase = &input.to_lowercase();
        match input_lowercase
        {
            String::from("acid") => Ok(DamageType::Acid),
            String::from("bludgeoning") => Ok(DamageType::Bludgeoning),
            String::from("cold") => Ok(DamageType::Cold),
            String::from("fire") => Ok(DamageType::Fire),
            String::from("force") => Ok(DamageType::Force),
            String::from("lightning") => Ok(DamageType::Lightning),
            String::from("necrotic") => Ok(DamageType::Necrotic),
            String::from("piercing") => Ok(DamageType::Piercing),
            String::from("poison") => Ok(DamageType::Poison),
            String::from("psychic") => Ok(DamageType::Psychic),
            String::from("radiant") => Ok(DamageType::Radiant),
            String::from("slashing") => Ok(DamageType::Slashing),
            String::from("thunder") => Ok(DamageType::Thunder),
            _ => Err("Invalid Damage Type")
        }
    }
}

//represents the amount of damage of a certain type that could potentially be dealt
pub struct DamageCategory
{
    pub category_type: DamageType,
    pub die: i32,
    pub num_dice: i32,
    pub bonus: i32
}
impl DamageCategory
{
    //Calculates the damage done in that category and returns it as an instance of Damage
    fn roll_for_category(&self, is_critical_hit: bool) -> Damage
    {
        let mut running_total = self.bonus;
        for x in 0..self.num_dice
        {
            running_total += rand::thread_rng().gen_range(1..=self.die);
        }
        return Damage { damage_type_for_damage: (self.category_type), amount: (running_total * !is_critical_hit as i32) }
    }
}

#[derive(Debug)]
pub struct Damage
{
    pub damage_type_for_damage: DamageType,
    pub amount: i32,
}

pub struct Attack
{
    pub name: String,
    pub to_hit_bonus: i32,
    pub damage_categories: Vec<DamageCategory>
}


impl Attack
{
    pub fn roll_damage(&self, is_critical_hit: bool) -> Vec<Damage>
    {
        let mut damage_categories: Vec<Damage> = vec![];
        for i in &self.damage_categories
        {
            damage_categories.push(i.roll_for_category(is_critical_hit));
        }
        return damage_categories;
    }
    pub fn roll_to_hit(&self, ac: &i32) -> bool
    {
        let roll = rand::thread_rng().gen_range(1..=20);
        return (roll + self.to_hit_bonus) >= *ac;
    }
}

//represents a combatant
pub struct Entity
{
    pub is_player: bool,
    pub name: String,
    pub hitpoints: i32,
    pub maximum_hitpoints: i32,
    pub temporary_hitpoints: i32,
    pub armour_class: i32,
    pub attacks: Vec<Attack>,
    pub resistances: Vec<DamageType>
}

impl Entity
{
    //takes damage done and damage type and returns the current hitpoints of the combatant
    //ADD TEMPORARY HIT POINTS
    fn take_single_damage(&mut self, damage_done: Damage)
    {
        let mut is_resistant: bool = false;
        for x in &self.resistances
        {
            is_resistant = is_resistant | (x == &damage_done.damage_type_for_damage);
        }
        self.hitpoints -= damage_done.amount * 0.5.pow(is_resistant as i32);
    }
    pub fn take_damage(&mut self, damages: Vec<Damage>)
    {
        for x in damages
        {
            &self.take_single_damage(x);
        }
    }
    //ADD ERROR HANDLING
    pub fn attack(&self, attack_name: &String, is_critical_hit: bool) ->Vec<Damage>
    {
        let mut attack = &self.attacks[0];
        for x in &self.attacks
        {
            if &x.name == attack_name
            {
                attack = x;
                break;
            }
        }
        return attack.roll_damage(is_critical_hit);
    }
}