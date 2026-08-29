use cudarc::{
    driver::{CudaContext, LaunchConfig},
    nvrtc::Ptx,
};

const HELLO_WORLD_PTX: &str = include_str!(concat!(env!("OUT_DIR"), "/hello_world.ptx"));

// Being run from a machine with only 1 GPU
const GPU_IDX: usize = 0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world from Rust!");

    let ctx = CudaContext::new(GPU_IDX)?;
    let stream = ctx.default_stream();

    let ptx = Ptx::from_src(HELLO_WORLD_PTX);
    let module = ctx.load_module(ptx)?;
    let kernel = module.load_function("hello_world")?;

    // Set to just launch on one thread
    let config = LaunchConfig {
        grid_dim: (1, 1, 1),
        block_dim: (1, 1, 1),
        shared_mem_bytes: 0,
    };

    unsafe {
        // SAFETY: hello_world takes no arguments, so there's nothing to mismatch.
        stream.launch_builder(&kernel).launch(config)?;
    }

    stream.synchronize()?;

    Ok(())
}
