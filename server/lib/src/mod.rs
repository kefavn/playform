//! This crate contains server-only components of Playform.
#![deny(missing_docs)]
#![deny(warnings)]

#![feature(box_syntax)]
#![feature(test)]
#![feature(unboxed_closures)]

extern crate bincode;
extern crate cgmath;
extern crate collision;
extern crate common;
#[macro_use]
extern crate log;
extern crate nanomsg;
extern crate num;
extern crate rand;
extern crate rustc_serialize;
extern crate stopwatch;
extern crate terrain;
extern crate test;
extern crate thread_scoped;
extern crate time;
extern crate voxel_data;

mod client_recv_thread;
mod entity;
mod in_progress_terrain;
mod init_mobs;
mod lod;
mod mob;
mod octree;
mod physics;
mod player;
mod run;
mod server;
mod sun;
mod terrain_loader;
mod update_gaia;
mod update_world;

pub use run::run;
