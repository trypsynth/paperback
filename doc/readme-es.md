<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,55bac79e,a548b5d0,71df8e94,e9860ee8,c7735cbe); please review and edit as needed -->

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

## Tipos de archivo actualmente compatibles

Paperback admite los siguientes formatos y extensiones:

* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Audiolibros M4B (`.m4b`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto sin formato y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para usarse principalmente con el teclado. Aquí están los atajos actuales.

Los atajos siguientes son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque Ctrl+G, Ctrl+W y Alt+Left/Right ya están ocupados por otras convenciones del sistema o de la aplicación en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abre un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cierra el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cierra todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabre el último documento cerrado.
* `Ctrl+R`: Muestra el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS está en el menú de la aplicación).

### Menú Ir

* `Ctrl+F`: Muestra el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Busca siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Busca anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir a porcentaje.
* `Ctrl+P`: Ir a página (cuando es compatible con el documento actual).
* `=`: Anuncia tu porcentaje de lectura actual.
* `Alt+Left` (macOS: `Cmd+[`): Atrás en el historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Adelante en el historial de navegación.
* `[`: Sección anterior.
* `]`: Siguiente sección.
* `Shift+H`: Encabezado anterior.
* `H`: Siguiente encabezado.
* `Shift+1` a `Shift+6`: Encabezado anterior de nivel 1-6.
* `1` a `6`: Siguiente encabezado de nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Siguiente página.
* `Shift+B`: Marcador anterior.
* `B`: Siguiente marcador.
* `/`: Establece tu marcador temporal.
* `\`: Salta a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Siguiente nota.
* `Ctrl+B`: Salta a todos los marcadores y notas.
* `Ctrl+Alt+B`: Salta solo a marcadores.
* `Ctrl+Alt+M`: Salta solo a notas.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir, la tecla Control física en lugar de Cmd): Ve el texto de la nota en la posición actual.
* `Shift+K`: Enlace anterior.
* `K`: Siguiente enlace.
* `Shift+G`: Imagen anterior.
* `G`: Siguiente imagen.
* `Shift+F`: Figura anterior.
* `F`: Siguiente figura.
* `Shift+T`: Tabla anterior.
* `T`: Siguiente tabla.
* `Shift+S`: Separador anterior.
* `S`: Siguiente separador.
* `Shift+L`: Lista anterior.
* `L`: Siguiente lista.
* `Shift+I`: Elemento de lista anterior.
* `I`: Siguiente elemento de lista.
* `Shift+,`: Ir al inicio del contenedor actual (lista o tabla).
* `,`: Ir más allá del final del contenedor actual (lista o tabla).

### Menú Herramientas

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Muestra el recuento de palabras del documento actual.
* `Ctrl+I`: Muestra la información del documento.
* `Ctrl+T`: Muestra la tabla de contenidos.
* `F7`: Muestra la lista de elementos.
* `Ctrl+Shift+C`: Abre la carpeta contenedora.
* `Ctrl+Shift+V`: Abre el contenido actual en Vista web.
* `Ctrl+U`: Ve la fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exporta datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importa datos del documento (`.paperback`).
* `Ctrl+E`: Exporta el documento actual a texto plano.
* `Ctrl+Shift+B`: Activa/desactiva el marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añade o edita la nota del marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Activa/desactiva el ajuste de palabras.
* `Ctrl+Space`: Reproduce/pausa la narración de audio.
* `'`: Avanza la narración de audio.
* `;`: Retrocede la narración de audio.
* `Ctrl+'`: Aumenta la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuye la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Activa/desactiva pantalla completa.
* `Ctrl+,`: Abre opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Activa/desactiva el temporizador de reposo.

### Menú Ayuda

