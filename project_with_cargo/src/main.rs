/* nombre del proyecto deve estar escito en snake_case $ cargo new <name_snake_case>
en la raiz del proyecto ejecutar $ cargo build
esto crea un carpeta target/debug detro y el archivo ejecutable es el que tiene el mismo nombre que el proyecto
en la raiz del proyecto ejecuto $ cargo run
cargo run tbn puede compilar asi q no es necesario cargo build pero deveriamos de hacer la compilación
*/
fn main() {
  /*----------------------------------------------------------------------------
    $Variables
  ----------------------------------------------------------------------------*/

  // rust posee tipado fuerte pero se puede declarar variabes sin definir el tipo
  // por default todas las variables son inmutables no podemos numero_dos = 15 da error
  let name_variable_snake_case = 15;

  // i32 significa entero de 32 bits
  // para que sea mutable tenemos que agregarle mut
  let mut numero_dos: i32 = 10;
  // produce un warning pero no da error y normal compila
  numero_dos = 15;

  let resultado = name_variable_snake_case + numero_dos;

  //el juego d ellaves es remplazado por la variable
  println!("El valor de la variable es {}", name_variable_snake_case);
  // el remplazo de {} es de izquierda a derecha
  println!(
    "El resultado es {} + {} = {}",
    name_variable_snake_case, numero_dos, resultado
  );

  /*----------------------------------------------------------------------------
    $Constantes
  ----------------------------------------------------------------------------*/
  // constantes no es lo mismo que las variables inmutables
  //constantes declarar const o static
  // Es obliagtorio indicar el tipo de dato de la constante

  const NAME_CONST: i32 = 10;

  println!(
    "La constante es {} y la suma total es {} + {} + {} = {}",
    NAME_CONST, name_variable_snake_case, numero_dos, NAME_CONST, resultado
  )
}
