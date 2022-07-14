use crate::{basic_component::vec3::Vec3, texture::Texture, utility::random_int};

#[derive(Clone, Copy)]
pub struct Perlin {
    pub perm_x: [i32; 256],
    pub perm_y: [i32; 256],
    pub perm_z: [i32; 256],
    pub ranvec: [Vec3; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut _ranvec: [Vec3; 256] = [Vec3::new(0., 0., 0.); 256];
        //用随机的单位向量来代替随机的double，可以进一步避免格子化的出现
        for i in 0..256 {
            _ranvec[i] = Vec3::unit_vector(Vec3::random(-1., 1.));
        }

        Self {
            perm_x: Perlin::perlin_generate_from(),
            perm_y: Perlin::perlin_generate_from(),
            perm_z: Perlin::perlin_generate_from(),
            ranvec: _ranvec,
        }
    }

    pub fn perlin_generate_from() -> [i32; 256] {
        // 生成一个随机交换的数组
        let mut p: [i32; 256] = [0; 256];
        for i in 0..256 {
            let t = i as usize;
            p[t] = i;
        }
        Perlin::permute(p, 256);

        p
    }

    pub fn permute(mut p: [i32; 256], n: i32) {
        //随机交换
        for i in (0..n).rev() {
            let t = i as usize;
            let target = random_int(0, i) as usize;
            let tmp = p[t];
            p[t] = p[target];
            p[target] = tmp;
            // swap(&mut p[t], &mut p[target]);
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = (p.x.floor()) as i32;
        let j = (p.y.floor()) as i32;
        let k = (p.z.floor()) as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2]; //三维数组c[2][2][2]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Perlin::perlin_interp(c, u, v, w)
    }

    pub fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        //三维线性化处理，让纹理更平滑

        // Hermitian Smoothing
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let mut accum = 0.;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * Vec3::dot(c[i][j][k], weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        //模拟多个波叠加下的真实声波
        let mut accum = 0.;
        let mut tmp = p;
        let mut weight = 1.;

        for _i in 0..depth {
            accum += weight * Perlin::noise(&self, tmp);
            weight *= 0.5;
            tmp *= 2.;
        }

        accum.abs()
    }
}

//------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64, //控制频率大小
}

impl Texture for NoiseTexture {
    fn get_color_value(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        //计算灰度？
        Vec3::new(1., 1., 1.) * 0.5 * (1. + (self.scale * p.z + 10. * self.noise.turb(p, 7)).sin())
        //利用正弦函数模拟出大理石的纹理
    }
}
