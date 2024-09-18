use syrette::{di_container_bind, DIContainer};

use crate::interfaces::printer::IPrinter;
use crate::printer::Printer;
use crate::r#struct::printer::StructPrinter;

pub fn bootstrap() -> DIContainer
{
    let mut di_container = DIContainer::new();

    di_container_bind!(IPrinter<String> => Printer, di_container);
    di_container_bind!(IPrinter<i32> => Printer, di_container);

    di_container_bind!(StructPrinter<String>, di_container);
    di_container_bind!(StructPrinter<i32>, di_container);


    di_container
}
