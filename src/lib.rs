use alloc::vec::Vec;
#![no_std]

#[warn(non_camel_case_types)]
pub struct zstr {
    chars: &'static str
}

impl zstr {
    pub fn from_str(s: &'static str) -> zstr {
        zstr { chars: s }
    }

    pub fn as_str(&self) -> &'static str {
        self.chars
    }

    pub fn as_bytes(&self) -> &'static [u8] {
        self.chars.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub fn new() -> zstr {
        zstr { chars: "" }
    }

    pub fn from_u8(s: &'static [u8]) -> zstr {
        zstr { chars: core::str::from_utf8(s).unwrap() }
    }

    pub fn push(&mut self, c: char) {
        let mut new_chars = [0u8; 4];
        let mut bytes = unsafe {
            self.chars.as_bytes_mut()
        };
        let c_bytes = (c as u32).to_be_bytes();
        for i in 0..4 {
            new_chars[i] = bytes[i];
        }
        for i in 0..4 {
            bytes[i] = c_bytes[i];
        }
        self.chars = core::str::from_utf8(&new_chars).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;


}
