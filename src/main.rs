use cogrrs::main_loop_run;
use smol_voxel_world::SmolVoxelWorld;

mod compute_passes;
mod constants;
mod helpers;
mod io;
mod key_mapping;
mod smol_voxel_world;
mod world;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("could not setup logging");

    //return;
    main_loop_run::<SmolVoxelWorld>(constants::TICKS_PER_SECOND).unwrap();
}
