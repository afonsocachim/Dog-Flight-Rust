use macroquad::prelude::*;

pub struct Position {
    pub x:f32, pub y:f32
}

pub struct MovUnity {
    pub pos: Position,
    pub ang: f32,
    pub speed: f32,
    pub max_ang_per_s: f32,
}

pub struct LinUnity {
    pub pos: Position,
    pub speed: f32,
    pub ang: f32,
}

pub struct Bullet {
    pub mov: LinUnity,
    pub lifetime: f32,
    pub age: f32,
}

impl Bullet {
    pub fn new (x:f32, y:f32, ang: f32) -> Self {
        let modifier = rand::gen_range(-2, 2) * 2;
        Bullet {
            mov: LinUnity {
                pos: Position { x, y },
                ang: ang + modifier as f32,
                speed: 350f32,
            },
            lifetime: 1f32,
            age: 0f32
        }
    }
}

pub struct Player {
    pub hp: i32,
    pub mov: MovUnity,
    pub controls: Controls,
    pub texture: Texture2D,
    pub bullets: Vec<Bullet>,
    pub radius: f32,
}

pub struct Controls {
    pub l: KeyCode,
    pub r: KeyCode,
    pub fire: KeyCode,
}