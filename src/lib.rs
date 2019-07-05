use image::{GrayImage, GrayAlphaImage, RgbaImage};
use imageproc::{filter, morphology, distance_transform::Norm};
use rand::Rng;


pub struct GeneratedAsteroid {
    pub layers: Vec<(GrayImage, u8)>,
    pub colored_img: Option<RgbaImage>,
    pub combined_img: Option<GrayAlphaImage>,
    pub layer_size: (u32, u32),
}

impl GeneratedAsteroid {
    pub fn smoothen_all(mut self, smooth_parameter: u8) -> Self {
        for (img, _) in self.layers.iter_mut() {
            morphology::dilate_mut(img, Norm::LInf, smooth_parameter);
        }
        self
    }

    pub fn save_layers(self, base_name: &str) -> Self {
        let mut tmp_img = GrayAlphaImage::new(self.layer_size.0, self.layer_size.1);
        for pix in tmp_img.pixels_mut() {
            *pix = image::LumaA([0, 0]);
        }
        for (i, (img, col)) in self.layers.iter().enumerate() {
            img.enumerate_pixels().filter(|(x, y, p)| p[0] != 0).for_each(|(x, y, p)| {
                tmp_img.put_pixel(x, y, image::LumaA([255, 255]));
            });
            tmp_img.save(format!("{}{}.png", base_name, i)).unwrap();
        }
        self
    }

    pub fn save_gray(mut self, name: &str) -> Self {
        if self.combined_img.is_none() {
            self = self.combine_gray();
        }
        self.combined_img = self.combined_img.map(|img| {
            img.save(name).unwrap();
            img
        });
        self
    }

    pub fn combine_gray(mut self) -> Self {
        let mut gray_img = match self.combined_img {
            Some(p) => p,
            None => GrayAlphaImage::new(self.layer_size.0, self.layer_size.1),
        };
        for pix in gray_img.pixels_mut() {
            *pix = image::LumaA([0, 0]);
        }
        for (i, (img, col)) in self.layers.iter().enumerate().rev() {
            img.enumerate_pixels().filter(|(x, y, p)| p[0] != 0).for_each(|(x, y, p)| {
                gray_img.put_pixel(x, y, image::LumaA([*col, 255]));
            });
        }
        self.combined_img = Some(gray_img);
        self
    }

    pub fn combine_colored(mut self, hue: Option<[u8; 3]>) -> Self {
        let mut col_img = match self.colored_img {
            Some(p) => p,
            None => RgbaImage::new(self.layer_size.0, self.layer_size.1),
        };

        let hue = match hue {
            Some(c) => image::Rgb([c[0] as f32, c[1] as f32, c[2] as f32]),
            None => image::Rgb([255.0, 255.0, 255.0]),
        };

        for pix in col_img.pixels_mut() {
            *pix = image::Rgba([0, 0, 0, 0]);
        }
        for (i, (img, col)) in self.layers.iter().enumerate().rev() {
            let fac = *col as f32 / 255.0;
            let color = [(fac * hue[0]) as u8, (fac * hue[1]) as u8, (fac * hue[2]) as u8];
            img.enumerate_pixels().filter(|(_x, _y, p)| p[0] != 0).for_each(|(x, y, _p)| {
                col_img.put_pixel(x, y, image::Rgba([color[0], color[1], color[2], 255]));
            });
        }
        self.colored_img = Some(col_img);
        self
    }

    pub fn save_colored(mut self, name: &str) -> Self {
        if self.colored_img.is_none() {
            self = self.combine_colored(None);
        }
        self.colored_img = self.colored_img.map(|img| {
            img.save(name);
            img
        });
        self
    }

    pub fn blur_gray(mut self, sigma: f32) -> Self {
        if self.combined_img.is_none() {
            self = self.combine_gray();
        }
        self.combined_img = Some(filter::gaussian_blur_f32(&self.combined_img.unwrap(), sigma));
        self
    }

    fn blur_colored(mut self, sigma: f32) -> Self {
        self
    }
}

pub fn generate(area: u32, bands: usize, axis: Option<(f32, f32)>) -> GeneratedAsteroid {
    let mut ax = (0.0, 0.0);
    let push_fac = if let Some(vec) = axis {
        ax = norm(vec);
        push_factor
    } else {
        fake_push_factor
    };

    let size = (2.5 * area as f32).sqrt() as i32;
    let mut img = GrayImage::new(size as u32, size as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Luma([0]);
    }

    let mut counter: u32 = 0;
    let mut rng = rand::thread_rng();
    let center = (size as f32 / 2.0, size as f32 / 2.0);
    let mut queue: Vec<(u32, u32)> = Vec::with_capacity((area / 2) as usize);
    queue.push(((size / 2) as u32, (size / 2) as u32));

    let mut img_vec: Vec<(GrayImage, u8)> = Vec::with_capacity(bands); 
    let mut band_idx = 0;
    let mut band_img = GrayImage::new(size as u32, size as u32);
    for p in band_img.pixels_mut() {
        *p = image::Luma([0]);
    }
    let mut next_band = area / bands as u32;
    let mut band_color = 255;

    for _ in 0..area {
        if queue.len() == 0 {
            break;
        }
        let (x, y) = loop {
            let idx = rng.gen_range(0, queue.len());
            let (x, y) = queue.remove(idx);
            if img.get_pixel(x, y)[0] == 0 {
                break (x, y)
            } else {
            }
        };
        img.put_pixel(x, y, image::Luma([255]));
        band_img.put_pixel(x, y, image::Luma([255]));
        counter += 1;
        if counter > next_band {
            next_band += area / bands as u32;
            if next_band > area {
                break;
            }
            img_vec.push((band_img, band_color));
            band_idx += 1;
            band_img = GrayImage::new(size as u32, size as u32);
            for p in band_img.pixels_mut() {
                *p = image::Luma([0]);
            }
            band_color = 255 - band_idx * (255 / bands as u8);
        }
        let (ix, iy) = (x as i32, y as i32);
        for dy in -1 as i32 ..=1 {
            for dx in -1 as i32 ..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }
                if ix + dx < 0 || ix + dx >= size || iy + dy < 0 || iy + dy >= size {
                    continue
                }
                if img.get_pixel((ix + dx) as u32, (iy + dy) as u32)[0] == 255 {
                    continue;
                }
                let vec = (x as f32 - center.0, y as f32 - center.1);
                let r = rand::random::<f32>();
                let pf = push_fac(vec, ax);
                if r < pf {
                    queue.push(((ix + dx) as u32, (iy + dy) as u32));
                }
            }
        }
    }
    img_vec.push((band_img, band_color));

    GeneratedAsteroid {
        layers: img_vec,
        colored_img: None,
        combined_img: None,
        layer_size: (img.width(), img.height())
    }
}

fn dot(v1: (f32, f32), ax: (f32, f32)) -> f32 {
    (v1.0 * ax.0 + v1.1 * ax.1)
}

fn push_factor(v1: (f32, f32), ax: (f32, f32)) -> f32 {
    if mag(v1) < 0.001 {
        return 1.1;
    }
    dot(norm(v1), ax).abs().max(0.45)
}

fn norm(v: (f32, f32)) -> (f32, f32) {
    let mag = mag(v);
    (v.0 / mag, v.1 / mag)
}

fn mag(v: (f32, f32)) -> f32 {
    dot(v, v).sqrt()
}

fn fake_push_factor(v1: (f32, f32), ax: (f32, f32)) -> f32 {
    1.1
}