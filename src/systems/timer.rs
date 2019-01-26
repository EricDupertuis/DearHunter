use amethyst::core::timing::Time;
use amethyst::ecs::{Read, System, Write};

pub struct GameTimer {
    pub timer: f32,
}
impl Default for GameTimer {
    fn default() -> Self {
        GameTimer { timer: 5. }
    }
}

pub struct TimerSystem;

impl<'s> System<'s> for TimerSystem {
    type SystemData = (Write<'s, GameTimer>, Read<'s, Time>);

    fn run(&mut self, (mut game_timer, timer): Self::SystemData) {
        game_timer.timer -= timer.delta_seconds();
        if game_timer.timer <= 0. {
            println!("Time's up!");
        }
    }
}
