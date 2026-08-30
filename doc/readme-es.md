<!-- machine-translated from doc/readme.md (source-hash: d49e7044d9856698); please review and edit as needed -->

# Paperback - versión 0.9.1

## Introducción

Paperback es un lector de libros electrónicos y documentos ligero, rápido y accesible para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para ser accesible con lectores de pantalla, ofrecer velocidades rápidas y una experiencia sin exceso de funciones innecesarias.

## Requisitos del sistema

Paperback se ejecuta actualmente en Windows 10/11 y todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba públicas planeadas poco después del lanzamiento de escritorio 0.9.0, antes de un lanzamiento unificado 1.0 que cubra las cuatro plataformas.

## Características

* Completamente independiente, sin requerir ningún software instalado en tu ordenador para empezar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz con pestañas simple, permitiéndote abrir tantos documentos como desees lado a lado.
* Guarda tu posición exacta de lectura en cada documento que abras.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura en el siguiente lanzamiento.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de documentos.
* Incluye un diálogo de búsqueda robusto, con características como historial y soporte de expresiones regulares.
* Puede ejecutarse completamente de forma portátil o instalarse con asociaciones de archivos configuradas automáticamente.
* Soporta una amplia gama de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, hay un problema conocido para los usuarios de JAWS.

### JAWS y pantallas Braille

Si usas JAWS con una pantalla Braille, es posible que encuentres que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de tu pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en la gestión de JAWS del control de texto RICHEDIT50W, no algo en Paperback mismo, y uno que tardó bastante tiempo en encontrar una solución dados los esfuerzos de Vispero por responder a problemas con software de código abierto.

La solución, finalmente encontrada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente soportados

Paperback soporta los siguientes formatos y extensiones:

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
* Archivos de texto sin formato y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para el uso primero por teclado. Aquí están los atajos actuales.

Los atajos a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque Ctrl+G, Ctrl+W y Alt+Left/Right ya están utilizados por otras convenciones del sistema u otras aplicaciones en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abre un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cierra el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cierra todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabre el último documento cerrado.
* `Ctrl+R`: Muestra el diálogo "Todos los documentos" (de Documentos recientes).
* `Ctrl+Q`: Sale (solo Windows; en macOS esto está en el menú de la aplicación).

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
* `]`: Sección siguiente.
* `Shift+H`: Encabezado anterior.
* `H`: Encabezado siguiente.
* `Shift+1` hasta `Shift+6`: Encabezado anterior en nivel 1-6.
* `1` hasta `6`: Encabezado siguiente en nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Página siguiente.
* `Shift+B`: Marcador anterior.
* `B`: Marcador siguiente.
* `/`: Establece tu marcador temporal.
* `\`: Salta a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Salta a todos los marcadores y notas.
* `Ctrl+Alt+B`: Salta a marcadores solamente.
* `Ctrl+Alt+M`: Salta a notas solamente.
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

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Muestra el recuento de palabras del documento actual.
* `Ctrl+I`: Muestra la información del documento.
* `Ctrl+T`: Muestra la tabla de contenidos.
* `F7`: Muestra la lista de elementos.
* `Ctrl+Shift+C`: Abre carpeta contenedora.
* `Ctrl+Shift+V`: Abre contenido actual en Vista web.
* `Ctrl+U`: Ver la fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exporta el documento actual a texto sin formato.
* `Ctrl+Shift+B`: Activa/desactiva marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añade o edita nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Activa/desactiva ajuste de palabras.
* `Ctrl+Space`: Reproduce/pausa narración de audio.
* `'`: Busca narración de audio hacia adelante.
* `;`: Busca narración de audio hacia atrás.
* `Ctrl+'`: Aumenta la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuye la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Comando+F): Activa/desactiva pantalla completa.
* `Ctrl+,`: Abre opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Activa/desactiva temporizador de reposo.

### Menú Ayuda

* `Ctrl+F1`: Muestra el diálogo Acerca de.
* `F1`: Ver ayuda en tu navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Busca actualizaciones.
* `Ctrl+D`: Abre la página de donación en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cierra la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activa enlace en cursor, o abre una vista de tabla cuando está en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abre el menú contextual.

