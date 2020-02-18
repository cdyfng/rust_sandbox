//use std::fmt::{Display, Formatter};
use pancurses::{Window, Input, initscr, endwin};

enum VertDir {
    Up,
    Down,
}
enum HorizDir {
    Left,
    Right,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}


impl Game{
    fn new(window: &Window) -> Result<Game, String> {
        let (max_y, max_x) = window.get_max_yx();
        if max_y < 10 || max_x < 10 {
            return Err(String::from("Window is too small, exiting"));
        }
        let frame = Frame {
            width : max_x as u32 - 2, 
            height : max_y as u32 - 2, 
        };
        let ball = Ball {
            x : 2,
            y : 4,
            vert_dir : VertDir::Up,
            horiz_dir : HorizDir::Left,
        };
        Ok(Game {frame, ball})
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width -1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Up;
        } else if self.y == frame.height -1 {
            self.vert_dir = VertDir::Down;
        }       
    }

    fn mv(&mut self){
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }

        match self.vert_dir {
            VertDir::Up => self.y += 1,
            VertDir::Down => self.y -= 1,
        }

    }
}


// impl Display for Game {
//     fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
//         let top_bottom =  |fmt: &mut Formatter| {
//             write!(fmt, "+");
//             for _ in 0..self.frame.width {
//                 write!(fmt, "-");
//             }
//             write!(fmt, "+\n");
//         };

//         let body = |fmt: &mut Formatter| {
//             for row in 0..self.frame.height {
//                 write!(fmt, "|");
//                 // more code will go here
//                 for column in 0..self.frame.width {
//                     let c = if row == self.ball.y && column == self.ball.x {
//                         'o'
//                     } else {
//                         ' '
//                     };
//                     write!(fmt, "{}", c);
//                 }
//                 write!(fmt, "|\n");
//             }
//         };


//         top_bottom(fmt);
//         body(fmt);
//         top_bottom(fmt);
//         Ok(())
//     }
// }

pub fn run()-> Result<(), String> {
    let window = initscr();
    window.timeout(33);

    let mut game = Game::new(&window)?;
    loop{
        window.clear();
        window.border(
            '|', // left
            '|', // right
            '-', // top
            '-', // bottom
            '+', // top left
            '+', // top right
            '+', // bottom left
            '+', // bottom right           
        );
        window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
        window.mv(0, 0);

        window.refresh();
        match window.getch(){
            Some(Input::Character('q')) => {
                endwin();
                println!("Thanks for playing!");
                return Ok(());   
            }
            _ => {
                game.step();
            }    
        }
    }
    //let sleep_duration = std::time::Duration::from_millis(33);
    // loop{
    //     println!("{}", game);
    //     game.step();
    //     std::thread::sleep(sleep_duration);
    // }
}
