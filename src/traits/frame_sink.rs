use rusty_ffmpeg::ffi::{AVCodecContext, AVFrame};

pub trait FrameSink {
    fn open(&mut self, ctx:AVCodecContext)->bool;
    fn close(&mut self);
    fn push(&mut self, frame:AVFrame)->bool;
}