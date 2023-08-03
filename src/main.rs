use survex_dist::command::{fatal_error, run};

fn main() {
    env_logger::init();

    match run() {
        Ok(_) => {}
        Err(e) => {
            fatal_error(e.to_string());
        }
    }
}
