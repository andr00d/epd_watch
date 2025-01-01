use crate::pages::Pages;
use crate::shared_data::SharedData;
use crate::display::shape::ArrowDir;
use crate::display::font::Anchor;
use crate::io::Event;

// field works by using specific values for specific items. 
// 0: empty field
// 1: apple
// 2+: snake, with each incrementing number indicating the length

const FIELD_LEN: usize = 225;

struct GameState
{
    pub curr_pos: (u8, u8), 
    pub field: [u8; FIELD_LEN],
    pub game_lost: bool,
    pub game_won: bool,
    pub snake_len: u8,
    pub curr_dir: ArrowDir,
    pub menu_active: bool,
    pub menu_item: u8,
}

impl Pages
{
    pub(super) fn pg_snake(data: &mut SharedData) 
    {
        let mut state = GameState
        {
            curr_pos: (10, 12),
            field: [0; FIELD_LEN],
            game_lost: false,
            game_won: false,
            snake_len: 2,
            curr_dir: ArrowDir::Left,
            menu_active: true,
            menu_item: 0,
        };
        
        // setup start state
        state.field[55] = 1;
        state.field[190] = 2;
        state.field[191] = 3;

        loop
        {
            if state.menu_active {Self::draw_menu(data, &mut state);}
            else {Self::draw_game(data, &mut state);}
            data.display.update();

            let ev = data.io.get_input_waitms(500);
            let exit = Self::update_game(ev, &mut state); 
            if exit {break;}
        }
    }

    ///////////////////////////////////////////

    fn draw_game(data: &mut SharedData, state: &mut GameState)
    {
        // field border
        data.display.rect(9, 24, 152, 1);
        data.display.rect(9, 176, 152, 1);
        data.display.rect(9, 24, 1, 152);
        data.display.rect(161, 24, 1, 153);

        if state.game_lost || state.game_won
        {
            data.display.text("GAME", 85, 55, 5, Anchor::Center);
            if state.game_lost {data.display.text("OVER", 85, 85, 5, Anchor::Center);}
            if state.game_won {data.display.text("WON", 85, 85, 5, Anchor::Center);}
            return;
        }

        let (dir_up, dir_down) = Self::get_possible_dirs(&state.curr_dir);
        data.display.arrow(175, 25, 10, dir_up);
        data.display.arrow(175, 150, 10, dir_down);

        for i in 0..FIELD_LEN
        {
            let (x, y) = Self::get_field_xy(i);

            if state.field[i] == 1
            {
                data.display.rect(x+3, y+3, 4, 4);
            }
            else if state.field[i] > 1
            {
                if Self::is_connected(state, i, ArrowDir::Up)  {data.display.rect(x+2, y, 6, 8);}
                if Self::is_connected(state, i, ArrowDir::Down) {data.display.rect(x+2, y+2, 6, 8);}
                if Self::is_connected(state, i, ArrowDir::Left) {data.display.rect(x, y+2, 8, 6);}
                if Self::is_connected(state, i, ArrowDir::Right) {data.display.rect(x+2, y+2, 8, 6);}
            }
        }
    } 

    fn draw_menu(data: &mut SharedData, state: &mut GameState)
    {
        data.display.rect(10, 25, 150, 1);
        data.display.rect(10, 175, 150, 1);
        data.display.rect(10, 25, 1, 150);
        data.display.rect(160, 25, 1, 150);
        
        data.display.arrow(175, 35, 9, ArrowDir::Up);
        data.display.arrow(175, 160, 9, ArrowDir::Down);

        data.display.text("play", 140, 65, 5, Anchor::Right);
        data.display.text("exit", 140, 110, 5, Anchor::Right);
        
        if state.menu_item == 0 {data.display.arrow(145, 68, 9, ArrowDir::Left);}
        if state.menu_item == 1 {data.display.arrow(145, 113, 9, ArrowDir::Left);}
    }

    fn get_field_xy(field_index: usize) -> (u8, u8)
    {
        let x = 10 + 10 * (field_index % 15);
        let y = 25 + 10 * (field_index / 15);
        return (x as u8, y as u8);
    }

