use crate::personas::{Persona, Personas};
use crate::error_handler::CustomError;

extern crate csv;

use actix_web::{delete, get, post, put, web, HttpResponse, Result,Error};
use serde_json::json;
//librerias para cargar el un archivo
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
//librerias leer csv

use csv::ReaderBuilder;
use csv::StringRecord;



#[get("/personas")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let personas = Personas::find_all()?;
    Ok(HttpResponse::Ok().json(personas))
}

#[get("/personas/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let persona = Personas::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(persona))
}

#[post("/personas")]
async fn create(persona: web::Json<Persona>) -> Result<HttpResponse, CustomError> {
    let persona = Personas::create(persona.into_inner())?;
    Ok(HttpResponse::Ok().json(persona))
}

#[put("/personas/{id}")]
async fn update(id: web::Path<i32>,persona: web::Json<Persona>,) -> Result<HttpResponse, CustomError> {
    let persona = Personas::update(id.into_inner(), persona.into_inner())?;
    Ok(HttpResponse::Ok().json(persona))
}

#[delete("/personas/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_persona = Personas::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_persona })))
}

#[post("/personas/csv")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  //recorre la lista de archivo
  while let Ok(Some(mut field)) = payload.try_next().await {
    // Extrae la data del array de bytes de cada archivo
      while let Some(chunk) = field.next().await {
          let data = chunk.unwrap();
          // los bytes se transforman a string para que luego la libreria de csv los pueda leer
          let csv = String::from_utf8_lossy(&data);
          // lee el string de la data
          let mut rdr = ReaderBuilder::new()
          .has_headers(false)
          .delimiter(b';').from_reader(csv.as_bytes());
          // extrae los registros del reader
          let records = rdr
          .records()
          .collect::<Result<Vec<StringRecord>, csv::Error>>();
          //recorre los registros
          for result in records {
            let record = result;
            // todos los datos se almacenan el primer registro
            // se recorre el primer registro para optener cada uno de los datos
            for x in 0..record.len() {
              // se extraer cada registro individualmente para procesarlo
              if let Some(value) =  record.get(x){
         
                let _persona = Personas::validar_persona(
                    String::from(value.get(0).unwrap()),
                    String::from(value.get(1).unwrap()).to_uppercase(),
                    String::from(value.get(2).unwrap()).to_uppercase(),
                    String::from(value.get(3).unwrap()).to_uppercase(),
                    String::from(value.get(4).unwrap()).to_uppercase(),
                    String::from(value.get(5).unwrap()).to_uppercase(),
                    String::from(value.get(6).unwrap()).to_uppercase(),
                    String::from(value.get(7).unwrap()).to_uppercase());
                  Personas::create(_persona)?;
              }

          }
        }
      }
  }
  Ok(HttpResponse::Ok().into())
}
pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
    comfig.service(upload);
}
