# Graficas_lab4
Laboratorio 4 de Gráficas por computadora: Shaders

---

## Screenshots demostrativos

---

## Descripción del laboratorio

Este proyecto implementa un renderizador 3D en tiempo real para planetas utilizando el lenguaje de programación Rust. La aplicación simula diferentes objetos celestiales, como planetas rocosos, gigantes gaseosos y estrellas, generados de forma procedimental utilizando funciones de ruido y shaders personalizados.

El pipeline de renderizado es completamente personalizado, implementado en Rust, sin utilizar texturas externas ni materiales: todos los efectos visuales son generados matemáticamente.

---

## Dependencias/Instalación/Ejecución

### Asegúrate de incluir las siguientes dependencias en tu archivo Cargo.toml:

```toml
Copy code
[dependencies]
nalgebra-glm = "0.5"
minifb = "0.6"
fastnoise-lite = "0.1"
rand = "0.8"
```

### Instalación y Configuración

Clona el Repositorio:
```bash
Copy code
git clone https://github.com/nel-eleven11/Graficas_lab4
cd Graficas_lab4
```
Compila el Proyecto: Ejecuta el siguiente comando para compilar el proyecto:
```bash
Copy code
cargo build --release
```

Ejecuta la Aplicación:
```bash
Copy code
cargo run --release
```

---

## Uso

Controles de Cámara:
- Flechas: Rotar la cámara alrededor de la escena.
- A/D: Mover la cámara hacia la izquierda/derecha.
- Q/E: Mover la cámara hacia arriba/abajo.
- Arriba/Abajo: Hacer zoom in/out.

Cambio de Shader:
- Presiona C para cambiar entre diferentes shaders.

Salir de la Aplicación:
- Presiona ESC para cerrar la ventana.

---