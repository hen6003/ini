use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub enum IniValue {
    String(String),
    Integer(i32),
    Float(f32),
    Bool(bool),
}

impl Default for IniValue {
    fn default() -> Self {
        Self::Bool(false)
    }
}

#[derive(Debug, Clone)]
pub struct Ini {
    sections: HashMap<String, HashMap<String, IniValue>>,
}

impl Ini {
    pub fn read<R: Read>(reader: BufReader<R>) -> Result<Self> {
        let mut sections: HashMap<String, HashMap<String, IniValue>> = HashMap::new();
        let mut current_section: String = "".to_string();

        for line in reader.lines() {
            let line = line?;

            let mut chars = line.chars();
            match chars.next() {
                // Empty line
                None => (),
                // Comment
                Some('#') => (),

                // Section header
                Some('[') => {
                    if chars.next_back() == Some(']') {
                        let section_name: String = chars.collect();

                        current_section = section_name;
                    } else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Section header not closed",
                        ));
                    }
                }
                // Key-value
                Some(_) => {
                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();

                        let value = if let Ok(v) = str::parse::<i32>(value) {
                            IniValue::Integer(v)
                        } else if let Ok(v) = str::parse::<f32>(value) {
                            IniValue::Float(v)
                        } else if let Ok(v) = str::parse::<bool>(value) {
                            IniValue::Bool(v)
                        } else {
                            IniValue::String(value.to_string())
                        };

                        let section = sections.entry(current_section.to_string()).or_default();
                        let key = section.entry(key.to_string()).or_default();

                        *key = value;
                    } else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Key-value pair incorrectly formatted",
                        ));
                    }
                }
            }
        }

        Ok(Self { sections })
    }

    pub fn read_file<S: AsRef<Path>>(path: S) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        Self::read(reader)
    }

    pub fn sections(&self) -> &HashMap<String, HashMap<String, IniValue>> {
        &self.sections
    }
}
