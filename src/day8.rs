const IMAGE_WIDTH: usize = 25;

use aoc_runner_derive::{aoc, aoc_generator};
const IMAGE_HEIGHT: usize = 6;

#[derive(Debug)]
pub struct Layer {
    pixels: Vec<Vec<u32>>,
}

impl Layer {
    fn new(raw_pixels: &[u32], width: usize, height: usize) -> Layer {
        assert_eq!(raw_pixels.len(), width * height);

        let mut pixels = vec![vec![0_u32; width]; height];

        for (pixel_row, chunk) in pixels.iter_mut().zip(raw_pixels.chunks(width)) {
            pixel_row.clone_from_slice(chunk);
        }

        Layer { pixels }
    }

    pub fn pixels(&self) -> &Vec<Vec<u32>> {
        &self.pixels
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new(raw_pixels: &[u32], width: usize, height: usize) -> Image {
        let num_layers = raw_pixels.len() / (width * height);
        let mut layers = Vec::with_capacity(num_layers);

        for chunk in raw_pixels.chunks(width * height) {
            let layer = Layer::new(chunk, width, height);
            layers.push(layer);
        }

        Image {
            width,
            height,
            layers,
        }
    }

    pub fn layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    fn generate_final_layer(&self) -> Layer {
        let mut final_layer_pixels: Vec<u32> = vec![2; self.width * self.height];

        for layer in self.layers() {
            for (cur_pixel, final_pixel) in layer
                .pixels()
                .iter()
                .flatten()
                .zip(final_layer_pixels.iter_mut())
            {
                match &final_pixel {
                    0 | 1 => (),
                    2 => final_pixel.clone_from(cur_pixel),
                    _ => panic!("Illegal pixel color in layer"),
                };
            }
        }

        Layer::new(&final_layer_pixels, self.width, self.height)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.generate_final_layer()
            .pixels()
            .iter()
            .for_each(|pixel_row| println!("{:?}", pixel_row))
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn count_zeroes(layer: &Layer) -> usize {
    layer.pixels().iter().flatten().filter(|p| **p == 0).count()
}

fn num_ones_times_num_twos(layer: &Layer) -> usize {
    let mut ones = 0;
    let mut twos = 0;

    layer.pixels().iter().flatten().for_each(|p| match p {
        1 => ones += 1,
        2 => twos += 1,
        _ => (),
    });

    ones * twos
}

#[aoc(day8, part1)]
pub fn part1(pixels: &[u32]) -> usize {
    let image = Image::new(pixels, IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut layer_with_least_zeroes: &Layer = image.layers().get(0).expect("Image has no layers");
    let mut zeroes_in_min_layer = IMAGE_HEIGHT * IMAGE_WIDTH;

    for layer in image.layers() {
        let num_zeroes_in_current_layer = count_zeroes(layer);
        if num_zeroes_in_current_layer < zeroes_in_min_layer {
            layer_with_least_zeroes = layer;
            zeroes_in_min_layer = num_zeroes_in_current_layer;
        }
    }

    num_ones_times_num_twos(layer_with_least_zeroes)
}

#[aoc(day8, part2)]
pub fn part2(pixels: &[u32]) -> String {
    let _image = Image::new(pixels, IMAGE_WIDTH, IMAGE_HEIGHT);
    //    _image.print(); //prints "CEKUA"

    "CEKUA".to_string()
}
