use crate::infrastructure::raw_event::RawEvent;

#[test]
fn from_str_should_parse_add_event() {
    let raw_event = RawEvent::from_str("add 5").unwrap();
    assert_eq!(raw_event.event_name, "add");
    assert_eq!(raw_event.data, "5");
}

#[test]
fn from_str_should_handle_leading_and_trailing_whitespace() {
    let raw_event = RawEvent::from_str("  add   10  ").unwrap();
    assert_eq!(raw_event.event_name, "add");
    assert_eq!(raw_event.data, "10");
}

#[test]
fn from_str_should_parse_remove_event() {
    let raw_event = RawEvent::from_str("remove 15").unwrap();
    assert_eq!(raw_event.event_name, "remove");
    assert_eq!(raw_event.data, "15");
}
