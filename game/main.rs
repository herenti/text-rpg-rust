use colored::Colorize;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;
use std::process;
use std::{thread, time};
use enable_ansi_support;

/*
 * - INCOMPLETE.
 * - Tested to work only on linux.
 * - TODO:
 * - dexterity to check for traps. mimics, door traps. fails or succeeds on chance.
 * - speech for better prices.
 * - split sections into modules.
 * - add interact and examine commands.
 * - add npc struct.
 * - make inventory hashmap instead of vec?
 * - later add battle events and commands.
*/

fn save(user: &mut User) {
    let mut file = File::create("data.txt");
    user.inventory.retain(|x| !x.is_empty());
    let inventory = user.inventory.join(",");
    user.event_flags.retain(|x| !x.is_empty());
    let event_flags = user.event_flags.join(",");
    user.user_flags.retain(|x| !x.is_empty());
    let user_flags = user.user_flags.join(",");
    let info = format!("{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",user.name, inventory, user.level, user.location, event_flags, user.health, user.exp, user_flags, user.progress, user.speech, user.dex, user.strength);
    let info = info.as_bytes();
    file.expect("reason").write_all(info);
}

fn load_story() -> Vec<Vec<String>> {
    let file_check = File::open("story");
    if file_check.is_ok() {
        let contents = std::fs::read_to_string("story");
        let contents = contents.expect("");
        let contents = contents.lines();
        let contents: Vec<String> = contents.map(|x| x.to_string()).collect();
        let mut library = vec![];
        for i in contents {
            let _contents = i.trim().split("#");
            let _contents: Vec<String> = _contents.map(|x| x.to_string()).collect();
            library.push(_contents)
        }
        library
    } else {
        println!("ERROR: story file not found.");
        vec![]
    }
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
            speech: contents[9].parse().expect("failed to read user speech."),
            dex: contents[10].parse().expect("failed to read user dexs."),
            strength: contents[11].parse().expect("failed to read user strength."),
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
    let input = input.split(" ");
    let input = input.collect::<Vec<&str>>();
    let args = if input.len() > 1 {
        &input[1..].join(" ")
    } else {
        ""
    };
    let input = input[0];
    let clear = std::process::Command::new("clear").status().unwrap();
    if flag == "gamestart" {
        match input {
            "new" => {
                print!("[{}]: ", "Enter name".blue().bold());
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
                let input = input.trim();
                let mut user = User::new(&input);
                clear;
                println!("Welcome to the game {}! This game autosaves. Press enter with no command to continue the story content if available.\r\n", input.cyan());
                save(&mut user);
            }
            "load" => {
                let file_check = File::open("data.txt");
                if file_check.is_ok() {
                    let mut user = load();
                    clear;
                    println!("Loading user {}... Press enter with no command to continue story content if available,", user.name);
                } else {
                    println!("No save game present. Please use the {} command.", "new".green());
                    print!("[{}]: ", "Command".blue().bold());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input.");
                    let input = input.trim();
                    commands(&input, "gamestart");
                }
            }
            "quit" => {
                clear;
                println!("Exiting the game...");
                std::process::exit(1);
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
                events(true);
            }
            "travel" => {
                if user.user_flags.contains(&"story_lock".to_string()) {
                    println!("[{}] The {} command is not available right now.", "Game".blue().bold(), "travel".green())
                } else {
                    let id = &travel_lookup(args);
                    if id != "no" {
                        if travel_check(id){
                            travel(id);
                            println!("[{}] traveled to {}.\r\n", "Game".blue().bold(), args);
                            events(false);
                        } else {
                            println!("[{}] -\"I am not able to travel there right now...\"-", user.name.blue().bold());
                        }
                    } else {
                        println!("[{}] That location is not available to travel to from here.", "Game".blue().bold());
                    }
                }
            }
            "quit" => {
                clear;
                println!("Exiting the game...");
                std::process::exit(1);
            }
            _ => {
                clear;
                println!("[{}] Incorrect command usage.", "ERROR".red().bold());
            }
        }
    }
}

fn events(flag: bool) {
    let mut user = load();
    let mut to_progress = true;
    let mut rsl = false;
    let mut ssl = false;
    if user.event_flags.contains(&"start".to_string()) {
        if user.progress == 6 {
            inventory_update("coins", 20);
        } else if user.progress == 7 {
            rsl = true;
        } else if user.progress == 8 {
            if &user.location == "0002" {
                ssl = true;
            } else if flag{
                println!("[{}] -\"I need to go to the market for food...\"-", user.name.blue().bold());
                to_progress = false;
            } else{
                to_progress = false;
            }
        } else if user.progress == 12{
            rsl = true;
        } else if user.progress == 13{
            if &user.location == "0003" {
                inventory_update("coins", -20);
                ssl = true;
            } else if flag{
                println!("[{}] -\"I need to go to the market for food...\"-", user.name.blue().bold());
                to_progress = false;
            } else{
                to_progress = false;
            }
        } else if user.progress == 22 {
            rsl = true;
        } else{

        }
        let library = load_story();
        if to_progress {
            for i in library {
                if user.progress == i[0].parse().unwrap() {
                    println!("[{}] {}", i[2].blue().bold(), i[3])
                }
            }
        }
    }
    let mut user = load();
    if to_progress {
        user.progress += 1;
    } else {
    }
    if rsl{
        let index = user.user_flags.iter().position(|x| *&x == "story_lock").unwrap();
        user.user_flags.remove(index);
    } else {
    }
    if ssl {
        user.user_flags.push("story_lock".to_string());
    } else {
    }
    save(&mut user);
}

