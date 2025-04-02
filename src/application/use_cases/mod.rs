use std::io::Error;

use crate::domain::{event::Event, event_aggregate::EventAggregate};

use super::storage::Storage;

pub struct UseCases<T: Storage> {
    storage: T,
}

impl<T: Storage> UseCases<T> {
    pub fn new(storage: T) -> Self {
        UseCases { storage }
    }

    pub fn add(&self, value: usize) -> Result<(), Error> {
        let event = Event::Add(value);
        self.storage.save(event)?;
        Ok(())
    }

    pub fn remove(&self, value: usize) -> Result<(), Error> {
        let event = Event::Remove(value);
        self.storage.save(event)?;
        Ok(())
    }

    pub fn get_amount(&self) -> Result<isize, Error> {
        let events = self.storage.load_all()?;
        let mut event_aggregate = EventAggregate::new();
        event_aggregate.add_events(&events);
        Ok(event_aggregate.get_amount())
    }
}
