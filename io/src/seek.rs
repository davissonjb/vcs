// --------------------------------------------------------------------------------------------------------------------------
//!
//! File: vcs/io/src/seek.rs
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

use crate::file::*;
use std::{
    collections::HashSet,
    env, fs,
    iter::Iterator,
    ops::Deref,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub struct Crawler {
    root: Box<PathBuf>,
    dirs: HashSet<Box<PathBuf>>,
    files: HashSet<Box<PathBuf>>,
    next: Box<PathBuf>,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            root: Box::<PathBuf>::new(PathBuf::new()),
            dirs: HashSet::<Box<PathBuf>>::new(),
            files: HashSet::<Box<PathBuf>>::new(),
            next: Box::<PathBuf>::new(PathBuf::new()),
        }
    }

    pub fn setRoot(&mut self, r: String) -> () {
        // Dereferencing PathBuf contained in the
        // heap-allocated Box type, to allow an
        // assignment to the same.
        *self.root = PathBuf::from(&r);
        *self.next = PathBuf::from(&r);
    }

    pub fn setCurr(&mut self) -> () {
        *self.root = env::current_dir().unwrap();
        *self.next = env::current_dir().unwrap();
    }

    fn setnext(&mut self, p: PathBuf) -> () {
        *self.next = p;
    }

    pub fn init(&mut self) -> () {
        println!("Recursively seeking in root: {}", self.root.display());
        let a: HashSet<PathBuf> = fs::read_dir(self.root.deref())
            .unwrap()
            .into_iter()
            .filter(|a| a.is_ok())
            .map(|e| e.unwrap().path())
            .filter(|b| b.is_dir())
            .collect::<HashSet<_>>();
        // .into_iter()
        // .collect();
        for b in a.iter().filter(|r| !r.ends_with(".git")) {
            self.dirs.insert(Box::new(b.clone()));
        }
        println!("-------------------------------------------");
        println!("-------------------------------------------");
    }

    fn recurse(&mut self, p: PathBuf) -> () {}

    // fn sort(&mut self) -> () {
    //     self.dirs.sort();
    //     self.files.sort();
    // }

    fn sieve(&mut self) -> usize {
        let initDirCnt: usize = self.dirs.len();
        let initFileCnt: usize = self.files.len();
        // dirCnt + fileCnt
        usize::default()
    }

    pub fn print(&mut self) -> () {
        // self.sort();
        for d in self.dirs.iter() {
            println!("{}", d.display());
            // for f in self.files.iter() {
            //     println!("----{}", f.display());
            // }
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------------------------------------

impl Iterator for Crawler {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        Some(*self.dirs.iter().next().unwrap().clone())
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
