fn main(){
  println!("Hello, world from rust"); //función macro
}

// compilar
//$ rustc hello_world.rs
// esto genera un archivo ejecutable

//ejecutar programa
// $ ./hello_world

//error en windows
// corregirlo hace falta instalar "Visual C++ Build Tools 2015". Puedes descargarlo de acá https://visualstudio.microsoft.com/es/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16 una vez se instale este le dará un menú de opciónes de cuales complementos descargar, incialmente le recomiendo solo descargar "Desarrollo para el escritorio C++" dado que requiere algo de tiempo por su peso, a menos que tengas una buena velocidad de descarga. Pero en mi caso estoy en el tiempo gratuito del curso.

// manejador de packetes se llama cargo
// generar un proyecto con cargo
// cargo new <nameProject>
