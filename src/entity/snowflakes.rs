use rand::Rng;


pub enum UpdateError {
    OutOfIndex,
}

pub enum UpdateSuccess {
    PairUpdated,
}


pub struct Snowflakes {
    capacity: usize,
    radius: Vec<f64>,
    velocity: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
}

pub struct Snowflake {
    pub id: usize,
    pub radius: f64,
    pub velocity: f64,
    pub x: f64,
    pub y: f64,
}


impl Into<[f64; 4]> for Snowflake {
    fn into(self) -> [f64; 4] {
        [
            self.x, self.y, self.radius, self.radius
        ]
    }
}


impl Snowflakes {
    ///
    /// Create a new instance and fill the vectors with `0.0_f64` values.
    ///
    ///  ```rust
    ///  let snowflakes = Snowflakes::new(100);
    ///  ```
    ///
    pub fn new(capacity: usize) -> Self {
        let mut x: Vec<f64> = vec![];
        let mut y: Vec<f64> = vec![];
        let mut radius: Vec<f64> = vec![];
        let mut velocity: Vec<f64> = vec![];

        let mut gen = rand::thread_rng();

        for _ in 0..capacity {
            x.push(0.);
            y.push(0.);

            radius.push(gen.gen_range(5.0f64, 15.0f64));
            velocity.push(gen.gen_range(0.5f64, 1.0f64));
        }

        Self { capacity, radius, velocity, x, y }
    }

    #[inline(always)]
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn get_snowflake(&self, index: usize) -> Option<Snowflake> {
        if index > self.capacity {
            return None
        }

        Some(Snowflake {
            id: index,
            radius: self.radius[index],
            velocity: self.velocity[index],
            x: self.x[index],
            y: self.y[index],
        })
    }

    pub fn update_pair(&mut self, index: usize, new_val: (f64, f64)) -> Result<UpdateSuccess, UpdateError> {
        if index > self.capacity {
            return Err(UpdateError::OutOfIndex)
        }

        self.x[index] = new_val.0;
        self.y[index] = new_val.1;

        Ok(UpdateSuccess::PairUpdated)
    }

    pub fn randomize_vectors(&mut self, max_x: f64, max_y: f64) {
        let mut gen = rand::thread_rng();

        for i in 0..self.capacity {
            let new_x = gen.gen_range(0., max_x);
            let new_y = gen.gen_range(0., max_y);

            self.update_pair(i, (new_x, new_y)).ok();
        }
    }
}