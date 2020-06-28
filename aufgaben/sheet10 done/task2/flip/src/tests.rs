use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions


/// Coin tests

#[test]
fn normal_coin_throw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("coin");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn multiple_coin_throw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("-t=20")
        .arg("coin");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("heads"));
    Ok(())
}

#[test]
fn coin_throw_times_wrong() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("-t=2x0")
        .arg("coin");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No valid number for --times."));
    Ok(())
}

/// Dice tests

#[test]
fn dice_normal() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("dice");
    cmd.assert()
        .success();

    Ok(())
}

#[test]
fn dice_with_sides() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("dice")
        .arg("--sides=66");
    cmd.assert()
        .success();

    Ok(())
}

#[test]
fn dice_wrong_number_sides() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("dice")
        .arg("--sides=None");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Not a number for sides!"));

    Ok(())
}

/// Choose tests

#[test]
fn choose_three_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("choose")
        .arg("--count=3")
        .arg("Peter")
        .arg("Paul")
        .arg("Mary");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Peter"))
        .stdout(predicate::str::contains("Paul"))
        .stdout(predicate::str::contains("Mary"));

    Ok(())
}

#[test]
fn choose_count_more_than_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("choose")
        .arg("--count=4")
        .arg("Peter")
        .arg("Paul")
        .arg("Mary");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Count cannot be higher than number of elements."));

    Ok(())
}

#[test]
fn choose_too_little_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("flip")?;
    cmd.arg("choose")
        .arg("--count=1")
        .arg("Mary");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Specify at least two options to choose from."));

    Ok(())
}

