extern crate tangra;

fn main() {
    let mut logger = tangra::Logger::new();
    logger.set_log_file_path("example.log");
    logger.debug("$[fg:red]$[effect:blink]$[effect:bold]$[sys:username] says hello!");
}
