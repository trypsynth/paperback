<!-- machine-translated from doc/readme.md (source-hash: d583a89d8ac391f5; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,af36028a,71df8e94,e9860ee8,93dd8dd6); please review and edit as needed -->

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

Paperback está diseñado para uso con teclado en primer lugar. Estos son los atajos actuales.

Los atajos que se indican a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left`/`Right` ya están utilizados por otras convenciones del sistema u otras aplicaciones en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabrir el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS esto está en el menú de la aplicación).

### Menú Ir

* `Ctrl+F`: Mostrar el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Buscar siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir a porcentaje.
* `Ctrl+P`: Ir a página (cuando lo admita el documento actual).
* `=`: Anunciar tu porcentaje de lectura actual y página, p. ej. "15%, página 30". La página se omite para documentos que no tienen números de página.
* `Alt+Left` (macOS: `Cmd+[`): Ir atrás en el historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Ir adelante en el historial de navegación.
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
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir, la tecla Control física en lugar de Cmd): Ver el texto de la nota en la posición actual.
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

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Mostrar el recuento de palabras del documento actual.
* `Ctrl+I`: Mostrar información del documento.
* `Ctrl+T`: Mostrar tabla de contenidos.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir carpeta contenedora.
* `Ctrl+Shift+V`: Abrir contenido actual en Vista web.
* `Ctrl+U`: Ver el código fuente del documento en una pestaña nueva.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto sin formato.
* `Ctrl+Shift+B`: Alternar marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añadir o editar nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar ajuste de palabras.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Buscar narración de audio hacia adelante.
* `;`: Buscar narración de audio hacia atrás.
* `Ctrl+'`: Aumentar la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuir la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Alternar temporizador de sueño.

### Menú Ayuda

* `Ctrl+F1`: Mostrar diálogo Acerca de.
* `F1`: Ver ayuda en tu navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Buscar actualizaciones.
* `Ctrl+D`: Abrir la página de donación en tu navegador predeterminado.

### Teclas adicionales de visualización de documentos

* `Delete` / `Numpad Delete` en el control de pestaña: Cerrar la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activar enlace en el cursor, o abrir una vista de tabla cuando está en un marcador de tabla.
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
* Los audiolibros ya no hacen que tu lector de pantalla lea una serie de espacios cuando enfoques el campo de texto.
* Los audiolibros ahora nombran el archivo mientras lo avanzas por sección.
* Los audiolibros ahora informan de su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar Web View con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de él.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte actualmente cargada.
* Buscar ahora va directo a la línea encontrada, en lugar de hacer que el lector de pantalla vuelva a leer la ventana completa mientras el foco regresa al libro.
* Se corrigieron los EPUB que llevaban un bloque ZIP64 extraño rechazando abrir con "encabezado de archivo local inválido".
* Se corrigió que documentos largos volvieran al inicio mientras un lector de pantalla leía continuamente a través de ellos.
* Los enlaces en la WebView ahora te llevan a la sección a la que apuntan, en lugar de fallar con "archivo no encontrado".
* El atajo `=` ahora anuncia la página además del porcentaje, por ejemplo "15%, página 30", y se mantiene como estaba para documentos sin números de página.
* El anuncio automático "Documento recargado" ya no interrumpe tu lector de pantalla a mitad de oración, sino que espera a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora tabulea a través de sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción verificar actualizaciones.
* La actualización ahora trae la ventana relanzada al frente, en lugar de dejarla detrás de todas las demás ventanas en Alt+Tab.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la etiqueta completa del programa.
* Word Count e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y rastrear su línea de tiempo en silencio.
* Se corrigió que las comillas rizadas, guiones largos y caracteres similares desaparecieran de documentos RTF, uniendo las palabras circundantes mientras se iban.
* Se corrigió que las imágenes RTF filtraran sus datos sin procesar en el documento como texto corrupto.
* Se corrigió que el submenú Documentos recientes mantuviera entradas obsoletas hasta que algo más ocurriera para reconstruirlo.
* Los aceleradores de teclado están de vuelta en cada traducción, así que los menús de Ruso nuevamente tienen acceso por teclado.
* Los grandes documentos CHM ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran en Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Opciones ha sido renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, tamaño y estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora pueden ser traducidos.
* El título del documento ahora viene primero en la barra de título, para que los libros abiertos se puedan distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Añadido

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados por Paperback a HTML, Markdown o texto plano.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil por ejemplo para editar Markdown.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor reporta cualquier rareza encontrada con esto.

##### Compatibilidad de plataforma
* ¡Soporte de Windows ARM64!
* ¡Soporte nativo de macOS!
* Un botón de pantalla completa.

##### Diálogo Todos los documentos
* Un botón ubicar para localizar libros perdidos que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido de general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafo;
    * Espaciado de letras;
    * Alineación del texto.
* Un elemento de menú de ajuste de línea y su tecla de acceso rápido correspondiente.
* Un botón para determinar cómo deseas que se muestren las tablas, y unificó cómo se muestran las tablas en todos los documentos.

##### Navegación
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar a modo examinar en lectores de pantalla.
* El atajo de teclado igual para anunciar tu porcentaje actual en un documento.

##### Marcapáginas
* Marcapáginas temporales: puedes tener uno por documento, y persisten. Usa barra inclinada para establecer uno e invertida para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer tu velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si hay una selección activa al abrir el diálogo de recuento de palabras, se mostrará cuántas palabras tienes seleccionadas.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento de menú exportar para permitir exportar a HTML y Markdown, además de texto plano.

##### Actualizador
* Un botón cancelar al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no ha sido alterado.

##### Web View
* La webview ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con tanto audio DAISY (incluyendo DAISY audio + texto) como archivos zip de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, buscar adelante y atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio, y elegir si buscar más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos PowerPoint ahora soportan tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Reabrir el último cerrado" intentando reabrir el readme incluido.
* Tu pestaña seleccionada no se enfoca correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no serán cargados forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alto DPI.
* El menú ahora se actualiza correctamente, y el foco se mueve al control de texto, al abrir ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes a la mitad del tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Escape no cerraba los diálogos Información del documento y Todos los documentos.
* La barra de título no se actualizaba después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se agregará a tu lista de todos los documentos al abrirse a través de Shift+F1.
* La eliminación de documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de eliminar un documento.

##### Navegación
* La navegación de página anunciando texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetaban la ventana de documento cargada en documentos grandes.

##### Marcapáginas
* Los sonidos de marcapáginas/notas ahora deberían reproducirse exclusivamente cuando navegues sobre una palabra que contiene uno.

##### Legibilidad
* Aplicar ajuste de línea te disparaba al inicio de tu documento.

##### Web View
* El diálogo webview no era redimensionable y aparecía en un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la webview incrustada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de la versión.

##### Libros DAISY
* Los libros DAISY mostrando información incorrecta en la barra de estado.
* Carga de libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes filepos en libros Mobi dividían etiquetas HTML e insertar basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 mayormente mejorado.

##### Documentos Word
* Los documentos Word con nombres de estilo específicos del idioma no renderizaban correctamente sus títulos.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producen saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora vuelve a la extracción de texto plano para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcapáginas ya no causarán que Paperback se bloquee al abrir.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos Microsoft Office encriptados. Actualmente se soportan Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos Microsoft Word heredados!
* ¡Se agregó soporte para presentaciones Powerpoint heredadas!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió que ir a página fuera incorrecta por 1.
* Se corrigió que la tecla escape no funcionara para cerrar el diálogo abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla de aplicaciones.
* Se corrigió que a veces el documento incorrecto recibiera foco al abrir documentos desde la línea de comandos.
* Los PDF solo de imagen ahora se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte de DAISY XML, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primera letra nativa de Win32 en el árbol de tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La webview ahora se abrirá mucho más rápido y suavemente.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la webview en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no pondría un espacio entre palabras en casos raros.
* Se corrigieron párrafos siendo divididos en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y títulos!
* Las tabulaciones y saltos de línea RTF ahora se representan exactamente como aparecen en el documento.
* Se volvió a cambiar a la confiable biblioteca pdfium para análisis de PDF, haciendo que la representación de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora soporta seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigió que las rutas de archivo que contienen caracteres no ASCII (como bosnio š, č, ć, ž) se corrompieran al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió que el texto PDF se leyera en el orden incorrecto y espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió que la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional por alcanzar un marcapáginas o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó soporte para documentos RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de texto de documento abierto plano!
* ¡Se agregó soporte para presentaciones de documento abierto plano!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento mayor de 300 caracteres ahora agregará automáticamente a tu historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en la Web View.
* Se corrigió que las tablas no se renderizaran correctamente en archivos Markdown.
* Los PDF solo de imagen ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrustó correctamente la información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilidad de uso y navegación.
* Se cambió a Hayro para análisis de PDF, lo que resultó en más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido, y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tabla para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Enter para ver una en una webview.
* ¡Se agregó una característica de representación web básica! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o muestras de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Borrar todo al diálogo Todos los documentos.
* El verificador de actualización ahora muestra notas de la versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigió la traducción de botones Sí/No en diálogos de confirmación.
* Se corrigió que se cargaran configuraciones al ejecutar como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió navegar al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que los TOC de epub ocasionalmente te llevaran al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error fuera por uno en navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos de menú relacionados con marcapáginas así como la lista de elementos ahora están deshabilitados correctamente cuando no hay documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para contribuyentes.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejor rendimiento y mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte para PDF protegidos con contraseña!
* Se agregó una característica muy básica de ir a posición anterior/siguiente. Si presionas enter en un enlace interno y mueve tu cursor, esa posición ahora será recordada, y puede ser navegada con flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los títulos en tu documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigió que los enlaces en algunos documentos Epub no funcionaran correctamente.
* Se corrigió el análisis de TOC de Epub que contienen rutas relativas.
* Se corrigió que algunos documentos epub no mostraran un título o autor.
* Se corrigió que los títulos de algunos capítulos de epub no aparecieran correctamente en el diálogo TOC.
* Se corrigió que no puedas usar la barra espaciadora para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de títulos en documentos Word.
* Ahora obtendrás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentas traer el diálogo.

### Versión 0.6.0
* Se ha agregado una nueva opción para mostrar el menú ir en una forma mucho más compacta al diálogo de opciones, marcada por defecto.
* Se agregó una opción para hacer que la navegación por elementos estructurales se envuelva.
* Se agregó una opción al menú herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* Se agregó un sistema de actualización bastante simple pero muy efectivo.
* Se agregó una característica básica de temporizador de sueño, accesible con Ctrl+Shift+S.
* ¡Se agregó soporte para análisis de libros FB2!
* ¡Se agregó soporte para análisis de presentaciones de documento abierto!
* ¡Se agregó soporte para análisis de archivos de texto de documento abierto!
* Los marcapáginas ahora pueden ser marcapáginas de una línea completa, o marcar solo texto especificado. Si no tienes selección activa al colocar un marcapáginas, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas algún texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con solo marcapáginas, solo notas, o solo no notas seleccionadas con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un molesto prefijo "marcapáginas x".
* Los libros Epub que contienen contenido HTML pretendiendo ser XML ahora se manejarán correctamente.
* Se corrigió la carga de grandes documentos Markdown.
* Se corrigió presionar espacio en el árbol de vista de tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al principio de etiquetas pre en documentos HTML y XHTML.
* Se corrigió que el control de texto no recuperara foco a veces al regresar a la ventana de Paperback.
* Se corrigió que el campo de texto en el diálogo ir a porcentaje no actualice el valor del control deslizante.
* Se corrigió la representación de ID HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras una instancia existente de Paperback se está ejecutando, ya no obtendrás un error si cargar tu documento toma más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde dentro del diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra tal archivo en el mismo directorio que un archivo al cargarlo, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú herramientas.
* ¡Los enlaces dentro de documentos ahora son completamente soportados! Usa k y shift+k para moverte hacia adelante y hacia atrás a través de ellos, y presiona enter para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido Markdown ahora se preprocesan para ser compatible con CommonMark antes de renderizar.
* ¡La navegación por listas y sus elementos ahora es completamente soportada! Usa L y Shift+L para ir por las listas en sí, e I y Shift+I para ir a través de elementos de lista.
* Eliminar Numpad ahora funciona para eliminar documentos de la barra de pestañas además de eliminar normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está deshabilitada por defecto, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, capaz de ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es completamente traducible! La lista de idiomas que soporta es actualmente bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido drásticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora mostrará un número personalizable, siendo el resto de los documentos que alguna vez has abierto accesible a través de un pequeño diálogo.
* Varias pequeñas mejoras a los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de nuevas líneas dentro de párrafos en documentos Word, y agregar viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte de documentos Microsoft Word!
* ¡Se agregó soporte para presentaciones PowerPoint!
* Se corrigió que ciertos elementos de menú no estuvieran deshabilitados sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas por URL y/o ID de fragmento.
* Se corrigió que los espacios en blanco fueran eliminados de títulos XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora soportan la característica de tabla de contenidos! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los títulos en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título como se establece en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar el habla. Esto significa que no se envían DLL de lector de pantalla junto con el programa, y más lectores de pantalla serán soportados ahora, como Microsoft Narrator.
* Se cambió bibliotecas zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que te pregunta si deseas abrir tu documento como texto plano se ha reescrito completamente, y ahora te permite abrir tu documento como texto plano, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje al que ir.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub será preservada exactamente nuevamente.
* El espacio sin salto unicode ahora se considera al eliminar líneas en blanco.
* Ya no te preguntarán cómo deseas abrir un archivo no reconocido cada vez que lo cargas, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú Inicio opcional al instalador.
* La tabla de contenidos ahora debería estar más limpia en algunos casos, por ejemplo si tienes un elemento hijo y padre con el mismo texto en la misma posición ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcapáginas! Puedes tener tantos marcapáginas en tantos documentos como desees. Puedes saltar hacia adelante y hacia atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y traer un diálogo para saltar a un marcapáginas específico con control+b.
* ¡Se agregó un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, e configurará automáticamente asociaciones de archivo.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora es manejado por su propio diálogo basado en control deslizante, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda entre reinicios de aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería ser más estrictamente desinfectada.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados por URL.
* Se corrigió la navegación de título en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigió que elementos de TOC anidados en libros Epub colocaran tu cursor en la posición incorrecta.
* Se corrigió una caída en la salida de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú ayuda o a través del enlace patrocinar este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió bibliotecas de PDF a la utilizada en Chromium, lo que resulta en análisis de PDF mucho más confiable en general.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar eliminar en un documento en el control de pestañas para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Se permite tabular desde el contenido del documento a tu lista de documentos abiertos.
* Se corrigió que las pulsaciones de tecla de título a veces abrieran documentos recientes si tenías suficientes de ellos.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió que la navegación de título a veces te colocara en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos Markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de tecla para navegar por títulos en contenido HTML, incluidos libros epub y documentos Markdown. Estas pulsaciones de tecla se diseñaron para funcionar similar a un lector de pantalla.
* Se corrigió la carga de epub con nombres de archivo codificados por URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro de ellos.
* Ahora se habla un mensaje si el documento no soporta una tabla de contenidos o secciones, en lugar de que los elementos de menú estén deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar enter en uno lo abrirá para lectura.
* Se reescribió completamente el diálogo de búsqueda, haciéndolo mucho más simple de usar, mientras también se agregó un historial de tus últimas 25 búsquedas y soporte de expresiones regulares.
* Los documentos abiertos anteriormente ahora son recordados entre reinicios de aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú herramientas.
* Se agregó shift+f1 para abrir el readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
