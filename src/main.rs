use std::{io, time::Duration};
use std::io::Write;


use rand::Rng;

use crossterm::{
    cursor::{self},
    event::{read, poll, Event, KeyCode, KeyEvent},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, 
    Result,
    style::{self, Stylize}, QueueableCommand,
};


const USAGE_TO_PLAY: &str = 
"   Key  |  Action\n\r--------|--------\n\r    z   |   Up   \n\r    s   |   Down \n\r    q   |   Left \n\r    d   |   Right\n\r";
const OTHERS_USAGE: &str = "    a   |   Quit \n\r ctrl+c |   Quit \n\r";
const GRID_SIZE:  usize = 20;
const DELTA_TIME: u64 = 200;
const GAMEOVER: &str = "  ____                       ___\n\r / ___| __ _ _ __ ___   ___ / _ \\__   _____ _ __\n\r| |  _ / _` | '_ ` _ \\ / _ \\ | | \\ \\ / / _ \\ '__|\n\r| |_| | (_| | | | | | |  __/ |_| |\\ V /  __/ |\n\r \\____|\\__,_|_| |_| |_|\\___|\\___/  \\_/ \\___|_|\n\r";


#[derive(Debug)]
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

struct Coordinate {
    x: usize,
    y: usize, // Do not forget to multiply by grid_size ...
}

struct Player {
    direction: PlayerDirection,
    body: Vec<Coordinate>
}
impl Player {
    fn new(game_size: usize) -> Player {
        let range = 0+(game_size as f64 *0.20) as usize..(game_size as f64 *0.90) as usize;

        Player { 
            direction: PlayerDirection::Down,
            body: vec![
                Coordinate { 
                    x: rand::thread_rng().gen_range(range.clone()),
                    y: rand::thread_rng().gen_range(range)
                }
            ]
        }
    }

    fn eat(&mut self, x: usize, y: usize) {
        self.body.push( Coordinate {
            x: x,
            y: y,
        })
    }

    fn up(&mut self) {
        unimplemented!()
    }
    
    fn down(&mut self) {
        unimplemented!()
    }
    
    fn left(&mut self) {
        unimplemented!()
    }
    
    fn right(&mut self) {
        unimplemented!()
    }
    
    fn forward(&mut self) {
        unimplemented!()
    }
}

struct Food {
    body: Coordinate
}
impl Food {
    fn new(game_size: usize) -> Food {
        let range = 0+(game_size as f64 *0.20) as usize..(game_size as f64 *0.90) as usize;

        Food {
            body: Coordinate { 
                x: rand::thread_rng().gen_range(range.clone()),
                y: rand::thread_rng().gen_range(range)
            }
        }
    }
}


#[derive(Clone, Copy)]
enum GameContent {
    Empty,
    Snake,
    Food,

}

struct Screen {
    size: usize,
    buffer: Vec<GameContent>,
}
impl Screen {
    fn new(size: usize) -> Screen {
        Screen {
            size: size,
            buffer: vec![GameContent::Empty; size*size],
        }
    }
    
    fn get_content_at(&self, x: usize, y: usize) -> GameContent {
        self.buffer[x + y * self.size]
    }

    fn set_content_at(&mut self, x: usize, y: usize, content_type: GameContent) {
        self.buffer[x + y * self.size] = content_type;        
    }

    fn draw(&self, stdout: &mut std::io::Stdout) -> Result<()> {

        for y in 0..self.size {
            for x in 0..self.size {
                let content = self.get_content_at(x, y);

                let styled_content = match content {
                    GameContent::Empty => style::PrintStyledContent( ".".white()),
                    GameContent::Snake => style::PrintStyledContent( "O".white()),
                    GameContent::Food => style::PrintStyledContent( "o".white())
                };

                stdout.queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(styled_content)?;
            }
        }

        stdout.flush()
    }

    fn update_buffer(&mut self, player: &Player, food: &Food) {
        /* Update buffer with Player position */
        for el in &player.body {
            self.set_content_at(el.x, el.y, GameContent::Snake)
        }

        /* Update buffer with Food position */
        self.set_content_at(food.body.x, food.body.y, GameContent::Food)

    }
}

fn main() -> Result<()> {
    
    /* Configuring the terminal into raw mode and hiding the cursor */ 
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;
    stdout.flush()?;
    
    let mut stdin = io::stdin();

    /* Instance the screen */
    let mut screen = Screen::new(GRID_SIZE);


    /* Instance the player */
    let mut player = Player::new(GRID_SIZE);

    /* Instance the food */
    let mut food = Food::new(GRID_SIZE);

    screen.update_buffer(&player, &food);

    screen.draw(&mut stdout)?;

    'game_loop:  loop {

        if poll(Duration::from_millis(500))? {

            let event = read()?;

            if event == Event::Key(KeyCode::Esc.into()) {
                break 'game_loop
            } else if event == Event::Key(KeyCode::Char('z').into()) {
                player.up();
            } else if event == Event::Key(KeyCode::Char('s').into()) {
                player.down();
            } else if event == Event::Key(KeyCode::Char('q').into()) {
                player.left();
            } else if event == Event::Key(KeyCode::Char('d').into()) {
                player.right();
            }
        }

        player.forward();


    }
    disable_raw_mode()
}

