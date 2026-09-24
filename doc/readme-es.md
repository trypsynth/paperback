<!-- machine-translated from doc/readme.md (source-hash: 4d3bd6acdc082011; sections: 84030068,db723a70,df2f4c18,14335443,d44bf4c8,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,fabb029c); please review and edit as needed -->

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

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, existe un problema conocido para los usuarios de JAWS.

### JAWS y pantallas braille

Si utiliza JAWS con una pantalla braille, es posible que descubra que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de su pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en el manejo de JAWS del control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tardó bastante tiempo en encontrarse una solución dada la entusiasmo de Vispero en responder a problemas con software de código abierto.

La solución alternativa, que finalmente surgió a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También deseará habilitar "Pan Text by Paragraph", de lo contrario su pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en lugar, el desplazamiento debería funcionar correctamente.

### JAWS y los mensajes de Paperback

Paperback dice cosas como "No pages." o "This document has no audio." como notificaciones de accesibilidad, lo que permite que un lector de pantalla las hable sobre lo que sea que esté diciendo. JAWS solo actúa sobre ellas cuando "Enable accessible notification events" está activado para la aplicación, y en algunas máquinas no lo está.

Si JAWS no dice nada cuando presiona una tecla que debería informar algo, abra el Centro de Configuración con Paperback en primer plano (`Insert+6`), busque "notification" y marque "Enable accessible notification events". Esto escribe la configuración en `paperback.jcf`, por lo que se aplica solo a Paperback.

## Tipos de archivo actualmente compatibles

Paperback admite los siguientes formatos y extensiones:

* Archivos de cómics (`.cbz`)
* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` como BSD `mdoc` (`.1` a `.9`, `.man`, `.roff` y sus formas comprimidas)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolibros M4B (`.m4b`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Audiolibros MP3 (`.mp3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos reStructuredText (`.rst`, `.rest`)
* Documentos RTF (`.rtf`)
* Documentos Windows Write (`.wri`)
* Archivos WinHelp (`.hlp`)
* Archivos de texto sin formato y registros (`.txt`, `.log`)

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

### Apertura de documentos

* Usa el botón Abrir Libro, o abre un documento desde la aplicación Archivos u otra aplicación y elige Paperback.
* En Android, puedes activar el navegador de archivos dentro de la aplicación en Configuración. Necesita permiso de Acceso a todos los archivos y abre archivos grandes directamente en lugar de copiarlos primero.
* Mantén pulsado el botón Abrir Libro para importar o exportar los datos de un documento (`.paperback`), los mismos archivos que usa la aplicación de escritorio.

### Lectura y escucha

Cada aplicación tiene dos formas de leer un documento. En modo texto, lees el texto con tu lector de pantalla. En modo lectura en voz alta, Paperback lee el texto por ti con la voz que elijas en Configuración, y continúa en segundo plano y desde la pantalla de bloqueo. Cambia entre ellos desde el menú Más opciones.

Los audiolibros, como DAISY, M4B y libros MP3, reproducen su propia grabación en su lugar.

### La barra de lectura

La barra en la parte inferior de la pantalla tiene, de izquierda a derecha:

* La unidad de navegación, como párrafo, encabezado, página o enlace. Desliza hacia arriba o hacia abajo en ella para cambiarla.
* Botones Anterior, reproducir y Siguiente. Anterior y Siguiente se mueven por la unidad de navegación.
* La velocidad de voz. Desliza hacia arriba o hacia abajo en ella para cambiar la velocidad de lectura de Paperback.

También puedes deslizar hacia arriba o hacia abajo en el botón reproducir para moverte por la unidad de navegación, sin necesidad de alcanzar los botones Anterior y Siguiente. Si eso es todo lo que usas, la configuración Ocultar botones Anterior y Siguiente los elimina del camino del lector de pantalla. La configuración Deslizar hacia arriba avanza elige en qué dirección va un deslizamiento.

### Más opciones

