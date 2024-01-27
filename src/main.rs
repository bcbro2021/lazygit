mod base;
use base::*;

use std::env;
fn main() {
    let version = 0.2;
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        if args[1] == "init" {
            git_init();
        } else if args[1] == "deploy" {
            git_deploy();
        } else if args[1] == "clone" {
            git_clone(&args);
    
        } else if args[1] == "help" {
            print_help(&version);
        }
    } else {
        print_help(&version);
    }

    
    
}
