<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

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

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, hay dos problemas conocidos para los usuarios de JAWS.

### JAWS y pantallas Braille

Si usa JAWS con una pantalla Braille, es posible que encuentre que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de su pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en el manejo de JAWS del control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tardó bastante tiempo en encontrar una solución dado el entusiasmo de Vispero por responder a problemas con software de código abierto.

La solución alternativa, finalmente presentada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrá habilitar "Pan Text by Paragraph", de lo contrario su pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones implementadas, el desplazamiento debería funcionar correctamente.

### JAWS y los mensajes de Paperback

Paperback dice cosas como "No pages." o "This document has no audio." como notificaciones de accesibilidad, lo que permite que un lector de pantalla las hable sobre lo que esté diciendo. JAWS solo actúa sobre ellas cuando "Enable accessible notification events" está activado para la aplicación, y en algunas máquinas no lo está.

Si JAWS no dice nada cuando presiona una tecla que debería reportar algo, abra Settings Center con Paperback en primer plano (`Insert+6`), busque "notification" y marque "Enable accessible notification events". Esto escribe la configuración en `paperback.jcf`, por lo que se aplica solo a Paperback.

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

1.0 es el primer lanzamiento en las cinco plataformas: Windows, macOS, Linux, iOS y Android, con las aplicaciones de iOS y Android en la App Store y Google Play.

#### Añadido

##### General
* Compatibilidad con Linux, como AppImage o tar.gz, con integración de escritorio para que los documentos se abran desde tu gestor de archivos.
* Marca el comienzo de una selección con `Alt+F9`, copia todo desde ahí hasta donde hayas llegado con `Alt+F10`, y vuelve a la marca con `Alt+Shift+F9`, para copiar un largo fragmento de texto sin usar mayúscula y flechas. Los tres están en Herramientas > Seleccionar y copiar.
* El atajo `=` ahora anuncia la página además del porcentaje, por ejemplo "15%, página 30", y sigue siendo como era para documentos sin números de página.
* El cuadro Acerca de ahora muestra la licencia de Paperback y todos los traductores.
* Una traducción al ucraniano.

##### Nuevos formatos
* Archivos de historietas (`.cbz`).
* Audiolibros M4B, divididos en sus capítulos.
* Páginas de manual, tanto `man` como `mdoc` de BSD, comprimidas o no.
* Audiolibros MP3, divididos en capítulos cuando el archivo los tiene.
* Documentos reStructuredText.
* Archivos Windows Write (`.wri`).
* Archivos WinHelp (`.hlp`).
* Documentos Word 6 y Word 95.

##### OCR
* Las páginas PDF escaneadas ahora pueden reconocerse con el OCR integrado en Windows y macOS. Presiona `Enter` en una página escaneada para reconocerla, o usa OCR por lotes (`Ctrl+Shift+O`) para un rango de páginas.

##### Navegación
* Las fórmulas MathML en EPUB y HTML se representan como AsciiMath usando MathCAT. Usa `M` o `Shift+M` para navegar fórmulas, luego `Enter` o `Space` para abrir el MathML original en Vista de fórmulas.
* Un botón Buscar todo en el diálogo Buscar, que lista cada línea con una coincidencia para que puedas saltar directamente a la que quieras.
* Vistas de tablas, listas y páginas en la lista de elementos (`F7`).
* Ir a línea, Ir a página e Ir a porcentaje ahora aceptan `+n` y `-n` para moverte relativamente desde donde estés.
* Libros EPUB, MOBI y CHM sin encabezados propios ahora obtienen navegación de encabezados de su tabla de contenidos.
* Los libros KF8 (AZW3) ahora admiten navegación de secciones.
* Las páginas EPUB que son solo una imagen ahora muestran una línea para ella, para que puedas llegar a ellas en lugar de saltarlas directamente.

