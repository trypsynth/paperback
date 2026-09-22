<!-- machine-translated from doc/readme.md (source-hash: 651d0b411879a6d8; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ce87a64f,a9eba369,e9860ee8,007c0542); please review and edit as needed -->

# Paperback - versión 1.0

## Introducción

Paperback es un lector ligero, rápido y accesible para libros electrónicos, documentos y audiolibros, para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para accesibilidad con lectores de pantalla, velocidad rápida y una experiencia sin excesos.

## Requisitos del Sistema

Paperback funciona en Windows 10/11, todas las versiones modernas de ARM macOS, Linux, iOS 17 y posteriores, y Android 7 y posteriores. Las aplicaciones iOS y Android están disponibles en la App Store y Google Play.

## Características

* Completamente independiente, sin requerir que instales software en tu computadora para comenzar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz simple con pestañas, que te permite abrir tantos documentos como desees lado a lado.
* Guarda tu posición exacta de lectura en cada documento que abres.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura al siguiente inicio.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente por los documentos.
* Incluye un diálogo de búsqueda robusto, con características como historial y soporte de expresiones regulares.
* Puede ejecutarse completamente de forma portátil o instalarse con asociaciones de archivo configuradas automáticamente.
* Admite una gran variedad de formatos de archivo comunes.
* Reproduce audiolibros, con velocidad ajustable y marcadores que recuerdan el tiempo exacto.
* Lee páginas PDF escaneadas con el OCR integrado en Windows y macOS.
* Marcadores y notas, para que puedas marcar tu lugar y volver a él.
* Cada atajo de teclado puede ser personalizado.
* Viene con `pb`, una herramienta de línea de comandos que convierte cualquier documento compatible a HTML, Markdown o texto plano.

## Compatibilidad con Lectores de Pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, hay un problema conocido para los usuarios de JAWS.

### JAWS y Pantallas Braille

Si usas JAWS con una pantalla braille, es posible que encuentres que los párrafos largos se truncan cuando se desplazan hacia adelante con las teclas de navegación de tu pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en el manejo de JAWS del control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tardó bastante tiempo en salir a la luz una solución dada el entusiasmo de Vispero por responder a problemas con software de código abierto.

La solución alternativa, finalmente encontrada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla se mantendrá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente compatibles

Paperback admite los siguientes formatos y extensiones:

* Archivos de cómics (`.cbz`)
* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` como BSD `mdoc` (`.1` a `.9`, `.man`, `.roff`, y las formas comprimidas de cada una)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos de Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolibros M4B (`.m4b`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Audiolibros MP3 (`.mp3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Documentos de Windows Write (`.wri`)
* Archivos WinHelp (`.hlp`)
* Archivos de texto plano y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para usar el teclado como elemento principal. Aquí están los atajos actuales.

Los atajos que aparecen a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left`/`Alt+Right` ya están reservados por otras convenciones del sistema o de la aplicación en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abre un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cierra el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cierra todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabre el último documento cerrado.
* `Ctrl+R`: Muestra el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Sale (solo en Windows; en macOS está en el menú de la aplicación).

### Menú Ir

* `Ctrl+F`: Muestra el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Busca el siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Busca el anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a la línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir al porcentaje.
* `Ctrl+P`: Ir a la página (cuando lo admita el documento actual).
* `=`: Anuncia tu porcentaje de lectura actual y página, p. ej. "15%, página 30". La página se omite para documentos sin números de página.
* `Alt+Left` (macOS: `Cmd+[`): Retrocede en el historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Avanza en el historial de navegación.
* `[`: Sección anterior.
* `]`: Sección siguiente.
* `Shift+H`: Encabezado anterior.
* `H`: Encabezado siguiente.
* `Shift+1` a `Shift+6`: Encabezado anterior del nivel 1-6.
* `1` a `6`: Encabezado siguiente del nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Página siguiente.
* `Shift+B`: Marcador anterior.
* `B`: Marcador siguiente.
* `/`: Establece tu marcador temporal.
* `\`: Salta a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Salta a todos los marcadores y notas.
* `Ctrl+Alt+B`: Salta solo a marcadores.
* `Ctrl+Alt+M`: Salta solo a notas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir, la tecla Control física en lugar de Cmd): Ve el texto de nota en la posición actual.
* `Shift+K`: Enlace anterior.
* `K`: Enlace siguiente.
* `Shift+G`: Imagen anterior.
* `G`: Imagen siguiente.
* `Shift+F`: Figura anterior.
* `F`: Figura siguiente.
* `Shift+T`: Tabla anterior.
* `T`: Tabla siguiente.
* `Shift+M`: Fórmula anterior.
* `M`: Fórmula siguiente.
* `Shift+S`: Separador anterior.
* `S`: Separador siguiente.
* `Shift+L`: Lista anterior.
* `L`: Lista siguiente.
* `Shift+I`: Elemento de lista anterior.
* `I`: Elemento de lista siguiente.
* `Shift+,`: Ir al inicio del contenedor actual (lista o tabla).
* `,`: Ir más allá del final del contenedor actual (lista o tabla).

### Menú Herramientas

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Muestra el recuento de palabras del documento actual.
* `Ctrl+I`: Muestra información del documento.
* `Ctrl+T`: Muestra la tabla de contenidos.
* `F7`: Muestra la lista de elementos.
* `Ctrl+Shift+C`: Abre la carpeta contenedora.
* `Ctrl+Shift+V`: Abre el contenido actual en Vista web.
* `Ctrl+U`: Ve la fuente del documento en una pestaña nueva.
* `Ctrl+Shift+E`: Exporta datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importa datos del documento (`.paperback`).
* `Ctrl+E`: Exporta el documento actual a texto sin formato.
* `Ctrl+Shift+B`: Alterna el marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añade o edita una nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alterna el ajuste de palabras.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, es decir, la tecla Control física, ya que `Cmd+Space` abre Spotlight): Reproduce/pausa la narración de audio.
* `'`: Busca hacia adelante en la narración de audio.
* `;`: Busca hacia atrás en la narración de audio.
* `Shift+'`: Aumenta la cantidad de búsqueda de audio.
* `Shift+;`: Reduce la cantidad de búsqueda de audio.
* `Ctrl+Shift+.`: Acelera la narración de audio.
* `Ctrl+Shift+,`: Ralentiza la narración de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Comando+F): Alterna pantalla completa.
* `Ctrl+,`: Abre Configuración (macOS: en el menú de la aplicación).
* `Ctrl+Shift+S`: Alterna el temporizador de reposo.
* `Ctrl+Shift+O`: Reconoce un rango de páginas PDF escaneadas con OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Marca el inicio de una selección, para que todo desde aquí hasta donde llegues pueda copiarse de una sola vez.
* `Alt+F10` (macOS: `Cmd+F10`): Copia todo desde el inicio marcado de la selección hasta la posición actual.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Salta al inicio marcado de la selección, dejando la marca en su lugar.

