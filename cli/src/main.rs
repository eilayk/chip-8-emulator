use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Stylize},
    terminal::{self},
    queue,
};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Stdout, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use chip_8::{Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the ROM file to load
    rom_path: PathBuf,

    /// Clock speed in Hz (instructions per second)
    #[arg(short, long, default_value_t = 700)]
    clock_speed: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Load ROM
    let mut rom_file = File::open(&cli.rom_path)?;
    let mut rom_data = Vec::new();
    rom_file.read_to_end(&mut rom_data)?;

    // Init Chip8
    let mut chip8 = Chip8::new();
    chip8.init();
    chip8.load_rom(&rom_data);

    // Setup Terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // Run loop
    let result = run_loop(&mut chip8, &mut stdout, cli.clock_speed);

    // Cleanup
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

fn run_loop(chip8: &mut Chip8, stdout: &mut Stdout, clock_speed: u64) -> Result<(), Box<dyn Error>> {
    let mut last_frame_time = Instant::now();
    let mut last_instruction_time = Instant::now();
    let instruction_duration = Duration::from_micros(1_000_000 / clock_speed);
    let frame_duration = Duration::from_millis(16); // ~60Hz
    
    // Key state tracking: index -> last_pressed_time
    let mut key_last_seen = [None; 16]; 
    let key_retention = Duration::from_millis(100); // hold key for 100ms after press event

    loop {
        // Handle Input
        // We poll multiple times or just once? Poll all available events.
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)) {
                    return Ok(());
                }
                
                // Map keys
                let chip8_key = match key.code {
                    KeyCode::Char('1') => Some(0x1),
                    KeyCode::Char('2') => Some(0x2),
                    KeyCode::Char('3') => Some(0x3),
                    KeyCode::Char('4') => Some(0xC),
                    KeyCode::Char('q') => Some(0x4),
                    KeyCode::Char('w') => Some(0x5),
                    KeyCode::Char('e') => Some(0x6),
                    KeyCode::Char('r') => Some(0xD),
                    KeyCode::Char('a') => Some(0x7),
                    KeyCode::Char('s') => Some(0x8),
                    KeyCode::Char('d') => Some(0x9),
                    KeyCode::Char('f') => Some(0xE),
                    KeyCode::Char('z') => Some(0xA),
                    KeyCode::Char('x') => Some(0x0),
                    KeyCode::Char('c') => Some(0xB),
                    KeyCode::Char('v') => Some(0xF),
                    _ => None,
                };
                
                if let Some(k) = chip8_key {
                    key_last_seen[k] = Some(Instant::now());
                }
            }
        }

        // Update Chip8 Key State based on timestamps
        let mut keys = [false; 16];
        let now = Instant::now();
        for i in 0..16 {
            if let Some(last_time) = key_last_seen[i] {
                if now.duration_since(last_time) < key_retention {
                    keys[i] = true;
                }
            }
        }
        chip8.set_pressed_keys(keys);

        // Execute Instructions
        // Catch up on cycles
        while last_instruction_time.elapsed() >= instruction_duration {
             chip8.cycle();
             last_instruction_time += instruction_duration;
        }
        
        // Timer Tick & Draw (60Hz)
        if last_frame_time.elapsed() >= frame_duration {
            chip8.tick_timers();
            draw_screen(chip8, stdout)?;
            last_frame_time = Instant::now();
        }
        
        // Sleep a tiny bit to yield
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn draw_screen(chip8: &Chip8, stdout: &mut Stdout) -> io::Result<()> {
    let pixels = chip8.get_display();
    
    // Reset cursor
    queue!(stdout, cursor::MoveTo(0, 0))?;
    
    for y in (0..SCREEN_HEIGHT).step_by(2) {
        for x in 0..SCREEN_WIDTH {
            let p1 = pixels[y * SCREEN_WIDTH + x];
            let p2 = if y + 1 < SCREEN_HEIGHT {
                pixels[(y + 1) * SCREEN_WIDTH + x]
            } else {
                false
            };

            let c = match (p1, p2) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            };
            queue!(stdout, style::Print(c))?;
        }
        queue!(stdout, style::Print("\r\n"))?;
    }
    
    // Draw status/info line
    queue!(stdout, style::Print("Controls: 1234 QWER ASDF ZXCV | Esc/Ctrl+C to Quit\r\n"))?;
    
        stdout.flush()?;
    
        Ok(())
    
    }
    
    
