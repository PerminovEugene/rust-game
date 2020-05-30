use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;

const ENEMy_MOVEMENT_SPEED: i32 = 20;

pub struct AI;

impl<'a> System<'a> for AI {
    type SystemData = (
        ReadExpect<'a, Option<Position>>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        for (_, vel) in (&data.1, &mut data.2).join() {
            // match movement_command {
            //     &MovementCommand::Move(direction) => {
            //         vel.speed = PLAYER_MOVEMENT_SPEED;
            //         vel.direction = direction;
            //     },
            //     MovementCommand::Stop => vel.speed = 0,
            // }
        }
    }
}