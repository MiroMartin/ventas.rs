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
        "INSERT OR IGNORE INTO articulos (codigo, nombre, precio) VALUES (?1, ?2, ?3)",
        params![codigo, nombre, precio],
    )?;

    println!("Articulo '{}' insertado", nombre);
    Ok(())
}

pub fn get_articulos(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT nombre FROM articulos")?;
    let rows = stmt.query_map([], |row| row.get(0))?;

    let mut nombre = Vec::new();
    for name_result in rows {
        nombre.push(name_result?);
    }
    Ok(nombre)
}

pub fn update_articulo_precio(conn: &Connection, codigo: i64, nuevo_precio: f64) -> Result<()> {
    conn.execute(
        "UPDATE articulos SET precio = ?1 WHERE codigo = ?2",
        params![nuevo_precio, codigo],
        )?;

    println!("Precio del articulo codigo {} actualizado a {}", codigo, nuevo_precio);
    Ok(())
}

pub fn delete_articulo(conn: &Connection, codigo: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM articulos WHERE codigo = ?1",
        params![codigo],
        )?;

        println!("Articulo {} eliminado", codigo);
    Ok(())
}

//TODO: implementar el TUI para mostrar el inventario
