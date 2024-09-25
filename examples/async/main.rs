#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod animals;
mod bootstrap;
mod food;
mod interfaces;
mod generics;
mod singleton;

use std::sync::Arc;
use anyhow::Result;
use tokio::spawn;

use crate::bootstrap::bootstrap;
use crate::generics::{IPrinter, Printer};
use crate::interfaces::dog::IDog;
use crate::interfaces::food::IFoodFactory;
use crate::interfaces::human::IHuman;
use crate::singleton::{SomeSingleton, SomeTrait};

#[tokio::main]
async fn main() -> Result<()>
{

    println!("Hello, world!");

    let di_container = bootstrap().await?;

    {
        let dog = di_container
            .get::<dyn IDog>()
            .await?
            .threadsafe_singleton()?;

        dog.woof();
    }

    let food_factory = di_container
        .get::<IFoodFactory>()
        .await?
        .threadsafe_factory()?;

    let food = food_factory();

    food.eat();

    let z = di_container.get::<Printer<String>>()
        .await?.transient()?;
    z.print(&"Hello, Generic".to_string());

    let z = di_container.get::<dyn IPrinter<String>>()
        .await?.transient()?;
    z.print(&"Hello, Generic".to_string());

    di_container.get::<SomeSingleton>().await?.threadsafe_singleton()?.struct_increment();
    di_container.get::<dyn SomeTrait>().await?.threadsafe_singleton()?.trait_increment();


    spawn(async move {
        let human = di_container.get::<dyn IHuman>().await?.transient()?;

        human.make_pets_make_sounds();

        Ok::<_, anyhow::Error>(())
    })
    .await??;



    Ok(())
}
