/* neira:meta
id: NEI-20280401-120000-russian-curriculum
intent: feature
summary: |
  Добавлен загрузчик учебного курса по русскому алфавиту: валидация
  алфавита, слогов и слов, конвертация в JSON и получение статистики.
*/
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub const DEFAULT_RUSSIAN_CURRICULUM_PATH: &str = "static/training/russian_literacy.json";
pub const RUSSIAN_CURRICULUM_ID: &str = "russian_literacy_v1";
pub const MIN_INQUIRY_VOCABULARY_WORDS: usize = 10;
pub const MAX_INQUIRY_VOCABULARY_WORDS: usize = 30;
const INQUIRY_VOCABULARY_PURPOSE: &str = "inquiry_vocabulary";
const INQUIRY_THEMES: &[&str] = &["семья", "жильё", "еда", "город", "природа"];

#[derive(Debug, Error)]
pub enum CurriculumError {
    #[error("ошибка чтения файла: {0}")]
    Io(#[from] std::io::Error),
    #[error("ошибка JSON: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("некорректные данные курса: {0}")]
    Validation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphabetEntry {
    pub upper: String,
    pub lower: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub sound: String,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyllableEntry {
    pub syllable: String,
    pub letters: Vec<String>,
    pub structure: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEntry {
    pub word: String,
    pub syllables: Vec<String>,
    pub meaning: String,
    pub theme: String,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InquiryVocabularySeed {
    pub id: String,
    pub language: String,
    pub purpose: String,
    pub words: Vec<WordEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RussianLiteracyCurriculum {
    pub id: String,
    pub language: String,
    pub alphabet: Vec<AlphabetEntry>,
    pub syllables: Vec<SyllableEntry>,
    pub words: Vec<WordEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurriculumSummary {
    pub letters: usize,
    pub syllables: usize,
    pub words: usize,
}

impl RussianLiteracyCurriculum {
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self, CurriculumError> {
        let data = fs::read_to_string(path)?;
        let curriculum: Self = serde_json::from_str(&data)?;
        curriculum.validate()?;
        Ok(curriculum)
    }

    pub fn load_default() -> Result<Self, CurriculumError> {
        Self::load_from_path(default_curriculum_path())
    }

    pub fn validate(&self) -> Result<(), CurriculumError> {
        if self.id != RUSSIAN_CURRICULUM_ID {
            return Err(CurriculumError::Validation(format!(
                "ожидался идентификатор {}, получен {}",
                RUSSIAN_CURRICULUM_ID, self.id
            )));
        }
        if self.language != "ru" {
            return Err(CurriculumError::Validation(format!(
                "ожидался язык ru, получен {}",
                self.language
            )));
        }
        if self.alphabet.len() != 33 {
            return Err(CurriculumError::Validation(format!(
                "алфавит должен содержать 33 буквы, найдено {}",
                self.alphabet.len()
            )));
        }
        let mut unique_letters = HashSet::new();
        for entry in &self.alphabet {
            if entry.upper.trim().is_empty() || entry.lower.trim().is_empty() {
                return Err(CurriculumError::Validation(
                    "буква в алфавите не может быть пустой".into(),
                ));
            }
            if !unique_letters.insert(entry.upper.clone()) {
                return Err(CurriculumError::Validation(format!(
                    "дубликат буквы {} в алфавите",
                    entry.upper
                )));
            }
        }
        if self.syllables.is_empty() {
            return Err(CurriculumError::Validation(
                "список слогов не может быть пустым".into(),
            ));
        }
        let mut syllable_map: HashMap<&str, &SyllableEntry> = HashMap::new();
        for syllable in &self.syllables {
            if syllable.syllable.trim().is_empty() {
                return Err(CurriculumError::Validation(
                    "обнаружен пустой слог".into(),
                ));
            }
            if syllable_map
                .insert(syllable.syllable.as_str(), syllable)
                .is_some()
            {
                return Err(CurriculumError::Validation(format!(
                    "дубликат слога {}",
                    syllable.syllable
                )));
            }
        }
        if self.words.is_empty() {
            return Err(CurriculumError::Validation(
                "словарь не может быть пустым".into(),
            ));
        }
        if self.words.len() > 100 {
            return Err(CurriculumError::Validation(format!(
                "допустимо не более 100 слов, найдено {}",
                self.words.len()
            )));
        }
        for word in &self.words {
            if word.word.trim().is_empty() {
                return Err(CurriculumError::Validation(
                    "обнаружено пустое слово".into(),
                ));
            }
            if word.syllables.is_empty() {
                return Err(CurriculumError::Validation(format!(
                    "слово {} не содержит слогов",
                    word.word
                )));
            }
            let mut composed = String::new();
            for syll in &word.syllables {
                if let Some(entry) = syllable_map.get(syll.as_str()) {
                    if entry.letters.is_empty() {
                        return Err(CurriculumError::Validation(format!(
                            "слог {} в слове {} не содержит букв",
                            syll, word.word
                        )));
                    }
                } else {
                    return Err(CurriculumError::Validation(format!(
                        "слог {} в слове {} не найден в общем списке",
                        syll, word.word
                    )));
                }
                composed.push_str(syll);
            }
            if composed != word.word {
                return Err(CurriculumError::Validation(format!(
                    "слова {} и набор его слогов {} не совпадают",
                    word.word,
                    word.syllables.join("-")
                )));
            }
        }
        Ok(())
    }

    pub fn summary(&self) -> CurriculumSummary {
        CurriculumSummary {
            letters: self.alphabet.len(),
            syllables: self.syllables.len(),
            words: self.words.len(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn to_json_value(&self) -> Result<Value, CurriculumError> {
        serde_json::to_value(self).map_err(|err| {
            CurriculumError::Validation(format!(
                "не удалось сериализовать учебный курс: {err}"
            ))
        })
    }

    pub fn build_inquiry_seed(&self) -> Result<InquiryVocabularySeed, CurriculumError> {
        let mut selected = self
            .words
            .iter()
            .filter(|word| is_inquiry_theme(&word.theme) && has_meaning(word))
            .cloned()
            .collect::<Vec<_>>();

        sort_seed_words(&mut selected);
        let mut known = selected
            .iter()
            .map(|entry| entry.word.clone())
            .collect::<HashSet<_>>();

        if selected.len() < MIN_INQUIRY_VOCABULARY_WORDS {
            let mut fallback = self.words.clone();
            sort_seed_words(&mut fallback);
            for word in fallback {
                if known.contains(&word.word) || !has_meaning(&word) {
                    continue;
                }
                known.insert(word.word.clone());
                selected.push(word);
                if selected.len() >= MIN_INQUIRY_VOCABULARY_WORDS {
                    break;
                }
            }
        }

        if selected.len() < MIN_INQUIRY_VOCABULARY_WORDS {
            return Err(CurriculumError::Validation(format!(
                "недостаточно слов для базового словаря вопросов: найдено {}, требуется минимум {}",
                selected.len(),
                MIN_INQUIRY_VOCABULARY_WORDS
            )));
        }

        selected.truncate(MAX_INQUIRY_VOCABULARY_WORDS);

        Ok(InquiryVocabularySeed {
            id: format!("{}/inquiry_seed", self.id),
            language: self.language.clone(),
            purpose: INQUIRY_VOCABULARY_PURPOSE.to_string(),
            words: selected,
        })
    }
}

impl InquiryVocabularySeed {
    pub fn to_json_value(&self) -> Result<Value, CurriculumError> {
        serde_json::to_value(self).map_err(|err| {
            CurriculumError::Validation(format!(
                "не удалось сериализовать словарь вопросов: {err}"
            ))
        })
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    pub fn word_list(&self) -> Vec<String> {
        self.words.iter().map(|entry| entry.word.clone()).collect()
    }
}

fn is_inquiry_theme(theme: &str) -> bool {
    INQUIRY_THEMES.iter().any(|candidate| candidate == &theme)
}

fn has_meaning(word: &WordEntry) -> bool {
    !word.meaning.trim().is_empty()
}

fn sort_seed_words(words: &mut Vec<WordEntry>) {
    words.sort_by(|a, b| a.level.cmp(&b.level).then_with(|| a.word.cmp(&b.word)));
}

pub fn default_curriculum_path() -> PathBuf {
    let relative = PathBuf::from(DEFAULT_RUSSIAN_CURRICULUM_PATH);
    if relative.exists() {
        relative
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_RUSSIAN_CURRICULUM_PATH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn curriculum_loads_and_validates() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("static/training/russian_literacy.json");
        let curriculum = RussianLiteracyCurriculum::load_from_path(&path)
            .expect("curriculum should load");
        assert_eq!(curriculum.id(), RUSSIAN_CURRICULUM_ID);
        let summary = curriculum.summary();
        assert_eq!(summary.letters, 33);
        assert!(summary.words <= 100);
        assert!(summary.syllables > summary.words);
    }
}

