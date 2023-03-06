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

    // NotesCluster,
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

    //Repeat Signs
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

    //Tempo
    Accelerando,
    Ritardando,
    Tempo,

    //Text
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

impl TagId {
    pub fn lookup(name: &str) -> Result<TagId> {
        lazy_static! {
            static ref TAG_ID_LOOKUP: std::collections::HashMap<&'static str, TagId> = {
                let mut m = std::collections::HashMap::new();
                m.insert("|", TagId::Bar);
                m.insert("acc", TagId::Accidental);
                m.insert("accol", TagId::Accolade);
                m.insert("cresc", TagId::Crescendo);
                m.insert("dim", TagId::Decrescendo);
                m.insert("instr", TagId::Instrument);
                m.insert("mord", TagId::Mordent);
                m.insert("pizz", TagId::Pizzicato);
                m.insert("set", TagId::Auto);
                m.insert("stacc", TagId::Staccato);
                m.insert("ten", TagId::Tenuto);
                m
            };
        }
        let res = if TAG_ID_LOOKUP.contains_key(name) {
            Ok(TAG_ID_LOOKUP[name])
        } else {
            TagId::from_str(name).map_err(|e| anyhow!("{e}"))
        };

        if res.is_err() {
            eprintln!("{}", format!("Tag ID \"{name}\" not found\n").red());
        }

        res
    }
}
