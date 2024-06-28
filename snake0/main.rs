//贪吃蛇
use crossterm::{event::{self, KeyCode, KeyEvent}, execute, terminal};
use std::io::{self, Write};
use std::time::Duration;
use rand::Rng;

const W: i32 = 80;
const H: i32 = 24;

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
            body: vec![(head_x, head_y)],
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

    fn change_dir(&mut self, new_dir: Dir){
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

fn print(snake: &mut Snake, food: &mut Food){
    print!("■");
    for y in 0..W {
        print!("■");
    }
    print!("■");
    println!("");
    for x in 0..H {
        print!("■");
        for y in 0..W {
            if snake.body.contains(&(x, y)) || food.pos.contains(&(x,y)) {
                print!("■");
            }
            else {
                print!(" ");
            }
        }
        print!("■");
        println!("");
    }
    print!("■");
    for y in 0..W {
        print!("■");
    }
    print!("■");
    println!("");
}

fn main() {
    let mut snake = Snake::new();
    let mut food = Food::new(&mut snake);
    println!("Press ['w','s','a','d'] to change your direction");
    println!("Press the [Enter] to start and press the [Esc] to quit");
    let _ = terminal::enable_raw_mode();
    if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
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
    let _ = terminal::disable_raw_mode();
    println!("");
    loop{
        execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
        print(&mut snake, &mut food);
        let _ = io::stdout().flush();
        let _ = terminal::enable_raw_mode();
        if event::poll(Duration::from_millis(500)).unwrap() {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                let _ = terminal::disable_raw_mode();
                match code {
                    KeyCode::Char('w') => snake.change_dir(Dir::Up),
                    KeyCode::Char('s') => snake.change_dir(Dir::Down),
                    KeyCode::Char('a') => snake.change_dir(Dir::Left),
                    KeyCode::Char('d') => snake.change_dir(Dir::Right),
                    KeyCode::Esc => {
                        println!("Quit!");
                        return;
                    }
                    _ => {}
                }
            }
        }
        let _ = terminal::disable_raw_mode();
        if snake.move_on(&mut food) {

        }
        else {
            println!("Defeat!");
            return ;
        }
    }
}