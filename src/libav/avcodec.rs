use std::any::Any;

use super::imported::{AVCodecID, avcodec_find_decoder, avcodec_alloc_context3};
use super::AVCodecContext;

pub struct AVCodec(*const super::imported::AVCodec);
impl AVCodec{
    pub fn find_decoder(id: AVCodecID)->Option<Self>{
        unsafe{
            let codec = avcodec_find_decoder(id);
            if codec.is_null(){
                None
            }else {
                Some(Self(codec))
            }
        }
    }
    pub fn alloc_context3(&self)->Option<AVCodecContext>{
        unsafe{
            let ctx = avcodec_alloc_context3(self.0);
            if ctx.is_null(){
                None
            }else {
                Some(AVCodecContext::from_raw_ptr(ctx))
            }

        }
    }
    pub fn get_type(&self)->i32{
        unsafe{(
            *(self.0)).type_ 
        }
    }
}