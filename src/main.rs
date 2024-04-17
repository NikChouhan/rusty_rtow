mod vec;
use std::{fs::File,io::{BufWriter, Write}};
use vec::{Vec3, Color};

fn main(){ 
    const IMAGE_WIDTH : u64 = 256;
    const IMAGE_HEIGHT : u64 = 256;

    let file = File::create("image.ppm").expect("unable to create file");
    //Render

    let mut file = BufWriter::new(file);

    write!(file,"P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write");

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new((i as f64) / ((IMAGE_WIDTH-1) as f64),
                                               (j as f64) / ((IMAGE_HEIGHT-1) as f64),
                                               0.25);

        write!(file, "{} \n", pixel_color.format_color()).expect("Error in writing to the last line");
            
        }
    }  
    
}
