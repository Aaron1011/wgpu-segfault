use gfx_hal::Instance;

fn run() {
    gfx_backend_gl::Instance::create("wgpu", 1).unwrap();
    println!("Thread exiting!");
}

fn main() {
    std::thread::spawn(|| {
        run();
    }).join().unwrap();
    println!("Back in main!");
}
