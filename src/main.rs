use::macroquad::prelude::*;
// internal mods
mod enemy;
mod global;
mod player;
mod structs;
mod utils;
use structs::*;
use utils::*;
use enemy::*;


fn draw_border () {
    draw_rectangle_lines(0f32, 0f32, screen_width() as f32, screen_height() as f32 , 1f32, WHITE);
}

enum Winner {
    Player,
    Enemy
}

fn run_game_n_continue (player: &mut Player, enemies: &mut Vec<Enemy>, results: &mut Vec<Winner>) -> bool {
    // move player
    update_player( player, get_frame_time());
    draw_player(&player.mov, player.texture);
    draw_bullets(&player.bullets);
    let mut remove_vec: Vec<usize> = Vec::new();
    for (index, enemy) in enemies.iter_mut().enumerate() {
        update_enemy( enemy, get_frame_time(), &player.mov);
        player.hp -= hp_lost(&enemy.bullets, &player.mov.pos);
        enemy.hp -= hp_lost(&player.bullets, &enemy.mov.pos);
        if enemy.hp < 0 {
            remove_vec.push(index);
            return false
        };
        draw_player(&enemy.mov, enemy.texture);
        draw_bullets(&enemy.bullets);
        draw_enemy_hp(enemy.hp);
    }
    remove_vec.reverse();

    for i in remove_vec.iter() {
        enemies.remove(*i);
    }

    if enemies.len() <= 0 {
        results.push(Winner::Player);
        return false
    };
    if player.hp < 0 {
        results.push(Winner::Enemy);
        return false
    };

    draw_player_hp(player.hp);
    return true;
}

#[macroquad::main("dogfight")]
async fn main() {
    // SE, E, NE, NW, W, SW
    // let c:Vec<(i32, i32)> = vec!((-1, 1),(-1, 0),(-1, -1),(0, -1),(1, 0),(0, 1));
    let p_texture: Texture2D = load_texture("src/player.png").await.unwrap();
    let e_texture: Texture2D = load_texture("src/enemy.png").await.unwrap();
    let mut player = Player::new(p_texture);
    let mut enemy = Enemy::new(e_texture, screen_width(), screen_height());
    let mut enemies: Vec<Enemy> = vec![enemy];
    let mut results: Vec<Winner> = Vec::new();
    let mut playing = true;

    loop {
        clear_background(BLACK);
        draw_border ();
        if results.len() > 0 {
            if playing {
                playing = run_game_n_continue(&mut player, &mut enemies, &mut results);
            } else {
                let response = ask(results.last().unwrap());
                match response {
                    Response::Yes => {
                        player = Player::new(p_texture);
                        enemy = Enemy::new(e_texture, screen_width(), screen_height());
                        enemies = vec![enemy];
                        playing = true
                    },
                    _ => (),
                }
            }
        } else {
            let response = start_playing();
            match response {
                Response::Yes => {results.push(Winner::Player);},
                _ => ()
            }
        }
        next_frame().await;
    }
}

enum Response {
    Yes,
    Waiting
}

const FONT_SIZE: f32 = 30f32;
const PLAY_AGAIN_TEXT: &str = "Press (Y) to play again!";

fn draw_hor_centered_text (text: &str, y: f32) {
    draw_text(text, screen_width() / 2. - (text.len() as f32 * FONT_SIZE) / 4.5, y, FONT_SIZE, WHITE);
}

fn draw_hor_centered_text_size (text: &str, y: f32, font_size: f32) {
    draw_text(text, screen_width() / 2. - (text.len() as f32 * font_size) / 4.5, y, font_size, WHITE);
}

fn ask (winner: &Winner) -> Response {
	let mut result= Response::Waiting;
    match winner {
        Winner::Enemy => {
            draw_hor_centered_text_size("Game Over!", screen_height() / 2. - FONT_SIZE * 3., FONT_SIZE * 2.);
        },
        Winner::Player => {
            draw_hor_centered_text_size("You won!", screen_height() / 2. - FONT_SIZE * 3., FONT_SIZE * 2.);
        }
    }

    draw_hor_centered_text(PLAY_AGAIN_TEXT, screen_height() / 2.);
    if is_key_down(KeyCode::Y) {result = Response::Yes};
	return result;
}

fn start_playing () -> Response {
	let mut result= Response::Waiting;
    draw_hor_centered_text_size("Press (Y) to start playing!", screen_height() / 2. - FONT_SIZE * 2., FONT_SIZE * 2.);
    draw_hor_centered_text("Space to shoot", screen_height() / 2.);
    draw_hor_centered_text("Left and Right Arrow to move", screen_height() / 2. + FONT_SIZE);
    if is_key_down(KeyCode::Y) {result = Response::Yes};
	return result;
}