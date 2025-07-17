use crate::types::nope::Nope;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct NopeService {
    nopes: Vec<Nope>,
}

impl NopeService {
    pub fn new() -> Self {
        let nopes_json = include_str!("../nopes.json");
        let nopes: Vec<Nope> =
            serde_json::from_str(nopes_json).expect("Failed to parse nopes.json");

        Self { nopes }
    }

    pub fn get_random_nope(&self, lang: Option<String>) -> Option<&Nope> {
        let mut rng = rand::thread_rng();

        // If a language is specified, try to find a nope in that language.
        if let Some(lang_code) = lang {
            let lang_nopes: Vec<&Nope> = self
                .nopes
                .iter()
                .filter(|n| n.language == lang_code)
                .collect();

            if !lang_nopes.is_empty() {
                return lang_nopes.choose(&mut rng).copied();
            }
        }

        // Default to English if no language was specified or if the specified language had no nopes.
        let en_nopes: Vec<&Nope> = self
            .nopes
            .iter()
            .filter(|n| n.language == "en")
            .collect();

        if !en_nopes.is_empty() {
            return en_nopes.choose(&mut rng).copied();
        }

        // As a final fallback, return any random nope if no English nopes are found.
        self.nopes.choose(&mut rng)
    }
}
