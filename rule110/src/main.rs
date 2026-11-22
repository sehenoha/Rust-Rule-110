


fn search <const N: usize>(a: [(u8,u8,u8);N], elt:(u8,u8,u8)) -> bool {
for i in 0..N {
    if a[i]==elt {
        return true;
    };
}
false

}

fn main() {
    play(10);
}

fn play(const N:u64){
    let mut data= [255; 3*(const N)*(const N)];
    let mut jeu_pair= [0;N+1]; //état du jeu après le tour n_iter si n_iter pair
    let mut jeu_impair= [0;N+1]; //état du jeu après le tour n_iter si n_iter impair
    
    //initialisation de la première ligne
    jeu_pair[(const N)-1]=1;
    for k in 0..3 {data[3*((const N)-1)+k]=255};
    
    //boucle
    for n_iter in 1..(const N) { 
    if n_iter%2==0 {
        //on traite le bord
        if jeu_impair[1]==1{
            jeu_pair[0]=1}
        else {jeu_pair[0]=0};
        
        for k in 0..3 {data[n_iter*3*(const N)+k]=255*jeu_pair[0]};
        
        for ind in 0..(const N) { 
            match (jeu_impair[ind-1],jeu_impair[ind],jeu_impair[ind+1]) {
                x if search([(1,1,1),(1,0,0),(0,0,0)],x) => jeu_pair[ind]=0,
                _ => jeu_pair[ind]=1,
            };
            for k in 0..3 {data[n_iter*3*(const N)+3*ind+k]=255*jeu_pair[ind]};
        };

    }
    else{ //on traite le bord
        if jeu_pair[1]==1{
            jeu_impair[0]=1}
        else {jeu_impair[0]=0};
        
        for k in 0..3 {data[n_iter*3*(const N)+k]=255*jeu_impair[0]};
        
        for ind in 0..(const N) { 
            match (jeu_pair[ind-1],jeu_pair[ind],jeu_pair[ind+1]) {
                x if search([(1,1,1),(1,0,0),(0,0,0)],x) => jeu_impair[ind]=0,
                _ => jeu_impair[ind]=1,
            };
            for k in 0..3 {data[n_iter*3*(const N)+3*ind+k]=255*jeu_impair[ind]}
        };

    };

    };
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

let path = Path::new(r"/path/to/image.png");
let file = File::create(path).unwrap();
let ref mut w = BufWriter::new(file);

let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
encoder.set_color(png::ColorType::Rgb);
encoder.set_depth(png::BitDepth::Eight);
encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
    (0.31270, 0.32900),
    (0.64000, 0.33000),
    (0.30000, 0.60000),
    (0.15000, 0.06000)
);
encoder.set_source_chromaticities(source_chromaticities);
let mut writer = encoder.write_header().unwrap();

writer.write_image_data(&data).unwrap(); // Save
}
    
