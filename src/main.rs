use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

const LIMIT_FPS: i32 = 60;

struct Tcod {
    root: Root,
    con: Offscreen,
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    /// move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

/// A tile of the map and its properties

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

fn main() {
    let root = Root::initializer()
        .font("font10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rouge?")
        .init();

    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };
    // player axis
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    // create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);

    // create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);

    // the list of objects with those two
    let mut objects = [player, npc];

    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let game = Game {
        // generate map (at this point it's not drawn to the screen)
        map: make_map(),
    };

    tcod::system::set_fps(LIMIT_FPS);
    while !tcod.root.window_closed() {
        tcod.con.clear();

        fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
            // draw all objects in the list
            for object in objects {
                object.draw(&mut tcod.con);
            }

            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    let wall = game.map[x as usize][y as usize].block_sight;
                    if wall {
                        tcod.con
                            .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
                    } else {
                        tcod.con
                            .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
                    }
                }
            }

            blit(
                &tcod.con,
                (0, 0),
                (SCREEN_WIDTH, SCREEN_HEIGHT),
                &mut tcod.root,
                (0, 0),
                1.0,
                1.0,
            );
        }

        tcod.root.flush();
        render_all(&mut tcod, &game, &objects);

        let player = &mut objects[0];

        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}

fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    let key = tcod.root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }

    false
}

fn make_map() -> Map {
    // fill map with "unblocked" tiles
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    // map[30][22] = Tile::wall();
    // map[50][22] = Tile::wall();

    map
}
