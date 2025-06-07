const GRAVITY: f32 = -9.81;
const DRAG: f32 = 0.005;

#[derive(Clone, Copy)]
struct Window {
    width: u32,
    height: u32,
}

struct Particle {
    radius: u32,
    // mass: u32,
    position: (f32, f32),
    velocity: (f32, f32),
}

impl Particle {
    fn create(radius: u32) -> Particle {
    Particle {
        radius,
        // mass,
        position: (50f32, 50f32),
        velocity: (10f32, 10f32),
    }
}

    fn update(&mut self) {
        self.velocity.0 *= DRAG;
        self.velocity.1 *= DRAG;
        self.velocity.1 += GRAVITY;

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
    } 

    fn collision(&mut self, &boundary : &Window) {
        if (self.position.0 - self.radius as f32) < 0f32 {
            println!("Plink! Particle hit wall at position {:?}", self.position);
            self.position.0 = 0f32 + self.radius as f32;
            self.velocity.0 *= -1f32
        }
        if  (self.position.0 + self.radius as f32) > boundary.width as f32 {
            println!("Plink! Particle hit wall at position {:?}", self.position);
            self.position.0 = boundary.width as f32 - self.radius as f32;
            self.velocity.0 *= -1f32
            
        }
        if (self.position.1 - self.radius as f32) < 0f32 {
            println!("Plink! Particle hit wall at position {:?}", self.position);
            self.position.1 = 0f32 + self.radius as f32;
            self.velocity.1 *= -1f32
        }
        if (self.position.1 + self.radius as f32) > boundary.height as f32 {
            println!("Plink! Particle hit wall at position {:?}", self.position);
            self.position.1 = boundary.height as f32 - self.radius as f32;
            self.velocity.1 *= -1f32
        }
    }

}


fn main() {
    let mut particle: Particle = Particle::create(5);

    let window: Window = Window {
        width: 100,
        height: 100,
    };

    let mut counter: u32 = 0;

    // let mut previous_time = std::time::Instant::now();
    while counter < 50 {
        particle.update();

        particle.collision(&window);
        std::thread::sleep(std::time::Duration::from_millis(100));

        counter += 1;
    }
}
