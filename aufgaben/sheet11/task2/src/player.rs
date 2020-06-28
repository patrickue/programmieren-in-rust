use std::fmt;
use crate::playfield::{SquareMarkType, PlayField};
use crate::read_string;

pub const PLAYER_TYPES: &'static[&str] = &[
    "human",
    "smart-ai",
    "stupid-ai"
];

#[derive(Debug, Copy, Clone)]
pub enum PlayerType {
    Human,
    SmartAI,
    StupidAI
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub name:  u8,
    pub my_mark: SquareMarkType,
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let symbol = match self.my_mark {
            SquareMarkType::Cross  => {'X'},
            SquareMarkType::Circle => {'O'}
        };
        write!(f, "Player {} ({})", self.name, symbol)
    }
}

pub fn new_player_with_type(p_name: u8, type_str: &str, mark: SquareMarkType) -> Box<dyn PlayerBehaviour>{
    let p = Player{
        name: p_name,
        my_mark: mark,
    };

    match type_str {
        "human" => {
            Box::new( HumanPlayer {
                player: p
            })
        }
        "smart-ai" => {
            Box::new(SmartAiPlayer {
                player: p
            })
        }
        "stupid-ai" => {
            Box::new(StupidAiPlayer {
                player: p
            })
        }
        _ => unreachable!(),
    }
}

impl Player{
    pub fn parse_square_coordinate(text: &str) -> Option<(usize, usize)>
    {
        if text.len() != 2 {
            return None;
        }
        let x_coord = match text.chars().nth(0).unwrap() {
            'a' => Some(0),
            'A' => Some(0),
            'b' => Some(1),
            'B' => Some(1),
            'c' => Some(2),
            'C' => Some(2),
            _ => None
        };
        let y_coord = match text.chars().nth(1).unwrap() {
            '1' => Some(0),
            '2' => Some(1),
            '3' => Some(2),
            _ => None
        };

        Some((x_coord?, y_coord?))
    }

}

pub trait PlayerBehaviour: fmt::Display {
    fn choose_and_mark_square(&self, pf: &mut PlayField) -> ();
}

pub struct HumanPlayer {
    player: Player
}

impl PlayerBehaviour for HumanPlayer {
    fn choose_and_mark_square(&self, playfield: &mut PlayField) -> () {
        loop {
            println!("{}, please choose an empty square by typing the coordinate (i.e. 'b2')",
                     self);

            let input_str = read_string();
            let coord = Player::parse_square_coordinate(&input_str);
            //println!("Debug: Thanks for typing: {:?}", coord);
            match coord {
                Some(c) => {
                    if playfield.mark_square(c, self.player.my_mark) == Ok(())
                    {
                        break;
                    }
                }
                None => println!("Invalid coordinate, try again")
            }
        }
    }
}

impl fmt::Display for HumanPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.player)
    }
}

pub struct StupidAiPlayer {
    player: Player
}


impl PlayerBehaviour for StupidAiPlayer {
    fn choose_and_mark_square(&self, playfield: &mut PlayField) -> () {
        let c_order:[(usize, usize); 9] = [(0,0), (0,1), (0,2), (1,0), (1,1), (1,2), (2,0), (2,1), (2,2)];
        for c in c_order.iter() {
            if playfield.mark_square(*c, self.player.my_mark) == Ok(())
            {
                break;
            }
        }
    }
}

impl fmt::Display for StupidAiPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.player)
    }
}

pub struct SmartAiPlayer {
    pub player: Player
}

impl PlayerBehaviour for SmartAiPlayer {
    fn choose_and_mark_square(&self, playfield: &mut PlayField) -> () {
        println!("{} is Smart-AI an choosing.", self);

        let coord = self.calc_best_square(playfield);
        println!("Smart-AI chose: {:?}", coord);
        if playfield.mark_square(coord, self.player.my_mark) == Err(())
        {
            unreachable!()
        }
    }
}

impl SmartAiPlayer {
    pub fn calc_best_square(&self, playfield: &PlayField) -> (usize, usize) {
        let mut best_square = (3, 3);
        let mut best_ratio = (0, 0, 1);
        // check winning ratio for each open square
        for i in 0..3 {
            for j in 0..3 {
                match playfield.value_of((i, j)) {
                    None => {
                        //default, set best_square to the first empty field
                        if best_square==(3, 3) {
                            best_square = (i, j);
                        }
                        let mut temp_pf = playfield.clone();
                        temp_pf.mark_square((i, j), self.player.my_mark);
                        let tmp_res = self.calc_winning_ratio(&temp_pf, self.player.my_mark.toggle());
                        println!("Winning resolution for {:?} is {:?}", (i, j), tmp_res);
                        match self.player.my_mark {
                            SquareMarkType::Cross => {
                                if (tmp_res.0*best_ratio.2) > (best_ratio.0*tmp_res.2) {
                                    // Actual comparison, but divisor can be zero
                                    //((tmp_res.0) / (tmp_res.2)) > ((best_ratio.0) / (best_ratio.2))
                                    //old: (2, 0)
                                    best_square = (i, j);
                                    best_ratio = tmp_res;
                                }
                            },
                            SquareMarkType::Circle => {
                                if (tmp_res.1 * best_ratio.2) > (best_ratio.2 * tmp_res.1) {
                                    // ((tmp_res.1) / (tmp_res.2)) > ((best_ratio.1) / (best_ratio.2))
                                    best_square = (i, j);
                                    best_ratio = tmp_res;
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        println!("Best square: {:?}, best ratio: {:?}", best_square, best_ratio);
        return best_square;
    }

    pub fn calc_winning_ratio(&self, playfield: &PlayField, mark: SquareMarkType) -> (usize, usize, usize) {
        println!("Coming in with {}", playfield);
        let mut res = (0, 0, 0);
        // 1. Check if somebody won
        match playfield.check_if_somebody_won() {
            Some(x) => {
                match x {
                    SquareMarkType::Cross => {
                        println!("Cross won {}", playfield);
                        return (1, 0, 1);
                    },
                    SquareMarkType::Circle => {
                        println!("Circle won {}", playfield);
                        return (0, 1, 1);
                    },
                }
            }
            None => {}
        }

        let mut any_empty_fields = false;
        // 2. If not, check if empty fields
        for i in 0..3 {
            for j in 0..3 {
                match playfield.value_of((i, j)) {
                    None => {
                        any_empty_fields = true;
                        let mut temp_pf = playfield.clone();
                        //println!("On field: ({}{}) with: {}", i, j, mark);
                        temp_pf.mark_square((i, j), mark);
                        let tmp_res = self.calc_winning_ratio(&temp_pf, mark.toggle());
                        // expect the most intelligent decision from the other player:
                        match mark {
                            SquareMarkType::Cross => {
                                if (tmp_res.0*res.2) > (res.0*tmp_res.2) {
                                    res = tmp_res;
                                }
                            },
                            SquareMarkType::Circle => {
                                if (tmp_res.1*res.2) > (res.1*tmp_res.2) {
                                    res = tmp_res;
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        return match any_empty_fields {
            //in case of all fields full, this will return (0, 0)
            true => res,
            //draw, nobody won
            false => (0, 0, 1)
        }
    }
}

impl fmt::Display for SmartAiPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.player)
    }
}

fn add_points(a: (usize, usize, usize), b: (usize, usize, usize)) -> (usize, usize, usize) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}
