use crate::factory::*;
use crate::ingredients::*;
use std::fmt;

#[derive(Debug)]
pub enum PizzaType {
    Cheese,
    Veggie,
    Clam,
    Pepperoni,
    Sausage,
}

pub struct Pizza {
    name: String,
    dough: Option<Dough>,
    sauce: Option<Sauce>,
    veggies: Vec<Veggies>,
    cheese: Option<Cheese>,
    pepperoni: Option<Pepperoni>,
    sausage: Option<Sausage>,
    clams: Option<Clams>,
}

impl Pizza {
    pub fn bake(&mut self) {
        println!("Bake for 25 minutes at 350");
    }

    pub fn cut(&mut self) {
        println!("Cutting the pizza into diagonal slices");
    }

    pub fn put_in_box(&mut self) {
        println!("Place pizza in official PizzaStore box");
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cheese(dough: Dough, sauce: Sauce, cheese: Cheese) -> Self {
        Pizza {
            name: String::new(),
            dough: Some(dough),
            sauce: Some(sauce),
            veggies: Vec::new(),
            cheese: Some(cheese),
            pepperoni: None,
            sausage: None,
            clams: None,
        }
    }

    pub fn make_cheese(factory: &impl PizzaIngredientFactory) -> Self {
        Self::cheese(
            factory.create_dough(),
            factory.create_sauce(),
            factory.create_cheese(),
        )
    }

    pub fn veggie(dough: Dough, sauce: Sauce, cheese: Cheese, veggies: Vec<Veggies>) -> Self {
        Pizza {
            name: String::new(),
            dough: Some(dough),
            sauce: Some(sauce),
            veggies,
            cheese: Some(cheese),
            pepperoni: None,
            sausage: None,
            clams: None,
        }
    }

    pub fn make_veggie(factory: &impl PizzaIngredientFactory) -> Self {
        Self::veggie(
            factory.create_dough(),
            factory.create_sauce(),
            factory.create_cheese(),
            factory.create_veggies(),
        )
    }

    pub fn clam(dough: Dough, sauce: Sauce, cheese: Cheese, clams: Clams) -> Self {
        Pizza {
            name: String::new(),
            dough: Some(dough),
            sauce: Some(sauce),
            veggies: Vec::new(),
            cheese: Some(cheese),
            pepperoni: None,
            sausage: None,
            clams: Some(clams),
        }
    }

    pub fn make_clam(factory: &impl PizzaIngredientFactory) -> Self {
        Self::clam(
            factory.create_dough(),
            factory.create_sauce(),
            factory.create_cheese(),
            factory.create_clams(),
        )
    }

    pub fn pepperoni(
        dough: Dough,
        sauce: Sauce,
        cheese: Cheese,
        veggies: Vec<Veggies>,
        pepperoni: Pepperoni,
    ) -> Self {
        Pizza {
            name: String::new(),
            dough: Some(dough),
            sauce: Some(sauce),
            veggies,
            cheese: Some(cheese),
            pepperoni: Some(pepperoni),
            sausage: None,
            clams: None,
        }
    }

    pub fn make_pepperoni(factory: &impl PizzaIngredientFactory) -> Self {
        Self::pepperoni(
            factory.create_dough(),
            factory.create_sauce(),
            factory.create_cheese(),
            factory.create_veggies(),
            factory.create_pepperoni(),
        )
    }

    pub fn sausage(
        dough: Dough,
        sauce: Sauce,
        cheese: Cheese,
        veggies: Vec<Veggies>,
        sausage: Sausage,
    ) -> Self {
        Pizza {
            name: String::new(),
            dough: Some(dough),
            sauce: Some(sauce),
            veggies,
            cheese: Some(cheese),
            pepperoni: None,
            sausage: Some(sausage),
            clams: None,
        }
    }

    pub fn make_sausage(factory: &impl PizzaIngredientFactory) -> Self {
        Self::sausage(
            factory.create_dough(),
            factory.create_sauce(),
            factory.create_cheese(),
            factory.create_veggies(),
            factory.create_sausage(),
        )
    }
}

impl fmt::Debug for Pizza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "---- {} ----\n", self.name)?;
        if let Some(dough) = &self.dough {
            write!(f, "{:?}\n", dough)?;
        }
        if let Some(sauce) = &self.sauce {
            write!(f, "{:?}\n", sauce)?;
        }
        if let Some(cheese) = &self.cheese {
            write!(f, "{:?}\n", cheese)?;
        }
        for i in 0..self.veggies.len() {
            write!(f, "{:?}", self.veggies[i])?;
            if i < self.veggies.len() - 1 {
                write!(f, ", ")?;
            } else {
                write!(f, "\n")?;
            }
        }
        if let Some(sausage) = &self.sausage {
            write!(f, "{:?}\n", sausage)?;
        }
        if let Some(clams) = &self.clams {
            write!(f, "{:?}\n", clams)?;
        }
        if let Some(pepperoni) = &self.pepperoni {
            write!(f, "{:?}\n", pepperoni)?;
        }
        Ok(())
    }
}
