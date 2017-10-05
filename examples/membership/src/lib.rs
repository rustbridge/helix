#[macro_use]
extern crate helix;

use helix::{Error, FromRuby};

ruby! {
    reopen class Array {
        def is_superset_of(&self, needle: Vec<u64>) -> Result<bool, Error> {
            if needle.is_empty() { return Ok(true) }

            let haystack = try!(self.as_vec());

            if haystack.is_empty() { return Ok(false) }

            let mut needle = needle.into_iter();
            let mut needle_item = needle.next().unwrap();

            for item in haystack {
                if item == needle_item {
                    match needle.next() {
                        None => return Ok(true),
                        Some(next_item) => needle_item = next_item
                    }
                }
            }

            Ok(false)
        }
    }
}

impl Array {
    fn as_vec(&self) -> Result<Vec<u64>, Error> {
        Vec::<u64>::from_ruby(self.helix).map(Vec::<u64>::from_checked)
    }
}
