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
                            println!("-\"[{}] I am not able to travel there right now...\"-", user.name.blue().bold());
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
    for i in &user.event_flags {
        if i == "start" {
            match user.progress {
                /*
                0 => {
                    println!("[{}] ", "Story".blue().bold());
                }
                */
                0 => {
                    println!("[{}] BANG!!!! THUMP. THUMP.", "Story".blue().bold());
                }
                1 => {
                    println!("[{}] {} groaned and opened her eyes, laying in a confused stupor.", "Story".blue().bold(), user.name);
                }
                2 => {
                    println!("[{}] -\"Must be the upstairs neighbors again...\"-", user.name.blue().bold());
                }
                3 => {
                    println!("[{}] {} decided there was no use going back to sleep now. She wanted to give the landlord a peice of her mind, but knew it would be useless. Plus the rent was very cheap.", "story".blue().bold(), user.name);
                }
                4 => {
                    println!("[{}] Getting up {} threw on the least filthy clothes she could find, and looked pitifully into the empty pantry. The mice had not even left a single crumb to eat, and the moldy cheese on the center table was certainly not edible.", "Story".blue().bold(), user.name);
                }
                5 => {
                    println!("[{}] It was time to get more food with the remaining money from the last pay day.", "Story".blue().bold());
                }
                6 => {
                    println!("[{}] {} grabbed the small purse off of the table. [20 {} added to the inventory]", "Story".blue().bold(), user.name, "coins".cyan());
                    inventory_update("coins", 20);
                }
                7 => {
                    println!("[{}] Start by using the {} command to examine your area. This will show you people and things you can interact with using the {} command. It will also show you locations nearby that you can travel to from where you are using the {} command.", "Game".blue().bold(), "examine".green(), "interact".green(), "travel".green());
                    rsl = true;
                }
                8 => {
                    if &user.location == "0002" {
                        println!("[{}] Blinking blearily in the intense mid day sun, {} almost bumps into someone passing by in the street.", "Story".blue().bold(), user.name);
                        ssl = true;
                    } else if flag{
                        println!("[{}] -\"I need to go to the market for food...\"-", user.name.blue().bold());
                        to_progress = false;
                    } else{
                        to_progress = false;
                    }
                }
                9 =>{
                    println!("[{}] Watch it!", "Strange man".blue().bold());
                }
                10 =>{
                    println!("[{}] Sorry!!! Sorry!", user.name.blue().bold());
                }
                11 =>{
                    println!("[{}] The man grunts and walks away with a slouch. He is a full two heads taller than {}, with a rather rotund drinking belly, a balding head, and the strong smell of spice.", "Story".blue().bold(), user.name);
                }
                12 =>{
                    println!("[{}] Spice is not an uncommon thing to smell in this part of the city. A mostly harmless substance but with addiction and withdrawals all the same. It can liven ones social skills and float you on a cloud for hours, but expect an irritable mood with a headache once it wears off...", user.name.blue().bold());
                    rsl = true;
                }
                13 => {
                    if &user.location == "0003" {
                        println!("[{}] walking through the market, {} barely has time to react as a small figure darts out from behind a stall, deftly cuts the string of her purse, and runs off with it. [20 {} removed from the inventory].", "Story".blue().bold(), user.name, "coins".cyan());
                        inventory_update("coins", -20);
                        ssl = true;
                    } else if flag{
                        println!("[{}] -\"I need to go to the market for food...\"-", user.name.blue().bold());
                        to_progress = false;
                    } else{
                        to_progress = false;
                    }
                }
                14 => {
                    println!("[{}] \"Stop!! Thief!!!!!\"", user.name.blue().bold());
                }
                15 => {
                    println!("[{}] But the figure is already gone, weaving their way through the crowd. Some people look around at {}, but nobody saw it actually happen, and nobody really seems to care.", "Story".blue().bold(), user.name);
                }
                16 => {
                    println!("[{}] -\"That was my only money for at least a week, no way is my boss going to pay me early.\"-", user.name.blue().bold());
                }
                17 => {
                    println!("[{}] -\"I would cut off that thiefs hands myself if i could... What am i going to do?\"-", user.name.blue().bold());
                }
                18 => {
                    println!("[{}] {} stands in the middle of the street completely hopeless for a few moments before catching a fragment of conversation:", "Story".blue().bold(), user.name);
                }
                19 => {
                    println!("[{}] \"... Heard the guild has been making good money lately with it's job offers. Some trouble in the area with monsters ...\"", "Woman".blue().bold());
                }
                20 => {
                    println!("[{}] -\"Thats right, the guild. I am going to starve if I don't get any money by tomorrow... Doing a guild job might be the only way to do it, but it could also get me killed...\"-", user.name.blue().bold());
                }
                21 => {
                    println!("[{}] -\"I really dont have any other choice. Maybe i can find one that won't be too difficult?\"-", user.name.blue().bold());
                }
                22 => {
                    println!("[{}]  -\"I need to go to Central Anshanli and then straight to the guild.\"-", user.name.blue().bold());
                    rsl = true;
                }
                _ => {

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
