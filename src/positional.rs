use futures::{Sink, Stream};
use futures::future::{Future, ok, err, loop_fn, IntoFuture, Loop};
use futures;

pub struct PositionalAudio {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// type VoxIn = futures::sync::mpsc::channel<(i32, Vec<u8>, PositionalAudio)>;