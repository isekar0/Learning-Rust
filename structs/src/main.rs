const GRAVITY: f32 = -9.81;
const DRAG: f32 = 0.2;

struct Window {
    width: u32,
    height: u32,
}

struct Particle {
    radius: u32,
    mass: u32,
    position: [f32; 2],
    velocity: [f32; 2],
}

fn particle_create(radius: u32, mass: u32) -> Particle {
    Particle {
        radius,
        mass,
        position: [0f32; 2],
        velocity: [0f32; 2],
    }
}

fn particle_update_velocity(particle_: &mut Particle) {
    for component in 0..=1 {
        particle_.position[component] += particle_.velocity[component];
        particle_.velocity[component] *= 1f32 - DRAG;

        if component == 1 {
            particle_.velocity[component] += GRAVITY;
        }
    }
}

fn particle_collision_wall_resolve(particle_: &mut Particle, component: usize, boundry: &str) {
    assert!(component == 0 || component == 1);
    assert!(boundry == "down_left" || boundry == "up_right");

    particle_.position[component] = 0f32;
    particle_.velocity[component] *= -1f32;

    match boundry {
        "down_left" => particle_.position[component] += particle_.radius as f32,
        "up_right" => particle_.position[component] -= particle_.radius as f32,
        &_ => particle_.position[component] = 0f32,
    }
}

fn particle_collision_wall_detect(particle_: &mut Particle, window_: &Window) {
    let _wid: f32 = window_.width as f32;
    let _hei: f32 = window_.height as f32;

    let mut _collided: bool = false;

    match particle_.position[0] {
        f32::MIN..0f32 => {
            particle_collision_wall_resolve(particle_, 0, "down_left");
            _collided = true;
        }
        wid if (wid..=f32::MAX).contains(&wid) => {
            particle_collision_wall_resolve(particle_, 0, "up_right");
            _collided = true;
        }
        _ => return,
    }

    match particle_.position[1] {
        f32::MIN..0f32 => {
            particle_collision_wall_resolve(particle_, 1, "down_left");
            _collided = true;
        }
        hei if (hei..=f32::MAX).contains(&hei) => {
            particle_collision_wall_resolve(particle_, 1, "up_right");
            _collided = true;
        }
        _ => return,
    }

    if _collided == true {
        println!("Pling! Particle hit the wall")
    }
}

fn particle_status_update(particle_: &Particle) {
    // let speed: f32;

    let mut running_speed: f32 = 0f32;

    for x in particle_.velocity.iter() {
        running_speed += x * x;
    }

    assert!(running_speed >= 0f32);
    println!(
        "Currently at {:?} with a speed of {running_speed}",
        particle_.position
    )
}

fn main() {
    let mut particle_: Particle = particle_create(10, 10);

    let window_: Window = Window {
        width: 500,
        height: 500,
    };

    loop {
        particle_update_velocity(&mut particle_);
        particle_collision_wall_detect(&mut particle_, &window_);
        particle_status_update(&particle_);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
