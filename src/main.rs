use macroquad::prelude::*;

const BALL_SPEED: f32 = 400f32;
const BALL_SIZE: f32 = 30f32;

struct Ball {
    circle: Circle,
    vel: Vec2,
    regular: bool,
}

impl Ball {
    pub fn new(pos: Vec3, target: bool) -> Self {
        Self {
            circle : Circle::new(pos.x, pos.y, pos.z),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
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
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut balls = Vec::new();
    let mut tar = true;
    
    for i in 0..10 (
        if 0<= i && i <= 2 {
            tar = true;
        } else {
            tar = false;
        },

        balls.push(Ball::new(vec3( screen_width() *( i * 0.1f32), 20f32,15f32), tar))
    )
    {     
    loop {
        for ball in balls.iter() {
            ball.draw();
        }

        next_frame().await
    }
    }
}