El menú Más opciones es donde vive todo lo demás. Algunos elementos funcionan un poco diferente en cada aplicación.

* **Cambiar a modo TTS o Cambiar a modo Texto:** se mueve entre el modo lectura en voz alta y el modo texto, descrito arriba. En modo texto, un elemento Lectura en voz alta inicia y pausa la lectura en voz alta sin salir del modo texto.
* **Tabla de contenidos:** los capítulos del libro, abiertos en el que estás leyendo. Elige uno para ir directamente a él. Las entradas con capítulos bajo ellas pueden expandirse y contraerse con las acciones del lector de pantalla.
* **Elementos:** una lista de los encabezados o enlaces del documento. Cambia entre los dos con el selector de Tipo en iOS, o las pestañas en Android, luego elige uno para ir a él.
* **Buscar:** escribe lo que quieres buscar, o elige una búsqueda anterior del Historial de búsqueda, y elige si deseas coincidir mayúsculas y minúsculas, coincidir solo palabras completas, o usar una expresión regular. Buscar anterior y Buscar siguiente saltan a una coincidencia y dicen dónde llegó, y Buscar permanece abierto para que puedas continuar. En modo lectura en voz alta, Buscar también aparece como una unidad de navegación en la barra de lectura, para que puedas navegar por las coincidencias desde allí también.
* **Ir a:** salta a una línea, una página, o un porcentaje del documento. Elige cuál con el selector de Modo.
* **Documentos recientes:** todos los documentos que has abierto, cada uno marcado como actualmente abierto, cerrado o archivo no encontrado. Cada uno tiene dos acciones del lector de pantalla: Eliminar lo quita de la lista, y Localizar te permite encontrar un documento cuyo archivo se ha movido. Borrar documentos recientes vacía la lista sin eliminar ningún documento.
* **Contar palabras:** el número de palabras en el documento.
* **Información del documento:** el título, el autor, el nombre del archivo, y en iOS también el número de líneas y caracteres.
* **Exportar:** guarda el documento como texto plano, HTML o Markdown.
* **Temporizador de sueño:** detiene la lectura después de 5, 10, 15, 30, 45 o 60 minutos, o un tiempo personalizado. Abrelo de nuevo mientras se está ejecutando para ver cuánto tiempo queda, o para cancelarlo.
* **Ayuda:** abre este readme.
* **Configuración:**
    * **Conversión de texto a voz:** la voz, la velocidad del habla y el tono, un botón Reproducir muestra para escucharlos, y la pausa entre párrafos. Android también te permite elegir el motor de voz. En iOS, aquí también está el diccionario de voz: reglas que cambian cómo se pronuncian las palabras, para todas las voces o solo algunas.
    * **Legibilidad:** tamaño del texto, espaciado de líneas, espaciado de párrafos, alineación y texto de alto contraste. iOS también tiene apariencia clara y oscura.
    * **Comportamiento:** si reabre los documentos cuando inicia la aplicación, en qué dirección se mueve un deslizamiento en el botón reproducir, y si ocultar los botones Anterior y Siguiente. Android también tiene el navegador de archivos dentro de la aplicación aquí.

### Teclados y auriculares

Con un teclado, los atajos de escritorio para abrir libros, documentos recientes, Buscar, Ir a, la tabla de contenidos, contar palabras, información del documento, exportar y el temporizador de sueño funcionan todos, usando `Cmd` en lugar de `Ctrl` en iOS. También lo hacen las teclas de una sola letra para moverte por encabezado, página, enlace y el resto, y `Space` reproduce y pausa. En iOS, las teclas de una sola letra solo llegan a Paperback mientras la Quick Nav de una sola letra de VoiceOver está desactivada.

En Android, un botón de auricular reproduce y pausa con una pulsación, avanza con dos, y retrocede con tres.

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

#### Agregado

