use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
};

use super::{RuntimeError, Symbol, Value};

/// An environment of symbol bindings. Used for the base environment, for
/// closures, for `let` statements, for function arguments, etc.
#[derive(Debug, Default)]
pub struct Env {
    frames: Vec<Frame>,
}

impl Env {
    pub fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn add_default_frame(&mut self) {
        let frame = Frame::default();
        self.add_frame(frame);
    }

    pub fn pop_frame(&mut self) -> Result<Frame, RuntimeError> {
        self.frames.pop().ok_or_else(|| {
            let message = "No env frames left.".to_owned();
            RuntimeError { message }
        })
    }

    /// Finds the value behind a symbol in the current environment.
    pub fn get(&self, symbol: &Symbol) -> Option<&Value> {
        self.frames.iter().find_map(|frame| frame.get(symbol))
    }

    /// Define a new key in the current environment.
    pub fn set(&mut self, symbol: Symbol, value: Value) {
        if let Some(frame) = self.frames.last_mut() {
            frame.set(symbol, value)
        }
    }

    /// Find the environment where this key is defined, and update its value.
    /// Returns an Err if the symbol has not been defined anywhere in the hierarchy.
    pub fn update(&mut self, symbol: Symbol, value: Value) -> Result<(), RuntimeError> {
        if let Some(frame) = self.next_frame_with_symbol_mut(&symbol) {
            Ok(frame.set(symbol, value))
        } else {
            let message = format!("Tried to set value of undefined symbol \"{}\"", symbol);
            Err(RuntimeError::new(message))
        }
    }

    /// Delete the nearest (going upwards) definition of this key
    pub fn delete(&mut self, symbol: &Symbol) {
        if let Some(frame) = self.next_frame_with_symbol_mut(symbol) {
            frame.delete(symbol);
        }
    }

    fn next_frame_with_symbol_mut(&mut self, symbol: &Symbol) -> Option<&mut Frame> {
        self.frames
            .iter_mut()
            .filter(|frame| frame.has(&symbol))
            .next()
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Env:\n")?;

        for (index, frame) in self.frames.iter().enumerate() {
            write!(f, "Frame {}:\n{}\n", index, frame)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Frame {
    entries: HashMap<Symbol, Value>,
}

impl Frame {
    fn get(&self, symbol: &Symbol) -> Option<&Value> {
        self.entries.get(symbol)
    }

    fn set(&mut self, symbol: Symbol, value: Value) {
        self.entries.insert(symbol, value);
    }

    fn delete(&mut self, symbol: &Symbol) {
        self.entries.remove(symbol);
    }

    fn has(&self, symbol: &Symbol) -> bool {
        self.entries.contains_key(symbol)
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (symbol, value) in self.entries.iter() {
            write!(f, "{}: {}", symbol, value)?;
        }

        Ok(())
    }
}

impl Hash for Frame {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for entry in self.entries {
            entry.hash(state);
        }
    }
}
