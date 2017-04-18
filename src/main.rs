extern crate rand;
extern crate ncurses;
use ncurses::*;


#[derive(PartialEq)]
enum Status { SUCCESS, FAILURE }

#[derive(Clone, Copy, Debug)]
enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(Clone, Copy, PartialEq)]
struct Point {
  x: u32,
  y: u32
}

struct Board {
  xmax: u32,
  ymax: u32,
  snake: Vec<Point>,
  foods: Vec<Point>
}

impl Board {
  fn eat_food(&mut self, point: Point) {
    self.snake.insert(0, point);
  }

  fn move_to(&mut self, point: Point) {
    self.snake.insert(0, point);
    self.snake.pop();
  }

  fn add_new_food(&mut self) {
    // don't spawn new food inside the snake
    let mut point = self.create_random_cell();
    while self.snake.contains(&point) || self.foods.contains(&point) {
      point = self.create_random_cell();
    }
    self.foods.push(point);
  }

  fn create_random_cell(&self) -> Point {
    Point {
      x: rand::random::<u32>()% self.xmax, 
      y: rand::random::<u32>()% self.ymax
    }
  }

  fn initialize(&mut self) {
    self.snake.push(Point{x: 2, y: 3});
    self.snake.push(Point{x: 2, y: 2});
    // 1 food per 150 squares on the board
    let num_food = self.xmax * self.ymax / 150;
    for _ in 1..num_food {
      self.add_new_food();
    }
  }

  fn move_snake(&mut self, dir: Direction) -> Status {
    let beginning = self.next_move(dir);
    if beginning.is_err() { return Status::FAILURE }
    let point: Point = beginning.unwrap();
    // if we're going backwards, ignore and move on
    if self.snake[1] == point {
     return Status::SUCCESS;
    }
    // Check for collisions!
    if self.snake.contains(&point) {
     return Status::FAILURE;
    }
    if self.foods.contains(&point) {
     self.eat_food(point);
     self.foods.retain(|&x| x != point);
     self.add_new_food();
     return Status::SUCCESS;
    }
    self.move_to(point);
    Status::SUCCESS
  }

  fn next_move(&self, dir: Direction) -> Result<Point, ()> {
    let head = &self.snake[0];
    let mut new_x = head.x as i32;
    let mut new_y = head.y as i32;
    match dir {
      Direction::UP => {
        new_y -= 1;
      }
      Direction::DOWN => {
        new_y += 1;
      }
      Direction::RIGHT => {
        new_x += 1;
      }
      Direction::LEFT => {
        new_x -= 1;
      }
    }
    if new_x < 0 || new_y < 0 || new_x >= self.xmax as i32 || new_y >= self.ymax as i32 {
      Err(())
    } else {
      Ok(Point{x: new_x as u32, y: new_y as u32})
    }
  }

}

// some functions to make ncurses work

fn display_points(snake: &[Point], symbol: chtype) {
  for point in snake {
    mvaddch(point.y as i32, point.x as i32, symbol);
  }
}

fn get_next_move(previous: Direction) -> Direction {
  let ch = getch();
  match (ch, previous) {
    // don't let people turn 180 degrees, it doesn't make sense
    (KEY_LEFT, Direction::RIGHT) => previous,
    (KEY_RIGHT, Direction::LEFT) => previous,
    (KEY_UP, Direction::DOWN) => previous,
    (KEY_DOWN, Direction::UP) => previous,
    (KEY_RIGHT, _) => Direction::RIGHT,
    (KEY_LEFT, _) => Direction::LEFT,
    (KEY_DOWN, _) => Direction::DOWN,
    (KEY_UP, _) => Direction::UP,
    _ => previous
  }
}


fn main() {
  initscr();
  cbreak();
  noecho();
  keypad(stdscr(), true); // make keys work
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // hide cursor
  timeout(100);

  let mut xmax: i32 = 0;
  let mut ymax: i32 = 0;
  getmaxyx(stdscr(), &mut ymax, &mut xmax);
  let mut dir = Direction::RIGHT;

  let mut board = Board{xmax: xmax as u32, ymax: ymax as u32, foods: vec!(), snake: vec!()};
  board.initialize();

  let mut status = Status::SUCCESS;

  while status == Status::SUCCESS {
    clear();
    display_points(&board.snake, ACS_BLOCK());
    display_points(&board.foods, ACS_DIAMOND());
    refresh();
    dir = get_next_move(dir);
    status = board.move_snake(dir);
  }
  endwin();
}
