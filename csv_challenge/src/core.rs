pub mod read;
pub mod write;
use crate::err::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
