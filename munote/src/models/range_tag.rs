use crate::event::Event;
use crate::tag_id::TagId;
use crate::tag_param::TagParam;

#[derive(Debug, Clone, PartialEq)]
pub struct RangeTag {
    pub id: TagId,
    pub params: Vec<TagParam>,
    pub events: Vec<Box<dyn Event>>,
    pub start: usize,
    pub end: usize,
}

impl RangeTag {
    pub fn from_id(id: TagId, start: usize, end: usize) -> Self {
        Self {
            id,
            params: Vec::new(),
            events: Vec::new(),
            start,
            end,
        }
    }

    pub fn with_param(mut self, param: TagParam) -> Self {
        self.params.push(param);
        self
    }

    pub fn with_event(mut self, event: Box<dyn Event>) -> Self {
        self.events.push(event);
        self
    }
}
