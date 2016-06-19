//! This crate contains server-only components of Playform.
#![deny(missing_docs)]
#![deny(warnings)]

#![feature(box_syntax)]
#![feature(plugin)]
#![feature(test)]
#![feature(unboxed_closures)]

#![plugin(clippy)]
#![allow(let_and_return)]
#![allow(match_ref_pats)]
#![allow(mutex_atomic)]
#![allow(type_complexity)]
#![allow(items_after_statements)]
#![allow(collapsible_if)]
#![allow(match_same_arms)]
#![allow(new_without_default)]
#![allow(doc_markdown)]
#![allow(needless_borrow)]
#![allow(assign_op_pattern)]
#![allow(new_without_default_derive)]

#![allow(mutex_atomic)]

extern crate bincode;
extern crate cgmath;
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