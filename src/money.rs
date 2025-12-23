use bevy::prelude::{Commands, Component};

use crate::goods::{get_base_price, GoodType};

#[derive(Component)]
pub struct Bank {
    pub id: u32,
    pub name: String,
    pub reserve: Vec<(GoodType, f32)>, // Gold/Silver
    pub money_reserve: f32,            // Gold/Silver
    pub processing_fee: f32,
}

impl Bank {
    // in this moment i only transform gold in money and apply a tax, the money is literally gold,
    // then if needed we can transform currency in gold too, this system will improve much in the future
    pub fn mint_currency(&mut self, gold_amount: f32) -> f32 {
        let money = get_base_price(GoodType::Gold) * gold_amount;
        let fee = money * self.processing_fee;
        let net_money = money - fee;
        println!("net_money: {:?}", net_money);

        self.money_reserve += fee;
        // for (good, amount) in &mut self.reserve {
        //     if *good == GoodType::Gold {
        //         *amount += gold_amount;
        //     }
        // }

        net_money
    }
}

pub fn add_banks(mut commands: Commands) {
    commands.spawn(Bank {
        id: 1,
        name: "World Bank".to_string(),
        reserve: vec![(GoodType::Gold, 0.0)],
        money_reserve: 0.0,
        processing_fee: 0.20,
    });
}
