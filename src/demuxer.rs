
use std::io::{Read, Error};

use rusty_ffmpeg::ffi::{
    AVCodecID, 
    AVCodecID_AV_CODEC_ID_H264, 
    AVCodecID_AV_CODEC_ID_HEVC, 
    AVCodecID_AV_CODEC_ID_AV1, 
    AVCodecID_AV_CODEC_ID_OPUS, 
    AVCodecID_AV_CODEC_ID_AAC, 
    AVCodecID_AV_CODEC_ID_PCM_S16LE, 
    AVCodecID_AV_CODEC_ID_NONE, AV_NOPTS_VALUE, AV_PKT_FLAG_KEY, 
};

use crate::libav::{AVPacket, AVCodec};

const SC_PACKET_HEADER_SIZE: usize = 12;
const SC_PACKET_FLAG_CONFIG: u64 = 1<<63;
const SC_PACKET_FLAG_KEY_FRAME: u64 = 1<<62;
const SC_PACKET_PTS_MASK: u64 = SC_PACKET_FLAG_KEY_FRAME-1;

const SCCODEC_ID_H264: u32 = 0x68323634;
const SCCODEC_ID_H265: u32 = 0x68323635;
const SCCODEC_ID_AV1: u32 = 0x00617631;
const SCCODEC_ID_OPUS: u32 = 0x6f707573;
const SCCODEC_ID_AAC: u32 = 0x00616163;
const SCCODEC_ID_RAW: u32 = 0x00726177;

fn decode_codec_id(codec_id: u32)->AVCodecID{
    match codec_id {
        SCCODEC_ID_H264=>AVCodecID_AV_CODEC_ID_H264,
        SCCODEC_ID_H265=>AVCodecID_AV_CODEC_ID_HEVC,
        SCCODEC_ID_AV1 =>AVCodecID_AV_CODEC_ID_AV1,
        SCCODEC_ID_OPUS=>AVCodecID_AV_CODEC_ID_OPUS,
        SCCODEC_ID_AAC=>AVCodecID_AV_CODEC_ID_AAC,
        SCCODEC_ID_RAW=>AVCodecID_AV_CODEC_ID_PCM_S16LE,
        _=>{AVCodecID_AV_CODEC_ID_NONE}
    }
}

struct Demuxer{
 socket: std::net::TcpStream
}
impl Demuxer {
    pub fn read_codec_id(&mut self) -> Result<AVCodecID, std::io::Error>{
        let mut buf = [0u8;4];
        self.socket.read_exact(&mut buf)?;
        let sc_codec_id :u32= u32::from_be_bytes(buf);

        Ok(decode_codec_id(sc_codec_id))
    }
    pub fn read_viceo_size(&mut self)-> Result<(u32,u32), std::io::Error>{
        let mut x = [0u8;4];
        let mut y = [0u8;4];
        self.socket.read_exact(&mut x)?;
        self.socket.read_exact(&mut y)?;
        let h = u32::from_be_bytes(x);
        let w = u32::from_be_bytes(y);
        Ok((h,w))
    }
    pub fn read_packet(&mut self)->Result<AVPacket, std::io::Error>{
        let mut header = [0u8;SC_PACKET_HEADER_SIZE];
        self.socket.read_exact(&mut header)?;
        let pts_flags_buf: [u8;8] = header[0..8].try_into().unwrap();
        let pts_flags = u64::from_be_bytes(pts_flags_buf);
        let len_buf: [u8;4] = header[8..12].try_into().unwrap();
        let len = u32::from_be_bytes(len_buf);
        let mut packet= AVPacket::new();
        packet.new_packet(len.try_into().unwrap());
        packet.read_from_stream(&mut self.socket)?;
        if (pts_flags&SC_PACKET_FLAG_CONFIG) !=0 {
            packet.set_pts(AV_NOPTS_VALUE)
        }
        else {
            packet.set_pts((pts_flags&SC_PACKET_PTS_MASK).try_into().unwrap())
        }
        if (pts_flags&SC_PACKET_FLAG_KEY_FRAME) !=0 {
            packet.set_flags(AV_PKT_FLAG_KEY.try_into().unwrap())
        }
        packet.set_dts(packet.get_pts());
        Ok(packet)
    }
    pub fn run(&mut self)->Result<(), std::io::Error>{
        let codec_id = self.read_codec_id()?;
        if codec_id == AVCodecID_AV_CODEC_ID_NONE{
            return Err(Error::new(std::io::ErrorKind::Other, "No / unknown codec id"));
        }
        let codec = AVCodec::find_decoder(codec_id);
        Ok(())
    }
}