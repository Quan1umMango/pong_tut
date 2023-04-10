use macroquad::prelude::*;
use ::rand::{Rng,thread_rng};

const PADDLE_W:f32 = 20f32;
const PADDLE_H:f32 = 80f32;
const PADDLE_COLOR:Color = DARKBLUE;
const PADDLE_SPEED:f32 = 10f32;

const BALL_RADIUS:f32 = 15f32;
const BALL_COLOR:Color = PINK;
const BALL_SPEED:f32 = 5f32;

fn conf() -> Conf {
    Conf {
        window_title:"Pong".to_owned(),
        window_resizable:false,
        ..Default::default()
    }
}

#[derive(Copy,Clone)]
struct Paddle {
    rect:Rect,
}

#[derive(Copy,Clone)]
struct Ball {
    circle:Circle,
    dir:Vec2,
}

impl Paddle {
    fn new(rect:Rect) -> Self {
        Self {
            rect,    
            
         }
    }

    fn movement(&mut self,up:KeyCode,down:KeyCode) {
        if is_key_down(up) {
            self.rect.y -= PADDLE_SPEED;
        }else if is_key_down(down) {
            self.rect.y += PADDLE_SPEED;
        }
        if self.rect.y > screen_height()-PADDLE_H {
            self.rect.y = screen_height()-PADDLE_H;
        }else if self.rect.y < 0f32 {
            self.rect.y = 0f32;
        }
    }

    fn draw(&self) {
        let r = self.rect;
        draw_rectangle(r.x,r.y,r.w,r.h,PADDLE_COLOR);
    }

}

impl Ball {
    fn new(circle:Circle) -> Self {
        let mut rng = thread_rng();
        let mut x_dir = rng.gen_range(-1..1) as f32;
        if x_dir == 0. {
            x_dir = 1.;
        }
        let mut y_dir = rng.gen_range(-1..1) as f32;
        if y_dir == 0. {
            y_dir = -1.;
        }
         Self {
            circle,
            dir:vec2(x_dir,y_dir)
        }
    }

    fn movement(&mut self) {
        self.circle.x += self.dir.x*BALL_SPEED;
        self.circle.y += self.dir.y*BALL_SPEED;

        if self.circle.y > screen_height() || self.circle.y < 0.{
            self.dir.y = -self.dir.y;
        }
    }
    
    fn collision_with_paddle(&mut self,paddle_rect:&Rect) {
       let ball_rect = Rect::new(self.circle.x,self.circle.y,BALL_RADIUS,BALL_RADIUS);
       match  ball_rect.intersect(*paddle_rect) {
            Some(_intersection) => {
                self.dir.x = -self.dir.x
            },
            _=> return,
        } 
    }
    
    fn draw(&self) {
        draw_circle(self.circle.x,self.circle.y,self.circle.r,BALL_COLOR);
    }

    
}

struct Game {
    paddle_left:Paddle,
    paddle_right:Paddle,
    ball:Ball,
    scores: Vec<u32>
}

impl Game {
    fn new() -> Self {
  
        let mut paddle_left = Paddle::new(Rect::new(PADDLE_W,screen_height()/2.,PADDLE_W,PADDLE_H));
        let mut paddle_right = Paddle::new(Rect::new(screen_width()-PADDLE_W*2.,screen_height()/2.,PADDLE_W,PADDLE_H));
        let mut ball = Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
         Self {
            paddle_left,
            paddle_right,
            ball,
           scores: vec![0,0] 
        }
    }

    async fn run(&mut self) {
        let mut paddle_left = self.paddle_left;
        let mut paddle_right  = self.paddle_right;
        let font =  load_ttf_font("./res/Roboto-Black.ttf")
        .await
        .unwrap();
        let text_params =  TextParams {
            font_size:70,
            font,
            ..Default::default()
        };
        loop {
            
        
            paddle_left.movement(KeyCode::W,KeyCode::S);
            paddle_right.movement(KeyCode::Up,KeyCode::Down);
           self. ball.collision_with_paddle(&paddle_left.rect);
            self.ball.collision_with_paddle(&paddle_right.rect);
            self.score();
             self.ball.movement();
        
            clear_background(SKYBLUE);



        // Escape To Exit
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        paddle_left.draw();
        paddle_right.draw();
        self.ball.draw();
       

        //view scores
        draw_text_ex(&format!("{}",self.scores[0]).as_str(),100., 100.,text_params);
        draw_text_ex(&format!("{}",self.scores[1]).as_str(),screen_width()-100., 100.,text_params);
        
         next_frame().await;
              
        }
     }

    fn score(&mut self) {
        if self.ball.circle.x > screen_width() {
            self.ball = Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
            self.scores[0] += 1;
        }else if self.ball.circle.x < 0. {
            self.scores[1] += 1;
            self.ball = Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
         }
    }
}


#[macroquad::main(conf)] 
async fn main() {
    Game::new()
        .run().await;    
}