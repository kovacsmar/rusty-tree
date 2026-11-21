use std::{array::repeat, sync::mpsc, time::Duration};

use ctrlc::Error;
use rand::{RngExt, rngs::ThreadRng};

// Escape charater for ANSI sequences
static ESC: &str = "\x1b";

// Set up alternative screen and restoration after graceful shutdown
struct ScreenHandler;

impl ScreenHandler {
    fn new() -> Self {
        alt_screen();
        ScreenHandler
    }
}

impl Drop for ScreenHandler {
    fn drop(&mut self) {
        reset_screen();
    }
}

fn alt_screen() {
    // ANSI escape codes
    // Enter alternative buffer
    print!("{ESC}[?1049h");
    // Hide cursor
    print!("{ESC}[?25l");
    // Move cursor to home position
    print!("{ESC}[H");
}

fn reset_screen() {
    // ANSI escape codes
    // Restor cursor
    print!("{ESC}[?25h");
    // Exit alternative buffer
    print!("{ESC}[?1049l");
    // Reset all modes (styles and colors)
    print!("{ESC}[0m")
}

fn main() -> Result<(), Error> {
    // Set up capturing SIGINT for graceful shutdown
    let (tx, rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })?;

    let _screen_handler = ScreenHandler::new();

    // Set up random star display rng and base values
    let mut rng = rand::rng();

    let mut stars1: [bool; 25] = repeat(true);
    let mut stars2: [bool; 25] = repeat(false);

    loop {
        // Second loop for sequencing X-mas tree decoration
        for i in 0..4 {
            randomize_stars(&mut rng, &mut stars1);
            randomize_stars(&mut rng, &mut stars2);
            print_image(i, &stars1, &stars2);
            if rx.recv_timeout(Duration::from_millis(1000)).is_ok() {
                return Ok(());
            }
        }
    }
}

fn randomize_stars(rng: &mut ThreadRng, stars: &mut [bool; 25]) {
    for star in stars.iter_mut() {
        if roll_d100(rng) == 100 {
            *star = !*star;
        }
    }
}

fn roll_d100(rng: &mut ThreadRng) -> i32 {
    rng.random_range(1..=100)
}

fn star(is_star: &bool) -> &str {
    if *is_star { "✦" } else { " " }
}

