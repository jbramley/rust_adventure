use rand::prelude::IndexedRandom;
use std::array;
use std::collections::{HashSet, VecDeque};

pub fn build_map() -> [Room; 16] {
    let mut rooms: [Room; 16] = array::from_fn(|_| Room::default());
    fully_connect_map(&mut rooms);
    add_loops_to_map(&mut rooms);
    rooms
}

fn add_loops_to_map(rooms: &mut [Room; 16]) {
    loop {
        let sparse_rooms = sparse_rooms(rooms);
        if sparse_rooms.len() < 2 {
            break;
        }
        let room_0 = sparse_rooms[0];
        let room_1 = sparse_rooms[1..]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        for direction in Direction::VALUES {
            if rooms[room_0].doors[direction as usize] != None {
                continue;
            }
            if rooms[room_1].doors[direction.opposite() as usize] != None {
                continue;
            }
            rooms[room_0].doors[direction as usize] = Some(room_1);
            rooms[room_1].doors[direction.opposite() as usize] = Some(room_0);
        }
    }
}

fn fully_connect_map(rooms: &mut [Room; 16]) {
    loop {
        let (seen_rooms, unseen_rooms) = split_rooms(rooms);
        if unseen_rooms.is_empty() {
            break;
        }
        let room_0 = seen_rooms.choose(&mut rand::thread_rng()).unwrap().clone();
        let room_1 = unseen_rooms
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let room_0_dir = Direction::VALUES
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        let room_1_dir = room_0_dir.opposite() as usize;
        let room_0_dir = room_0_dir as usize;
        if rooms[room_0].doors[room_0_dir] != None || rooms[room_1].doors[room_1_dir] != None {
            continue;
        }
        rooms[room_0].doors[room_0_dir] = Some(room_1);
        rooms[room_1].doors[room_1_dir] = Some(room_0);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl Direction {
    const VALUES: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<String> for Direction {
    fn into(self) -> String {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        }
        .to_string()
    }
}

impl From<String> for Direction {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "north" => Direction::North,
            "south" => Direction::South,
            "east" => Direction::East,
            "west" => Direction::West,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
pub struct Room {
    pub doors: [Option<usize>; 4],
}

impl Room {
    pub fn get_directions(&self) -> Vec<String> {
        Direction::VALUES
            .iter()
            .filter(|d| self.doors[**d as usize] != None)
            .map(|d| (*d).into())
            .collect()
    }
}

impl Default for Room {
    fn default() -> Self {
        Room { doors: [None; 4] }
    }
}

fn split_rooms(rooms: &[Room]) -> (Vec<usize>, Vec<usize>) {
    let mut room_queue: VecDeque<usize> = VecDeque::with_capacity(rooms.len());
    let mut seen_rooms: HashSet<usize> = HashSet::with_capacity(rooms.len());
    let mut unseen_rooms: HashSet<usize> = HashSet::from_iter(0..rooms.len());
    room_queue.push_back(0);
    loop {
        if let Some(room) = room_queue.pop_front() {
            seen_rooms.insert(room);
            unseen_rooms.remove(&room);
            for direction in Direction::VALUES {
                if rooms[room].doors[direction as usize] != None
                    && !seen_rooms.contains(&rooms[room].doors[direction as usize].unwrap())
                {
                    room_queue.push_back(rooms[room].doors[direction as usize].unwrap());
                }
            }
        } else {
            break;
        }
    }
    (
        seen_rooms.into_iter().collect(),
        unseen_rooms.into_iter().collect(),
    )
}

fn sparse_rooms(rooms: &[Room]) -> Vec<usize> {
    let mut room_queue: VecDeque<usize> = VecDeque::with_capacity(rooms.len());
    let mut seen_rooms: HashSet<usize> = HashSet::with_capacity(rooms.len());
    let mut sparse_rooms: Vec<usize> = Vec::with_capacity(rooms.len());
    room_queue.push_back(0);
    loop {
        if let Some(room) = room_queue.pop_front() {
            seen_rooms.insert(room);
            if rooms[room]
                .doors
                .iter()
                .filter(|&&d| d == None)
                .collect::<Vec<_>>()
                .len()
                >= 3
            {
                sparse_rooms.push(room);
            }
            for direction in Direction::VALUES {
                if rooms[room].doors[direction as usize] != None
                    && !seen_rooms.contains(&rooms[room].doors[direction as usize].unwrap())
                {
                    room_queue.push_back(rooms[room].doors[direction as usize].unwrap());
                }
            }
        } else {
            break;
        }
    }
    sparse_rooms
}
