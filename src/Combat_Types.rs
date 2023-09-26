
use rand::Rng;
//Enumeration representing the damage type of damage
#[derive(Copy, Clone, Debug)]
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
    category_type: DamageType,
    die: i32,
    num_dice: i32,
    bonus: i32
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

pub struct Damage
{
    pub damage_type_for_damage: DamageType,
    pub amount: i32,
}

pub struct Attack
{
    pub to_hit_bonus: i32,
    pub damage_categories: Vec<DamageCategory>
}

impl Attack
{
    //fn roll_damage(&self) -> Damage
    //{
        
    //}
    
    fn roll_to_hit(&self, ac: &i32) -> bool
    {
        let roll = rand::thread_rng().gen_range(1..=20);
        return (roll + self.to_hit_bonus) >= *ac;
    }
}