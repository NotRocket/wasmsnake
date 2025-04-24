use rand::prelude::*;
use std::collections::VecDeque;
use speedy2d::window::VirtualKeyCode;
#[derive(Copy, Clone,PartialEq)]
pub enum TileState{
    SnakeOccupied,
    FoodOccupied,
    BorderDeathZone,
    Free
}
#[derive(Copy, Clone)]
pub struct Tile{
    pub state: TileState,
    pub x_coordinate: usize,
    pub y_coordinate: usize,
}
pub struct GameBoard{
    pub board: Vec<Vec<Tile>>,
    length: usize,
    width: usize,
}
impl GameBoard{
    pub fn new(length: usize, width: usize) -> GameBoard{
        let buildaboard: Vec<Vec<Tile>> = (0..width).map(|y| {
            (0..length).map(|x| {
                Tile {
                    state: TileState::Free,
                    x_coordinate: x,
                    y_coordinate: y,
                }
            }).collect()
        }).collect();

        let mut _board = GameBoard{
            board: buildaboard,
            length,
            width,
        };
        _board.generate_borders();
        _board.board[_board.length / 2][_board.width / 2].state = TileState::SnakeOccupied;
        _board.spawn_food();

        _board
    }

    fn generate_borders(&mut self){
        // top row
        for tile in self.board[0].iter_mut(){
            tile.state = TileState::BorderDeathZone;
        }
        // bottom row
        for tile in self.board[self.length-1].iter_mut(){
            tile.state = TileState::BorderDeathZone;
        }
        for edge in 0..self.width-1{
            self.board[edge][0].state = TileState::BorderDeathZone; // left edge
            self.board[edge][self.width -1].state = TileState::BorderDeathZone; // right edge
        }
    }

    pub fn spawn_food(&mut self){
        let mut freepool: Vec<Tile> = Vec::new();
        for y in &self.board{
            for x in y{
                if x.state == TileState::Free{
                    freepool.push(x.clone());
                }
            }
        }

        let mut rng = rand::thread_rng();
        let mut selectedTile = freepool.choose(&mut rng).unwrap();
        self.board[selectedTile.y_coordinate][selectedTile.x_coordinate].state = TileState::FoodOccupied;
    }
}

pub struct Snake{
    head: Tile,
    tail: Tile,
    body: VecDeque<Tile>,
    pub maxlength: usize,
    pub alive: bool,
}

impl Snake{
    pub fn new(mut board: &mut GameBoard) -> Snake{
        let mut head = board.board[board.length / 2][board.width / 2];
        head.state = TileState::SnakeOccupied;
        board.board[head.y_coordinate][head.x_coordinate].state = TileState::SnakeOccupied;
        let mut body = VecDeque::from(vec![head]);
        Snake{
            head: head,
            tail: head,
            body: body,
            maxlength: 1,
            alive: true,
        }
    }
    pub fn move_snake(&mut self, board: &mut GameBoard, key: VirtualKeyCode){
        match key{
            VirtualKeyCode::W => {
                self.head = board.board[self.head.y_coordinate - 1][self.head.x_coordinate];
            },
            VirtualKeyCode::S => {
                self.head = board.board[self.head.y_coordinate + 1][self.head.x_coordinate];
            },
            VirtualKeyCode::A => {
                self.head = board.board[self.head.y_coordinate][self.head.x_coordinate - 1];
            },
            VirtualKeyCode::D => {
                self.head = board.board[self.head.y_coordinate][self.head.x_coordinate + 1];
            },
            _ => {}
        }
        match board.board[self.head.y_coordinate][self.head.x_coordinate].state{
            TileState::FoodOccupied => {
                board.board[self.head.y_coordinate][self.head.x_coordinate].state = TileState::SnakeOccupied;
                self.body.push_back(self.head);
                self.maxlength += 1;
                board.spawn_food();

            },
            TileState::BorderDeathZone => {
                self.alive = false;
            },
            TileState::SnakeOccupied => {
                self.alive = false;
            }
            TileState::Free => {
                board.board[self.head.y_coordinate][self.head.x_coordinate].state = TileState::SnakeOccupied;
                self.body.push_back(self.head);
            }
        }
        if self.body.len() > self.maxlength{
            let mut removed_tile = self.body.pop_front().unwrap();
            board.board[removed_tile.y_coordinate][removed_tile.x_coordinate].state = TileState::Free;
        }

    }
}