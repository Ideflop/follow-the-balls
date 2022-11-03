use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::seq::index::sample; 
use std::process;

const BALL_SPEED: f32 = 125f32;
const BALL_SIZE: f32 = 20f32;
const BALL_SIZE_MENU: f32 = 35f32;
const BALL_START_POSITION: f32 = 30f32;
const BALL_START_POSITION_MENU: f32 = 375f32;
const TIME_NO_BLUE: f64 = 4f64;
const TIME_TO_GUESS: f64 = 10f64;

pub enum GameState {
   Menu,
   Game,
   Guess,
   Answer,
   Result,
}

#[derive(Clone)]
pub struct Ball {
    circle: Circle,
    vel: Vec2,
    regular: bool,
}

impl Ball {
    pub fn new(pos: Vec3, target: bool) -> Self {
        Self {
            circle : Circle::new(pos.x, pos.y, pos.z), // pos.z is the radius name need to be change
            vel: vec2(rand::gen_range(-1f32, 1f32), rand::gen_range(-1f32,1f32)).normalize(),
            regular: target,
        }
    }

    pub fn draw(&self, activ: bool) {
        let mut color = GREEN;
        if activ && self.regular {
            color = BLUE;
        }

        draw_circle( self.circle.x, self.circle.y, self.circle.r, color);
    }

    pub fn update(&mut self, dt : f32) {
        self.circle.x += self.vel.x * dt * BALL_SPEED; 
        self.circle.y += self.vel.y * dt * BALL_SPEED; 
        
        if self.circle.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.circle.y < 0f32 {
            self.vel.y = 1f32;
        }

        if screen_width() < self.circle.x {
            self.vel.x = -1f32;
        }

        if screen_height() < self.circle.y {
            self.vel.y = -1f32;
        }
        
    }
}

