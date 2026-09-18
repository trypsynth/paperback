<!-- machine-translated from doc/readme.md (source-hash: 11f05688d690d71a; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,2fb18876,71df8e94,e9860ee8,a7ac6234); please review and edit as needed -->

# Paperback - versión 0.9.2

## Introducción

Paperback es un lector ligero, rápido y accesible de libros electrónicos y documentos para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para accesibilidad con lectores de pantalla, velocidad rápida y una experiencia sin elementos innecesarios.

## Requisitos del Sistema

Paperback se ejecuta actualmente en Windows 10/11 y en todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba pública planeadas poco después del lanzamiento de la versión 0.9.0 de escritorio, antes de un lanzamiento unificado de la versión 1.0 que cubra las cuatro plataformas.

## Características

* Completamente independiente, sin requerir que instales ningún software en tu computadora para empezar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz simple con pestañas, permitiéndote abrir tantos documentos como desees lado a lado.
* Guarda tu posición exacta de lectura en cada documento que abres.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura en el próximo lanzamiento.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de los documentos.
* Incluye un diálogo de búsqueda robusto, con características como historial y compatibilidad con expresiones regulares.
* Puede ejecutarse completamente de forma portátil, o instalarse con asociaciones de archivos configuradas automáticamente.
* Admite una gran variedad de formatos de archivo comunes.

## Compatibilidad con Lectores de Pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, hay un problema conocido para los usuarios de JAWS.

### JAWS y Pantallas Braille

Si utilizas JAWS con una pantalla Braille, es posible que encuentres que los párrafos largos se truncan al avanzar con las teclas de navegación de tu pantalla. El comando de lectura del párrafo actual también se ve afectado. Se trata de un error en la forma en que JAWS maneja el control de texto RICHEDIT50W, no algo en Paperback, y uno que tardó bastante tiempo en encontrar una solución dado el entusiasmo de Vispero por responder a los problemas del software de código abierto.

La solución alternativa, finalmente surgida a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Formatos de archivo actualmente admitidos

Paperback admite los siguientes formatos y extensiones:

