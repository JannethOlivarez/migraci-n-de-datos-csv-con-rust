use crate::db;
use crate::error_handler::CustomError;
use crate::schema::personas;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

use regex::Regex;

use std::fmt;

use chrono::{DateTime, Local, NaiveDate,NaiveDateTime, Utc};
use chrono::format::ParseError;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "personas"]
pub struct Persona {
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estado_civil: String,
    pub fecha_nacimiento: String,
    pub telefono: String,
    pub direccion: String,
    pub email: String,
    pub estado: i32,
    pub observacion: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "personas"]
pub struct Personas {
    pub id: i32,
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estado_civil: String,
    pub fecha_nacimiento: String,
    pub telefono: String,
    pub direccion: String,
    pub email: String,
    pub estado: i32,
    pub observacion: String,
}


impl Personas {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let personas = personas::table.load::<Personas>(&conn)?;
        Ok(personas)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = personas::table.filter(personas::id.eq(id)).first(&conn)?;
        Ok(persona)
    }

    pub fn create(persona: Persona) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = Persona::from(persona);
        let persona = diesel::insert_into(personas::table)
            .values(persona)
            .get_result(&conn)?;
        Ok(persona)
    }

    pub fn update(id: i32, persona: Persona) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = diesel::update(personas::table)
            .filter(personas::id.eq(id))
            .set(persona)
            .get_result(&conn)?;
        Ok(persona)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(personas::table.filter(personas::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    pub fn validar_persona( mut identificacion_data: String,
        nombre_data: String,
        genero_data: String,
        estado_civil_data: String,
        fecha_nacimiento_data: String,
        telefono_data: String,
        direccion_data: String,
        email_data: String) -> Persona{
        let mut observaciones = "".to_owned();
        let mut valido=1;

        let mut identificacion = &identificacion_data;
        let mut email = &email_data;
        let mut direccion = &direccion_data;
        let mut fecha_nacimiento = &fecha_nacimiento_data;
        //if numerodocumento(identificacion_data)
        if Self::numerodocumento(String::from(identificacion))==false{
            observaciones = observaciones +"Cédula no válida \n"
        }
        if Self::validar_correo(email)==false{
            observaciones = observaciones +"Email no válida \n"
        }
        if Self::validar_direccion(direccion)==false{
            observaciones = observaciones +"Dirección no válida \n"
        }
        if Self::validar_edad(fecha_nacimiento)==false{
            observaciones = observaciones +"Edad no válida \n"
        }

        //println!("{:?}", observaciones);

        if observaciones!= ""{
            valido=0;
        }
        println!("{} {}", observaciones,valido);

        return Persona::from_data( String::from(identificacion),
            nombre_data,
            genero_data,
            estado_civil_data,
            String::from(fecha_nacimiento),
            telefono_data,
            String::from(direccion),
            String::from(email),
                    valido,
                    observaciones
                 ) ;
    

    }
    // validacion de documento de identificacion
    pub fn numerodocumento(mut numerodoc: String)-> bool{
      let cantidad= numerodoc.len();
      //println!("Documento {}",numerodoc);
      if cantidad==10{
          return Self::verificador(numerodoc);           
      } else  if cantidad==13{
          return Self::validacionruc(numerodoc);
      }else{
          return false;
      }
    
    }
    //validacion de ruc
    pub fn validacionruc(ruc:String)-> bool{
      let ultimostres=&ruc[10..13];
      if ultimostres=="001"
      {
          let diezprimeros=&ruc[0..10];        
          return Self::verificador(diezprimeros.to_string());
      }
      else
      {
          return false;
    
      }
    
    }
    //numero verificador
    pub fn verificador(mut cedula: String) -> bool{
        let cantidad= cedula.len();
       if cantidad ==10
       {
           let tercerdigitos = &cedula[2..3];
           let tercerdigitoe= tercerdigitos.parse::<u32>().unwrap();
           if tercerdigitoe<6
           {
               
               let coefvalcedula = vec![2, 1, 2, 1, 2, 1, 2, 1, 2];
               let decimodigitos = &cedula[9..10];
               let verificador= decimodigitos.parse::<u32>().unwrap();
               let mut suma=0;
               let mut digito=0;
               for i in 1..9
               {
                   digito =&cedula[i.. i + 1].parse::<u32>().unwrap()* coefvalcedula[i];
                   suma += (digito % 10) + (digito / 10);
               }
               if (suma % 10 == 0) && (suma % 10 == verificador) {
                   return true;
                  }
                  else if 10 - (suma % 10) == verificador {
                   return true;
                 } else {
                   return true;
                  }
           }
           else
           {
           return false;
           }
       }
       else
       {
          return false;
       }
      }

    pub fn validar_correo(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
        }
        RE.is_match(text)
        
    } 
    pub fn validar_direccion(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\b[A-Za-z0-9\s'_-]+\b){2,}").unwrap();
        }
        RE.is_match(text)
        
    }
    
    pub fn validar_edad(text: &str) -> bool {
        let now = Local::now();
        //let owned_string: String = "hello ".to_owned();
        //et another_owned_string: String = " 0:0:0".to_owned();
        let borrowed_string: &str = " 0:0:0";
        
        let together = text.to_owned() + borrowed_string;
        
        let naive_dt = NaiveDateTime::parse_from_str(&together, "%Y-%m-%d %H:%M:%S").unwrap();
        let other_dt = DateTime::<Utc>::from_utc(naive_dt, Utc);
    
        let diff = now.signed_duration_since(other_dt);
        //let days =Duration::days(diff);
    
        diff.num_days()/365>8
        
    }
}

impl Persona {
    fn from(persona: Persona) -> Persona {
        Persona {
            identificacion: persona.identificacion,
            nombre: persona.nombre,
            genero: persona.genero,
            estado_civil: persona.estado_civil,
            fecha_nacimiento: persona.fecha_nacimiento,
            telefono: persona.telefono,
            direccion: persona.direccion,
            email: persona.email,
            estado: persona.estado,
            observacion: persona.observacion
        }
    }
    pub fn from_data(identificacion_data: String,nombre_data: String,genero_data: String,estado_civil_data: String,
                     fecha_nacimiento_data: String,telefono_data: String,direccion_data: String,email_data: String,
                     estado_data: i32,observacion_data: String) -> Persona {
        Persona {
            identificacion: identificacion_data,
            nombre: nombre_data,
            genero: genero_data,
            estado_civil: estado_civil_data,
            fecha_nacimiento: fecha_nacimiento_data,
            telefono: telefono_data,
            direccion: direccion_data,
            email: email_data,
            estado: estado_data,
            observacion: observacion_data,
        }
    }
}