### Menú Ayuda

* `Ctrl+F1`: Muestra el diálogo Acerca de.
* `F1`: Ve la ayuda en tu navegador predeterminado.
* `Shift+F1`: Ve la ayuda en Paperback.
* `Ctrl+Shift+U`: Comprueba si hay actualizaciones.
* `Ctrl+D`: Abre la página de donaciones en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cierra la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Sigue un enlace o abre una vista de tabla o fórmula en el cursor.
* `Enter` en una página PDF escaneada: Reconoce la página con OCR.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abre el menú contextual.

## iOS y Android

Las aplicaciones de iOS y Android utilizan el mismo motor de lectura que la versión de escritorio, por lo que abren los mismos formatos y recuerdan tu lugar de la misma manera. Están diseñadas para usarse con VoiceOver en iOS y TalkBack en Android.

### Abrir documentos

* Utiliza el botón Open Book, o abre un documento desde la aplicación Files u otra aplicación y elige Paperback.
* En Android, puedes activar el navegador de archivos integrado en Settings. Requiere permiso de All Files Access y abre archivos grandes directamente en lugar de copiarlos primero.
* Mantén presionado el botón Open Book para importar o exportar los datos de un documento (`.paperback`), los mismos archivos que utiliza la aplicación de escritorio.

### Leer y escuchar

Cada aplicación tiene dos formas de leer un documento. En modo texto, lees el texto con tu lector de pantalla. En modo lectura en voz alta, Paperback te lee el texto con la voz que selecciones en Settings, y continúa en segundo plano y desde la pantalla de bloqueo. Cambia entre ellos desde el menú More Options.

Los audiolibros, como DAISY, M4B y libros MP3, reproducen su propia grabación en su lugar.

### La barra de lectura

La barra en la parte inferior de la pantalla tiene, de izquierda a derecha:

* La unidad de navegación, como párrafo, encabezado, página o enlace. Desliza hacia arriba o hacia abajo en ella para cambiarla.
* Botones de anterior, reproducir y siguiente. Los botones anterior y siguiente se mueven por la unidad de navegación.
* La velocidad del habla. Desliza hacia arriba o hacia abajo en ella para cambiar la velocidad de lectura de Paperback.

También puedes deslizar hacia arriba o hacia abajo en el botón de reproducir para moverte por la unidad de navegación, sin tener que alcanzar los botones de anterior y siguiente. Si eso es todo lo que usas, la opción Hide previous and next buttons los elimina del acceso de tu lector de pantalla. La opción Swipe up moves forward elige la dirección de un deslizamiento.

### Más opciones

El menú More Options es donde está todo lo demás. Algunos elementos funcionan de manera un poco diferente en cada aplicación.

