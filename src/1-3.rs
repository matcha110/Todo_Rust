fn main(){
    // let mut x = 5;
    // println!("numeber is: {}", x);
    // x = 6;
    // println!("numeber is: {}", x);
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    fn lifespan(&self) -> u32;
    fn scientific_name(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis Lupus familiaris".to_string()
    }
}

struct Cat;
impl Animal for Cat{
    fn lifespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

fn show_animal_data<T: Animal>(animal: T){
    println!("Lifespan:{} years", animal.lifespan());
    println!("scientific_name:{} ", animal.scientific_name());
}


