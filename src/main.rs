use snake::Pos;
use snake::Direction;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use crossterm::{event, terminal, cursor, event::KeyCode, ExecutableCommand};

mod snake;
mod config;

enum ThreadMsg {
    SnakeDied,
    Quit,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let config = config::Config::new(&args);

    if config.num_food > ((config.board_width * config.board_height) / 2) as u32 {
        eprintln!("Too much food!");
        std::process::exit(-1);
    }

    let term_size = (
        terminal::size().unwrap().0 as usize,
        terminal::size().unwrap().1 as usize,
    );
    let check_x = (config.board_width + 1) * 2 > term_size.0;
    let check_y = config.board_height + 3 > term_size.1;
    if check_x || check_y {
        eprintln!(
            "Error: terminal not large enough for specified dimensions. x: {}, y: {}",
            check_x, check_y
        );
        std::process::exit(-1);
    }

    let mut snake = snake::Snake::new(
        config.board_width,
        config.board_height,
        Pos {
            x: config.board_width / 2,
            y: config.board_height / 2 
        },
        config.num_food,
        Direction::Right);

    let (key_tx, key_rx) = mpsc::channel::<event::KeyCode>();
    let (msg_tx, msg_rx) = mpsc::channel::<ThreadMsg>();

    let input_thread = thread::spawn(move || {
        let mut last_key = KeyCode::Null;

        loop {
            match msg_rx.try_recv() {
                Ok(msg) => match msg {
                    ThreadMsg::SnakeDied => last_key = KeyCode::Null,
                    ThreadMsg::Quit => break,
                },
                Err(_) => {},
            }

            if let Ok(true) = event::poll(std::time::Duration::from_millis(0)) {
                if let event::Event::Key(key) =
                    event::read().expect("An error occured while getting input")
                {
                    if key.code != last_key {
                        key_tx.send(key.code).unwrap();
                        last_key = key.code;
                    }
                }
            }
        }
    });

    std::io::stdout().execute(cursor::Hide).unwrap();

    'outer: loop {
        snake.reset();
        cursor_move(0, 0);
        clear();
    
        println!("{}", snake);
    
        let msg = "Press press a movement key to begin";
        cursor_move(((config.board_width * 2 + 2) / 2 - msg.len().min(config.board_width) / 2 - 2) as u16, (config.board_height / 2 + 2) as u16);
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
    
        // flush key press queue
        while key_rx.try_recv().is_ok() {}
        loop {
            match key_rx.recv().unwrap() {
                KeyCode::Up | KeyCode::Char('w') => {
                    snake.set_dir(Direction::Up, false);
                    break;
                }
                KeyCode::Down | KeyCode::Char('s') => {
                    snake.set_dir(Direction::Down, false);
                    break;
                }
                KeyCode::Left | KeyCode::Char('a') => {
                    snake.set_dir(Direction::Left, false);
                    break;
                }
                KeyCode::Right | KeyCode::Char('d') => {
                    snake.set_dir(Direction::Right, false);
                    break;
                }
                KeyCode::Esc => {
                    clear();
                    cursor_move(0, 0);
                    break 'outer;
                }
                _ => {}
            }
        }
    
        cursor_move(0, 0);
        clear();
        loop {
            if snake.dead() {
                println!("You died!");
                msg_tx.send(ThreadMsg::SnakeDied).unwrap();
                std::thread::sleep(std::time::Duration::from_millis(2500));
                break;
            }
    
            if let Ok(key) = key_rx.try_recv() {
                match key {
                    KeyCode::Up | KeyCode::Char('w') => snake.set_dir(Direction::Up, true),
                    KeyCode::Down | KeyCode::Char('s') => snake.set_dir(Direction::Down, true),
                    KeyCode::Left | KeyCode::Char('a') => snake.set_dir(Direction::Left, true),
                    KeyCode::Right | KeyCode::Char('d') => snake.set_dir(Direction::Right, true),
                    KeyCode::Esc => break 'outer,
                    _ => {}
                }
            }
    
            snake.step();
            purge();
            cursor_move(0, 0);
            println!("{}", snake);
            std::thread::sleep(std::time::Duration::from_millis(1000 - config.speed.clamp(0, 999)));
        }
    }
    
    msg_tx.send(ThreadMsg::Quit).unwrap();
    input_thread.join().unwrap();
    std::io::stdout().execute(cursor::Show).unwrap();
}

fn clear() {
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
}

fn purge() {
    std::io::stdout()
        .execute(terminal::Clear(terminal::ClearType::Purge))
        .unwrap();
}

fn cursor_move(x: u16, y: u16) {
    std::io::stdout().execute(cursor::MoveTo(x, y)).unwrap();
}