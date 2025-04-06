mod goods;
mod market;
mod population;
mod production;
mod province;
mod time;
mod ui;

use bevy::prelude::*;

use crate::goods::{add_goods, get_base_price, get_good_name, GoodType};
use crate::market::{update_prices_system, Market};
use crate::population::{add_pops, population_consumption_system};
use crate::production::{add_factories, production_system};
use crate::province::add_provincies;
use crate::time::{advance_time, NewDayEvent, TimeTracker};
use crate::ui::{setup_ui, update_market_ui, update_ui};

const KEY_SPEEDS: [(KeyCode, u8); 5] = [
    (KeyCode::Digit1, 1),
    (KeyCode::Digit2, 2),
    (KeyCode::Digit3, 3),
    (KeyCode::Digit4, 4),
    (KeyCode::Digit5, 5),
];

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut time_tracker: ResMut<TimeTracker>) {
    if keys.just_pressed(KeyCode::Space) {
        time_tracker.pause = !time_tracker.pause
    }

    for (key, speed) in KEY_SPEEDS {
        if keys.just_pressed(key) {
            time_tracker.speed = speed;
        }
    }
}

fn main() {
    let goods: Vec<(GoodType, f32, f32)> = vec![
        (GoodType::Grain, 50.0, get_base_price(GoodType::Grain)),
        (GoodType::Wine, 0.0, get_base_price(GoodType::Wine)),
        (GoodType::Fruit, 50.0, get_base_price(GoodType::Fruit)),
    ]; // idk if is a good practice initialize this data here

    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<NewDayEvent>()
        .insert_resource(TimeTracker::default())
        .insert_resource(Market { goods })
        .add_systems(
            Startup,
            (setup_ui, add_goods, add_provincies, add_factories, add_pops).chain(),
        )
        .add_systems(
            Update,
            (
                production_system,
                population_consumption_system,
                update_prices_system,
                advance_time,
                update_ui,
                update_market_ui,
                keyboard_input,
            )
                .chain(),
        )
        .run();
}
