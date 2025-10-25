/* neira:meta
id: NEI-20280105-voice-phonemes
intent: code
summary: |
  Простая фонетическая карта для русского текста: даёт стабильный вывод
  для TTS и вспомогательных метрик.
*/

pub fn phonemize(text: &str) -> String {
    text.split_whitespace()
        .map(phonemize_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn phonemize_word(word: &str) -> String {
    let mut result = Vec::new();
    for ch in word.chars() {
        let mapped = match ch.to_lowercase().next().unwrap_or(ch) {
            'а' => Some("A".to_string()),
            'б' => Some("B".to_string()),
            'в' => Some("V".to_string()),
            'г' => Some("G".to_string()),
            'д' => Some("D".to_string()),
            'е' | 'ё' => Some("YE".to_string()),
            'ж' => Some("ZH".to_string()),
            'з' => Some("Z".to_string()),
            'и' | 'й' => Some("I".to_string()),
            'к' => Some("K".to_string()),
            'л' => Some("L".to_string()),
            'м' => Some("M".to_string()),
            'н' => Some("N".to_string()),
            'о' => Some("O".to_string()),
            'п' => Some("P".to_string()),
            'р' => Some("R".to_string()),
            'с' => Some("S".to_string()),
            'т' => Some("T".to_string()),
            'у' => Some("U".to_string()),
            'ф' => Some("F".to_string()),
            'х' => Some("H".to_string()),
            'ц' => Some("TS".to_string()),
            'ч' => Some("CH".to_string()),
            'ш' => Some("SH".to_string()),
            'щ' => Some("SCH".to_string()),
            'ы' => Some("Y".to_string()),
            'э' => Some("E".to_string()),
            'ю' => Some("YU".to_string()),
            'я' => Some("YA".to_string()),
            'ъ' | 'ь' => None,
            '-' => Some("-".to_string()),
            other => {
                if other.is_ascii_alphanumeric() {
                    Some(other.to_uppercase().collect::<String>())
                } else {
                    None
                }
            }
        };
        if let Some(p) = mapped {
            result.push(p);
        }
    }
    if result.is_empty() {
        word.to_uppercase()
    } else {
        result.join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::phonemize;

    #[test]
    fn basic_phonemize() {
        let res = phonemize("Нейра говорит");
        assert_eq!(res, "N-YE-I-R-A G-O-V-O-R-I-T");
    }
}
