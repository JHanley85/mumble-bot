use futures::{Sink, Stream};
use futures::future::{Future, ok, err, loop_fn, IntoFuture, Loop};
use futures;
use std;
use cgmath::*;

#[derive(Copy, Clone, Debug)]
pub struct PositionalAudio {
    pub loc : Vector3<f32>,
    pub rot : Quaternion<f32>,
}

impl PositionalAudio {
    pub fn zero() -> PositionalAudio {
        PositionalAudio {loc: Vector3::zero(), rot: Quaternion::one()}
    }
}

pub struct VoxIn {
    pub session_id: u64,
    pub last_io: std::time::SystemTime,
    pub tx: futures::sync::mpsc::Sender<(Vec<u8>, PositionalAudio)>,
}
