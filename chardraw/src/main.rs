use std::borrow::BorrowMut;
use std::env;

use pancurses::*;

const CHAR_WIDTH: u16 = 5;
const CHAR_HEIGHT: u16 = 7;

const CELL_WIDTH: u16 = 2;
const CELL_HEIGHT: u16 = 1;
const CELL_SELECTED_ATTR: Attribute = Attribute::Reverse;

const PIXEL_ENABLED: u8 = 0x01;
const PIXEL_DISABLED: u8 = 0x00;

const BORDER_THICKNESS: u16 = 01;

#[derive(PartialEq, PartialOrd, Debug)]
struct Descartes {
    x: u16,
    y: u16,
}

impl Descartes {
    fn from_tuple(t: (i32, i32)) -> Descartes {
        Descartes {
            x: t.1 as u16,
            y: t.0 as u16,
        }
    }
}

type Cursor = Descartes;
type Sizes = Descartes;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("No file presented! Run arduino-chardraw.exe [NAME]");
        return;
    }

    let main_window = create_main_window();
    let container_window = create_container_window(&main_window);
    let editor_window = create_grid_window(&container_window);

    let mut grid = vec![vec![0u8; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize];
    let mut cursor = Cursor { x: 3, y: 0 };

    draw_grid(&editor_window, &cursor, &grid);

    loop {
        match main_window.getch() {

            // Left
            Some(Input::KeyLeft) |
            Some(Input::Character('a')) |
            Some(Input::Character('h')) => {
                if cursor.x != 0 {
                    cursor.x -= 1;
                } else {
                    cursor.x = CHAR_WIDTH - 1;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Right
            Some(Input::KeyRight) |
            Some(Input::Character('d')) |
            Some(Input::Character('l')) => {
                if cursor.x != CHAR_WIDTH - 1 {
                    cursor.x += 1;
                } else {
                    cursor.x = 0;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Up
            Some(Input::KeyUp) |
            Some(Input::Character('w')) |
            Some(Input::Character('k')) => {
                if cursor.y != 0 {
                    cursor.y -= 1;
                } else {
                    cursor.y = CHAR_HEIGHT - 1;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Down
            Some(Input::KeyDown) |
            Some(Input::Character('s')) |
            Some(Input::Character('j')) => {
                if cursor.y != CHAR_HEIGHT - 1 {
                    cursor.y += 1;
                } else {
                    cursor.y = 0;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Enable pixel
            Some(Input::Character(' ')) => {
                switch_pixel(&mut grid, &cursor);
                draw_grid(&editor_window, &cursor, &grid);
            }

            // Fill whole line
            Some(Input::Character('r')) => {
                for x in 0..CHAR_WIDTH {
                    grid[cursor.y as usize][x as usize] = PIXEL_ENABLED;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Fill whole column
            Some(Input::Character('c')) => {
                for y in 0..CHAR_HEIGHT {
                    grid[y as usize][cursor.x as usize] = PIXEL_ENABLED;
                }

                draw_grid(&editor_window, &cursor, &grid);
            }

            // Exit
            Some(Input::Character('q')) => {
                endwin();

                println!("byte {}[8] = {{", args[1]);
                for l in 0..CHAR_HEIGHT {
                    print!("\tB");
                    for p in grid[l as usize].iter() {
                        print!("{}", p);
                    }
                    println!(",");
                }
                println!("}};");

                break;
            }

            _ => {}
        }
    }
}

fn create_main_window() -> Window {
    let window = initscr();

    cbreak();
    noecho();

    // Enable mouse and arrows keys
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());
    window.keypad(true);

    // Setup colors
    start_color();
    use_default_colors();
    window.attrset(COLOR_PAIR(1));

    // Hide cursor
    curs_set(0);

    // Clear screen
    window.clear();
    window.refresh();

    return window;
}

fn create_container_window(main_window: &Window) -> Window {
    let window: Window;
    let terminal_size = Sizes::from_tuple(main_window.get_max_yx()) as Sizes;

    let pos = Cursor {
        y: (terminal_size.y / 2) - (CHAR_HEIGHT * CELL_HEIGHT / 2),
        x: (terminal_size.x / 2) - (CHAR_WIDTH * CELL_WIDTH / 2),
    };

    let size = Sizes {
        y: (CHAR_HEIGHT * CELL_HEIGHT) + (BORDER_THICKNESS * 2),
        x: (CHAR_WIDTH * CELL_WIDTH) + (BORDER_THICKNESS * 2),
    };

    match main_window.subwin(
        size.y as i32,
        size.x as i32,
        pos.y as i32,
        pos.x as i32,
    ) {
        Ok(win) => { window = win }
        Err(_) => panic!("Can't create container subwin"),
    };

    window.border(
        '│',
        '│',
        '─',
        '─',
        '┌',
        '┐',
        '└',
        '┘',
    );

    window.refresh();

    return window;
}

fn create_grid_window(main_win: &Window) -> Window {
    let size = Sizes {
        y: (CHAR_HEIGHT * CELL_HEIGHT),
        x: (CHAR_WIDTH * CELL_WIDTH),
    };

    match main_win.derwin(
        size.y as i32,
        size.x as i32,
        BORDER_THICKNESS as i32,
        BORDER_THICKNESS as i32,
    ) {
        Ok(win) => return win,
        Err(_) => panic!("Can't create editor subwin"),
    }
}

fn draw_grid(window: &Window, current_position: &Cursor, grid: &Vec<Vec<u8>>) {
    window.attroff(CELL_SELECTED_ATTR);

    for y in 0..CHAR_HEIGHT {
        for x in 0..CHAR_WIDTH {
            let cell_position = Cursor { x, y };

            if *current_position == cell_position {
                window.attron(CELL_SELECTED_ATTR);
            }

            for cy in 0..CELL_HEIGHT {
                for cx in 0..CELL_WIDTH {
                    let current_char =
                        if grid[y as usize][x as usize] == PIXEL_ENABLED {
                            '█'
                        } else {
                            '░'
                        };

                    window.mvaddch(
                        (cell_position.y * CELL_HEIGHT + cy) as i32,
                        (cell_position.x * CELL_WIDTH + cx) as i32,
                        current_char,
                    );
                }
            }

            if *current_position == cell_position {
                window.attroff(CELL_SELECTED_ATTR);
            }
        }
    }

    window.refresh();
}

fn switch_pixel(grid: &mut Vec<Vec<u8>>, cursor: &Cursor) {
    let cell_ref = grid[cursor.y as usize][cursor.x as usize].borrow_mut();

    *cell_ref = if *cell_ref == PIXEL_ENABLED {
        PIXEL_DISABLED
    } else {
        PIXEL_ENABLED
    };
}