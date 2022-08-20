use macroquad::prelude::*;

const BALL_SPEED: f32 = 125f32;
const BALL_SIZE: f32 = 20f32;
const BALL_START_POSITION: f32 = 30f32;

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

    pub fn draw(&self) {
        let color = match self.regular {
            true => BLUE,
            false => GREEN,
        };

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

pub fn collision(a: &mut Ball, b: &Ball){ // change a.circle. to improve ?
    if a.circle.overlaps(&b.circle) {
        let x_distance =  a.circle.x - b.circle.x;
        let y_distance = a.circle.y - b.circle.y;
        let random = rand::gen_range(1, 3);

        if x_distance > 0.0 {
            if y_distance > 0.0{
                match random {
                    1 => {
                        a.vel.x = -1f32;
                        // circle
                    }
                    2 => {
                        a.vel.y = -1f32;
                        // cercle
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
                        a.vel.y = -1f32;
                        // circle
                    }
                    2 => {
                        a.vel.x = 1f32 
                        // circle
                    }
                    _ => (),
                }
            }
        }

        if x_distance == 0.0 {
            if y_distance > 0.0{
                a.vel.x = -1f32;
                // circle
            }
            if y_distance == 0.0{
            }
            if y_distance < 0.0{
                a.vel.x = 1f32;
                // circle
            }
        }

        if x_distance < 0.0 {
            if y_distance > 0.0{
                match random {
                    1 => {
                        a.vel.x = -1f32;
                        //a.circle.x -= 20f32;
                    }
                    2 => {
                        a.vel.y = 1f32 
                        //a.circle.y -= 20f32;
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
                        // circle
                    }
                    2 => {
                        a.vel.y = 1f32 
                        // circle
                    }
                    _ => (),
                }
            }
        }
    }
}

pub fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut balls = Vec::new();
    
    for i in 0..15 {
        let tar = i == 0;
        // let tar = matches!(i, 0..=2); // if 0<=i<=2 then true else false
        balls.push(Ball::new(vec3( screen_width() *( (i as f32 +0.5) * 0.07f32), BALL_START_POSITION, BALL_SIZE), tar));
    }

        
    loop {
        // clear_background(WHITE);

        for ball in balls.iter() {
            ball.draw();
        }
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }

        let colli = &balls.clone();
        
        for  ball in balls.iter_mut() {
            for coll in colli.iter() {
                let one = ball.circle.point();
                let two = coll.circle.point();

                if !(one.x == two.x) && !(one.y == two.y){
                    collision( ball, coll);
                }
            }
        }
        
        next_frame().await
    }
}