* **Switch to TTS Mode o Switch to Text Mode:** cambia entre el modo de lectura en voz alta y el modo de texto, descritos anteriormente. Android también tiene un elemento Read Aloud que inicia y pausa la lectura en voz alta.
* **Table of Contents:** los capítulos del libro. Elige uno para ir directamente a él. En Android, las entradas que tienen capítulos bajo ellas se pueden expandir y contraer usando las acciones del lector de pantalla. En iOS, se muestra la lista completa de una vez.
* **Elements:** una lista de los encabezados o enlaces del documento. Cambia entre los dos con el Type picker en iOS, o las pestañas en Android, luego elige uno para ir a él.
* **Find:** escribe lo que buscas y elige si deseas coincidir mayúsculas/minúsculas, coincidir solo palabras completas o usar una expresión regular. En Android, una barra con Find Previous y Find Next permanece en la parte inferior de la pantalla hasta que la cierres, y las búsquedas anteriores están bajo Search History. En iOS, los botones Find Previous y Find Next están en la pantalla Find, y Find también aparece como una unidad de navegación en la barra de lectura, para que puedas avanzar por las coincidencias desde ahí.
* **Go To:** salta a una línea, una página o un porcentaje del documento. Elige cuál con el Mode picker.
* **Recent Documents:** todos los documentos que has abierto, cada uno marcado como actualmente abierto, cerrado o archivo no encontrado. Cada uno tiene dos acciones del lector de pantalla: Remove lo elimina de la lista, y Locate te permite encontrar un documento cuyo archivo se ha movido. Clear Recent Documents vacía la lista sin eliminar ningún documento.
* **Word Count:** el número de palabras en el documento.
* **Document Info:** el título, el autor, el nombre de archivo, y en iOS también los conteos de líneas y caracteres.
* **Export:** guarda el documento como texto plano, HTML o Markdown.
* **Sleep Timer:** detiene la lectura después de 5, 10, 15, 30, 45 o 60 minutos, o un tiempo personalizado. Ábrelo de nuevo mientras se está ejecutando para ver cuánto tiempo queda, o para cancelarlo.
* **Help:** abre este archivo readme.
* **Settings:**
    * **Text to speech:** la voz, velocidad del habla y tono, un botón Play Sample para escucharlos, y la pausa entre párrafos. Android también te permite elegir el motor de síntesis de voz. En iOS, aquí también está el diccionario de habla: reglas que cambian la forma en que se pronuncian las palabras, para todas las voces o solo algunas.
    * **Readability:** tamaño del texto, espaciado de línea, espaciado de párrafo y alineación. iOS también tiene apariencia clara y oscura, y texto de alto contraste.
    * **Behavior:** si reabre tus documentos cuando se inicia la aplicación, la dirección que un deslizamiento en el botón de reproducir mueve, y si ocultar los botones de anterior y siguiente. Android también tiene el navegador de archivos integrado aquí.

### Teclados y auriculares

Con un teclado, los atajos del escritorio para abrir libros, documentos recientes, Find, Go To, la tabla de contenidos, conteo de palabras, información del documento, exportar y el temporizador de sueño funcionan todos, usando `Cmd` en lugar de `Ctrl` en iOS. También funcionan las teclas de una sola letra para moverte por encabezado, página, enlace y demás, y `Space` reproduce y pausa. En iOS, las teclas de una sola letra solo llegan a Paperback mientras Quick Nav de una sola letra de VoiceOver está desactivado.

En Android, un botón de auricular reproduce y pausa con una pulsación, avanza con dos y retrocede con tres.

## Idiomas compatibles

Paperback se traduce a muchos idiomas diferentes, y se añaden más constantemente. A continuación se presenta una lista completa.

Para aprender cómo contribuir, por favor lee nuestra [Guía de traducción](translating.md).

* Bosnio
* Checo
* Holandés
* Finlandés
* Francés
* Alemán
* Japonés
* Polaco
* Portugués (Brasil)
* Ruso
* Chino simplificado
* Serbio
* Español
* Ucraniano
* Vietnamita

## Créditos
### Desarrollo
* Quin Gillespie: desarrollador principal y fundador del proyecto.
* Aryan Choudhary: contribuidor principal.

### Donaciones
Las siguientes personas han realizado donaciones de diversos montos al desarrollo de Paperback. Si haces una donación, tu nombre no se añadirá automáticamente aquí; solo añado a las personas que desean que su donación sea pública.

Nota: Considero que ser un patrocinador público de GitHub es motivo suficiente para la inclusión automática en esta lista.

* Alex Hall
* Brandon McGinty
* Brian Hartgen
* Debbie Yuille
* Devin Prater
* Felix Steindorff
* Hamish Mackenzie
* James Scholes
* Jayson Smith
* Jonathan Rodriguez
* Jonathan Schuster
* Keao Wright
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Registro de cambios

### Versión 1.0

1.0 es el primer lanzamiento en las cinco plataformas: Windows, macOS, Linux, iOS y Android, con las aplicaciones de iOS y Android en App Store y Google Play.

#### Añadido

##### General
* Soporte para Linux, como AppImage o tar.gz, con integración de escritorio para que los documentos se abran desde tu gestor de archivos.
* Marca el principio de una selección con `Alt+F9`, copia todo desde ahí hasta donde hayas llegado con `Alt+F10`, y vuelve a la marca con `Alt+Shift+F9`, para copiar un largo fragmento de texto sin tener que pasar por él con mayúscula+flecha. Los tres están bajo Herramientas > Seleccionar y copiar.
* El atajo `=` ahora anuncia la página además del porcentaje, p. ej. "15%, página 30", y se mantiene como estaba para documentos sin números de página.
* El cuadro Acerca de ahora muestra la licencia de Paperback y todos los traductores.
* Una traducción al ucraniano.

