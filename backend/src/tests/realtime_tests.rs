use blocksmith::realtime::events::{
    RealtimeEvent,
    RealtimeEventType,
};

#[test]
fn test_realtime_event_creation() {
    let event = RealtimeEvent {
        event_type:
            RealtimeEventType::BlockCreated,

        message:
            "New block created"
                .to_string(),

        username:
            "nirvanjain"
                .to_string(),

        repository:
            "BlockSmith"
                .to_string(),

        timestamp:
            "2026-01-01"
                .to_string(),
    };

    assert_eq!(
        event.username,
        "nirvanjain"
    );
}