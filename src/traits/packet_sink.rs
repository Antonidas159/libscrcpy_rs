use rusty_ffmpeg::ffi::AVCodecContext;

use crate::libav::AVPacket;
pub trait PacketSink{
    fn open(&mut self, avctx: AVCodecContext)->bool;
    fn close(&mut self);
    fn push(&mut self, packet: AVPacket)->bool;
    fn disable(&mut self);
}