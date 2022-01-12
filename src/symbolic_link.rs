use winapi::km::wdm::{IoCreateSymbolicLink, IoDeleteSymbolicLink};
use winapi::shared::ntdef::UNICODE_STRING;

use crate::{create_unicode_string, debug_println, Error};

pub struct SymbolicLink<'a> {
    name: &'a str,
}

impl<'a> SymbolicLink<'a> {
    pub fn new(name: &'a str, target: &str) -> Result<Self, Error> {
        unsafe {
            debug_println!("Creating Symbolic Link");

            let name_unicode: UNICODE_STRING = create_unicode_string(name);
            let target_unicode: UNICODE_STRING = create_unicode_string(target);

            IoCreateSymbolicLink(&name_unicode, &target_unicode);

            Ok(Self { name })
        }
    }

    pub fn close(&self) {
        unsafe {
            debug_println!("Dropping Symbolic Link");
            IoDeleteSymbolicLink(&create_unicode_string(self.name));
        }
    }
}

impl<'a> Drop for SymbolicLink<'a> {
    fn drop(&mut self) {
        self.close()
    }
}