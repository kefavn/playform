//! Defines the messages passed between client and server.

use cgmath::{Vector2, Vector3, Point3};
use collision::{Aabb3};
use std::default::Default;
use std::ops::Add;

use entity;
use voxel;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, RustcEncodable, RustcDecodable)]
/// Unique client ID.
pub struct ClientId(u32);

impl Default for ClientId {
  fn default() -> ClientId {
    ClientId(0)
  }
}

impl Add<u32> for ClientId {
  type Output = ClientId;

  fn add(self, rhs: u32) -> ClientId {
    let ClientId(i) = self;
    ClientId(i + rhs)
  }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
/// Messages the client sends to the server.
pub enum ClientToServer {
  /// Notify the server that the client exists, and provide a "return address".
  Init(String),
  /// Ping
  Ping(ClientId),
  /// Ask the server to create a new player.
  AddPlayer(ClientId),
  /// Add a vector the player's acceleration.
  Walk(entity::id::Player, Vector3<f32>),
  /// Rotate the player by some amount.
  RotatePlayer(entity::id::Player, Vector2<f32>),
  /// [Try to] start a jump for the player.
  StartJump(entity::id::Player),
  /// [Try to] stop a jump for the player.
  StopJump(entity::id::Player),
  /// Ask the server to send a block of terrain.
  RequestVoxels {
    /// The time, in nanoseconds, when the voxels were requested.
    time_requested_ns : u64,
    /// The ID of the requesting client.
    client_id       : ClientId,
    /// The bounds of the voxels to fetch.
    voxels          : Vec<voxel::bounds::T>,
  },
  /// Brush-remove where the player's looking.
  Add(entity::id::Player),
  /// Brush-add at where the player's looking.
  Remove(entity::id::Player),
}

/// Why a block is being sent to a client.
#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub enum VoxelReason {
  /// The client asked for it.
  Requested {
    /// The time, in nanoseconds, when the voxels were requested.
    at: u64,
  },
  /// The block has been updated.
  Updated,
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
/// Collision events. First ID is "collider", rest of IDs are collidee(s).
#[allow(missing_docs)]
pub enum Collision {
  PlayerTerrain(entity::id::Player),
  PlayerMisc(entity::id::Player),
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
/// Messages the server sends to the client.
pub enum ServerToClient {
  /// Provide the client a unique id to tag its messages.
  LeaseId(ClientId),
  /// Ping
  Ping,

  /// Complete an AddPlayer request.
  PlayerAdded(entity::id::Player, Point3<f32>),

  /// Update a player's position.
  UpdatePlayer(entity::id::Player, Aabb3<f32>),
  /// Update the client's view of a mob with a given mesh.
  UpdateMob(entity::id::Mob, Aabb3<f32>),
  /// The sun as a [0, 1) portion of its cycle.
  UpdateSun(f32),

  /// Provide a block of terrain to a client.
  Voxels {
    /// The voxels requested, and their associated bounds.
    voxels : Vec<(voxel::bounds::T, voxel::T)>,
    /// The reason the voxels are being sent.
    reason : VoxelReason,
  },
  /// A collision happened.
  Collision(Collision),
}