##### Audiolibros
* Controles de velocidad de reproducción, desde media velocidad hasta tres veces más rápido. Usa `Ctrl+Shift+.` y `Ctrl+Shift+,`, o el menú Herramientas.
* Los marcadores y notas en libros solo de audio ahora recuerdan la hora exacta en que los estableciste.
* Posición siguiente y anterior (`Alt+Left` y `Alt+Right`) ahora funcionan en audiolibros.
* El progreso a través de un audiolibro ahora se mide por su grabación, por lo que Ir a porcentaje y la barra de estado coinciden con cuán avanzado estés realmente.

##### Documentos recientes
* Un elemento Borrar documentos recientes en el submenú Documentos recientes.

##### Documentos PDF
* Una configuración para mantener cada línea de un PDF separada, en lugar de unirlas en párrafos.
* Las imágenes y figuras en PDFs ahora se anuncian.
* Los PDFs que llevan estructura de lectura pero no etiquetan ninguna de sus imágenes ahora anuncian esas imágenes, en lugar de omitirlas completamente del libro.

##### Vista web
* Cualquier documento ahora puede abrirse en la vista web, no solo EPUB, HTML y Markdown.

##### Legibilidad
* Los encabezados ahora se dibujan en un tamaño que coincide con su nivel, y las imágenes y tablas se separan del texto que las rodea.

##### pb
* `pb --list-formats` lista todos los formatos que pb puede leer.
* pb ahora indica qué archivo no pudo leer y por qué.

#### Corregido

##### General
* Un libro reabierto al iniciar ahora se lee inmediatamente, en lugar de permanecer en silencio hasta que se cierre y se abra de nuevo.
* Un documento cuyo archivo se ha perdido ahora se puede eliminar de Todos los documentos, en lugar de permanecer en la lista sin importar cuántas veces confirmes.
* Se corrigió un bloqueo al cerrar Paperback.
* Cerrar Paperback ahora oculta la ventana inmediatamente, en lugar de dejarla en pantalla mientras se guarda.
* Los libros grandes con poco formato ahora se abren en aproximadamente la mitad del tiempo.
* Los mensajes elegidos de un menú, como "Este documento no tiene audio", ya no se cortan por el lector de pantalla antes de que los escuches.
* Abrir un documento ya no deja habilitada la opción Reabrir el último cerrado cuando no hay nada que reabrir.
* Paperback ya no sigue reintentando documentos en tu lista reciente que se han perdido, y limita cuántos documentos recientes almacena.
* El archivo de configuración INI antiguo ahora se elimina una vez que se ha movido al nuevo formato.
* Los títulos de los diálogos de fuente y color, y el menú Exportar como en vietnamita, ahora están traducidos.
* La actualización ahora trae la ventana relanzada al frente, en lugar de dejarla detrás de todas las demás ventanas en `Alt+Tab`.
* El ajuste de línea ahora se aplica inmediatamente en documentos grandes, en lugar de recargar todo.

##### Navegación
* `Alt+Left` ahora vuelve a donde saltaste, en lugar de a una posición anterior.
* Los sonidos de marcapáginas ahora solo se reproducen cuando te mueves sobre un marcapáginas, no cuando aterrizas en la línea donde se encuentra.
* Cerrar la tabla de contenidos, la lista de elementos y los diálogos Ir ahora te lleva directamente a la línea en la que aterrizas, en lugar de hacerte escuchar que el lector de pantalla lea la ventana de nuevo.
* Ir a línea, Ir a página e Ir a porcentaje ahora rechazan números fuera del documento en lugar de ir silenciosamente a otro lugar.
* NVDA ya no corta el anuncio cuando un documento no tiene páginas.
* Presionar Aceptar en la tabla de contenidos sin movimiento ahora va a la entrada que ya estaba seleccionada.
* La tabla de contenidos, la lista de elementos y la lista de marcapáginas ya no se retrasan ni se cuelgan en libros con miles de entradas.
* Las flechas Arriba y Abajo ahora recuerdan su columna por documento, en lugar de llevarla cuando cambias de pestaña.

