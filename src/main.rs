use bevy::prelude::*;

#[derive(Resource)]
struct TimeTracker {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub speed: u8,         // todo: implement speed (1-5) in advance time system
    pub elapsed_time: f32, // Accumulates delta time
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self {
            day: 0,
            month: 1,
            year: 1830,
            speed: 1,
            elapsed_time: 0.0,
        }
    }
}

#[derive(Event)]
pub struct NewDayEvent {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

fn advance_time(
    mut time_tracker: ResMut<TimeTracker>,
    time: Res<Time>,
    mut new_day_ev: EventWriter<NewDayEvent>,
) {
    time_tracker.elapsed_time += time.delta_secs(); // Accumulate real-time

    if time_tracker.elapsed_time >= 1.0 {
        // One game day passes every second
        time_tracker.elapsed_time = 0.0;
        time_tracker.day += 1;

        new_day_ev.send(NewDayEvent {
            day: time_tracker.day,
            month: time_tracker.month,
            year: time_tracker.year,
        });

        if time_tracker.day > 30 {
            // Example: 30 days per month
            time_tracker.day = 1;
            time_tracker.month += 1;
        }

        if time_tracker.month > 12 {
            time_tracker.month = 1;
            time_tracker.year += 1;
        }

        println!(
            "Game Date: {}-{}-{}",
            time_tracker.day, time_tracker.month, time_tracker.year
        );
    }
}

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

#[derive(Resource)]
pub struct Market {
    pub goods: Vec<(GoodType, f32, f32)>, // (type, quantity available, actual price)
}

#[derive(Component)]
pub struct Province {
    pub id: u32,
    pub name: String,
    pub owner: String,
}

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

fn get_base_price(good: GoodType) -> f32 {
    match good {
        GoodType::Wine => 9.7,
        GoodType::Grain => 2.2,
        GoodType::Fruit => 1.8,
    }
}

fn add_goods(mut commands: Commands) {
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

fn add_provincies(mut commands: Commands) {
    commands.spawn(Province {
        id: 1,
        name: "province1".to_string(),
        owner: "owner1".to_string(),
    });
}

fn add_factories(mut commands: Commands) {
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

fn add_pops(mut commands: Commands) {
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

fn setup(mut commands: Commands) {}

fn production_system(
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

fn update_prices_system(mut market: ResMut<Market>, mut new_day_ev: EventReader<NewDayEvent>) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for (good_type, quantity, price) in &mut market.goods {
            println!(
                "good: {:?}, quantity: {}, price: {}",
                good_type, quantity, price
            );
            let base_price = get_base_price(*good_type);
            *price = base_price * (1.0 + (50.0 - *quantity).max(-50.0) / 100.0);
        }
    }
}

fn population_consumption_system(
    mut market: ResMut<Market>,
    mut query: Query<&PopGroup>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    if !new_day_ev.is_empty() {
        new_day_ev.clear(); // clean processed events

        for pop in query.iter() {
            for (good_type, quantity) in &pop.needs {
                let demand = quantity * pop.size as f32 / 1000 as f32;
                println!(
                    "good_type: {:?}, quantity: {:?}, demand: {:?}",
                    good_type, quantity, demand
                );

                if let Some((_, available_quantity, _)) =
                    market.goods.iter_mut().find(|(gt, _, _)| gt == good_type)
                {
                    if *available_quantity >= demand {
                        *available_quantity -= demand;
                        println!("Pop {} consumiu {:.4} de {:?}", pop.id, demand, good_type);
                    } else {
                        println!(
                            "Pop {} queria {:.4} de {:?} mas só tinha {:.4}",
                            pop.id, demand, good_type, available_quantity
                        );
                        // talvez consumir só o que está disponível:
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

fn main() {
    let goods: Vec<(GoodType, f32, f32)> = vec![
        (GoodType::Grain, 100.0, get_base_price(GoodType::Grain)),
        (GoodType::Wine, 100.0, get_base_price(GoodType::Wine)),
        (GoodType::Fruit, 100.0, get_base_price(GoodType::Fruit)),
    ]; // idk if is a good practice initialize this data here

    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NewDayEvent>()
        .insert_resource(TimeTracker::default())
        .insert_resource(Market { goods })
        .add_systems(
            Startup,
            (setup, add_goods, add_provincies, add_factories, add_pops).chain(),
        )
        .add_systems(
            Update,
            (
                production_system,
                population_consumption_system,
                update_prices_system,
                advance_time,
            )
                .chain(),
        )
        .run();
}
