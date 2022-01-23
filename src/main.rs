// #![allow(unused_variables)]
// #![allow(dead_code)]

use std::io;
use std::io::Stdout;
use std::io::Write;
use std::collections::LinkedList;
use std::thread;
use std::time;

use rand::Rng;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayerDirection {
    Down,
    Up,
    Left,
    Right,
}
impl PlayerDirection {
    fn opposite(&self) -> PlayerDirection {
        match *self {
            PlayerDirection::Up => PlayerDirection::Down,
            PlayerDirection::Down => PlayerDirection::Up,
            PlayerDirection::Left => PlayerDirection::Right,
            PlayerDirection::Right => PlayerDirection::Left
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Block {
    x: i32,
    y: i32,
}

struct Player {
    grid_size: i32,
    direction: PlayerDirection,
    body: LinkedList<Block>,
    symbole: String,
    popped_tail: Option<Block>,
}
impl Player {
    fn new(grid_size: i32) -> Player {
        let mut body = LinkedList::new();
        body.push_back(Block{
            x: grid_size/2,
            y: grid_size/2
        });

        Player {
            grid_size: grid_size,
            direction: PlayerDirection::Down,
            body: body,
            symbole: String::from("0"),
            popped_tail: None,
        }
    }

    fn forward(&mut self, dir: &PlayerDirection) {
        let mut _x = 0;
        let mut _y = 0;
        match dir {
            PlayerDirection::Down   => _y += 1,
            PlayerDirection::Up     => _y -= 1,
            PlayerDirection::Left   => _x -= 1,
            PlayerDirection::Right  => _x += 1,
        }

        let _body = self.body.clone();
        let _head = _body.front();
        if let Some(b) = _head {
            if _x + b.x > self.grid_size - 1 {
                _x = -b.x
            } else if _x + b.x < 0 {
                _x = self.grid_size
            }

            if _y + b.y > self.grid_size -1 {
                _y = -b.y
            } else if _y + b.y < 0 {
                _y = self.grid_size
            }
            self.body.push_front(Block {
                x: _x + b.x,
                y: _y + b.y,
            });
        }
        
        self.popped_tail = self.body.pop_back();
    }

    fn grow(&mut self, object_body: &Block) {
        self.body.push_front(object_body.clone());
    }

    fn get_direction(&self) -> PlayerDirection {
        self.direction
    }

    fn get_size(&self) -> usize {
        self.body.len()
    }

    fn check_eat_himself(&self) -> bool {
        let mut _body = self.body.clone();
        let mut _head = _body.front().unwrap().clone();
        _body.split_off(1).contains(&mut _head)
    }
}

struct Object {
    body: Block,
    symbole: String,
}
impl Object {
    fn new(grid_size: i32) -> Object {
        Object {
            body: Block {
                x: rand::thread_rng().gen_range(0..grid_size),
                y: rand::thread_rng().gen_range(0..grid_size)
            },
            symbole: String::from("*")
        }
    }
}

fn draw_game(stdout: &mut RawTerminal<Stdout>, grid: &mut [String], grid_size: i32, player: &Player, object: &Object) {

    const USAGE_TO_PLAY: &str = 
        "   Key  |  Action\n\r--------|--------\n\r    z   |   Up   \n\r    s   |   Down \n\r    q   |   Left \n\r    d   |   Right\n\r";
    const OTHERS_USAGE: &str = "    a   |   Quit \n\r ctrl+c |   Quit \n\r";
    
    // Add on the grid the body of the snake
    for block in &player.body {
        let _x = block.x as usize;
        grid[block.y as usize].replace_range(_x..=_x, player.symbole.as_str());
    }

    // Remove the last tail (as it forwards)
    if let Some(b) = &player.popped_tail {
        let _x = b.x as usize;
        grid[b.y as usize].replace_range(_x..=_x, ".");
    }

    // Add on the grid the object
    let _x = object.body.x as usize;
    grid[object.body.y as usize].replace_range(_x..=_x, object.symbole.as_str());

    write!(stdout, "{}\n\r{}{}", grid.join("\n\r"), USAGE_TO_PLAY, OTHERS_USAGE)
        .expect("[draw_game] Failed to write to stdout\n\r");

}

fn check_player_object(player: &Player, object_to_eat: &Object, _dir: &PlayerDirection) -> bool{
    &object_to_eat.body == player.body.front().unwrap()
}

fn main() {
    
    /* Configuring the terminal */ 
    // Stdout in raw mode
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    
    // Use asynchronous stdin
    let mut stdin = termion::async_stdin().keys();

    stdout.flush().unwrap();
    
    const GRID_SIZE:  i32 = 30;
    const DELTA_TIME: u64 = 200;
    const GAMEOVER: &str = "  ____                       ___\n\r / ___| __ _ _ __ ___   ___ / _ \\__   _____ _ __\n\r| |  _ / _` | '_ ` _ \\ / _ \\ | | \\ \\ / / _ \\ '__|\n\r| |_| | (_| | | | | | |  __/ |_| |\\ V /  __/ |\n\r \\____|\\__,_|_| |_| |_|\\___|\\___/  \\_/ \\___|_|\n\r";
    let mut game_lose = false;

    // Create the player (snake)
    let mut player = Player::new(GRID_SIZE);

    // Instanciate a new Object to eat
    let mut object_to_eat = Object::new(GRID_SIZE);

    let mut dir = PlayerDirection::Down;

    // Initialize the grid
    const EMPTY_STRING: String = String::new();
    let mut grid: [String; GRID_SIZE as usize] = [EMPTY_STRING; GRID_SIZE as usize];
    for i in 0..grid.len() {
        grid[i] = String::from(".".repeat(GRID_SIZE as usize));
    }

    loop {
        let prev_dir = dir;
        
        // Getting the input from the player
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            dir = match key {
                    // Exit if 'a' or ctr+c is pressed
                    termion::event::Key::Char('a')  => break,
                    termion::event::Key::Ctrl('c')  => break,
                    termion::event::Key::Char('s')  => PlayerDirection::Down,
                    termion::event::Key::Down       => PlayerDirection::Down,
                    termion::event::Key::Char('z')  => PlayerDirection::Up,
                    termion::event::Key::Up         => PlayerDirection::Up,
                    termion::event::Key::Char('q')  => PlayerDirection::Left,
                    termion::event::Key::Left       => PlayerDirection::Left,
                    termion::event::Key::Char('d')  => PlayerDirection::Right,
                    termion::event::Key::Right      => PlayerDirection::Right,
                    _ => dir
            };
        }

        if dir == player.get_direction().opposite() && player.get_size() > 1 {
            dir = prev_dir;
        } else {
            player.direction = dir
        }

        
        if check_player_object(&player, &object_to_eat, &dir) {
            // He grow by the front so he go forward implicitly when eating
            player.grow(&object_to_eat.body);
            object_to_eat = Object::new(GRID_SIZE);
        } else {
            player.forward(&dir);
            if player.check_eat_himself() {
                eprintln!("Eated himself !");
                game_lose = true;
                break;
            }
        }
        
        
        // Drawing the game
        draw_game(&mut stdout, &mut grid, GRID_SIZE, &player, &object_to_eat);

        
        thread::sleep(time::Duration::from_millis(DELTA_TIME));

        write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Hide)
            .expect("[main] Failed to clear screen");
    }


    if game_lose {
        writeln!(stdout, "{}", GAMEOVER)
            .expect("[main] Failed to write to stdout\n\r");
    }


}

