use crate::db::*;
use anyhow::Result;

mod db;

const DB_PATH: &str = "data/inventario.db";

fn main() -> Result<()>{

    let db_path = DB_PATH;
    let conn = crear_conexion(db_path)?;

    //crear tabla
    crear_tabla(&conn)?;

    insertar_articulos(&conn, 1111, "pan", 15.50)?;

    Ok(())
}


