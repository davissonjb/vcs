// --------------------------------------------------------------------------------------------------------------------------
//!
//! File: vcs/io/src/file.rs
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

pub struct FileData {
    files: BTreeMap<PathBuf, Vec<PathBuf>>,
}

impl FileData {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::<PathBuf, Vec<PathBuf>>::new(),
        }
    }

    pub fn insert(&mut self, d: PathBuf, f: PathBuf) -> () {
        self.files.entry(d).or_insert_with(Vec::new).push(f);
    }

    pub fn count(&self) -> usize {
        self.files.len()
    }

    // pub fn sort(&mut self) -> () {
    //     self.files
    //         .into_iter()
    //         .collect::<BTreeMap<PathBuf, Vec<PathBuf>>>()
    //         .into_iter()
    //         .collect();
    // }

    pub fn print(&self) -> () {
        for (k, v) in self.files.clone().into_iter() {
            for i in v.iter() {
                println!("{} :: {}", k.display(), i.display());
            }
        }
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
