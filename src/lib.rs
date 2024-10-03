pub struct SuggestionsRepository {
    suggestions: Vec<String>, 
}

impl SuggestionsRepository {
    pub fn new() -> SuggestionsRepository {
        SuggestionsRepository {
            suggestions: Vec::new(),
        }
    }

    pub fn get_suggestion_by_index(&self, index: usize) -> &str {
        &self.suggestions[index]
    }

    pub fn is_empty(&self) -> bool {
        self.suggestions.is_empty()
    }

    pub fn len(&self) -> usize {
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
        assert_eq!(db.len(), 0);
    }

    #[test]
    #[should_panic]
    fn panic_on_empty_repository() {
        let db = SuggestionsRepository::new();

        let _suggestion = db.get_suggestion_by_index(0);
    }
}