##### Audiolibros
* La reproducción de audio ahora usa `Control+Space` en macOS, ya que `Command+Space` pertenece a Spotlight.

##### Documentos PDF
* Se corrigieron los PDF exportados desde Apple Pages que se leían como texto sin formato, sin los encabezados y listas con los que se escribieron.
* Se corrigieron los párrafos y encabezados PDF que se dividían en cada línea, y palabras que se dividían en espacios.
* Se corrigieron los encabezados PDF numerados que se ejecutaban juntos en un encabezado.
* Se corrigieron los PDF cuyo árbol de estructura no lleva a ningún texto y se abrían vacíos.
* Las líneas configuradas en una fuente monoespaciada, como código, ya no se unen en párrafos.
* Los encabezados y pies de página ya no se leen en cada página de PDF sin etiquetar.
* Los PDF que etiquetan sus encabezados y pies de página como texto ordinario ya no repiten el título y el número de página entre dos párrafos en cada página.
* Los PDF ahora muestran su título real, en lugar del nombre de archivo.

##### Libros MOBI/AZW3
* Los libros MOBI grandes ya no se quedan sin memoria, y ya no se cortan después de 20 MB.
* Los libros MOBI y AZW3 ahora se abren mucho más rápido.
* Se corrigió la pérdida de la lista de capítulos en libros MOBI.
* Se corrigió el texto distorsionado donde los libros MOBI cambian de un registro a otro.

##### Vista web
* La vista web ya no carga todo un libro enorme a la vez.
* La vista web ahora muestra documentos completos cuando el lector los muestra completos, en lugar de solo una parte de ellos.

##### Otros formatos
* Los libros FictionBook (.fb2) escritos en windows-1251, que es la mayoría de ellos, ahora se abren en lugar de no poder leerse.
* Los libros FictionBook que usan un espacio de nombres o una entidad HTML que nunca declararon ahora se abren, en lugar de ser rechazados como rotos.
* Los libros en codificaciones heredadas ahora se abren mucho más rápido.
* Se corrigió que algunos archivos de texto en chino se abrieran como texto distorsionado.
* Los archivos OpenDocument protegidos con contraseña ahora solicitan su contraseña, en lugar de ser reportados como rotos.
* Los archivos PowerPoint heredados protegidos con contraseña ahora se abren, y las diapositivas PowerPoint heredadas ya no pierden su texto.
* Los archivos de texto sin formato guardados con una extensión `.rtf` ahora se abren como texto, en lugar de fallar con un error.
* Las palabras de control RTF ya no aparecen como texto.

#### iOS y Android

Las aplicaciones de iOS y Android abren todos los formatos que hace el escritorio, e incluyen:

* Lectura en voz alta, con tu opción de voz, velocidad y tono, un control de velocidad de habla en la barra de lectura y una pausa opcional entre párrafos.
* Reproducción de audiolibros DAISY, M4B y MP3, que continúa en segundo plano y desde la pantalla de bloqueo.
* Navegación por encabezados, páginas, enlaces, tablas, listas y más desde la barra de lectura, además de la tabla de contenidos y Buscar.
* Un temporizador de sueño, recuento de palabras y exportación de documentos, además de un diccionario de habla en iOS. En iOS, la exportación se realiza a través de la hoja de compartir, por lo que un libro puede ir a otra aplicación o a Archivos, en otro formato o exactamente como es.
* Opciones de tamaño de texto, espaciado y texto de alto contraste.
* Atajos de teclado que coinciden con el escritorio.

