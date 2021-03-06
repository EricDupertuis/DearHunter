use amethyst::core::timing::Time;
use amethyst::ecs::{Read, ReadExpect, System, Write, WriteStorage};
use amethyst::ui::UiText;

use crate::game_termination::GameTermination;
use crate::score::{GameTimer, ScoreText};

pub struct TimerSystem;

impl<'s> System<'s> for TimerSystem {
    type SystemData = (
        Write<'s, GameTimer>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        Write<'s, GameTermination>,
    );

    fn run(
        &mut self,
        (mut game_timer, timer, mut ui_text, score_text, mut game_termination): Self::SystemData,
    ) {
        if game_timer.active {
            if game_timer.timer > 0. {
                game_timer.timer -= timer.delta_seconds();
            } else {
                game_timer.timer = 0.;
            }

            if let Some(text) = ui_text.get_mut(score_text.timer) {
                text.text = format!("{:.1}", game_timer.timer);
                if game_timer.timer == 0. {
                    text.color = [1., 0., 0., 1.];
                    game_termination.timeout = true;
                }
            }
        }
    }
}