##### General
* Compatibilidad con Linux, como AppImage o tar.gz, con integración de escritorio para que los documentos se abran desde el administrador de archivos.
* Marca el comienzo de una selección con `Alt+F9`, copia todo desde allí hasta donde hayas llegado con `Alt+F10`, y vuelve a la marca con `Alt+Shift+F9`, para copiar un largo tramo de texto sin usar mayúscula y flecha. Los tres están en Herramientas > Seleccionar y copiar.
* El atajo `=` ahora anuncia la página además del porcentaje, por ejemplo "15%, página 30", y se mantiene como estaba para documentos sin números de página.
* El cuadro Acerca de ahora muestra la licencia de Paperback y todos los traductores.
* Una traducción al ucraniano.

##### Nuevos formatos
* Archivos de cómics (`.cbz`).
* Audiolibros M4B, divididos en sus capítulos.
* Páginas de manual, tanto `man` como BSD `mdoc`, comprimidas o no.
* Audiolibros MP3, divididos en capítulos cuando el archivo los tiene.
* Documentos reStructuredText.
* Archivos de Windows Write (`.wri`).
* Archivos de WinHelp (`.hlp`).
* Documentos de Word 6 y Word 95.

##### OCR
* Las páginas de PDF escaneadas ahora se pueden reconocer con el OCR integrado en Windows y macOS. Presiona `Enter` en una página escaneada para reconocerla, o usa OCR por lotes (`Ctrl+Shift+O`) para un rango de páginas.

##### Navegación
* Las fórmulas MathML en EPUB e HTML se renderizan como AsciiMath usando MathCAT. Usa `M` o `Shift+M` para navegar fórmulas, luego `Enter` o `Space` para abrir el MathML original en Vista de fórmula.
* Un botón Buscar todo en el diálogo Buscar, que enumera cada línea con una coincidencia para que puedas saltar directamente a la que desees.
* Vistas de tablas, listas y páginas en la lista de elementos (`F7`).
* Ir a línea, Ir a página e Ir a porcentaje ahora aceptan `+n` y `-n` para moverse relativamente a donde te encuentras.
* Libros EPUB, MOBI y CHM sin sus propios títulos ahora obtienen navegación de títulos de su tabla de contenidos.
* Los libros KF8 (AZW3) ahora soportan navegación de secciones.
* Las páginas EPUB que son solo una imagen ahora muestran una línea para ella, para que puedas aterrizar en ellas en lugar de saltarlas.

##### Audiolibros
* Controles de velocidad de reproducción, de media velocidad a tres veces más rápido. Usa `Ctrl+Shift+.` y `Ctrl+Shift+,`, o el menú Herramientas.
* Los marcadores y notas en libros solo de audio ahora recuerdan la hora exacta en que los configuraste.
* Posición siguiente y anterior (`Alt+Left` y `Alt+Right`) ahora funcionan en audiolibros.
* El progreso a través de un audiolibro ahora se mide por su grabación, por lo que Ir a porcentaje y la barra de estado coinciden con qué tan lejos estás realmente.

##### Documentos recientes
* Un elemento Borrar documentos recientes en el submenú Documentos recientes.

##### Documentos PDF
* Una configuración para mantener cada línea de un PDF separada, en lugar de unirlas en párrafos.
* Las imágenes y figuras en PDF ahora se anuncian.
* Los PDF que tienen estructura de lectura pero no etiquetan ninguna de sus imágenes ahora anuncian esas imágenes, en lugar de dejarlas fuera del libro completamente.

##### Vista web
* Cualquier documento ahora se puede abrir en la vista web, no solo EPUB, HTML y Markdown.

##### Legibilidad
* Los títulos ahora se dibujan en un tamaño que coincide con su nivel, y las imágenes y tablas se separan del texto a su alrededor.

##### pb
* `pb --list-formats` lista cada formato que pb puede leer.
* pb ahora dice qué archivo no pudo leer y por qué.

#### Corregido

