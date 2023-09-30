#![allow(dead_code)]

pub struct Note {
    pub name: String,
    pub scale_num: u8,
    pub note_num: u16,
    pub frequency: f32,
    pub color: String,
}
impl Note {
    pub fn new(name: String, scale_num: u8, note_num: u16, frequency: f32, color: String) -> Self {
        Note {
            name,
            scale_num,
            note_num,
            frequency,
            color,
        }
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_scale_num(&mut self, new_scale_num: u8) {
        self.scale_num = new_scale_num;
    }
    pub fn set_note_num(&mut self, new_note_num: u16) {
        self.note_num = new_note_num;
    }
    pub fn set_frequency(&mut self, new_frequency: f32) {
        self.frequency = new_frequency;
    }
    pub fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }
}

pub struct Scale {
    name: String,
    description: String,
    scale_num: u8,
    scale_type: String,
    reference_note: u16,
    tuning_hz: f32,
    octave_divisions: u16,
    note_class_set: Vec<String>,
    notes: Vec<Note>,
}
pub enum ScaleType {
    EqualTemperament {
        new_reference_note: u16,
        new_tuning_hz: f32,
        new_octave_divisions: u16,
        new_note_class_set: Vec<String>,
        octave: i8,
        note_amount: u16,
    },
    JustIntonation,
    Pythagorean5Limit,
    Werckmeister,
    Kirnberger,
    Maqam,
    Ndebele,
    Gagaku,
    Pelog,
    Slendro,
    Hijaz,
    ShonaMbira,
    BohlenPierce,
}
impl Scale {
    pub fn new(scale_type: ScaleType) -> Self {
        match scale_type {
            ScaleType::EqualTemperament {
                new_reference_note,
                new_tuning_hz,
                new_octave_divisions,
                new_note_class_set,
                mut octave,
                note_amount,
            } => {
                let mut notes = Vec::new();
                let mut note_class_idx = 0;

                for i in 0..note_amount {
                    let distance_from_ref = i as i32 - new_reference_note as i32;
                    let frequency = new_tuning_hz * 2.0f32.powf((distance_from_ref as f32) / (new_octave_divisions as f32));
                    let name = format!("{}{}", new_note_class_set[note_class_idx], octave);

                    let note = Note {
                        name,
                        scale_num: 0,
                        note_num: new_reference_note + i as u16,
                        frequency,
                        color: String::from("#FFFFFF"),
                    };
                    notes.push(note);

                    note_class_idx += 1;
                    if note_class_idx == new_note_class_set.len() {
                        note_class_idx = 0;
                        octave += 1;
                    }
                }

                Scale {
                    name: format!("{}-Tone Equal Temperament", new_octave_divisions),
                    description: if new_tuning_hz == 440.0 && new_octave_divisions == 12 {
                        String::from("The Standard MIDI Scale")
                    } else {
                        format!("{} {}-Tone Equal Temperament Scale", new_tuning_hz, new_octave_divisions)
                    },
                    scale_num: 0,
                    scale_type: String::from("EqualTemperament"),
                    reference_note: new_reference_note,
                    tuning_hz: new_tuning_hz,
                    octave_divisions: new_octave_divisions,
                    note_class_set: new_note_class_set,
                    notes,
                }
            },
            _ => Scale {
                name: String::from("Placeholder"),
                description: String::from("Placeholder description"),
                scale_num: 0,
                scale_type: String::from("PlaceholderType"),
                reference_note: 0,
                tuning_hz: 0.0,
                octave_divisions: 0,
                note_class_set: vec![],
                notes: vec![],
            },
        }
    }

    fn refresh_note_nums(&mut self) {
        for (i, note) in self.notes.iter_mut().enumerate() {
            note.note_num = self.reference_note + i as u16;
        }
    }
}

pub struct MIDIParams {
    pub output: Vec<bool>,
}

pub struct Oscillator {
    pub output: bool,
    pub wave_type: String,
    pub gain: f32,
}

pub struct ActionInput {
    pub kb_down: Vec<String>,
    pub kb_up: Vec<String>,
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
    pub kb_down: Vec<TranspositionKB>,
    pub kb_up: Vec<TranspositionKB>,
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
    pub oscillators: Vec<Oscillator>,
    pub require_fret: bool,
    pub require_aero: bool,
    pub open_notes: Vec<u16>,
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
impl Qrud {
    pub fn refresh_scale_nums(&mut self) {
        for (index, scale) in self.scales.iter_mut().enumerate() {
            scale.scale_num = index as u8;
            for note in &mut scale.notes {
                note.scale_num = index as u8;
            }
        }
    }
    pub fn add_scale(&mut self, scale: Scale) {
        self.scales.push(scale);
        self.refresh_scale_nums();
        for gut in &mut self.guts {
            gut.open_notes.push(0);
        }
    }
    pub fn remove_scale_at(&mut self, index: usize) {
        if index < self.scales.len() {
            self.scales.remove(index);
            self.refresh_scale_nums();
            
            for gut in &mut self.guts {
                if let Some(pos) = gut.open_notes.iter().position(|&x| x == index as u16) {
                    gut.open_notes.remove(pos);
                }
            }
        }
    }

    pub fn add_scale_note_at(&mut self, scale_idx: usize, note: Note, note_idx: usize) {
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
        if scale_idx < self.scales.len() {
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