## Idiomas admitidos

Paperback se traduce a muchos idiomas diferentes, y se están añadiendo más constantemente. A continuación aparece una lista completa.

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
Las siguientes personas han hecho donaciones de cierto tamaño al desarrollo de Paperback. Si haces una donación tu nombre no se añadirá automáticamente aquí, solo añado personas que quieren que su donación sea pública.

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

### Versión 0.9.1
* Los sonidos de marcadores y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrirse y rastrear su línea de tiempo en silencio.
* Se corrigió que las comillas rizadas, rayas largas y caracteres similares desaparecieran de los documentos RTF, juntando las palabras circundantes.
* Se corrigió que las imágenes RTF filtraran sus datos sin procesar en el documento como texto garbled.
* Se corrigió que el submenú Documentos recientes mantuviera entradas antiguas hasta que algo más ocurriera para reconstruirlo.
* Los aceleradores de teclado están de vuelta en cada traducción, por lo que los menús de ruso tienen acceso de teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Las opciones se han renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, el tamaño y el estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el libro de audio completo en lugar de solo su texto.
* Los nombres de acción del diálogo Personalizar atajos de teclado ahora pueden traducirse.
* El título del documento ahora aparece primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados de Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver fuente para abrir la fuente de un documento en una nueva pestaña, útil para editar Markdown por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puede cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, informe de cualquier rareza encontrada.

##### Compatibilidad de plataforma
* ¡Compatibilidad con ARM64 en Windows!
* ¡Compatibilidad nativa con macOS!
* Un botón de pantalla completa.

##### Diálogo Todos los documentos
* Un botón localizar para localizar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que pueda filtrar por estado del documento y ver cuántos documentos se muestran y seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver más abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafo;
    * Espaciado de letra;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y posterior hotkey.
* Un interruptor para determinar cómo desea que se muestren las tablas, y unificó cómo se muestran las tablas en documentos.

##### Navegación
* Compatibilidad para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de navegación en lectores de pantalla.
* El atajo de teclado de iguales para anunciar su porcentaje actual a través de un documento.

##### Marcadores
* Marcadores temporales: puede tener uno por documento, y persisten. Use la barra diagonal para establecer uno y la barra diagonal inversa para ir a él.

##### Contador de palabras
* Tiempo de lectura estimado en el diálogo de contador de palabras, así como la capacidad de establecer su velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si una selección está activa cuando abre el diálogo de contador de palabras, ahora se mostrará cuántas palabras ha seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento del menú exportar para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón cancelar al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido alterado.

##### Vista web
* La vista web ahora se abre en su posición de lectura actual.

##### Libros DAISY
* Compatibilidad con libros DAISY 2.0.
* Compatibilidad con reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente compatible con audio DAISY (incluido audio DAISY + texto) y archivos zip de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, buscar hacia adelante y hacia atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si buscar más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Compatibilidad con listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora soportan tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Rearir último cerrado" intentando reabrir el archivo readme agrupado.
* Su pestaña seleccionada no se enfoca correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se le pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el archivo readme ahora respetará su idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta resolución.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, cuando se abre la ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes reduciendo a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Escape no cierra los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualiza después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se agregará a su lista de todos los documentos cuando se abre a través de Shift+F1.
* Eliminar documentos del diálogo de recientes ahora también cerrará su pestaña activa.
* Su filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* La navegación de página anuncia texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocando el cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetan la ventana del documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deben reproducirse exclusivamente cuando navegue sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de línea lo dispara al inicio de su documento.

##### Vista web
* El diálogo de vista web no es redimensionable y aparece en un tamaño inicial muy pequeño.
* Las imágenes ahora deben mostrarse correctamente en la vista web incrustada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código de marcado en las notas de la versión.

##### Libros DAISY
* Los libros DAISY muestran información incorrecta en la barra de estado.
* Carga de libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes de posición de archivo en libros Mobi dividen las etiquetas HTML y ponen basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 enormemente mejorado.

