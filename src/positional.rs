use futures::{Sink, Stream};
use futures::future::{Future, ok, err, loop_fn, IntoFuture, Loop};
use futures;
use std;

#[derive(Clone, Debug)]
pub struct PositionalAudio {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PositionalAudio {
    pub fn zero() -> PositionalAudio {
        PositionalAudio {x: 0f32, y: 0f32, z: 0f32}
    }
}

pub struct VoxIn {
    pub session_id: u64,
    pub last_io: std::time::SystemTime,
    pub tx: futures::sync::mpsc::Sender<(Vec<u8>, PositionalAudio)>,
}