##### Nuevos formatos
* Archivos de cómics (`.cbz`).
* Audiolibros M4B, divididos en sus capítulos.
* Páginas de manual, tanto `man` como `mdoc` de BSD, comprimidos o no.
* Audiolibros MP3, divididos en capítulos cuando el archivo los contiene.
* Archivos de Windows Write (`.wri`).
* Archivos de WinHelp (`.hlp`).
* Documentos de Word 6 y Word 95.

##### OCR
* Las páginas de PDF escaneadas ahora pueden reconocerse con el OCR integrado en Windows y macOS. Presiona `Enter` en una página escaneada para reconocerla, o usa OCR por lotes (`Ctrl+Shift+O`) para un rango de páginas.

##### Navegación
* Las fórmulas MathML en EPUB e HTML se renderizan como AsciiMath usando MathCAT. Usa `M` o `Shift+M` para navegar fórmulas, luego `Enter` o `Space` para abrir el MathML original en Vista de fórmula.
* Un botón Buscar todo en el diálogo Buscar, listando cada línea con una coincidencia para que puedas saltar directamente a la que quieras.
* Vistas de Tablas, Listas y Páginas en la lista de elementos (`F7`).
* Ir a línea, Ir a página e Ir a porcentaje ahora aceptan `+n` y `-n` para moverte de forma relativa a donde estés.
* Libros EPUB, MOBI y CHM sin encabezados propios ahora obtienen navegación de encabezados de su tabla de contenidos.
* Los libros KF8 (AZW3) ahora soportan navegación de secciones.
* Las páginas EPUB que son solo una imagen ahora muestran una línea para ella, para que puedas llegar a ellas en lugar de saltarlas directamente.

##### Audiolibros
* Controles de velocidad de reproducción, desde media velocidad hasta tres veces más rápido. Usa `Ctrl+Shift+.` y `Ctrl+Shift+,`, o el menú Herramientas.
* Los marcadores y notas en libros solo de audio ahora recuerdan la hora exacta en la que los estableciste.
* Posición siguiente y anterior (`Alt+Left` y `Alt+Right`) ahora funcionan en audiolibros.
* El progreso a través de un audiolibro ahora se mide por su grabación, por lo que Ir a porcentaje y la barra de estado coinciden con lo lejos que realmente has llegado.

##### Documentos recientes
* Un elemento Borrar documentos recientes en el submenú Documentos recientes.

##### Documentos PDF
* Una configuración para mantener cada línea de un PDF separada, en lugar de unirlas en párrafos.
* Las imágenes y figuras en PDFs ahora se anuncian.
* Los PDFs que tienen estructura de lectura pero no etiquetan ninguna de sus imágenes ahora anuncian esas imágenes, en lugar de omitirlas completamente del libro.

##### Vista web
* Cualquier documento ahora puede abrirse en la vista web, no solo EPUB, HTML y Markdown.

##### Legibilidad
* Los encabezados ahora se dibujan con un tamaño que coincide con su nivel, e imágenes y tablas se separan del texto que las rodea.

##### pb
* `pb --list-formats` lista todos los formatos que pb puede leer.
* pb ahora dice qué archivo no pudo leer y por qué.

#### Correcciones

##### General
* Se corrigió un bloqueo al cerrar Paperback.
* Al cerrar Paperback ahora la ventana se oculta inmediatamente, en lugar de permanecer en pantalla mientras se guarda.
* Abrir un documento ya no deja Reabrir último cerrado habilitado cuando no hay nada que reabrir.
* Paperback ya no sigue reintentar documentos en su lista reciente que han desaparecido, y limita cuántos documentos recientes almacena.
* El antiguo archivo de configuración INI ahora se elimina una vez que se ha trasladado al nuevo formato.
* Los títulos de los diálogos de fuente y color, y el menú Exportar como en vietnamita, ahora están traducidos.
* La actualización ahora trae la ventana relanzada al frente, en lugar de dejarla detrás de todas las demás ventanas en Alt+Tab.
* El ajuste de línea ahora se aplica inmediatamente en documentos grandes, en lugar de recargar todo.

##### Navegación
* `Alt+Left` ahora vuelve a donde saltó, en lugar de a una posición más antigua.
* Los sonidos de marcapáginas ahora solo se reproducen cuando se mueve sobre un marcapáginas, no cuando llega a la línea en la que se encuentra.
* Cerrar la tabla de contenidos, la lista de elementos y los diálogos Ir ahora lo lleva directamente a la línea en la que llega, en lugar de obligarlo a esperar a que el lector de pantalla lea la ventana nuevamente.
* Ir a línea, Ir a página e Ir a porcentaje ahora rechazan números fuera del documento en lugar de ir silenciosamente a otro lugar.
* NVDA ya no corta el anuncio cuando un documento no tiene páginas.
* Presionar Aceptar en la tabla de contenidos sin moverse ahora va a la entrada que ya estaba seleccionada.
* La tabla de contenidos, la lista de elementos y la lista de marcapáginas ya no se ralentizan ni se cuelgan en libros con miles de entradas.
* Las flechas Arriba y Abajo ahora recuerdan su columna por documento, en lugar de transferirla cuando cambia de pestañas.

