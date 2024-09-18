mod bootstrap;
mod interfaces;
mod printer;
mod generic_struct;

use std::error::Error;

use crate::bootstrap::bootstrap;
use crate::generic_struct::printer::GenericStructPrinter;
use crate::interfaces::printer::IPrinter;

fn main() -> Result<(), Box<dyn Error>>
{
    let di_container = bootstrap();

    let string_printer = di_container.get::<dyn IPrinter<String>>()?.transient()?;

    string_printer.print("Hello there".to_string());

    let int_printer = di_container.get::<dyn IPrinter<i32>>()?.transient()?;

    int_printer.print(2782028);

    let generic_struct_string_printer = di_container.get::<GenericStructPrinter<String>>()?.transient()?;
    generic_struct_string_printer.print(&"Hello Concrete".to_string());

    let generic_struct_i32_printer = di_container.get::<GenericStructPrinter<i32>>()?.transient()?;
    generic_struct_i32_printer.print(&10);

    Ok(())
}
