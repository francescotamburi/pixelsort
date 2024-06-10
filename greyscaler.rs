use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::env;

const COLOURS: usize = 3;
const V_SIZE: usize = 4309;
const H_SIZE: usize = 2868;

fn luminance(r: u8, g: u8, b: u8) -> f32{
    let l: u32 = 299 * r as u32 +
                 587 * g as u32 +
                 114 * b as u32;
    let l: f32 = l as f32 / 255000.0;
    return l;
}


fn conmask(mut r: u8, mut g: u8, mut b: u8, minval: f32, maxval: f32) -> [u8;3] {
    let l:f32 = luminance(r,g,b);
    if l < minval || l > maxval{r = 0; g = 0; b = 0}
    return [r,g,b];
}

/*
fn rev_conmask(mut r: u8, mut g: u8, mut b: u8, minval: f32, maxval: f32) -> [u8;3] {
    let l:f32 = luminance(r,g,b);
    if l > minval && l < maxval{r = 0; g = 0; b = 0}
    return [r,g,b];
}*/

fn monochrome(pix: [u8;3], r: u8, g: u8, b: u8) -> [u8;3] {
    let l = luminance(pix[0],pix[1],pix[2]);
    let pix_lum = [l * r as f32, l * g as f32, l * b as f32];
    return[pix_lum[0] as u8, pix_lum[1] as u8, pix_lum[2] as u8];
}

fn main(){
    let mut pixelvect = Vec::new();
    let mut file_in = File::open("raw_img.np").expect("Unable to open file");
    file_in.read_to_end(&mut pixelvect).expect("Unable to read data");
    println!("{}", pixelvect.len());

    let og = pixelvect.clone();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let minval: f32 = args[1].parse().expect("First arg not float");
    let maxval: f32 = args[2].parse().expect("Second arg not float");

    let mut count: usize = 0;
    for _i in 0..V_SIZE{
        for _j in 0..H_SIZE{
            let rgb = conmask(pixelvect[count],pixelvect[count+1],pixelvect[count+2], minval, maxval);
            pixelvect[count]   = rgb[0];
            pixelvect[count+1] = rgb[1];
            pixelvect[count+2] = rgb[2];
            count += COLOURS;
        };
    };

    let mut count: usize = 0;
    for _i in 0..V_SIZE{
        for _j in 0..H_SIZE{
            if !(pixelvect[count] == 0 && pixelvect[count+1] == 0 && pixelvect[count+2] == 0){
                let rgb = monochrome([pixelvect[count],pixelvect[count+1],pixelvect[count+2]], 112,250,193);
                pixelvect[count]   = rgb[0];
                pixelvect[count+1] = rgb[1];
                pixelvect[count+2] = rgb[2];
            }
            count += COLOURS;
        };
    };

    let mut count: usize = 0;
    for _i in 0..V_SIZE{
        for _j in 0..H_SIZE{
            if pixelvect[count] == 0 && pixelvect[count+1] == 0 && pixelvect[count+2] == 0{
                pixelvect[count]   = og[count];
                pixelvect[count+1] = og[count+1];
                pixelvect[count+2] = og[count+2];
            }
            count += COLOURS;
        };
    };

    let mut file_out = File::create("processed_img.np").expect("Unable to write file");
    file_out.write_all(&pixelvect).expect("Unable to write data to file");
}