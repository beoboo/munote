use crate::duration::Duration;
use std::{any::Any, fmt::Debug};

pub trait Symbol: Debug {
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, _: &dyn Symbol) -> bool;

    fn octave(&self) -> i8;

    fn duration(&self) -> Duration;
}
