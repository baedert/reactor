use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::io::Read;


fn entry_exists(name: &str) -> bool {
    let mut path = PathBuf::from("/home/baedert/.config/reactor/");
    path.push(name);

    let f = File::open(path);

    match f {
        Ok(_)  => return true,
        _      => return false
    }
}


fn get_entry (args: &Vec<String>) {
    if args.len() < 3 {
        println!("ERR: 'get' used but no argument given");
        return;
    }
    let entry_name = &args[2];

    if !entry_exists(entry_name) {
        panic!("Entry called '{}' does not exist", entry_name);
    }

    let mut path = PathBuf::from("/home/baedert/.config/reactor/");
    path.push(entry_name);
    let mut file = File::open(path).unwrap();

    let mut buf = String::new();
    let result = file.read_to_string(&mut buf);
    match result {
        Err(e) => panic!("Error at read: {}", e),
        Ok(_)  => {}
    }

    println!("{}", buf);
}


fn add_entry(args: &Vec<String>) {
    if args.len() < 4 {
        println!("ERR: Name and content needed");
        return;
    }

    let entry_name    = &args[2];
    let entry_content = &args[3];

    if entry_exists(entry_name) {
        println!("ERR: Entry '{}' does already exist", entry_name);
        return;
    }

    let mut path = PathBuf::from("/home/baedert/.config/reactor/");
    path.push(entry_name);

    let mut file = File::create(path).unwrap();
    let result = file.write_all(entry_content.as_bytes());

    match result {
        Err(e) => panic!("Error while writing: {}", e),
        Ok(_)  => {}
    }
}


/*
 * Save entries in ~/.config/reactor/<name>
 */
fn list_entries () {
    let paths = fs::read_dir(&Path::new("/home/baedert/.config/reactor")).unwrap ();

    for path in paths {
        println!("     {}", path.unwrap().path().file_name().unwrap().to_str().unwrap());
    }
}


fn delete_entry (args: &Vec<String>) {
    if args.len() < 3 {
        println!("ERR: No entry name given");
        return;
    }

    let entry_name = &args[2];

    if entry_exists(entry_name) {
        let mut path = PathBuf::from("/home/baedert/.config/reactor/");
        path.push(entry_name);
        let result = fs::remove_file(path);
        match result {
            Err(e) => println!("Error: {}", e),
            _ => {}
        }
    } else {
        println!("Entry '{}' does not exist.", entry_name);
    }
}



fn main () {
    let args = env::args();

    let mut get    = false;
    let mut list   = false;
    let mut add    = false;
    let mut rm     = false;

    for arg in args {
        match &arg[..] {
            "get"        => get = true,
            "list"       => list = true,
            "add"        => add = true,
            "rm"         => rm = true,
            _            => {}
        }
    }

    let args :Vec<String> = env::args().collect();

    if list {
        list_entries();
    } else if get {
        get_entry(&args);
    } else if add {
        add_entry(&args);
    } else if rm {
        delete_entry(&args);
    } else {
        panic!("No valid command given");
    }
}
