<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc); please review and edit as needed -->

# Paperback - versión 0.9.2

## Introducción

Paperback es un lector ligero, rápido y accesible de libros electrónicos y documentos para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para accesibilidad con lectores de pantalla, velocidad rápida y una experiencia sin bloatware.

## Requisitos del sistema

Paperback actualmente se ejecuta en Windows 10/11 y todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba público planeadas poco después del lanzamiento de escritorio 0.9.0, antes de un lanzamiento unificado 1.0 que cubra las cuatro plataformas.

## Características

* Completamente independiente, sin necesidad de instalar ningún software en tu equipo para empezar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz simple con pestañas, que te permite abrir tantos documentos como desees lado a lado.
* Guarda tu posición de lectura exacta en todos los documentos que abres.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura al siguiente inicio.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de los documentos.
* Incluye un diálogo de búsqueda robusto, con características como historial y compatibilidad con expresiones regulares.
* Puede ejecutarse completamente de forma portátil, o instalarse con asociaciones de archivos configuradas automáticamente.
* Admite una enorme variedad de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, existe un problema conocido para los usuarios de JAWS.

### JAWS y pantallas Braille

Si usas JAWS con una pantalla Braille, es posible que encuentres que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de tu pantalla. El comando de leer párrafo actual también se ve afectado. Se trata de un error en el manejo de JAWS del control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tomó bastante tiempo para que Vispero mostrara entusiasmo en responder a problemas con software de código abierto.

La solución, finalmente identificada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente soportados

Paperback admite los siguientes formatos y extensiones:

* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos de Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolibros M4B (`.m4b`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto plano y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para un uso basado principalmente en el teclado. Aquí están los atajos actuales.

Los atajos que aparecen a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left/Right` ya están siendo utilizados por otras convenciones del sistema u otras aplicaciones en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabrir el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo "Todos los Documentos" (desde Documentos Recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS esto está en el menú de la aplicación en su lugar).

### Menú Ir

* `Ctrl+F`: Mostrar el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Buscar siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir a porcentaje.
* `Ctrl+P`: Ir a página (cuando sea compatible con el documento actual).
* `=`: Anunciar tu porcentaje de lectura actual.
* `Alt+Left` (macOS: `Cmd+[`): Retroceder en el historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Avanzar en el historial de navegación.
* `[`: Sección anterior.
* `]`: Sección siguiente.
* `Shift+H`: Encabezado anterior.
* `H`: Encabezado siguiente.
* `Shift+1` a `Shift+6`: Encabezado anterior de nivel 1-6.
* `1` a `6`: Encabezado siguiente de nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Página siguiente.
* `Shift+B`: Marcador anterior.
* `B`: Marcador siguiente.
* `/`: Establecer tu marcador temporal.
* `\`: Ir a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Ir a todos los marcadores y notas.
* `Ctrl+Alt+B`: Ir solo a marcadores.
* `Ctrl+Alt+M`: Ir solo a notas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir, la tecla Control física en lugar de Cmd): Ver texto de nota en la posición actual.
* `Shift+K`: Enlace anterior.
* `K`: Enlace siguiente.
* `Shift+G`: Imagen anterior.
* `G`: Imagen siguiente.
* `Shift+F`: Figura anterior.
* `F`: Figura siguiente.
* `Shift+T`: Tabla anterior.
* `T`: Tabla siguiente.
* `Shift+S`: Separador anterior.
* `S`: Separador siguiente.
* `Shift+L`: Lista anterior.
* `L`: Lista siguiente.
* `Shift+I`: Elemento de lista anterior.
* `I`: Elemento de lista siguiente.
* `Shift+,`: Ir al inicio del contenedor actual (lista o tabla).
* `,`: Ir más allá del final del contenedor actual (lista o tabla).

### Menú Herramientas

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Mostrar recuento de palabras para el documento actual.
* `Ctrl+I`: Mostrar información del documento.
* `Ctrl+T`: Mostrar tabla de contenidos.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir carpeta contenedora.
* `Ctrl+Shift+V`: Abrir contenido actual en Vista Web.
* `Ctrl+U`: Ver la fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto sin formato.
* `Ctrl+Shift+B`: Alternar marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Agregar o editar nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar ajuste de línea.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Avanzar narración de audio.
* `;`: Retroceder narración de audio.
* `Ctrl+'`: Aumentar la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuir la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Alternar temporizador de suspensión.

### Menú Ayuda

* `Ctrl+F1`: Mostrar diálogo Acerca de.
* `F1`: Ver ayuda en tu navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Buscar actualizaciones.
* `Ctrl+D`: Abrir la página de donaciones en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cerrar la pestaña del documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activar enlace en el cursor, o abrir una vista de tabla cuando se está en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas compatibles

Paperback está traducido a muchos idiomas diferentes, y se están agregando más todo el tiempo. A continuación se presenta una lista completa.

Para aprender cómo contribuir, consulta nuestra [Guía de traducción](translating.md).

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
* Vietnamita

## Créditos
### Desarrollo
* Quin Gillespie: desarrollador principal y fundador del proyecto.
* Aryan Choudhary: colaborador principal.

### Donaciones
Las siguientes personas han realizado donaciones de cierto monto al desarrollo de Paperback. Si realizas una donación, tu nombre no se agregará automáticamente aquí; solo agrego personas que desean que su donación sea pública.

Nota: Considero que ser patrocinador público de GitHub es motivo de inclusión automática en esta lista.

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

### Versión 0.9.2
* Los audiolibros ya no hacen que el lector de pantalla lea una serie de espacios cuando enfocas el campo de texto.
* Los audiolibros ahora nombran el archivo a medida que avanzas por secciones.
* Los audiolibros ahora informan su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar la Vista Web con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te proporciona el documento completo, en lugar de solo la parte que está cargada actualmente.
* Buscar ahora te lleva directamente a la línea encontrada, en lugar de hacerte escuchar al lector de pantalla leyendo la ventana de nuevo cuando el enfoque regresa al libro.
* Se corrigieron los EPUBs que tienen un bloque ZIP64 extraño que se niegan a abrirse con "Encabezado de archivo local inválido".
* Se corrigieron los documentos largos que retrocedían a su inicio mientras un lector de pantalla leía continuamente a través de ellos.
* Los enlaces en la Vista Web ahora te llevan a la sección a la que apuntan, en lugar de fallar con "Archivo no encontrado".
* El anuncio automático "Documento recargado" ya no interrumpe al lector de pantalla a mitad de oración, sino que espera a que termine de hablar.
* La pestaña General del diálogo de Configuración ahora se desplaza por sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la frase de eslogan completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y seguir su línea de tiempo en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares desapareciendo de documentos RTF, pegando las palabras circundantes mientras desaparecían.
* Se corrigieron las imágenes RTF filtrando sus datos sin procesar en el documento como texto desordenado.
* Se corrigió el submenú Documentos recientes manteniendo entradas obsoletas hasta que otra cosa ocurriera para reconstruirlo.
* Los aceleradores de teclado están de vuelta en cada traducción, por lo que los menús de ruso tienen acceso por teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Las opciones se han renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda su posición de ventana, tamaño y estado maximizado entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora se pueden traducir.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos compatibles de Paperback a HTML, Markdown o texto plano.
* Una opción para recargar documentos que han sido modificados por otros programas en disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, reporta cualquier rareza encontrada con esto.

##### Compatibilidad de plataforma
* ¡Soporte para Windows ARM64!
* ¡Soporte nativo para macOS!
* Un alternar de pantalla completa.

##### Diálogo Todos los documentos
* Un botón localizar para ubicar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de palabras (movido de general);
    * Renderizar tablas en línea (nuevo en esta versión, ver más abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento del menú de ajuste de palabras y una tecla de acceso rápido posterior.
* Un alternar para determinar cómo deseas que se muestren las tablas, y unificar cómo se muestran las tablas en todos los documentos.

##### Navegación
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo navegación en lectores de pantalla.
* El atajo de teclado igual para anunciar tu porcentaje actual a través de un documento.

##### Marcapáginas
* Marcapáginas temporales: puedes tener uno por documento, y persisten. Usa barra diagonal para establecer uno y barra invertida para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer tu velocidad de lectura para que esta métrica sea realmente útil.
* Si una selección está activa cuando abres el diálogo de recuento de palabras, ahora se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportación
* Expandió el elemento del menú de exportación para permitir exportar a HTML y Markdown, además de texto plano.

##### Actualizador
* Un botón cancelar al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido modificado.

##### Vista Web
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con audio DAISY (incluyendo audio DAISY + texto) y archivos zip de audio.
* Atajos de teclado y elementos del menú para reproducir/pausar narración, buscar hacia adelante y hacia atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora admiten tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Reabre el último cerrado" intentando reabrirse el archivo readme incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alto DPI.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, cuando se abre la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá cuando se cambie entre pestañas.
* Se redujo el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice interno por carácter.

##### Diálogo Todos los documentos
* Escape no cerrando los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualiza después de cerrar un documento desde el diálogo de todos los documentos.
* El archivo Readme.html ya no se agregará a tu lista de todos los documentos cuando se abra a través de Shift+F1.
* Eliminar documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* La navegación de página anunciaba texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetaban la ventana de documento cargado en documentos grandes.

##### Marcapáginas
* Los sonidos de marcapáginas/notas ahora deberían reproducirse correctamente solo cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de palabras te disparaba al inicio de tu documento.

##### Vista Web
* El diálogo de vista web no se puede cambiar de tamaño y aparecía con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de lanzamiento.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen integrados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes filepos en libros Mobi dividían etiquetas HTML y ponían basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 muy mejorado.

##### Documentos de Word
* Los documentos de Word con nombres de estilo específicos de la configuración regional no renderizaban sus títulos correctamente.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto plano para archivos PDF etiquetados incorrectamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcapáginas ya no bloquearán Paperback al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office encriptados. Actualmente se admiten Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos de Microsoft Word heredados!
* ¡Se agregó soporte para presentaciones de Powerpoint heredadas!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes integradas ahora debería mostrarse correctamente.
* Los documentos CHM ahora son compatibles correctamente con la navegación de enlaces internos.
* Se corrigió ir a página siendo incorrecto en 1.
* Se corrigió la tecla Escape no funcionando para cerrar el diálogo Abrir como.
* Se corrigió el menú contextual del lector no mostrándose al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió el documento incorrecto a veces siendo enfocado al abrir documentos desde la línea de comandos.
* Los archivos PDF solo con imágenes se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte de XML DAISY, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primera letra nativa de Win32 en el árbol de tabla de contenidos.
* El diálogo de carga de errores ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suave.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epubs que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no colocaría un espacio entre palabras en casos raros.
* Se corrigieron párrafos divididos en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y títulos!
* Las tabulaciones y saltos de línea RTF ahora se representan exactamente como aparecen en el documento.
* Se volvió a cambiar a la biblioteca pdfium probada y verdadera para analizar PDF, haciendo que el renderizado de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabre el último documento cerrado.
* El diálogo Todos los documentos ahora es compatible con la selección de múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron las rutas de archivo que contienen caracteres no ASCII (como š, č, ć, ž bosnios) corrompiéndose al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió el texto PDF siendo leído en el orden incorrecto y espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional al llegar a un marcapáginas o una nota, gracias a Andre Louis por los sonidos!
* ¡Se agregó soporte para documentos RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de Texto de Documento Abierto plano!
* ¡Se agregó soporte para presentaciones de Documento Abierto plano!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento mayor a 300 caracteres ahora agregará automáticamente al historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en la Vista Web.
* Se corrigió que las tablas no se renderizaran correctamente en archivos Markdown.
* Los archivos PDF solo con imágenes ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrustó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar el uso y la navegación.
* Se cambió a Hayro para analizar PDF, lo que lleva a más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. El nuevo código es más seguro, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tabla para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Intro para ver una en una vista web.
* ¡Se agregó una función básica de renderizado web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se agregó una traducción al ruso, gracias a Ruslan Gulmagomedov!
* Se agregó un botón Borrar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de lanzamiento cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigió la traducción de botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió el diálogo de búsqueda no ocultándose correctamente cuando se usan los botones siguiente/anterior.
* Se corrigió que los TOC de epub ocasionalmente te lanzaran al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error off-by-one en navegación de enlaces.
* Se corrigieron algunos libros con espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos del menú relacionados con marcapáginas así como la lista de elementos ahora están deshabilitados correctamente cuando no hay documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para los colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte para PDF protegido con contraseña!
* Se agregó una función muy básica de ir a posición anterior/siguiente. Si presionas Intro en un enlace interno y mueve tu cursor, esa posición ahora será recordada, y se puede navegar con las flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los títulos en tu documento o una lista de enlaces, pero hay planes para expandirla en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigieron los enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de TOC de Epub que contenían rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigieron los títulos de algunos capítulos epub que no aparecían correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra de espacios para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de títulos en documentos de Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentas abrir el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú ir en una forma mucho más compacta al diálogo de opciones, marcada por defecto.
* Se agregó una opción para hacer que la navegación por elementos estructurales se ajuste.
* ¡Se agregó una opción al menú herramientas para abrir la carpeta contenedora del documento actualmente enfocado!
* Se agregó un sistema de actualización bastante simple, pero muy efectivo.
* ¡Se agregó una función de temporizador de sueño básico, accesible con Ctrl+Shift+S!
* ¡Se agregó soporte para analizar ebooks FB2!
* ¡Se agregó soporte para analizar presentaciones de OpenDocument!
* ¡Se agregó soporte para analizar archivos de Texto de OpenDocument!
* Los marcapáginas ahora pueden marcar una línea completa, o marcar solo parte del texto especificado. Si no tienes selección activa al colocar un marcapáginas, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas a ellos! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con todos los marcapáginas, solo notas o solo no notas seleccionadas con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un molesto prefijo "marcapáginas x".
* Los libros Epub que contienen contenido HTML pretendiendo ser XML ahora serán manejados correctamente.
* Se corrigió la carga de documentos grandes de Markdown.
* Se corrigió presionar espacio en el árbol de vista de la tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al principio de etiquetas pre tanto en documentos HTML como XHTML.
* Se corrigió el control de texto no recuperando el enfoque a veces cuando se regresa a la ventana de Paperback.
* Se corrigió el campo de texto en el diálogo de porcentaje ir a no actualizar el valor del deslizador.
* Se corrigió el renderizado de ID HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargabas un libro con un parámetro de línea de comandos mientras una instancia de Paperback existente estaba ejecutándose, ya no obtendrás un error si la carga de tu documento tarda más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde el diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento en particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra un archivo de este tipo en el mismo directorio que un archivo mientras lo carga, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Usa k y shift+k para moverse hacia adelante y hacia atrás a través de ellos, y presiona Intro para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido de Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Usa L y Shift+L para pasar por las listas mismas, e I y Shift+I para atravesar elementos de lista.
* Suprimir en el teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además del Suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está deshabilitada por defecto, pero activarla hará que la opción minimizar en el menú del sistema coloque Paperback en tu bandeja, que se puede restaurar haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que admite actualmente es bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha ampliado dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, con el resto de los documentos que alguna vez has abierto siendo accesibles a través de un pequeño diálogo.
* Varias pequeñas mejoras a los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de nuevas líneas dentro de párrafos en documentos word, y agregar viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte de documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos del menú no siendo deshabilitados sin documentos abiertos.
* Se corrigió la orientación del deslizador de porcentaje ir a.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o ID de fragmento.
* Se corrigió el espaciado en blanco siendo eliminado de títulos XHTML de formas raras.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora son compatibles con la función de tabla de contenidos! Cuando cargues un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los títulos en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título tal como se establece en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre del archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar lenguaje. Esto significa que no se envían DLL de lector de pantalla junto con el programa, y se admitirán más lectores de pantalla, como Microsoft Narrator.
* Se cambió la biblioteca zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que te pregunta si deseas abrir tu documento como texto plano ha sido completamente rehecho, y ahora te permite abrir tu documento como texto plano, HTML o Markdown.
* El diálogo de porcentaje ir a ahora incluye un campo de texto que te permite introducir manualmente un porcentaje para saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se conservará una vez más exactamente.
* El espacio sin ruptura unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te pedirá cómo deseas abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú Inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo si tienes un elemento secundario y principal con el mismo texto en la misma posición, ahora solo verás el elemento principal.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título tal como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte de archivos CHM!
* ¡Se agregó soporte para marcapáginas! Puedes tener tantos marcapáginas como desees en tantos documentos como desees. Puedes saltar hacia adelante y hacia atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y abrir un diálogo para saltar a un marcapáginas específico con control+b.
* ¡Se agregó un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente asociaciones de archivos para ti.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja por su propio diálogo basado en deslizador, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda en los reinicios de la aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería ser desinfectada más estrictamente.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de títulos en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron los elementos TOC anidados en libros Epub colocando el cursor en la posición incorrecta.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de palabras!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú ayuda o a través del enlace de patrocinador este proyecto al final de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si falta la metadata.
* Se cambió la biblioteca PDF a la utilizada en Chromium, lo que lleva a un análisis de PDF mucho más confiable en general.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya está en ejecución abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar Suprimir en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permitir hacer tabulaciones del contenido del documento a tu lista de documentos abiertos.
* Se corrigieron los pulsaciones de tecla de título a veces abriendo documentos recientes si tienes suficientes.
* Paperback ahora eliminará guiones suaves innecesarios de la salida de texto.
* Se corrigió la navegación de título a veces colocándote en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de tecla para navegar por títulos en contenido HTML, incluyendo libros epub y documentos markdown. Estas pulsaciones de tecla fueron diseñadas para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de epubs con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML integrado dentro de ellos.
* Se habla un mensaje si el documento no es compatible con una tabla de contenidos o secciones, en lugar de que los elementos del menú estén deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar Intro en uno lo abrirá para lectura.
* ¡Se reescribió completamente el diálogo de búsqueda, haciéndolo mucho más simple de usar, mientras se agregaba un historial de tus últimas 25 búsquedas y soporte de expresiones regulares!
* Los documentos abiertos anteriormente ahora se recuerdan en los reinicios de la aplicación. Esto es configurable a través del nuevo elemento opciones en el menú herramientas.
* Se agregó shift+f1 para abrir el archivo readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
