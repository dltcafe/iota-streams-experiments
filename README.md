# IOTA streams experiments

Experimentos en rust con los streams de IOTA [https://wiki.iota.org/streams/libraries/rust/overview].

**Nota** A 16 de diciembre de 2021 la compilación de la librería en su versión 1.2.0 está rota por el (mal) uso del versionado semántico de algunos crates de bee.

Para compilar tenemos que fijar las versiones de dos crates en Cargo.toml.

Específicamente, fijamos las siguientes dependencias.

```toml
[dependencies]
tokio = { version = "1.1" }
anyhow = { version = "1.0", default-features = false }
iota-streams = { git = "https://github.com/iotaledger/streams", tag = "1.2.0" }
bee-message = "=0.1.5"
bee-rest-api = "=0.1.2"
```