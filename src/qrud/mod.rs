#![allow(dead_code)]
use std::time::{SystemTime, UNIX_EPOCH};
mod qr_input;
mod scale;

pub struct MIDIParams {
    pub output: Vec<bool>,
}
impl MIDIParams {
    pub fn new(output: Vec<bool>) -> Self {
        return MIDIParams { output };
    }
}

pub struct Oscillator {
    pub output: bool,
    pub wave_type: String,
    pub gain: f32,
}
impl Oscillator {
    pub fn new(output: bool, wave_type: String, gain: f32) -> Self {
        return Oscillator {
            output,
            wave_type,
            gain,
        };
    }
}

pub struct Gut {
    pub name: String,
    pub description: String,
    pub gut_num: u8,
    pub midi: MIDIParams,
    pub oscillators: Vec<Oscillator>,
    pub require_fret: bool,
    pub require_aero: bool,
    pub open_notes: Vec<u16>,
    pub act_in: qr_input::ActionInput,
    pub sus_in: qr_input::ActionInput,
    pub asu_in: qr_input::ActionInput,
    pub sos_in: qr_input::ActionInput,
    pub aso_in: qr_input::ActionInput,
    pub transp_in: qr_input::TranspositionInput,
    pub frets: Vec<Fret>,
}
impl Gut {
    pub fn new(
        name: String,
        description: String,
        gut_num: u8,
        midi: MIDIParams,
        oscillators: Vec<Oscillator>,
        require_fret: bool,
        require_aero: bool,
        open_notes: Vec<u16>,
        act_in: qr_input::ActionInput,
        sus_in: qr_input::ActionInput,
        asu_in: qr_input::ActionInput,
        sos_in: qr_input::ActionInput,
        aso_in: qr_input::ActionInput,
        transp_in: qr_input::TranspositionInput,
        frets: Vec<Fret>,
    ) -> Self {
        return Gut {
            name,
            description,
            gut_num,
            midi,
            oscillators,
            require_fret,
            require_aero,
            open_notes,
            act_in,
            sus_in,
            asu_in,
            sos_in,
            aso_in,
            transp_in,
            frets,
        };
    }
}

pub struct Fret {
    pub name: String,
    pub description: String,
    pub gut_num: u8,
    pub fret_num: u8,
    pub act_in: qr_input::ActionInput,
    pub sus_in: qr_input::ActionInput,
    pub asu_in: qr_input::ActionInput,
    pub sos_in: qr_input::ActionInput,
    pub aso_in: qr_input::ActionInput,
    pub transp_in: qr_input::TranspositionInput,
}
impl Fret {
    pub fn new(
        name: String,
        description: String,
        gut_num: u8,
        fret_num: u8,
        act_in: qr_input::ActionInput,
        sus_in: qr_input::ActionInput,
        asu_in: qr_input::ActionInput,
        sos_in: qr_input::ActionInput,
        aso_in: qr_input::ActionInput,
        transp_in: qr_input::TranspositionInput,
    ) -> Self {
        return Fret {
            name,
            description,
            gut_num,
            fret_num,
            act_in,
            sus_in,
            asu_in,
            sos_in,
            aso_in,
            transp_in,
        };
    }
}

pub struct Pad {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub pad_num: u8,
    pub act_in: qr_input::ActionInput,
    pub sus_in: qr_input::ActionInput,
    pub asu_in: qr_input::ActionInput,
    pub sos_in: qr_input::ActionInput,
    pub aso_in: qr_input::ActionInput,
}
impl Pad {
    pub fn new(
        name: String,
        description: String,
        aero_num: u8,
        pad_num: u8,
        act_in: qr_input::ActionInput,
        sus_in: qr_input::ActionInput,
        asu_in: qr_input::ActionInput,
        sos_in: qr_input::ActionInput,
        aso_in: qr_input::ActionInput,
    ) -> Self {
        return Pad {
            name,
            description,
            aero_num,
            pad_num,
            act_in,
            sus_in,
            asu_in,
            sos_in,
            aso_in,
        };
    }
}

pub struct Delta {
    pub note_bool: bool,
    pub cent_bool: bool,
    pub note_delta: i16,
    pub cents_delta: f32,
}
impl Delta {
    pub fn new() -> Self {
        return Delta {
            note_bool: true,
            cent_bool: true,
            note_delta: 0,
            cents_delta: 0.0,
        };
    }
}

