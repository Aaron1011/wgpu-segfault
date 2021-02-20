use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
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
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    std::thread::spawn(|| {
        run();
    }).join().unwrap();
    println!("Sleeping!");
    std::thread::sleep(std::time::Duration::new(2, 0));
}
