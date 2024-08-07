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

use crate::handler::file::*;
use std::{
    collections::HashSet,
    env, fs,
    iter::Iterator,
    ops::Deref,
    path::{Path, PathBuf},
};

// Need to really reconsider some of these entries
// in struct Crawler. It should be reasonably possible
// to consolidate these data structures into a smaller
// required list.
//
// One quick thought :: Might choose to utilize somthing
// along the lines of a Vec<Box<(PathBuf, PathBuf)>> for
// maintaining association of files with their containing
// folder. Surely there is a better (HashMap??) option for
// doing this in std::collections.
//

pub struct Crawler {
    root: Box<PathBuf>,
    dirs: Vec<Box<PathBuf>>,
    files: Vec<Box<PathBuf>>,
    next: Box<PathBuf>,
    tree: FileData,
}

impl Crawler {
    pub fn new() -> Self {
        Self {
            root: Box::<PathBuf>::new(PathBuf::new()),
            dirs: Vec::<Box<PathBuf>>::new(),
            files: Vec::<Box<PathBuf>>::new(),
            next: Box::<PathBuf>::new(PathBuf::new()),
            tree: FileData::new(),
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

    ///----------------------------------------------------------------------
    ///----------------------------------------------------------------------
    ///
    /// Signature: pub fn init(&mut self) -> ()
    /// Purpose:   Generate initial data structure with the
    ///            directories, and included files, as found
    ///            in root directory specified.
    /// Features:  During collection into a Vector data structure,
    ///            each directory found in root path is filtered
    ///            to ensure that it is a directory. These directories
    ///            are then collected as a HashSet, which ensures
    ///            uniqueness of the paths therein before being
    ///            transformed back into, and collected as a Vector.
    ///            The contents of said Vector are further refined
    ///            by removing paths that end with ".git" or "target,"
    ///            to ensure that build products for Rust projects,
    ///            and the VCS folder contents are not captured.
    ///
    ///----------------------------------------------------------------------
    ///----------------------------------------------------------------------
    pub fn init(&mut self) -> () {
        println!("Recursively seeking in root: {}", self.root.display());
        let a: Vec<PathBuf> = fs::read_dir(self.root.deref())
            .unwrap()
            .into_iter()
            .filter(|a| a.is_ok())
            .map(|e| e.unwrap().path())
            .filter(|b| b.is_dir())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        for b in a
            .iter()
            .filter(|r| !r.ends_with(".git") && !r.ends_with("target"))
        {
            self.dirs.push(Box::new(b.clone()));
        }
        // println!("-------------------------------------------");
        // println!("-------------------------------------------");
        let tmp: Vec<Box<PathBuf>> = self.dirs.clone();
        for i in tmp {
            self.recurse(i.deref());
        }
    }

    fn recurse(&mut self, p: &PathBuf) -> std::io::Result<()> {
        for ent in fs::read_dir(p)? {
            let entry = ent?;
            let path = entry.path();
            let file = entry.file_name();
            match path.is_dir() {
                true => self.recurse(&path)?,
                false => {
                    let dir = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
                    self.tree.insert(dir, PathBuf::from(file));
                }
            }
        }
        Ok(())
    }

    fn sort(&mut self) -> () {
        self.dirs.sort();
        self.files.sort();
    }

    fn sieve(&mut self) -> usize {
        let initDirCnt: usize = self.dirs.len();
        let initFileCnt: usize = self.files.len();
        // dirCnt + fileCnt
        usize::default()
    }

    pub fn print(&mut self) -> () {
        self.sort();
        for d in self.dirs.iter() {
            println!("{}", d.display());
            // for f in self.files.iter() {
            //     println!("----{}", f.display());
            // }
        }
        println!("-------------------------------------------");
        println!("-------------------------------------------");
        println!("Here is the tree...");
        println!("-------------------------------------------");
        println!("-------------------------------------------");
        println!("-------------------------------------------");
        self.tree.print();
    }

    pub fn treeCount(&self) -> usize {
        self.tree.count()
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
