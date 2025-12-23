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
    Grain,
    Fruit,
    Liquor,
    Wine,
    Gold,
    Silver,
}

pub fn get_base_price(good: GoodType) -> f32 {
    match good {
        GoodType::Wine => 9.7,
        GoodType::Liquor => 6.4,
        GoodType::Grain => 2.2,
        GoodType::Fruit => 1.8,
        GoodType::Gold => 8.0,
        GoodType::Silver => 4.0,
    }
}

pub fn get_good_name(good: GoodType) -> String {
    let good_name = match good {
        GoodType::Wine => "Wine",
        GoodType::Liquor => "Liquor",
        GoodType::Grain => "Grain",
        GoodType::Fruit => "Fruit",
        GoodType::Silver => "Silver",
        GoodType::Gold => "Gold",
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

    commands.spawn(Good {
        id: 4,
        name: GoodType::Liquor,
        base_price: get_base_price(GoodType::Liquor),
    });

    commands.spawn(Good {
        id: 5,
        name: GoodType::Gold,
        base_price: get_base_price(GoodType::Gold),
    });

    commands.spawn(Good {
        id: 6,
        name: GoodType::Silver,
        base_price: get_base_price(GoodType::Silver),
    });
}
