use crate::descriptions::DESCRIPTIONS;
use crate::goat::Goat;
use crate::map::Direction;
use rand::Rng;
use std::env;

mod descriptions;
mod goat;
mod map;

fn main() {
    let debug: bool = env::var("DEBUG").is_ok();
    let rooms = map::build_map();
    let mut current_room: usize = 0;
    let tulip_room = rand::thread_rng().gen_range(0..rooms.len());
    let num_goats = rand::thread_rng().gen_range(2..5);
    let mut goats = Vec::from_iter(
        (0..num_goats)
            .map(|_| rand::thread_rng().gen_range(0..rooms.len()))
            .map(|r| Goat { room: r }),
    );

    /// Banner generated at http://www.patorjk.com/software/taag/#p=display&f=Flower%20Power&t=%20H%20u%20n%20t%20%0A%20%20T%20h%20e%20%0AT%20u%20l%20i%20p%0A
    let banner = include_str!("banner.txt");
    println!("{}", banner);
    let _ready = rprompt::prompt_reply("Ready to play? ").unwrap();

    loop {
        if debug {
            println!("Field {}", current_room);
        }
        println!("{}", DESCRIPTIONS[current_room]);
        if current_room == tulip_room {
            println!("You found a tulip!");
            break;
        }
        if goats.iter().any(|g| g.room == current_room) {
            println!("You see a goat!");
        } else {
            let adjacent_rooms: Vec<Option<usize>> = rooms[current_room]
                .doors
                .iter()
                .copied()
                .filter(|d| d.is_some())
                .collect();
            if goats.iter().any(|g| adjacent_rooms.contains(&Some(g.room))) {
                println!("You hear goats nearby.");
            }
        }

        println!(
            "You can go {}",
            rooms[current_room].get_directions().join(", ")
        );

        let cmd = rprompt::prompt_reply("What now? ").unwrap();
        match cmd.to_lowercase().as_str() {
            "quit" => break,
            "north" | "south" | "east" | "west" => {
                let direction = Direction::from(cmd);
                if let Some(new_room) = rooms[current_room].doors[direction as usize] {
                    current_room = new_room;
                } else {
                    println!("You can't go that way.");
                }
            }
            _ => println!("I don't understand."),
        }
        goats
            .iter_mut()
            .map(|mut g| g.maybe_move(&rooms[g.room].doors))
            .filter_m
            .filter(|r| r.is_some());
    }
}
