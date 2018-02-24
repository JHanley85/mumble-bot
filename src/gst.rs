use gstreamer as gst;
use gstreamer_app as gst_app;
use gstreamer_audio as gst_audio;
use self::gst::prelude::*;
use glib;

use byte_slice_cast::*;

use std;
use std::i16;
use std::i32;
use std::io::ErrorKind;
use std::thread;
use std::sync::{Arc, Mutex};

use std::error::Error as StdError;
use failure::Error;

use futures;
use futures::{Sink, Stream};
use futures::future::{err, loop_fn, ok, Future, IntoFuture, Loop};

use positional::*;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
struct MissingElement(&'static str);

#[derive(Debug, Fail)]
#[fail(display = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    #[cause]
    cause: glib::Error,
}

fn sink_pipeline(vox_out_tx: futures::sync::mpsc::Sender<Vec<u8>>) -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);

    let src =
        gst::ElementFactory::make("autoaudiosrc", None).ok_or(MissingElement("autoaudiosrc"))?;

    let resample =
        gst::ElementFactory::make("audioresample", None).ok_or(MissingElement("audioresample"))?;

    let caps = gst::Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &gst_audio::AUDIO_FORMAT_S16.to_string()),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &(16000i32)),
        ],
    );

    let caps_filter =
        gst::ElementFactory::make("capsfilter", None).ok_or(MissingElement("capsfilter"))?;
    if let Err(err) = caps_filter.set_property("caps", &caps) {
        panic!("caps_filter.set_property:caps {:?}", err);
    }

    let sink = gst::ElementFactory::make("appsink", None).ok_or(MissingElement("appsink"))?;

    pipeline.add_many(&[&src, &resample, &caps_filter, &sink])?;

    src.link(&resample)?;
    resample.link(&caps_filter)?;
    caps_filter.link(&sink)?;

    let appsink = sink.clone()
        .dynamic_cast::<gst_app::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    appsink.set_caps(&caps);
    let vox_out_tx = Arc::new(Mutex::new(vox_out_tx.wait()));

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            .new_sample(move |appsink| {
                let sample = match appsink.pull_sample() {
                    None => return gst::FlowReturn::Eos,
                    Some(sample) => sample,
                };

                let buffer = sample
                    .get_buffer()
                    .expect("Unable to extract buffer from the sample");

                let map = buffer
                    .map_readable()
                    .expect("Unable to map buffer for reading");

                if let Ok(samples) = map.as_slice().as_slice_of::<u8>() {
                    let mut vox_out_tx = vox_out_tx.lock().unwrap();
                    let v = samples.to_vec();
                    if let Err(_) = vox_out_tx.send(v) {
                        gst::FlowReturn::Error
                    } else {
                        return gst::FlowReturn::Ok;
                    }
                } else {
                    return gst::FlowReturn::Error;
                }
            })
            .build(),
    );

    Ok(pipeline)
}

fn sink_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing).into_result()?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    println!("start sink_loop");

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use self::gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null).into_result()?;
                Err(ErrorMessage {
                    src: msg.get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                })?;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).into_result()?;

    println!("stop sink_loop");

    Ok(())
}

pub fn sink_main(vox_out_tx: futures::sync::mpsc::Sender<Vec<u8>>) -> impl Fn() -> () {
    let pipeline = sink_pipeline(vox_out_tx).unwrap();
    let p = pipeline.clone();

    thread::spawn(move || {
        let _ = sink_loop(pipeline);
    });

    move || {
        let ev = gst::Event::new_eos().build();
        p.send_event(ev);
    }
}

