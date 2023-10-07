use fxhash::FxHashMap;

use crate::advent_of_code;

#[allow(dead_code)]
pub fn run() {
    let input = advent_of_code::read_input_lines(12);
    let tile_map = parse_map(&input);
    advent_of_code::answer(1, Some(504), part1(&tile_map));
    advent_of_code::answer(2, Some(500), part2(&tile_map));
}

fn part1(tile_map: &TileMap) -> i32 {
    let mut distance = 1;
    let mut moves = vec![(tile_map.end.0, tile_map.end.1, 0)];

    loop {
        let mut valid = vec![];
        for i in 0..moves.len() {
            valid.append(&mut get_valid_tiles((moves[i].0, moves[i].1), &tile_map.tiles, tile_map.height, tile_map.width));
        }
        
        for tile in &valid[..] {
            if !moves.iter().any(|&x| tile.0 == x.0 && tile.1 == x.1)
            {
                moves.push((tile.0, tile.1, distance));
            }
        }

        if valid.iter().any(|&x| tile_map.start.0 == x.0 && tile_map.start.1 == x.1)
        {
            break;
        }

        distance += 1;
    }

    distance
}

fn part2(tile_map: &TileMap) -> i32 {
    let mut distance = 1;
    let mut moves = vec![(tile_map.end.0, tile_map.end.1, 0)];

    loop {
        let mut valid = vec![];
        for i in 0..moves.len() {
            valid.append(&mut get_valid_tiles((moves[i].0, moves[i].1), &tile_map.tiles, tile_map.height, tile_map.width));
        }
        
        for tile in &valid[..] {
            if !moves.iter().any(|&x| tile.0 == x.0 && tile.1 == x.1)
            {
                moves.push((tile.0, tile.1, distance));
            }
        }

        if valid.iter().any(|x| tile_map.tiles.get(x).unwrap() == &'a')
        { 
            break;
        }

        distance += 1;
    }

    distance
}

fn parse_map(input: &[String]) -> TileMap
{
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut tiles: FxHashMap<(i32, i32), char> = FxHashMap::default();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let char = input[y].chars().nth(x).unwrap();
            if char == 'S' {
                start = (x as i32, y as i32);
            }

            if char == 'E' {
                end = (x as i32, y as i32);
            }

            tiles.insert((x as i32, y as i32), input[y].chars().nth(x).unwrap());
        }
    }

    TileMap { tiles, start, end, height: input.len() as i32, width: input[0].len() as i32 }
}

fn get_valid_tiles(tile: (i32, i32), tiles: &FxHashMap<(i32, i32), char>, height: i32, width: i32) -> Vec<(i32, i32)> {
    let mut valid = vec![];

    let tile_height = tiles.get(&tile).unwrap();

    // Left
    if tile.0 - 1 >= 0 && can_move(tile_height, tiles.get(&(tile.0 - 1, tile.1)).unwrap()) {
        valid.push((tile.0 - 1, tile.1));
    }

    // Up
    if tile.1 - 1 >= 0 && can_move(tile_height, tiles.get(&(tile.0, tile.1 - 1)).unwrap()) {
        valid.push((tile.0, tile.1 - 1));
    }

    // Right
    if tile.0 + 1 < width && can_move(tile_height, tiles.get(&(tile.0 + 1, tile.1)).unwrap()) {
        valid.push((tile.0 + 1, tile.1));
    }

    // Down
    if tile.1 + 1 < height && can_move(tile_height, tiles.get(&(tile.0, tile.1 + 1)).unwrap()) {
        valid.push((tile.0, tile.1 + 1));
    }

    valid
}

fn can_move(height1: &char, height2: &char) -> bool
{
    let h1 = match height1.to_owned() as i32 {
        83 => 97,
        69 => 122,
        x => x
    };

    let h2 = match height2.to_owned() as i32 {
        83 => 97,
        69 => 122,
        x => x
    };

    h1 - h2 <= 1
}

struct TileMap {
    tiles: FxHashMap<(i32, i32), char>,
    start: (i32, i32),
    end: (i32, i32),
    height: i32,
    width: i32,
}
