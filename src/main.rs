mod processor;
mod tests;
mod opcode_info;
use crate::processor::CPU;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use rand::Rng;
extern crate sdl2;

const SCALE: f32 = 10.0;
const W: u32 = 32; // Width
const H: u32 = 32; // Height

fn main() {

    // Initialize sdl
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("6502 Snake", (W as f32 * SCALE) as u32, (H as f32 * SCALE) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(SCALE, SCALE).unwrap();

    // Texture is W x H, each pixel is represented by three bytes (RGB)
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, W, H).unwrap();

    // Opcodes for the snake game
    let snake_opcodes = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
        0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
        0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
        0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
        0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
        0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
        0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
        0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
        0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
        0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
        0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
        0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
        0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
        0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
        0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
        0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
        0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
        0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
        0xea, 0xca, 0xd0, 0xfb, 0x60
    ];

    let mut cpu = CPU::new();
    cpu.load_program(&snake_opcodes);
    
    let mut screen_state = [0 as u8; W as usize * 3 * H as usize];
    let mut rng = rand::thread_rng();

    // On every new iteration of the match function (the giant function with all the opcode cases), this function will be called back
    cpu.callback(move |cpu| {
        check_user_input(cpu, &mut event_pump); // Check if the user has inputted anything
        cpu.write_memory_u8(0xFE, rng.gen_range(1, 16)); // Write a new random number into the address, which chooses the random color

        // If the screen state has changed, this function is called
        if check_screen_state(cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32*3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        ::std::thread::sleep(std::time::Duration::new(0, 50_000)); // To make sure the game doesn't go too fast
});
}

fn check_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Escape), ..} =>{
                std::process::exit(0);
            }

            // Go up
            Event::KeyDown { keycode: Some(Keycode::W) | Some(Keycode::Up), .. } => {
                cpu.write_memory_u8(0xFF, 0x77);
            }
            
            // Go Down
            Event::KeyDown { keycode: Some(Keycode::S) | Some(Keycode::Down), .. } => {
                cpu.write_memory_u8(0xFF, 0x73);
            }

            // Go left
            Event::KeyDown { keycode: Some(Keycode::A) | Some(Keycode::Left), .. } => {
                cpu.write_memory_u8(0xFF, 0x61);
            }

            // Go right
            Event::KeyDown { keycode: Some(Keycode::D) | Some(Keycode::Right), .. } => {
                cpu.write_memory_u8(0xFF, 0x64);
            }

            _ => {}
        }
    }
} 

fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 3 | 4 | 5 | 6 | 7 => sdl2::pixels::Color::BLUE,
        9 | 10 | 11 | 12 | 13 | 14 => sdl2::pixels::Color::CYAN,
        _ => sdl2::pixels::Color::GREEN,

    }
}

fn check_screen_state(cpu: &CPU, frame: &mut [u8; 32*3*32]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_idx = cpu.read_memory_u8(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        }

        frame_idx += 3;
    }

    update
}