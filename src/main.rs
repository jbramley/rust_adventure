use crate::descriptions::DESCRIPTIONS;
use crate::map::Direction;
use rand::Rng;

mod descriptions;
mod map;

fn main() {
    let rooms = map::build_map();
    let mut current_room: usize = 0;
    let tulip_room = rand::thread_rng().gen_range(0..rooms.len());
    let num_goats = rand::thread_rng().gen_range(2..5);
    let goats =
        Vec::from_iter((0..num_goats).map(|_| rand::thread_rng().gen_range(0..rooms.len())));
    loop {
        println!("Field {}", current_room);
        println!("{}", DESCRIPTIONS[current_room]);
        if current_room == tulip_room {
            println!("You found a tulip!");
            break;
        }
        if goats.contains(&current_room) {
            println!("You see a goat!");
        } else if rooms[current_room]
            .doors
            .iter()
            .any(|r| r.is_some() && goats.contains(&r.unwrap()))
        {
            println!("You hear goats nearby.");
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
    }
}
