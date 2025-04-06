use bevy::prelude::*;

use crate::time::NewDayEvent;
use crate::{get_base_price, GoodType};

#[derive(Resource)]
pub struct Market {
    pub goods: Vec<(GoodType, f32, f32)>, // (type, quantity available, actual price)
}

pub fn update_prices_system(mut market: ResMut<Market>, mut new_day_ev: EventReader<NewDayEvent>) {
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