##### Documentos de Word
* Los documentos de Word con nombres de estilo específicos de la configuración regional no renderizan sus encabezados correctamente.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producen saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no bloquearán Paperback al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office encriptados. Actualmente se admiten Word heredado, Word moderno y PowerPoint moderno, con PowerPoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos heredados de Microsoft Word (*.doc)!
* ¡Se agregó soporte para presentaciones heredadas de PowerPoint (*.ppt)!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (DAISY y Word)!
* El texto alternativo para imágenes incrustadas ahora debe mostrarse correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió que los sonidos de marcador se activen al inicio del párrafo en lugar de la posición del marcador.
* Se corrigió que ir a página esté desactivado por 1.
* Se corrigió que la tecla Escape no funcionara para cerrar el diálogo abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF solo con imágenes se detectan nuevamente y le alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará la configuración de modo oscuro de su aplicación.
* Se eliminó el soporte de DAISY XML, ya que ya no es necesario.
* Se volvió a cambiar a la navegación de primera letra nativa de Win32 en el árbol de la tabla de contenidos.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y sin problemas.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no colocaría un espacio entre palabras en casos raros.
* Se corrigió que los párrafos se dividieran en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las pestañas RTF y los alimentadores de línea ahora se renderizan exactamente como aparecen en el documento.
* Se volvió a cambiar a la biblioteca probada y verdadera de pdfium para analizar PDF, haciendo que el renderizado de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para rearir el último documento cerrado.
* El diálogo Todos los documentos ahora admite la selección de varios documentos para abrirlos a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigió que las rutas de archivo que contienen caracteres no ASCII (como bosnio š, č, ć, ž) se corrompieran al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió que el texto PDF se lea en el orden incorrecto y el espaciado incorrecto alrededor de palabras capitalizadas.
* Se corrigió que la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará su versión instalada actualmente de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional para alcanzar un marcador o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó compatibilidad con documentos RTF!
* Se agregó compatibilidad con documentos DAISY XML.
* ¡Se agregó compatibilidad con archivos de Texto de documento abierto plano!
* ¡Se agregó compatibilidad con presentaciones de documentos abiertos planos!
* Se agregó compatibilidad con separadores con s y shift+s.
* Cualquier movimiento superior a 300 caracteres ahora agregará automáticamente a su historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos de Markdown mostraran texto sin procesar en lugar de HTML renderizado en la Vista web.
* Se corrigió que las tablas no se rendericen correctamente en archivos de Markdown.
* Los PDF solo con imágenes ahora le advertirán de su existencia cuando intente cargar uno.
* Ahora es posible verificar nuevas compilaciones de desarrollo en lugar de versiones estables al verificar actualizaciones.
* Incruste correctamente la información de versión en el ejecutable de Paperback.
* Divida el diálogo de opciones en pestañas para facilitar su uso y navegación.
* Se cambió a Hayro para analizar PDF, lo que lleva a más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tabla para documentos basados en HTML y XHTML! Navegue entre tablas usando T y Shift+T, y presione Intro para ver una en una vista web.
* ¡Se agregó una característica básica de representación web! Presione Ctrl+Shift+V para abrir la sección actual de su documento en un renderizador basado en web, útil para contenido como formato complejo o muestras de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Borrar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de la versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de los botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML e HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se cerrara correctamente al usar los botones siguiente/anterior.
* Se corrigieron los TOC de epub ocasionalmente llevándolo al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió un error de desactivación por uno en la navegación de enlaces.
* Se corrigieron algunos libros que tenían espacios en blanco al final de sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos de menú relacionados con marcadores, así como la lista de elementos, ahora están deshabilitados correctamente cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documentos.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la capacidad de mantenimiento.