pub fn collision(a: &mut Ball, b: &Ball){ // change a.circle. to improve ? no
    if a.circle.overlaps(&b.circle) {
        let x_distance =  a.circle.x - b.circle.x;
        let y_distance = a.circle.y - b.circle.y;
        let random = rand::gen_range(1, 3);

        if x_distance > 0.0 {
            if y_distance > 0.0{
                match random {
                    1 => {
                        a.vel.x = -1f32;
                    }
                    2 => {
                        a.vel.y = -1f32;
                    }
                    _ => (),
                }
            }
            if y_distance == 0.0{
                a.vel.y = -1f32;
            }
            if y_distance < 0.0{
                match random {
                    1 => {
                        a.vel.y = -1f32;
                    }
                    2 => {
                        a.vel.x = 1f32 
                    }
                    _ => (),
                }
            }
        }

        if x_distance == 0.0 {
            if y_distance > 0.0{
                a.vel.x = -1f32;
            }
            if y_distance == 0.0{
            }
            if y_distance < 0.0{
                a.vel.x = 1f32;
            }
        }

        if x_distance < 0.0 {
            if y_distance > 0.0{
                match random {
                    1 => {
                        a.vel.x = -1f32;
                    }
                    2 => {
                        a.vel.y = 1f32 
                    }
                    _ => (),
                }
            }
            if y_distance == 0.0{
                a.vel.y = -1f32;
                // circle
            }
            if y_distance < 0.0{
                match random {
                    1 => {
                        a.vel.x = 1f32;
                    }
                    2 => {
                        a.vel.y = 1f32 
                    }
                    _ => (),
                }
            }
        }
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let font = load_ttf_font("font/ubuntu.mono.ttf").await.unwrap();
    let mut balls = Vec::new();
    let mut balls_menu = Vec::new();
    let mut count = 0;
    let mut time_game = 0f64;
    let mut game_state = GameState::Menu;
    let mut lives = 3;
    let mut ball_found_score = 0;
    let mut ball_found = Vec::new();
    let mut ball_choosen = Vec::new();
    let mut win_or_loose = String::new();
    let mut balls_result = Vec::new();
    
    for i in 0..10 {
        balls_menu.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.1f32), BALL_START_POSITION_MENU, BALL_SIZE_MENU), true));
    }

    loop {
        match game_state {
            GameState::Menu => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    count = 0;
                    let cursor_pos = mouse_position();
                    let mouse_pos = Vec2::new(cursor_pos.0,cursor_pos.1);
                    for ball in balls_menu.iter() {
                        count += 1;
                        if ball.circle.contains(&mouse_pos) {
                            break;
                        }
                    }
                }
                
                for (ct,ball) in balls_menu.iter().enumerate() {
                    if ct < count {
                        ball.draw(true);
                    } else {
                        ball.draw(false);
                    }
                }

                if is_key_down(KeyCode::Space) && count != 0{
                    game_state = GameState::Game;
                    time_game = get_time();

                    let mut rng = thread_rng(); // rand
                    ball_choosen = sample(&mut rng, 14, count).into_vec(); // rand

                    for i in 0..15 {
                        if ball_choosen.iter().any(|&l| l==i){
                            balls.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.07f32), BALL_START_POSITION, BALL_SIZE), true));
                        } else {
                            balls.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.07f32), BALL_START_POSITION, BALL_SIZE), false));
                        }
                    }
                }
            }
            GameState::Game => {
                for ball in balls.iter() {
                    if get_time() - time_game < TIME_NO_BLUE {
                        ball.draw(true);
                    } else {
                        ball.draw(false);
                    }
                }
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let colli = &balls.clone();
                
                for  ball in balls.iter_mut() {
                    for coll in colli.iter() {
                        collision( ball, coll);
                    }
                }

                if get_time() - time_game > TIME_TO_GUESS {
                    game_state = GameState::Guess;
                }
            }
            GameState::Guess => {

                for (ct,ball) in balls.iter().enumerate() {
                    if ball.regular && ball_found.iter().any(|&l| l== ct){
                        ball.draw(true);
                    } else {
                        ball.draw(false);
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let cursor_pos = mouse_position();
                    let mouse_pos = Vec2::new(cursor_pos.0,cursor_pos.1);
                    for (ct,ball) in balls.iter().enumerate() {
                        if ball.circle.contains(&mouse_pos) {
                            if ball.regular && ball_choosen.iter().any(|&l| l== ct){
                                ball_found.push(ct);
                                let index = ball_choosen.iter().position(|x| *x == ct).unwrap();
                                ball_choosen.remove(index);
                                ball_found_score += 1;
                                ball.draw(true);
                            } else if !ball_found.iter().any(|z| *z == ct){
                                lives -= 1;
                            }
                        }
                    }
                }

                if lives == 0 {
                    game_state = GameState::Answer;
                    win_or_loose = "loose".to_string();
                    time_game = get_time();
                }
                if ball_found_score == count {
                    game_state = GameState::Answer;
                    win_or_loose = "win".to_string();
                    time_game = get_time();
                }

            }
            GameState::Answer => {
                let ctime = get_time() - time_game;
                if win_or_loose == "loose" {
                    if ctime < 2f64 {
                        for ball in balls.iter() {
                           ball.draw(true);
                        }
                    } else {
                        game_state = GameState::Result;
                    }
                }
                if win_or_loose == "win" {
                    if ctime < 1.5f64 {
                        for ball in balls.iter() {
                           ball.draw(true);
                        }
                    } else {
                        game_state = GameState::Result;
                    }
                }
            }
            GameState::Result => {

                let time = get_time();
                if time - time_game  > 0.3f64 {
                    let mut rng = thread_rng(); // rand
                    let result = sample(&mut rng, 10, 4).into_vec(); // rand
                    for i in 0..10 {
                        if result.iter().any(|&l| l==i){
                            balls_result.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.1f32), BALL_START_POSITION_MENU, BALL_SIZE_MENU), true));
                        } else {
                            balls_result.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.1f32), BALL_START_POSITION_MENU, BALL_SIZE_MENU), false));
                        }
                    }
                    time_game += 0.6f64;
                }

                for ball in balls_result.iter() {
                    if ball.regular{
                        ball.draw(true);
                    } else {
                        ball.draw(false);
                    }
                }

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    balls = Vec::new();
                    balls_menu = Vec::new();
                    count = 0;
                    time_game = 0f64;
                    lives = 3;
                    ball_found_score = 0;
                    ball_found = Vec::new();
                    ball_choosen = Vec::new();
                    win_or_loose = String::new();
                    for i in 0..10 {
                        balls_menu.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.1f32), BALL_START_POSITION_MENU, BALL_SIZE_MENU), true));
                    }
                }
            
               if is_key_pressed(KeyCode::Q) {
                   process::exit(1);
               } 
            }
        }


        match game_state {
            GameState::Menu => {
                let title  = "Follow the Ball !".to_string(); // all title are the same
                draw_text_ex(&title,
                             275.0,
                             100.0,
                             TextParams { 
                                 font, 
                                 font_size: 30u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );

                let title  = "Choose how many balls you want to find".to_string();
                draw_text_ex(&title,
                             175.0,
                             200.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );

                let title  = "then press SPACE".to_string();
                draw_text_ex(&title,
                             300.0,
                             275.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );

                let title  = &format!("{} balls selected", count);
                draw_text_ex(title,
                             300.0,
                             475.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );
            }
            GameState::Guess => {
                let title  = &format!("{}/{} found", ball_found_score, count);
                draw_text_ex(title,
                             350.0,
                             475.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );
                let title  = &format!("{} lives remaining", lives);
                draw_text_ex(title,
                             300.0,
                             525.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );
            }
            GameState::Result => {
                let title  = &format!("You {} !", win_or_loose);
                draw_text_ex(title,
                             350.0,
                             150.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );
                let title  = "Press SPACE to play again or Q to quit".to_string();
                draw_text_ex(&title,
                             175.0,
                             225.0,
                             TextParams { 
                                 font, 
                                 font_size: 25u16, 
                                 color: WHITE, 
                                 ..Default::default() 
                            }
                        );
            }

            GameState::Game => {}
            GameState::Answer => {}
        }
        
        next_frame().await
    }
}
