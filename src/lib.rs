use edit_xlsx::{Format, FormatAlignType, FormatBorderType, FormatColor, Workbook, WorkbookResult, WorkSheet, Write};


struct Articulos {
    codigo: i64,
    nombre: String,
    precio: f64,
}

pub fn crear_plantilla_db() -> WorkbookResult<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.get_worksheet(1)?;
    workbook.save_as("resources/db.xlsx");
    Ok(())
}

pub fn agregar_articulo() -> WorkbookResult<()> {
    WorkSheet::write(&mut self, casilla, contenido)?;
    Ok(())
}


pub fn data_base() {
    print! ("hola mundo\n");
}