pub struct Combo {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub combo_num: u16,
    pub combo: Vec<bool>,
    pub transp_in: qr_input::TranspositionInput,
    pub delta_set: Vec<Delta>,
}
impl Combo {
    pub fn new(
        name: String,
        description: String,
        aero_num: u8,
        combo_num: u16,
        combo: Vec<bool>,
        transp_in: qr_input::TranspositionInput,
        delta_set: Vec<Delta>,
    ) -> Self {
        return Combo {
            name,
            description,
            aero_num,
            combo_num,
            combo,
            transp_in,
            delta_set,
        };
    }
}

pub struct Aero {
    pub name: String,
    pub description: String,
    pub aero_num: u8,
    pub transp_in: qr_input::TranspositionInput,
    pub pads: Vec<Pad>,
    pub combos: Vec<Combo>,
}
impl Aero {
    pub fn new(
        name: String,
        description: String,
        aero_num: u8,
        transp_in: qr_input::TranspositionInput,
        pads: Vec<Pad>,
        combos: Vec<Combo>,
    ) -> Self {
        return Aero {
            name,
            description,
            aero_num,
            transp_in,
            pads,
            combos,
        };
    }
}

pub struct Qrud {
    pub name: String,
    pub version: String,
    pub unix_timestamp: u64,
    pub description: String,
    pub debounce_timer: u16,
    pub scales: Vec<scale::Scale>,
    pub guts: Vec<Gut>,
    pub aeros: Vec<Aero>,
}
impl Qrud {
    pub fn new(&mut self) -> Self {
        let current_time = SystemTime::now();
        let new_unix_timestamp = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards") // This should not happen unless system clock is misconfigured
            .as_secs(); // This gives you the timestamp as u64

        return Qrud {
            name: String::from("Qwert-O-Phone Redux Default"),
            version: String::from("0.1.0"),
            unix_timestamp: new_unix_timestamp,
            description: String::from("Default QRUD"),
            debounce_timer: 0,
            scales: vec![scale::Scale::new(scale::ScaleType::EqualTemperament {
                new_reference_note: 69,
                new_tuning_hz: 440.0,
                new_octave_divisions: 12,
                new_note_class_set: vec![
                    String::from("C"),
                    String::from("C#"),
                    String::from("D"),
                    String::from("D#"),
                    String::from("E"),
                    String::from("F"),
                    String::from("F#"),
                    String::from("G"),
                    String::from("G#"),
                    String::from("A"),
                    String::from("A#"),
                    String::from("B"),
                ],
                octave: -2,
                note_amount: 128,
            })],
            guts: vec![],
            aeros: vec![],
        };
    }

    pub fn refresh_scale_nums(&mut self) {
        for (index, scale) in self.scales.iter_mut().enumerate() {
            scale.scale_num = index as u8;
            for note in &mut scale.notes {
                note.scale_num = index as u8;
            }
        }
    }
    pub fn add_scale(&mut self, scale: scale::Scale) {
        self.scales.push(scale);
        self.refresh_scale_nums();
        for gut in &mut self.guts {
            gut.open_notes.push(0);
        }
    }
    pub fn remove_scale_at(&mut self, index: usize) {
        if index < self.scales.len() && self.scales.len() > 1 {
            self.scales.remove(index);
            self.refresh_scale_nums();

            for gut in &mut self.guts {
                if let Some(pos) = gut.open_notes.iter().position(|&x| x == index as u16) {
                    gut.open_notes.remove(pos);
                }
            }
        }
    }

    pub fn add_scale_note_at(&mut self, scale_idx: usize, note: scale::Note, note_idx: usize) {
        if scale_idx < self.scales.len() {
            let scale = &mut self.scales[scale_idx];
            if note_idx <= scale.notes.len() {
                scale.notes.insert(note_idx, note);
            }
            scale.refresh_note_nums();

            for gut in &mut self.guts {
                if let Some(open_note) = gut.open_notes.get_mut(scale_idx) {
                    if *open_note >= scale.notes.len() as u16 {
                        *open_note = (scale.notes.len() - 1) as u16;
                    }
                }
            }
        }
    }
    pub fn remove_scale_note_at(&mut self, scale_idx: usize, note_idx: usize) {
        if scale_idx < self.scales.len() && self.scales.len() > 1 {
            let scale = &mut self.scales[scale_idx];
            if note_idx < scale.notes.len() {
                scale.notes.remove(note_idx);
            }
            scale.refresh_note_nums();

            for gut in &mut self.guts {
                if let Some(open_note) = gut.open_notes.get_mut(scale_idx) {
                    if *open_note >= scale.notes.len() as u16 {
                        *open_note = (scale.notes.len() - 1) as u16;
                    }
                }
            }
        }
    }
}
