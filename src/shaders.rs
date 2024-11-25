// shaders.rs

use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
	// Transform position
	let position = Vec4::new(
		vertex.position.x,
		vertex.position.y,
		vertex.position.z,
		1.0
	);
	let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

	// Perform perspective division
	let w = transformed.w;
	let ndc_position = Vec4::new(
		transformed.x / w,
		transformed.y / w,
		transformed.z / w,
		1.0
	);

	// apply viewport matrix
	let screen_position = uniforms.viewport_matrix * ndc_position;

	// Transform normal
	let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
	let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

	let transformed_normal = normal_matrix * vertex.normal;

	// Create a new Vertex with transformed attributes
	Vertex {
		position: vertex.position,
		normal: vertex.normal,
		tex_coords: vertex.tex_coords,
		color: vertex.color,
		transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
		transformed_normal,
	}
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
	
	//lava_planet_shader(fragment, uniforms)
	//gas_planet_color(fragment, uniforms)
	//sun_shader(fragment, uniforms)
	rocky_planet_shader(fragment, uniforms)
	//gas_giant_shader(fragment, uniforms)
	//ice_planet_shader(fragment, uniforms)
	//moon_color(fragment, uniforms)

}

fn moon_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Aumenta la escala del ruido para más detalles.
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 20.0, fragment.vertex_position.z * 20.0);
    
    let elevation = noise_value; // Puedes combinar varios niveles de ruido si lo deseas.

    // Define umbrales para diferentes tipos de terreno lunar.
    let low_threshold = -0.1; 
    let medium_threshold = 0.1;
    let high_threshold = 0.3; // Agregar un nuevo umbral para cráteres.

    // Define colores representativos para la luna.
    let dark_surface_color = Color::new(169, 169, 169); // Gris oscuro.
    let light_surface_color = Color::new(211, 211, 211); // Gris claro.
    let crater_color = Color::new(255, 255, 255);       // Blanco para los cráteres.

    // Determina el color basado en la elevación lunar.
    let color = if elevation < low_threshold {
        dark_surface_color
    } else if elevation < medium_threshold {
        light_surface_color
    } else if elevation < high_threshold {
        crater_color // Área de cráteres.
    } else {
        Color::new(240, 240, 240) // Color para áreas muy altas.
    };

    // Devuelve el color multiplicado por la intensidad del fragmento.
    color * fragment.intensity
}

fn gas_planet_color(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Utiliza la posición del fragmento y el tiempo para generar un "seed" para el ruido.
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
    
    // Crea un generador de números aleatorios basado en el seed.
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
    
    // Genera un número aleatorio para la variación en el color.
    let random_number = rng.gen_range(0..=100);

    // Define colores base para el planeta gaseoso.
    let base_color = Color::new(70, 130, 180); // Azul
    let cloud_color = Color::new(255, 255, 255); // Blanco para nubes
    let shadow_color = Color::new(50, 50, 100); // Color oscuro para sombras

    // Calcular el factor de nubes usando el ruido
    let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.z * 5.0);
    let cloud_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

    // Selección de color basado en el número aleatorio para agregar variación.
    let planet_color = if random_number < 50 {
        base_color * (1.0 - cloud_factor) + cloud_color * cloud_factor
    } else {
        cloud_color * cloud_factor // Predominan las nubes
    };

    // Añadir sombras sutiles
    let shadow_factor = (1.0 - noise_value).max(0.0);
    let shadow_effect = shadow_color * shadow_factor * 0.3; // Sombra suave

    // Combina el color del planeta y las sombras
    let final_color = planet_color + shadow_effect;

    // Brillo atmosférico (opcional)
    let glow_color = Color::new(200, 200, 255); // Brillo azul claro
    let glow_factor = (1.0 - (fragment.vertex_position.y / 10.0).max(0.0).min(1.0)).max(0.0); // Basado en altura
    let final_glow = glow_color * glow_factor * 0.1; // Brillo sutil

    // Devuelve el color final combinado
    final_color + final_glow
}


fn lava_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
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

fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;
    let distance = position.magnitude(); // Distancia del centro

    // Base colors for the star
    let core_color = Color::new(255, 204, 0); // Brillante amarillo
    let edge_color = Color::new(255, 69, 0);  // Naranja más oscuro

    // Noise to create surface turbulence
    let noise_value = uniforms.noise.get_noise_3d(position.x * 10.0, position.y * 10.0, uniforms.time as f32 * 0.01);
    let turbulence = noise_value.abs();

    // Blend core and edge colors based on distance from center
    let blend_factor = (distance - 0.2).clamp(0.0, 1.0);
    let base_color = core_color.lerp(&edge_color, blend_factor);

    // Add dynamic turbulence effect
    let dynamic_color = base_color * (1.0 + turbulence * 0.3);

    // Glow effect based on proximity to the center
    let glow_factor = (1.0 - distance).clamp(0.0, 1.0).powi(2);
    let glow_color = Color::new(255, 255, 224) * glow_factor;

    dynamic_color + glow_color
}

fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;

    // Base colors for rocky surface
    let base_color = Color::new(139, 69, 19);   // Marrón
    let crater_color = Color::new(105, 105, 105); // Gris oscuro

    // Generate noise for surface texture
    let _surface_noise = uniforms.noise.get_noise_3d(position.x * 5.0, position.y * 5.0, position.z * 5.0);
    let crater_noise = uniforms.noise.get_noise_3d(position.x * 10.0, position.y * 10.0, position.z * 10.0).abs();

    // Simulate craters
    let crater_factor = (crater_noise - 0.5).clamp(0.0, 1.0).powi(2); // Cráter más profundo al acercarse a 1.0

    // Blend base color with crater color
    let rocky_color = base_color.lerp(&crater_color, crater_factor);

    // Simulate lighting intensity
    rocky_color * fragment.intensity
}


fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let position = fragment.vertex_position;

    // Base colors for gas giant bands
    let base_color = Color::new(70, 130, 180); // Azul
    let band_color = Color::new(255, 255, 255); // Blanco para las bandas

    // Generate horizontal bands using sine waves
    let band_factor = (position.y * 10.0).sin().abs();

    // Turbulence effect
    let turbulence = uniforms.noise.get_noise_3d(position.x * 5.0, position.y * 5.0, uniforms.time as f32 * 0.01).abs();

    // Blend band and base colors
    let gas_color = base_color.lerp(&band_color, band_factor * turbulence);

    // Add slight glow to simulate atmospheric scattering
    let glow_color = Color::new(200, 200, 255); // Azul claro
    let glow_factor = (1.0 - position.magnitude() / 10.0).clamp(0.0, 1.0);
    let final_glow = glow_color * glow_factor * 0.1;

    gas_color + final_glow
}


fn ice_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
	let position = fragment.vertex_position;

	// Base colors for the ice planet
	let base_color = Color::new(240, 248, 255); // Blanco azulado
	let ice_color = Color::new(173, 216, 230);  // Azul claro

	// Generate noise for surface texture
	let noise_value = uniforms.noise.get_noise_3d(position.x * 5.0, position.y * 5.0, position.z * 5.0);
	let ice_factor = (noise_value * 0.5 + 0.5).powi(2); // Escala el ruido entre 0 y 1.

	// Blend base color with ice color
	let ice_planet_color = base_color.lerp(&ice_color, ice_factor);

	// Add slight glow to simulate atmospheric scattering
	let glow_color = Color::new(200, 200, 255); // Azul claro
	let glow_factor = (1.0 - position.magnitude() / 10.0).clamp(0.0, 1.0);
	let final_glow = glow_color * glow_factor * 0.1;

	ice_planet_color + final_glow
}

