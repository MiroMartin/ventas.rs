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
    insertar_articulos(&conn, 1112, "cafe", 15.30)?;
    insertar_articulos(&conn, 1113, "leche", 12.50)?;

    //leer datos
    let articulos = get_articulos(&conn)?;
    println!("\nArticulos en la base de datos");
    for articulo in &articulos {
        println!("{:?}", articulos);
    }

    //actualizar datos
    update_articulo_precio(&conn, 1112, 16.00)?;
    println!("\nNuevo precio del articulo:");

    let articulo_actualizado = get_articulos(&conn)?;
    for articulo in articulo_actualizado {
        println!("{:?}", articulo);
    }

    //eliminar un articulo
    delete_articulo(&conn, 1113)?;
    println!("\nArticulo eliminado");

    let final_articulos = get_articulos(&conn)?;
    for articulo in final_articulos {
        println!("{:?}", articulo);
    }

    Ok(())
}


