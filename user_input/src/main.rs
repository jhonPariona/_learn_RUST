use std::io;

fn main() {
  println!("Ingrese su nombre");
  //la variable debe ser de tipo string ya que la fx read_line recive como argumento un string
  let mut name_user = String::new(); //new es un método estático de la clase string

  // & significa que estamos prestando la variable pero la queremos de vuela
  // readline retorna un Result(objeto que puede recibir exito o error) por ello sale un warning si no definimos el error
  io::stdin().read_line(&mut name_user).unwrap(); // vamos  a prestar(&) con permisos de mutabilidad name:user por referencia

  //uso de shadowing para eliminar el salto de linea al presionar enter
  let name_user = name_user.trim();

  println!("ingrese su edad");

  let mut edad = String::new();
  io::stdin().read_line(&mut edad).unwrap();
  // parse tbn retorna un objeto Result
  let edad = edad.trim();
  // parse intye que debe convertir a int ya que edad es de tipo int
  //unwrap obtiene el caso de exito
  let edad: i32 = edad.parse().unwrap();

  println!("Hello {} tienes {} años", name_user, edad);
}
