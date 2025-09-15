use edit_xlsx::{Workbook, WorkbookResult};

use crate::lib::crear_plantilla_db;

pub mod lib;
fn main() -> WorkbookResult<()>{
    let _ = crear_plantilla_db();
    Ok(())
}
