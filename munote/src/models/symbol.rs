use std::any::Any;
use std::fmt::Debug;
use crate::duration::Duration;

pub trait Symbol: Debug {
    fn as_any(&self) -> &dyn Any;

    fn equals(&self, _: &dyn Symbol) -> bool;

    fn octave(&self) -> i8;

    fn duration(&self) -> Duration;
}