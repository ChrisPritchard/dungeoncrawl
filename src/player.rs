use crate::prelude::*;

pub struct Player {
    position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(self.position.x - camera.left_x, self.position.y - camera.top_y, WHITE, BLACK, to_cp437('@'));
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = 
                match key {
                    VirtualKeyCode::Up => Point::new(0, -1),
                    VirtualKeyCode::Right => Point::new(1, 0),
                    VirtualKeyCode::Down => Point::new(0, 1),
                    VirtualKeyCode::Left => Point::new(-1, 0),
                    _ => Point::zero(),
                };
            let next_position = self.position + delta;
            if map.can_enter_tile(next_position) {
                self.position = next_position;
                camera.on_player_move(next_position);
            }
        }
    }
}