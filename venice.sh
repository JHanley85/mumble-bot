VENICE=/d/Proj-UE4/proj-redpill/SocialVenueUE4/Plugins/UE4-Venice
PATH=$VENICE/gstreamer/1.0/x86_64/bin:$VENICE/gstreamer/1.0/x86_64/lib/gstreamer-1.0:$VENICE/Binaries/ThirdParty/OculusAudioSDK:$VENICE/Binaries/ThirdParty/tensorflow:$PATH
GST_PLUGIN_PATH=/d/Proj-UE4/proj-redpill/SocialVenueUE4/Plugins/UE4-Venice/gstreamer/1.0/x86_64/lib
# gst-inspect-1.0 veniceaudio
GST_DEBUG=3 RUST_BACKTRACE=1 cargo run
# gst-inspect-1.0 oculuspositional
# gst-launch-1.0 --gst-debug-level=3 audiotestsrc ! audioconvert ! audioresample ! capsfilter caps=audio/x-raw,channels=2,format=F32LE,layout=interleaved,rate=44100 ! oculuspositional in-use=1 range-min=0 range-max=100 x=100 y=0 z=0  ! audioconvert ! audioresample ! autoaudiosink
# gst-launch-1.0 audiotestsrc ! audioconvert ! audioresample ! audiobuffersplit output-buffer-duration=512/44100 ! redpillbasicsignaldata stem-id=0 ! audioconvert ! audioresample ! capsfilter caps=audio/x-raw,channels=2  ! oculuspositional in-use=1 range-min=3000 range-max=10000 x=3000 y=3000 z=0 ! audioconvert ! audioresample ! autoaudiosink
# gst-inspect-1.0 audiobuffersplit
# gst-inspect-1.0 capsfilter
# GST_DEBUG=3 gst-launch-1.0 filesrc location=data/man16kHz.raw ! \
# capsfilter caps=audio/x-raw,format=S16LE,channels=1,rate=16000 ! audioconvert ! audioresample ! \
# capsfilter caps=audio/x-raw,format=F32LE,channels=1,rate=44100,layout=interleaved ! audioconvert ! audioresample ! \
# audiobuffersplit output-buffer-duration=512/44100 ! redpillbasicsignaldata stem-id=0 ! audioconvert ! audioresample ! \
# capsfilter caps=audio/x-raw,channels=2 ! oculuspositional in-use=1 range-min=3000 range-max=10000 x=3000 y=3000 z=0 ! audioconvert ! audioresample ! \
# fakesink dump=true
# autoaudiosink
# gst-launch-1.0 filesrc location=data/man16kHz.raw ! capsfilter caps=audio/x-raw,format=S16LE,channels=1,rate=16000 ! audioconvert ! audioresample ! autoaudiosink
#GST_DEBUG=*:2 gst-launch-1.0 -v filesrc location=data/man16kHz.raw ! capsfilter caps=audio/x-raw,format=S16LE,channels=1,rate=16000,layout=interleaved ! audioconvert ! audioresample ! \
# GST_DEBUG=*:2 gst-launch-1.0 -v filesrc location=data/thx.mp3 ! decodebin ! audioconvert ! audioresample ! \
# capsfilter caps=audio/x-raw,format=F32LE,channels=1,rate=44100,layout=interleaved ! \
# audiobuffersplit output-buffer-duration=512/44100 ! audioconvert ! audioresample ! \
# capsfilter caps=audio/x-raw,format=S16LE,channels=2,rate=44100,layout=interleaved ! \
# audioconvert ! audioresample ! autoaudiosink

#gst-launch-1.0 audiotestsrc ! audioconvert ! audioresample ! audiobuffersplit output-buffer-duration=512/44100 ! redpillbasicsignaldata stem-id=0 ! audioconvert ! audioresample ! capsfilter caps=audio/x-raw,channels=2  ! oculuspositional in-use=1 range-min=3000 range-max=10000 x=3000 y=3000 z=0 ! audioconvert ! audioresample ! autoaudiosink
#gst-launch-1.0 filesrc location=man16kHz.raw ! capsfilter caps=audio/x-raw,format=S16LE,channels=1,rate=16000,layout=interleaved ! audioconvert ! audioresample ! audiobuffersplit output-buffer-duration=512/44100 ! redpillbasicsignaldata stem-id=0 ! audioconvert ! audioresample ! capsfilter caps=audio/x-raw,channels=2  ! oculuspositional in-use=1 range-min=3000 range-max=10000 x=3000 y=3000 z=0 ! audioconvert ! audioresample ! autoaudiosink
 

