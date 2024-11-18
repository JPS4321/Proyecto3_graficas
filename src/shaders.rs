
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;


#[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum ShaderType {
  Mercury,
  CrackedEarth,
  BlackAndWhite,
  Dalmata,
  Cloud,
  Cellular,
  Lava,
  water_shader,
  crystal_shader,
  arid_shader,
}

pub fn apply_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: ShaderType) -> Color {
  match shader_type {
      ShaderType::Mercury => mercury_shader(fragment, uniforms),
      ShaderType::CrackedEarth => cracked_earth_shader(fragment, uniforms),
      ShaderType::BlackAndWhite => black_and_white(fragment, uniforms),
      ShaderType::Dalmata => dalmata_shader(fragment, uniforms),
      ShaderType::Cloud => cloud_shader(fragment, uniforms),
      ShaderType::Cellular => cellular_shader(fragment, uniforms),
      ShaderType::Lava => lava_shader(fragment, uniforms),
      ShaderType::water_shader => water_shader(fragment, uniforms),
      ShaderType::crystal_shader => crystal_shader(fragment, uniforms),
      ShaderType::arid_shader => arid_shader(fragment, uniforms),


  }
}


pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  cracked_earth_shader(fragment, uniforms)
  
    
    
    
    
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
      Color::new(0, 0, 0)
    } else {
      Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}
  
fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let noise_value = uniforms.noise.get_noise_2d(
      (x + ox) * zoom,
      (y + oy) * zoom,
    );
  
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); 
    let base_color = Color::new(0, 0, 0); 
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  
    let ox = 100.0; 
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
    
    let cloud_threshold = 0.5; 
    let cloud_color = Color::new(255, 255, 255); 
    let sky_color = Color::new(30, 97, 145); 
  
    
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  
    let ox = 50.0;    
    let oy = 50.0;    
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    
    let cell_color_1 = Color::new(85, 107, 47);   
    let cell_color_2 = Color::new(124, 252, 0);   
    let cell_color_3 = Color::new(34, 139, 34);   
    let cell_color_4 = Color::new(173, 255, 47);  
  
    
    let final_color = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    
    final_color * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    
    let bright_color = Color::new(255, 240, 0); 
    let dark_color = Color::new(130, 20, 0);   
  
    
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;
  
    
    let zoom = 1000.0; 
    let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  
  
    
    let color = dark_color.lerp(&bright_color, noise_value);
  
    color * fragment.intensity
}

fn mercury_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 120.0;
  let ox = 15.0;
  let oy = 15.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let depth = fragment.depth;

  
  let noise_value1 = uniforms.noise.get_noise_3d(
      (x + ox) * zoom,
      (y + oy) * zoom,
      depth * zoom,
  );

  let noise_value2 = uniforms.noise.get_noise_3d(
      (x + ox + 30.0) * zoom,
      (y + oy + 30.0) * zoom,
      (depth + 30.0) * zoom,
  );

  
  let noise_value = (noise_value1 + noise_value2) * 0.5;

  
  let base_color = Color::new(169, 169, 169); 
  let crater_color = Color::new(105, 105, 105); 
  let highlight_color = Color::new(192, 192, 192); 

  
  let surface_color = if noise_value < 0.25 {
      crater_color 
  } else if noise_value < 0.7 {
      base_color 
  } else {
      highlight_color 
  };

  
  let light_intensity = (uniforms.time as f32 * 0.05).sin() * 0.1 + 0.9;
  let final_color = surface_color.lerp(&highlight_color, light_intensity * fragment.intensity);

  final_color
}
fn cracked_earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 80.0;  
  let ox = 50.0;    
  let oy = 50.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  
  let crack_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  
  let earth_color = Color::new(34, 139, 34);  
  let crack_color = Color::new(0, 0, 255);    

  
  let base_color = if crack_noise_value < 0.2 {
      crack_color 
  } else {
      earth_color 
  };

  
  let cloud_zoom = 100.0;
  let cloud_offset_x = 100.0;
  let cloud_offset_y = 100.0;
  let t = uniforms.time as f32 * 0.5;

  
  let cloud_noise_value = uniforms.noise.get_noise_2d(
      x * cloud_zoom + cloud_offset_x + t,
      y * cloud_zoom + cloud_offset_y,
  );

  
  let cloud_threshold = 0.8;  
  let cloud_color = Color::new(255, 255, 255); 

  
  let final_color = if cloud_noise_value > cloud_threshold {
      cloud_color.lerp(&base_color, 0.3) 
  } else {
      base_color 
  };

  
  final_color * fragment.intensity
}
fn water_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  
  let zoom = 50.0; 
  let offset_x = 0.0; 
  let offset_y = 0.0;

  
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5; 

  
  let wave_noise = uniforms.noise.get_noise_2d(x * zoom + offset_x + t, y * zoom + offset_y + t);

  
  let wave_intensity = (wave_noise * 0.5 + 0.5) * fragment.intensity;

  
  let shallow_color = Color::new(64, 164, 223); 
  let deep_color = Color::new(15, 82, 186);     
  let foam_color = Color::new(255, 255, 255);   

  
  let base_color = shallow_color.lerp(&deep_color, wave_intensity);

  
  let foam_threshold = 0.8;
  let final_color = if wave_noise > foam_threshold {
      foam_color.lerp(&base_color, 0.3) 
  } else {
      base_color
  };

  
  let brightness = (t.sin() * 0.1 + 0.9) * fragment.intensity;
  final_color * brightness
}

fn crystal_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  
  let zoom = 150.0;   
  let refraction_intensity = 0.5; 
  let sparkle_threshold = 0.8; 
  let sparkle_intensity = 1.5; 
  let time = uniforms.time as f32 * 0.1; 

  
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.depth;

  
  let noise_value = uniforms.noise.get_noise_3d(x * zoom, y * zoom, z * zoom + time);

  
  let base_color = Color::new(135, 206, 235); 
  let highlight_color = Color::new(173, 216, 230); 
  let refracted_color = base_color.lerp(&highlight_color, noise_value * refraction_intensity);

  
  let sparkle_noise = uniforms.noise.get_noise_2d(x * zoom + time, y * zoom + time);
  let sparkle_color = if sparkle_noise > sparkle_threshold {
      Color::new(255, 255, 255) * sparkle_intensity 
  } else {
      Color::new(0, 0, 0) 
  };

  
  let final_color = refracted_color + sparkle_color;

  
  final_color * fragment.intensity
}


fn arid_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  
  let zoom = 100.0; 
  let offset_x = 50.0;
  let offset_y = 50.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  
  let crack_noise = uniforms.noise.get_noise_2d(x * zoom + offset_x, y * zoom + offset_y).abs();

  
  let sand_color = Color::new(237, 201, 175); 
  let crack_color = Color::new(117, 76, 36);  
  let highlight_color = Color::new(255, 223, 186); 

  
  let base_color = if crack_noise < 0.2 {
      crack_color 
  } else {
      sand_color 
  };

  
  let light_intensity = (uniforms.time as f32 * 0.05).sin() * 0.1 + 0.9; 
  let illuminated_color = base_color.lerp(&highlight_color, light_intensity * fragment.intensity);

  illuminated_color
}