fn src_pipeline() -> Result<(gst::Pipeline, Vec<gst_app::AppSrc>), Error> {
    gst::init()?;

    // let caps0 = gst::Caps::new_simple(
    //     "audio/x-raw",
    //     &[
    //         ("format", &gst_audio::AUDIO_FORMAT_F32.to_string()),
    //         ("rate", &(44100i32)),
    //         ("layout", &"interleaved"),
    //         ("channels", &(2i32)),
    //     ],
    // );

    let caps = gst::Caps::new_simple(
        "audio/x-raw",
        &[
            ("format", &gst_audio::AUDIO_FORMAT_S16.to_string()),
            ("layout", &"interleaved"),
            ("channels", &(1i32)),
            ("rate", &(16000i32)),
        ],
    );

    let pipeline = gst::Pipeline::new(None);

    let mut appsrcs = Vec::new();
    for _ in 0..16 {
        // let audiotestsrc =
        //     gst::ElementFactory::make("audiotestsrc", None).ok_or(MissingElement("audiotestsrc"))?;

        let appsrc = gst::ElementFactory::make("appsrc", None).ok_or(MissingElement("appsrc"))?;

        let audioconvert0 =
            gst::ElementFactory::make("audioconvert", None).ok_or(MissingElement("audioconvert"))?;
        // let audioresample0 = gst::ElementFactory::make("audioresample", None)
        //     .ok_or(MissingElement("audioresample"))?;

        // let audiobuffersplit = gst::ElementFactory::make("audiobuffersplit", None)
        //     .ok_or(MissingElement("audiobuffersplit"))?;
        // audiobuffersplit
        //     .set_property("output-buffer-duration", &gst::Fraction::new(512, 44100))
        //     .expect("Unable to set property in the element");
        // // audiobuffersplit
        // //     .set_property("strict-buffer-size", &(true))
        // //     .expect("Unable to set property in the element");
        // // audiobuffersplit
        // //     .set_property("alignment-threshold", &(0u64))
        // //     .expect("Unable to set property in the element");
        // // audiobuffersplit
        // //     .set_property("discont-wait", &(0u64))
        // //     .expect("Unable to set property in the element");

        // let redpillbasicsignaldata = gst::ElementFactory::make("redpillbasicsignaldata", None)
        //     .ok_or(MissingElement("redpillbasicsignaldata"))?;
        // redpillbasicsignaldata
        //     .set_property("stem-id", &(0u64))
        //     .expect("Unable to set property in the element");

        // let audioconvert1 =
        //     gst::ElementFactory::make("audioconvert", None).ok_or(MissingElement("audioconvert"))?;
        // let audioresample1 = gst::ElementFactory::make("audioresample", None)
        //     .ok_or(MissingElement("audioresample"))?;

        // let capsfilter =
        //     gst::ElementFactory::make("capsfilter", None).ok_or(MissingElement("capsfilter"))?;
        // capsfilter
        //     .set_property("caps", &caps0)
        //     .expect("Unable to set property in the element");

        // let positional = gst::ElementFactory::make("oculuspositional", None)
        //     .ok_or(MissingElement("positional"))?;
        // positional
        //     .set_property("range-min", &(0.0f32))
        //     .expect("Unable to set property in the element");
        // positional
        //     .set_property("range-max", &(1000.0f32))
        //     .expect("Unable to set property in the element");
        // positional
        //     .set_property("in-use", &(true))
        //     .expect("Unable to set property in the element");
        // positional
        //     .set_property("x", &(0.0f32))
        //     .expect("Unable to set property in the element");
        // positional
        //     .set_property("y", &(0.0f32))
        //     .expect("Unable to set property in the element");
        // positional
        //     .set_property("z", &(0.0f32))
        //     .expect("Unable to set property in the element");

        // let audioconvert2 =
        //     gst::ElementFactory::make("audioconvert", None).ok_or(MissingElement("audioconvert"))?;
        // let audioresample2 = gst::ElementFactory::make("audioresample", None)
        //     .ok_or(MissingElement("audioresample"))?;

        let sink = gst::ElementFactory::make("autoaudiosink", None)
            .ok_or(MissingElement("autoaudiosink"))?;
        sink.set_property("async-handling", &true)
            .expect("Unable to set property in the element");

        pipeline.add_many(&[
            &appsrc,
            // &audiotestsrc,
            &audioconvert0,
            // &audioresample0,
            // &audiobuffersplit,
            // &redpillbasicsignaldata,
            // &audioconvert1,
            // &audioresample1,
            // &capsfilter,
            // &positional,
            // &audioconvert2,
            // &audioresample2,
            &sink,
        ])?;

        // audiotestsrc.link(&audioconvert0)?;
        // appsrc.link(&audioconvert0)?;
        // audioconvert0.link(&audioresample0)?;
        // audioresample0.link(&audiobuffersplit)?;
        // audiobuffersplit.link(&redpillbasicsignaldata)?;
        // redpillbasicsignaldata.link(&audioconvert1)?;
        // audioconvert1.link(&audioresample1)?;
        // audioresample1.link(&capsfilter)?;
        // capsfilter.link(&positional)?;
        // positional.link(&audioconvert2)?;
        // audioconvert2.link(&audioresample2)?;
        // audioresample2.link(&sink)?;

        appsrc.link(&audioconvert0)?;
        audioconvert0.link(&sink)?;

        let appsrc = appsrc
            .clone()
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("Source element is expected to be an appsrc!");
        appsrc.set_caps(&caps);

        appsrcs.push(appsrc);
    }

    // let appsrcs = appsrcs.iter().map(|src| {
    //     let appsrc = (*src).clone()
    //         .dynamic_cast::<gst_app::AppSrc>()
    //         .expect("Source element is expected to be an appsrc!");
    //     appsrc.set_caps(&caps);
    //     appsrc
    // });

    // let appsrcs = appsrcs.collect();

    pipeline.use_clock(None::<&gst::Clock>);

    Ok((pipeline, appsrcs))
}

