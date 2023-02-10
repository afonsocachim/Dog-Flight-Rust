use macroquad::prelude::*;

use crate::{structs::{MovUnity, Player, Position, Bullet}, enemy::Enemy};

pub fn draw_player(mov:&MovUnity, texture:Texture2D) {
    let params = DrawTextureParams {
        dest_size: Some(Vec2::new(45f32, 45f32)),
        source:None,
        rotation: (mov.ang).to_radians(),
        flip_x: false,
        flip_y: false,
        pivot:None,
    };
    draw_texture_ex(
        texture,
        mov.pos.x - 15f32,
        mov.pos.y - 15f32,
        WHITE,
        params

    );
}

pub fn draw_player_hp (hp: i32) {
    draw_rectangle(10f32, 10f32, 100., 10f32, DARKGRAY);
    draw_rectangle(10f32, 10f32, hp as f32, 10f32, BLUE);
    draw_rectangle_lines(10f32, 10f32, 100., 10f32, 1f32, WHITE);
}

pub fn draw_enemy_hp (hp: i32) {
    draw_rectangle(screen_width() - 100. - 10., 10f32, 100., 10f32, DARKGRAY);
    draw_rectangle(screen_width() - 100. - 10., 10f32, hp as f32, 10f32, RED);
    draw_rectangle_lines(screen_width() - 100. - 10., 10f32, 100., 10f32, 1f32, WHITE);
}

pub fn draw_bullets (bullets: &Vec<Bullet>) {
    for b in bullets.iter() {
        draw_rectangle(b.mov.pos.x, b.mov.pos.y, 2f32, 2f32, LIGHTGRAY)
    }
}

pub fn normalize_degrees (calc_ang: f32) -> f32 {
    let mut temp_ang = calc_ang - (calc_ang / 360.).abs().floor();
    if temp_ang > 180. {temp_ang = temp_ang-360.}
    if temp_ang < -180. {temp_ang = 360.+temp_ang}
    return temp_ang;
}

pub fn ang_diff (a: f32, b:f32) -> f32 {
    let mut d = a - b;
    if d > 180. {d -= 360.};
    if d < -180. {d += 360.};
    return d.abs();
}

pub fn update_player (p: &mut Player, dt:f32) {
    // calculate new angle
    let direction = match(is_key_down(p.controls.l), is_key_down(p.controls.r)){
        (true, false) => -1f32,
        (false, true) => 1f32,
        _ => 0f32,
    };
    let calc_ang = p.mov.ang + (dt * p.mov.max_ang_per_s * direction);
    let new_ang = normalize_degrees(calc_ang);
    p.mov.ang = new_ang;

    // calculate new position
    let distance = dt * p.mov.speed;

    p.mov.pos.y += distance * p.mov.ang.to_radians().sin();
    if p.mov.pos.y < 0f32 { p.mov.pos.y = screen_height() as f32}
    if p.mov.pos.y > screen_height() as f32{p.mov.pos.y  = 0f32}

    p.mov.pos.x += distance * p.mov.ang.to_radians().cos();
    if p.mov.pos.x < 0f32 {p.mov.pos.x = screen_width() as f32}
    if p.mov.pos.x > screen_width() as f32{p.mov.pos.x  = 0f32}

    // remove dead bullets
    let mut remove_vec: Vec<usize> = Vec::new();
    for (index, value) in p.bullets.iter().enumerate() {
        if value.age > value.lifetime {remove_vec.push(index)}
    }
    remove_vec.reverse();
    for i in remove_vec.iter() {
        p.bullets.remove(*i);
    }
    // update bullet age
    for bullet in p.bullets.iter_mut() {
        bullet.age += dt;
    }
    // update bullet position
    for bullet in p.bullets.iter_mut() {
        let bullet_distance = bullet.mov.speed * dt;
        bullet.mov.pos.y += bullet_distance * bullet.mov.ang.to_radians().sin();
        if bullet.mov.pos.y < 0f32 {
            bullet.mov.pos.y = screen_height() as f32;
        }
        if bullet.mov.pos.y > screen_height() as f32{
            bullet.mov.pos.y  = 0f32;
        }
    
        bullet.mov.pos.x += bullet_distance * bullet.mov.ang.to_radians().cos();
        if bullet.mov.pos.x < 0f32 {
            bullet.mov.pos.x = screen_width() as f32;
        }
        if bullet.mov.pos.x > screen_width() as f32{
            bullet.mov.pos.x  = 0f32;
        }
    }

    // fire bullets
    if !is_key_down(p.controls.fire) {return};
    if p.bullets.len() >= 10 {return};
    if p.bullets.len() > 0 && p.bullets[(p.bullets.len() - 1) as usize].age < 0.1 {return};
    let new_bullet = Bullet::new(p.mov.pos.x, p.mov.pos.y, p.mov.ang);
    p.bullets.push(new_bullet);
}

