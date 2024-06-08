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
    env, fs,
    ops::Deref,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub struct Crawler {
    root: Box<PathBuf>,
    dirs: Vec<Box<PathBuf>>,
    files: Vec<Box<PathBuf>>,
    next: Box<PathBuf>,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            root: Box::<PathBuf>::new(PathBuf::new()),
            dirs: Vec::<Box<PathBuf>>::new(),
            files: Vec::<Box<PathBuf>>::new(),
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

    pub fn seek(&mut self) -> () {
        println!("Recursively seeking in root: {}", self.root.display());
        for e in fs::read_dir(self.root.deref()).unwrap() {
            let ent = e.unwrap();
            println!("Entry:: {}\n", ent.path().display());
            let p = ent.path();
            println!("----entry.path :: {}", p.display());
            let md = fs::metadata(&p).unwrap();
            if md.is_dir() && ent.path().as_os_str() != "target" {
                self.dirs.push(Box::new(p.clone()));
                self.setnext(p.clone());
                self.seekNext(*self.next.clone());
            }
            if md.is_file() {
                self.files
                    .push(Box::new(p.clone().file_name().unwrap().into()))
            }
        }
    } // END pub fn seek(&mut self) -> ()

    fn seekNext(&mut self, p: PathBuf) -> () {
        let root = self.root.clone();
        // println!("[seekNext]:: Root is: {}", root.display());
        for e in fs::read_dir(self.next.deref()).unwrap() {
            let ent = e.unwrap();
            // println!("SeekNext - Entry:: {}\n", ent.path().display());
            let p = ent.path();
            if (ent.file_name() == p.file_name().unwrap()) {
                println!("They're the same!");
            }
            // println!("----entry.path :: {}", p.display());
            let md = fs::metadata(&p).unwrap();
            if md.is_dir() && (ent.path().as_os_str() != "target") {
                self.dirs.push(Box::new(p.clone()));
                self.setnext(p.clone());
                self.seekNext(*self.next.clone());
            }
            if md.is_file() {
                self.files
                    .push(Box::new(p.clone().file_name().unwrap().into()))
            }
        }
    }

    pub fn print(&self) -> () {
        for d in self.dirs.iter() {
            println!("{}", d.display());
            for f in self.files.iter() {
                println!("----{}", f.display());
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