* `Ctrl+F1`: Muestra el diálogo Acerca de.
* `F1`: Ve la ayuda en tu navegador predeterminado.
* `Shift+F1`: Ve la ayuda en Paperback.
* `Ctrl+Shift+U`: Comprueba si hay actualizaciones.
* `Ctrl+D`: Abre la página de donaciones en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cierra la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activa el enlace en el cursor, o abre una vista de tabla cuando está en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abre el menú contextual.

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
* Los audiolibros ahora nombran el archivo mientras avanzas por secciones.
* Los audiolibros ahora reportan su duración real, en lugar de afirmar que cada archivo dura 24 horas.
* Cerrar la Vista Web con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de ella.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte que está actualmente cargada.
* Buscar ahora va directamente a la línea encontrada, en lugar de hacerte esperar mientras el lector de pantalla lee la ventana nuevamente cuando el enfoque vuelve al libro.
* Se corrigieron los EPUB que llevan un bloque ZIP64 extraño y se negaban a abrirse con "Encabezado de archivo local inválido".
* Se corrigieron los documentos largos que volvían al inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en la Vista Web ahora te llevan a la sección a la que apuntan, en lugar de fallar con "Archivo no encontrado".
* El anuncio automático "Documento recargado" ya no corta el lector de pantalla a mitad de oración, en su lugar espera a que termine de leer.
* La pestaña General del diálogo Configuración ahora recorre sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la línea de presentación completa del programa.
* Conteo de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y rastrear su línea de tiempo en silencio.
* Se corrigieron las comillas tipográficas, guiones largos y caracteres similares desapareciendo de documentos RTF, fusionando las palabras circundantes al hacerlo.
* Se corrigieron las imágenes RTF que perdían sus datos sin procesar en el documento como texto distorsionado.
* Se corrigió el submenú Documentos recientes manteniendo entradas antiguas hasta que algo más las reconstruyera.
* Los aceleradores de teclado están de vuelta en todas las traducciones, así que los menús de Ruso tienen acceso de teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y la lista reciente del menú Inicio.
* Opciones ha sido renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, tamaño y estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora están traducidas, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora pueden traducirse.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos pueden distinguirse en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados de Paperback a HTML, Markdown o texto plano.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor reporta cualquier rareza encontrada con esto.

##### Soporte de plataforma
* ¡Soporte para Windows ARM64!
* ¡Soporte nativo para macOS!
* Un botón de pantalla completa.

##### Diálogo Todos los documentos
* Un botón para localizar documentos faltantes que acaban de cambiar de ruta.
* Un filtro de estado y barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y están seleccionados.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y tecla de acceso rápido correspondiente.
* Un botón para determinar cómo deseas que se muestren las tablas, y unificar cómo se muestran las tablas en documentos.

##### Navegación
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo exploración en lectores de pantalla.
* El atajo de teclado de igual para anunciar tu porcentaje actual a través de un documento.

##### Marcapáginas
* Marcapáginas temporales: puedes tener uno por documento, y persisten. Usa barra diagonal para establecer uno y barra diagonal inversa para saltar a él.

##### Conteo de palabras
* Tiempo de lectura estimado en el diálogo de conteo de palabras, así como la capacidad de establecer tu velocidad de lectura para hacer esta métrica realmente útil.
* Si hay una selección activa cuando abres el diálogo de conteo de palabras, se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportación
* Se expandió el elemento del menú de exportación para permitir exportar a HTML y Markdown, además de texto plano.

##### Actualizador
* Un botón de cancelación al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido alterado.

##### Vista Web
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente soportando tanto DAISY de audio (incluyendo DAISY de audio + texto) como archivos zip de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, avanzar y retroceder, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda más allá del final de un capítulo continúa al siguiente.

##### Documentos CHM
* Soporte para listas, elementos de listas, figuras e imágenes.

##### PowerPoint
* Los documentos PowerPoint ahora soportan tablas.

#### Correcciones

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Reabrir el último cerrado" intentando reabrir el léame incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* La gestión de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente al restaurar documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el léame ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta DPI.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, cuando se abre la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se lee al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes reduciendo a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Escape no cerraba los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualizaba después de cerrar un documento del diálogo de todos los documentos.
* Readme.html ya no se agregará a tu lista de todos los documentos cuando se abre vía Shift+F1.
* Eliminar documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de eliminar un documento.

##### Navegación
* La navegación de páginas anunciaba texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocaban tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetaban la ventana del documento cargado en documentos grandes.

##### Marcapáginas
* Los sonidos de marcapáginas/notas ahora deberían reproducirse correctamente de forma exclusiva cuando navegas sobre una palabra que contiene uno.

##### Legibilidad
* Aplicar ajuste de línea te disparaba al inicio de tu documento.

##### Vista Web
* El diálogo de vista web no era redimensionable y aparecía con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de etiquetas de código markdown en notas de versión.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imágenes incrustadas ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes Filepos en libros Mobi dividían etiquetas HTML y ponían basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 muy mejorado.

##### Documentos Word
* Los documentos Word con nombres de estilo específicos de la configuración regional no renderizaban sus encabezados correctamente.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto plano para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcapáginas ya no bloquearán Paperback al abrirse.

