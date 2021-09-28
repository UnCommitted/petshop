use petshop::pets::cat::Cat;
use petshop::pets::dog::Dog;

#[test]
fn test_dog_print_default() {
    let t_dog = Dog::default();
    assert_eq!(
        "Name: Unnamed, Species: Canine".to_string(),
        t_dog.to_string()
    );
}

#[test]
fn test_dog_print_new_no_name() {
    let t_dog = Dog::new(None);
    assert_eq!(
        "Name: Unnamed, Species: Canine".to_string(),
        t_dog.to_string()
    );
}

#[test]
fn test_dog_print_new_with_name() {
    let t_dog = Dog::new(Some("Rover".to_string()));
    assert_eq!(
        "Name: Rover, Species: Canine".to_string(),
        t_dog.to_string()
    );
}

#[test]
fn test_cat_print_default() {
    let t_cat = Cat::default();
    assert_eq!(
        "Name: Unnamed, Species: Feline".to_string(),
        t_cat.to_string()
    );
}

#[test]
fn test_cat_print_new_no_name() {
    let t_cat = Cat::new(None);
    assert_eq!(
        "Name: Unnamed, Species: Feline".to_string(),
        t_cat.to_string()
    );
}

#[test]
fn test_cat_print_new_with_name() {
    let t_cat = Cat::new(Some("Tiger".to_string()));
    assert_eq!(
        "Name: Tiger, Species: Feline".to_string(),
        t_cat.to_string()
    );
}
