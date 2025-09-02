use std::collections::{HashMap, VecDeque};

/* neira:meta
id: NEI-20260214-loop-detector
intent: code
summary: |-
  Sliding window detector for repetitive SSE sequences; publishes loop_detected_total.
*/

/// Check incoming tokens for loops or low entropy.
///
/// Returns Some(ratio) if a loop is detected and increments `loop_detected_total`.
#[allow(clippy::float_cmp)]
pub fn check_sequence(
    win: &mut VecDeque<String>,
    token: &str,
    window: usize,
    threshold: f32,
    entropy_min: f32,
) -> Option<f32> {
    if window == 0 {
        return None;
    }
    win.push_back(token.to_string());
    if win.len() > window {
        let _ = win.pop_front();
    }
    if win.len() < window / 2 {
        return None;
    }
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for t in win.iter() {
        *freq.entry(t.as_str()).or_insert(0) += 1;
    }
    let max_rep = freq.values().copied().max().unwrap_or(0) as f32;
    let ratio = max_rep / (win.len() as f32);
    let mut ent: f32 = 0.0;
    if entropy_min > 0.0 {
        let concat = win.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(" ");
        let mut cf: HashMap<char, usize> = HashMap::new();
        for ch in concat.chars() {
            *cf.entry(ch).or_insert(0) += 1;
        }
        let total = concat.chars().count() as f32;
        if total > 0.0 {
            for v in cf.values() {
                let p = (*v as f32) / total;
                ent += -(p * p.log2());
            }
        }
    }
    if ratio >= threshold || (entropy_min > 0.0 && ent < entropy_min) {
        metrics::counter!("loop_detected_total").increment(1);
        Some(ratio)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_simple_loop() {
        let mut win = VecDeque::new();
        let tokens = ["a", "b", "a", "b", "a", "b", "a", "b"]; // 50% repetition
        let mut detected = false;
        for t in tokens {
            if check_sequence(&mut win, t, 6, 0.6, 0.0).is_some() {
                detected = true;
                break;
            }
        }
        assert!(detected);
    }

    #[test]
    fn ignores_unique_sequence() {
        let mut win = VecDeque::new();
        let tokens = ["a", "b", "c", "d", "e", "f"]; // no repetition
        let mut detected = false;
        for t in tokens {
            if check_sequence(&mut win, t, 6, 0.6, 0.0).is_some() {
                detected = true;
                break;
            }
        }
        assert!(!detected);
    }
}
