//! SigmaOS-btrfs-progs — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_btrfs_progs_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-btrfs-progs: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-btrfs-progs: ok");
}
