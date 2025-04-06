use bevy::prelude::*;

use crate::goods::{get_base_price, GoodType};
use crate::market::Market;
use crate::time::NewDayEvent;

#[derive(Component)]
pub struct Production {
    pub input_goods: Vec<(GoodType, f32)>, // Goods consumed per day
    pub output_goods: Vec<(GoodType, f32)>, // Goods produced per day
}

#[derive(Component)]
pub struct Factory {
    pub efficiency: f32,  // 0.0 - 1.0
    pub province_id: u32, // province owner
}

pub fn add_factories(mut commands: Commands) {
    commands.spawn((
        Factory {
            efficiency: 0.5,
            province_id: 1,
        },
        Production {
            input_goods: vec![(GoodType::Fruit, 5.0)],
            output_goods: vec![(GoodType::Wine, 1.0)],
        },
    ));
}

pub fn production_system(
    mut market: ResMut<Market>,
    query: Query<(&Factory, &Production)>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for (factory, production) in query.iter() {
            let mut can_produce = true;
            for (good_type, amount) in &production.input_goods {
                if let Some((_, qty, _)) =
                    market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                {
                    if *qty < *amount {
                        can_produce = false;
                        break;
                    }
                } else {
                    can_produce = false;
                    break;
                }
            }

            if can_produce {
                for (good_type, amount) in &production.input_goods {
                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty -= amount;
                    }
                }

                for (good_type, amount) in &production.output_goods {
                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty += amount * factory.efficiency;
                    } else {
                        market.goods.push((
                            *good_type,
                            amount * factory.efficiency,
                            get_base_price(*good_type),
                        ));
                    }
                }
            }

            if !can_produce {
                println!("Factory couldn't produce: insufficient inputs");
            }
        }
    }
}
