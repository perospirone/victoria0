use bevy::prelude::*;

#[derive(Resource)]
pub struct TimeTracker {
    pub day: u32,
    pub month: u32,
    pub year: u32,
    pub speed: u8, // todo: implement speed (1-5) in advance time system
    pub pause: bool,
    pub elapsed_time: f32, // Accumulates delta time
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self {
            day: 0,
            month: 1,
            year: 1830,
            speed: 1,
            pause: false,
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

pub fn advance_time(
    mut time_tracker: ResMut<TimeTracker>,
    time: Res<Time>,
    mut new_day_ev: EventWriter<NewDayEvent>,
) {
    if time_tracker.pause {
        return;
    }

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
