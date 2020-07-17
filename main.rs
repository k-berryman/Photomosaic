extern crate image;
use image::{GenericImageView, DynamicImage, Pixel};

fn main()
{
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("C:/Users/Owner/Desktop/Photomosaic_Pics/water.JPG").unwrap();




    // The dimensions method returns the images width and height.
    //   println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    //  println!("{:?}", img.color());

    let img = img.rotate180();

    // println!("{:?}", img.color());

    let mut r_total : u64 = 0;
    let mut g_total : u64 = 0;
    let mut b_total : u64 = 0;
    let mut num_pix : u64 = 0;

    for p in img.pixels() {
        r_total += (p.2).0[0] as u64;
        g_total += (p.2).0[1] as u64;
        b_total += (p.2).0[2] as u64;
        num_pix += 1;

    //println!("{:?}, {:?}, {:?}", r_avg, g_avg, b_avg);

    img.save("test.png").unwrap();
}

fn cal_avg(pixels : Vec<(u32, u32, R)>) -> u64 {
    let mut r_total : u64 = 0;
    let mut g_total : u64 = 0;
    let mut b_total : u64 = 0;
    let mut num_pix : u64 = 0;

    for p in iter {
        r_total += (p.2).0[0] as u64;
        g_total += (p.2).0[1] as u64;
        b_total += (p.2).0[2] as u64;
        num_pix += 1;
        //println!("{:?}, {:?}, {:?}, {:?}", p, (p.2).0[0], (p.2).0[1], (p.2).0[2]);
    }

    let r_avg = r_total / num_pix;
    let g_avg = g_total / num_pix;
    let b_avg = b_total / num_pix;
}
