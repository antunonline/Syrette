use syrette::ptr::TransientPtr;
use syrette::AsyncDIContainer;

use crate::animals::cat::Cat;
use crate::animals::dog::Dog;
use crate::animals::human::Human;
use crate::food::Food;
use crate::generics::{IPrinter, Printer, PrinterConfig, SomethingElse};
use crate::interfaces::cat::ICat;
use crate::interfaces::dog::IDog;
use crate::interfaces::food::{IFood, IFoodFactory};
use crate::interfaces::human::IHuman;
use crate::singleton::{SomeSingleton, SomeTrait};

pub async fn bootstrap() -> Result<AsyncDIContainer, anyhow::Error>
{
    let mut di_container = AsyncDIContainer::new();

    di_container
        .bind::<dyn IDog>()
        .to::<Dog>()?
        .in_singleton_scope()
        .await?;

    di_container.bind::<dyn ICat>().to_dynamic_value(&|_| {
        Box::new(|| {
            let cat: TransientPtr<dyn ICat> = TransientPtr::new(Cat::new());

            cat
        })
    })?;

    di_container.bind::<dyn IHuman>().to::<Human>()?;

    di_container.bind::<IFoodFactory>().to_factory(&|_| {
        Box::new(|| {
            let food: Box<dyn IFood> = Box::new(Food::new());

            food
        })
    })?;

    di_container.bind::<SomethingElse>().to::<SomethingElse>()?;
    di_container.bind::<PrinterConfig<String>>().to::<PrinterConfig<String>>()?;
    di_container.bind::<Printer<String>>().to::<Printer<String>>()?;
    di_container.bind::<dyn IPrinter<String>>().to::<Printer<String>>()?;

    di_container.bind::<SomeSingleton>().to::<SomeSingleton>()?.in_singleton_scope().await?;
    di_container.bind::<dyn SomeTrait>().to::<SomeSingleton>()?.in_singleton_scope_from_existing().await?;


    Ok(di_container)
}
