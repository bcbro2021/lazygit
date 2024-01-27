
use std::process::Command;
use std::env;

pub fn print_from_ascii(bytes: &Vec<u8>) {
    for char in bytes {
        print!("{}",*char as char);
    }
    print!("\n");
}

pub fn print_help(version: &f64) {
    println!("  LAZYGIT v{}\n
    init - creates a new git project
    deploy - push changes to master branch of your repo
    clone - clones a repo",version);
}

pub fn git_init() {
    let output = Command::new("git")
                                    .args(["init"])
                                    .output()
                                    .expect("failed to execute process");
    print_from_ascii(&output.stdout);
}

pub fn git_deploy() {
    let pull = Command::new("git")
                                .arg("pull")
                                .output()
                                .expect("failed to execute process");

    print_from_ascii(&pull.stdout);

    let addfiles = Command::new("git")
                                    .args(["add", "."])
                                    .output()
                                    .expect("failed to execute process");
    print_from_ascii(&addfiles.stdout);

    let commit_message = "\"changes made\"";
    let commit = Command::new("git")
                                    .args(["commit","-m", commit_message])
                                    .output()
                                    .expect("failed to execute process");
    print_from_ascii(&commit.stdout);

    let push = Command::new("git")
                                    .args(["push"])
                                    .output()
                                    .expect("failed to execute process");
    print_from_ascii(&push.stdout);
}

pub fn git_clone(args: &Vec<String>) {
    println!("Cloning repo - {}",args[2]);
    let out = Command::new("git")
                                .args(["clone",args[2].as_str()])
                                .output()
                                .expect("failed to execute process");
    print_from_ascii(&out.stdout);
    println!("Done!");
}
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
