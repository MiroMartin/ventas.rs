use rusqlite::{Connection, Row, params};
use anyhow::Result;
use std::error::Error;

#[derive(Debug)]
pub struct Articulos {
    codigo: i64,
    nombre: String,
    precio: f64,
}

impl Articulos {
    pub fn from_row(row: &Row) -> Result<Self, Box<dyn Error>> {
        Ok(Articulos {
            codigo: row.get(0)?,
            nombre: row.get(1)?,
            precio: row.get(2)?,
        })
    }
}

pub fn crear_conexion(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    println!("Base de datos abierta: {}", db_path);
    Ok(conn)
}

pub fn crear_tabla(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articulos (
        codigo INTEGER PRIMARY KEY,
        nombre TEXT NOT NULL,
        precio FLOAT
    )",
    [],
    )?;

    println!("Tabla 'articulos' creada/verificada");
    Ok(())
}

pub fn insertar_articulos(conn: &Connection, codigo: i64, nombre: &str, precio: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO articulos (codigo, nombre, precio) VALUES (?1, ?2, ?3)",
        params![codigo, nombre, precio],
    )?;

    println!("Articulo '{}' insertado", nombre);
    Ok(())
}

//TODO: seguir agregando funciones (leer db, actualizar y eliminar)
