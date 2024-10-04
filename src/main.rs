use ruggestions::SuggestionsRepository;
use std::error::Error;
use std::fmt;

struct Config {
    suggestions_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, ConfigError> {
        match args.len() {
            2 => Ok(Config {
                suggestions_file: args[1].clone(),
            }),
            _ => Err(ConfigError::new("No suggestion file was provided")),
        }
    }
}

#[derive(Debug, Clone)]
struct ConfigError {
    description: String,
}

impl ConfigError {
    fn new(description: &str) -> ConfigError {
        ConfigError {
            description: description.to_string(),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        &self.description
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args)?;
    let _db = SuggestionsRepository::new_from_file(config.suggestions_file.as_str());

    println!(
        "Loaded suggestions repository from: {}",
        config.suggestions_file
    );

    Ok(())
}