    fn is_connected(state: &mut GameState, block: usize, dir: ArrowDir) -> bool
    {
        let i = match dir
        {
            ArrowDir::Up =>    {if block < 15 {210 + block} else {block - 15}},
            ArrowDir::Right => {if block % 15 == 14 {block - 14} else {block + 1}},
            ArrowDir::Down =>  {if block >= 210 {block - 210} else {block + 15}},
            ArrowDir::Left =>  {if block % 15 == 0 {block + 14} else {block - 1}},
        };

        let a = state.field[block];
        let b = state.field[i];
        
        let diff = if a<b {b-a} else {a-b};
        return diff == 1 && b != 1;
    }

    ///////////////////////////////////////////

    fn update_game(ev: Event, state: &mut GameState) -> bool
    {
        if state.menu_active
        {
            if ev == Event::BtnUp || ev == Event::BtnDown
            {
                state.menu_item = (state.menu_item + 1) % 2;
            }
            if ev == Event::BtnMid && state.menu_item == 0 {state.menu_active = false;}
            if ev == Event::BtnMid {return state.menu_item == 1;}
            return false;
        }

        if ev == Event::BtnMid {state.menu_active = true; return false;}
        if state.game_lost || state.game_won {return false;}

        // wall check
        let (next_x, next_y, new_dir, hit_wall) = Self::get_next_step(ev, &state.curr_dir, state.curr_pos);
        state.game_lost |= hit_wall;
        state.curr_dir = new_dir;
        
        if state.game_lost {return false;}
        let square = &mut state.field[(next_y*15 + next_x) as usize];
        
        // stop hitting yourself
        if *square > 2
        {
            state.game_lost = true;
            return false;
        }

        // apple get
        if *square == 1
        {
            *square = 0;
            state.snake_len += 1;
            
            if state.snake_len == FIELD_LEN as u8{state.game_won = true;}
            
            // dont want to deal with rng module, so fake randomness.
            let next_x = ((state.curr_pos.0 as u32 * state.curr_pos.1 as u32 * 
                          (state.curr_dir.clone() as u32 + 4) + 3) % 15) as i16;
            let next_y = ((state.curr_pos.0 as u32 * state.curr_pos.1 as u32 * 
                         (state.curr_dir.clone() as u32 + 7) + 1) % 15) as i16;
            let mut next_index = next_y * 15 + next_x;

            if state.field[next_index as usize] > 0
            {
                let step = if state.curr_dir.clone() as u8 % 2 == 1 {-1} else {1};
                
                loop
                {
                    next_index = (next_index + step) % FIELD_LEN as i16;
                    if next_index < 0 {next_index = (FIELD_LEN-1) as i16;}
                    if state.field[next_index as usize] == 0 {break;}
                }
            }
            state.field[next_index as usize] = 1;
        }

        // perform step
        for i in 0..FIELD_LEN
        {
            let val = &mut state.field[i];
            if *val > 1 {*val += 1;}
            if *val >  state.snake_len + 1 {*val = 0;}
        }
        
        state.field[(next_y*15 + next_x) as usize] = 2;
        state.curr_pos = (next_x, next_y);
        return false;
    }

    fn get_next_step(ev: Event, dir: &ArrowDir, curr_pos: (u8, u8)) -> (u8, u8, ArrowDir, bool)
    {
        let (dir_a, dir_b) = Self::get_possible_dirs(&dir);
        let new_dir = match ev
        {
            Event::BtnUp => dir_a,
            Event::BtnDown => dir_b,
            _ => dir.clone(),
        };

        let (mut out_x, mut out_y) = curr_pos;
        let mut hit_wall = false;

        match new_dir
        {
            ArrowDir::Up => 
            {
                if out_y == 0 {hit_wall = true;}
                else {out_y-= 1;}
            },
            ArrowDir::Right => 
            {
                if out_x >= (14) as u8{hit_wall = true;}
                else {out_x+= 1;}
            },
            ArrowDir::Down => 
            {
                if out_y >= (14) as u8{hit_wall = true;}
                else {out_y+= 1;}
            },
            ArrowDir::Left => 
            {
                if out_x == 0 {hit_wall = true;}
                else {out_x-= 1;}
            },
        };

        return (out_x, out_y, new_dir, hit_wall);
    }

    fn get_possible_dirs(dir: &ArrowDir) -> (ArrowDir, ArrowDir)
    {
        return match dir
        {
            ArrowDir::Up => (ArrowDir::Left, ArrowDir::Right),
            ArrowDir::Right => (ArrowDir::Up, ArrowDir::Down),
            ArrowDir::Down => (ArrowDir::Right, ArrowDir::Left),
            ArrowDir::Left => (ArrowDir::Down, ArrowDir::Up),
        };
    } 
}