##### Audiolibros
* La reproducción de audio ahora usa `Control+Space` en macOS, ya que `Command+Space` pertenece a Spotlight.

##### Documentos PDF
* Se corrigieron los PDF exportados desde Apple Pages que se leían como texto sin formato, sin ninguno de los encabezados y listas con los que fueron escritos.
* Se corrigieron los párrafos y encabezados PDF que se dividían en cada línea, y las palabras que se dividían en espacios.
* Se corrigieron los encabezados PDF numerados que se ejecutaban juntos en un encabezado.
* Se corrigieron los PDF cuyo árbol de estructura no conduce a ningún texto abierto en blanco.
* Los encabezados y pies de página ya no se leen en cada página de PDF sin etiquetar.
* Los PDF que etiquetan sus encabezados y pies de página como texto ordinario ya no repiten el título y el número de página entre dos párrafos en cada página.
* Los PDF ahora muestran su título real, en lugar de su nombre de archivo.
* Las líneas establecidas en una fuente monoespaciada, como código, ya no se unen en párrafos.

##### Libros MOBI/AZW3
* Los libros MOBI grandes ya no se quedan sin memoria, y ya no se cortan después de 20 MB.
* Los libros MOBI y AZW3 ahora se abren mucho más rápido.
* Se corrigió que los libros MOBI perdieran su lista de capítulos.
* Se corrigió el texto garbled donde los libros MOBI pasan de un registro a otro.

##### Vista web
* La vista web ya no carga la totalidad de un libro enorme a la vez.
* La vista web ahora muestra documentos completos cuando el lector los muestra completos, en lugar de solo una parte de ellos.

##### Otros formatos
* Los libros FictionBook (.fb2) escritos en windows-1251, que es la mayoría de ellos, ahora se abren en lugar de no poder leer en absoluto.
* Los libros FictionBook que utilizan un espacio de nombres o una entidad HTML que nunca declararon ahora se abren, en lugar de ser rechazados como rotos.
* Los libros en codificaciones heredadas ahora se abren mucho más rápido.
* Se corrigieron algunos archivos de texto chino que se abrían como texto garbled.
* Los archivos OpenDocument protegidos con contraseña ahora solicitan su contraseña, en lugar de ser reportados como rotos.
* Los archivos PowerPoint heredados protegidos con contraseña ahora se abren, y las diapositivas PowerPoint heredadas ya no pierden su texto.
* Los archivos de texto sin formato guardados con una extensión `.rtf` ahora se abren como texto, en lugar de fallar con un error.
* Las palabras de control RTF ya no se muestran como texto.

#### iOS y Android

Las aplicaciones iOS y Android abren todos los formatos que lo hace la versión de escritorio, e incluyen:

* Lectura en voz alta, con su opción de voz, velocidad e inclinación, un control de velocidad de habla directamente en la barra de lectura, y una pausa opcional entre párrafos.
* Reproducción de audiolibros DAISY, M4B y MP3, que continúa en segundo plano y desde la pantalla de bloqueo.
* Navegación por encabezados, páginas, enlaces, tablas, listas y más desde la barra de lectura, además de la tabla de contenidos y Buscar.
* Un temporizador de sueño, recuento de palabras y exportación de documentos, además de un diccionario de voz en iOS.
* Opciones de tamaño de texto y espaciado, además de texto de alto contraste en iOS.
* Atajos de teclado que coinciden con la versión de escritorio.

### Versión 0.9.2
* Los audiolibros ya no hacen que el lector de pantalla lea una serie de espacios cuando enfocas el campo de texto.
* Los audiolibros ahora nombran el archivo conforme avanzan a través de las secciones.
* Los audiolibros ahora reportan su duración real, en lugar de afirmar que todos los archivos duran 24 horas.
* Cerrar la Vista Web con Escape ya no muestra un cuadro de alerta de depuración después de seguir un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te da todo el documento, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directamente a la línea encontrada, en lugar de hacerte esperar a que el lector de pantalla lea la ventana de nuevo cuando el enfoque regresa al libro.
* Se corrigieron los EPUB que llevan un bloque ZIP64 extraviado rechazándose abrir con "Invalid local file header".
* Se corrigieron los documentos largos que retrocedían a su inicio mientras un lector de pantalla leía continuamente a través de ellos.
* Los enlaces en la Vista Web ahora te llevan a la sección a la que apuntan, en lugar de fallar con "File not found".
* El anuncio automático "Document reloaded" ya no interrumpe el lector de pantalla a mitad de frase, esperando en su lugar a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora alterna a través de sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de buscar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la etiqueta completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrir y rastrear su cronología en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares desapareciendo de documentos RTF, juntando las palabras que los rodeaban.
* Se corrigieron las imágenes RTF filtrando sus datos sin procesar al documento como texto garbled.
* Se corrigió el submenú de Documentos recientes manteniéndose con entradas antiguas hasta que algo más sucediera para reconstruirlo.
* Los aceleradores de teclado están de nuevo en todas las traducciones, por lo que los menús del ruso tienen acceso de teclado de nuevo.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Opciones ha sido renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, tamaño y estado maximizado de su ventana entre ejecuciones.
* Los plurales ahora están traducidos, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar accesos directos de teclado ahora se pueden traducir.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI llamada pb para convertir rápidamente cualquiera de los formatos compatibles de Paperback a HTML, Markdown o texto plano.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver Fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puede cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, reporte cualquier comportamiento extraño que encuentre con esto.

