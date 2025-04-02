#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};

use crate::domain::event::Event;

pub struct RawEvent {
    event_name: String,
    data: String,
}

impl RawEvent {
    fn get_event_type(event: &Event) -> String {
        match event {
            Event::Add(_) => "add".to_string(),
            Event::Remove(_) => "remove".to_string(),
        }
    }

    fn get_event_data(event: &Event) -> String {
        match event {
            Event::Add(amount) => amount.to_string(),
            Event::Remove(amount) => amount.to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split_whitespace();
        let event_type = parts.next().ok_or("Missing event type".to_string())?;
        let data_str = parts.next().ok_or("Missing event data".to_string())?;
        // Если есть лишние части, считаем это ошибкой
        if parts.next().is_some() {
            return Err("Too many arguments provided".to_string());
        }

        // Пытаемся распарсить число
        let amount: usize = data_str
            .parse()
            .map_err(|_| "Invalid number format".to_string())?;

        // Выбираем нужное событие на основе типа
        let event = match event_type.to_lowercase().as_str() {
            "add" => Event::Add(amount),
            "remove" => Event::Remove(amount),
            _ => return Err("Unknown event type".to_string()),
        };

        // Преобразуем событие в RawEvent с помощью уже реализованного From<Event>
        Ok(event.into())
    }
}

impl From<Event> for RawEvent {
    fn from(value: Event) -> Self {
        RawEvent {
            event_name: RawEvent::get_event_type(&value),
            data: RawEvent::get_event_data(&value),
        }
    }
}

impl TryFrom<RawEvent> for Event {
    type Error = String;

    fn try_from(raw: RawEvent) -> Result<Self, Self::Error> {
        // Пытаемся распарсить data как число
        let number: usize = raw
            .data
            .parse()
            .map_err(|_| "Invalid data format for number".to_string())?;

        match raw.event_name.as_str() {
            "add" => Ok(Event::Add(number)),
            "remove" => Ok(Event::Remove(number)),
            _ => Err(format!("Unknown event type: {}", raw.event_name)),
        }
    }
}

impl Display for RawEvent {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.event_name, self.data)
    }
}
