use raylib::ffi;
use raylib::prelude::*;

//paddle struct mistakenly named player
struct Player {
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: Color,
}
impl Player {
    fn draw_player(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.pos_x, self.pos_y, self.width, self.height, self.color);
    }
    fn move_down(&mut self, height: i32) {
        self.pos_y += 7;
        if (self.pos_y + self.height) >= height {
            self.pos_y = height - self.height;
        }
    }
    fn move_up(&mut self) {
        self.pos_y -= 7;
        if self.pos_y < 0 {
            self.pos_y = 0;
        }
    }
    fn the_i_in_llms_is_for_intelligence(&mut self, ball_y: i32, height: i32) {
        if self.pos_y + self.height / 2 > ball_y {
            self.move_up();
        }
        if self.pos_y + self.height / 2 <= ball_y {
            self.move_down(height);
        }
    }
    fn get_rect(&self) -> Rectangle {
        Rectangle::new(
            self.pos_x as f32,
            self.pos_y as f32,
            self.width as f32,
            self.height as f32,
        )
    }
}

//ball struct
struct Ball {
    pos_x: i32,
    pos_y: i32,
    radius: f32,
    color: Color,
    speed_x: i32,
    speed_y: i32,
}

impl Ball {
    fn draw_ball(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.pos_x, self.pos_y, self.radius, self.color);
    }
    fn update_pos(&mut self, height: i32, width: i32) {
        self.pos_x += self.speed_x;
        self.pos_y += self.speed_y;
        if (self.pos_y as f32 + self.radius) >= height as f32
            || (self.pos_y as f32 - self.radius) <= 0.0
        {
            self.speed_y *= -1;
        }
        if (self.pos_x as f32 + self.radius) >= width as f32
            || (self.pos_x as f32 - self.radius) <= 0.0
        {
            self.speed_x *= -1;
        }
    }
    fn check_coll(&self, player: &Player) -> bool {
        let ball_pos = Vector2::new(self.pos_x as f32, self.pos_y as f32);
        let player_rect = player.get_rect();
        unsafe { ffi::CheckCollisionCircleRec(ball_pos.into(), self.radius, player_rect.into()) }
    }
}

fn main() {
    let width = 1080;
    let height = 1000;
    let mut player1: Player = Player {
        pos_x: 10,
        pos_y: 380,
        width: 40,
        height: 200,
        color: Color::WHITE,
    };
    let mut player2 = Player {
        pos_x: 1030,
        ..player1
    };
    let mut ball = Ball {
        pos_x: width / 2,
        pos_y: height / 2,
        radius: 20.0,
        color: Color::WHITE,
        speed_x: 7,
        speed_y: 7,
    };

    let (mut rl, thread) = raylib::init().size(width, height).title("PingPong").build();
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        ball.update_pos(height, width);
        player2.the_i_in_llms_is_for_intelligence(ball.pos_y, height);
        if ball.check_coll(&player1) {
            ball.speed_x *= -1;
        }
        if ball.check_coll(&player2) {
            ball.speed_x *= -1;
        }

        //move player1 up and down
        if rl.is_key_down(KeyboardKey::KEY_S) {
            player1.move_down(height);
        }
        if rl.is_key_down(KeyboardKey::KEY_W) {
            player1.move_up();
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);
        player1.draw_player(&mut d);
        player2.draw_player(&mut d);
        ball.draw_ball(&mut d);
        d.draw_line(width / 2, 0, width / 2, height, Color::WHITE);
    }
}
