
extern "C"{
    fn check_app_canary();
    fn zemu_log_stack(ctx: *const u8);
    fn zemu_log(buf: *const u8);
    fn zemu_log_num(buf: *const u8, num: u32);
}

pub fn z_check_app_canary(){
    unsafe{
        check_app_canary()
    }
}

pub fn zlog(buf: &str){
    unsafe{
        zemu_log(buf.as_bytes().as_ptr())
    }
}
pub fn zlog_stack(buf: &str){
    unsafe{
        zemu_log_stack(buf.as_bytes().as_ptr())
    }
}
pub fn zlog_num(buf: &str, num: u32){
    unsafe{
        zemu_log_num(buf.as_bytes().as_ptr(), num)
    }
}