use super::imported::{AVCodecID, avcodec_find_decoder};


pub struct AVCodec(*const super::imported::AVCodec);
impl AVCodec{
    pub fn find_decoder(id: AVCodecID)->Option<Self>{
        unsafe{
            let codec = avcodec_find_decoder(id);
            if codec != std::ptr::null(){
                Some(Self(codec))
            }else {
                None
            }
        }
    }
}