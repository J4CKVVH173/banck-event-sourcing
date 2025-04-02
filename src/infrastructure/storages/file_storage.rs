use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, ErrorKind, Result, Write},
};

use crate::{application::Storage, domain::event::Event, infrastructure::raw_event::RawEvent};

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
        FileStorage {
            file_path: file_path.to_string(),
        }
    }
}

impl Storage for FileStorage {
    fn save(&self, event: Event) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true) // Создать файл, если он не существует
            .append(true) // Добавлять в конец файла
            .open(&self.file_path)?;
        let raw_event: RawEvent = event.into();
        writeln!(file, "{}", raw_event.to_string())?;
        Ok(())
    }
    fn load_all(&self) -> Result<Vec<Event>> {
        let file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(ref e) if e.kind() == ErrorKind::NotFound => return Ok(vec![]),
            Err(e) => return Err(e),
        };

        let reader = BufReader::new(file);
        let mut events = Vec::new();

        // Читаем файл построчно
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let raw_event = RawEvent::from_str(trimmed)
                .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
            // Преобразуем RawEvent в Event через TryFrom<RawEvent>
            let event = Event::try_from(raw_event)
                .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
            events.push(event);
        }
        Ok(events)
    }
}