### Versión 0.9.2
* Los audiolibros ya no hacen que el lector de pantalla lea una serie de espacios cuando enfocas el campo de texto.
* Los audiolibros ahora nombran el archivo mientras lo recorres por secciones.
* Los audiolibros ahora informan su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar la Vista Web con `Escape` ya no muestra una alerta de depuración después de haber seguido un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directamente a la línea encontrada, en lugar de obligarte a escuchar al lector de pantalla leyendo la ventana nuevamente cuando el enfoque regresa al libro.
* Se corrigieron los EPUB que llevan un bloque ZIP64 extraviado que se negaban a abrirse con "Invalid local file header".
* Se corrigieron los documentos largos que volvían a su inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en la Vista Web ahora te llevan a la sección a la que apuntan, en lugar de fallar con "File not found".
* El anuncio automático "Document reloaded" ya no interrumpe a tu lector de pantalla a mitad de oración, sino que espera a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora recorre sus opciones en el orden en que aparecen en la pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la línea de eslogan completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y su duración total.

### Versión 0.9.1
* Los sonidos de marcadores y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrir y rastrear su cronología en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares que desaparecían de los documentos RTF, combinando las palabras circundantes.
* Se corrigieron las imágenes RTF que filtraban sus datos sin procesar en el documento como texto desordenado.
* Se corrigió el submenú Documentos recientes que mantenía entradas obsoletas hasta que algo más sucedía para reconstruirlo.
* Los aceleradores de teclado están de vuelta en todas las traducciones, por lo que los menús de ruso tienen acceso desde teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Options ha sido renombrado a Settings, coincidiendo con las aplicaciones móviles y, en macOS, con la convención de la plataforma.
* Paperback ahora recuerda la posición, tamaño y estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora están traducidas, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el archivo ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora pueden ser traducidos.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta de CLI, llamada pb, para convertir rápidamente cualquiera de los formatos compatibles de Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver origen para abrir el código fuente de un documento en una nueva pestaña, útil para editar Markdown por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puede cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, informe de cualquier comportamiento extraño que encuentre.

##### Compatibilidad de plataformas
* ¡Compatibilidad con Windows ARM64!
* ¡Compatibilidad nativa con macOS!
* Un botón para alternar pantalla completa.

##### Diálogo Todos los documentos
* Un botón localizar para ubicar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y una barra de estado, para que pueda filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido de general);
    * Renderizar tablas en línea (nuevo en esta versión, ver a continuación);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafo;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento del menú de ajuste de línea y su correspondiente tecla de atajo.
* Un botón para determinar cómo desea que se muestren las tablas, y se unificó cómo se muestran las tablas en todos los documentos.

##### Navegación
* Compatibilidad con la navegación por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de exploración en lectores de pantalla.
* El atajo de teclado de iguales para anunciar su porcentaje actual a través de un documento.

##### Marcadores
* Marcadores temporales: puede tener uno por documento, y persisten. Use barra diagonal para establecer uno y barra invertida para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer su velocidad de lectura para que esta métrica sea realmente útil.
* Si hay una selección activa cuando abre el diálogo de recuento de palabras, ahora se mostrará cuántas palabras ha seleccionado.

##### Atajos de teclado
* La capacidad de personalizar todos los atajos de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento del menú de exportación para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón cancelar al diálogo de actualización en curso.
* El actualizador ahora valida que el archivo descargado no haya sido manipulado.

##### Vista web
* La vista web ahora se abre en su posición de lectura actual.

##### Libros DAISY
* Compatibilidad con libros DAISY 2.0.
* Compatibilidad con reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible tanto con audio DAISY (incluyendo audio DAISY + texto) como con archivos ZIP de archivos de audio.
* Atajos de teclado y elementos del menú para reproducir/pausar la narración, avanzar y retroceder, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Compatibilidad con listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora son compatibles con tablas.

#### Corregido

##### General
* Los documentos codificados en conjuntos de caracteres CJK heredados, como GBK, Big5 y Shift_JIS, ahora se mostrarán correctamente en lugar de como mojibake.
* "Reabrir última cerrada" intentando reabrir el archivo readme incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* El manejo de archivos en unidades de red de Windows en Paperback: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta resolución.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes reduciendo a la mitad el tamaño de las tablas de índice interno por carácter.

