use bevy::prelude::*;

use crate::time::NewDayEvent;
use crate::{GoodType, Market};

pub enum Culture {
    Brazilian,
}

pub enum Religion {
    Catholic,
}

pub enum PopClass {
    Farmers,
    Laborers,
    Craftsmen,
    Artisans,
    Aristocrat,
    Capitalist,
}

#[derive(Component)]
pub struct PopGroup {
    pub id: u32,
    pub size: u32,
    pub class: PopClass,
    pub culture: Culture,
    pub religion: Religion,
    pub literacy: f32,        // 0.0 - 1.0
    pub happiness: f32,       // 0.0 - 1.0
    pub political_power: f32, // 0.0 - 1.0
    pub money: f32,
    pub needs: Vec<(GoodType, f32)>, // goods needed(good_type, quantity per 1000 pops)
    pub province_id: i32,
}

pub fn add_pops(mut commands: Commands) {
    commands.spawn(PopGroup {
        id: 1,
        size: 1000,
        class: PopClass::Laborers,
        money: 0.0,
        needs: vec![(GoodType::Wine, 0.1), (GoodType::Grain, 0.2)],
        culture: Culture::Brazilian,
        religion: Religion::Catholic,
        literacy: 0.2,
        happiness: 1.0,
        political_power: 0.1,
        province_id: 1,
    });
}

pub fn population_consumption_system(
    mut market: ResMut<Market>,
    mut query: Query<&PopGroup>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for pop in query.iter() {
            for (good_type, quantity) in &pop.needs {
                let demand = quantity * pop.size as f32 / 1000.0;
                println!(
                    "good_type: {:?}, quantity: {:?}, demand: {:?}",
                    good_type, quantity, demand
                );

                if let Some((_, available_quantity, _)) =
                    market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                {
                    if *available_quantity >= demand {
                        *available_quantity -= demand;
                        println!("Pop {} consumed {:.4} of {:?}", pop.id, demand, good_type);
                    } else {
                        println!(
                            "Pop {} wants {:.4} of {:?} but only had {:.4}",
                            pop.id, demand, good_type, available_quantity
                        );
                        // maybe only consume what was available:
                        *available_quantity = 0.0;
                    }
                } else {
                    println!(
                        "Pop {} queria {:?}, mas não havia esse item no mercado!",
                        pop.id, good_type
                    );
                }
            }
        }
    }
}
