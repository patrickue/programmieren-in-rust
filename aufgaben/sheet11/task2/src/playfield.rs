use std::fmt;
use std::error;

#[derive(Debug, Clone)]
struct WrongCoordError;

#[derive(Debug, Clone)]
struct FieldOccupiedError;

impl error::Error for WrongCoordError {}
impl error::Error for FieldOccupiedError {}

impl fmt::Display for WrongCoordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given String cannot be parsed into a valid coordinate")
    }
}

impl fmt::Display for FieldOccupiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The field to be marked has already been marked before")
    }
}

/// Implement some displaying for Square
// This is the marker we'll use to define our custom Display impl.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    Cross,
    Circle,
    Empty
}

impl Square{
    pub fn toggle(&self) -> Self {
        match *self {
            Square::Cross => Square::Circle,
            Square::Circle => Square::Cross,
            Square::Empty => unreachable!()
        }
    }
}

// And here's the display logic.
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Cross => write!(f, "𐄂"),
            Square::Circle => write!(f, "◯"),
            Square::Empty => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayField {
    field: [Square; 9]
}

impl fmt::Display for PlayField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\n  a b c\n")?;
        write!(f, "1|{} {} {}\n", self.field[0], self.field[1], self.field[2])?;
        write!(f, "2|{} {} {}\n", self.field[3], self.field[4], self.field[5])?;
        write!(f, "3|{} {} {}\n", self.field[6], self.field[7], self.field[8])?;
        Ok(())
    }
}

impl PlayField {
    pub fn new_empty() -> PlayField {
        PlayField{
            field: [Square::Empty; 9]
        }
    }

    pub fn mark_square_by_str(&mut self, text: &str, mark: Square) -> Result<(), Box<error::Error>>
    {
        if text.len() != 2 {
            return Err(Box::new(WrongCoordError));
        }
        let x_coord = match text.chars().nth(0).unwrap() {
            'a' => Ok(0),
            'A' => Ok(0),
            'b' => Ok(1),
            'B' => Ok(1),
            'c' => Ok(2),
            'C' => Ok(2),
            _ => Err(WrongCoordError)
        };
        let y_coord = match text.chars().nth(1).unwrap() {
            '1' => Ok(0),
            '2' => Ok(1),
            '3' => Ok(2),
            _ => Err(WrongCoordError)
        };
        let coord = x_coord? + 3 * y_coord?;
        self.mark_square(coord, mark)
    }

    pub fn mark_square(&mut self, field_nr: usize, mark: Square) -> Result<(), Box<error::Error>> {
        match self.field[field_nr] {
            Square::Empty => {
                self.field[field_nr] = mark;
                Ok(())
            }
            _ => Err(Box::new(FieldOccupiedError)),
        }
    }

    pub fn value_of(&self, idx: usize) -> Square {
        self.field[idx].clone()
    }

    // TODO Switch to Coord instead of usize
    pub fn compare_three_squares(&self, a: usize, b: usize, c: usize) -> Option<Square>
    {
        return match self.field[a] {
            Square::Empty => {None}
            _ => {
                if (self.field[a] == self.field[b] &&
                    self.field[b] == self.field[c])
                {
                    Some(self.field[a])
                }
                else { None }
            }
        };
    }

    pub fn check_if_somebody_won(&self) -> Option<Square> {
        // Check vertical rows
        self.compare_three_squares(0, 1, 2).or(
        self.compare_three_squares(3, 4, 5).or(
        self.compare_three_squares(6, 7, 8).or(
        self.compare_three_squares(6, 7, 8).or(
        // Check horizontal rows
        self.compare_three_squares(0, 3, 6).or(
        self.compare_three_squares(1, 4, 7).or(
        self.compare_three_squares(2, 5, 8).or(
        //Check Diagonal lines
        self.compare_three_squares(0, 4, 8).or(
        self.compare_three_squares(2, 4, 6)
        ))))))))
    }
}