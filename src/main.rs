extern crate pizza_injection;
use pizza_injection::*;

fn main() {
    let ny_store = PizzaStore::new(NYPizzaIngredientFactory::default(), "New York");
    let chicago_store = PizzaStore::new(ChicagoPizzaIngredientFactory::default(), "Chicago");

    println!("Ethan ordered a {:?}\n", ny_store.order_pizza(PizzaType::Cheese));
    println!("Joel ordered a {:?}\n", chicago_store.order_pizza(PizzaType::Cheese));
    println!("Ethan ordered a {:?}\n", ny_store.order_pizza(PizzaType::Clam));
    println!("Joel ordered a {:?}\n", chicago_store.order_pizza(PizzaType::Clam));
    println!("Ethan ordered a {:?}\n", ny_store.order_pizza(PizzaType::Pepperoni));
    println!("Joel ordered a {:?}\n", chicago_store.order_pizza(PizzaType::Pepperoni));
    println!("Ethan ordered a {:?}\n", ny_store.order_pizza(PizzaType::Veggie));
    println!("Joel ordered a {:?}\n", chicago_store.order_pizza(PizzaType::Veggie));
}