### Versión 0.6.1
* ¡Se agregó compatibilidad con PDF protegido por contraseña!
* Se agregó una característica muy básica de ir a la posición anterior/siguiente. Si presiona Intro en un enlace interno y mueve el cursor, esa posición se recordará ahora, y puede navegarse con las flechas Alt+Izquierda/Derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en su documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado de forma predeterminada.
* Se corrigieron los enlaces en algunos documentos Epub que no funcionaban correctamente.
* Se corrigió el análisis de Epub TOC que contiene rutas relativas.
* Se corrigió que algunos documentos epub no mostraran un título o autor.
* Se corrigieron los títulos de algunos capítulos epub que no aparecían correctamente en el diálogo TOC.
* Se corrigió que no pudiera usar la barra espaciadora para activar los botones Aceptar/Cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora recibirá retroalimentación hablada si la lista de documentos recientes está vacía cuando intente traer el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú Ir en una forma mucho más compacta al diálogo de opciones, marcada de forma predeterminada.
* Se agregó una opción para que la navegación por elementos estructurales se envuelva.
* Se agregó una opción al menú de herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* ¡Se agregó un sistema de actualización bastante simple, pero muy efectivo!
* ¡Se agregó una característica de temporizador de sueño básica, accesible con Ctrl+Shift+S!
* ¡Se agregó compatibilidad para analizar libros electrónicos FB2!
* ¡Se agregó compatibilidad para analizar presentaciones de documentos abiertos!
* ¡Se agregó compatibilidad para analizar archivos de Texto de documento abierto!
* ¡Los marcadores ahora pueden crear marcadores en una línea completa, o marcar solo texto específico. Si no tiene selección activa al colocar un marcador, el comportamiento es como en la versión 0.6 anterior, y marcará la línea completa. Sin embargo, si selecciona texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas! Navegue entre marcadores que contengan notas con N y Shift+N, o abra el diálogo de marcadores con todos los marcadores, solo notas o solo no notas seleccionados con hotkeys específicos.
* Los marcadores en el diálogo de marcadores ya no tendrán un prefijo "marcador x" molesto.
* Los libros Epub que contienen contenido HTML fingiendo ser XML ahora se manejarán correctamente.
* Se corrigió la carga de documentos de Markdown grandes.
* Se corrigió presionar espacio en la vista de árbol de la tabla de contenidos activando el botón Aceptar.
* Se corrigió el manejo de espacios en blanco al inicio de etiquetas pre tanto en documentos HTML como XHTML.
* Se corrigió que el control de texto no recuperara el enfoque a veces al volver a la ventana de Paperback.
* Se corrigió que el campo de texto en el diálogo ir a porcentaje no actualiza el valor del control deslizante.
* Se corrigió la representación de ID HTML personalizados en documentos de Markdown.
* HTML dentro de bloques de código de Markdown ahora se representará correctamente.
* Si carga un libro con un parámetro de línea de comandos mientras una instancia de Paperback existente se está ejecutando, ya no obtendrá un error si la carga del documento tarda más de 5 segundos.
* Si ejecuta Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde el diálogo de marcadores.
* Ahora es posible importar y exportar sus marcadores y posición de lectura para un documento en particular. El archivo generado se nombra según el archivo con una extensión .paperback. Si se encuentra un archivo de este tipo en el mismo directorio que un archivo al cargarlo, se cargará automáticamente. De lo contrario, puede importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Use k y shift+k para moverse hacia adelante y hacia atrás a través de ellos, y presione Intro para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido de Markdown ahora se preprocesa para ser conforme a CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Use L y Shift+L para ir por las propias listas, e I y Shift+I para ir a través de elementos de lista.
* Eliminar de teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además de eliminar normalmente.
* ¡Paperback ahora puede minimizarse opcionalmente en su bandeja del sistema! Esta opción está desactivada de forma predeterminada, pero activarla hará que la opción de minimizar en el menú del sistema ponga Paperback en su bandeja, pudiendo ser restaurada haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que admite es actualmente bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo readme en su navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido dramáticamente! En lugar de simplemente mostrarle los últimos 10 documentos que abrió, ahora le mostrará un número personalizable, siendo el resto de los documentos que ha abierto accesibles a través de un pequeño diálogo.
* Varias mejoras pequeñas en los analizadores en toda la junta, incluida la colocación de una línea en blanco entre diapositivas en presentaciones PPTX, corrección del manejo de línea nueva dentro de párrafos en documentos de palabra, y agregación de puntos de bala a elementos de lista.

