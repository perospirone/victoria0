use bevy::prelude::*;

use crate::goods::{get_base_price, GoodType};
use crate::market::Market;
use crate::money::Bank;
use crate::population::{PopClass, PopGroup};
use crate::time::NewDayEvent;

const EMPLOYEES_PER_LEVEL: u32 = 10000;

#[derive(Component, Clone)]
pub struct Production {
    pub id: u32,
    pub employees: Vec<(PopClass, u32, Vec<u32>, f32)>, // class, number of employes, pop_id/employee_id, salary
    pub employees_type: Vec<PopClass>,
    pub money: f32,
    pub level: u32,
    pub input_goods: Vec<(GoodType, f32)>, // Goods consumed per day per level
    pub output_goods: Vec<(GoodType, f32)>, // Goods produced per day per level(each 10k people)
    pub province_id: u32,
}

impl Production {
    pub fn add_money(&mut self, amount: f32) {
        self.money += amount;
    }

    pub fn remove_money(&mut self, amount: f32) {
        self.money -= amount;
    }
}

fn debit_production_money(production_id: u32, amount: f32) {
    todo!();
}

#[derive(Component)]
pub struct Factory {
    pub efficiency: f32, // 0.0 - 1.0
}

#[derive(Component)]
pub struct Farm {
    pub efficiency: f32, // 0.0 - 1.0
}

#[derive(Component)]
pub struct Mine {
    pub efficiency: f32, // 0.0 - 1.0
}

pub fn add_factories(mut commands: Commands) {
    commands.spawn((
        Factory { efficiency: 0.5 },
        Production {
            id: 1,
            employees: vec![(PopClass::Craftsmen, 0, vec![], 10.0)],
            employees_type: vec![PopClass::Craftsmen],
            money: 0.0,
            level: 1,
            input_goods: vec![(GoodType::Fruit, 3.0)],
            output_goods: vec![(GoodType::Wine, 1.0)],
            province_id: 1,
        },
    ));

    commands.spawn((
        Factory { efficiency: 0.5 },
        Production {
            id: 1,
            employees: vec![(PopClass::Craftsmen, 0, vec![], 10.0)],
            employees_type: vec![PopClass::Craftsmen],
            money: 0.0,
            level: 1,
            input_goods: vec![(GoodType::Grain, 2.0)],
            output_goods: vec![(GoodType::Liquor, 1.0)],
            province_id: 1,
        },
    ));
}

pub fn add_farms(mut commands: Commands) {
    commands.spawn((
        Farm { efficiency: 0.5 },
        Production {
            id: 2,
            employees: vec![(PopClass::Farmers, 0, vec![], 10.0)],
            employees_type: vec![PopClass::Farmers],
            money: 0.0,
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
            employees: vec![(PopClass::Farmers, 0, vec![], 10.0)],
            employees_type: vec![PopClass::Farmers],
            money: 0.0,
            level: 1,
            input_goods: vec![],
            output_goods: vec![(GoodType::Grain, 5.0)],
            province_id: 1,
        },
    ));
}

pub fn add_mines(mut commands: Commands) {
    commands.spawn((
        Mine { efficiency: 0.5 },
        Production {
            id: 3,
            employees: vec![(PopClass::Laborers, 0, vec![], 10.0)],
            employees_type: vec![PopClass::Laborers],
            money: 0.0,
            level: 1,
            input_goods: vec![],
            output_goods: vec![(GoodType::Gold, 2.0)],
            province_id: 1,
        },
    ));
}

