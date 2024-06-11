// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
//!
//! File:               vcs/io/src/store/file.rs
//! Author:             Jacob Davisson
//! Last Modified:      10 June 2024
//! Purpose:            Provides an encapsulation of structs/enums/methods/functions required to
//!                     maintain a VCS folder somewhere on the local drive. This implementation
//!                     is quite subject to change -- various thoughts that I've had on this local
//!                     storage of VCS information are:
//!                         --> SQLite database
//!                         --> Raw file storage (similar to git, and where I'm planning to start)
//!
//! Depends:
//!          
//!
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use std::collections::*;
use std::fmt::{Debug, Display};
use std::{
    collections::BTreeMap,
    collections::HashMap,
    collections::HashSet,
    env, fs,
    iter::Iterator,
    ops::Deref,
    path::{Path, PathBuf},
};
use user::info::VCSUser;

/// Really need to do some thinking about how to structure, and
/// choose what metadata headers/footers need to format/include.
pub enum MetaData {
    Header(&'static str),
    Footer(&'static str),
}

impl MetaData {
    pub fn makeHeader(&mut self, h: &'static str) -> MetaData {
        MetaData::Header(h)
    }

    pub fn makeFooter(&mut self, f: &'static str) -> MetaData {
        MetaData::Footer(f)
    }

    pub fn getData(&self) -> String {
        match self {
            MetaData::Header(a) => a.to_string(),
            MetaData::Footer(b) => b.to_string(),
        }
    }
}

impl Display for MetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaData::Header(a) => write!(f, "Header :: {}", a),
            MetaData::Footer(b) => write!(f, "Footer :: {}", b),
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------

pub struct LocalData {
    userData: VCSUser,
    hostDir: PathBuf,
}

impl LocalData {
    pub fn new() -> Self {
        Self {
            userData: VCSUser::new(),
            hostDir: match env::var_os("HOME") {
                Some(val) => PathBuf::from(val),
                None => PathBuf::new(),
            },
        }
    } // END pub fn new() -> Self

    // pub fn init(&mut self) -> bool has a usage
    // distinguished from the new() -> Self method.
    // This method is intended to use an initialized
    // LocalData struct, and read from the local VCS
    // directory to fill in the VCSUser struct therein.
    pub fn init(&mut self) -> bool {
        match self.hostDir.is_dir() {
            true => true,
            false => false,
        }
    }

    pub fn readLocal(&mut self) -> bool {
        todo!()
    }
}

impl Display for LocalData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nHome Directory: {}",
            self.userData,
            self.hostDir.display()
        )
    }
}

// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