* Archivos de cómics (`.cbz`, `.cbr`)
* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Páginas de manual, tanto `man` como BSD `mdoc` (`.1` a `.9`, `.man`, `.roff` y las versiones comprimidas de cada uno)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos de Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolibros M4B (`.m4b`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos WinHelp (`.hlp`)
* Archivos de texto sin formato y archivos de registro (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para el uso prioritario del teclado. Aquí están los atajos actuales.

Los atajos siguientes son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque Ctrl+G, Ctrl+W, y Alt+Left/Right ya están siendo utilizados por otras convenciones de sistema u aplicación en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabre el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Salir (Solo Windows; en macOS esto está en el menú de aplicación).

### Menú Ir

* `Ctrl+F`: Mostrar el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Buscar siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir a porcentaje.
* `Ctrl+P`: Ir a página (cuando es soportado por el documento actual).
* `=`: Anuncia tu porcentaje de lectura actual y página, por ejemplo "15%, página 30". La página se omite para documentos sin números de página.
* `Alt+Left` (macOS: `Cmd+[`): Volver al historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Avanzar en el historial de navegación.
* `[`: Sección anterior.
* `]`: Siguiente sección.
* `Shift+H`: Encabezado anterior.
* `H`: Siguiente encabezado.
* `Shift+1` hasta `Shift+6`: Encabezado anterior de nivel 1-6.
* `1` hasta `6`: Siguiente encabezado de nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Siguiente página.
* `Shift+B`: Marcador anterior.
* `B`: Siguiente marcador.
* `/`: Establecer tu marcador temporal.
* `\`: Saltar a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Siguiente nota.
* `Ctrl+B`: Saltar a todos los marcadores y notas.
* `Ctrl+Alt+B`: Saltar solo a marcadores.
* `Ctrl+Alt+M`: Saltar solo a notas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir la tecla Control física en lugar de Cmd): Ver el texto de la nota en la posición actual.
* `Shift+K`: Enlace anterior.
* `K`: Siguiente enlace.
* `Shift+G`: Imagen anterior.
* `G`: Siguiente imagen.
* `Shift+F`: Figura anterior.
* `F`: Siguiente figura.
* `Shift+T`: Tabla anterior.
* `T`: Siguiente tabla.
* `Shift+M`: Fórmula anterior.
* `M`: Siguiente fórmula.
* `Shift+S`: Separador anterior.
* `S`: Siguiente separador.
* `Shift+L`: Lista anterior.
* `L`: Siguiente lista.
* `Shift+I`: Elemento de lista anterior.
* `I`: Siguiente elemento de lista.
* `Shift+,`: Ir al inicio del contenedor actual (lista o tabla).
* `,`: Pasar el final del contenedor actual (lista o tabla).

### Menú Herramientas

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir la tecla Control física en lugar de Cmd): Mostrar el recuento de palabras del documento actual.
* `Ctrl+I`: Mostrar información del documento.
* `Ctrl+T`: Mostrar tabla de contenidos.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir carpeta contenedora.
* `Ctrl+Shift+V`: Abrir contenido actual en Vista Web.
* `Ctrl+U`: Ver la fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto plano.
* `Ctrl+Shift+B`: Alternar marcador en la selección actual/cursor.
* `Ctrl+Shift+N`: Añadir o editar nota de marcador en la selección actual/cursor.
* `Ctrl+Alt+W`: Alternar ajuste de palabras.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Adelantar narración de audio.
* `;`: Retroceder narración de audio.
* `Ctrl+'`: Aumentar la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuir la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir Control+Comando+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de aplicación).
* `Ctrl+Shift+S`: Alternar temporizador de sueño.
* `Alt+F9` (macOS: `Cmd+F9`): Marca el inicio de una selección, para que todo desde aquí hasta donde llegues pueda copiarse de una vez.
* `Alt+F10` (macOS: `Cmd+F10`): Copia todo desde el inicio marcado de la selección hasta la posición actual.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Saltar de vuelta al inicio marcado de la selección, dejando la marca en su lugar.

### Menú Ayuda

* `Ctrl+F1`: Mostrar diálogo Acerca de.
* `F1`: Ver ayuda en tu navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Buscar actualizaciones.
* `Ctrl+D`: Abrir la página de donación en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cerrar la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Seguir un enlace o abrir una vista de tabla o fórmula en el cursor.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas admitidos

Paperback está traducido a muchos idiomas diferentes, y se añaden más constantemente. A continuación hay una lista completa.

Para aprender cómo contribuir, lee nuestra [Guía de traducción](translating.md).

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
* Aryan Choudhary: contribuidor principal.

### Donaciones
Las siguientes personas han realizado donaciones de algún tipo al desarrollo de Paperback. Si realizas una donación, tu nombre no se añadirá automáticamente aquí; solo añado personas que quieren que su donación sea pública.

Nota: considero que ser patrocinador público en GitHub es motivo para la inclusión automática en esta lista.

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
* Los audiolibros ya no hacen que tu lector de pantalla lea una serie de espacios cuando enfocas el campo de texto.
* Los audiolibros ahora nombran el archivo mientras avanzan por secciones.
* Los audiolibros ahora informan su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar la Vista Web con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directamente a la línea encontrada, en lugar de hacer que el lector de pantalla lea la ventana nuevamente mientras el enfoque vuelve al libro.
* Se corrigió que EPUBs que contienen un bloque ZIP64 huérfano se negaran a abrirse con "Encabezado de archivo local inválido".
* Se corrigió que documentos largos volvieran al inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en WebView ahora te llevan a la sección a la que apuntan, en lugar de fallar con "Archivo no encontrado".
* Marca el comienzo de una selección con `Alt+F9`, copia todo desde allí hasta donde hayas llegado con `Alt+F10`, y vuelve a la marca con `Alt+Shift+F9`, para copiar un largo tramo de texto sin usar mayús+flecha. Los tres están en Herramientas > Seleccionar y copiar.
* El atajo `=` ahora anuncia la página además del porcentaje, por ejemplo "15%, página 30", y se mantiene como estaba para documentos sin números de página.
* El anuncio automático "Documento recargado" ya no interrumpe tu lector de pantalla a mitad de oración, esperando a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora navega por sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Actualizar ahora trae la ventana reiniciada al frente, en lugar de dejarla detrás de todas las demás ventanas en Alt+Tab.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la etiqueta completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrir y rastrear su línea de tiempo en silencio.
* Se corrigió que las comillas rizadas, rayas largas y caracteres similares desaparecieran de documentos RTF, uniendo las palabras circundantes.
* Se corrigió que las imágenes RTF filtraran sus datos sin procesar al documento como texto garbled.
* Se corrigió que el submenú Documentos recientes mantuviera entradas obsoletas hasta que algo más lo reconstruyera.
* Los aceleradores de teclado están de vuelta en cada traducción, por lo que los menús de Ruso tienen acceso de teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y la lista reciente del menú Inicio.
* Las opciones se han renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda su posición de ventana, tamaño y estado maximizado entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora se pueden traducir.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora se traduce.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados de Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor reporta cualquier rareza encontrada con esto.

##### Soporte de plataforma
* ¡Soporte para Windows ARM64!
* ¡Soporte nativo para macOS!
* Un toggle de pantalla completa.

##### Diálogo Todos los documentos
* Un botón de localizar para ubicar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, por lo que puedes filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de palabras (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de líneas;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de palabras y atajo de teclado posterior.
* Un toggle para determinar cómo deseas que se muestren las tablas, y unificó cómo se muestran las tablas en documentos.

##### Navegación
* Las fórmulas MathML en EPUB e HTML se renderizan como AsciiMath usando MathCAT. Usa `M` o `Shift+M` para navegar fórmulas, luego `Enter` o `Space` para abrir el MathML original en Vista de fórmula.
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de navegación en lectores de pantalla.
* El atajo de teclado equals para anunciar tu porcentaje actual a través de un documento.

##### Marcapáginas
* Marcapáginas temporales: puedes tener uno por documento, y sí persisten. Usa barra oblicua para establecer uno y barra oblicua inversa para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, además de la capacidad de establecer tu velocidad de lectura para hacer esta métrica realmente útil.
* Si hay una selección activa cuando abres el diálogo de recuento de palabras, se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento del menú de exportación para permitir la exportación a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón de cancelación al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido manipulado.

##### Vista Web
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente soportando tanto DAISY de audio (incluyendo DAISY audio + texto) como zips de archivos de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, buscar hacia adelante y hacia atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio, y elegir si buscar más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos PowerPoint ahora soportan tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de un montón de mojibake.
* "Reabrir el último cerrado" intentando reabrir el archivo readme incluido.
* Tu pestaña seleccionada no se enfoca correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en la carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta DPI.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se lee al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice interno por carácter.

##### Diálogo Todos los documentos
* Escape no cierra los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualiza después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se agregará a tu lista de todos los documentos cuando se abre a través de Shift+F1.
* Remover documentos del diálogo recientes ahora también cierra su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de remover un documento.

##### Navegación
* La navegación de página anuncia texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocan tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetan la ventana de documento cargado en documentos grandes.

##### Marcapáginas
* Los sonidos de marcapáginas/notas ahora deberían reproducirse correctamente de manera exclusiva cuando navegas sobre una palabra que contiene uno.

##### Legibilidad
* Aplicar ajuste de palabras te dispara al inicio de tu documento.

##### Vista Web
* El diálogo de vista web no era redimensionable y aparecía en un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de lanzamiento.

##### Libros DAISY
* Los libros DAISY muestran información incorrecta en la barra de estado.
* Cargar libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Los grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren al texto del documento.

##### Libros Mobi/AZW3
* Los anclajes filepos en libros Mobi dividen etiquetas HTML y ponen basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 enormemente mejorado.

##### Documentos Word
* Los documentos Word con nombres de estilos específicos de configuración regional no renderizan correctamente sus encabezados.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producen saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora vuelve a la extracción de texto sin formato para PDFs etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcapáginas ya no bloquearán Paperback al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos cifrados de Microsoft Office. Actualmente se soportan Word heredado y Word moderno, y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos heredados de Microsoft Word!
* ¡Se agregó soporte para presentaciones heredadas de Powerpoint!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora se debe mostrar correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió que ir a página estuviera desactivado por 1.
* Se corrigió que la tecla escape no funcionara para cerrar el diálogo de abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento equivocado al abrir documentos desde la línea de comandos.
* Los PDFs solo con imágenes se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/mayús+g y f/mayús+f, respectivamente.
* Paperback ahora respetará tu configuración del modo oscuro de la aplicación.
* Se eliminó el soporte de DAISY XML, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primera letra nativa de Win32 en el árbol de la tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suavemente.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epubs que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no colocaría un espacio entre palabras en casos raros.
* Se corrigió que los párrafos se dividieran en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las tabulaciones y saltos de línea RTF ahora se renderizan exactamente como aparecen en el documento.
* Se volvió a cambiar a la librería pdfium probada y comprobada para analizar PDFs, haciendo que la representación de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora soporta seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores en el analizador RTF.
* Se corrigió que las rutas de archivo que contienen caracteres no ASCII (como el š, č, ć, ž bosnio) se corrompieran al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió que el texto PDF se leyera en el orden equivocado, y espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional por alcanzar un marcapáginas o una nota, gracias a Andre Louis por los sonidos!
* ¡Se agregó soporte para documentos RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de Texto de Documento Abierto Plano!
* ¡Se agregó soporte para presentaciones de Documento Abierto Plano!
* Se agregó soporte para separadores con s y mayús+s.
* Cualquier movimiento mayor a 300 caracteres ahora agregará automáticamente a tu historial de navegación.
* Se corrigió restaurar la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin formato en lugar de HTML renderizado en la Vista Web.
* Se corrigió que las tablas no se renderizaran correctamente en archivos Markdown.
* Los PDFs solo con imágenes ahora te advertirán de su existencia cuando intentes cargar uno.
* Incrustar correctamente información de versión en el ejecutable de Paperback.
* Dividir el diálogo de opciones en pestañas para facilidad de uso y navegación.
* Se cambió a Hayro para analizar PDFs, lo que lleva a más confiabilidad, velocidad y menos DLLs.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido, y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tabla para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Enter para ver una en una vista web.
* ¡Se agregó una característica básica de representación web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se agregó una traducción al ruso, gracias a Ruslan Gulmagomedov!
* Se agregó un botón Limpiar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de lanzamiento cuando una nueva versión está disponible.
* Se corrigió restaurar la ventana desde la bandeja del sistema.
* Se corrigió las traducciones de botones Sí/No en diálogos de confirmación.
* Se corrigió cargar configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML e HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió navegar al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que los TOC de epub ocasionalmente te lanzaran al elemento equivocado.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error de 1 desactivado en navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos del menú relacionados con marcapáginas, así como la lista de elementos, ahora se deshabilitan correctamente cuando no hay documento abierto.
* Se mejoró el manejo de listas en varios formatos de documentos.
* Se mejoró el flujo de trabajo de traducción para contribuyentes.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica comercial de la aplicación de C++ a Rust para mejor rendimiento y mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte para PDF protegido con contraseña!
* Se agregó una característica muy básica de ir a posición anterior/siguiente. Si presionas enter en un enlace interno y mueve tu cursor, esa posición ahora será recordada, y se puede navegar a ella con flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirla en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigió que los enlaces en algunos documentos Epub no funcionaran correctamente.
* Se corrigió el análisis de TOCs de Epub que contienen rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigió que los títulos de algunos capítulos epub no aparecieran correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Una nueva opción para mostrar el menú de ir en una forma mucho más compacta se ha agregado al diálogo de opciones, marcada por defecto.
* Se agregó una opción para que la navegación por elementos estructurales se ajuste.
* Se agregó una opción al menú de herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* ¡Se agregó un sistema de actualización bastante simple pero muy efectivo!
* ¡Se agregó una característica básica de temporizador de sueño, accesible con Ctrl+Shift+S!
* ¡Se agregó soporte para analizar libros FB2!
* ¡Se agregó soporte para analizar presentaciones de OpenDocument!
* ¡Se agregó soporte para analizar archivos de Texto de OpenDocument!
* Los marcapáginas ahora se pueden hacer para marcar una línea completa, o para marcar solo texto específico. Si no tienes ninguna selección activa cuando colocas un marcapáginas, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas algo de texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con todos los marcapáginas, solo notas, o solo no-notas seleccionados con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un prefijo molesto "marcapáginas x".
* Los libros Epub que contienen contenido HTML haciéndose pasar por XML ahora se manejarán correctamente.
* Se corrigió cargar documentos Markdown grandes.
* Se corrigió presionar espacio en el árbol de vista de la tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al comienzo de etiquetas pre en documentos HTML y XHTML.
* Se corrigió que el control de texto no recuperara el enfoque a veces cuando volvías a la ventana de Paperback.
* Se corrigió que el campo de texto en el diálogo ir a porcentaje no actualice el valor del control deslizante.
* Se corrigió la representación de IDs HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras una instancia de Paperback existente se está ejecutando, ya no obtendrás un error si cargar tu documento toma más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde el diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento en particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra tal archivo en el mismo directorio que un archivo mientras se carga, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente soportados! Usa k y mayús+k para moverte hacia adelante y hacia atrás a través de ellos, y presiona enter para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido de Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente soportada! Usa L y Shift+L para ir por las listas mismas, e I y Shift+I para ir a través de elementos de lista.
* Ahora Suprimir en el teclado numérico funciona para remover documentos de la barra de pestañas además de suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está desactivada por defecto, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, pudiendo ser restaurada haciendo clic en el icono generado.
* ¡Paperback es ahora totalmente traducible! La lista de idiomas que soporta es actualmente bastante pequeña, pero está creciendo constantemente.
* Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, con el resto de los documentos que has abierto siendo accesibles a través de un pequeño diálogo.
* Varias pequeñas mejoras a los analizadores en toda la junta, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, arreglar el manejo de nuevas líneas dentro de párrafos en documentos word, y agregar puntos de viñeta a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte para documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú que no se deshabilitaban sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió que los espacios en blanco se eliminaran de encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora soportan la función de tabla de contenidos! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos fuera de la estructura de los encabezados en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título tal como se establece en la etiqueta title, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar discurso. Esto significa que no se envían DLLs de lector de pantalla junto con el programa, y más lectores de pantalla ahora serán soportados, como Microsoft Narrator.
* Se cambió de librerías zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que te pregunta si quieres abrir tu documento como texto sin formato ha sido completamente rehecho, y ahora te permite abrir tu documento como texto sin formato, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje al cual saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se preservará una vez más exactamente.
* El espacio sin salto de unicode ahora se considera cuando se eliminan líneas en blanco.
* Ya no se te pedirá cómo quieres abrir un archivo no reconocido cada vez que lo cargas, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono del menú Inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo si tienes un elemento hijo y padre con el mismo texto en la misma posición ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título tal como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcapáginas! Puedes tener tantos marcapáginas como quieras a través de tantos documentos como quieras. Puedes saltar hacia adelante y hacia atrás a través de ellos con b y mayús+b, establecer uno con control+mayús+b, y abrir un diálogo para saltar a un marcapáginas específico con control+b.
* ¡Se agregó un instalador junto con el archivo zip portátil! El instalador instalará Paperback en tu directorio Program Files, y configurará automáticamente asociaciones de archivos para ti.
* Los archivos de texto con BOMs ahora se decodificarán correctamente, y el BOM ya no se mostrará al comienzo del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora mostrará tu línea, carácter y porcentaje de lectura actuales.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja mediante su propio diálogo basado en control deslizante, accesible con control+mayús+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán uno por defecto.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debe escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda entre reinicios de aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debe sanitizarse más estrictamente.
* Se corrigió la navegación de la tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el uso alto de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió cargar archivos de texto UTF-8.
* Se corrigió que los elementos TOC anidados en libros Epub colocaran tu cursor en la posición equivocada.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de palabras!
* Ahora es posible donar al desarrollo de Paperback, a través del nuevo elemento donar en el menú de ayuda o a través del enlace proyecto de patrocinador en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió de librería de PDF a la utilizada en Chromium, lo que lleva a análisis de PDF mucho más confiables en toda la junta.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar delete en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permitir tabulación del contenido del documento a tu lista de documentos abiertos.
* Se corrigieron algunos errores donde los pulsaciones de encabezado a veces abrían documentos recientes si tenías suficientes de ellos.
* Paperback ahora eliminará guiones suaves innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados a veces colocándote en el carácter equivocado.

### Versión 0.2.0
* ¡Se agregó soporte para documentos markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones para navegar por encabezados en contenido HTML, incluyendo libros epub y documentos markdown. Estas pulsaciones fueron diseñadas para funcionar similar a un lector de pantalla.
* Se corrigió cargar epubs con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió cargar libros epub 3 con XHTML incrustado dentro de ellos.
* Ahora se pronuncia un mensaje si el documento no soporta una tabla de contenidos o secciones, en lugar de que los elementos del menú se deshabiliten.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena los últimos 10 documentos que abriste, y presionar enter en uno lo abrirá para lectura.
* ¡Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras se agrega un historial de tus últimas 25 búsquedas y soporte de expresión regular!
* Los documentos abiertos anteriormente ahora se recuerdan entre reinicios de aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú de herramientas.
* Se agregó mayús+f1 para abrir el archivo readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
