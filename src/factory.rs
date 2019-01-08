use crate::ingredients::*;

pub trait PizzaIngredientFactory {
    fn create_dough(&self) -> Dough;
    fn create_sauce(&self) -> Sauce;
    fn create_cheese(&self) -> Cheese;
    fn create_veggies(&self) -> Vec<Veggies>;
    fn create_pepperoni(&self) -> Pepperoni;
    fn create_clams(&self) -> Clams;
}

#[derive(Default)]
pub struct NYPizzaIngredientFactory {}
impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_dough(&self) -> Dough {
        Dough::ThinCrust
    }
    fn create_sauce(&self) -> Sauce {
        Sauce::Marinara
    }
    fn create_cheese(&self) -> Cheese {
        Cheese::Reggiano
    }
    fn create_veggies(&self) -> Vec<Veggies> {
        vec![
            Veggies::Garlic,
            Veggies::Onion,
            Veggies::Mushroom,
            Veggies::RedPepper,
        ]
    }
    fn create_pepperoni(&self) -> Pepperoni {
        Pepperoni::Sliced
    }
    fn create_clams(&self) -> Clams {
        Clams::Fresh
    }
}

#[derive(Default)]
pub struct ChicagoPizzaIngredientFactory {}
impl PizzaIngredientFactory for ChicagoPizzaIngredientFactory {
    fn create_dough(&self) -> Dough {
        Dough::ThickCrust
    }
    fn create_sauce(&self) -> Sauce {
        Sauce::PlumTomato
    }
    fn create_cheese(&self) -> Cheese {
        Cheese::Mozzarella
    }
    fn create_veggies(&self) -> Vec<Veggies> {
        vec![Veggies::BlackOlives, Veggies::Spinach, Veggies::Eggplant]
    }
    fn create_pepperoni(&self) -> Pepperoni {
        Pepperoni::Sliced
    }
    fn create_clams(&self) -> Clams {
        Clams::Frozen
    }
}
