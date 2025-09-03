/* neira:meta
id: NEI-20250317-test-recorder-counters
intent: test
summary: shared recorder now captures counters in addition to histograms.
*/
#![allow(clippy::type_complexity)]
use metrics::{Counter, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit};
use std::sync::{Arc, Mutex, OnceLock};

struct TestRecorder {
    data: Arc<Mutex<Vec<(String, f64)>>>,
}

impl Recorder for TestRecorder {
    fn describe_counter(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_gauge(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}
    fn describe_histogram(&self, _: KeyName, _: Option<Unit>, _: SharedString) {}

    fn register_counter(&self, key: &Key, _metadata: &Metadata<'_>) -> Counter {
        let name = key.name().to_string();
        let data = self.data.clone();
        let ctr = TestCounter { name, data };
        Counter::from_arc(Arc::new(ctr))
    }
    fn register_gauge(&self, _key: &Key, _metadata: &Metadata<'_>) -> Gauge {
        Gauge::noop()
    }
    fn register_histogram(&self, key: &Key, _metadata: &Metadata<'_>) -> Histogram {
        let name = key.name().to_string();
        let data = self.data.clone();
        let hist = TestHistogram { name, data };
        Histogram::from_arc(Arc::new(hist))
    }
}

struct TestHistogram {
    name: String,
    data: Arc<Mutex<Vec<(String, f64)>>>,
}

impl metrics::HistogramFn for TestHistogram {
    fn record(&self, value: f64) {
        self.data.lock().unwrap().push((self.name.clone(), value));
    }
}

struct TestCounter {
    name: String,
    data: Arc<Mutex<Vec<(String, f64)>>>,
}

impl metrics::CounterFn for TestCounter {
    fn increment(&self, value: u64) {
        self.data
            .lock()
            .unwrap()
            .push((self.name.clone(), value as f64));
    }
    fn absolute(&self, value: u64) {
        self.data
            .lock()
            .unwrap()
            .push((self.name.clone(), value as f64));
    }
}

static RECORDER: OnceLock<Arc<Mutex<Vec<(String, f64)>>>> = OnceLock::new();

pub fn init_recorder() -> Arc<Mutex<Vec<(String, f64)>>> {
    let data = RECORDER
        .get_or_init(|| {
            let data = Arc::new(Mutex::new(Vec::new()));
            let recorder = TestRecorder { data: data.clone() };
            let _ = metrics::set_global_recorder(recorder);
            data
        })
        .clone();
    data.lock().unwrap().clear();
    data
}
