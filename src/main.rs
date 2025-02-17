mod framebuffer;
mod bmp;
mod line;

use nalgebra_glm::Vec3;
use framebuffer::Framebuffer;
use line::draw_line;

fn draw_polygon(fb: &mut Framebuffer, points: &[Vec3], border_color: u32, fill_color: u32) {
    if points.len() < 2 {
        return;
    }

    // Scanline fill algorithm to fill the polygon
    fb.set_current_color(fill_color);

    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;

    // Encontrar el rango vertical del polígono
    for point in points {
        if point.y < y_min {
            y_min = point.y;
        }
        if point.y > y_max {
            y_max = point.y;
        }
    }

    // Redondear los límites y para usarlos en el bucle de escaneo
    let y_min = y_min.round() as isize;
    let y_max = y_max.round() as isize;

    // Algoritmo de escaneo para rellenar el polígono
    for y in y_min..=y_max {
        let mut intersections = Vec::new();

        for i in 0..points.len() {
            let start = points[i];
            let end = if i == points.len() - 1 {
                points[0]
            } else {
                points[i + 1]
            };

            // Encontrar intersecciones con el borde del polígono
            if (start.y <= y as f32 && end.y > y as f32) || (end.y <= y as f32 && start.y > y as f32) {
                let x = start.x + (y as f32 - start.y) * (end.x - start.x) / (end.y - start.y);
                intersections.push(x);
            }
        }

        // Ordenar las intersecciones para dibujar las líneas horizontales
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Dibujar líneas horizontales entre pares de intersecciones
        for chunk in intersections.chunks_exact(2) {
            if let [x1, x2] = chunk {
                let x1 = x1.round() as isize;
                let x2 = x2.round() as isize;
                draw_line(fb, Vec3::new(x1 as f32, y as f32, 0.0), Vec3::new(x2 as f32, y as f32, 0.0));
            }
        }
    }

    // Dibujar el contorno del polígono
    fb.set_current_color(border_color);
    for i in 0..points.len() {
        let start = points[i];
        let end = if i == points.len() - 1 {
            points[0]
        } else {
            points[i + 1]
        };
        draw_line(fb, start, end);  // Aquí se desreferencian start y end
    }
}

fn flip_y(points: &[Vec3], height: f32) -> Vec<Vec3> {
    points.iter().map(|p| Vec3::new(p.x, height - p.y, p.z)).collect()
}


fn main() -> std::io::Result<()> {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0xe8dbcf);

    // Definir detalles para polígono 1
    let poligono1_points = [
        Vec3::new(165.0, 380.0, 0.0), Vec3::new(185.0, 360.0, 0.0), Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0), Vec3::new(233.0, 330.0, 0.0), Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0), Vec3::new(220.0, 385.0, 0.0), Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];
    let poligono1_border_color = 0xFFFFFF; // Blanco
    let poligono1_fill_color = 0xfff000;   // Amarillo

    // Definir detalles para polígono 2
    let poligono2_points = [
        Vec3::new(321.0, 335.0, 0.0), Vec3::new(288.0, 286.0, 0.0), Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0),
    ];
    let poligono2_border_color = 0xFFFFFF; // Blanco
    let poligono2_fill_color = 0x0000ff;   // Azul

    // Definir detalles para polígono 3
    let poligono3_points = [
        Vec3::new(377.0, 249.0, 0.0), Vec3::new(411.0, 197.0, 0.0), Vec3::new(436.0, 249.0, 0.0),
    ];
    let poligono3_border_color = 0xFFFFFF; // Blanco
    let poligono3_fill_color = 0xFF0000;   // Rojo

    // Definir detalles para polígono 4
    let poligono4_points = [
        Vec3::new(413.0, 177.0, 0.0), Vec3::new(448.0, 159.0, 0.0), Vec3::new(502.0, 88.0, 0.0), 
        Vec3::new(553.0, 53.0, 0.0), Vec3::new(535.0, 36.0, 0.0), Vec3::new(676.0, 37.0, 0.0), 
        Vec3::new(660.0, 52.0, 0.0), Vec3::new(750.0, 145.0, 0.0), Vec3::new(761.0, 179.0, 0.0), 
        Vec3::new(672.0, 192.0, 0.0), Vec3::new(659.0, 214.0, 0.0), Vec3::new(615.0, 214.0, 0.0), 
        Vec3::new(632.0, 230.0, 0.0), Vec3::new(580.0, 230.0, 0.0), Vec3::new(597.0, 215.0, 0.0), 
        Vec3::new(552.0, 214.0, 0.0), Vec3::new(517.0, 144.0, 0.0), Vec3::new(466.0, 180.0, 0.0),
    ];
    let poligono4_border_color =  0xFFFFFF; // Blanco
    let poligono4_fill_color = 0x00FF00;   // Verde

    // Definir detalles para polígono 5
    let poligono5_points = [
        Vec3::new(682.0, 175.0, 0.0), Vec3::new(708.0, 120.0, 0.0), Vec3::new(735.0, 148.0, 0.0), 
        Vec3::new(739.0, 170.0, 0.0),
    ];
    let poligono5_color = 0xe8dbcf; // background color

    // Dibujar polígonos
    draw_polygon(&mut framebuffer, &poligono1_points, poligono1_border_color, poligono1_fill_color);
    draw_polygon(&mut framebuffer, &poligono2_points, poligono2_border_color, poligono2_fill_color);
    draw_polygon(&mut framebuffer, &poligono3_points, poligono3_border_color, poligono3_fill_color);
    draw_polygon(&mut framebuffer, &poligono4_points, poligono4_border_color, poligono4_fill_color);
    draw_polygon(&mut framebuffer, &poligono5_points, poligono5_color, poligono5_color);
    
    framebuffer.render_buffer("output.bmp")
}