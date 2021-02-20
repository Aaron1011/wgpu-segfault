fn run() {
    wgpu_core::hub::Global::new(
            "wgpu",
            wgpu_core::hub::IdentityManagerFactory,
            wgpu_types::BackendBit::all()
    );
}

fn main() {
    std::thread::spawn(|| {
        run();
    }).join().unwrap();
}
