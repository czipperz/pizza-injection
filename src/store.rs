use crate::factory::*;
use crate::pizza::*;

pub struct PizzaStore<F> {
    factory: F,
    style: String,
}
impl<F: PizzaIngredientFactory> PizzaStore<F> {
    pub fn new(factory: F, style: impl ToString) -> Self {
        Self {
            factory,
            style: style.to_string(),
        }
    }

    fn create_pizza(&self, pizza_type: PizzaType) -> Pizza {
        let creator = match pizza_type {
            PizzaType::Cheese => Pizza::make_cheese,
            PizzaType::Veggie => Pizza::make_veggie,
            PizzaType::Clam => Pizza::make_clam,
            PizzaType::Pepperoni => Pizza::make_pepperoni,
        };
        let mut pizza = creator(&self.factory);
        pizza.set_name(format!("{} {:?} Pizza", self.style, pizza_type));
        pizza
    }

    pub fn order_pizza(&self, pizza_type: PizzaType) -> Pizza {
        let mut pizza = self.create_pizza(pizza_type);
        println!("--- Making a {} ---", pizza.name());
        pizza.bake();
        pizza.cut();
        pizza.put_in_box();
        pizza
    }
}