##### Compatibilidad de Plataforma
* ¡Compatibilidad con ARM64 Windows!
* ¡Compatibilidad nativa con macOS!
* Un toggle de pantalla completa.

##### Diálogo Todos los Documentos
* Un botón de localización para localizar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que pueda filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y Legibilidad
* Una pestaña de legibilidad con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, véase a continuación);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y su correspondiente tecla de acceso rápido.
* Un toggle para determinar cómo desea que se muestren las tablas, y se unificó cómo se muestran las tablas en todos los documentos.

##### Navegación
* Compatibilidad para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de exploración en lectores de pantalla.
* El atajo de teclado de igual para anunciar su porcentaje actual en un documento.

##### Marcadores
* Marcadores temporales: puede tener uno por documento, y se persisten. Use barra diagonal para establecer uno y barra invertida para saltar a él.

##### Conteo de Palabras
* Tiempo de lectura estimado en el diálogo de conteo de palabras, así como la capacidad de establecer su velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si una selección está activa cuando abre el diálogo de conteo de palabras, ahora se mostrará cuántas palabras tiene seleccionadas.

##### Atajos de Teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento del menú de exportación para permitir exportar a HTML y Markdown, además de texto plano.

##### Actualizador
* Un botón de cancelación al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido modificado.

##### Vista Web
* La vista web ahora se abre en su posición actual de lectura.

##### Libros DAISY
* Compatibilidad con libros DAISY 2.0.
* Compatibilidad con reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con DAISY audio (incluyendo DAISY audio + texto) y archivos zip de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar la narración, buscar hacia adelante y hacia atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Compatibilidad con listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora admiten tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se mostrarán correctamente en lugar de como un montón de mojibake.
* "Reabrir último cerrado" intentando reabrir el archivo readme incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alto DPI.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los Documentos
* Escape no cerraba los diálogos de Información de Documento y Todos los Documentos.
* La barra de título no se actualizaba después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se añadirá a tu lista de todos los documentos cuando se abra mediante `Shift+F1`.
* Eliminar documentos del diálogo de recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de eliminar un documento.

##### Navegación
* Navegación de página anunciando texto de línea incorrecto en algunas situaciones.
* Ir a Línea, Ir a Página e Ir a Porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar Siguiente no respetaban la ventana de documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deberían reproducirse exclusivamente cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de línea te disparaba al inicio de tu documento.

##### Vista Web
* El diálogo de vista web no era redimensionable y se abría con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de versión.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Carga de libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* RTF `\pict` grupos para que los datos de imagen integrados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Anclajes filepos en libros Mobi dividiendo etiquetas HTML e introduciendo basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 significativamente mejorado.

##### Documentos de Word
* Documentos de Word con nombres de estilo específicos de la configuración regional que no rendían correctamente sus encabezados.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no bloquearán Paperback al abrir.

### Versión 0.8.5
* Se añadió soporte de página a libros epub.
* Se añadió soporte para documentos de Microsoft Office cifrados. Actualmente se admiten Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se añadió soporte para documentos heredados de Microsoft Word!
* ¡Se añadió soporte para presentaciones heredadas de Powerpoint!
* ¡Se añadió soporte para libros mobi y AZW3!
* ¡Se añadió soporte para archivos PDF etiquetados!
* Se añadió el atajo `ctrl+q` para salir de la aplicación.
* ¡Se añadió soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes integradas ahora debería mostrarse correctamente.
* Los documentos CHM ahora admiten correctamente la navegación de enlaces internos.
* Se corrigió que ir a página estuviera desviado por 1.
* Se corrigió que la tecla escape no funcionara para cerrar el diálogo abrir como.
* Se corrigió que el menú contextual del lector no se mostrara en el clic derecho o la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF de solo imagen se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con `g`/`Shift+G` y `f`/`Shift+F`, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte de XML DAISY, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primera letra Win32 nativa en el árbol de tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suavemente.

