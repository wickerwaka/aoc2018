use std::collections::HashMap;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

extern crate chrono;
use chrono::{NaiveDateTime, Timelike};

#[derive(Debug)]
enum EventInfo {
    WakeUp,
    FallAsleep,
    OnDuty(i32),
}

#[derive(Debug)]
struct Event {
    dt: NaiveDateTime,
    event: EventInfo,
}

impl EventInfo {
    fn from_str(s: &str) -> Result<EventInfo, ()> {
        let re = Regex::new(r"Guard\s+#(\d+)\s+begins").unwrap();
        if s.starts_with("wakes") {
            Ok(EventInfo::WakeUp)
        } else if s.starts_with("falls") {
            Ok(EventInfo::FallAsleep)
        } else if let Some(captures) = re.captures(s) {
            let id = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            Ok(EventInfo::OnDuty(id))
        } else {
            Err(())
        }
    }
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;

    let re = Regex::new(r"\[(.*)\]\s+(.*)").unwrap();

    let mut series: Vec<Event> = s
        .lines()
        .filter_map(|x| {
            if let Some(captures) = re.captures(x) {
                let timestamp = {
                    let ts = captures.get(1)?.as_str();
                    NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M").unwrap()
                };
                let event_str = captures.get(2)?.as_str();
                let event = EventInfo::from_str(event_str).unwrap();

                Some(Event {
                    dt: timestamp,
                    event: event,
                })
            } else {
                None
            }
        }).collect();

    series.sort_by_key(|x| x.dt);
    //series.iter().for_each(|x| println!("{:?}", x));

    part1(series);

    Ok(())
}

fn part1(series: Vec<Event>) {
    let mut sleep_duration: HashMap<i32, [i32; 60]> = HashMap::new();
    let mut guard_id = 0;
    let mut asleep_minute: Option<u32> = None;

    for event in series.iter() {
        match event {
            Event {
                dt: _,
                event: EventInfo::OnDuty(x),
            } => {
                guard_id = *x;
                asleep_minute = None;
            }
            Event {
                dt,
                event: EventInfo::FallAsleep,
            } => asleep_minute = Some(dt.minute()),
            Event {
                dt,
                event: EventInfo::WakeUp,
            } => {
                let mut durations = sleep_duration.entry(guard_id).or_insert([0; 60]);
                for x in asleep_minute.unwrap()..dt.minute() {
                    durations[x as usize] += 1;
                }
                asleep_minute = None;
            }
        }
    }

    if let Some((id, durations)) = sleep_duration
        .iter()
        .max_by_key(|&x| x.1.iter().sum::<i32>())
    {
        let (idx, _) = durations.iter().enumerate().max_by_key(|x| x.1).unwrap();
        println!("{}", *id * idx as i32);
    }

    let mut max_duration = 0;
    let mut max_idx = 0;
    let mut max_guard = 0;
    for (&id, durations) in &sleep_duration {
        for (idx, &duration) in durations.iter().enumerate() {
            if duration > max_duration {
                max_idx = idx;
                max_guard = id;
                max_duration = duration;
            }
        }
    }
    println!("{}", max_guard * max_idx as i32);
}
