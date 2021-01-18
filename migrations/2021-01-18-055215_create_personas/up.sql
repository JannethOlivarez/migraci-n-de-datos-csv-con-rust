-- Your SQL goes here
CREATE TABLE "personas"(
id SERIAL NOT NULL PRIMARY KEY,
identificacion VARCHAR(13) NOT NULL,
nombre VARCHAR(255) NOT NULL,
genero VARCHAR(1) NOT NULL,
estado_civil VARCHAR(17) NOT NULL,
fecha_nacimiento VARCHAR NOT NULL,
telefono VARCHAR(10) NOT NULL,
direccion VARCHAR(255) NOT NULL,
email VARCHAR(50) NOT NULL,
estado INT NOT NULL,
observacion VARCHAR(600) NOT NULL
)