### Versión 0.8.5
* Se agregó soporte de páginas a libros epub.
* Se agregó soporte para documentos de Microsoft Office cifrados. Actualmente se soportan Word heredado, Word moderno y PowerPoint moderno, con PowerPoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos de Microsoft Word heredados!
* ¡Se agregó soporte para presentaciones PowerPoint heredadas!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió ir a página estando desplazado por 1.
* Se corrigió la tecla de escape no funcionando para cerrar el diálogo de abrir como.
* Se corrigió el menú contextual del lector no apareciendo al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió el documento incorrecto siendo enfocado a veces al abrir documentos desde la línea de comandos.
* Los PDF de solo imagen se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de aplicación.
* Se eliminó el soporte DAISY XML, ya que ya no es necesario.
* Se cambió de vuelta a la navegación de primera letra Win32 nativa en el árbol de tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suave.

### Versión 0.8.2
* ¡Se agregó soporte de páginas a documentos RTF!
* Se corrigió un error donde abrir la vista web en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no ponía un espacio entre palabras en casos raros.
* Se corrigieron párrafos divididos en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las tabulaciones y saltos de línea RTF ahora se renderizan exactamente como aparecen en el documento.
* Se cambió de vuelta a la biblioteca pdfium probada y verdadera para analizar PDF, haciendo que el renderizado de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora soporta seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron las rutas de archivos que contienen caracteres no ASCII (como š, č, ć, ž de bosnio) corrompiéndose al abrir un archivo mediante una segunda instancia de Paperback.
* Se corrigió el texto PDF siendo leído en el orden incorrecto, y espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional al alcanzar un marcapáginas o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó soporte de documentos RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de Texto de documento abierto plano!
* ¡Se agregó soporte para presentaciones de documentos abiertos planos!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento mayor que 300 caracteres ahora agregará automáticamente a tu historial de navegación.
* Se corrigió restaurar la ventana de Paperback desde la bandeja del sistema.
* Se corrigieron documentos Markdown mostrando texto sin procesar en lugar de HTML renderizado en la Vista Web.
* Se corrigieron tablas no renderizándose correctamente en archivos Markdown.
* Los PDF de solo imagen ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrusó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar el uso y la navegación.
* Se cambió a Hayro para analizar PDF, lo que lleva a más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tablas para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Enter para ver una en una vista web.
* ¡Se agregó una característica básica de renderizado web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o muestras de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Limpiar todo al diálogo Todos los documentos.
* El verificador de actualización ahora muestra notas de versión cuando una nueva versión está disponible.
* Se corrigió restaurar la ventana desde la bandeja del sistema.
* Se corrigieron traducciones de botones Sí/No en diálogos de confirmación.
* Se corrigió cargar configuraciones al ejecutarse como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió navegar al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió el diálogo de búsqueda no ocultándose correctamente al usar los botones siguiente/anterior.
* Se corrigieron TOC de epub ocasionalmente lanzándote al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en etiquetas XML, HTML y pre.
* Se corrigió error de desplazamiento por uno en navegación de enlaces.
* Se corrigieron algunos libros teniendo espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos del menú relacionados con marcapáginas así como la lista de elementos ahora están correctamente deshabilitados cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para un rendimiento y mantenibilidad mejorados.

