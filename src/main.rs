use rand::prelude::*;


fn main() {
    let kart: Map = Map::create_map(Map, 4, 4);
    println!("{:?}", kart.matrix)
}
#[derive(Debug)]
struct Tile {
    tile_type: u8, // 0 = None, 1 = Bomb
    close_bombs: u8,
    position: (i32, i32)
}

struct Map {
    matrix: Vec<Vec<Tile>>
}

fn bomb_chooser(chance_to_place: u32) -> u8 {
    let mut rng = rand::thread_rng();


    // Makes a random number between 0 and 1
    let mut rand_num: f64 = rng.gen();
    rand_num = rand_num * 100 as f64;
    
    
    println!("[bomb_chooser] Random number {} | Presentage {} | Bomb is placed here? {}", (rand_num), chance_to_place, ((rand_num) > chance_to_place.into()));
    if rand_num > chance_to_place.into() {
        return 0
    } else {
        return 1
    }
    
}

impl Map {
    fn create_map(&self, sizex: i32, sizey: i32) -> Map {
        let mut map: Vec<Vec<Tile>> = vec![];
        for x in 0..sizex {
            let mut list: Vec<Tile> = vec![];
            for y in 0..sizey {
                list.insert(y.try_into().unwrap(), Tile { tile_type: bomb_chooser(50), close_bombs: 0, position: (x,y) })
            }
            map.push(list)
        
        }
        return Map { matrix: map };
    }
    fn print_map(self) {
        let sizex:usize = self.matrix.len();
        let sizey: usize = match self.matrix.get(0) {
            Some(tile) => {
                tile.len()
            },
            None => {
                panic!("Uhm uhh... size not sizey enough");
            },
        };
        for x in 0..sizex {
            for y in 0..sizey {

            }
        }
    }
}
/*
⌜ ⁀ ⌝
|    |
⌞ ‿ ⌟


*/

/*

◜◝◟◞⌜⌝⌞⌟◻⁋◉

*/