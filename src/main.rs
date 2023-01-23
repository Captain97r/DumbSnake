use std::{thread, time};
use std::io::{stdout};
use std::io::Stdout;
use rand::Rng;
use crossterm::{ ExecutableCommand, terminal, cursor };
use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Copy, Clone)]
enum Direction {
    Right = 1,
    Up,
    Left,
    Down
}

struct Point {
    x : u16,
    y : u16
}

impl Point {
    fn new(_x : u16, _y : u16) -> Self {
        Self {
            x: _x,
            y: _y
        }
    }
}

struct Snake {
    length : usize,
    body : Vec<Point>,
    direction: Direction,
    is_alive : bool
}

impl Snake {
    fn new(_len : usize) -> Self {

        Self {
            length: _len,
            body: Vec::new(),
            direction: Direction::Right,
            is_alive: true
        }
    }

    fn do_move(&mut self) {
        

        for i in (1..self.length).rev() {
            let slice = &mut self.body[i-1..i+1];
            slice[1].x = slice[0].x;
            slice[1].y = slice[0].y;
        }

        let head = self.body.get_mut(0);

        if head.is_none() {
            return;
        }

        let head : &mut Point = head.unwrap();

        match self.direction {
            Direction::Right => head.x+=1,
            Direction::Left => head.x-=1,
            Direction::Down => head.y+=1,
            Direction::Up => head.y-=1
        }
    }

    fn set_direction(&mut self, dir : Direction) {
        if (self.direction as u8) % 2 == (dir as u8) % 2 {
            return;
        }

        self.direction = dir;
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }

    fn feed(&mut self) {
        self.body.push(Point::new(self.body[self.length - 1].x, self.body[self.length - 1].y));
        self.length += 1;
    }

    fn draw(&self, term : &mut Stdout) {

        for i in 0..self.length {
            term.execute(cursor::MoveTo(self.body[i].x, self.body[i].y)).expect("Error");
            print!("O");
        }
    }

}

struct Food {
    coords : Point
}

impl Food {
    fn new() -> Self {
        Self {
            coords: Point::new(rand::thread_rng().gen_range(1..=10), rand::thread_rng().gen_range(1..=10))
        }
    }

    fn draw(&self, term : &mut Stdout) {
        term.execute(cursor::MoveTo(self.coords.x, self.coords.y)).expect("Error");
        print!("X");
    }
}

struct SnakeGame {
    stdout : Stdout,
    keyboard : DeviceState,
    field_size : Point,
    snake : Snake,
    food : Food
}

impl SnakeGame {
    fn new(width: u16, height: u16) -> Self {
        Self {
            stdout: stdout(),
            keyboard: DeviceState::new(),
            field_size : Point::new(width, height),
            snake: Snake::new(4),
            food: Food::new()
        }
    }

    fn init(&mut self) {
        self.snake.body.push(Point::new(self.field_size.x / 2, self.field_size.y / 2));
        self.snake.body.push(Point::new(self.field_size.x / 2 - 1, self.field_size.y / 2));
        self.snake.body.push(Point::new(self.field_size.x / 2 - 2, self.field_size.y / 2));
        self.snake.body.push(Point::new(self.field_size.x / 2 - 3, self.field_size.y / 2));
    }

    fn draw_field(&mut self) {
        for x in 0..self.field_size.x {
            for y in 0..self.field_size.y {
                if x == 0 || x == self.field_size.x - 1 || y == 0 || y == self.field_size.y - 1 {
                    self.stdout.execute(cursor::MoveTo(x,y)).expect("Error");
                    print!("H");
                }
            }
        }
    }

    fn draw(&mut self) {
        self.draw_field();
        self.food.draw(&mut self.stdout);
        self.snake.draw(&mut self.stdout);
    }

    fn collision_detector(&mut self) {
        //walls
        let head = &self.snake.body[0];

        if head.x == 0 || head.x == self.field_size.x - 1 || head.y == 0 || head.y == self.field_size.y - 1 {
            self.snake.kill();
        }

        for i in 1..self.snake.length {
            if self.snake.body[0].x == self.snake.body[i].x && self.snake.body[0].y == self.snake.body[i].y {
                self.snake.kill();
            }
        }

        let head = &self.snake.body[0];
        if head.x == self.food.coords.x && head.y == self.food.coords.y {
            self.snake.feed();
            self.food = Food::new();
        }
    }

    fn get_key(&mut self) {
        let keys: Vec<Keycode> = self.keyboard.get_keys();
        match keys.last() {
            Some(key)=>{
                match key {
                    Keycode::W => self.snake.set_direction(Direction::Up),
                    Keycode::A => self.snake.set_direction(Direction::Left),
                    Keycode::S => self.snake.set_direction(Direction::Down),
                    Keycode::D => self.snake.set_direction(Direction::Right),
                    _=> {}
                }
            }
            None=>{}
        }
    }

    fn go(&mut self) {
        while self.snake.is_alive {
            self.stdout.execute(terminal::Clear(terminal::ClearType::All)).expect("Error");

            self.get_key();
            self.snake.do_move();
            self.draw();

            self.collision_detector();
            
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}


fn main() {
    let mut game = SnakeGame::new(20, 20);
    game.init();
    game.go();
}
