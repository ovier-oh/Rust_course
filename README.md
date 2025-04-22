# Rust Course 

Rust, ¡es un lenguaje poderoso y muy bueno para trabajar con rendimiento, ideal si más adelante quieres escalar a procesamiento de audio y ML!

Aquí tienes una lista de ideas simples y progresivas que te pueden ir encaminando a tu meta, todas orientadas a audio o AI, pero pensadas para principiantes en Rust:

1. __Calculadora en Consola__

    - __Descripción__: Un programa interactivo que permite al usuario realizar operaciones básicas como suma, resta, multiplicación y división entre dos números. Ideal para comenzar con entrada/salida y funciones.

    - __Funcionalidades__:

        - [ ] Menú: Sumar, Restar, Multiplicar, Dividir, Salir.

        - [ ] Leer dos números y una operación desde consola.

        - [ ] Validar división entre cero.

        - [ ] Funciones separadas para cada operación.

        - [ ] Mostrar resultado y volver al menú.

2. __Conversor de Temperaturas__

    - __Descripción__: Convierte temperaturas entre Celsius, Fahrenheit y Kelvin. Sirve para practicar lógica, condiciones y funciones numéricas.

    - __Funcionalidades__:

        - [ ] Menú con 4 tipos de conversión + opción Salir.

        - [ ] Leer temperatura y mostrar resultado convertido.

        - [ ] Validar que no sea menor a -273.15°C.

        - [ ] Funciones independientes por conversión.

        - [ ] Bucle hasta que el usuario elija salir.

3. __Contador de Palabras__

    - __Descripción__: Un programa que analiza una frase e imprime cuántas palabras contiene y cuántas veces aparece cada una (frecuencia).

    - __Funcionalidades__:

        - [ ] Leer una frase desde consola.

        - [ ] Dividir en palabras y contarlas.

        - [ ] Mostrar total de palabras.

        - [ ] Mostrar frecuencia de cada palabra con HashMap.

        - [ ] Ignorar mayúsculas/minúsculas (opcional).

3. __Generador de Onda Senoidal__

    - __Descripción__: Simula los valores de una onda senoidal a partir de frecuencia, duración y tasa de muestreo. Guarda los datos en un archivo .txt.

    - __Funcionalidades__:

        - [ ] Entrada: frecuencia, duración, sample rate.

        - [ ] Calcular valores con sin(2πft).

        - [ ] Mostrar los primeros 10 valores en pantalla.

        - [ ] Guardar todos los valores en un archivo .txt.

        - [ ] Validación de entradas numéricas.        

4. __Mini Banco de Señales__

    - __Descripción__: Permite guardar múltiples señales numéricas en archivos .txt, listarlas y eliminarlas. Pensado como base para futuros sistemas de audio.

    - __Funcionalidades__:

        - [ ] Menú: `Agregar`, `Ver`, `Eliminar`, `Salir`.

        - [ ] Guardar señales como listas de números en archivos.

        - [ ] Listar archivos guardados.

        - [ ] Eliminar señales por nombre.

        - [ ] Validar nombres duplicados.

6. __Visor ASCII de Señal__

    - __Descripción__: Muestra una gráfica simple en la terminal basada en números guardados en un .txt. Simula una "onda visual" con caracteres.

    - __Funcionalidades__:

        - [ ] Leer archivo .txt con números.

        - [ ] Escalar valores a longitud de línea.

        - [ ] Mostrar visualización con caracteres como #.

        - [ ] Validar contenido del archivo.

7. __Compresor de Texto (Run-Length Encoding)__

    - __Descripción__: Implementa un algoritmo simple de compresión, como RLE, donde secuencias repetidas se comprimen en el formato a3b2c1.

    - __Funcionalidades__:

        - [ ] Leer cadena del usuario.

        - [ ] Comprimirla con RLE.

        - [ ] Mostrar resultado comprimido.

        - [ ] Implementar también una función para descomprimir.

        - [ ] Validar entrada (sin espacios o caracteres raros).

8. __Lector de Archivos .txt Numéricos__

    - __Descripción:__ Programa que abre un archivo .txt que contiene una lista de números y calcula estadísticas como promedio, máximo y mínimo.

    - __Funcionalidades__:

        - [ ] Leer archivo línea por línea.

        - [ ] Convertir texto a números.

        - [ ] Calcular y mostrar: media, mínimo, máximo.

        - [ ] Validar errores de lectura o conversión.

9. Simulador de Audio Numérico (modo texto)

    - __Descripción__: Crea una señal con diferentes formas (senoidal, cuadrada, triangular) y la guarda en un .txt, sin generar audio real todavía.

    - __Funcionalidades__:

        - [ ] Menú para elegir tipo de onda.

        - [ ] Entrada: frecuencia, duración, sample rate.

        - [ ] Generar valores de la señal.

        - [ ] Guardar en archivo .txt.

        - [ ] Usar funciones por tipo de onda.

10. ___Menú de Utilidades para Señales__

    - __Descripción__: Interfaz de consola que une varios proyectos anteriores en un solo menú: generar onda, visualizar, guardar, eliminar, cargar y calcular estadísticas.

    - __Funcionalidades__:

        - [ ] Menú principal con todas las opciones: generar, guardar, cargar, visualizar, borrar, analizar.

        - [ ] Permitir seleccionar archivos de señales.

        - [ ] Modular todo usando funciones limpias.

        - [ ] Agregar confirmaciones y validaciones.

        - [ ] Ideal como primer “sistema completo” de consola.