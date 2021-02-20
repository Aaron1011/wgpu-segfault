use gfx_hal::Instance;
use env_logger::Builder;
use log::LevelFilter;

fn run() {
    /*wgpu_core::hub::Global::new(
            "wgpu",
            wgpu_core::hub::IdentityManagerFactory,
            wgpu_types::BackendBit::all()
    );*/
    gfx_backend_gl::Instance::create("wgpu", 1);
    println!("Thread exiting!");
}

fn main() {

    let mut builder = Builder::new();
    builder.filter_level(LevelFilter::Trace);
    builder.init();


    std::thread::spawn(|| {
        run();
    }).join().unwrap();
    println!("Back in main!");
}
