use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

//mod crate::player;
use crate::player::{Player, new_player_with_type, SmartAiPlayer};
use crate::playfield::{PlayField, SquareMarkType};

/// TicTacToe tests

#[test]
fn accept_human_stupidai() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictactoe")?;
    cmd.arg("human");
    cmd.arg("stupid-ai");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn accept_stupidai_smartai() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictactoe")?;
    cmd.arg("stupid-ai");
    cmd.arg("smart-ai");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn not_accept_other_playertypes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tictactoe")?;
    cmd.arg("rezo");
    cmd.arg("xaviernaidoo");
    cmd.assert()
        .failure();
    Ok(())
}

#[test]
fn test_player_parse_coord() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(Player::parse_square_coordinate("a1").unwrap(), (0, 0));
    Ok(())
}


#[test]
fn test_playfield_draw() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 0), SquareMarkType::Circle);
    pf.mark_square((0, 1), SquareMarkType::Cross);
    pf.mark_square((0, 2), SquareMarkType::Circle);
    pf.mark_square((1, 0), SquareMarkType::Circle);
    pf.mark_square((1, 1), SquareMarkType::Cross);
    pf.mark_square((1, 2), SquareMarkType::Circle);
    pf.mark_square((2, 0), SquareMarkType::Cross);
    pf.mark_square((2, 1), SquareMarkType::Circle);
    pf.mark_square((2, 2), SquareMarkType::Cross);
    assert_eq!(pf.check_if_somebody_won(), None);
    Ok(())
}

#[test]
fn test_playfield_who_won_left_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 0), SquareMarkType::Circle);
    pf.mark_square((0, 1), SquareMarkType::Circle);
    pf.mark_square((0, 2), SquareMarkType::Circle);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Circle));
    Ok(())
}

#[test]
fn test_playfield_who_won_right_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((2, 0), SquareMarkType::Cross);
    pf.mark_square((2, 1), SquareMarkType::Cross);
    pf.mark_square((2, 2), SquareMarkType::Cross);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Cross));
    Ok(())
}


#[test]
fn test_playfield_who_won_top_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 0), SquareMarkType::Circle);
    pf.mark_square((1, 0), SquareMarkType::Circle);
    pf.mark_square((2, 0), SquareMarkType::Circle);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Circle));
    Ok(())
}

#[test]
fn test_playfield_who_won_bottom_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 2), SquareMarkType::Cross);
    pf.mark_square((1, 2), SquareMarkType::Cross);
    pf.mark_square((2, 2), SquareMarkType::Cross);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Cross));
    Ok(())
}

#[test]
fn test_playfield_who_won_diagonal_down() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 0), SquareMarkType::Circle);
    pf.mark_square((1, 1), SquareMarkType::Circle);
    pf.mark_square((2, 2), SquareMarkType::Circle);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Circle));
    Ok(())
}

#[test]
fn test_playfield_who_won_diagonal_up() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    pf.mark_square((0, 2), SquareMarkType::Cross);
    pf.mark_square((1, 1), SquareMarkType::Cross);
    pf.mark_square((2, 0), SquareMarkType::Cross);
    assert_eq!(pf.check_if_somebody_won(), Some(SquareMarkType::Cross));
    Ok(())
}

#[test]
fn test_smart_algo_final_field() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    // X X O
    // O O X
    // X O -
    pf.mark_square((0, 0), SquareMarkType::Cross);
    pf.mark_square((0, 1), SquareMarkType::Cross);
    pf.mark_square((0, 2), SquareMarkType::Circle);
    pf.mark_square((1, 0), SquareMarkType::Circle);
    pf.mark_square((1, 1), SquareMarkType::Circle);
    pf.mark_square((1, 2), SquareMarkType::Cross);
    pf.mark_square((2, 0), SquareMarkType::Cross);
    pf.mark_square((2, 1), SquareMarkType::Circle);

    let player = Player {
        name: 1,
        my_mark: SquareMarkType::Cross,
    };
    let smart_ai = SmartAiPlayer { player: player };

    assert_eq!(smart_ai.calc_best_square(&pf), (2, 2));
    //assert_eq!(smart_ai.calc_winning_ratio(&pf, SquareMarkType::Cross), (0, 0, 1));
    Ok(())
}

#[test]
fn test_smart_algo_two_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut pf = PlayField::new_empty();
    // X X O
    // O X O
    // X - -
    pf.mark_square((0, 0), SquareMarkType::Cross);
    pf.mark_square((1, 0), SquareMarkType::Cross);
    pf.mark_square((2, 0), SquareMarkType::Circle);
    pf.mark_square((0, 1), SquareMarkType::Circle);
    pf.mark_square((1, 1), SquareMarkType::Cross);
    pf.mark_square((2, 1), SquareMarkType::Circle);
    pf.mark_square((0, 2), SquareMarkType::Cross);

    let player = Player {
        name: 1,
        my_mark: SquareMarkType::Circle,
    };
    let smart_ai = SmartAiPlayer { player: player };

    assert_eq!(smart_ai.calc_best_square(&pf), (1, 2));
    //assert_eq!(smart_ai.calc_winning_ratio(&pf, SquareMarkType::Circle), (1, 1, 2));
    Ok(())
}

#[test]
fn test_smart_algo_three_options() -> Result<(), Box<dyn std::error::Error>> {
    println!("==== Test Three Option");
    let mut pf = PlayField::new_empty();
    // O X O
    // O X X
    // - - -
    pf.mark_square((0, 0), SquareMarkType::Circle);
    pf.mark_square((1, 0), SquareMarkType::Cross);
    pf.mark_square((2, 0), SquareMarkType::Circle);
    pf.mark_square((0, 1), SquareMarkType::Circle);
    pf.mark_square((1, 1), SquareMarkType::Cross);
    pf.mark_square((2, 1), SquareMarkType::Cross);

    let player = Player {
        name: 1,
        my_mark: SquareMarkType::Cross,
    };
    let smart_ai = SmartAiPlayer { player: player };
    assert_eq!(smart_ai.calc_best_square(&pf), (1, 2));
    //////////////assert_eq!(smart_ai.calc_winning_ratio(&pf, SquareMarkType::Cross), (2, 1, 5));
    Ok(())
}
