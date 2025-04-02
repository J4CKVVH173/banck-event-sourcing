use super::event_aggregate::EventAggregate;
use super::event::Event;

#[test]
fn test_getting_amount() {
    let adding = Event::Add(80);
    let removing = Event::Remove(10);
    let removing_2 = Event::Remove(30);

    let mut event_aggregate = EventAggregate::new();
    let events = vec![adding, removing, removing_2];
    event_aggregate.add_events(&events);
    assert_eq!(event_aggregate.get_amount(), 40);
}

#[test]
fn test_empty_aggregate() {
    let event_aggregate = EventAggregate::new();
    assert_eq!(event_aggregate.get_amount(), 0);
}

#[test]
fn test_negative_amount() {
    let adding = Event::Add(10);
    let removing = Event::Remove(20);

    let mut event_aggregate = EventAggregate::new();
    event_aggregate.add_event(adding);
    event_aggregate.add_event(removing);
    assert_eq!(event_aggregate.get_amount(), -10);
}