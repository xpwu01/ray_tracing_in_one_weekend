use crate::*;

pub struct Perlin<const N: usize> {
    randvec: [Vec3; N],
    perm_x: [i32; N],
    perm_y: [i32; N],
    perm_z: [i32; N],
}

impl<const N: usize> Perlin<N> {
    pub fn new() -> Self {
        let mut randvec = [Vec3::zero(); N];
        for i in 0..N {
            randvec[i] = Vec3::random_range(-1.0, 1.0).unit_vector();
        }

        let mut perm_x = [0; N];
        let mut perm_y = [0; N];
        let mut perm_z = [0; N];
        Self::perlin_generate_perm(&mut perm_x);
        Self::perlin_generate_perm(&mut perm_y);
        Self::perlin_generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x
                        [((i + di) & (N as i32 - 1)) as usize]
                        ^ self.perm_y[((j + dj) & (N as i32 - 1)) as usize]
                        ^ self.perm_z[((k + dk) & (N as i32 - 1)) as usize])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm(perm: &mut [i32; N]) {
        for i in 0..N {
            perm[i] = i as i32;
        }

        Self::permute(perm);
    }

    fn permute(perm: &mut [i32; N]) {
        for i in (0..N).rev() {
            let target = random_int_range(0, i as i32) as usize;
            perm.swap(i, target);
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }
}
