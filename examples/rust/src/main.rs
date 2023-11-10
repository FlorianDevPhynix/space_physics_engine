use space_physics_engine::{
    math::{prelude::*, Vec3, Mat3, mat3, vec3, vec2},
    util::clamp::{
        calc_vec3_percentage, 
        bring_vec3_closer, calc_scale
    }
};

fn main() {
    let cam = Vec3::default();
    let planet_pos = DVec3 { x: 1_000_000_000.0, y: 1_000_000_000.0, z: 1_000_000_000.0 };
    let planet_size = DVec3 { x: 695_508_000.0, y: 695_508_000.0, z: 695_508_000.0 };
    println!("distance: {}", cam.as_dvec3().distance(planet_pos));

    let percentage = calc_vec3_percentage(1000.0, cam, planet_pos);
    println!("percentage: {}", percentage);

    let closer_planet_pos = bring_vec3_closer(percentage, cam, planet_pos);
    let closer_planet_scale = calc_scale(percentage, planet_size);
    println!("planet: new position={}; new distance={}; new size={}",
        closer_planet_pos, cam.as_dvec3().distance(closer_planet_pos.as_dvec3()),
        closer_planet_scale
    );

    let from_cam_towards_planet = cam.as_dvec3() - planet_pos;
    let from_cam_towards_newplanet = cam - closer_planet_pos;
    let original_angle = f64::atan(planet_size.x / from_cam_towards_planet.length());
    let new_angle = f32::atan(closer_planet_scale.x / from_cam_towards_newplanet.length());
    println!("angle to planet border: original={}; new={}", original_angle.to_degrees() as f32, new_angle.to_degrees());

    fn new_translation(dx: f32, dy: f32) -> Mat3 {
        mat3(
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(dx, dy, 1.0)
        )
    }
    fn new_rotation(winkel: f32) -> Mat3 {
        mat3(
            vec3(f32::cos(winkel), f32::sin(winkel), 0.0),
            vec3(-f32::sin(winkel), f32::cos(winkel), 0.0),
            vec3(0.0, 0.0, 1.0)
        )
    }

    let t_person = new_translation(-20.0, -6.0);
    let t_fahrzeug = new_translation(-10.0, -3.0);
    let r_fahrzeug = new_rotation(17.0);
    let p_scheinwerfer = vec3(1.5, 0.0, 1.0);

    let p_fahrzeug_lokal = t_person * t_fahrzeug * r_fahrzeug * p_scheinwerfer;
    //(-28.57, -8.57, 1)
    println!("\nscheinwerfer in person lokal={}", p_fahrzeug_lokal);


    use space_physics_engine::pmo::PlanetaryMassObject;
    let pmos = vec![
        PlanetaryMassObject::new(1000.0, 1.0),
        PlanetaryMassObject::new(1000.0, 1.0),
        PlanetaryMassObject::new(1000.0, 1.0),
    ];
    space_physics_engine::pmo::calc_gravity_forces(pmos);
}

#[cfg(test)]
mod test {
    #[test]
    fn window_viewport_transformation() {
        use space_physics_engine::{
            math::{vec3, mat3},
            math::vec2,
        };

        let p1 = vec2(0.1, 0.3);
        let p2 = vec2(0.9, 0.3);
        let p3 = vec2(0.1, 0.8);
        let p4 = vec2(0.9, 0.8);

        let q1 = vec2(1.0, 2.0);
        let q2 = vec2(5.0, 2.0);
        let q3 = vec2(1.0, 10.0);
        let q4 = vec2(5.0, 10.0);

        let x_min = p1.x;
        let x_max = p4.x;
        let delta_x = x_max - x_min;
        let y_min = p1.y;
        let y_max = p4.y;
        let delta_y = y_max - y_min;

        let u_min = q1.x;
        let u_max = q4.x;
        let delta_u = u_max - u_min;
        let v_min = q1.y;
        let v_max = q4.y;
        let delta_v = v_max - v_min;

        // Skalierungsfaktor
        let s = vec2(delta_u / delta_x, delta_v / delta_y);


        let mat = mat3(
            vec3(s.x, 0.0, 0.0),
            vec3(0.0, -s.y, 0.0),
            vec3(-s.x * x_min + u_min, s.y * y_min + v_max, 1.0)
        );

        let a = mat * vec3(0.4, 0.4, 1.0);
        let b = mat * vec3(0.7, 0.4, 1.0);
        let c = mat * vec3(0.7, 0.7, 1.0);
        println!("A={}; B={}; C={}", a, b, c);
    }
}