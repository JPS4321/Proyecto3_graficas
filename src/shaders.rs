
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
  //black_and_white(fragment, uniforms)
    //dalmata_shader(fragment, uniforms)
    // cloud_shader(fragment, uniforms)
    // cellular_shader(fragment, uniforms)
    // lava_shader(fragment, uniforms)
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
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  // to move our values 
    let ox = 100.0; // offset x in the noise map
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue
  
    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0;    // Offset x in the noise map
    let oy = 50.0;    // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0);   // Light green
    let cell_color_3 = Color::new(34, 139, 34);   // Forest green
    let cell_color_4 = Color::new(173, 255, 47);  // Yellow green
  
    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0);   // Darker red-orange
  
    // Get fragment position
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;
  
    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
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
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions
  
    // Use lerp for color blending based on noise value
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

  // Generar ruido para la textura de la superficie
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

  // Promediar valores de ruido para obtener una textura más suave
  let noise_value = (noise_value1 + noise_value2) * 0.5;

  // Colores para simular la superficie de Mercurio
  let base_color = Color::new(169, 169, 169); // Gris claro
  let crater_color = Color::new(105, 105, 105); // Gris oscuro para los cráteres
  let highlight_color = Color::new(192, 192, 192); // Gris más claro para los puntos elevados

  // Determinar el color en función del valor de ruido
  let surface_color = if noise_value < 0.25 {
      crater_color // Zonas de cráteres
  } else if noise_value < 0.7 {
      base_color // Superficie rocosa
  } else {
      highlight_color // Zonas elevadas y brillantes
  };

  // Añadir un leve efecto de iluminación pulsante
  let light_intensity = (uniforms.time as f32 * 0.05).sin() * 0.1 + 0.9;
  let final_color = surface_color.lerp(&highlight_color, light_intensity * fragment.intensity);

  final_color
}
fn cracked_earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 80.0;  // Ajusta el zoom para controlar el tamaño de las grietas
  let ox = 50.0;    // Desplazamiento en el mapa de ruido
  let oy = 50.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Generar el patrón de grietas para la textura de tierra
  let crack_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Colores para la tierra y las grietas
  let earth_color = Color::new(34, 139, 34);  // Verde para la tierra
  let crack_color = Color::new(0, 0, 255);    // Azul para las grietas

  // Determinar el color de la tierra según el valor de ruido de las grietas
  let base_color = if crack_noise_value < 0.2 {
      crack_color // Área de las grietas
  } else {
      earth_color // Área de la tierra
  };

  // Aplicar el efecto de nubes encima de la textura de tierra
  let cloud_zoom = 100.0;
  let cloud_offset_x = 100.0;
  let cloud_offset_y = 100.0;
  let t = uniforms.time as f32 * 0.5;

  // Generar el valor de ruido para las nubes
  let cloud_noise_value = uniforms.noise.get_noise_2d(
      x * cloud_zoom + cloud_offset_x + t,
      y * cloud_zoom + cloud_offset_y,
  );

  // Definir los colores para las nubes y el cielo
  let cloud_threshold = 0.8;  // Aumentamos el umbral para que las nubes sean menos frecuentes
  let cloud_color = Color::new(255, 255, 255); // Blanco para las nubes

  // Mezclar el color de las nubes con el color de la tierra
  let final_color = if cloud_noise_value > cloud_threshold {
      cloud_color.lerp(&base_color, 0.3) // Mezcla 30% de las nubes con el color de la tierra
  } else {
      base_color // Sin nubes, muestra el color de la tierra
  };

  // Aplicar intensidad para simular efectos de iluminación
  final_color * fragment.intensity
}
fn water_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Zoom y desplazamiento para controlar la escala de las olas
  let zoom = 50.0; 
  let offset_x = 0.0; 
  let offset_y = 0.0;

  // Posición del fragmento en el espacio del shader
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5; // Control del tiempo para animación

  // Generar ruido para simular olas dinámicas
  let wave_noise = uniforms.noise.get_noise_2d(x * zoom + offset_x + t, y * zoom + offset_y + t);

  // Intensidad de las olas (movimiento y profundidad)
  let wave_intensity = (wave_noise * 0.5 + 0.5) * fragment.intensity;

  // Colores base para el agua
  let shallow_color = Color::new(64, 164, 223); // Azul claro (agua poco profunda)
  let deep_color = Color::new(15, 82, 186);     // Azul oscuro (agua profunda)
  let foam_color = Color::new(255, 255, 255);   // Blanco para la espuma

  // Determinar el color base según la intensidad de las olas
  let base_color = shallow_color.lerp(&deep_color, wave_intensity);

  // Añadir espuma en las crestas de las olas
  let foam_threshold = 0.8;
  let final_color = if wave_noise > foam_threshold {
      foam_color.lerp(&base_color, 0.3) // Mezcla 30% espuma con color base
  } else {
      base_color
  };

  // Añadir un efecto de brillo dinámico según la profundidad
  let brightness = (t.sin() * 0.1 + 0.9) * fragment.intensity;
  final_color * brightness
}

