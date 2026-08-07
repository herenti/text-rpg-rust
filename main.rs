use colored::Colorize;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;
use std::process;
use std::{thread, time};
use enable_ansi_support;

fn save(user: &mut User) {
    let mut file = File::create("data.txt");
    let inventory = user.inventory.join(",");
    let event_flags = user.event_flags.join(",");
    let user_flags = user.user_flags.join(",");
    let info = format!("{}:{}:{}:{}:{}:{}:{}:{}:{}",user.name, inventory, user.level, user.location, event_flags, user.health, user.exp, user_flags, user.progress);
    let info = info.as_bytes();
    file.expect("reason").write_all(info);
}

fn load() -> User {
        let contents = std::fs::read_to_string("data.txt");
        let contents = contents.expect("");
        let contents = contents.trim().split(":");
        let contents = contents.collect::<Vec<&str>>();
        let inventory = contents[1];
        let inventory = inventory.split(",");
        let inventory: Vec<String> = inventory.map(|x| x.trim().to_string()).collect();
        let event_flags = contents[4];
        let event_flags = event_flags.split(",");
        let event_flags: Vec<String> = event_flags.map(|x| x.trim().to_string()).collect();
        let user_flags = contents[7];
        let user_flags = user_flags.split(",");
        let user_flags: Vec<String> = user_flags.map(|x| x.trim().to_string()).collect();
        User {
            name: contents[0].to_string(),
            inventory: inventory,
            level: contents[2].parse().expect("failed to read user level."),
            location: contents[3].to_string(),
            event_flags: event_flags,
            health: contents[5].parse().expect("failed to read user health."),
            exp: contents[6].parse().expect("failed to read user exp."),
            user_flags: user_flags,
            progress: contents[8].parse().expect("failed to read user progress."),
        }
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    println!("Welcome to the game. Type {} to load the saved game or {} to start a new game. Type {} to quit.\r\n", "load".green(), "new".green(), "quit".green());
    let mut beginning = true;
    loop {
        print!("[{}]: ", "Command".blue().bold());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim();
        if beginning == true {
            commands(&input, "gamestart");
        } else {
            commands(&input, "");
        }
        beginning = false;
    }

}

fn commands(input: &str, flag: &str) {
    let clear = std::process::Command::new("clear").status().unwrap();
    if flag == "gamestart" {
        match input {
            "new" => {
                print!("\r\n[{}]: ", "Enter name".blue().bold());
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
                let input = input.trim();
                let mut user = User::new(&input);
                clear;
                println!("Welcome to the game {}! This game autosaves. Press enter with no command to continue story content.\r\n", input.cyan());
                save(&mut user);
            }
            "load" => {
                let mut user = load();
                clear;
                println!("Loading user {}... Press enter with no command to continue story content,", user.name);
            }
            _ =>{
                clear;
                println!("[{}] Incorrect command usage. type {}, {}, or {}.", "ERROR".red().bold(), "new".green(), "load".green(), "quit".green());
                print!("[{}]: ", "Command".blue().bold());
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input.");
                let input = input.trim();
                commands(&input, "gamestart");
            }
        }
    } else {
        let mut user = load();
        match input {
            "" => {
                clear;
                events();
            }
            _ => {
                clear;
                println!("[{}] Incorrect command usage.", "ERROR".red().bold());
            }

        }
    }
}

fn events() {
    let mut user = load();
    let mut to_progress = false;
    for i in &user.event_flags {
        if i == "start" {
            match user.progress {
                0 => {
                    println!("BANG!!!");
                    to_progress = true;

                }
                1 => {
                    println!("{} wakes up quickly and looks around... After a few moments of confusion, {} remembers that they have the most annoying neighbors living upstairs.", user.name, user.name);
                    to_progress = true;

                }
                2 => {
                    println!("{} lays in their lumpy, sagging bed for a few more moments before slowly getting up and getting dressed. \"I need to go to the store to get food today.\" {} mutters under their breath. The moldy cheese sitting on the table was probably not edible, and the mice had not even left crumbs to eat in the pantry. {} picks up the coins on table... [10 {} added to the inventory]", user.name, user.name, user.name, "coins".cyan());
                    inventory_update("coins", 10);
                    to_progress = true;
                }
                _ => {

                }
            }
        }
    }
    let mut user = load();
    if to_progress == true {
        user.progress += 1;
    } else {

    }
    save(&mut user);

}

fn inventory_update(item: &str, _amount: i32) {
    let mut user = load();
    let mut updated_amount = 0;
    let mut index = 0;
    let mut _name = "".to_string();
    let mut op = true;
    for i in &user.inventory {
        let data = i.split("-");
        let data: Vec<String> = data.map(|x| x.trim().to_string()).collect();
        if data.len() > 1 {
            let name = &data[0];
            let mut amount = data[1].parse().unwrap();
            if name == item {
                amount += _amount;
                let amount = if amount <=0 {
                    0
                } else {
                    amount
                };
                index += user.inventory.iter().position(|x| *&x == i).unwrap();
                updated_amount += amount;
                _name = name.to_string();

            }
        } else {
            op = false;
        }

    }
    if op == true {
        if updated_amount == 0 {
            user.inventory.remove(index);
        } else {
            user.inventory.remove(index);
            user.inventory.push(format!("{}-{}", _name, updated_amount.to_string()));
        }
    } else {
        user.inventory.push(format!("{}-{}", item, _amount.to_string()));
    }
    save(&mut user);
}

struct User {
    name: String,
    inventory: Vec<String>,
    level: i32,
    location: String,
    event_flags: Vec<String>,
    health: i32,
    exp: i32,
    user_flags: Vec<String>,
    progress: i32,
}

impl User {
    fn new(username: &str) -> User {
        User {
            name: username.to_string(),
            inventory: vec![],
            level: 1,
            location: "home".to_string(),
            event_flags: vec!["start".to_string()],
            health: 100,
            exp: 0,
            user_flags: vec!["no_travel".to_string()],
            progress: 0,
        }
    }
}

struct Objects {
    items: HashMap<String, i32>, // value
    potions: HashMap<String, (String, i32, i32)>, // (type, value, statamount)
    weapons: HashMap<String, (i32, i32)>, // (value, damage)
    scrolls: HashMap<String, (String, i32, i32)>, //(type, value, damage)
}

impl Objects {
    fn new() -> Objects {
        let mut objects = Objects {
            items: HashMap::new(),
            potions: HashMap::new(),
            weapons: HashMap::new(),
            scrolls: HashMap::new(),
        };

        objects.items.insert("coins".to_string(), 1);
        objects.potions.insert("minor health potion".to_string(), ("potion".to_string(), 25, 50));
        objects.weapons.insert("rusty dagger".to_string(), (10, 15));
        objects.weapons.insert("rusty sword".to_string(), (30, 20));
        objects.scrolls.insert("firespark".to_string(), ("fire".to_string(), 50, 50));
        objects
    }
}