##### General
* Se corrigió un bloqueo al cerrar Paperback.
* Cerrar Paperback ahora oculta la ventana inmediatamente, en lugar de dejarla visible mientras se guarda.
* Abrir un documento ya no deja habilitado Reabrir último cerrado cuando no hay nada que reabrir.
* Paperback ya no sigue reintentando documentos en tu lista reciente que se han perdido, y limita cuántos documentos recientes almacena.
* El archivo de configuración INI antiguo ahora se elimina una vez que se ha migrado al nuevo formato.
* Los títulos de los diálogos de fuente y color, y el menú Exportar como en vietnamita, ahora están traducidos.
* Actualizar ahora trae la ventana reiniciada al frente, en lugar de dejarla detrás de todas las demás ventanas en Alt+Tab.
* El ajuste de línea ahora se aplica inmediatamente en documentos grandes, en lugar de recargar todo.

##### Navegación
* `Alt+Left` ahora vuelve a donde saltaste desde, en lugar de a una posición anterior.
* Los sonidos de marcapáginas ahora solo se reproducen cuando te mueves sobre un marcapáginas, no cuando llegas a la línea en la que se encuentra.
* Cerrar la tabla de contenidos, la lista de elementos y los diálogos Ir ahora te lleva directamente a la línea en la que llegas, en lugar de hacer que esperes a que el lector de pantalla vuelva a leer la ventana.
* Ir a línea, Ir a página e Ir a porcentaje ahora rechazan números fuera del documento en lugar de ir silenciosamente a otro lugar.
* NVDA ya no corta el anuncio cuando un documento no tiene páginas.
* Presionar Aceptar en la tabla de contenidos sin movimiento ahora va a la entrada que ya estaba seleccionada.
* La tabla de contenidos, la lista de elementos y la lista de marcapáginas ya no se ralentizan ni se cuelgan en libros con miles de entradas.
* Las flechas Arriba y Abajo ahora recuerdan su columna por documento, en lugar de llevarla cuando cambias de pestaña.

##### Audiolibros
* La reproducción de audio ahora usa `Control+Space` en macOS, ya que `Command+Space` pertenece a Spotlight.

##### Documentos PDF
* Se corrigieron los PDF exportados desde Apple Pages que se leían como texto plano, sin ninguno de los encabezados y listas con los que fueron escritos.
* Se corrigieron los párrafos y encabezados PDF que se dividían en cada línea, y palabras que se dividían en espacios.
* Se corrigieron los encabezados PDF numerados que se ejecutaban juntos en un encabezado.
* Se corrigieron los PDF cuyo árbol de estructura no lleva a ningún texto abierto vacío.
* Los encabezados y pies de página ya no se leen en cada página de PDF sin etiquetar.
* Los PDF que etiquetan sus encabezados y pies de página como texto ordinario ya no repiten el título y el número de página entre dos párrafos en cada página.
* Los PDF ahora muestran su título real, en lugar del nombre de archivo.
* Las líneas establecidas en una fuente monoespaciada, como código, ya no se unen en párrafos.

##### Libros MOBI/AZW3
* Los libros MOBI grandes ya no se quedan sin memoria, ni se cortan después de 20 MB.
* Los libros MOBI y AZW3 ahora se abren mucho más rápido.
* Se corrigió la pérdida de lista de capítulos en libros MOBI.
* Se corrigió el texto distorsionado donde los libros MOBI pasan de un registro a otro.

##### Vista web
* La vista web ya no carga todo un libro enorme a la vez.
* La vista web ahora muestra documentos completos cuando el lector los muestra completos, en lugar de solo una parte de ellos.

##### Otros formatos
* Los libros FictionBook (.fb2) escritos en windows-1251, que son la mayoría, ahora se abren en lugar de no poder leer en absoluto.
* Los libros FictionBook que usan un espacio de nombres o una entidad HTML que nunca declararon ahora se abren, en lugar de ser rechazados como rotos.
* Los libros en codificaciones heredadas ahora se abren mucho más rápido.
* Se corrigió que algunos archivos de texto chino se abrieran como texto distorsionado.
* Los archivos OpenDocument protegidos con contraseña ahora piden su contraseña, en lugar de ser reportados como rotos.
* Los archivos PowerPoint heredados protegidos con contraseña ahora se abren, y las diapositivas PowerPoint heredadas ya no pierden su texto.
* Los archivos de texto plano guardados con una extensión `.rtf` ahora se abren como texto, en lugar de fallar con un error.
* Las palabras de control RTF ya no aparecen como texto.