fn src_rx<'a>(
    _pipeline: gst::Pipeline,
    appsrcs: Vec<gst_app::AppSrc>,
    vox_inp_rx: futures::sync::mpsc::Receiver<(i32, Vec<u8>, PositionalAudio)>,
) -> impl Future<Item = (), Error = std::io::Error> + 'a {
    vox_inp_rx
        .fold(appsrcs, |appsrcs, (session, bytes, _pos)| {
            if session >= 0 && session < 16 {
                // pipeline.

                let idx = session as usize;
                let buffer =
                    gst::Buffer::from_slice(bytes).expect("gst::Buffer::from_slice(bytes)");
                //buffer.set_pts(i * 500 * gst::MSECOND);
                // println!("x: {}", pos.x);
                // elems[idx]
                //     .1
                //     .set_property("x", &(pos.x))
                //     .expect("Unable to set property in the element");
                // elems[idx]
                //     .1
                //     .set_property("y", &(pos.y))
                //     .expect("Unable to set property in the element");
                // elems[idx]
                //     .1
                //     .set_property("z", &(pos.z))
                //     .expect("Unable to set property in the element");
                // elems[idx].0.push_buffer(buffer);
                if appsrcs[idx].push_buffer(buffer) != gst::FlowReturn::Ok {
                    let _ = appsrcs[idx].end_of_stream();
                    err(())
                } else {
                    ok(appsrcs)
                }
                // ok(elems)
            } else {
                ok::<_, ()>(appsrcs)
            }
        })
        .map(|_| ())
        .map_err(|_| std::io::Error::new(ErrorKind::Other, "vox_inp_task"))
}

fn src_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing).into_result()?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    println!("start src_loop");

    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use self::gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null).into_result()?;
                Err(ErrorMessage {
                    src: msg.get_src()
                        .map(|s| s.get_path_string())
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().description().into(),
                    debug: err.get_debug(),
                    cause: err.get_error(),
                })?;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).into_result()?;

    println!("stop src_loop");

    Ok(())
}

pub fn src_main<'a>(
    vox_inp_rx: futures::sync::mpsc::Receiver<(i32, Vec<u8>, PositionalAudio)>,
) -> (
    impl Fn() -> (),
    impl Future<Item = (), Error = std::io::Error> + 'a,
) {
    let (pipeline, appsrcs) = src_pipeline().unwrap();
    let p0 = pipeline.clone();
    let p1 = pipeline.clone();

    thread::spawn(move || {
        println!("start thread src_loop");
        src_loop(pipeline).unwrap();
        println!("stop thread src_loop");
    });

    let kill_pipe = move || {
        let ev = gst::Event::new_eos().build();
        p0.send_event(ev);
    };

    (kill_pipe, src_rx(p1, appsrcs, vox_inp_rx))
}
