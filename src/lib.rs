use std::fs::read_to_string;

pub struct SuggestionsRepository {
    suggestions: Vec<String>,
}

impl SuggestionsRepository {
    pub fn new() -> SuggestionsRepository {
        SuggestionsRepository {
            suggestions: Vec::new(),
        }
    }

    pub fn new_from_file(file: &str) -> SuggestionsRepository {
        let mut file_content = Vec::new();

        for line in read_to_string(file).unwrap().lines() {
            file_content.push(line.to_string());
        }

        SuggestionsRepository {
            suggestions: file_content,
        }
    }

    pub fn get_suggestion_by_index(&self, index: usize) -> &str {
        &self.suggestions[index]
    }

    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    pub fn size(&self) -> usize {
        self.suggestions.len()
    }
}

#[cfg(test)]
use std::fs::{self, File};

#[cfg(test)]
use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_repository() {
        let db = SuggestionsRepository::new();
        assert!(db.is_empty());
    }

    #[test]
    fn empty_repository_size() {
        let db = SuggestionsRepository::new();
        assert_eq!(db.size(), 0);
    }

    #[test]
    fn empty_repository_from_existing_file() {
        let path = "./empty_file.txt";
        File::create(path).unwrap();

        let db = SuggestionsRepository::new_from_file(path);
        assert!(db.is_empty());
        
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn non_empty_repository_from_existing_file() {
        let path = "./non_empty_file.txt";
        let mut file = File::create(path).unwrap();

        writeln!(file, "first line").unwrap();
        writeln!(file, "second line").unwrap();

        let db = SuggestionsRepository::new_from_file(path);
        assert!(!db.is_empty());
        assert_eq!(db.size(), 2);

        fs::remove_file(path).unwrap();
    }

    #[test]
    #[should_panic]
    fn panic_on_empty_repository() {
        let db = SuggestionsRepository::new();

        let _suggestion = db.get_suggestion_by_index(0);
    }

    #[test]
    #[should_panic]
    fn panic_on_non_existing_file() {
        let _db = SuggestionsRepository::new_from_file("./non_existing_file.txt");
    }

}