#### iOS y Android

Las aplicaciones iOS y Android abren todos los formatos que lo hace la versión de escritorio, e incluyen:

* Leer en voz alta, con tu opción de voz, velocidad y tono, un control de velocidad de habla directamente en la barra de lectura, y una pausa opcional entre párrafos.
* Reproducción de audiolibros DAISY, M4B y MP3, que continúa en segundo plano y desde la pantalla de bloqueo.
* Navegación por encabezados, páginas, enlaces, tablas, listas y más desde la barra de lectura, además de la tabla de contenidos y Buscar.
* Un temporizador de sueño, recuento de palabras y exportación de documentos, además de un diccionario de habla en iOS.
* Opciones de tamaño de texto, espaciado y texto de alto contraste.
* Atajos de teclado que coinciden con la versión de escritorio.

### Versión 0.9.2
* Los audiolibros ya no hacen que tu lector de pantalla lea una serie de espacios cuando enfocas el campo de texto.
* Los audiolibros ahora nombran el archivo mientras avanzas por secciones.
* Los audiolibros ahora informan su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar la vista web con Escape ya no muestra una alerta de depuración después de haber seguido un enlace en ella.
* Copiar después de Seleccionar todo ahora te da todo el documento, en lugar de solo la parte que está cargada actualmente.
* Buscar ahora va directamente a la línea que encontró, en lugar de hacer que esperes a que el lector de pantalla vuelva a leer la ventana mientras el enfoque vuelve al libro.
* Se corrigieron los EPUB que llevan un bloque ZIP64 extraño que se negaban a abrirse con "Encabezado de archivo local inválido".
* Se corrigieron los documentos largos que retrocedían a su inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en WebView ahora te llevan a la sección a la que apuntan, en lugar de fallar con "Archivo no encontrado".
* El anuncio automático "Documento recargado" ya no corta tu lector de pantalla a mitad de oración, sino que espera a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora se desplaza por sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la etiqueta completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcadores y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y seguir su línea de tiempo en silencio.
* Se corrigió el problema de que las comillas rizadas, guiones largos y caracteres similares desaparecían de los documentos RTF, uniendo las palabras circundantes al hacerlo.
* Se corrigió el problema de que las imágenes RTF filtraban sus datos sin procesar en el documento como texto confuso.
* Se corrigió el submenú Documentos recientes que mantenía entradas obsoletas hasta que algo más sucedía para reconstruirlo.
* Los aceleradores de teclado vuelven en cada traducción, por lo que los menús en ruso tienen acceso por teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Opciones ha sido renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, con la convención de la plataforma.
* Paperback ahora recuerda la posición, el tamaño y el estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora pueden traducirse.
* El título del documento ahora aparece primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y `Alt+Tab`.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos compatibles de Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puede cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, informe de cualquier rareza encontrada con esto.

##### Compatibilidad con plataformas
* ¡Compatibilidad con Windows ARM64!
* ¡Compatibilidad nativa con macOS!
* Un botón de pantalla completa.

##### Diálogo Todos los documentos
* Un botón localizar para encontrar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y una barra de estado, para que pueda filtrar por estado del documento y ver cuántos documentos se muestran y seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido de general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y una tecla de acceso rápido posterior.
* Un botón para determinar cómo desea que se muestren las tablas, e unificar cómo se muestran las tablas en todos los documentos.

##### Navegación
* Compatibilidad con la navegación por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de exploración en lectores de pantalla.
* El atajo de teclado de igualdad para anunciar su porcentaje actual en un documento.