fn inventory_update(item: &str, _amount: i32) { //having inventory as hashmap would fix this mess?
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

fn travel(id: &str) {
    let mut user = load();
    user.location = id.to_string();
    save(&mut user);
}

fn travel_lookup(place: &str) -> String {
    let mut user = load();
    let locations = Locations::new();
    let nearby = &locations.library[&user.location]["nearby"];
    let mut id = "".to_string();
    for (key, value) in &locations.library {
        if nearby.contains(&key){
            if &locations.library[key]["name"][0] == place {
                return key.clone();
            } else {
            }
        } else {
        }
    }
    return "no".to_string();
}

fn travel_check(id: &str) -> bool {
    let user = load();
    if user.event_flags.contains(&"start".to_string()) {
        if id == "0004" {
            if user.progress < 22 {
                return false;
            } else {
            }
        } else {
        }
    } else {
    }
    return true;
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
    speech: f32,
    dex: f32,
    strength: f32,
}

impl User {
    fn new(username: &str) -> User {
        User {
            name: username.to_string(),
            inventory: vec![],
            level: 1,
            location: "0000".to_string(),
            event_flags: vec!["start".to_string()],
            health: 100,
            exp: 0,
            user_flags: vec!["story_lock".to_string(), "game_lock".to_string()],
            progress: 0,
            speech: 1.0,
            dex: 1.0,
            strength: 1.0,
        }
    }
}

struct Objects { //rename Items
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

struct Locations {
    library: HashMap<String, HashMap<String, Vec<String>>>,
}

impl Locations {
    fn new() -> Locations {
        let mut locations = Locations {
            library: HashMap::new(),
        };
        let _list = vec![
            //vec!["id", "name", "description", "nearby", "items", "npc"],
            vec!["0000", "home", "A run down and kind of dirty single room apartment. There is a single Table in the center of the room with two chairs, a stove for cooking, and a few cupboards and countertops. There is not much else in this room besides a dresser for clothes and moldy cheese sitting on the table.", "0001", "modly cheese", ""],
            vec!["0001", "apartment building", "An old and derepit apartment building. Inside it can be navigated by a rather narrow hallway that is barely lit. People seemed to use the hallway as extra storage, even though there was no room for it.", "0000;0002", "wood crate", ""],
            vec!["0002", "dubari district", "One of the most poor districts in Anshanli; however it is still more safe than the pleasure district due to the volunteer work done by temple priests and nuns.", "0001;0003;0004", "", "dice player;suspicious stranger"],
            vec!["0003", "dubari market", "A bustling market filled with all sorts of people. The Dubari market in particular seemes to attract a colorful crowd; for better or worse. Among the delicious scents of food, one can catch a whiff of rotten fish, and something that reminds one of a dead dog's asshole.", "0002", "", "food seller;apothecary;shady merchant"],
            vec!["0004", "central anshanli", "The clean and prosperous center of the city. Home to many of the cities buisnesses and it's ornate temple to the Divine Mora. Patrolled often by the city guard, this is not a place where the wealthy feel unsafe.", "0002;0005;0006", "", ""],
            vec!["0005", "anshanli guild", "A large, upscale multi-story building. The spacious room inside the imposing front doors also functions as a tavern. Often a lively place, filled with a tough looking crowd. A board hosting jobs for the guild members sits on the far wall.", "0004", "", "barkeep;job board;guild official"],
            vec!["0006", "temple of mora", "Just as impressive on the inside as the outside. Ornate carvings cover the outisde of the building. Some are beautiful and conforting, while others show scenes of distruction and chaos. On the candle-lit inside, a vaulted ceiling is painted with intricate patterns. A few priests and nuns can be seen doing various duities.", "0004", "shrine", "head priest;head nun"],
        ];
        for i in _list {
            let id = &i[0];
            let name = &i[1];
            let description = &i[2];
            let nearby: Vec<String> = i[3].split(';')
            .map(String::from)
            .collect();
            let items: Vec<String> = i[4].split(';')
            .map(String::from)
            .collect();
            let npc: Vec<String> = i[5].split(';')
            .map(String::from)
            .collect();
            let mut place = HashMap::new();
            place.insert("name".to_string(), vec![name.to_string()]);
            place.insert("description".to_string(), vec![description.to_string()]);
            place.insert("nearby".to_string(), nearby);
            place.insert("items".to_string(), items);
            place.insert("npc".to_string(), npc);
            locations.library.insert(id.to_string(), place);
        }
        locations
    }
}
