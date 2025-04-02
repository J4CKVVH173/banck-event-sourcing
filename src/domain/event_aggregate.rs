use super::event::Event;

pub struct EventAggregate {
    events: Vec<Event>,
}

impl EventAggregate {
    pub fn new() -> Self {
        EventAggregate { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn add_events(&mut self, events: &[Event]) {
        self.events.extend_from_slice(events);
    }

    pub fn get_amount(&self) -> isize {
        self.events
            .iter()
            .map(|event| match event {
                Event::Add(amount) => *amount as isize,
                Event::Remove(amount) => -(*amount as isize),
            })
            .sum()
    }
}