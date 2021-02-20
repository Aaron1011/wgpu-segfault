use gfx_hal::Instance;

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

    std::thread::spawn(|| {
        run();
    }).join().unwrap();
    println!("Sleeping!");
    std::thread::sleep(std::time::Duration::new(2, 0));
}
