use edit_xlsx::{Format, FormatAlignType, FormatBorderType, FormatColor, Workbook, WorkbookResult, WorkSheet, Write};


struct Articulos {
    codigo: i64,
    nombre: String,
    precio: f64,
}

//crear una plantilla en caso de que no exista
pub fn crear_plantilla_db() -> WorkbookResult<()> {
    let mut workbook = Workbook::new();
    let mut worksheet = workbook.get_worksheet(1)?;
    workbook.save_as("resources/db.xlsx");
    Ok(())
}

//Agregar texto a una cuadrilla de la plantilla 
pub fn agregar_informacion(mut worksheet: WorkSheet, casilla: &str, contenido: &str) -> WorkbookResult<()> {
    worksheet.write(casilla, contenido);
    Ok(())
}


pub fn data_base() {
    print! ("hola mundo\n");
}