### Versión 0.6.1
* ¡Se agregó soporte de PDF protegido con contraseña!
* Se agregó una característica muy básica de ir a posición anterior/siguiente. Si presionas enter en un enlace interno y mueve tu cursor, esa posición ahora se recordará y se puede navegar con flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado de forma predeterminada.
* Se corrigieron enlaces en algunos documentos Epub no funcionando correctamente.
* Se corrigió el análisis de TOC de Epub que contienen rutas relativas.
* Se corrigieron algunos documentos epub que no mostraban título o autor.
* Se corrigieron los títulos de algunos capítulos de epub no apareciendo correctamente en el diálogo TOC.
* Se corrigió no poder usar la barra espaciadora para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora obtendrás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú ir en una forma mucho más compacta al diálogo de opciones, marcada de forma predeterminada.
* Se agregó una opción para que la navegación por elementos estructurales se envuelva.
* Se agregó una opción al menú herramientas para abrir la carpeta contenedora del documento actualmente enfocado.
* ¡Se agregó un sistema de actualización bastante simple pero muy efectivo!
* ¡Se agregó una característica de temporizador de sueño básico, accesible con Ctrl+Shift+S!
* ¡Se agregó soporte para analizar libros electrónicos FB2!
* ¡Se agregó soporte para analizar presentaciones de documentos abiertos!
* ¡Se agregó soporte para analizar archivos de Texto de documento abierto!
* Los marcapáginas ahora pueden hacerse para marcar una línea completa, o para marcar solo algún texto especificado. Si no tienes selección activa cuando colocas un marcapáginas, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas algo de texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas a ellos! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con todos los marcapáginas, solo notas, o solo sin notas seleccionados con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un prefijo molesto "marcapáginas x".
* Los libros Epub que contienen contenido HTML fingiendo ser XML ahora serán manejados correctamente.
* Se corrigió cargar documentos Markdown grandes.
* Se corrigió presionar espacio en la vista de árbol de tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al principio de etiquetas pre en documentos HTML y XHTML.
* Se corrigió el control de texto no recuperando el enfoque a veces al volver a la ventana de Paperback.
* Se corrigió el campo de texto en el diálogo ir a porcentaje no actualizando el valor del control deslizante.
* Se corrigió el renderizado de IDs HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargador de un libro con un parámetro de línea de comandos mientras hay una instancia de Paperback existente en ejecución, ya no obtendrás un error si cargar tu documento toma más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde el diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra tal archivo en el mismo directorio que un archivo mientras se carga, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente soportados! Usa k y shift+k para moverte hacia adelante y hacia atrás a través de ellos, y presiona enter para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizar.
* ¡La navegación por listas y sus elementos ahora es totalmente soportada! Usa L y Shift+L para ir por las listas mismas, e I y Shift+I para ir a través de elementos de lista.
* La eliminación de teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además de la eliminación normal.
* ¡Paperback ahora puede minimizarse opcionalmente a tu bandeja del sistema! Esta opción está deshabilitada de forma predeterminada, pero activarla hará que la opción minimizar en el menú del sistema ponga a Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que soporta es actualmente bastante pequeña, pero crece constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el léame en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha ampliado dramáticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, con el resto de los documentos que has abierto siendo accesibles a través de un pequeño diálogo.
* Varias mejoras pequeñas en los analizadores en todos lados, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de saltos de línea dentro de párrafos en documentos de word, y agregar viñetas a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte para documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones PowerPoint!
* Se corrigieron ciertos elementos del menú no siendo deshabilitados sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió el espaciado en blanco siendo eliminado de encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora soportan la característica de tabla de contenidos! Cuando cargues un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos de la estructura de los encabezados en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar voz. Esto significa que no se envían DLL de lector de pantalla junto al programa, y se soportarán más lectores de pantalla, como Microsoft Narrator.
* Se cambió de biblioteca zip para permitir abrir una matriz más amplia de libros epub.
* El diálogo pidiéndote si deseas abrir tu documento como texto plano ha sido completamente rehecho, y ahora te permite abrir tu documento como texto plano, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje al que saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub será nuevamente preservada exactamente.
* El espacio no rompedor unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te preguntará cómo deseas abrir un archivo no reconocido cada sola vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú Inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo si tienes un elemento padre e hijo con el mismo texto en la misma posición ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas.
* Los documentos CHM ahora deberían mostrar su título como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcapáginas! Puedes tener tantos marcapáginas como quieras a lo largo de tantos documentos como quieras. Puedes saltar hacia adelante y hacia atrás con b y shift+b, establecer uno con control+shift+b, y abrir un diálogo para saltar a un marcapáginas específico con control+b.
* ¡Se agregó un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente asociaciones de archivos.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora es manejado por su propio diálogo basado en control deslizante, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado cuando cerraste Paperback ahora se recuerda entre reinicios de aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería ser desinfectada más estrictamente.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron elementos TOC anidados en libros Epub poniendo tu cursor en la posición incorrecta.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú ayuda o a través del enlace patrocina este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si los metadatos faltan.
* Se cambió de biblioteca PDF a la utilizada en Chromium, lo que lleva a un análisis de PDF mucho más confiable en todos lados.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar eliminar en un documento en el control de pestañas para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permitir tabulación del contenido del documento a tu lista de documentos abiertos.
* Se corrigieron algunos errores con los atajos de encabezados a veces abriendo documentos recientes si tenías suficientes.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados a veces poniéndote en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de teclas para navegar por encabezados en contenido HTML, incluyendo libros epub y documentos markdown. Estas pulsaciones de teclas fueron diseñadas para funcionar similar a un lector de pantalla.
* Se corrigió la carga de epub con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro.
* Ahora se habla un mensaje si el documento no soporta tabla de contenidos o secciones, en lugar de que los elementos del menú estén deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar enter en uno lo abrirá para leer.
* ¡Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, al tiempo que también agregó un historial de tus últimas 25 búsquedas y soporte de expresiones regulares!
* Los documentos abiertos anteriormente ahora se recuerdan entre reinicios de aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú herramientas.
* Se agregó shift+f1 para abrir el léame directamente en Paperback mismo.

### Versión 0.1.0
* Versión inicial.
