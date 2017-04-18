use std::io;

enum Status { SUCCESS, FAILURE }
enum Direction { UP, DOWN, LEFT, RIGHT }

#[derive(PartialEq)]
struct Point {
  x: i32,
  y: i32
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
}

fn next_move(board: &Board, dir: Direction) -> Result<Point, ()> {
  let snake = &board.snake;
  let head = &snake[0];
  let mut new_x = head.x;
  let mut new_y = head.y;
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
  if new_x < 0 || new_y < 0 || new_x >= board.xmax as i32 || new_y >= board.ymax as i32 {
    return Err(());
  } else {
    return Ok(Point{x: new_x, y: new_y});
  }
}

fn move_snake(board: &mut Board, dir: Direction) -> Status {
   let beginning = next_move(board, dir);
   match beginning {
     Err(e) => {return Status::FAILURE}
     Ok(_) => {}
   }
   let point: Point = beginning.unwrap();

   // if we're going backwards, ignore and move on
   if board.snake[1] == point {
    return Status::SUCCESS;
   }
   // Check for collisions!
   if board.snake.contains(&point) {
    return Status::FAILURE;
   }
   if board.foods.contains(&point) {
    board.eat_food(point);
//     remove_from_list(beginning, &(board->foods));
//     add_new_food(board);
    return Status::SUCCESS;
   }
   return Status::SUCCESS;
//   // Check for food
//   if (list_contains(beginning, board->foods)) {
//     // Attach the beginning to the rest of the snake;
//     beginning->next = board->snake;
// 
//     return SUCCESS;
//   }
// 
//   // Attach the beginning to the rest of the snake
//   beginning->next = board->snake;
//   board->snake = beginning;
// 
// 
//   // Cut off the end
//   PointList* end = board->snake;
//   while(end->next->next) {
//     end = end->next;
//   }
//   free(end->next);
//   end->next = NULL;
// 
//   return SUCCESS;
}


fn main() {
    println!("Guess the number!");
}
