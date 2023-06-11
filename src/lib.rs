use pronto_graphics::Window;
use roc_std::RocStr;

mod glue;
mod roc;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let glue::R1 { init, draw } = glue::mainForHost();

    let window = match init.discriminant() {
        glue::discriminant_Init::Fullscreen => Window::new_fullscreen(),
        glue::discriminant_Init::Windowed => {
            let init = init.unwrap_Windowed();
            Window::new(init.f0, init.f1, &init.f2.to_string())
        }
    };

    loop {
        
    }
}