##### Marcadores
* Marcadores temporales: puede tener uno por documento y persisten. Use la barra diagonal para establecer uno y la barra invertida para ir a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer su velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si una selección está activa cuando abre el diálogo de recuento de palabras, ahora se mostrará cuántas palabras ha seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento del menú de exportación para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón Cancelar al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido alterado.

##### Vista web
* La vista web ahora se abre en su posición de lectura actual.

##### Libros DAISY
* Compatibilidad con libros DAISY 2.0.
* Compatibilidad con reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con audio DAISY (incluyendo audio DAISY + texto) y archivos zip de archivos de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, buscar hacia adelante y hacia atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si buscar más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Compatibilidad con listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora son compatibles con tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de mostrar mojibake.
* "Reabrir cerrado recientemente" intentando reabrir el archivo readme incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* El manejo de Paperback para archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente al restaurar documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo especificado en el explorador.
* Abrir el archivo readme ahora respetará el idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta densidad de píxeles.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los Documentos
* Escape no cerraba los diálogos Información de Documento y Todos los Documentos.
* La barra de título no se actualizaba después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se agregará a tu lista de todos los documentos cuando se abra mediante Shift+F1.
* Eliminar documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* Navegación de página anunciando texto de línea incorrecto en algunas situaciones.
* Ir a Línea, Ir a Página e Ir a Porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar Siguiente no respetaban la ventana del documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deberían reproducirse exclusivamente cuando navegas sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de línea te llevaba al inicio de tu documento.

##### Vista Web
* El diálogo de vista web no era redimensionable y se abría con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de la versión.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Analizando documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Anclajes filepos en libros Mobi dividiendo etiquetas HTML e insertando basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 muy mejorado.

