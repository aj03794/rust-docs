use std::env;
use std::process;

use minigrep;
use minigrep::Config;   


fn main() {
    let args: Vec<String> = env::args().collect();

    // " |err| " is the Err in the Config::new function if it occurs
    let config = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1)
    });

    // run(config);
    // Because "run" only returns "()" in teh success case, we only care about detecting
    // an error, so we don't need "unwarp_or_else" to return the unwrapped value because it
    // would only be "()"
    if let Err(e) = minigrep::run(config) {
        process::exit(1);
    }

}