pub fn eq(x: char, y: char) -> bool {
    x == y
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    x + y + z
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}

fn main(){
    println!("{}, {} and {}", eq('s','f'), add(4.0,5.0,6.0), cast(4,5,7.8));    
}