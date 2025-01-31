use passwords::analyzer::analyze;
use passwords::PasswordGenerator;
use passwords::scorer::score;
use crate::domain::app::error::AppError;
use crate::domain::cli::password_params::PasswordParams;
use crate::utils::constants::*;

pub fn generate_pwd(params: PasswordParams) -> Result<String, AppError> {
    if !(PASSWORD_MIN_LEN..PASSWORD_MAX_LEN).contains(&params.length) {
        return Err(AppError::Other(
            format!(
                "Password length must be between {} and {} characters",
                PASSWORD_MIN_LEN,
                PASSWORD_MAX_LEN
            ))
        );
    }
    let generator = PasswordGenerator::new()
        .length(params.length)
        .numbers(true)
        .symbols(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .symbols(true)
        .exclude_similar_characters(params.avoid_ambiguous);
    Ok(generator.generate_one().unwrap())
}

pub fn analyze_pwd(password: String) -> (f64, String) {
    let analyzed = analyze(password);
    let score = score(&analyzed);
    let classification = if score < 20.0 {
        "Very dangerous"
    } else if score < 40.0 {
        "Dangerous"
    } else if score < 60.0 {
        "Very weak"
    } else if score < 80.0 {
        "Weak"
    } else if score < 90.0 {
        "Medium"
    } else if score < 95.0 {
        "Strong"
    } else if score < 99.0 {
        "Very strong"
    } else {
        "Invulnerable"
    };
    (score, classification.to_string())
}