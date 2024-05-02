use std::{collections::HashMap, sync::{Mutex, OnceLock}, time::{Duration, Instant}};

static INIT_INSTANT: OnceLock<Instant> = OnceLock::new();
static DURATIONS: Mutex<Vec::<TimerInfo>> = Mutex::new(Vec::new());
static NAME_REPETITIONS: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();

struct TimerInfo {
    pub start_time: Instant,
    pub start_order: usize,
    pub end_time: Instant,
    pub end_order: usize,
    pub name: String,
    pub same_name_order: usize,
    pub duration: Duration,
}

pub struct Timer {
    // only for convenient drop
    name: Option<String>,
    start: Instant,
    min_duration_to_log: Duration,
}

impl Timer {
    pub fn new(name: impl Into<String>) -> Self {
        Self::new_heavy_only(name, Duration::ZERO)
    }

    pub fn new_heavy_only(name: impl Into<String>, min_duration_to_log: Duration) -> Self {
        _ = INIT_INSTANT.set(Instant::now());

        Self {
            name: Some(name.into()),
            min_duration_to_log,
            start: Instant::now(),
        }
    }

    pub fn log_all_timers() {
        let mut durations = DURATIONS.lock().unwrap();

        durations.sort_by_key(|i| u128::MAX - i.duration.as_micros());

        for timer_info in durations.iter() {
            println!("[Timer log] #{} {} ({} mcs.)", timer_info.same_name_order, timer_info.name, timer_info.duration.as_micros())
        }
    }

    pub fn export_all_timers_to_json() -> String {
        let mut durations = DURATIONS.lock().unwrap();

        durations.sort_by_key(|i| u128::MAX - i.duration.as_micros());

        let mut json = Vec::<String>::new();

        json.push(String::from(" { "));

        json.push(String::from(" \"timers\" : "));
        json.push(String::from(" [ "));


        for timer_info in durations.iter() {
            let init_instant = INIT_INSTANT.get().unwrap();

            json.push(String::from(" { "));
            
            json.push(String::from(" \"startTime\" : "));
            json.push((timer_info.start_time - *init_instant).as_nanos().to_string());
            json.push(String::from(" , "));
            
            json.push(String::from(" \"startOrder\" : "));
            json.push(timer_info.start_order.to_string());
            json.push(String::from(","));
            
            json.push(String::from(" \"endTime\" : "));
            json.push((timer_info.end_time - *init_instant).as_nanos().to_string());
            json.push(String::from(","));
            
            json.push(String::from(" \"endOrder\" : "));
            json.push(timer_info.end_order.to_string());
            json.push(String::from(","));
            
            json.push(String::from(" \"name\" : "));
            json.push(String::from(" \""));
            json.push(format!("#{} {}", timer_info.same_name_order, timer_info.name));
            json.push(String::from("\", "));
            
            json.push(String::from(" \"durationNs\" : "));
            json.push(timer_info.duration.as_nanos().to_string());

            json.push(String::from(" } "));
            json.push(String::from(", "));
        }

        if durations.len() != 0 {
            json.pop();
        }

        json.push(String::from(" ] "));

        json.push(String::from(" } "));

        json.join("")
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let now = Instant::now();

        let duration = now - self.start;

        let name = self.name.take().unwrap();

        let repetitions_count = {
            let mut repetitions = NAME_REPETITIONS.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap();
            if let Some(repetitions_count) = repetitions.get_mut(&name) {
                *repetitions_count += 1;
                *repetitions_count
            } else {
                repetitions.insert(name.clone(), 1);
                1
            }
        };

        if duration >= self.min_duration_to_log {
            DURATIONS.lock().unwrap().push(TimerInfo {
                duration,
                start_time: self.start,
                // todo
                start_order: 0,
                end_time: now,
                // todo
                end_order: 0,
                same_name_order: repetitions_count,
                name: name
            });
        }
    }
}