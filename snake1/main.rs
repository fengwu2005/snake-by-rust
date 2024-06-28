//贪吃蛇
use crossterm::{cursor, event::{self, KeyCode, KeyEvent, KeyModifiers}, execute, terminal, ExecutableCommand};
use std::io::{self, Write};
use std::time::Duration;
use rand::Rng;
use std::process::Command;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;

const W: i32 = 80;
const H: i32 = 60;

pub mod colors {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const LIGHTBLUE: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.0, 1.0, 1.0];
    pub const ANGEL: [f32; 4] = [0.5, 0.5, 1.0, 0.5];
    pub const GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];
}

#[derive(PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    pos: Vec<(i32, i32)>,
}

impl Food {

    fn new(snake: &mut Snake) -> Food {
        let mut temp: Vec<(i32, i32)> = vec![];
        for x in 0..H {
            for y in 0..W {
                if snake.body.contains(&(x, y)) {
                    
                }
                else {
                    temp.push((x, y));
                }
            }
        }
        let mut id = rand::thread_rng().gen_range(0..temp.len());
        let fi = temp[id];
        temp.remove(id);
        id = rand::thread_rng().gen_range(0..temp.len());
        let se = temp[id];
        temp.remove(id);
        id = rand::thread_rng().gen_range(0..temp.len());
        let th = temp[id];
        temp.remove(id);
        Food {
            pos: vec![fi, se, th],
        }
    }

    fn eat(&mut self, no: &(i32, i32)) -> bool {
        if self.pos.contains(no) {
            self.pos.retain(|&x| x != *no);
            true
        }
        else {
            false
        }
    }

    fn create(&mut self, snake: &mut Snake){
        let mut temp: Vec<(i32, i32)> = vec![];
        for x in 0..H {
            for y in 0..W {
                if snake.body.contains(&(x, y)) || self.pos.contains(&(x,y)) {
                    
                }
                else {
                    temp.push((x, y));
                }
            }
        }
        if temp.len() > 1 {
            self.pos.push(temp[rand::thread_rng().gen_range(0..temp.len())]);
        }
    }
}


struct Snake {
    body: Vec<(i32, i32)>,
    dir: Dir,
}

impl Snake {

    fn new() -> Snake {
        let head_x = H/2;
        let head_y = W/2;
        Snake {
            body: vec![(head_x, head_y),(head_x + 1, head_y)],
            dir: Dir::Right,
        }
    }

    fn move_on(&mut self, food: &mut Food) -> bool {
        let (head_x, head_y) = self.body[0];
        let new_head = match self.dir {
            Dir::Up => (head_x - 1, head_y),
            Dir::Down => (head_x + 1, head_y),
            Dir::Left => (head_x, head_y - 1),
            Dir::Right => (head_x, head_y + 1),
        };
        if food.eat(&new_head) {
            food.create(self);
        }
        else {
            self.body.pop();
        }
        if self.body.contains(&new_head) {
            return false;
        }
        if new_head.0 < 0
            || new_head.0 >= H 
            || new_head.1 < 0
            || new_head.1 >= W 
        {
            return false;
        }
        self.body.insert(0, new_head);
        true
    }

    fn change_dir(&mut self, key: Key){
        let new_dir = match key{
            Key::Up => Dir::Up,
            Key::Down => Dir::Down,
            Key::Left => Dir::Left,
            Key::Right => Dir::Right,
            _ => {
                return ;
            }
        };
        if self.dir == Dir::Up && new_dir == Dir::Down 
            || self.dir == Dir::Down && new_dir == Dir::Up
            || self.dir == Dir::Left && new_dir == Dir::Right
            || self.dir == Dir::Right && new_dir == Dir::Left 
        {
            return;
        }
        self.dir = new_dir;
    }
}

fn calc(block: (i32, i32), args: &RenderArgs) -> [f64; 4] {
    use graphics::*;
    let block_size_x = args.window_size[0] / (W as f64);
    let block_size_y = args.window_size[1] / (H as f64);
    let window_pos_x = (block.1 as f64) * block_size_x;
    let window_pos_y = (block.0 as f64) * block_size_y;
    rectangle::rectangle_by_corners(
        window_pos_x,
        window_pos_y,
        window_pos_x + block_size_x,
        window_pos_y + block_size_y,
    )
}

fn render(snake: &mut Snake, food: &mut Food, args: &RenderArgs, gl: &mut GlGraphics) {
    use graphics::*;
    
    gl.draw(args.viewport(), |c, gl| {

        clear(colors::BLACK, gl);

        rectangle(
            colors::GREEN,
            calc(snake.body[0], args),
            c.transform,
            gl,
        );
        rectangle(
            colors::RED,
            calc(food.pos[0], args),
            c.transform,
            gl,
        );
        rectangle(
            colors::RED,
            calc(food.pos[1], args),
            c.transform,
            gl,
        );
        rectangle(
            colors::RED,
            calc(food.pos[2], args),
            c.transform,
            gl,
        );
        
        for i in 0..snake.body.len() {
            if i==0 {
                continue;
            }
            rectangle(
                color::WHITE,
                calc(snake.body[i], args),
                c.transform,
                gl,
            );
        }
    });
}

fn main() {

    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("snakes", [960, 720])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build PistonWindow: {}", e));


    let mut gl = GlGraphics::new(opengl);

    let ref mut events = Events::new(EventSettings::new());
    events.set_ups(5);

    let mut snake = Snake::new();
    let mut food = Food::new(&mut snake);

    println!("Press ['上','下','左','右'] to change your direction");
    println!("Press the [Enter] to start and press the [Esc] to quit");

    // let _ = terminal::enable_raw_mode();

    if let event::Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
        match code {
            KeyCode::Enter => {
                println!("Start!");
            }
            KeyCode::Esc => {
                println!("Quit!");
                return;
            }
            _ => {
                return;
            }
        }
    }

    // let _ = terminal::disable_raw_mode();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            render(&mut snake, &mut food, &args, &mut gl);
        }
        if let Some(args) = e.update_args() {
            if snake.move_on(&mut food){

            }
            else {
                std::thread::sleep(Duration::from_secs(2));
                println!("Defeat!");
                break;
            }
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(key) => {
                    snake.change_dir(key);
                },
                _ => {}
            }
            if snake.move_on(&mut food){

            }
            else {
                std::thread::sleep(Duration::from_secs(2));
                println!("Defeat!");
                break;
            }
        }
    }
    println!("Quit!");
}