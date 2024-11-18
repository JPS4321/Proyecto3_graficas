use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use nalgebra_glm::Vec4;
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, apply_shader, ShaderType};  
use fastnoise_lite::{FastNoiseLite, NoiseType};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite
}

fn create_noise() -> FastNoiseLite {
    create_cloud_noise()
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render_with_shader(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader_type: ShaderType,
) {
    
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        if x < framebuffer.width && y < framebuffer.height {
            let shaded_color = apply_shader(&fragment, &uniforms, shader_type);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}
fn generate_stars(num_stars: usize, framebuffer_width: usize, framebuffer_height: usize) -> Vec<(usize, usize)> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut stars = Vec::with_capacity(num_stars);

    for _ in 0..num_stars {
        let x = rng.gen_range(0..framebuffer_width);
        let y = rng.gen_range(0..framebuffer_height);
        stars.push((x, y));
    }

    stars
}

fn draw_stars(framebuffer: &mut Framebuffer, stars: &[(usize, usize)]) {
    for &(x, y) in stars {
        framebuffer.set_current_color(0xFFFFFF); 
        framebuffer.point(x, y, 1.0); 
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Camera Following Planets with Orbit Lines and Offsets",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000); 

    
    let stars = generate_stars(100, framebuffer_width, framebuffer_height);

    
    let base_distance = 5.0;
    let distance_increment = 5.0;
    let speed_multiplier = 4.0;
    let planet_radius = 1.0;          
    let disappearance_buffer = 2.0;  

    
    let orbit_offsets = vec![
        0.0, 
        std::f32::consts::PI / 3.0, 
        std::f32::consts::PI / 4.0, 
        std::f32::consts::PI / 6.0, 
        std::f32::consts::PI / 2.0, 
        std::f32::consts::PI / 8.0, 
    ];

    let spheres = vec![
        (Vec3::new(0.0, 0.0, 0.0), ShaderType::Lava),
        (Vec3::new(base_distance, 0.0, 0.0), ShaderType::arid_shader),
        (Vec3::new(base_distance + distance_increment, 0.0, 0.0), ShaderType::CrackedEarth),
        (Vec3::new(base_distance + 2.0 * distance_increment, 0.0, 0.0), ShaderType::Dalmata),
        (Vec3::new(base_distance + 3.0 * distance_increment, 0.0, 0.0), ShaderType::crystal_shader),
        (Vec3::new(base_distance + 4.0 * distance_increment, 0.0, 0.0), ShaderType::water_shader),
    ];

    let scale = 1.0f32;

    
    let mut current_planet = 1;
    let initial_camera_distance = 10.0; 
    let mut camera = Camera::new(
        Vec3::new(base_distance, 0.0, initial_camera_distance),
        Vec3::new(base_distance, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    
    let obj = Obj::load("assets/models/Sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();
    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        
        let mut planet_positions = vec![];
        for (index, _) in spheres.iter().enumerate() {
            let position = if index == 0 {
                Vec3::new(0.0, 0.0, 0.0) 
            } else {
                let radius = base_distance + (index as f32 - 1.0) * distance_increment;
                let orbital_speed = speed_multiplier / radius;
                let angle = time as f32 * 0.01 * orbital_speed + orbit_offsets[index];
                Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin())
            };
            planet_positions.push(position);
        }

        
        if window.is_key_down(Key::Key1) {
            current_planet = 1;
        } else if window.is_key_down(Key::Key2) && spheres.len() > 2 {
            current_planet = 2;
        } else if window.is_key_down(Key::Key3) && spheres.len() > 3 {
            current_planet = 3;
        } else if window.is_key_down(Key::Key4) && spheres.len() > 4 {
            current_planet = 4;
        } else if window.is_key_down(Key::Key5) && spheres.len() > 5 {
            current_planet = 5;
        }

        
        let planet_position = planet_positions[current_planet];
        let camera_offset = camera.eye - camera.center; 
        camera.center = planet_position; 
        camera.eye = camera.center + camera_offset; 

        
        handle_camera_input(&window, &mut camera);

        framebuffer.clear();

        
        draw_stars(&mut framebuffer, &stars);

        
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        
        for (index, _) in spheres.iter().enumerate() {
            if index == 0 {
                continue; 
            }

            let radius = base_distance + (index as f32 - 1.0) * distance_increment;

            
            render_orbit_line(&mut framebuffer, radius, &view_matrix, &projection_matrix, &viewport_matrix);
        }

        
        for (index, (_, shader_type)) in spheres.iter().enumerate() {
            let position = planet_positions[index];

            
            if !is_in_frustum(&position, &view_matrix, &projection_matrix) {
                continue; 
            }

            
            let camera_to_planet_distance = (camera.eye - position).magnitude();
            if index != current_planet && camera_to_planet_distance <= planet_radius + disappearance_buffer {
                continue; 
            }

            
            let model_matrix = create_model_matrix(position, scale, Vec3::new(0.0, time as f32 * 0.01, 0.0));
            let noise = create_noise();
            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
                noise,
            };

            framebuffer.set_current_color(0xFFDDDD);
            render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays, *shader_type);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}




fn render_orbit_line(
    framebuffer: &mut Framebuffer,
    radius: f32,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
    viewport_matrix: &Mat4,
) {
    const SEGMENTS: usize = 360;
    let color = 0xCCCCCC; 

    for i in 0..SEGMENTS {
        let angle = (i as f32) * 2.0 * std::f32::consts::PI / SEGMENTS as f32;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        let position = Vec3::new(x, 0.0, z);

        let position_4d = Vec4::new(position.x, position.y, position.z, 1.0);
        let clip_space_pos = projection_matrix * view_matrix * position_4d;
        if clip_space_pos.w != 0.0 {
            let ndc = Vec3::new(
                clip_space_pos.x / clip_space_pos.w,
                clip_space_pos.y / clip_space_pos.w,
                clip_space_pos.z / clip_space_pos.w,
            );

            
            let screen_x = ((ndc.x + 1.0) * 0.5 * framebuffer.width as f32) as usize;
            let screen_y = ((1.0 - ndc.y) * 0.5 * framebuffer.height as f32) as usize;

            if screen_x < framebuffer.width && screen_y < framebuffer.height {
                framebuffer.set_current_color(color);
                framebuffer.point(screen_x, screen_y, ndc.z);
            }
        }
    }
}




fn is_in_frustum(position: &Vec3, view_matrix: &Mat4, projection_matrix: &Mat4) -> bool {
    
    let position_4d = Vec4::new(position.x, position.y, position.z, 1.0);

    
    let clip_space_pos = projection_matrix * view_matrix * position_4d;

    
    let x_ndc = clip_space_pos.x / clip_space_pos.w;
    let y_ndc = clip_space_pos.y / clip_space_pos.w;
    let z_ndc = clip_space_pos.z / clip_space_pos.w;

    
    x_ndc >= -1.0 && x_ndc <= 1.0 &&
    y_ndc >= -1.0 && y_ndc <= 1.0 &&
    z_ndc >= 0.0 && z_ndc <= 1.0 
}



fn handle_camera_input(window: &Window, camera: &mut Camera) {
    let rotation_speed = PI / 50.0;
    let zoom_speed = 0.5;

    
    if window.is_key_down(Key::Left) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, rotation_speed);
    }

    
    if window.is_key_down(Key::W) {
        camera.zoom(-zoom_speed);
    }
    if window.is_key_down(Key::S) {
        camera.zoom(zoom_speed);
    }
}
