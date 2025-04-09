use bevy::prelude::*;

use crate::goods::{get_base_price, GoodType};
use crate::market::Market;
use crate::population::{PopClass, PopGroup};
use crate::time::NewDayEvent;

const EMPLOYEES_PER_LEVEL: u32 = 10000;

#[derive(Component, Clone)]
pub struct Production {
    pub id: u32,
    pub employees: Vec<(PopClass, u32)>,
    pub employees_type: Vec<PopClass>,
    pub level: u32,
    pub input_goods: Vec<(GoodType, f32)>, // Goods consumed per day per level
    pub output_goods: Vec<(GoodType, f32)>, // Goods produced per day per level
    pub province_id: u32,
}

#[derive(Component)]
pub struct Factory {
    pub efficiency: f32, // 0.0 - 1.0
}

#[derive(Component)]
pub struct Farm {
    pub efficiency: f32, // 0.0 - 1.0
}

pub fn add_factories(mut commands: Commands) {
    commands.spawn((
        Factory { efficiency: 0.5 },
        Production {
            id: 1,
            employees: vec![(PopClass::Craftsmen, 0)],
            employees_type: vec![PopClass::Craftsmen],
            level: 1,
            input_goods: vec![(GoodType::Fruit, 3.0)],
            output_goods: vec![(GoodType::Wine, 1.0)],
            province_id: 1,
        },
    ));
}

pub fn add_farms(mut commands: Commands) {
    commands.spawn((
        Farm { efficiency: 0.5 },
        Production {
            id: 2,
            employees: vec![(PopClass::Farmers, 0)],
            employees_type: vec![PopClass::Farmers],
            level: 1,
            input_goods: vec![],
            output_goods: vec![(GoodType::Fruit, 5.0)],
            province_id: 1,
        },
    ));

    commands.spawn((
        Farm { efficiency: 0.5 },
        Production {
            id: 3,
            employees: vec![(PopClass::Farmers, 0)],
            employees_type: vec![PopClass::Farmers],
            level: 1,
            input_goods: vec![],
            output_goods: vec![(GoodType::Grain, 5.0)],
            province_id: 1,
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

pub fn hiring_system(
    mut pop_query: Query<&mut PopGroup>,
    mut production_query: Query<&mut Production>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for mut production in production_query.iter_mut() {
            println!("production: {:?}", production.id);
            println!("production: {:?}", production.employees);
            let total_needed = production.level * EMPLOYEES_PER_LEVEL;
            let current: u32 = production.employees.iter().map(|(_, n)| *n).sum();

            if current < total_needed {
                for mut pop in pop_query.iter_mut() {
                    if pop.manpower_available == 0 {
                        continue;
                    }

                    if pop.province_id != production.province_id {
                        continue;
                    }

                    let prod_id = production.id;

                    for (class, value) in production.employees.iter_mut() {
                        if pop.class == *class {
                            if pop.manpower_available >= total_needed {
                                pop.manpower_available -= total_needed;
                                *value += total_needed;
                                pop.employers_id.push(prod_id);
                            } else {
                                *value += pop.manpower_available;
                                pop.manpower_available = 0;
                                pop.employers_id.push(prod_id);
                            }
                        }
                    }
                }
            }
        }
    }
}
