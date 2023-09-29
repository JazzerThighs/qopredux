#![allow(dead_code)]

pub struct Note {
    pub name: String,
    pub scale_num: u8,
    pub note_num: u16,
    pub frequency: f32,
    pub color: String,
}

pub struct Scale {
    pub name: String,
    pub description: String,
    pub scale_num: u8,
    pub scale_type: String,
    pub reference_note: u16,
    pub tuning_hz: f32,
    pub octave_divisions: u16,
    pub note_class_set: Vec<String>,
    pub notes: Vec<Note>,
}

pub struct MIDIParams {
    pub output: Vec<bool>,
}

pub struct Oscillator {
    pub output: Vec<bool>,
    pub wave_type: Vec<String>,
    pub gain: Vec<f32>,
}

pub struct ActionInput {
    pub kb_up: Vec<String>,
    pub kb_down: Vec<String>,
    pub m_n_on: Vec<u8>,
    pub m_n_off: Vec<u8>,
}
pub struct TranspositionKB {
    pub code: String,
    pub note_delta: i16,
    pub cent_delta: f32,
}
pub struct TranspostitionMIDI {
    pub msg: u8,
    pub note_bool: bool,
    pub cent_bool: bool,
    pub note_delta: i16,
    pub cents_delta: f32,
}
pub struct TranspositionInput {
    pub kb_up: Vec<TranspositionKB>,
    pub kb_down: Vec<TranspositionKB>,
    pub m_n_on: Vec<TranspostitionMIDI>,
    pub m_n_off: Vec<TranspostitionMIDI>,
}

pub struct Fret {
    pub name: String,
    pub description: String,
    pub gut_num: u8,
    pub fret_num: u8,
    pub act_in: ActionInput,
    pub sus_in: ActionInput,
    pub asu_in: ActionInput,
    pub sos_in: ActionInput,
    pub aso_in: ActionInput,
    pub transp_in: TranspositionInput,
}

pub struct Gut {
    pub name: String,
    pub description: String,
    pub gut_num: u8,
    pub midi: MIDIParams,
    pub oscillator: Oscillator,
    pub require_fret: bool,
    pub require_aero: bool,
    pub open_note: Vec<u16>,
    pub act_in: ActionInput,
    pub sus_in: ActionInput,
    pub asu_in: ActionInput,
    pub sos_in: ActionInput,
    pub aso_in: ActionInput,
    pub transp_in: TranspositionInput,
    pub frets: Vec<Fret>,
}

pub struct Pad {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub pad_num: u8,
    pub act_in: ActionInput,
    pub sus_in: ActionInput,
    pub asu_in: ActionInput,
    pub sos_in: ActionInput,
    pub aso_in: ActionInput,
}
pub struct Delta {
    pub note_bool: bool,
    pub cent_bool: bool,
    pub note_delta: i16,
    pub cents_delta: f32,
}
pub struct Combo {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub combo_num: u16,
    pub combo: Vec<bool>,
    pub transp_in: TranspositionInput,
    pub delta_set: Vec<Delta>,
}
pub struct Aero {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub transp_in: TranspositionInput,
    pub pads: Vec<Pad>,
    pub combos: Vec<Combo>,
}

pub struct Qrud {
    pub name: String,
    pub version: String,
    pub unix_timestamp: String,
    pub description: String,
    pub debounce_timer: u16,
    pub scales: Vec<Scale>,
    pub guts: Vec<Gut>,
    pub aeros: Vec<Aero>,
}
