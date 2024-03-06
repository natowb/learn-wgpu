

pub mod renderer;

pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {

        env_logger::init();
        pollster::block_on(renderer::run());
    }
}