pub fn update_enemy (enemy: &mut Enemy, dt:f32, player: &MovUnity) {

    let a = Position {x : player.pos.x - enemy.mov.pos.x, y: player.pos.y - enemy.mov.pos.y};
    let mut ang = (a.y / a.x).atan().to_degrees();
    if player.pos.x < enemy.mov.pos.x { ang += 180f32}

    let distance = dt * enemy.mov.speed;

    let distance_between_p_e = ((player.pos.x - enemy.mov.pos.x).powi(2) + (player.pos.y - enemy.mov.pos.y).powi(2)).sqrt();
    let x = screen_height() / ang.to_radians().tan();
    let max_distance = (x.powi(2) + screen_height().powi(2)).sqrt();
    let other_distance = max_distance - distance_between_p_e;


    if other_distance < distance_between_p_e  {
        ang += 180.;
        normalize_degrees(ang);
    }

    let r_ang = normalize_degrees(enemy.mov.ang + (dt * enemy.mov.max_ang_per_s * 1.));
    let l_ang = normalize_degrees(enemy.mov.ang + (dt * enemy.mov.max_ang_per_s * -1.));
    let r_dif = ang_diff(ang, r_ang);
    let l_dif = ang_diff(ang, l_ang);
    let mut new_ang: f32 = enemy.mov.ang;
    if r_dif < l_dif {new_ang = r_ang}
    if r_dif > l_dif {new_ang = l_ang}
    enemy.mov.ang = new_ang;

    enemy.mov.pos.y += distance * enemy.mov.ang.to_radians().sin();
    if enemy.mov.pos.y < 0f32 {
        enemy.mov.pos.y = screen_height() as f32;
    }
    if enemy.mov.pos.y > screen_height() as f32{
        enemy.mov.pos.y  = 0f32;
    }

    enemy.mov.pos.x += distance * enemy.mov.ang.to_radians().cos();
    if enemy.mov.pos.x < 0f32 {
        enemy.mov.pos.x = screen_width() as f32;
    }
    if enemy.mov.pos.x > screen_width() as f32{
        enemy.mov.pos.x  = 0f32;
    }


    // remove dead bullets
    let mut remove_vec: Vec<usize> = Vec::new();
    for (index, value) in enemy.bullets.iter().enumerate() {
        if value.age > value.lifetime {remove_vec.push(index)}
    }
    remove_vec.reverse();
    for i in remove_vec.iter() {
        enemy.bullets.remove(*i);
    }
    // update bullet age
    for bullet in enemy.bullets.iter_mut() {
        bullet.age += dt;
    }
    // update bullet position
    for bullet in enemy.bullets.iter_mut() {
        let bullet_distance = bullet.mov.speed * dt;
        bullet.mov.pos.y += bullet_distance * bullet.mov.ang.to_radians().sin();
        if bullet.mov.pos.y < 0f32 {
            bullet.mov.pos.y = screen_height() as f32;
        }
        if bullet.mov.pos.y > screen_height() as f32{
            bullet.mov.pos.y  = 0f32;
        }
    
        bullet.mov.pos.x += bullet_distance * bullet.mov.ang.to_radians().cos();
        if bullet.mov.pos.x < 0f32 {
            bullet.mov.pos.x = screen_width() as f32;
        }
        if bullet.mov.pos.x > screen_width() as f32{
            bullet.mov.pos.x  = 0f32;
        }
    }

    // fire bullets
    let player_in_range = (player.pos.x -enemy.mov.pos.x).powi(2) + (player.pos.y - enemy.mov.pos.y).powi(2) < 360f32.powi(2);
    if !player_in_range {return};
    let angle_difference = ang_diff(ang, enemy.mov.ang);
    if angle_difference > 10f32 {return};
    if enemy.bullets.len() >= 10 {return};
    if enemy.bullets.len() > 0 && enemy.bullets[(enemy.bullets.len() - 1) as usize].age < 0.1 {return};
    let new_bullet = Bullet::new(enemy.mov.pos.x, enemy.mov.pos.y, enemy.mov.ang);
    enemy.bullets.push(new_bullet);
}

pub fn hp_lost (bullets: &Vec<Bullet>, target: &Position) -> i32{
    let mut counter = 0;

    for bullet in bullets.iter() {
        let player_in_range = (target.x -bullet.mov.pos.x).powi(2) + (target.y - bullet.mov.pos.y).powi(2) < 20f32.powi(2);
        if player_in_range {
            counter += 1;
        }
    }

    return counter;
}