fn crystal_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Configuraciones básicas para el cristal
  let zoom = 150.0;   // Controla el detalle del patrón fractal
  let refraction_intensity = 0.5; // Intensidad de la refracción
  let sparkle_threshold = 0.8; // Nivel para añadir destellos
  let sparkle_intensity = 1.5; // Brillo de los destellos
  let time = uniforms.time as f32 * 0.1; // Control del tiempo para animaciones

  // Posición del fragmento
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let z = fragment.depth;

  // Generar ruido 3D para el efecto cristalino
  let noise_value = uniforms.noise.get_noise_3d(x * zoom, y * zoom, z * zoom + time);

  // Simular refracciones variando los colores según el ruido
  let base_color = Color::new(135, 206, 235); // Azul claro (base)
  let highlight_color = Color::new(173, 216, 230); // Azul pálido (destellos)
  let refracted_color = base_color.lerp(&highlight_color, noise_value * refraction_intensity);

  // Añadir un efecto de destellos al cristal
  let sparkle_noise = uniforms.noise.get_noise_2d(x * zoom + time, y * zoom + time);
  let sparkle_color = if sparkle_noise > sparkle_threshold {
      Color::new(255, 255, 255) * sparkle_intensity // Blanco brillante
  } else {
      Color::new(0, 0, 0) // Sin destello
  };

  // Mezclar los colores base y los destellos
  let final_color = refracted_color + sparkle_color;

  // Ajustar la intensidad general con el fragmento
  final_color * fragment.intensity
}


fn arid_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Configuración de ruido para grietas y textura del suelo
  let zoom = 100.0; // Ajusta el nivel de detalle de la textura
  let offset_x = 50.0;
  let offset_y = 50.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Generar el ruido para simular grietas y variación del terreno
  let crack_noise = uniforms.noise.get_noise_2d(x * zoom + offset_x, y * zoom + offset_y).abs();

  // Colores base para el planeta árido
  let sand_color = Color::new(237, 201, 175); // Arena (beige claro)
  let crack_color = Color::new(117, 76, 36);  // Grietas (marrón oscuro)
  let highlight_color = Color::new(255, 223, 186); // Brillos suaves (arena iluminada)

  // Determinar el color base según el ruido
  let base_color = if crack_noise < 0.2 {
      crack_color // Zonas de grietas
  } else {
      sand_color // Zonas de arena
  };

  // Añadir una capa de iluminación dinámica para resaltar el terreno
  let light_intensity = (uniforms.time as f32 * 0.05).sin() * 0.1 + 0.9; // Oscilación suave
  let illuminated_color = base_color.lerp(&highlight_color, light_intensity * fragment.intensity);

  illuminated_color
}
