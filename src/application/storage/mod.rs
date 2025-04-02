use std::{io::Error, result::Result};
use crate::domain::event::Event;

pub trait Storage {
    fn save(&self, event: Event) -> Result<(), Error>;
    fn load_all(&self) -> Result<Vec<Event>, Error>;
}