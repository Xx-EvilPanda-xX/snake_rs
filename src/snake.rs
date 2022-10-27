use crossterm::style::Colors;
use crossterm::style::Color;
use crossterm::ExecutableCommand;
use dyn_array::DynArray;
use rand::Rng;

pub struct Snake {
    field: Field,
    dir: Direction,
    head: Pos,
    tail: Pos,
    dead: bool,
    score: u32,
    best: u32,
    just_ate: bool,
    start_dir: Direction,
    start_pos: Pos,
    num_food: u32,
    closest_food: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Cell {
    Food(char),
    Snake(Option<Pos>),
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

type Field = DynArray<Cell, 2>;

impl Snake {
    pub fn new(width: usize, height: usize, start_pos: Pos, num_food: u32, start_dir: Direction) -> Self {
        let mut new = Self {
            field: Snake::init_field(width, height, start_pos, num_food),
            dir: start_dir,
            head: start_pos,
            tail: start_pos,
            dead: false,
            score: 0,
            best: 0,
            just_ate: false,
            start_dir,
            start_pos,
            num_food,
            closest_food: Pos { x: 0, y: 0 },
        };

        new.closest_food = Self::get_closest_food(start_pos, &new.field);
        new
    }

    fn init_field(width: usize, height: usize, start_pos: Pos, num_food: u32) -> Field {
        let mut field = Field::new([width, height], Cell::Nothing);

        field[[start_pos.x, start_pos.y]] = Cell::Snake(None);

        for _ in 0..num_food {
            Self::place_rand_food(&mut field);
        }

        field
    }

    pub fn set_dir(&mut self, dir: Direction, check: bool) {
        if check {
            match dir {
                Direction::Up => {
                    if self.dir != Direction::Down {
                        self.dir = dir;
                    }
                }
                Direction::Down => {
                    if self.dir != Direction::Up {
                        self.dir = dir;
                    }
                }
                Direction::Left => {
                    if self.dir != Direction::Right {
                        self.dir = dir;
                    }
                }
                Direction::Right => {
                    if self.dir != Direction::Left {
                        self.dir = dir;
                    }
                }
            }
        } else {
            self.dir = dir;
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn best(&self) -> u32 {
        self.best
    }

    pub fn dead(&self) -> bool {
        self.dead
    }

    pub fn reset(&mut self) {
        self.field = Snake::init_field(self.field.width(), self.field.height(), self.start_pos, self.num_food);
        self.dir = self.start_dir;
        self.head = self.start_pos;
        self.tail = self.start_pos;
        self.dead = false;
        self.score = 0;
        self.closest_food = Self::get_closest_food(self.head, &self.field);
    }

    pub fn step(&mut self) {
        self.just_ate = false;
        if self.dead {
            return;
        }

        let w = self.field.width();
        let h = self.field.height();
        if let Cell::Snake(next) = &mut self.field[[self.head.x, self.head.y]] {
            match self.dir {
                Direction::Up => {
                    if self.head.y == 0 {
                        self.dead = true;
                        return;
                    }
                    self.head.y -= 1;
                }
                Direction::Down => {
                    if self.head.y >= h - 1 {
                        self.dead = true;
                        return;
                    }
                    self.head.y += 1;
                }
                Direction::Right => {
                    if self.head.x >= w - 1 {
                        self.dead = true;
                        return;
                    }
                    self.head.x += 1;
                }
                Direction::Left => {
                    if self.head.x == 0 {
                        self.dead = true;
                        return;
                    }
                    self.head.x -= 1;
                }
            }

            *next = Some(self.head);
        } else {
            panic!("Invalid snake head");
        }

        let at_head = &mut self.field[[self.head.x, self.head.y]];
        let last_cell = at_head.clone();
        *at_head = Cell::Snake(None);

        let at_tail = &mut self.field[[self.tail.x, self.tail.y]];
        match last_cell {
            Cell::Food(_) => {
                self.score += 1;
                if self.score > self.best {
                    self.best = self.score;
                }

                Self::place_rand_food(&mut self.field);
                self.just_ate = true;
            }
            Cell::Snake(_) => self.dead = true,
            Cell::Nothing => if let Cell::Snake(Some(next)) = at_tail {
                self.tail = *next;
                *at_tail = Cell::Nothing;
            } else {
                panic!("Invalid snake tail");
            }
        }

        self.closest_food = Self::get_closest_food(self.head, &self.field);
    }

    fn place_rand_food(field: &mut Field) {
        if field.iter().filter(|(_, i)| matches!(i, Cell::Nothing)).count() == 0 {
            return;
        }

        loop {
            let x = rand::thread_rng().gen_range(0..field.width());
            let y = rand::thread_rng().gen_range(0..field.height());
            if let Cell::Nothing = field[[x, y]] {
                field[[x, y]] = Cell::Food('*');
                break;
            }
        }
    }

    fn get_closest_food(head_pos: Pos, field: &Field) -> Pos {
        let foods = field.iter().filter(|&cell| matches!(cell.1, Cell::Food(..)));
        let mut closest = (0.0, Pos { x: 0, y: 0 });

        for (food, _) in foods {
            let diff_x = head_pos.x as f32 - food[0] as f32;
            let diff_y = head_pos.y as f32 - food[1] as f32;
            let distance = (diff_x * diff_x + diff_y * diff_y).sqrt();

            if closest.0 == 0.0 || distance < closest.0 {
                closest = (distance, Pos { x: food[0], y: food[1] });
            }
        }

        closest.1
    }
}

impl std::fmt::Display for Snake {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Score: {} | Best: {}\n\r", self.score(), self.best())?;

        // top row of `-`
        for _ in 0..self.field.width() {
            write!(f, " -")?;
        }
        write!(f, " -\n\r|")?;

        // cells and side `|`
        for (pos, cell) in self.field.iter() {
            let i = pos[1] * self.field.width() + pos[0];
            if i % self.field.width() == 0 && i != 0 {
                write!(f, " |\n\r|")?;
            }

            write!(f, "{}", ' ')?;
            match cell {
                Cell::Food(c) => {
                    if (Pos { x: pos[0], y: pos[1] }) == self.closest_food {
                        set_color(Color::Magenta);
                        write!(f, "{}", *c)?;
                        set_color(Color::White);
                    } else {
                        write!(f, "{}", *c)?;
                    }
                }
                Cell::Snake(None) => {
                    set_color(if self.just_ate { Color::Green } else { Color::Blue });
                    write!(f, "{}", '@')?;
                    set_color(Color::White);
                }
                Cell::Snake(Some(..)) => {
                    set_color(if self.just_ate { Color::Green } else { Color::Red });
                    write!(f, "{}", '#')?;
                    set_color(Color::White);
                }
                Cell::Nothing => write!(f, "{}", ' ')?,
            }
        }

        // bottom row of `-`
        write!(f, " |\n\r")?;
        for _ in 0..self.field.width() {
            write!(f, " -")?;
        }
        write!(f, " -")?;

        Ok(())
    }
}

fn set_color(color: Color) {
    std::io::stdout()
        .execute(
            crossterm::style::SetColors(
                Colors {
                    foreground: Some(color),
                    background: None
                }
            )
        ).unwrap();
}