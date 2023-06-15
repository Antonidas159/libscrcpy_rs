use super::imported;
pub struct AVCodecContext(*mut imported::AVCodecContext);
impl AVCodecContext{
    #[inline]
    pub unsafe fn from_raw_ptr(ptr : *mut imported::AVCodecContext)->Self{
        Self(ptr)
    }
    #[inline]
    pub fn set_flags(&mut self, flags:u32){
        unsafe{
            (*(self.0)).flags |= flags as i32;
        }
    }
    pub fn unset_flags(&mut self, flags:u32){
        unsafe{
            (*(self.0)).flags &= flags as i32;
        }
    }
    pub fn set_width(&mut self, width: i32){
        unsafe{
            (*(self.0)).width = width.try_into().unwrap()
        }
    }
    pub fn set_height(&mut self, height: i32){
        unsafe{
            (*(self.0)).height = height.try_into().unwrap()
        }
    }
    pub fn set_pixel_format(&mut self, format:i32){
        unsafe{
            (*(self.0)).pix_fmt = format
        }
    }pub fn set_sample_rate(&mut self, rate:i32){
        unsafe{
            (*(self.0)).sample_rate = rate
        }
    }
}