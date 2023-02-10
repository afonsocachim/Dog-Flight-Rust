use crate::{Player, Controls, MovUnity, structs::Position};
use macroquad::prelude::*;


impl Player {
    pub fn new(texture:Texture2D)-> Self{
        Self {
            hp: 100,
            controls: Controls {
                l: KeyCode::Left,
                r: KeyCode::Right,
                fire: KeyCode::Space,
            },
            mov: MovUnity{
                speed: 150f32,
                max_ang_per_s: 150f32,
                pos: Position {x:30f32,y:30f32},
                ang: 45f32, // angle from x axis in degrees
            },
            texture,
            bullets: Vec::new(),
            radius: 45f32,
        }
    }
}