// --------------------------------------------------------------------------------------------------------------------------
//!
//! File: vcs/bin/src/main.rs
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

use io::store::file::LocalData;
use io::*;
use user::*;

fn main() {
    let mut c: io::handler::seek::Crawler = io::handler::seek::Crawler::new();
    // let mut u: user::info::VCSUser = user::info::VCSUser::new();
    // u.setName("Jacob Davisson".to_owned());
    // u.setEmail("davissonjb@gmail.com".to_owned());
    // println!("{}", u);
    c.setCurr();
    c.init();
    println!("Printing directory listing::");
    println!("-----------------------------");
    println!("-----------------------------");
    c.print();
    println!("-----------------------------");
    println!("Tree count: {}", c.treeCount());
    println!("-----------------------------");
    // println!("Hello, world!");
    // let ld: LocalData = LocalData::new();
    // println!("{}", ld);
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
