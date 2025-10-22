/* neira:meta
id: NEI-20270405-trigger-toxicity-test
intent: test
summary: Проверяет фильтрацию токсичных слов перед TriggerDetector.
*/
use backend::trigger_detector::TriggerDetector;

#[test]
fn censors_toxic_words_before_detection() {
    let detector = TriggerDetector::default();
    detector.add_keyword("идиот".into());
    let found = detector.detect("Ты ИДИОТ");
    assert!(found.is_empty());
}

#[test]
fn detects_keywords_case_insensitive() {
    let detector = TriggerDetector::default();
    let found = detector.detect("RUST делает системы живыми");
    assert_eq!(found, vec!["rust".to_string()]);
}