##### Documentos de Word
* Documentos de Word con nombres de estilos específicos de la configuración regional que no renderizaban correctamente sus encabezados.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDFs etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no causarán que Paperback se bloquee al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office encriptados. Actualmente se admiten Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos heredados de Microsoft Word!
* ¡Se agregó soporte para presentaciones heredadas de Powerpoint!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo Ctrl+Q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora se mostrará correctamente.
* Los documentos CHM ahora admiten correctamente la navegación de enlaces internos.
* Se corrigió ir a página estando desplazado por 1.
* Se corrigió la tecla Escape no funcionando para cerrar el diálogo abrir como.
* Se corrigió el menú contextual del lector no mostrándose al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF de solo imagen se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar por imágenes y figuras con g/Shift+G y f/Shift+F, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó la compatibilidad con DAISY XML, ya que ya no es necesaria.
* Se cambió nuevamente a la navegación de primera letra Win32 nativa en el árbol de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y sin problemas.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epubs que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no ponía un espacio entre palabras en casos raros.
* Se corrigió que los párrafos se dividieran en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las pestañas y saltos de línea de RTF ahora se renderizan exactamente como aparecen en el documento.
* Se cambió nuevamente a la biblioteca pdfium probada y verdadera para analizar PDFs, haciendo que el renderizado de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los Documentos ahora admite seleccionar varios documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron rutas de archivo que contienen caracteres no ASCII (como el bosnio š, č, ć, ž) corrompiéndose al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió el texto PDF siendo leído en el orden incorrecto, y espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se añadieron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se añadió un actualizador automático que ahora reemplazará tu versión instalada actualmente de Paperback en lugar de solo descargar la nueva versión!
* ¡Se añadió retroalimentación de sonido opcional al alcanzar un marcador o una nota, gracias a Andre Louis por los sonidos!
* ¡Se añadió compatibilidad con documentos RTF!
* Se añadió compatibilidad con documentos DAISY XML.
* Se añadió compatibilidad con archivos Flat Open Document Text.
* Se añadió compatibilidad con presentaciones Flat Open Document.
* Se añadió compatibilidad con separadores con `s` y `shift+s`.
* Cualquier movimiento de más de 300 caracteres se añadirá automáticamente a tu historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en la vista web.
* Se corrigieron las tablas que no se renderizaban correctamente en archivos Markdown.
* Los PDF solo con imágenes te advertirán de su existencia cuando intentes cargar uno.
* Se incluyó correctamente la información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar su uso y navegación.
* Se cambió a Hayro para analizar PDFs, lo que resultó en mayor confiabilidad, velocidad y menos DLLs.
* Se reescribió la aplicación completa en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y ampliar.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se añadió compatibilidad con tablas para documentos basados en HTML y XHTML! Navega entre tablas usando `T` y `Shift+T`, y presiona `Intro` para ver una en una vista web.
* ¡Se añadió una función básica de renderizado web! Presiona `Ctrl+Shift+V` para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se añadió una traducción al ruso, gracias a Ruslan Gulmagomedov!
* Se añadió un botón Limpiar todo al diálogo Todos los documentos.
* El comprobador de actualizaciones ahora muestra notas de lanzamiento cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de los botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones al ejecutarse como administrador.
* Se corrigió el manejo de comentarios en documentos XML e HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que el TOC de epub ocasionalmente te llevara al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error de fuera por uno en la navegación de enlaces.
* Se corrigieron algunos libros con espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos relacionados con marcadores así como la lista de elementos ahora se deshabilitan correctamente cuando no hay documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se añadió compatibilidad con PDF protegidos por contraseña!
* Se añadió una función muy básica de ir a la posición anterior/siguiente. Si presionas `Intro` en un enlace interno y mueve tu cursor, esa posición será recordada ahora, y podrá navegarse con las flechas `alt+left/right`.
* ¡Se añadió una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para ampliarlo en el futuro.
* Se añadió una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigieron los enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de TOCs de Epub que contienen rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigieron los títulos de algunos capítulos epub que no aparecían correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones Aceptar/Cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se ha añadido una nueva opción para mostrar el menú de navegación en una forma mucho más compacta al diálogo de opciones, marcada por defecto.
* Se ha añadido una opción para hacer que la navegación por elementos estructurales se reinicie.
* Se ha añadido una opción al menú de herramientas para abrir la carpeta que contiene el documento actualmente enfocado.
* Se ha añadido un sistema de actualización bastante simple, pero muy efectivo.
* Se ha añadido una función básica de temporizador de reposo, accesible con Ctrl+Shift+S.
* ¡Se ha añadido soporte para analizar libros electrónicos FB2!
* ¡Se ha añadido soporte para analizar presentaciones OpenDocument!
* ¡Se ha añadido soporte para analizar archivos OpenDocument Text!
* Los marcapáginas ahora pueden marcarse para marcar una línea completa, o para marcar solo un texto especificado. Si no tienes una selección activa al colocar un marcapáginas, el comportamiento es como el anterior a 0.6, y marcará la línea completa. Sin embargo, si seleccionas algún texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con todos los marcapáginas, solo notas, o solo sin notas seleccionadas con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un prefijo molesto "marcapáginas x".
* Los libros Epub que contienen contenido HTML que se pretende ser XML ahora se manejarán correctamente.
* Se ha corregido la carga de documentos Markdown grandes.
* Se ha corregido el problema de presionar espacio en la vista de árbol de la tabla de contenidos activando el botón Aceptar.
* Se ha corregido el manejo de espacios en blanco al principio de etiquetas pre en documentos HTML y XHTML.
* Se ha corregido el control de texto que a veces no recuperaba el enfoque al volver a la ventana de Paperback.
* Se ha corregido el campo de texto en el diálogo de ir al porcentaje que no actualizaba el valor del regulador.
* Se ha corregido la representación de IDs HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se representará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras se está ejecutando una instancia existente de Paperback, ya no obtendrás un error si la carga del documento tarda más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde el diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento en particular. El archivo generado se nombra según el archivo con una extensión `.paperback`. Si se encuentra un archivo así en el mismo directorio que un archivo mientras se carga, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Usa k y shift+k para moverte hacia adelante y hacia atrás a través de ellos, y presiona intro para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo que la aplicación sea más rápida y el binario más pequeño.
* El contenido Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Usa L y Shift+L para ir por las listas en sí, e I y Shift+I para pasar por los elementos de la lista.
* Suprimir en el teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además de la tecla Suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está desactivada por defecto, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que admite actualmente es bastante pequeña, pero ¡está creciendo constantemente!
* Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo léame en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, con el resto de documentos que hayas abierto siendo accesibles a través de un pequeño diálogo.
* Varias mejoras pequeñas en los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de saltos de línea dentro de párrafos en documentos de Word, y añadir puntos de viñeta a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte para documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú que no se deshabilitaban sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió el espaciado en blanco que se eliminaba de los encabezados XHTML de formas extrañas.
* Se corrigió el manejo del espaciado en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenidos! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los encabezados en tu documento, y la mostrará en el diálogo `ctrl+t`.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar voz. Esto significa que ya no se envían DLL de lectores de pantalla junto con el programa, y ahora se admitirán más lectores de pantalla, como Microsoft Narrator.
* Se cambió la biblioteca zip para permitir abrir una variedad más amplia de libros epub.
* El diálogo que te pregunta si deseas abrir tu documento como texto plano ha sido completamente rediseñado, y ahora te permite abrir tu documento como texto plano, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje al que saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se preservará nuevamente exactamente.
* El espacio sin ruptura Unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te preguntará cómo deseas abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú de inicio opcional al instalador.
* La tabla de contenidos debería ser más limpia en algunos casos, por ejemplo si tienes un elemento secundario y principal con el mismo texto en la misma posición, ahora solo verás el elemento principal.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título establecido en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcadores! Puedes tener tantos marcadores como desees en muchos documentos. Puedes saltar hacia adelante y hacia atrás a través de ellos con `b` y `shift+b`, establecer uno con `control+shift+b`, y abrir un diálogo para saltar a un marcador específico con `control+b`.
* ¡Se agregó un instalador junto con el archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente las asociaciones de archivos para ti.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas de script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento por porcentaje ahora se maneja con su propio diálogo basado en control deslizante, accesible con `control+shift+g`.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo escribirá en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda entre reinicios de la aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería ser desinfectada más estrictamente.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron elementos TOC anidados en libros Epub colocando tu cursor en la posición equivocada.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de palabras!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace patrocinar este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback debería ser capaz de cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió la biblioteca PDF a la utilizada en Chromium, lo que lleva a un análisis PDF mucho más confiable en general.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar `paperback.exe` con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar eliminar en un documento en el control de pestañas para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Se permite tabular del contenido del documento a tu lista de documentos abiertos.
* Se corrigieron las pulsaciones de tecla de encabezado que a veces abrían documentos recientes si tenías suficientes.
* Paperback ahora eliminará guiones suaves innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados que a veces te colocaba en el carácter equivocado.

### Versión 0.2.0
* ¡Se agregó compatibilidad con documentos markdown!
* ¡Se agregó compatibilidad con documentos PDF, incluida la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de teclas para navegar por encabezados en contenido HTML, incluidos libros EPUB y documentos markdown. Estas pulsaciones de teclas fueron diseñadas para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de EPUB con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros EPUB 3 con XHTML incrustado en ellos.
* Ahora se pronuncia un mensaje si el documento no admite una tabla de contenidos o secciones, en lugar de que los elementos del menú estén deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena los últimos 10 documentos abiertos, y presionar enter en uno lo abrirá para lectura.
* Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, además de agregar un historial de sus últimas 25 búsquedas y compatibilidad con expresiones regulares.
* Los documentos abiertos anteriormente ahora se recuerdan entre reinicios de la aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú de herramientas.
* Se agregó `Shift+F1` para abrir el archivo readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
