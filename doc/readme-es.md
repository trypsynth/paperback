<!-- machine-translated from doc/readme.md (source-hash: df18cffffe239932); please review and edit as needed -->

# Paperback - versión 0.9.1

## Introducción

Paperback es un lector de libros electrónicos y documentos ligero, rápido y accesible para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para accesibilidad con lectores de pantalla, velocidad rápida y una experiencia sin bloatware.

## Requisitos del sistema

Paperback actualmente se ejecuta en Windows 10/11 y todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba pública previstas poco después del lanzamiento de la versión 0.9.0 para escritorio, antes de un lanzamiento unificado de 1.0 que cubra las cuatro plataformas.

## Características

* Completamente independiente, sin necesidad de instalar ningún software en tu computadora para comenzar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz simple con pestañas, que te permite abrir tantos documentos como desees lado a lado.
* Guarda tu posición exacta de lectura en todos los documentos que abras.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa, y los restaura al siguiente lanzamiento.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de documentos.
* Incluye un robusto diálogo de búsqueda, con características como historial y compatibilidad con expresiones regulares.
* Puede ejecutarse de forma completamente portátil, o instalarse con asociaciones de archivos configuradas automáticamente.
* Admite una amplia variedad de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, existe un problema conocido para usuarios de JAWS.

### JAWS y pantallas Braille

Si usas JAWS con una pantalla Braille, es posible que encuentres que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de tu pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en la gestión de JAWS del control de texto RICHEDIT50W, no algo en Paperback mismo, y uno que tardó bastante tiempo en surgir una solución dada la disposición de Vispero para responder a problemas con software de código abierto.

La solución, finalmente descubierta a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente admitidos

Paperback admite los siguientes formatos y extensiones:

* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos de Microsoft Word (`.docx`, `.docm`, `.doc`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto plano y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para el uso prioritario del teclado. Aquí están los atajos actuales.

Los atajos que se muestran a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left`/`Right` ya están reclamados por otras convenciones del sistema u otras aplicaciones en esa plataforma.

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
* `Ctrl+P`: Ir a página (cuando es compatible con el documento actual).
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

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Mostrar el recuento de palabras del documento actual.
* `Ctrl+I`: Mostrar información del documento.
* `Ctrl+T`: Mostrar tabla de contenidos.
* `F7`: Mostrar lista de elementos.
* `Ctrl+Shift+C`: Abrir carpeta contenedora.
* `Ctrl+Shift+V`: Abrir contenido actual en Web View.
* `Ctrl+U`: Ver la fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto plano.
* `Ctrl+Shift+B`: Alternar marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añadir o editar nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar ajuste de palabras.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Avanzar narración de audio.
* `;`: Retroceder narración de audio.
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
* `Ctrl+D`: Abrir la página de donaciones en tu navegador predeterminado.

### Teclas adicionales de vista de documentos

* `Delete` / `Numpad Delete` en el control de pestañas: Cerrar la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activar enlace en cursor, o abrir una vista de tabla cuando estés en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas compatibles

Paperback está traducido a muchos idiomas diferentes, y se están añadiendo más constantemente. A continuación se muestra una lista completa.

Para saber cómo contribuir, lee nuestra [Guía de traducción](translating.md).

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
* Aryan Choudhary: principal colaborador.

### Donaciones
Las siguientes personas han hecho donaciones de algún tipo al desarrollo de Paperback. Si haces una donación, tu nombre no se añadirá automáticamente aquí, solo añado a las personas que desean que su donación sea pública.

Nota: considero que ser patrocinador público en GitHub es motivo de inclusión automática en esta lista.

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
* Los audiolibros ahora informan de su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar Web View con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directamente a la línea encontrada, en lugar de hacer que el lector de pantalla vuelva a leer la ventana mientras el enfoque regresa al libro.
* Se corrigieron los EPUB que llevan un bloque ZIP64 extraviado y se niegan a abrirse con "Encabezado de archivo local no válido".
* Se corrigieron los documentos largos que volvían al inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en Web View ahora te llevan a la sección a la que apuntan, en lugar de fallar con "Archivo no encontrado".
* El anuncio automático "Documento recargado" ya no corta tu lector de pantalla a mitad de frase, sino que espera a que termine de hablar.
* La pestaña General del diálogo Configuración ahora recorre sus opciones en el orden en que aparecen en la pantalla, con el canal de actualización directamente después de la opción de verificación de actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar del eslogan completo del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcadores y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y rastrear su cronología en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares que desaparecían de los documentos RTF, uniendo las palabras circundantes.
* Se corrigió la fuga de imágenes RTF de sus datos sin procesar en el documento como texto desordenado.
* Se corrigió el submenú Documentos recientes que mantenía entradas obsoletas hasta que algo más las reconstruyera.
* Los aceleradores de teclado están de vuelta en todas las traducciones, por lo que los menús de ruso tienen acceso nuevamente a través de teclado.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* "Opciones" ha sido renombrado a "Configuración", coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, el tamaño y el estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el archivo ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones en el diálogo Personalizar atajos de teclado ahora pueden traducirse.
* El título del documento ahora aparece primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados por Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, reporta cualquier rareza encontrada con esto.

##### Compatibilidad de plataforma
* ¡Soporte para Windows ARM64!
* ¡Soporte nativo para macOS!
* Un conmutador de pantalla completa.

##### Diálogo Todos los documentos
* Un botón de ubicación para localizar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver más abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafo;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y una tecla de acceso rápido posterior.
* Un conmutador para determinar cómo deseas que se muestren las tablas, y unificó cómo se muestran las tablas en todos los documentos.

##### Navegación
* Soporte para navegación por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo exploración en lectores de pantalla.
* El atajo de teclado igual para anunciar tu porcentaje actual en un documento.

##### Marcadores
* Marcadores temporales: puedes tener uno por documento, y persisten. Usa la barra inclinada para establecer uno y la barra invertida para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer tu velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si hay una selección activa cuando abres el diálogo de recuento de palabras, ahora se mostrará cuántas palabras tienes seleccionadas.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportación
* Se amplió el elemento de menú de exportación para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón de cancelación al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido alterado.

##### Web View
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con DAISY audio (incluido DAISY audio + texto) y zips de archivos de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar la narración, buscar hacia adelante y atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora admiten tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Reabrir último cerrado" intentando reabrir el archivo readme incluido.
* Tu pestaña seleccionada no enfocándose correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no serán cargados forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta DPI.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir la ayuda en Paperback.
* Cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Escape no cerrando los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualiza después de cerrar un documento desde el diálogo de todos los documentos.
* El archivo Readme.html ya no se agregará a tu lista de todos los documentos cuando se abre a través de Shift+F1.
* Eliminar documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* La navegación por página anunciando texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetando la ventana del documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota deberían reproducirse correctamente solo cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de línea te dispara al inicio de tu documento.

##### Web View
* El diálogo de vista web no es redimensionable y aparece con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de la versión.

##### Libros DAISY
* Los libros DAISY muestran información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* RTF `\pict` grupos para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Las anclas de posición de archivo en libros Mobi dividen etiquetas HTML y colocan basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 muy mejorado.

##### Documentos de Word
* Los documentos de Word con nombres de estilo específicos de la configuración regional no renderizaban correctamente sus encabezados.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producen saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no bloquearán Paperback al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office encriptados. Actualmente se admiten documentos Word heredados, Word moderno y PowerPoint moderno, con PowerPoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos de Microsoft Word heredados (*.doc)!
* ¡Se agregó soporte para presentaciones de PowerPoint heredadas (*.ppt)!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (DAISY y Word)!
* El texto alternativo para imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora admiten correctamente la navegación de enlaces internos.
* Se corrigieron los sonidos de marcadores que se disparan al inicio del párrafo en lugar de la posición del marcador.
* Se corrigió ir a página estando desviado por 1.
* Se corrigió la tecla Escape sin funcionar para cerrar el diálogo abrir como.
* Se corrigió el menú contextual del lector sin aparecer al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió el enfoque del documento incorrecto a veces al abrir documentos desde la línea de comandos.
* Los PDF solo de imágenes se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar por imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de aplicación.
* Se eliminó el soporte de DAISY XML, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primer nivel nativa de Win32 en el árbol de tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápida y sin problemas.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no pondría un espacio entre palabras en casos raros.
* Se corrigieron párrafos divididos en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las pestañas RTF y los saltos de línea ahora se representan exactamente como aparecen en el documento.
* Se volvió a cambiar a la biblioteca pdfium probada y verdadera para analizar PDF, haciendo que la renderización de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora admite la selección de múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron las rutas de archivo que contienen caracteres no ASCII (como š, č, ć, ž bosnios) que se corrompían al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió el texto PDF siendo leído en el orden incorrecto y espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de simplemente descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional para alcanzar un marcador o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó soporte de documento RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de Documento de texto abierto plano!
* ¡Se agregó soporte para presentaciones de Documento abierto plano!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento mayor de 300 caracteres ahora agregará automáticamente a tu historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió la visualización de documentos Markdown de texto sin formato en lugar de HTML renderizado en la vista web.
* Se corrigieron las tablas sin renderizar correctamente en archivos Markdown.
* Los PDF solo de imágenes ahora te advertirán de su existencia cuando intentes cargar uno.
* Ahora es posible verificar nuevas compilaciones de desarrollo en lugar de versiones estables al verificar actualizaciones.
* Se embebe correctamente la información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar su uso y navegación.
* Se cambió a Hayro para analizar PDF, lo que aumenta la confiabilidad, la velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tablas para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Intro para ver una en una vista web.
* ¡Se agregó una función básica de representación web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Borrar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de la versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigió la traducción de botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones al ejecutarse como administrador.
* Se corrigió el manejo de comentarios en documentos XML e HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió el diálogo de búsqueda sin ocultarse correctamente al usar los botones siguiente/anterior.
* Se corrigieron los TOC de epub ocasionalmente te llevan al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió el error de apagado en la navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos del menú relacionados con marcadores, así como la lista de elementos, ahora se desactivan correctamente cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica comercial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte de PDF protegido con contraseña!
* Se agregó una característica muy básica de ir a posición anterior/siguiente. Si presionas Intro en un enlace interno y mueve tu cursor, esa posición ahora se recordará y podrá navegarse con las teclas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirla en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado de forma predeterminada.
* Se corrigieron los enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de TOC de Epub que contenían rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigieron los títulos de algunos capítulos de epub que no aparecían correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones Aceptar/cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú ir en una forma mucho más compacta al diálogo de opciones, marcada por defecto.
* Se agregó una opción para envolver la navegación por elementos estructurales.
* Se agregó una opción al menú herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* ¡Se agregó un sistema de actualización bastante simple, pero muy efectivo!
* Se agregó una característica de temporizador de sueño básica, accesible con Ctrl+Shift+S.
* ¡Se agregó soporte para analizar libros electrónicos FB2!
* ¡Se agregó soporte para analizar presentaciones de OpenDocument!
* ¡Se agregó soporte para analizar archivos de OpenDocument Text!
* Los marcadores ahora se pueden hacer para marcar una línea completa o solo marcar texto especificado. Si no tienes una selección activa cuando colocas un marcador, el comportamiento es como anterior a 0.6, y marcará la línea completa. Sin embargo, si seleccionas algo de texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas! Navega entre marcadores que contienen notas con N y Shift+N, o abre el diálogo de marcadores con todos los marcadores, solo notas o solo no notas seleccionadas con teclas de acceso rápido específicas.
* Los marcadores en el diálogo de marcadores ya no tendrán un molesto prefijo "marcador x".
* Los libros Epub que contienen contenido HTML fingiendo ser XML ahora se manejarán correctamente.
* Se corrigió la carga de documentos Markdown grandes.
* Se corrigió al presionar espacio en el árbol de vista de tabla de contenidos activando el botón Aceptar.
* Se corrigió el manejo de espacios en blanco al principio de etiquetas pre en documentos HTML y XHTML.
* Se corrigió que el campo de texto no recupere el enfoque a veces cuando se regresa a la ventana de Paperback.
* Se corrigió que el campo de texto en el diálogo ir a porcentaje no actualice el valor del control deslizante.
* Se corrigió la renderización de ID HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras hay una instancia de Paperback existente ejecutándose, ya no obtendrás un error si cargar tu documento toma más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde el diálogo de marcadores.
* Ahora es posible importar y exportar tus marcadores y posición de lectura para un documento en particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra un archivo de este tipo en el mismo directorio que un archivo al cargarlo, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú herramientas.
* ¡Los enlaces dentro de documentos ahora son completamente compatibles! Usa k y shift+k para moverte hacia adelante y atrás a través de ellos, y presiona Intro para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido Markdown ahora se procesa previamente para ser conforme con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es completamente compatible! Usa L y Shift+L para ir por las listas mismas, e I y Shift+I para ir a través de elementos de lista.
* Ahora Suprimir en el teclado numérico funciona para eliminar documentos de la barra de pestañas además de Suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está desactivada de forma predeterminada, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es completamente traducible! La lista de idiomas que admite es actualmente bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se muestra en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes ha sido ampliada dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora mostrará una cantidad personalizable, siendo el resto de documentos que haya abierto accesibles a través de un pequeño diálogo.
* Varias mejoras pequeñas en los analizadores en todos los ámbitos, incluida la colocación de una línea en blanco entre diapositivas en presentaciones PPTX, la corrección del manejo de nuevas líneas dentro de párrafos en documentos de Word, y la adición de viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte de documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú no desactivados sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o ID de fragmento.
* Se corrigió que los espacios en blanco se eliminen de los encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenidos! Cuando cargues un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los encabezados en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título tal como se establece en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre del archivo sin la extensión.
* Se cambió de UniversalSpeech al uso de una región activa para reportar voz. Esto significa que no hay más DLL de lector de pantalla enviados junto con el programa, y más lectores de pantalla ahora serán compatibles, como Microsoft Narrator.
* Se cambió la biblioteca zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que te pregunta si deseas abrir tu documento como texto sin formato ha sido completamente rehecho, y ahora permite abrir tu documento como texto sin formato, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite introducir manualmente un porcentaje al que saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se preservará exactamente una vez más.
* El espacio que no se divide de Unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te preguntará cómo deseas abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono opcional del menú Inicio al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo, si tienes un elemento secundario y padre con el mismo texto en la misma posición, ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título tal como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte de archivo CHM!
* ¡Se agregó soporte de marcadores! Puedes tener tantos marcadores como desees en tantos documentos como desees. Puedes saltar hacia adelante y atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y abrir un diálogo para saltar a un marcador específico con control+b.
* ¡Se agregó un instalador junto con el archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente las asociaciones de archivo para ti.
* Los archivos de texto con BOM ahora se decodificarán correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja por su propio diálogo basado en control deslizante, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán uno predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que enfocaste cuando cerraste Paperback ahora se recuerda entre reinicios de aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería desinfectarse más estrictamente.
* Se corrigió la navegación de la tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación por encabezado en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron los elementos TOC anidados en libros Epub que colocaban tu cursor en la posición incorrecta.
* Se corrigió un bloqueo al cerrar la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace patrocina este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió la biblioteca PDF a la utilizada en Chromium, lo que lleva a un análisis PDF mucho más confiable en todos los ámbitos.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Al ejecutar paperback.exe con un nombre de archivo mientras ya está ejecutándose, abrirá ese documento en la instancia ya ejecutándose.
* Ahora puedes presionar Suprimir en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permitir tabulación del contenido del documento a tu lista de documentos abiertos.
* Se corrigieron los atajos de encabezado a veces abriendo documentos recientes si tenías suficientes.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió la navegación por encabezado a veces poniéndote en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte de documento Markdown!
* ¡Se agregó soporte de documento PDF, incluida la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de tecla para navegar por encabezados en contenido HTML, incluidos libros epub y documentos Markdown. Estas pulsaciones de tecla fueron diseñadas para funcionar similar a un lector de pantalla.
* Se corrigió la carga de epub con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro de ellos.
* Ahora se habla un mensaje si el documento no admite tabla de contenidos o secciones, en lugar de deshabilitar los elementos del menú.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar Intro en uno lo abrirá para leer.
* ¡Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras también se agregó un historial de tus últimas 25 búsquedas y soporte de expresiones regulares!
* Los documentos abiertos anteriormente ahora se recuerdan entre reinicios de aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú de herramientas.
* Se agregó shift+f1 para abrir el archivo readme directamente en Paperback mismo.

### Versión 0.1.0
* Lanzamiento inicial.
