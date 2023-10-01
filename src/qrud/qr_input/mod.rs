pub struct ActionInput {
    pub kb_down: Vec<String>,
    pub kb_up: Vec<String>,
    pub m_n_on: Vec<u8>,
    pub m_n_off: Vec<u8>,
}
pub struct TranspositionKB {
    pub code: String,
    pub note_bool: bool,
    pub cent_bool: bool,
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
