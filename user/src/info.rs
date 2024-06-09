// --------------------------------------------------------------------------------------------------------------------------
//!
//! File: vcs/user/src/info.rs
//! Author: Jacob Davisson
//! Last Modified: 08 June 2024
//! Purpose:
//!          
//!          
//! Depends:
//!          
//!
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
#![allow(dead_code)]

use std::fmt::{Debug, Display};

// TODO -- Make decision on whether or not to focus on
// including a timestamp in this struct. The basic
// stdlib does not seem to provide a nice way to do
// this, which would require including the Chrono (or
// similar) crate -- something that I am trying to avoid.
//
// A timestamp would be nice/useful/required for certain
// aspects of a VCS, in fact, I am spontaneously coming
// to the realization that I will need to include Chrono
// as a DateTime/TimeStamp provider...I'll do this once
// I get deeper into actually writing user config data
// into an on-disk file.
pub struct VCSUser {
    name: String,
    user: String,
    email: String,
}

impl VCSUser {
    /// Nothing particularly fancy going on here.
    /// The TimeStamp (self.ts) is initialized to
    /// current system time -- but will be updated
    /// in a later function that reads user creation
    /// time from an on-disk file.
    ///
    /// Given that forthcoming function that updates
    /// this value from file, this default value is
    /// more than suitable for basic initialization
    /// requirement.
    ///
    pub fn new() -> Self {
        Self {
            name: String::default(),
            user: String::default(),
            email: String::default(),
        }
    }

    pub fn setName(&mut self, n: String) -> () {
        self.name = n;
    }

    pub fn setEmail(&mut self, e: String) -> () {
        self.email = e;
    }
}

impl Display for VCSUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {} ---- Email: {}\nUserName: {}",
            self.name, self.email, self.user
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
