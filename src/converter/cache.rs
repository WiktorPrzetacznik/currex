use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub(super) struct Cache {
    data: HashMap<String, HashMap<String, InternalVal>>,
    last_full_fetch: HashMap<String, SystemTime>,
}

impl Cache {
    const LIVENESS: Duration = Duration::from_secs(3600);
    pub fn get_pair(&mut self, base: &String, target: &String) -> Option<Decimal> {
        let val = self.data.get(base)?.get(target)?;
        if Self::is_stale(&val.time) {
            self.data.get_mut(base).unwrap().remove(target);
            None
        } else {
            Some(val.rate)
        }
    }
    pub fn get_list(&mut self, base: &String) -> Option<HashMap<String, Decimal>> {
        let time = self.last_full_fetch.get(base)?;
        if Self::is_stale(time) {
            self.last_full_fetch.remove(base);
            None
        } else {
            let mut map = HashMap::new();
            self.data.get(base)?
                .iter()
                .for_each(|e| {
                    map.insert(e.0.clone(), e.1.rate);
                });
            Some(map)
        }
    }

    pub fn set_pair(&mut self, base: &String, target: &String, rate: Decimal) {
        let base_data = match self.data.get_mut(base) {
            None => {
                self.data.insert(base.clone(), Default::default());
                self.data.get_mut(base).unwrap()
            }
            Some(data) => data
        };
        base_data.insert(target.clone(), InternalVal {
            time: SystemTime::now(),
            rate,
        });
    }

    pub fn set_list(&mut self, base: &String, list: HashMap<String, Decimal>) {
        self.last_full_fetch.insert(base.clone(), SystemTime::now());
        list
            .iter()
            .for_each(|entry| self.set_pair(base, entry.0, *entry.1))
    }
    fn is_stale(time: &SystemTime) -> bool {
        time.
            checked_add(Cache::LIVENESS)
            .expect("OS ensures proper time management")
            .lt(&SystemTime::now())
    }
}

#[derive(Serialize, Deserialize)]
struct InternalVal {
    time: SystemTime,
    rate: Decimal,
}