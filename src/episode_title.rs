use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
struct Monster {
    s: &'static str,
    p: &'static str,
}

fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn adjective() -> String {
    let lines = read_file_lines("data/adjectives.txt").unwrap();
    let mut rng = rand::thread_rng();
    let word = lines.choose(&mut rng).unwrap();
    word.to_string()
}

fn noun() -> String {
    let lines = read_file_lines("data/nouns.txt").unwrap();
    let mut rng = rand::thread_rng();
    let word = lines.choose(&mut rng).unwrap();
    word.to_string()
}

fn place() -> String {
    let lines = read_file_lines("data/places.txt").unwrap();
    let mut rng = rand::thread_rng();
    let word = lines.choose(&mut rng).unwrap();
    word.to_string()
}

fn verb() -> String {
    let lines = read_file_lines("data/verbs.txt").unwrap();
    let mut rng = rand::thread_rng();
    let word = lines.choose(&mut rng).unwrap();
    word.to_string()
}

fn planet() -> String {
    let planets = vec![
    "Androzani Major",
    "Skaro",
    "Earth",
    "Gallifrey",
    "Telos",
    "Peladon",
    "Mondas",
    "Spiridon",
    "Balhoon",
    "Deva Loka",
    "Exillon",
    "Hell",
    "Karn",
    "Logopolis",
    "Mars",
    "Metebelis Three",
    "Midnight",
    "Poosh",
    "Ravolox",
    "Sontar",
    "Traken",
    "Utopia",
    "Zygor",
    ];
    let mut rng = rand::thread_rng();
    let planet = planets.choose(&mut rng).unwrap();
    planet.to_string()
}

fn get_monster() -> Monster {
    let mut monsters: Vec<Monster> = vec![
        Monster { s: "Dalek", p: "Daleks" },
        Monster { s: "Cyberman", p: "Cybermen" },
        Monster { s: "Sea Devil", p: "Sea Devils" },
        Monster { s: "Angel", p: "Angels" },
        Monster { s: "Silurian", p: "Silurians" },
        Monster { s: "Rutan", p: "Rutans" },
        Monster { s: "Bandrill", p: "Bandrills" },
        Monster { s: "Vashta Nerada", p: "Vashta Nerada" },
        Monster { s: "Dinosaur", p: "Dinosaurs" },
        Monster { s: "Planet", p: "Planets" },
        Monster { s: "Doctor", p: "Doctors" },
        Monster { s: "Chumbly", p: "Chumblies" },
        Monster { s: "Terileptil", p: "Terileptils" },
        Monster { s: "Judoon", p: "Judoon" },
        Monster { s: "Zygon", p: "Zygons" },
        Monster { s: "Morbius", p: "Time Lords" },
        Monster { s: "Omega", p: "CIA" },
        Monster { s: "Axon", p: "Axons" },
        Monster { s: "Wirrn", p: "Wirrn" },
        Monster { s: "Ice Warrior", p: "Ice Warriors" },
        Monster { s: "Yeti", p: "Yeti" },
        Monster { s: "Auton", p: "Autons" },
        Monster { s: "Mara", p: "Mara" },
        Monster { s: "Haemovore", p: "Haemovores" },
        Monster { s: "Mechanoid", p: "Mechanoids" },
        Monster { s: "Zarbi", p: "Zarbi" },
        Monster { s: "Macra", p: "Macra" },
        Monster { s: "Kroton", p: "Krotons" },
        Monster { s: "Quark", p: "Quarks" },
        Monster { s: "Ogron", p: "Ogrons" },
        Monster { s: "Drashig", p: "Drashigs" },
        Monster { s: "Draconian", p: "Draconians" },
        Monster { s: "Robot", p: "Robots" },
        Monster { s: "Movellan", p: "Movellans" },
        Monster { s: "Vampire", p: "Vampires" },
        Monster { s: "Dragon", p: "Dragons" },
        Monster { s: "Monster", p: "Monsters" },
        Monster { s: "Demon", p: "Demons" },
        Monster { s: "Sycorax", p: "Sycorax" },
        Monster { s: "Minotaur", p: "Minotaurs" },
        Monster { s: "Adipose", p: "Adipose" },
        Monster { s: "Stenza", p: "Stenza" },
        Monster { s: "Skithra", p: "Skithra" },
        Monster { s: "Witch", p: "Witches" },
    ];

    // Adding Daleks 6 times
    monsters.extend(vec![Monster { s: "Dalek", p: "Daleks" }; 6]);

    // Adding Cybermen 5 times
    monsters.extend(vec![Monster { s: "Cyberman", p: "Cybermen" }; 5]);

    // Adding Master 4 times
    monsters.extend(vec![Monster { s: "Master", p: "Master" }; 4]);

    // Adding Sontarans 3 times
    monsters.extend(vec![Monster { s: "Sontaran", p: "Sontarans" }; 3]);
    let mut rng = rand::thread_rng();
    let my_monster = monsters.choose(&mut rng).unwrap();
    return my_monster.clone()
}

pub fn get_episode_title() -> String {
    let mut rng = rand::thread_rng();
    let mut monster= get_monster();
    let mut monster_two = get_monster();

    while monster.s == monster_two.s {
        monster_two = get_monster();
    }

    let do_bad_to_options = vec!["in", "invade", "destroy", "take", "oppress", "dominate", "do"];
    let do_bad_to = do_bad_to_options.choose(&mut rng).unwrap();

    let mut sentences = vec![
        format!("{} of the {}", noun(), monster.p),
        format!("{} for the {}", noun(), monster.p),
        format!("{} of the {}", noun(), monster.p),
        format!("The {} {}", monster.p, verb()),
        format!("The {} {}", monster.s, noun()),
        format!("{} {} {}", monster.p, do_bad_to, place()),
        format!("{} {} {}", monster.p, verb(), place()),
        format!("{} the {}", verb(), monster.s),
        format!("{} of the {}", noun(), monster.p),
        format!("{} vs {}", monster.p, monster_two.p),
        format!("The {} {}", adjective(), noun()),
        format!("The {} {}", adjective(), monster.s),
        format!("The {} in the {}", monster.s, noun()),
        format!("{} on {}", monster.p, planet()),
    ];

    // 'of the' is the most common name so add more of them into the mix
    sentences.extend(vec![format!("{} of the {}", noun(), monster.p); 4]);
    sentences.choose(&mut rng).unwrap().clone()

}