pub fn production_system(
    mut market: ResMut<Market>,
    mut prod_sets: ParamSet<(
        Query<(&Factory, &Production)>,
        Query<(&Farm, &Production)>,
        Query<(&Mine, &mut Production)>,
    )>,
    mut bank_query: Query<&mut Bank>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    // TODO: i think that this function will need a refactor
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for (factory, production) in prod_sets.p0().iter() {
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

                let prod_manpower: u32 = production.employees.iter().map(|(_, n, _, _)| *n).sum();
                if prod_manpower == 0 {
                    can_produce = false;
                }
            }

            if can_produce {
                let total_employees: u32 = production.employees.iter().map(|(_, n, _, _)| *n).sum();
                let employee_factor = total_employees as f32 / EMPLOYEES_PER_LEVEL as f32;

                println!("total_employees: {:?}", total_employees);
                println!("employee_factor: {:?}", employee_factor);

                for (good_type, amount) in &production.input_goods {
                    let quantity_needed = amount * employee_factor;

                    println!("quantity_needed: {:?}", quantity_needed);

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty -= quantity_needed;
                    }
                }

                for (good_type, amount) in &production.output_goods {
                    let quantity_produced = (amount * employee_factor) * factory.efficiency;

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty += quantity_produced;
                    } else {
                        market.goods.push((
                            *good_type,
                            quantity_produced,
                            get_base_price(*good_type),
                        ));
                    }
                }
            }

            if !can_produce {
                println!("Factory couldn't produce: insufficient inputs");
            }
        }

        for (farm, production) in prod_sets.p1().iter() {
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
                let total_employees: u32 = production.employees.iter().map(|(_, n, _, _)| *n).sum();
                let employee_factor = total_employees as f32 / EMPLOYEES_PER_LEVEL as f32;

                for (good_type, amount) in &production.input_goods {
                    let quantity_needed = amount * employee_factor;

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty -= quantity_needed;
                    }
                }

                for (good_type, amount) in &production.output_goods {
                    let quantity_produced = (amount * employee_factor) * farm.efficiency;

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty += quantity_produced;
                    } else {
                        market.goods.push((
                            *good_type,
                            quantity_produced,
                            get_base_price(*good_type),
                        ));
                    }
                }
            }

            if !can_produce {
                println!("Farm couldn't produce: insufficient inputs");
            }
        }

        for (mine, mut production) in &mut prod_sets.p2().iter_mut() {
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
                let total_employees: u32 = production.employees.iter().map(|(_, n, _, _)| *n).sum();
                let employee_factor = total_employees as f32 / EMPLOYEES_PER_LEVEL as f32;

                for (good_type, amount) in &production.input_goods {
                    let quantity_needed = amount * employee_factor;

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty -= quantity_needed;
                    }
                }

                for (good_type, amount) in &production.output_goods {
                    let quantity_produced = (amount * employee_factor) * mine.efficiency;

                    println!("gold produced: {:?}", quantity_produced);

                    if *good_type == GoodType::Gold {
                        if let Ok(mut bank) = bank_query.get_single_mut() {
                            let minted_money = bank.mint_currency(quantity_produced);
                            production.money += minted_money;
                            println!("mine money: {:?}", production.money);

                            break;
                        }
                    }

                    if let Some((_, qty, _)) =
                        market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                    {
                        *qty += quantity_produced;
                    } else {
                        market.goods.push((
                            *good_type,
                            quantity_produced,
                            get_base_price(*good_type),
                        ));
                    }
                }
            }

            if !can_produce {
                println!("Mine couldn't produce: insufficient inputs");
            }
        }
    }
}

pub fn hiring_system(
    // todo: i need a better algorithm to hire pops
    mut pop_query: Query<&mut PopGroup>,
    mut production_query: Query<&mut Production>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for mut production in production_query.iter_mut() {
            // TODO: refactor this because this is
            // not optimized
            let total_needed: u32 = production.level * EMPLOYEES_PER_LEVEL;
            let current: u32 = production.employees.iter().map(|(_, n, _, _)| *n).sum();

            if current < total_needed {
                for mut pop in pop_query.iter_mut() {
                    if pop.manpower_available == 0 {
                        continue;
                    }

                    if pop.province_id != production.province_id {
                        continue;
                    }

                    let mut needed_class = false;

                    for item in &production.employees_type {
                        if *item == pop.class {
                            needed_class = true;
                            break;
                        }
                    }
                    if !needed_class {
                        continue;
                    }

                    let prod_id = production.id;

                    for (class, value, employees_id, _) in production.employees.iter_mut() {
                        if pop.class == *class {
                            if pop.manpower_available >= total_needed {
                                pop.manpower_available -= total_needed;
                                *value += total_needed;
                                pop.employers_id.push(prod_id);
                                employees_id.push(pop.id);
                            } else {
                                *value += pop.manpower_available;
                                pop.manpower_available = 0;
                                pop.employers_id.push(prod_id);
                                employees_id.push(pop.id);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn salary_system(
    mut pop_query: Query<&mut PopGroup>,
    mut production_query: Query<&mut Production>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for mut production in production_query.iter_mut() {
            for employee in production.employees.iter() {
                println!("{:?}", employee);

                for employee_id in employee.2.iter() {
                    println!("here4");
                    for mut pop in pop_query.iter_mut() {
                        if pop.id == *employee_id {
                            println!("found");
                            pop.receive_money(employee.3);

                            let id = production.id;
                            debit_production_money(id, employee.3);
                            //production.remove_money(employee.3);
                        }
                    }
                }
            }
        }
    }
}
