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

/*
fn lum_hline(line: &[u8]) -> [(f32,usize); H_SIZE/COLOURS]{
    let mut lum_line: [(f32,usize); H_SIZE/COLOURS] = [(0.0,0); H_SIZE/COLOURS];
    for i in 0..H_SIZE/COLOURS{
        let pixel: usize = i*COLOURS;
        lum_line[i] = (luminance(line[pixel], line[pixel+1], line[pixel+2]), i);
    }
    return lum_line;
}*/

fn conmask(mut r: u8, mut g: u8, mut b: u8, minval: f32, maxval: f32) -> [u8;3] {
    let l:f32 = luminance(r,g,b);
    if l < minval || l > maxval{r = 0; g = 0; b = 0}
    return [r,g,b];
}

fn main(){
    let mut pixelvect = Vec::new();
    let mut file_in = File::open("raw_img.np").expect("Unable to open file");
    file_in.read_to_end(&mut pixelvect).expect("Unable to read data");
    println!("{}", pixelvect.len());

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

    println!("{}",count);

    let mut segcount: i32 = 0;

    let mut count: usize = 0;
    for _i in 0..V_SIZE{
        let mut seg:        bool = false;
        let mut seg_start: usize = 0;
        let mut seg_end:   usize;
        for _j in 0..H_SIZE{
            //check to see if at the start of a segment
            if  seg == false{
                if  !(pixelvect[count] == 0 && pixelvect[count+1] == 0 && pixelvect[count+2] == 0){
                        seg = true;
                        seg_start = count; 
                    }
            }
            else{
                //  if within segment, check if end of segment                                 or end of line                
                    if pixelvect[count] == 0 && pixelvect[count+1] == 0 && pixelvect[count+2] == 0 || (count + COLOURS) % H_SIZE == 0{
                        seg = false;
                        seg_end = count;

                        segcount += 1;

                        //println!("original{:?}", &pixelvect[seg_start..seg_end]);

                        //sorting logic (bubble sort)
                        //let seg_size: usize    = seg_end-seg_start;
                        //let mut unsorted: bool = true;
                        let mut sorted: usize  = 1; 
                        for _sort in (seg_start..seg_end).step_by(3){
                            for pos in (seg_start..seg_end-sorted).step_by(3){
                                let next: [u8;3] = [pixelvect[pos+3], pixelvect[pos+4], pixelvect[pos+5]];
                                if luminance(pixelvect[pos  ], pixelvect[pos+1], pixelvect[pos+2])
                                 > luminance(next         [0], next         [1], next         [2]){
                                    pixelvect[pos+3] = pixelvect[pos];
                                    pixelvect[pos+4] = pixelvect[pos+1];
                                    pixelvect[pos+5] = pixelvect[pos+2];

                                    pixelvect[pos]   = next[0];
                                    pixelvect[pos+1] = next[1];
                                    pixelvect[pos+2] = next[2];

                                    sorted+=1;
                                }
                            }
                        }
                        //println!("sorted {:?}", &pixelvect[seg_start..seg_end]); 
                    }
            }
            count += COLOURS;
        }
    };
    
    println!("{}", segcount);

    let mut file_out = File::create("processed_img.np").expect("Unable to write file");
    file_out.write_all(&pixelvect).expect("Unable to write data to file");
}