##### Diálogo Todos los documentos
* Escape no cerraba los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualizaba después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se añadirá a tu lista de todos los documentos cuando se abre mediante Shift+F1.
* Eliminar documentos del diálogo de recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de eliminar un documento.

##### Navegación
* La navegación de página anunciaba texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocaban el cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetaban la ventana del documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deberían reproducirse correctamente solo cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de palabras te lanzaba al inicio del documento.

##### Vista web
* El diálogo de vista web no era redimensionable y aparecía con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de etiquetas de código markdown en las notas de la versión.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen integrados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes filepos en libros Mobi dividían etiquetas HTML e insertaban basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 significativamente mejorado.

##### Documentos Word
* Los documentos Word con nombres de estilos específicos de la configuración regional no mostraban correctamente sus encabezados.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no harán que Paperback falle al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office cifrados. Actualmente se admiten Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos de Microsoft Word heredados!
* ¡Se agregó soporte para presentaciones de Powerpoint heredadas!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo `ctrl+q` para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes integradas ahora debería mostrarse correctamente.
* Los documentos CHM ahora admiten correctamente la navegación de enlaces internos.
* Se corrigió que ir a página estuviera desviado por 1.
* Se corrigió que la tecla Escape no funcionara para cerrar el diálogo de apertura como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF solo de imagen se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar por imágenes y figuras con `g`/`Shift+G` y `f`/`Shift+F`, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte DAISY XML, ya que ya no es necesario.
* Se volvió a cambiar a la navegación nativa Win32 de primera letra en el árbol de tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suavemente.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epubs que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no colocaría un espacio entre palabras en casos raros.
* Se corrigieron párrafos divididos en varias líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico para navegación de enlaces y encabezados!
* Las pestañas y saltos de línea RTF ahora se representan exactamente como aparecen en el documento.
* Se volvió a cambiar a la biblioteca pdfium probada y confiable para analizar PDF, haciendo que la representación de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó `Ctrl+Shift+T` para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora admite la selección de múltiples documentos para abrirlos a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigió que las rutas de archivo que contenían caracteres no ASCII (como š, č, ć, ž bosníacos) se corrompieran al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió que el texto de PDF se leyera en el orden incorrecto y espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se añadieron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se añadió un actualizador automático que ahora reemplazará tu versión instalada de Paperback en lugar de simplemente descargar la nueva versión!
* ¡Se añadió retroalimentación de sonido opcional al alcanzar un marcador o una nota, gracias a Andre Louis por los sonidos!
* ¡Se añadió compatibilidad con documentos RTF!
* Se añadió compatibilidad con documentos DAISY XML.
* Se añadió compatibilidad con archivos de Texto OpenDocument plano.
* Se añadió compatibilidad con presentaciones OpenDocument plano.
* Se añadió compatibilidad con separadores usando s y shift+s.
* Cualquier movimiento mayor a 300 caracteres ahora agregará automáticamente al historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en la vista web.
* Se corrigieron las tablas que no se renderizaban correctamente en archivos Markdown.
* Los PDF solo con imágenes ahora te advertirán sobre su existencia cuando intentes cargar uno.
* Se incrustó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar el uso y la navegación.
* Se cambió a Hayro para analizar PDF, lo que genera mayor confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se añadió compatibilidad con tablas para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Intro para ver una en una vista web.
* ¡Se añadió una función básica de renderizado web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se añadió una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se añadió un botón Borrar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de la versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de los botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML e HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que el TOC de epub ocasionalmente te llevara al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error de uno en la navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco al final de sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos de menú relacionados con marcadores así como la lista de elementos ahora se deshabilitan correctamente cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica de negocio de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se añadió compatibilidad con PDF protegidos por contraseña!
* ¡Se añadió una función muy básica de ir a la posición anterior/siguiente! Si presionas Intro en un enlace interno y mueve tu cursor, esa posición ahora será recordada y se puede navegar con alt+left/right arrows.
* ¡Se añadió una lista de elementos! Actualmente solo muestra un árbol de todos los títulos en tu documento o una lista de enlaces, pero hay planes para expandirla en el futuro.
* Se añadió una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigieron enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de TOC de Epub que contenían rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título ni autor.
* Se corrigieron los títulos de algunos capítulos de epub que no aparecían correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones Aceptar/cancelar en el diálogo TOC.
* Se mejoró el manejo de títulos en documentos Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se ha añadido una nueva opción para mostrar el menú de navegación en una forma mucho más compacta en el diálogo de opciones, activada por defecto.
* Se ha añadido una opción para hacer que la navegación por elementos estructurales se envuelva.
* Se ha añadido una opción al menú de herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* Se ha añadido un sistema de actualización bastante simple, pero muy efectivo.
* Se ha añadido una función básica de temporizador de descanso, accesible con Ctrl+Shift+S.
* ¡Se ha añadido soporte para analizar libros electrónicos FB2!
* ¡Se ha añadido soporte para analizar presentaciones OpenDocument!
* ¡Se ha añadido soporte para analizar archivos OpenDocument Text!
* Los marcadores ahora pueden marcar una línea completa, o marcar solo algún texto especificado. Si no tienes ninguna selección activa al colocar un marcador, el comportamiento es como el anterior a 0.6, y marcará la línea completa. Sin embargo, si seleccionas algún texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas a ellos! Navega entre marcadores que contengan notas con N y Shift+N, o abre el diálogo de marcadores con todos los marcadores, solo notas, o solo no notas seleccionados con teclas de acceso rápido específicas.
* Los marcadores en el diálogo de marcadores ya no tendrán un molesto prefijo "marcador x".
* Los libros Epub que contienen contenido HTML fingiendo ser XML ahora se manejarán correctamente.
* Se ha corregido la carga de documentos Markdown grandes.
* Se ha corregido el problema de presionar espacio en la vista de árbol de la tabla de contenidos activando el botón Aceptar.
* Se ha corregido el manejo de espacios en blanco al principio de etiquetas pre tanto en documentos HTML como XHTML.
* Se ha corregido el problema del control de texto que a veces no recupera el enfoque al volver a la ventana de Paperback.
* Se ha corregido el campo de texto en el diálogo de ir al porcentaje que no actualiza el valor del control deslizante.
* Se ha corregido la representación de ID HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se representará correctamente.
* Si carga un libro con un parámetro de línea de comandos mientras se ejecuta una instancia de Paperback existente, ya no obtendrá un error si la carga de su documento tarda más de 5 segundos.
* Si ejecuta Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde el diálogo de marcadores.
* Ahora es posible importar y exportar sus marcadores y posición de lectura para un documento en particular. El archivo generado se nombra según el archivo con una extensión .paperback. Si se encuentra un archivo de este tipo en el mismo directorio que un archivo mientras se carga, se cargará automáticamente. De lo contrario, puede importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Usa k y shift+k para avanzar y retroceder entre ellos, y presiona entrar para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el archivo binario más pequeño.
* El contenido Markdown ahora se preprocesa para ser compatible con CommonMark antes de representarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Usa L y Shift+L para ir por las listas mismas, e I y Shift+I para recorrer elementos de lista.
* Suprimir del teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además de suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está desactivada por defecto, pero activarla hará que la opción minimizar en el menú del sistema coloque Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que soporta actualmente es bastante pequeña, ¡pero está creciendo constantemente!
* Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha ampliado dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora mostrará una cantidad personalizable, con el resto de documentos que hayas abierto siendo accesibles a través de un pequeño diálogo.
* Varias pequeñas mejoras en los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de saltos de línea dentro de párrafos en documentos de Word, y añadir viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se agregó compatibilidad con documentos de Microsoft Word!
* ¡Se agregó compatibilidad con presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú que no se deshabilitaban sin documentos abiertos.
* Se corrigió la orientación del control deslizante de ir al porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió el espaciado que se eliminaba de encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenidos! Cuando cargue un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los encabezados en su documento, y se la mostrará en el diálogo `ctrl+t`.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta title, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región en vivo para reportar voz. Esto significa que no se envían más DLL de lectores de pantalla junto con el programa, y ahora se admitirán más lectores de pantalla, como Microsoft Narrator.
* Se cambiaron las bibliotecas zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que le pregunta si desea abrir su documento como texto sin formato ha sido completamente rehecho, y ahora le permite abrir su documento como texto sin formato, HTML o Markdown.
* El diálogo de ir al porcentaje ahora incluye un campo de texto que le permite ingresar manualmente un porcentaje al que saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se conservará exactamente una vez más.
* El espacio sin salto Unicode ahora se considera al eliminar líneas en blanco.
* Ya no se le preguntará cómo desea abrir un archivo no reconocido cada vez que lo cargue, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú de inicio opcional al instalador.
* La tabla de contenidos debería ser más limpia en algunos casos; por ejemplo, si tiene un elemento hijo y padre con el mismo texto en la misma posición, ahora solo verá el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas.
* Los documentos CHM ahora deben mostrar su título según se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó compatibilidad con archivos CHM!
* ¡Se agregó compatibilidad con marcadores! Puede tener tantos marcadores como desee en tantos documentos como desee. Puede saltar hacia adelante y hacia atrás a través de ellos con `b` y `shift+b`, establecer uno con `control+shift+b`, y abrir un diálogo para saltar a un marcador específico con `control+b`.
* ¡Se agregó un instalador junto al archivo zip portátil! El instalador instalará Paperback en su directorio Archivos de programa y configurará automáticamente las asociaciones de archivo para usted.
* Los archivos de texto con BOM ahora se decodifican correctamente, y el BOM ya no se muestra al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora le mostrará su línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasa una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja mediante su propio diálogo basado en control deslizante, accesible con `control+shift+g`.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debe escribir en el disco cuando sea absolutamente necesario.
* El documento que tenía enfocado cuando cerró Paperback ahora se recuerda entre reinicios de la aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debe ser desinfectada más estrictamente.
* Se corrigió la navegación de la tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron elementos de TOC anidados en libros Epub poniendo su cursor en la posición incorrecta.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de palabras!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace patrocinar este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambiaron las bibliotecas PDF a la utilizada en Chromium, lo que conduce a un análisis de PDF mucho más confiable en general.
* Ahora solo puede tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puede presionar suprimir en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permitir tabular desde el contenido del documento a su lista de documentos abiertos.
* Se corrigieron los pulsaciones de tecla de encabezado que a veces abrían documentos recientes si tenía suficientes.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados que a veces lo ponía en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos Markdown!
* ¡Se agregó soporte para documentos PDF, incluida la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de teclas para navegar por encabezados en contenido HTML, incluidos libros EPUB y documentos Markdown. Estas pulsaciones de teclas fueron diseñadas para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de EPUB con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros EPUB 3 con XHTML incrustado en ellos.
* Ahora se reproduce un mensaje si el documento no admite una tabla de contenidos o secciones, en lugar de que los elementos del menú se deshabiliten.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena sus últimos 10 documentos abiertos, y presionar Intro en uno los abrirá para lectura.
* Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras se agregó un historial de sus últimas 25 búsquedas y soporte para expresiones regulares.
* Los documentos abiertos anteriormente ahora se recuerdan en los reinicios de la aplicación. Esto se puede configurar a través del nuevo elemento de opciones en el menú herramientas.
* Se agregó `Shift+F1` para abrir el archivo Léame directamente en Paperback.

### Versión 0.1.0
* Versión inicial.
