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
