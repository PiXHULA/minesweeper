use std::ops::{Deref, DerefMut};
use crate::components::Coordinates;
use rand::{thread_rng, Rng};
use crate::resources::tile::Tile;
//https://github.com/leonidv/bevy-minesweeper-tutorial/commit/45e742b4cab3aab62bb263cb3d366ae9ce006c45

/// Delta coordinates for all 8 square neighbors
/*
*--------*-------*-------*
| -1, 1  | 0, 1  | 1, 1  |
|--------|-------|-------|
| -1, 0  | tile  | 1, 0  |
|--------|-------|-------|
| -1, -1 | 0, -1 | 1, -1 |
*--------*-------*-------*
*/
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1), (0, -1), (1, -1),     //bottom left, bottom, bottom right
    (-1,  0),          (1,  0),     //Left, Right
    (-1,  1), (0,  1), (1,  1),      //top left, top, top right
];

//Base tile map
#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    // Generates an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width)
                .into_iter()
                .map(|_| Tile::Empty)
                .collect())
            .collect();
        Self {
            bomb_count: 0,
            height,
            width,
            map
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({},{}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width + 1))
            .into_iter()
            .map(|_| '-')
            .collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.coord_x >= self.width || coordinates.coord_y >=self.height {
            return false;
        }
        self.map[coordinates.coord_y as usize][coordinates.coord_x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }
        let res = self
            .safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count();
        res as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();
        // Place bombs
        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[y][x] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }
        /*
            The row number is a y coordinate, and the column number is an x coordinate,
            Confusing since normally you write [row (y)][col (x)]
            but we also usually write (x,y).
        */
        //Place bomb neighbors
        for row in 0..self.height {
            for column in 0..self.width {
                let coords = Coordinates { coord_x: column, coord_y: row, };
                if self.is_bomb_at(coords) {
                    continue;
                }
                let number_of_bombs = self.bomb_count_at(coords);
                if number_of_bombs == 0 {
                    continue;
                }
                let tile = &mut self[row as usize][column as usize];
                *tile = Tile::BombNeighbor(number_of_bombs);
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}