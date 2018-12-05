use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::result;

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
    let mut guard_sl = Hash
    let mut guard_id = 0;
    let mut asleep_minute: Option<u32>;
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
                dt: dt,
                event: FallAsleep,
            } => asleep_minute = Some(dt.minute()),
            Event {
                dt: dt,
                event: WakeUp
            } => {

            }

    }
}
