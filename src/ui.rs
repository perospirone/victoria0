use bevy::prelude::*;

use crate::{get_good_name, Market, NewDayEvent, TimeTracker};

#[derive(Component)]
pub struct MarketInfoText;

#[derive(Component)]
pub struct DateTimeText;

pub fn setup_ui(mut commands: Commands, mut market: ResMut<Market>) {
    // camera
    commands.spawn(Camera2d);

    // ui
    let mut market_info = String::new();
    for (good_type, quantity, price) in &market.goods {
        // Converte o GoodType para string
        let type_str = get_good_name(*good_type);
        // Monta a linha para o produto
        market_info.push_str(&format!(
            "{}: qty = {:.2}, price = {:.2}\n",
            type_str, quantity, price
        ));
    }

    commands.spawn((
        MarketInfoText,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("market info"),
        //TextFont {
        // This font is loaded and will be used instead of the default font.
        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        //font_size: 67.0,
        //..default()
        //},
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(1.0),
            top: Val::Px(1.0),
            ..default()
        },
    ));

    commands.spawn((
        DateTimeText,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Date: January 1836 Day 1 | Speed: 3 | Paused"),
        //TextFont {
        // This font is loaded and will be used instead of the default font.
        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        //font_size: 67.0,
        //..default()
        //},
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(1.0),
            bottom: Val::Px(2.0),
            ..default()
        },
    ));
}

pub fn update_ui(
    mut time_tracker: ResMut<TimeTracker>,
    mut new_day_ev: EventReader<NewDayEvent>,
    mut query: Query<&mut Text, With<DateTimeText>>,
    mut market: ResMut<Market>,
) {
    let mut text = query.single_mut();

    let month_name = match time_tracker.month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    };

    let pause_status = if time_tracker.pause {
        "Paused"
    } else {
        "Running"
    };

    text.0 = format!(
        "Date: {} {} {} | Speed: {} | {}",
        month_name, time_tracker.day, time_tracker.year, time_tracker.speed, pause_status
    );
}

// Add this new system
pub fn update_market_ui(
    mut market: ResMut<Market>,
    mut query: Query<&mut Text, With<MarketInfoText>>,
    mut new_day_ev: EventReader<NewDayEvent>,
) {
    let mut text = query.single_mut();
    let mut market_info = String::new();

    // Add header
    market_info.push_str("Market Status:\n");

    for (good_type, quantity, price) in &market.goods {
        //println!(
        //"good type {:?}, quantity {:?}, price {:?}",
        //good_type, quantity, price
        //);
        let type_str = get_good_name(*good_type);
        market_info.push_str(&format!(
            "{}: {:.3} units @ ${:.4}\n",
            type_str, quantity, price
        ));
    }

    // Update the text section
    text.0 = market_info;
}
