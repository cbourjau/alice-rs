use alice::event::Event;
use alice::track::Track;


pub trait ProcessEvent {
    fn process_event(&mut self, sel_event: &Event, sel_tracks: &[&Track]);
}
