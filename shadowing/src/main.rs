fn main() {
  let name_var: i32 = 10;
  println!("Variable inicial es {}", name_var);

  /* shadowing permite volver a declarar una variable con el mismo valor
  lo que sucede es que el compilador elimina la anterior variable y
  crea una nueva variable con el mismo nombre pero pude crearse con diferente
  tipo
  Las variables siguen siendo inmutables
  podemos hacer shadowng cuantas veces queramos
  */
  let name_var = false;

  println!(
    "La variable inicial se destruye y se crea una nueva variable con
    el mismo nombre pero diferente tipo {}",
    name_var
  )
}
