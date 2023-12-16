use colored::Colorize;

// Used for HOT
pub fn hot(string: &str, user_guess: i32) -> String {
    // Now we need to convert user_guess into a string
    let user_guess = user_guess.to_string();
    return format!("{} {}", user_guess.bold().red(), string.bold().red());
}
// Used for WARM
pub fn warm(string: &str, user_guess: i32) -> String {
    let user_guess = user_guess.to_string();
    return format!("{} {}", user_guess.bold().yellow(), string.bold().yellow());
}
// Used for COOL
pub fn cool(string: &str, user_guess: i32) -> String {
    let user_guess = user_guess.to_string();
    return format!("{} {}", user_guess.bold().cyan(), string.bold().cyan());
}
// Used for COLD
pub fn cold(string: &str, user_guess: i32) -> String {
    let user_guess = user_guess.to_string();
    return format!("{} {}", user_guess.bold().blue(), string.bold().blue());
}