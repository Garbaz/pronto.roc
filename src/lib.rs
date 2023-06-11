use roc_std::RocStr;

mod roc;
mod glue;

#[repr(u8)]
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum RocInit {
    Windowed(u32, u32, RocStr),
    Fullscreen,
}

extern "C" {
    #[link_name = "roc__initForHost_1_exposed"]
    fn init() -> RocInit;
}

extern "C" {
    #[link_name = "roc__drawForHost_1_exposed"]
    fn draw() -> ();
}

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let init = unsafe { init() };

    println!("{:?}", init);

    // Exit code
    0
}
