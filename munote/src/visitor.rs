use crate::chord::Chord;
use crate::note::Note;
use crate::ptr::Ptr;
use crate::rest::Rest;
use crate::tag::Tag;
use crate::voice::Voice;

pub trait Visitor {
    fn on_chord(&mut self, chord: &Chord);
    fn on_note(&mut self, note: &Note);
    fn on_rest(&mut self, rest: &Rest);
    fn on_staff_begin(&mut self);
    fn on_staff_end(&mut self);
    fn on_tag(&mut self, tag: &Tag);
    fn on_voice(&mut self, voice: &Voice);
}

pub type VisitorPtr = Ptr<Box<dyn Visitor>>;
