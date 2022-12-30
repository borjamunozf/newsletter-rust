

Rust - programación asíncrona se construye sobre Future trait.
- Expone el método **poll**, debe ser invocado para permitir al futuro progresar hasta que eventualmente sea resuelto a su futuro valor.
- La librería estándard no provee una *runtime* de ejecución asíncrono.
- 
Impl Responder