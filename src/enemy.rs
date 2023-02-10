use macroquad::prelude::{Texture2D, screen_width, screen_height};

use crate::{structs::{MovUnity, Position, Bullet}};

pub struct Enemy {
    pub hp: i32,
    pub mov: MovUnity,
    pub texture: Texture2D,
    pub bullets: Vec<Bullet>,
    pub radius: f32,
}

impl Enemy {
    pub fn new(texture:Texture2D, x:f32, y:f32)-> Self{
        Self {

            hp: 100,
            mov: MovUnity{
                speed: 150f32,
                max_ang_per_s: 150f32,
                pos: Position {x,y},
                ang: -180f32, // angle from x axis in degrees
            },
            texture,
            bullets: Vec::new(),
            radius: 45f32,
        }
    }
}