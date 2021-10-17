use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, ImageData};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use js_sys::Math::random;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//write cpu struct with impls
pub struct Emulator {
    current_opcode: u32,
    memory: [u8; 4096],

    //regs
    registers: [u8; 16],
    index_register: u16,
    program_counter: u16,

    //display
    screen: [bool; 32 * 64],

    //stack related
    stack: [usize; 16],
    stack_pointer: usize,

    //timers
    delay_timer: u8,
    sound_timer: u8,

    //input
    keypad: [bool; 16],
}

impl Emulator {
    fn new() -> Emulator {
        Emulator {
            current_opcode: 0,
            memory: [0; 4096],

            //regs
            registers: [0; 16],
            index_register: 0,
            program_counter: 0,

            //display
            screen: [false; 32 * 64],

            //stack related
            stack: [0; 16],
            stack_pointer: 0,

            //timers
            delay_timer: 0,
            sound_timer: 0,

            //input
            keypad: [false; 16],
        }
    }

    fn fetch_opcode(&self) -> u16 {
        (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[(self.program_counter as usize + 1) as usize] as u16
    }

    fn cycle(&self) {}

    fn process_opcode(&self, opcode: u16) {}
}

#[wasm_bindgen]
pub fn handle_input(key: String) {
    // logging on webpage
    // let keys = document().create_element("lol").unwrap();
    // keys.set_inner_html(&format!("<li>{}<li>", &key).to_string());
    // body().append_child(&keys).unwrap();

    //let text = format!("Keypress: {}", key);
}

//write opcodes pattern matching logic

// check this for the loop
// https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    // GRAPHICS INIT
    // let context = canvas
    //     .get_context("webgl2")?
    //     .unwrap()
    //     .dyn_into::<WebGl2RenderingContext>()?;

    // let vert_shader = compile_shader(
    //     &context,
    //     WebGl2RenderingContext::VERTEX_SHADER,
    //     r##"#version 300 es

    //     in vec4 position;

    //     void main() {
    //         gl_Position = position;
    //         gl_PointSize = 9.9;
    //     }
    //     "##,
    // )?;

    // let frag_shader = compile_shader(
    //     &context,
    //     WebGl2RenderingContext::FRAGMENT_SHADER,
    //     r##"#version 300 es

    //     precision highp float;
    //     out vec4 outColor;

    //     void main() {
    //         outColor = vec4(1, 1, 1, 1);
    //     }
    //     "##,
    // )?;
    // let program = link_program(&context, &vert_shader, &frag_shader)?;
    // context.use_program(Some(&program));

    // let position_attribute_location = context.get_attrib_location(&program, "position");
    // let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    // context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.

    // let vao = context
    //     .create_vertex_array()
    //     .ok_or("Could not create vertex array object")?;
    // context.bind_vertex_array(Some(&vao));

    // context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    // context.enable_vertex_attrib_array(position_attribute_location as u32);

    // context.bind_vertex_array(Some(&vao));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let t1 = Rc::new(RefCell::new(None));
    let t2 = t1.clone();

    let w = window();
    *t2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(&w, f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    let w2 = window();

    // HARDCODE GAME ATM

    let break_out = "129ffcfc80a202ddc100eea204dba100eea2036002610587008610d67171086f388f174f00121770026f108f074f00121500ee22057d04220500ee22057dfc220500ee8080400168ff40ff68015ac0225300ee80b070fb61f880127005a203d0a100ee220b8b948a84220b4b0069014b3f69ff4a0068014a1f68ff4f0122434a1f228500ee00e06b1e6a142205220b221100eefe073e0012936e04fe1500ee6d1e6c1e6b406a1dc901490069ff68ff2205220b22116007e0a1223b6009e0a122332263229312b5";

    // INIT EMULATOR
    // let emulator = Emulator::new();

    // EVENT LOOP
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        set_timeout(&w2, t1.borrow().as_ref().unwrap(), 167);

        let mut screen_array = [1.0; 64 * 32 * 2];

        // for (i, x) in screen_array.iter_mut().enumerate() {
        //     match i as u32 % 2 {
        //         // 1 => *x = ((random() * 64.0).floor() / 64.0) as f32,
        //         // 0 => *x = ((random() * 32.0).floor() / 32.0) as f32,
        //         1 => *x = 0.9,
        //         0 => *x = 0.1,
        //         _ => *x = 0.0,
        //     }
        //}
        // handle input here
        // compute cpu cycle
        // draw_screen(&context, screen_array, position_attribute_location as u32);

        draw(
            &context,
            600,
            600,
            ((random() * 32.0).floor() / 32.0),
            ((random() * 32.0).floor() / 32.0),
        );
    }) as Box<dyn FnMut()>));

    request_animation_frame(&window(), g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) -> i32 {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn FnMut()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    // The real workhorse of this algorithm, generating pixel data
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn draw_screen(context: &CanvasRenderingContext2d, screen_array: [f32; 64 * 32 * 2]) {
    // position_attribute_location: u32,
    // ) {
    //     unsafe {
    //         let positions_array_buf_view = js_sys::Float32Array::view(&screen_array);

    //         context.buffer_data_with_array_buffer_view(
    //             WebGl2RenderingContext::ARRAY_BUFFER,
    //             &positions_array_buf_view,
    //             WebGl2RenderingContext::STATIC_DRAW,
    //         );
    //     }

    //     context.enable_vertex_attrib_array(position_attribute_location as u32);
    //     context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1365);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale = 0.005;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 4) as u8);
            data.push((iter_index / 2) as u8);
            data.push(iter_index as u8);
            data.push(255);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