### Versión 0.8.2
* ¡Se añadió soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no colocaría un espacio entre palabras en casos raros.
* Se corrigió que los párrafos se dividieran en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las tabulaciones RTF y saltos de línea ahora se renderizan exactamente como aparecen en el documento.
* Se volvió a cambiar a la biblioteca pdfium probada y verdadera para analizar PDF, haciendo que la representación de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se añadió `Ctrl+Shift+T` para reabrir el último documento cerrado.
* El diálogo Todos los Documentos ahora admite seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron rutas de archivo que contienen caracteres que no son ASCII (como š, č, ć, ž en bosnio) que se corruptían al abrir un archivo mediante una segunda instancia de Paperback.
* Se corrigió que el texto PDF se leyera en el orden incorrecto y espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se añadieron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se añadió un actualizador automático que ahora reemplazará tu versión instalada de Paperback en lugar de simplemente descargar la nueva versión!
* ¡Se añadió retroalimentación de sonido opcional para alcanzar un marcador o una nota, gracias a Andre Louis por los sonidos!
* ¡Se añadió soporte para documentos RTF!
* Se añadió soporte para documentos DAISY XML.
* Se añadió soporte para archivos Flat Open Document Text.
* Se añadió soporte para presentaciones Flat Open Document.
* Se añadió soporte para separadores con s y `shift+s`.
* Ahora cualquier movimiento mayor de 300 caracteres añadirá automáticamente al historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin formato en lugar de HTML renderizado en la Vista Web.
* Se corrigieron las tablas que no se renderizaban correctamente en archivos Markdown.
* Los PDFs solo con imágenes ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrustó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar el uso y la navegación.
* Se cambió a Hayro para analizar PDFs, lo que genera mayor confiabilidad, velocidad y menos DLLs.
* Se rescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se añadió soporte de tablas para documentos basados en HTML y XHTML! Navega entre tablas usando T y `Shift+T`, y presiona Entrar para ver una en un renderizador web.
* ¡Se añadió una característica básica de renderización web! Presiona `Ctrl+Shift+V` para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se añadió una traducción al ruso, gracias a Ruslan Gulmagomedov!
* Se añadió un botón Borrar Todo al diálogo Todos los Documentos.
* El verificador de actualizaciones ahora muestra notas de la versión cuando está disponible una nueva versión.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió el diálogo de búsqueda que no se ocultaba correctamente al usar los botones siguiente/anterior.
* Se corrigió que el TOC de epub ocasionalmente te llevara al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en etiquetas XML, HTML y pre.
* Se corrigió un error de uno en la navegación de enlaces.
* Se corrigieron algunos libros que tenían espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos del menú relacionados con marcadores así como la lista de elementos ahora se deshabilitan correctamente cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para los colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se añadió soporte para PDFs protegidos con contraseña!
* Se añadió una característica muy básica para ir a la posición anterior/siguiente. Si presionas entrar en un enlace interno y mueve el cursor, esa posición ahora será recordada y se puede navegar a ella con las flechas `alt+izquierda/derecha`.
* ¡Se añadió una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se añadió una opción para iniciar Paperback en modo maximizado de forma predeterminada.
* Se corrigieron los enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de TOCs de Epub que contienen rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigieron los títulos de algunos capítulos de epub que no aparecían correctamente en el diálogo de TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones Aceptar/Cancelar en el diálogo de TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora obtendrás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se ha añadido una nueva opción para mostrar el menú de navegación en una forma mucho más compacta en el diálogo de opciones, activada por defecto.
* Se ha añadido una opción para hacer que la navegación por elementos estructurales sea envolvente.
* Se ha añadido una opción al menú de herramientas para abrir la carpeta que contiene el documento actualmente enfocado.
* Se ha añadido un sistema de actualización bastante simple, pero muy efectivo.
* Se ha añadido una función básica de temporizador de reposo, accesible con `Ctrl+Shift+S`.
* ¡Se ha añadido soporte para analizar libros electrónicos FB2!
* ¡Se ha añadido soporte para analizar presentaciones OpenDocument!
* ¡Se ha añadido soporte para analizar archivos de texto OpenDocument!
* Los marcadores ahora pueden marcar una línea completa, o marcar solo texto específico. Si no tiene ninguna selección activa al colocar un marcador, el comportamiento es como en pre-0.6 y marcará la línea completa. Sin embargo, si selecciona algo de texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas! Navegue entre marcadores que contienen notas con N y `Shift+N`, o abra el diálogo de marcadores con todos los marcadores, solo notas, o solo no-notas seleccionados con teclas de acceso rápido específicas.
* Los marcadores en el diálogo de marcadores ya no tendrán un prefijo molesto "marcador x".
* Los libros Epub que contienen contenido HTML pretendiendo ser XML ahora se manejarán correctamente.
* Se ha corregido la carga de grandes documentos Markdown.
* Se ha corregido el problema de presionar espacio en la vista de árbol de tabla de contenidos que activaba el botón OK.
* Se ha corregido el manejo de espacios en blanco al principio de etiquetas pre en documentos HTML y XHTML.
* Se ha corregido que el control de texto no recupere el foco a veces al volver a la ventana de Paperback.
* Se ha corregido que el campo de texto en el diálogo de ir a porcentaje no actualizara el valor del deslizador.
* Se ha corregido la renderización de IDs HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si carga un libro con un parámetro de línea de comandos mientras se está ejecutando una instancia existente de Paperback, ya no obtendrá un error si la carga de su documento tarda más de 5 segundos.
* Si ejecuta Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde el diálogo de marcadores.
* Ahora es posible importar y exportar sus marcadores y posición de lectura para un documento en particular. El archivo generado se nombra con el nombre del archivo y una extensión `.paperback`. Si se encuentra un archivo como este en el mismo directorio que el archivo mientras se carga, se cargará automáticamente. De lo contrario, puede importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Use k y `shift+k` para moverse hacia adelante y hacia atrás a través de ellos, y presione Intro para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo que la aplicación sea más rápida y el binario más pequeño.
* El contenido Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Use L y `Shift+L` para ir por las listas en sí, e I y `Shift+I` para ir a través de elementos de lista.
* Ahora Supr en el teclado numérico funciona para eliminar documentos de la barra de pestañas además de eliminar normal.
* ¡Paperback ahora puede minimizarse opcionalmente a su bandeja del sistema! Esta opción está desactivada por defecto, pero activarla hará que la opción de minimizar en el menú del sistema coloque Paperback en su bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que admite actualmente es bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo léame en su navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido dramáticamente! En lugar de simplemente mostrarle los últimos 10 documentos que abrió, ahora le mostrará una cantidad personalizable, siendo los documentos restantes que ha abierto alguna vez accesibles a través de un pequeño diálogo.
* Varias mejoras pequeñas en los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de saltos de línea dentro de párrafos en documentos de Word y añadir viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se añadió soporte para documentos de Microsoft Word!
* ¡Se añadió soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú que no se deshabilitaban sin documentos abiertos.
* Se corrigió la orientación del control deslizante de ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió el espaciado en blanco que se eliminaba de los encabezados XHTML de formas extrañas.
* Se corrigió el manejo del espaciado en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenidos! Cuando cargues un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos basada en la estructura de los encabezados de tu documento, y te la mostrará en el diálogo `ctrl+t`.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta title, si existe. De lo contrario, seguirán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar voz. Esto significa que ya no se envían DLLs de lectores de pantalla junto con el programa, y ahora se admitirán más lectores de pantalla, como Microsoft Narrator.
* Se cambiaron las librerías zip para permitir abrir una variedad más amplia de libros epub.
* El diálogo que te pregunta si deseas abrir tu documento como texto sin formato se ha rehecho por completo, y ahora te permite abrir tu documento como texto sin formato, HTML o Markdown.
* El diálogo de ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje al que saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se preservará nuevamente exactamente.
* El espacio de no separación Unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te preguntará cómo deseas abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se añadió un icono opcional del menú Inicio al instalador.
* La tabla de contenidos debería ser más limpia en algunos casos, por ejemplo si tienes un elemento hijo y padre con el mismo texto en la misma posición ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se añadió soporte para archivos CHM!
* ¡Se añadió soporte para marcadores! Puedes tener tantos marcadores como desees en tantos documentos como desees. Puedes saltar hacia adelante y hacia atrás a través de ellos con `b` y `shift+b`, establecer uno con `control+shift+b`, y abrir un diálogo para saltar a un marcador específico con `control+b`.
* ¡Se añadió un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Program Files y configurará automáticamente las asociaciones de archivo para ti.
* Los archivos de texto con BOMs ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se añadió mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja mediante su propio diálogo basado en controles deslizantes, accesible con `control+shift+g`.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor por defecto.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda entre reinicios de la aplicación.
* La entrada en los diálogos de ir a línea e ir a página ahora debería ser desinfectada de manera más estricta.
* Se corrigió la navegación de la tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode de múltiples bytes.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigió que los elementos de TOC anidados en libros Epub colocaran tu cursor en la posición incorrecta.
* Se corrigió un bloqueo en la salida de la aplicación en ciertos casos.
* ¡Se añadió una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace patrocinar este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió a la librería PDF utilizada en Chromium, lo que permite un análisis de PDF mucho más confiable en general.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar `paperback.exe` con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar suprimir en un documento en el control de pestañas para cerrarlo.

