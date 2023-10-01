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
    pub fn new(scale_type: ScaleType) -> Option<Self> {
        match scale_type {
            ScaleType::EqualTemperament {
                new_reference_note,
                new_tuning_hz,
                new_octave_divisions,
                new_note_class_set,
                mut octave,
                note_amount,
            } => {
                let mut new_notes = Vec::new();
                let mut note_class_idx = 0;

                for i in 0..note_amount {
                    let distance_from_ref = i as i32 - new_reference_note as i32;
                    let frequency = new_tuning_hz
                        * 2.0f32.powf((distance_from_ref as f32) / (new_octave_divisions as f32));
                    let name = format!("{}{}", new_note_class_set[note_class_idx], octave);

                    let note = Note {
                        name,
                        scale_num: 0,
                        note_num: new_reference_note + i as u16,
                        frequency,
                        color: String::from("#FFFFFF"),
                    };
                    new_notes.push(note);

                    note_class_idx += 1;
                    if note_class_idx == new_note_class_set.len() {
                        note_class_idx = 0;
                        octave += 1;
                    }
                }

                Some(Scale {
                    name: format!("{}-Tone Equal Temperament", new_octave_divisions),
                    description: if new_tuning_hz == 440.0 && new_octave_divisions == 12 {
                        String::from("MIDI")
                    } else {
                        format!(
                            "{}hz {}-Tone Equal Temperament Scale",
                            new_tuning_hz, new_octave_divisions
                        )
                    },
                    scale_num: 0,
                    scale_type: String::from("EqualTemperament"),
                    reference_note: new_reference_note,
                    tuning_hz: new_tuning_hz,
                    octave_divisions: new_octave_divisions,
                    note_class_set: new_note_class_set,
                    notes: new_notes,
                })
            },
            _ => None, // Return None for the placeholder
        }
    }

    pub fn refresh_note_nums(&mut self) {
        for (i, note) in self.notes.iter_mut().enumerate() {
            note.note_num = self.reference_note + i as u16;
        }
    }
}
