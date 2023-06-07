use rusty_ffmpeg::ffi::{AVPacket as RawAVPacket, av_packet_alloc, av_packet_free, av_new_packet};
pub struct AVPacket(*mut RawAVPacket);
impl AVPacket{
    #[inline]
    pub fn new()->Self{
        unsafe{
            Self(av_packet_alloc())
        }
    }
    #[inline]
    pub fn new_packet(&mut self, len:i32){
        unsafe{
            av_new_packet(self.0, len);
        }
    }
    #[inline]
    pub fn read_from_stream<T>(&mut self, stream: &mut T)->std::io::Result<()>
    where T: std::io::Read
    {
        unsafe{
            let len = (*(*self.0).buf).size;
            let mut packet_data_buffer: Vec<u8> = Vec::from_raw_parts((*(*self.0).buf).data, len, len);
            stream.read_exact(&mut packet_data_buffer)
        }
    }
    #[inline]
    pub fn set_pts(&mut self, pts: i64){
        unsafe {
            (*self.0).pts = pts
        }
    }
    #[inline]
    pub fn get_pts(&self)->i64{
        unsafe {
            (*self.0).pts
        }
    }
    #[inline]
    pub fn set_flags(&mut self, flags: i32){
        unsafe {
            (*self.0).flags |= flags
        }
    }
    #[inline]
    pub fn unset_flags(&mut self, flags: i32){
        unsafe {
            (*self.0).flags &= flags
        }
    }
    #[inline]
    pub fn get_flags(&mut self)->i32{
        unsafe {
            (*self.0).flags
        }
    }
    
    #[inline]
    pub fn set_dts(&mut self, dts: i64){
        unsafe {
            (*self.0).dts = dts
        }
    }
    #[inline]
    pub fn get_dts(&self)->i64{
        unsafe {
            (*self.0).dts
        }
    }
}



impl Drop for AVPacket{
    fn drop(&mut self) {
        unsafe {
            av_packet_free(&mut self.0)
        }
    }
}