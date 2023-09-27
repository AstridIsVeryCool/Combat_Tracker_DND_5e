
use rand::Rng;
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
    fn roll_for_category(&self) -> Damage
    {
        let mut running_total = self.bonus;
        for x in 0..self.num_dice
        {
            running_total += rand::thread_rng().gen_range(1..=self.die);
        }
        return Damage { damage_type_for_damage: (self.category_type), amount: (running_total) }
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
    pub fn roll_damage(&self) -> Vec<Damage>
    {
        let mut damage_categories: Vec<Damage> = vec![];
        for i in &self.damage_categories
        {
            damage_categories.push(i.roll_for_category());
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
    fn take_damage(&mut self, damage_done: Damage) -> i32
    {
        let mut is_resistant: bool = false;
        for x in &self.resistances
        {
            is_resistant = is_resistant | (x == &damage_done.damage_type_for_damage);
        }

        if(is_resistant)
        {
            self.hitpoints -= damage_done.amount/2;
            return self.hitpoints;
        }

        self.hitpoints -= damage_done.amount;
        return self.hitpoints;
    }
}