// in this moment Good component and good entities are not being used but i think that this will
// work for now, then i think that for now i will keep this but with the thought of change this
// later, and change this approach for other

use bevy::prelude::*;

#[derive(Component)]
pub struct Good {
    pub id: u32,
    pub name: GoodType,
    pub base_price: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GoodType {
    Wine,
    Grain,
    Fruit,
}

pub fn get_base_price(good: GoodType) -> f32 {
    match good {
        GoodType::Wine => 9.7,
        GoodType::Grain => 2.2,
        GoodType::Fruit => 1.8,
    }
}

pub fn get_good_name(good: GoodType) -> String {
    let good_name = match good {
        GoodType::Wine => "Wine",
        GoodType::Grain => "Grain",
        GoodType::Fruit => "Fruit",
    };

    good_name.to_string()
}

pub fn add_goods(mut commands: Commands) {
    // make a vector to all goods and only iterate can be a good idea
    commands.spawn(Good {
        id: 1,
        name: GoodType::Wine,
        base_price: get_base_price(GoodType::Wine),
    });
    commands.spawn(Good {
        id: 2,
        name: GoodType::Grain,
        base_price: get_base_price(GoodType::Grain),
    });
    commands.spawn(Good {
        id: 3,
        name: GoodType::Fruit,
        base_price: get_base_price(GoodType::Fruit),
    });
}
