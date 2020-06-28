use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SquareMarkType {
    Cross,
    Circle
}

impl SquareMarkType {
    pub fn toggle(&self) -> Self {
        use SquareMarkType::*;
        match *self {
            Cross => Circle,
            Circle => Cross,
        }
    }
}

impl fmt::Display for SquareMarkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            SquareMarkType::Cross  => { write!(f, "X") },
            SquareMarkType::Circle => { write!(f, "O") }
        }
    }
}

/// Implement some displaying for Square
// This is the marker we'll use to define our custom Display impl.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(Option<SquareMarkType>);

// This trait lets us extend Option<MyStruct> with a new method.
trait CustomSquareDisplay {
    fn display(self) -> Square;
}

impl CustomSquareDisplay for Option<SquareMarkType> {
    fn display(self) -> Square {
        Square(self)
    }
}

// And here's the display logic.
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref ms) => write!(f, "{}", ms),
            None => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlayField {
    field: [[Square; 3]; 3]
}

impl fmt::Display for PlayField {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "\n  a b c\n")?;
        write!(f, "1|{} {} {}\n", self.field[0][0], self.field[1][0], self.field[2][0])?;
        write!(f, "2|{} {} {}\n", self.field[0][1], self.field[1][1], self.field[2][1])?;
        write!(f, "3|{} {} {}\n", self.field[0][2], self.field[1][2], self.field[2][2])?;
        Ok(())
    }
}

impl PlayField {
    pub fn new_empty() -> PlayField {
        PlayField{
            field: [[Square(None); 3]; 3]
        }
    }

    pub fn mark_square(&mut self, pt: (usize, usize), mark: SquareMarkType) -> Result<(), ()> {
        match self.field[pt.0][pt.1] {
            Square(None) => {
                self.field[pt.0][pt.1] = Square(Some(mark));
                Ok(())
            }
            Square(Some(x)) => Err(()),
        }
    }

    pub fn value_of(&self, pt: (usize, usize)) -> Option<SquareMarkType> {
        self.field[pt.0][pt.1].0.clone()
    }

    pub fn check_if_somebody_won(&self) -> Option<SquareMarkType> {
        // Check vertical rows
        for i in 0..3 {
            match self.field[i][0] {
                Square(Some(x)) => {
                    let first = self.field[i][0];
                    if self.field[i].iter().all(|&item| item == first)
                    {
                        return Some(x);
                    }
                },
                _ => {}
            }
        }
        // Check horizontal rows
        for i in 0..3 {
            match self.field[0][i] {
                Square(Some(x)) => {
                    if self.field[0][i] == self.field[1][i] &&
                    self.field[0][i] == self.field[2][i]
                    {
                        return Some(x);
                    }
                },
                _ => {}
            }
        }
        //Check Diagonal lines
        // for diagonal win, the middle field has to be some
        match self.field[1][1] {
            Square(Some(x)) => {
                //compare middle to upper left and lower right
                if self.field[0][0] == self.field[1][1] &&
                    self.field[2][2] == self.field[1][1]
                {
                    return Some(x);
                }

                //compare middle to lower left and upper right
                if self.field[2][0] == self.field[1][1] &&
                    self.field[0][2] == self.field[1][1]
                {
                    return Some(x);
                }
            }
            _ => {}
        }
        None
    }
}