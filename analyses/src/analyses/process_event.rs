use alice::event::Event;


pub trait ProcessEvent {
    fn process_event(self, event: &Event) -> Self;
}
