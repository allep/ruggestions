use ruggestions::SuggestionsRepository;

struct Config {
    suggestions_file: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        assert!(args.len() > 1, "No suggestion file was provided");

        Config {
            suggestions_file: args[1].clone(),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);
    let _db = SuggestionsRepository::new_from_file(config.suggestions_file.as_str());
}
