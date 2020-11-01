use std::fmt;
use crate::playfield::{Square, PlayField};
use crate::read_string;
use std::hint::unreachable_unchecked;

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
    pub my_mark: Square,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum GameOutcome {
    Lost,
    Draw,
    DrawWin,
    Win
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let symbol = match self.my_mark {
            Square::Cross  => {'X'},
            Square::Circle => {'O'},
            Square::Empty => unreachable!()
        };
        write!(f, "Player {} ({})", self.name, symbol)
    }
}

pub fn new_player_with_type(p_name: u8, type_str: &str, mark: Square) -> Box<dyn PlayerBehaviour>{
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
            let res = playfield.mark_square_by_str(&input_str, self.player.my_mark);
            //println!("Debug: Thanks for typing: {:?}", coord);
            match res {
                Ok(()) => {
                    {
                        break;
                    }
                }
                Err(e) => println!("Invalid coordinate: {}", e)
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
        let c_order:[usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        for c in c_order.iter() {
            match playfield.mark_square(*c, self.player.my_mark) {
                Ok(()) => {
                    break;
                },
                _ => {}
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
        if coord > 8 {
            println!("The game ended DRAW!");
        }
        else {
            if playfield.mark_square(coord, self.player.my_mark).is_err()
            {
                unreachable!();
            }
        }
    }
}

impl SmartAiPlayer {
    pub fn calc_best_square(&self, playfield: &PlayField) -> usize {
        let mut best_square = 9; //default = invalid
        let mut best_outcome: GameOutcome = GameOutcome::Lost;
        // check winning ratio for each open square
        println!("==== Calculate best square ====");
        for i in 0..9 {
                match playfield.value_of(i) {
                    Square::Empty => {
                        //default, set best_square to the first empty field
                        if best_square == 9 {
                            best_square = i;
                        }
                        let mut temp_pf = playfield.clone();
                        temp_pf.mark_square(i, self.player.my_mark);
                        let tmp_outcome = self.calc_winning_ratio(&temp_pf, self.player.my_mark,
                                                                  self.player.my_mark.toggle());
                        println!("Field – Chance for {:?} is {:?}", i, tmp_outcome);
                        if tmp_outcome > best_outcome {
                            best_square = i;
                            best_outcome = tmp_outcome;
                        }
                    },
                    _ => {}
            }
        }
        println!("==== Best square: {:?}, best ratio: {:?} ====", best_square, best_outcome);
        return best_square;
    }

    pub fn calc_winning_ratio(&self, playfield: &PlayField, player_mark: Square,
                              turn_mark: Square) -> GameOutcome {
        println!("Coming in with {}", playfield);
        let mut res = GameOutcome::Draw;
        // 1. Check if somebody won
        match playfield.check_if_somebody_won() {
            Some(x) => {
                if x == player_mark {
                        println!("Player {} won. {}, return Win", player_mark, playfield);
                        return GameOutcome::Win;
                } else if x == player_mark.toggle() {
                        println!("Player {} lost. {}, return Lost", player_mark, playfield);
                        return GameOutcome::Lost;
                }
            }
            None => {}
        }

        let mut any_empty_fields = false;
        // 2. If not, check if empty fields
        for i in 0..9 {
            match playfield.value_of(i) {
                Square::Empty => {
                    any_empty_fields = true;
                    let mut temp_pf = playfield.clone();
                    println!("On field: ({}) with: {}, player {}", i, turn_mark, player_mark);
                    temp_pf.mark_square(i, turn_mark);
                    let tmp_res = self.calc_winning_ratio(&temp_pf, player_mark, turn_mark.toggle());
                    // expect the most intelligent decision from the other player:
                    if turn_mark == player_mark {
                        if tmp_res > res
                        {
                            res = tmp_res
                        }
                    } else if turn_mark == player_mark.toggle() {
                        // It's the other players turn, so we'll expect him to
                        // take the worst (for us) choice. But we want to keep the
                        // info if we have the change of winning, if other options are draw.
                        match tmp_res {
                            GameOutcome::Win => {
                                //DrawWin and Win include the win,
                                //Lost is lost anyway
                                if res == GameOutcome::Draw
                                {
                                    res = GameOutcome::DrawWin;
                                }
                            },
                            GameOutcome::DrawWin => {
                                //if res is already Win, DrawWin, no change
                                //if res is Lost, we keep that, too.
                                if res == GameOutcome::Draw {
                                    res = GameOutcome::DrawWin;
                                }
                            },
                            GameOutcome::Draw => {
                                //if res is DrawWin or Draw, no change
                                //if res is Lost, we'll keep that.
                                if res == GameOutcome::Win {
                                    res = GameOutcome::DrawWin;
                                }
                            },
                            GameOutcome::Lost => res = GameOutcome::Lost
                        }
                    } else {
                        unreachable!()
                    }
                },
                _ => {}
            }
        }
        let final_res = return match any_empty_fields {
            //in case of all fields full, this will return draw
            true => res,
            //draw, nobody won
            false => GameOutcome::Draw
        };
        println!("Returning {:?}", final_res);
        return final_res;
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
