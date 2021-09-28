use petshop::animals::Animal;
use petshop::pets::cat::Cat;
use petshop::pets::dog::Dog;
fn main() {
    let mut pet_vec: Vec<Box<dyn Animal>> =
        vec![Box::new(Dog::default()), Box::new(Cat::default())];
    for pet in pet_vec.iter_mut() {
        println!("{}", pet);
        println!("{}", pet.species());
        pet.set_name(Some("blah".to_string()));
        println!("{}", pet);
    }
}
