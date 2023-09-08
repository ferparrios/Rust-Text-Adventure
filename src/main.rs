// use regex::Regex;
use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::fs;

const FILENAME: &str = "../history.csv";
const FIRST_TAG: &str = "INICIO";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
    opciones: Vec<DatoHistoria>,
    
}

impl DatoHistoria {
    fn new(row: StringRecord) -> DatoHistoria{
        let vida = row.get(3).unwrap().trim();
        let vida: i32 = vida.parse().unwrap_or(0);
        return DatoHistoria {
            tipo_dato: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            texto: row.get(2).unwrap().trim().to_string(),
            vida: vida,
            opciones: vec![]
        };
    }
}

// fn suma_uno(numero_a_sumar: i32) -> i32{
//     let numero_final = numero_a_sumar + 1;
//     println!("{}", numero_final);

//     return  numero_final;
// }

fn main() {
    let vida = 100;
    let mut tag_actual = FIRST_TAG;
    let mut last_record: String = "".to_string();
    let mut datos_historia: HashMap<String, DatoHistoria> = HashMap::new();
    let content = fs::read_to_string(FILENAME).unwrap();

    // println!("{}", content);

    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let dato = DatoHistoria::new(result);
        if dato.tipo_dato == "SITUACION" {
            let record_tag = dato.tag.clone();
            datos_historia.insert(record_tag.clone(), dato);
            last_record = record_tag;
        } else if dato.tipo_dato == "OPCION" {
            if let Some(data) = datos_historia.get_mut(&last_record){
                (*data).opciones.push(dato);
            }
        }
        // datos_historia.insert(dato.tag.clone(), dato);
    }

    // Game Loop

    loop {
        println!("Tienes {} de vida", vida);

        if let Some(data) = datos_historia.get(tag_actual){
            // println!("{:?}", data);
            println!("{}", data.texto);

            for (indice, option) in data.opciones.iter().enumerate() {
                println!("[{}] {}", indice, option.texto);
            }

            let mut seleccion = String::new();

            std::io::stdin().read_line(&mut seleccion).unwrap();
            let seleccion = seleccion.trim().parse().unwrap_or(99);

            if let Some(opcion_elegida) = &data.opciones.get(seleccion){
                tag_actual = &opcion_elegida.tag;
            } else {
                println!("Comando no valido!");
            }
            
        }else {
            break;
        }

        //  Si la vida <= 0 entonces terminar juego

        if vida <= 0 {
            println!("Has perdido!");
            break;
        }
    }

    println!("{:?}", datos_historia["DERECHA"]);

    // let mut diccionario: HashMap<String, String> = HashMap::new();
    // diccionario.insert("Manzana".to_string(), "Es una fruta de color rojo".to_string());
    // diccionario.insert("Pera".to_string(), "Es una fruta de color verde".to_string());

    // println!("La descriptcion de manzana es: {}", diccionario["Pera"]);



    // CLase de funciones
    // let mut minumero = 10;
    // minumero = minumero + 1;
    // let diez_mas_uno = suma_uno(10);
    // suma_uno(diez_mas_uno);
    // suma_uno(11);
    // suma_uno(12);

    

    // Clase del ciclo for y vectores

    // let mut nombres: Vec<String> = Vec::new();
    
    // for i in 0..3 {
    //     println!("Por favor introduce un nombre: ");
    //     let mut nombre = String::new();
    //     std::io::stdin().read_line(&mut nombre).unwrap();

    //     nombres.push(nombre);
    // }


    // // println!("{:?}", nombres)
    // // println!("{}",nombres[0]);
    // // println!("{}",nombres.len());

    // for nombre in nombres {
    //     println!("El nombre es: {}", nombre)
    // }

    // let hola = ["H", "O", "L", "A"];

    // println!("{}", hola[0]);
    // println!("{}", hola[1]);
    // println!("{}", hola[2]);
    // println!("{}", hola[3]);

    // println!("Calculadora");

    // // Regex
    // let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    // let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // // (\d+) \s? \+ \s? (\d+) 

    // // Traer datos del usuario
    // println!("Ingresa tu expresión: ");
    // let mut expression = String::new();
    // std::io::stdin().read_line(&mut expression).unwrap();

    // // Multiplicacion
    // loop {
    //     // Aplicar validaciones
    //     let caps = re_mult.captures(expression.as_str());

    //     if caps.is_none(){
    //         break;
    //     }

    //     let caps = caps.unwrap();

    //     let cap_expression = caps.get(0).unwrap().as_str();
    //     let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    //     let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    //     let mult = left_value * right_value;

    //     expression = expression.replace(cap_expression, &mult.to_string());
    //     // println!("{:?} izq: {}, der: {}", caps, left_value, right_value);    
    // }

    // // Suma

    // loop {
    //     // Aplicar validaciones
    //     let caps = re_add.captures(expression.as_str());

    //     if caps.is_none(){
    //         break;
    //     }

    //     let caps = caps.unwrap();

    //     let cap_expression = caps.get(0).unwrap().as_str();
    //     let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    //     let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    //     let addition = left_value + right_value;

    //     expression = expression.replace(cap_expression, &addition.to_string());
    //     // println!("{:?} izq: {}, der: {}", caps, left_value, right_value);    
    // }



    // Mostrar resultado
    // println!("Resultado: {}", expression);


    // if true {
    //     println!("Esto se va a cumplir siempre")
    // }else {
    //     println!("No se va a cumplir")
    // }



    // println!("Por favor introduce tu nombre: ");
    // let mut nombre: String = String::new();
    // std::io::stdin().read_line(&mut nombre).unwrap();
    // nombre = nombre.trim().to_string();

    // // Obtener la edad de la persona
    // println!("Por favor introduce tu edad");
    // let mut edad: String = String::new();
    // std::io::stdin().read_line(&mut edad).unwrap();

    // // Obtener el pais
    // println!("Por favor dime de que pais vienes");
    // let mut pais: String = String::new();
    // std::io::stdin().read_line(&mut pais).unwrap();

    // // Convertir esas edad a un numero
    // let edad_int: u8 = edad.trim().parse().unwrap();

    // if edad_int >= 18 && edad_int != 30 {
    //     println!("Puedes entrar a la discoteca");
    // } else if edad_int == 30{
    //     println!("No puedes entrar a la discoteca");
    // } else {
    //     println!("Eres menor de edad")
    // }

    // println!("Hola, bienvenido {} de {} años de {}", nombre, edad_int, pais);

    // Dos numeros a sumar

    // let numero_1 = 123;
    // let numero_2 = 321;

    // let suma = numero_1 + numero_2;

    // loop {
    //     // Mostrar los dos numeros en pantalla
    //     println!("Por favor escribir la suma de {} y {}", numero_1, numero_2);

    //     // Obtener del usuario el numero que presenta la suma
    //     let mut suma_usuario = String::new();
    //     std::io::stdin().read_line(&mut suma_usuario).unwrap();

    //     let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();

    //     if suma_usuario_int == suma {
    //         println!("Lo has hecho muy bien, el resultado {} es correcto", suma);
    //         break;
    //     } else {
    //         println!("El resultado {} no es correcto, por favor intentalo de nuevo", suma_usuario_int)
    //     }
    // }

    


}
