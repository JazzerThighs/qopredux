pub struct ActionInput {
    pub kb_down: Vec<String>,
    pub kb_up: Vec<String>,
    pub midi_msg: Vec<midi::Message>,
}
pub struct TranspositionKB {
    pub code: String,
    pub note_delta: i16,
    pub cent_delta: f32,
}
pub struct TranspostitionMIDI {
    pub msg: Vec<midi::Message>,
    pub note_delta: i16,
    pub cents_delta: f32,
}
pub struct TranspositionInput {
    pub kb_down: Vec<TranspositionKB>,
    pub kb_up: Vec<TranspositionKB>,
    pub midi_msg: Vec<TranspostitionMIDI>,
}
