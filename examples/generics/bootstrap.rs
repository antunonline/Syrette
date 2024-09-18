use syrette::{di_container_bind, DIContainer};

use crate::interfaces::printer::IPrinter;
use crate::printer::Printer;
use crate::generic_struct::printer::GenericStructPrinter;

pub fn bootstrap() -> DIContainer
{
    let mut di_container = DIContainer::new();

    di_container_bind!(IPrinter<String> => Printer, di_container);
    di_container_bind!(IPrinter<i32> => Printer, di_container);

    di_container_bind!(GenericStructPrinter<String>, di_container);
    di_container_bind!(GenericStructPrinter<i32>, di_container);


    di_container
}