### Versión 0.5.0
* ¡Se agregó compatibilidad con documentos de Microsoft Word!
* ¡Se agregó compatibilidad con presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú no deshabilitados sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o ID de fragmento.
* Se corrigió que los espacios en blanco se quitaran de los encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenidos! Cuando carga un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los encabezados en su documento, y se lo mostrará en el diálogo Ctrl+T.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar voz. Esto significa que no se envían DLL de lector de pantalla junto con el programa, y más lectores de pantalla ahora serán compatibles, como Microsoft Narrator.
* Se cambió la biblioteca zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que le pregunta si desea abrir su documento como texto sin formato ha sido completamente rehecho, y ahora le permite abrir su documento como texto sin formato, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que le permite ingresar manualmente un porcentaje para saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub ahora se conservará exactamente.
* El espacio de no ruptura unicode ahora se considera al despojar líneas en blanco.
* Ya no se le pedirá cómo desea abrir un archivo no reconocido cada vez que lo cargue, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú Inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo, si tiene un elemento secundario y padre con el mismo texto en la misma posición, ahora solo verá el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deben mostrar su título establecido en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó compatibilidad con archivos CHM!
* ¡Se agregó compatibilidad con marcadores! Puede tener tantos marcadores en tantos documentos como desee. Puede saltar hacia adelante y hacia atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y traer un diálogo para saltar a un marcador específico con control+b.
* ¡Se agregó un instalador junto con el archivo zip portátil! El instalador instalará Paperback en su directorio Archivos de programa, y configurará automáticamente asociaciones de archivos para usted.
* Los archivos de texto con BOM ahora deben decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó información mucho más detallada a la barra de estado. Ahora le mostrará su línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasa una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja con su propio diálogo basado en control deslizante, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor predeterminado.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debe escribir en el disco cuando sea absolutamente necesario.
* Se recuerda el documento que tuvo enfocado cuando cerró Paperback a través de reinicios de aplicaciones.
* La entrada en los diálogos ir a línea e ir a página ahora debe desinfectarse más estrictamente.
* Se corrigió la navegación de la tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigió que los elementos anidados de TOC en libros Epub pusieran el cursor en la posición incorrecta.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace de patrocinio de este proyecto en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos de Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo de Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió a la biblioteca PDF utilizada en Chromium, lo que lleva a un análisis de PDF mucho más confiable en toda la junta.
* Ahora solo puede tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puede presionar Eliminar en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Permita tabular desde el contenido del documento a su lista de documentos abiertos.
* Se corrigieron algunos errores con los pulsaciones de encabezado abriendo ocasionalmente documentos recientes si tenía suficientes.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió que la navegación de encabezados a veces lo pusiera en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó compatibilidad con documentos de Markdown!
* ¡Se agregó compatibilidad con documentos PDF, incluida la capacidad de navegar entre páginas!
* Se agregaron pulsaciones para navegar por encabezados en contenido HTML, incluidos libros epub y documentos de Markdown. Estas pulsaciones fueron diseñadas para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de epub con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado en ellos.
* Ahora se habla un mensaje si el documento no admite una tabla de contenidos o secciones, en lugar de los elementos del menú deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena sus últimos 10 documentos abiertos, y presionar Intro en uno lo abrirá para leer.
* ¡Se reescribió completamente el diálogo de búsqueda, haciéndolo mucho más simple de usar, mientras también se agregó un historial de sus últimas 25 búsquedas y compatibilidad con expresiones regulares!
* Los documentos abiertos previamente se recuerdan ahora a través de reinicios de aplicaciones. Esto es configurable a través del nuevo elemento de opciones en el menú de herramientas.
* Se agregó shift+f1 para abrir el archivo readme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
