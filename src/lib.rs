extern crate sdl2;
extern crate gl;
extern crate nalgebra;
extern crate glhelper;

use sdl2::video::{GLProfile};
use sdl2::rect::Rect;
use sdl2::event::{Event};

use gl::types::{GLfloat, GLuint, GLint, GLsizeiptr};

use nalgebra::*;

use std::collections::{HashMap};
use std::mem;
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::f32::consts;

pub const CAMERA_DELTA: f32 = 0.3;

fn look_at(focus: &Point2<f32>, camera_z: f32) -> Matrix4<f32>
{
    let transform = Isometry3::look_at_rh(
        &Point3::new(focus.x, focus.y, camera_z),
        &Point3::new(focus.x, focus.y, 0.),
        &Vector3::new(0., 1., 0.));
    nalgebra::geometry::IsometryBase::to_homogeneous(&transform)
}

pub fn scale_mat(x: f32, y: f32, z: f32) -> Matrix4<f32>
{
    return Matrix4::new(
        x, 0., 0., 0.,
        0., y, 0., 0.,
        0., 0., z, 0.,
        0., 0., 0., 1.
        );
}

pub fn deg_to_rad(a: f32) -> f32
{
    return a * consts::PI / 180.0;
}

#[derive(PartialEq, Eq, Hash)]
pub enum ProgramKey
{
    Basic = 0,
    Fill,
    Line,
    Overlay,
}

pub struct PlotData
{
    pub axis_x: Vec<f32>,
    pub axis_y: Vec<f32>,
    pub values_x: Vec<f32>,
    pub values_y: Vec<f32>
}

