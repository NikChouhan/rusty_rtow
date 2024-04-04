use std::{fs::File,io::{BufWriter, Write}};

fn main(){ 
    const IMAGE_WIDTH : u32 = 256;
    const IMAGE_HEIGHT : u32 = 256;

    let file = File::create("image.ppm").expect("unable to create file");
    //Render

    let mut file = BufWriter::new(file);

    write!(file,"P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write");

    for j in 0..IMAGE_WIDTH {
        for i in 0..IMAGE_HEIGHT {
            let r : f32 = (i as f32)/(IMAGE_WIDTH-1) as f32;
            let g : f32 = (j as f32)/(IMAGE_HEIGHT-1) as f32;
            let b: f32 = 0.0;

            let ir : u32 = (255.99 * r) as u32;
            let ig : u32 = (255.99 * g) as u32;
            let ib : u32 = (255.99 * b) as u32; 

        write!(file, "{} {} {} \n", ir, ig, ib).expect("Error in writing to the last line");
            
        }
    }  
    
}
