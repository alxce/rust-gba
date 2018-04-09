// Graphics implementation
// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

pub mod colour;
pub mod bitmap;
pub mod tilemap;

use graphics::bitmap::{BitmapMode, Mode3};

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum Mode {
    MODE0 = 0,
    MODE1,
    MODE2,
    MODE3,
    MODE4,
    MODE5,
}

pub enum None {}

pub struct Graphics<T> {
    mode: T
}

// Methods for all graphics implementations
impl <T>Graphics<T> {
    // Fetch active graphics mode instance
    pub fn active(&mut self) -> &mut T {
        &mut self.mode
    }
    
    // Set graphics to mode 3
    pub fn mode3(mut self) -> Graphics<bitmap::Mode3> {
        Graphics::<bitmap::Mode3>::new()
    }

    // Set graphics to mode 3
    pub fn mode4(mut self) -> Graphics<bitmap::Mode4> {
        Graphics::<bitmap::Mode4>::new()
    }

    // Set graphics to mode 3
    pub fn mode5(mut self) -> Graphics<bitmap::Mode5> {
        Graphics::<bitmap::Mode5>::new()
    }
}

// Methods for bitmap::Mode3 graphics
impl Graphics<bitmap::Mode3> {
    // Create a new Mode3 graphics instance
    pub fn new() -> Graphics<bitmap::Mode3> {
        let mut mode = bitmap::Mode3::new();
        mode.enable();
        Graphics::<bitmap::Mode3>{mode: mode}
    }
}

// Methods for bitmap::Mode4 graphics
impl Graphics<bitmap::Mode4> {
    // Create a new Mode4 graphics instance
    pub fn new() -> Graphics<bitmap::Mode4> {
        let mut mode = bitmap::Mode4::new();
        mode.enable();
        Graphics::<bitmap::Mode4>{mode: mode}
    }
}

// Methods for bitmap::Mode5 graphics
impl Graphics<bitmap::Mode5> {
    // Create a new Mode5 graphics instance
    pub fn new() -> Graphics<bitmap::Mode5> {
        let mut mode = bitmap::Mode5::new();
        mode.enable();
        Graphics::<bitmap::Mode5>{mode: mode}
    }
}