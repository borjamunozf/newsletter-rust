

Rust - programación asíncrona se construye sobre Future trait.
- Expone el método **poll**, debe ser invocado para permitir al futuro progresar hasta que eventualmente sea resuelto a su futuro valor.
- La librería estándard no provee una *runtime* de ejecución asíncrono.
- 
Impl Responder


HttpResponse ya implementa Responder.

Form & FrmRequest
- Form is a wrapper generic over <T>
```rust
pub struct Form<T>(pub T)
```

Trait FromRequest contains extraction logic:
- URLEncoded::new - compresión y descomprensión de payloads. 
- serde_urlencoded::from_bytes::<T>(&body).map_err

T implementa DeserializedOwned de serde

Serde - deserialización y serialización de estructuras de datos de forma eficiente y genérica.
- Para JSON o Avro: serde_json o avro_rs
- Define una serie de interfaces o _data model_
- Serialize / deserialize trait

Monomorphization evita que serde sea más lento. El compilador de Rust crea una copia del cuerpo de la función reemplazando el tipo genérico por el tipo específco. Todo en tiempo de compilación, zero

## Bases de datos

tokio-postgres - tiempo de ejecución
sqlx - compilación
diesel - compilación

*Async*
- Hilos para trabajar en paralelo, asíncrono es para esperar en paralelo.
- Una base de datos asíncrona no va a reducir el tiempo de una consulta, pero permite aprovechar todos los cores de la CPU.
- Tiempos de ejecución asíncrono se basan en la asunción de que los futuros devolverán el control rápidamente al ejecutor. 

*sqlx-cli*

UNIQUE añade un B-Tree index en la columna asociada.

### App state en actix-web

Para persistir el estado de la aplicación, como la base de datos, utilizamos app-data para desvincularnos del ciclo de vida de una petición.
