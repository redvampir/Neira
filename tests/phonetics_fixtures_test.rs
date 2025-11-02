/* neira:meta
id: NEI-20251101-phonetics-fixtures-test
intent: test
summary: Валидация структуры фикстур phonetics_samples.json (id, audio_path, transcription_phonetic, meta).
*/
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Sample {
    id: String,
    audio_path: String,
    transcription_phonetic: String,
    #[allow(dead_code)]
    meta: serde_json::Value,
}

#[test]
fn phonetics_fixtures_have_expected_shape() {
    let path = "spinal_cord/training/fixtures/phonetics_samples.json";
    let raw = fs::read_to_string(path).expect("fixtures file should exist");
    let samples: Vec<Sample> = serde_json::from_str(&raw).expect("valid json array of samples");
    assert!(!samples.is_empty(), "at least one sample present");
    for s in samples {
        assert!(!s.id.trim().is_empty());
        assert!(!s.audio_path.trim().is_empty());
        assert!(!s.transcription_phonetic.trim().is_empty());
    }
}
