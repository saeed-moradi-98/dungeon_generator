use crossterm::{
    cursor, execute, style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use rand::Rng;
use std::io::{self, Write, Read};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Floor,
}

struct Dungeon {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Dungeon {
    fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::Wall; width]; height];
        Self { width, height, tiles }
    }

    fn initialize_random(&mut self, wall_probability: f64) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                self.tiles[y][x] = if rng.gen::<f64>() < wall_probability {
                    Tile::Wall
                } else {
                    Tile::Floor
                };
            }
        }
    }

    fn count_wall_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                // Treat out-of-bounds as walls
                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    count += 1;
                } else if self.tiles[ny as usize][nx as usize] == Tile::Wall {
                    count += 1;
                }
            }
        }
        count
    }

    fn simulate_step(&mut self) -> bool {
        let mut new_tiles = self.tiles.clone();
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let wall_count = self.count_wall_neighbors(x, y);
                
                // Cellular automata rules for cave generation
                let new_tile = if wall_count > 4 {
                    Tile::Wall
                } else if wall_count < 4 {
                    Tile::Floor
                } else {
                    self.tiles[y][x]
                };

                if new_tile != self.tiles[y][x] {
                    changed = true;
                }
                new_tiles[y][x] = new_tile;
            }
        }

        self.tiles = new_tiles;
        changed
    }

    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        
        execute!(stdout, cursor::MoveTo(0, 0))?;
        
        for row in &self.tiles {
            for &tile in row {
                match tile {
                    Tile::Wall => {
                        execute!(
                            stdout,
                            SetForegroundColor(Color::DarkGrey),
                            Print("█"),
                            ResetColor
                        )?;
                    }
                    Tile::Floor => {
                        execute!(
                            stdout,
                            SetForegroundColor(Color::Yellow),
                            Print("·"),
                            ResetColor
                        )?;
                    }
                }
            }
            execute!(stdout, Print("\n"))?;
        }
        
        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    // Setup terminal
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    // Create dungeon
    let mut dungeon = Dungeon::new(80, 30);
    dungeon.initialize_random(0.45);

    // Animate generation
    println!("Generating dungeon...\n");
    thread::sleep(Duration::from_millis(500));

    for iteration in 0..7 {
        dungeon.render()?;
        println!("\nIteration: {}", iteration + 1);
        thread::sleep(Duration::from_millis(300));
        
        if !dungeon.simulate_step() {
            break;
        }
    }

    // Final render
    dungeon.render()?;
    println!("\nDungeon complete! Press any key to exit...");
    
    // Cleanup
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer).ok();
    
    execute!(stdout, cursor::Show, terminal::Clear(ClearType::All))?;
    terminal::disable_raw_mode()?;
    
    Ok(())
}