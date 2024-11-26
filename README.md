# Graficas_lab4
Laboratorio 4 de Gráficas por computadora: Shaders

---

## Screenshots demostrativos

![image](https://github.com/user-attachments/assets/d753c604-de2d-4cf6-8015-2093677d2a7d)

![image](https://github.com/user-attachments/assets/b6139aeb-e167-461d-b235-8c7fcecee8c4)

![image](https://github.com/user-attachments/assets/73238d47-c6fb-4782-8b87-ec16173ebfe3)

![image](https://github.com/user-attachments/assets/81178001-f781-40ce-a3c4-0b6c885d723e)

![image](https://github.com/user-attachments/assets/60e67f68-037a-48a0-bd55-66c2bc9aaf32)

![image](https://github.com/user-attachments/assets/19391410-915a-40c1-8707-355265afb22e)

![image](https://github.com/user-attachments/assets/2cc04a77-c2d1-45af-b4fd-7b6f71fb55bb)

![image](https://github.com/user-attachments/assets/7fe53918-dabf-4bec-99cc-19e4a39204ae)

![image](https://github.com/user-attachments/assets/1450aaea-8365-4aa4-be77-35602af4d441)

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