pub fn init<F>(
    window_x: i32,
    window_y: i32,
    window_w: u32,
    window_h: u32,
    get_data: F) -> () 
    where F: Fn() -> PlotData
{
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);

    let window_bounds = Rect::new(window_x, window_y, window_w, window_h);
    let mut window = video_subsystem.window(
        "Plot", 
        window_bounds.width(), 
        window_bounds.height())
    .position(
        window_bounds.x(), 
        window_bounds.y())
    .opengl().build().unwrap();

    let context = match window.gl_create_context() {
        Ok(res) => res,
        Err(..) => panic!("Could not open vert shader")
    };
    match window.gl_make_current(&context) {
        Ok(_) => {},
        Err(..) => panic!("setting current context")
    }

    assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    assert_eq!(gl_attr.context_version(), (3, 3));

    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const c_void);

    video_subsystem.gl_set_swap_interval(1);

    let fps = 60.; /* match window.display_mode()
    {
        Ok(display) => { display.refresh_rate as f32 }
        Err(_) => { 60. }
    }; */
    let mut shaders: Vec<GLuint> = vec![];
    let line_program = glhelper::load_program(
        include_str!("../shaders/line.vert.glsl"), 
        include_str!("../shaders/line.frag.glsl"), 
        &mut shaders);
    // let line_proj_uni: GLint;
    // let line_view_uni: GLint;
    let line_transform_uni: GLint;
    let line_model_uni: GLint;
    let line_width_uni: GLint;
    let line_position_attr: GLuint;
    let line_normal_attr: GLuint;
    let mut line_vao: GLuint = 0;
    let mut line_vbo: GLuint = 0;

    let data = get_data();
    let mut LINE_DATA: Vec<f32> = vec![0.0; data.values_x.len()*glhelper::STRIDE];

    unsafe
    {
        gl::Viewport(0, 0, window_bounds.width() as i32, window_bounds.height() as i32);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);

        gl::UseProgram(line_program);
        gl::GenVertexArrays(1, &mut line_vao);
        gl::BindVertexArray(line_vao);
        gl::GenBuffers(1, &mut line_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, line_vbo);
        gl::BufferData(gl::ARRAY_BUFFER, (LINE_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&LINE_DATA[0]), gl::DYNAMIC_DRAW);
        // line_proj_uni = gl::GetUniformLocation(line_program, CString::new("proj").unwrap().as_ptr());
        // line_view_uni = gl::GetUniformLocation(line_program, CString::new("view").unwrap().as_ptr());
        line_transform_uni = gl::GetUniformLocation(line_program, CString::new("transform").unwrap().as_ptr());
        line_model_uni = gl::GetUniformLocation(line_program, CString::new("model").unwrap().as_ptr());
        line_width_uni = gl::GetUniformLocation(line_program, CString::new("width").unwrap().as_ptr());
        let line_position_size: GLint = 2;
        let line_normal_size: GLint = 2;
        let line_stride = (line_position_size + line_normal_size) * mem::size_of::<GLfloat>() as i32;
        line_position_attr = gl::GetAttribLocation(line_program, CString::new("position").unwrap().as_ptr()) as GLuint;
        gl::EnableVertexAttribArray(line_position_attr);
        gl::VertexAttribPointer(line_position_attr, line_position_size, gl::FLOAT, gl::FALSE, line_stride, ptr::null());
        gl::DisableVertexAttribArray(line_position_attr);
        line_normal_attr = gl::GetAttribLocation(line_program, CString::new("normal").unwrap().as_ptr()) as GLuint;
        gl::EnableVertexAttribArray(line_normal_attr);
        gl::VertexAttribPointer(line_normal_attr, line_normal_size, gl::FLOAT, gl::TRUE, line_stride, (line_position_size * mem::size_of::<GLfloat>() as i32) as *const c_void);
        gl::DisableVertexAttribArray(line_normal_attr);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }

    let mut keys: HashMap<ProgramKey, usize> = HashMap::new();
    let mut programs: Vec<GLuint> = vec![];
    let mut shaders: Vec<GLuint> = vec![];
    let mut vaos: Vec<GLuint> = vec![];
    let mut vbos: Vec<GLuint> = vec![];
    keys.insert(ProgramKey::Line, programs.len());
    programs.push(line_program);
    vaos.push(line_vao);
    vbos.push(line_vbo);

    let ortho: Matrix4<f32> = *Orthographic3::new(
        0.0, 
        window_w as f32, 
        0.0, 
        window_h as f32, 
        0., 
        1000.).as_matrix();

    let aspect: f32 = window_bounds.width() as f32 / window_bounds.height() as f32;
    let fovy: f32 = deg_to_rad(45.);
    let near: f32 = 1.;
    let far: f32 = 1000.;
    let camera_z: f32 = 100.;
    let mut focus = Point2::new(0., 0.);
    let proj: Matrix4<f32> = *Perspective3::new(aspect, fovy, near, far).as_matrix();

    let mut view = look_at(&focus, camera_z);

    let mut model: Matrix4<f32>;
    
    let mut event_pump = sdl.event_pump().unwrap();

    // let mut restart = false;
    let mut quit = false;
    // let mut show_menu = false;

    'render_loop: while !quit 
    {
        for event in event_pump.poll_iter()
        {
            match event
            {
                // QUIT
                Event::Quit {..} =>
                {
                    quit = true;
                    break;
                }
                _ => {}
            }
        }

        let data = get_data();

        let ortho: Matrix4<f32> = *Orthographic3::new(
            0.0, 
            1.0, 
            0.0, 
            1.0, 
            0., 
            1000.).as_matrix();

        let axis_ori_x = data.axis_x[0];
        let axis_ori_y = data.axis_y[0];
        let axis_len_x = data.axis_x.last().unwrap() - data.axis_x[0];
        let axis_len_y = data.axis_y.last().unwrap() - data.axis_y[0];

        let axes = data.axis_x.into_iter().zip(data.axis_y);
        let path = data.values_x.into_iter().zip(data.values_y).map(|(x, y)| ((x - axis_ori_x) / axis_len_x, (y - axis_ori_y) / axis_len_y)).collect::<Vec<(f32, f32)>>();

        glhelper::add_path_line(
            &path,
            path.len()-1,
            programs[keys[&ProgramKey::Line]],
            vaos[keys[&ProgramKey::Line]],
            vbos[keys[&ProgramKey::Line]]);

        unsafe
        {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model = nalgebra::geometry::IsometryBase::to_homogeneous(&Isometry3::new(
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.))
            ) * scale_mat(1., 1., 1.);
            gl::UseProgram(line_program);
            gl::BindVertexArray(line_vao);
            gl::EnableVertexAttribArray(line_position_attr);
            gl::EnableVertexAttribArray(line_normal_attr);
            // gl::UniformMatrix4fv(line_proj_uni, 1, gl::FALSE, mem::transmute(&proj));
            // gl::UniformMatrix4fv(line_view_uni, 1, gl::FALSE, mem::transmute(&view));
            gl::UniformMatrix4fv(line_transform_uni, 1, gl::FALSE, mem::transmute(&ortho));
            gl::UniformMatrix4fv(line_model_uni, 1, gl::FALSE, mem::transmute(&model));
            gl::Uniform1f(line_width_uni, 0.01);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4*(path.len()-1) as GLint);
            gl::DisableVertexAttribArray(line_position_attr);
            gl::DisableVertexAttribArray(line_normal_attr);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
        glhelper::check_gl_error(file!(), line!());

        window.gl_swap_window();
    }
}

#[cfg(test)]
mod tests 
{
	use super::*;

    #[test]
    fn it_works() 
    {

    }
}