### Versión 0.2.1
* Se añadió el número total de páginas a la etiqueta de página en el diálogo de ir a página.
* Se permite tabular desde el contenido del documento a tu lista de documentos abiertos.
* Se corrigieron los atajos de teclado de encabezados que a veces abrían documentos recientes si tenías suficientes de ellos.
* Paperback ahora eliminará guiones suaves innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados que a veces te colocaba en el carácter incorrecto.

### Versión 0.2.0
* ¡Se añadió soporte para documentos markdown!
* ¡Se añadió soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se añadieron atajos de teclado para navegar por encabezados en contenido HTML, incluyendo libros epub y documentos markdown. Estos atajos fueron diseñados para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de epubs con nombres de archivo codificados por URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro de ellos.
* Ahora se pronuncia un mensaje si el documento no admite una tabla de contenidos o secciones, en lugar de que los elementos del menú se desactiven.
* ¡Se añadió un menú de documentos recientes! Actualmente almacena los últimos 10 documentos abiertos, y presionar enter en uno abrirá para lectura.
* Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras se añadía un historial de sus últimas 25 búsquedas y soporte para expresiones regulares.
* Los documentos abiertos previamente ahora se recuerdan entre reinicios de la aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú herramientas.
* Se añadió `shift+f1` para abrir el readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
