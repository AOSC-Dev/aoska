#[cfg(any(target_os = "linux", test))]
mod linux_graphics;

fn main() {
    #[cfg(target_os = "linux")]
    linux_graphics::configure();
    aoska::run();
}
