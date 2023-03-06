use std::str::FromStr;

use anyhow::{anyhow, Result};
use colorize::AnsiColor;
use lazy_static::lazy_static;
use parse_display::FromStr;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, FromStr, Deserialize)]
#[display(style = "camelCase")]
#[serde(rename_all="camelCase")]
pub enum TagId {
    // Accidentals
    Accidental,
    Alter,

    // Articulations
    Accent,
    Bow,
    BreathMark,
    Fermata,
    Glissando,
    Marcato,
    PedalOn,
    PedalOff,
    Pizzicato,
    Slur,
    Staccato,
    Tenuto,

    // Barlines
    Bar,
    BarFormat,
    DoubleBar,
    EndBar,

    // Beaming
    Beam,
    BeamsAuto,
    BeamsOff,
    BeamsFull,
    #[display("fBeam")]
    FBeam,

    // Clef key meter
    Clef,
    Key,
    Meter,

    // Dynamics
    Crescendo,
    Decrescendo,
    Intensity,

    // Layout
    Accolade,
    NewPage,
    NewLine,
    PageFormat,
    Staff,
    StaffFormat,
    StaffOff,
    StaffOn,
    SystemFormat,

    // Miscellaneous
    Auto,
    Space,
    Special,

    // Notes
    Cluster,
    Cue,
    DisplayDuration,
    DotFormat,
    Grace,
    Harmonic,
    Mrest,
    NoteFormat,
    Octava,
    RestFormat,
    HeadsCenter,
    HeadsLeft,
    HeadsRight,
    HeadsNormal,
    HeadsReverse,
    StemsOff,
    StemsAuto,
    StemsDown,
    StemsUp,
    Tie,
    Tuplet,

    // Ornaments
    Arpeggio,
    Mordent,
    Trill,
    Turn,

    // Repeat Signs
    Coda,
    DaCapo,
    DaCapoAlFine,
    DaCoda,
    DalSegno,
    DalSegnoAlFine,
    Fine,
    RepeatBegin,
    RepeatEnd,
    Segno,
    Tremolo,
    Volta,

    // Tempo
    Accelerando,
    Ritardando,
    Tempo,

    // Text
    Composer,
    Fingering,
    Footer,
    Harmony,
    Instrument,
    Lyrics,
    Mark,
    Text,
    Title,
}
