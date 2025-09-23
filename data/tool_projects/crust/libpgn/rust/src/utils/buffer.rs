pub const PGN_BUFFER_INITIAL_SIZE: usize = 16;
pub const PGN_BUFFER_GROW_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PgnBuffer {
    buf: String,
}

impl PgnBuffer {
    /// Initializes an empty buffer with a predefined capacity (`pgn_buffer_init`)
    pub fn new() -> Self {
        PgnBuffer {
            buf: String::with_capacity(PGN_BUFFER_INITIAL_SIZE),
        }
    }

    /// Expands buffer capacity (`pgn_buffer_grow`)
    pub fn grow(&mut self) {
        unimplemented!()
    }

    /// Clears the buffer content while keeping allocated space (`pgn_buffer_reset`)
    pub fn reset(&mut self) {
        unimplemented!()
    }

    /// Appends a single character to the buffer (`pgn_buffer_append`)
    pub fn append(&mut self, ch: char) {
        unimplemented!()
    }

    /// Appends a null terminator (C-style) (`pgn_buffer_append_null_terminator`)
    pub fn append_null_terminator(&mut self) {
        unimplemented!()
    }

    /// Concatenates a string to the buffer (`pgn_buffer_concat`)
    pub fn concat(&mut self, s: &str) {
        unimplemented!()
    }

    /// Detaches the buffer's content and returns it, while clearing the structure (`pgn_buffer_detach`)
    pub fn detach(self) -> String {
        unimplemented!()
    }
}
