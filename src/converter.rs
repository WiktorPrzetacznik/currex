use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::converter::cache::Cache;

mod cache;
mod external_api;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Default)]
pub struct CurrencyConverter {
    cache: Cache,
}

impl CurrencyConverter {
    pub fn save(&self) -> Result<()> {
        let mut cache_file = File::create("cache")?;
        let data = serde_json::to_string(self)?;
        io::Write::write(&mut cache_file, data.as_ref())?;
        Ok(())
    }
    pub fn load() -> Result<Self> {
        let mut cache_file = File::open("cache")?;
        let mut buffer = String::new();
        cache_file.read_to_string(&mut buffer)?;
        Ok(serde_json::from_str(buffer.as_str())?)
    }
    pub fn get_pair_data(&mut self, base: &String, target: &String) -> Result<Decimal> {
        let cached = self.cache.get_pair(base, target);
        if let Some(val) = cached {
            return Ok(val);
        }
        let received_data = external_api::get_pair(base, target)?;
        self.cache.set_pair(base, target, received_data);
        Ok(received_data)
    }
    pub fn get_list_data(&mut self, base: &String) -> Result<HashMap<String, Decimal>> {
        let cached = self.cache.get_list(base);
        if let Some(val) = cached {
            return Ok(val);
        }
        let received_data = external_api::get_list(base)?;
        self.cache.set_list(base, received_data.clone());
        Ok(received_data)
    }
}