fn print_image(color_seq: u8, stars1: &[bool; 25], stars2: &[bool; 25]) {
    // ANSI escape codes for colors
    let green = format!("{ESC}[32m");
    let yellow = format!("{ESC}[93m");
    let brown = format!("{ESC}[38;5;94m");
    let color_reset = format!("{ESC}[39m");
    // decoration colors
    let red_decoration = format!("{ESC}[38;5;196m");
    let yellow_decoration = format!("{ESC}[38;5;226m");
    let blue_decoration = format!("{ESC}[38;5;27m");
    let green_decoration = format!("{ESC}[38;5;82m");
    let decoration_colors = [
        &red_decoration,
        &yellow_decoration,
        &blue_decoration,
        &green_decoration,
    ];
    let n = decoration_colors.len();

    let c1 = decoration_colors[color_seq as usize % n];
    let c2 = decoration_colors[(color_seq + 1) as usize % n];
    let c3 = decoration_colors[(color_seq + 2) as usize % n];
    let c4 = decoration_colors[(color_seq + 3) as usize % n];

    // Christmas tree outline building briks
    let left_needle = format!("{green}╱{color_reset}");
    let right_needle = format!("{green}╲{color_reset}");
    let bottom_needle = format!("{green}⎺{color_reset}");
    let bottom_needle_long = bottom_needle.repeat(24);

    let image = format!(
        " {}       {}    {}          {}    {}     {}    {}   {}       {}       {}        {}

    {}       {}       {}         {}        {}         {}       {}           {}       {}

   {}                    {}                                 {}                   {}

        {}                        {}             {}                              {}

    {}     {}          {}              {}         {}      {}          {}       {}

   {}                      {}                                 {}               {}

     {}                                     {}                          {}       {}
              {yellow}✩{color_reset}
              {left_needle}{right_needle}                             {}                 {}       {}
             {left_needle} ╷{right_needle}
            {left_needle} {left_needle}{c1}⬤{color_reset} {right_needle}                                 {}         {}       {}
            {bottom_needle}    {bottom_needle}
            {left_needle} {left_needle} ╷{right_needle}
           {left_needle}╷   {c2}⬤{color_reset} {right_needle}
          {left_needle} {c3}⬤{color_reset}{left_needle}     {right_needle}
         {left_needle} {left_needle} ╷   {right_needle}  {right_needle}
         {bottom_needle}   {c4}⬤{color_reset}      {bottom_needle}
         {left_needle} {left_needle}    ╷   {right_needle}
        {left_needle}╷      {c1}⬤{color_reset}  {right_needle}╷{right_needle}                            {brown}┏━━━━━━━━━━━━━━━━━━━━━┓{color_reset}
       {left_needle} {c1}⬤{color_reset} ╷ {left_needle}   ╷  {c3}⬤{color_reset} {right_needle}                           {brown}┃{color_reset}   Merry Christmas!  {brown}┃{color_reset}
      {left_needle}  {left_needle} {c2}⬤{color_reset}     {c4}⬤{color_reset}   {right_needle} {right_needle}                          {brown}┗━━━━━━━━━┳━┳━━━━━━━━━┛{color_reset}
      {bottom_needle}                {bottom_needle}                                    {brown}┃ ┃{color_reset}
      {left_needle} {left_needle} ╷       ╷   {right_needle}{right_needle}                                    {brown}┃ ┃{color_reset}
     {left_needle}    {c1}⬤{color_reset}       {c3}⬤{color_reset}   ╷ {right_needle}                                   {brown}┃ ┃{color_reset}
    {left_needle}╷    {left_needle}  ╷      {right_needle} {c2}⬤{color_reset}  {right_needle}                                  {brown}┃ ┃{color_reset}
   {left_needle} {c2}⬤{color_reset} {left_needle}     {c4}⬤{color_reset}         {right_needle}  {right_needle}                                 {brown}┃ ┃{color_reset}
   {bottom_needle_long}                                 {brown}┃ ┃{color_reset}
          {brown}⎟        ⎟{color_reset}                                        {brown}┃ ┃{color_reset}
          {brown}⎟        ⎟{color_reset}                                        {brown}┃ ┃{color_reset}
          {brown}⎠        ⎝{color_reset}",
    star(&stars1[0]), star(&stars2[0]),
    star(&stars1[1]), star(&stars2[1]),
    star(&stars1[2]), star(&stars2[2]),
    star(&stars1[3]), star(&stars2[3]),
    star(&stars1[4]), star(&stars2[4]),
    star(&stars1[5]), star(&stars2[5]),
    star(&stars1[6]), star(&stars2[6]),
    star(&stars1[7]), star(&stars2[7]),
    star(&stars1[8]), star(&stars2[8]),
    star(&stars1[9]), star(&stars2[9]),
    star(&stars1[10]), star(&stars2[10]),
    star(&stars1[11]), star(&stars2[11]),
    star(&stars1[12]), star(&stars2[12]),
    star(&stars1[13]), star(&stars2[13]),
    star(&stars1[14]), star(&stars2[14]),
    star(&stars1[15]), star(&stars2[15]),
    star(&stars1[16]), star(&stars2[16]),
    star(&stars1[17]), star(&stars2[17]),
    star(&stars1[18]), star(&stars2[18]),
    star(&stars1[19]), star(&stars2[19]),
    star(&stars1[20]), star(&stars2[20]),
    star(&stars1[21]), star(&stars2[21]),
    star(&stars1[22]), star(&stars2[22]),
    star(&stars1[23]), star(&stars2[23]),
    star(&stars1[24]), star(&stars2[24]),
    );
    // Set cursor to home position
    print!("{ESC}[H");
    println!("{image}");
}
