# migraci-n-de-datos-csv-con-rust
 Implementación de proceso de migración de datos CSV a base de datos PostgreSQL con Rust
 Prerrequisitos
 - Tener instalado diesel
 Proceso
 - Modificar en el archivo .env, la variable DATABASEURL con la url de la base de datos que corresponda
 - Ejecutar los comandos 
 diesel setup
 diesel migration run
 cargo run
 Consideraciones
 El servidor se levanta en el puerto 8090
 
