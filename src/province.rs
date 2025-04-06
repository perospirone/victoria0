use bevy::prelude::*;

#[derive(Component)]
pub struct Province {
    pub id: u32,
    pub name: String,
    pub owner: String,
}

pub fn add_provincies(mut commands: Commands) {
    commands.spawn(Province {
        id: 1,
        name: "province1".to_string(),
        owner: "owner1".to_string(),
    });
}
