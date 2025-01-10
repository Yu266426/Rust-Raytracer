use rand::{thread_rng, Rng};

use crate::vec3::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    rand_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let rand_vec = core::array::from_fn(|_| Vec3::random_range(-1.0, 1.0).normalize());

        Self {
            rand_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut perm = core::array::from_fn(|i| i);

        Self::permute(&mut perm, POINT_COUNT);

        perm
    }

    fn permute(perm: &mut [usize], n: usize) {
        let mut rng = thread_rng();

        for i in (1..(n - 1)).rev() {
            let target = rng.gen_range(0..i);

            let temp = perm[i];
            perm[i] = perm[target];
            perm[target] = temp;
        }
    }

    pub fn noise(&self, pos: &Vec3) -> f64 {
        let u = pos.x - pos.x.floor();
        let v = pos.y - pos.y.floor();
        let w = pos.z - pos.z.floor();

        let i = pos.x.floor() as i32;
        let j = pos.y.floor() as i32;
        let k = pos.z.floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize];
                    c[di][dj][dk] = self.rand_vec[index];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, pos: &Vec3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_pos = pos.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_pos);

            weight *= 0.5;
            temp_pos *= 2.0;
        }

        accum.abs()
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);

                    accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }
}
