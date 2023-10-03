#[cfg(test)]
use super::*;

// Define tests for the game logic
#[test]
fn test_win_game() {
    // Simulate a game where the player wins by guessing the correct number
    let mut game = GuessingGame::new(42);
    assert_eq!(game.guess(42), GameResult::Win);
}

#[test]
fn test_too_low() {
    // Simulate a game where the player's guess is too low
    let mut game = GuessingGame::new(75);
    assert_eq!(game.guess(50), GameResult::TooLow);
}

#[test]
fn test_too_high() {
    // Simulate a game where the player's guess is too high
    let mut game = GuessingGame::new(25);
    assert_eq!(game.guess(50), GameResult::TooHigh);
}

