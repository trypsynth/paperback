<!-- machine-translated from doc/readme.md (source-hash: bdf582cc25a739ea); please review and edit as needed -->

# Paperback - versión 0.9.0

## Introducción

Paperback es un lector de libros electrónicos y documentos ligero, rápido y accesible para todo el mundo, desde lectores ocasionales hasta usuarios avanzados intensivos. Está diseñado para ser accesible con lectores de pantalla, ofrecer gran velocidad y una experiencia libre de sobrecarga.

## Requisitos del sistema

Actualmente Paperback funciona en Windows 10/11 y en todas las versiones modernas de macOS ARM. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba públicas previstas poco después del lanzamiento 0.9.0 para escritorio, antes de una versión 1.0 unificada que abarcará las cuatro plataformas.

## Características

* Completamente autónomo, no requiere instalar ningún software en tu equipo para empezar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz sencilla con pestañas, que te permite abrir tantos documentos como quieras uno al lado del otro.
* Guarda tu posición exacta de lectura en todos los documentos que abres.
* Opcionalmente recuerda qué documentos tenías abiertos al cerrar el programa y los restaura en el siguiente inicio.
* Incluye funciones de navegación similares a las del modo de navegación web de muchos lectores de pantalla para desplazarse por los documentos de forma rápida y sencilla.
* Incluye un robusto diálogo de búsqueda, con funciones como historial y compatibilidad con expresiones regulares.
* Puede ejecutarse de forma totalmente portátil o instalarse con las asociaciones de archivos configuradas automáticamente.
* Admite una enorme variedad de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Existe, sin embargo, un problema conocido para los usuarios de JAWS.

### JAWS y líneas braille

Si usas JAWS con una línea braille, puede que los párrafos largos se corten al avanzar con las teclas de navegación de tu línea. El comando de leer el párrafo actual también se ve afectado. Este es un error en el manejo que hace JAWS del control de texto RICHEDIT50W, no algo de Paperback en sí, y uno que tardó bastante en obtener una solución dado el entusiasmo de Vispero por responder a problemas relacionados con software de código abierto.

La solución alternativa, que finalmente apareció a través del grupo de discusión de JAWS tras meses de espera, consiste en editar `paperback.jcf` y establecer «Braille Presentation and Panning» en «Always use DOM if available». También conviene activar «Pan Text by Paragraph»; de lo contrario, tu línea se quedará en el párrafo activo en lugar de avanzar. Con ambos ajustes aplicados, el desplazamiento debería funcionar correctamente.

## Tipos de archivo compatibles actualmente

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
* Presentaciones de PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto sin formato y de registro (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para usarse principalmente con el teclado. Estos son los atajos actuales.

Los atajos que figuran a continuación son para Windows. Cuando macOS difiere, el equivalente se indica entre paréntesis, principalmente porque Ctrl+G, Ctrl+W y Alt+Izquierda/Derecha ya están asignados a otras convenciones del sistema o de las aplicaciones en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Volver a abrir el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo «Todos los documentos» (desde Documentos recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS esto se encuentra en el menú de la aplicación).

### Menú Ir

* `Ctrl+F`: Mostrar el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Buscar siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a la línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir al porcentaje.
* `Ctrl+P`: Ir a la página (cuando el documento actual lo admita).
* `=`: Anunciar el porcentaje de lectura actual.
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
* `/`: Establecer el marcador temporal.
* `\`: Saltar al marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Saltar a todos los marcadores y notas.
* `Ctrl+Alt+B`: Saltar solo a los marcadores.
* `Ctrl+Alt+M`: Saltar solo a las notas.
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
* `Ctrl+T`: Mostrar el índice de contenidos.
* `F7`: Mostrar la lista de elementos.
* `Ctrl+Shift+C`: Abrir la carpeta contenedora.
* `Ctrl+Shift+V`: Abrir el contenido actual en la Vista web.
* `Ctrl+U`: Ver el código fuente del documento en una nueva pestaña.
* `Ctrl+Shift+E`: Exportar los datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos de documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto sin formato.
* `Ctrl+Shift+B`: Alternar marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Añadir o editar la nota del marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar el ajuste de línea.
* `Ctrl+Space`: Reproducir/pausar la narración de audio.
* `'`: Avanzar en la narración de audio.
* `;`: Retroceder en la narración de audio.
* `Ctrl+'`: Aumentar la cantidad de avance del audio.
* `Ctrl+;`: Reducir la cantidad de avance del audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir las opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Alternar el temporizador de suspensión.

### Menú Ayuda

* `Ctrl+F1`: Mostrar el diálogo Acerca de.
* `F1`: Ver la ayuda en el navegador predeterminado.
* `Shift+F1`: Ver la ayuda en Paperback.
* `Ctrl+Shift+U`: Buscar actualizaciones.
* `Ctrl+D`: Abrir la página de donaciones en el navegador predeterminado.

### Teclas adicionales de la vista de documento

* `Delete` / `Numpad Delete` en el control de pestañas: Cerrar la pestaña del documento seleccionado.
* `Enter` o `Space` en el texto del documento: Activar el enlace en el cursor, o abrir una vista de tabla cuando se está en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas admitidos

Paperback está traducido a muchos idiomas diferentes, y se añaden más continuamente. A continuación se ofrece una lista completa.

Para saber cómo contribuir, lee nuestra [Guía de traducción](translating.md).

* Bosnio
* Checo
* Neerlandés
* Finés
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
Las siguientes personas han realizado donaciones de algún tipo al desarrollo de Paperback. Si haces una donación, tu nombre no se añadirá automáticamente aquí; solo añado a las personas que desean que su donación se haga pública.

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

### Versión 0.9.0

#### Añadido

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos compatibles con Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que hayan sido modificados por otros programas en el disco.
* Una opción Ver código fuente para abrir el código fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto de los documentos ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor, informa de cualquier comportamiento extraño que encuentres con esto.

##### Compatibilidad de plataformas
* ¡Compatibilidad con Windows ARM64!
* ¡Compatibilidad nativa con macOS!
* Un conmutador de pantalla completa.

##### Diálogo Todos los documentos
* Un botón para localizar libros que falten y que solo hayan cambiado de ruta.
* Un filtro de estado y una barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y están seleccionados.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido de general);
    * Renderizar tablas en línea (nuevo en esta versión, véase más abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado entre líneas;
    * Espaciado entre párrafos;
    * Espaciado entre letras;
    * Alineación del texto.
* Un elemento de menú de ajuste de línea y su correspondiente tecla rápida.
* Un conmutador para determinar cómo quieres que se muestren las tablas, y se ha unificado la forma en que se muestran las tablas en todos los documentos.

##### Navegación
* Compatibilidad con la navegación por contenedores.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo de exploración de los lectores de pantalla.
* El atajo de teclado igual para anunciar tu porcentaje actual dentro de un documento.

##### Marcadores
* Marcadores temporales: puedes tener uno por documento, y sí se conservan. Usa la barra para establecer uno y la barra invertida para saltar a él.

##### Recuento de palabras
* Tiempo estimado de lectura en el diálogo de recuento de palabras, así como la posibilidad de establecer tu velocidad de lectura para que esta métrica sea realmente útil.
* Si hay una selección activa cuando abres el diálogo de recuento de palabras, ahora se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La posibilidad de personalizar todos los atajos de teclado de la aplicación mediante un sencillo diálogo.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Neerlandés, finés y polaco.

##### Exportación
* Se ha ampliado el elemento de menú de exportación para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón de cancelar en el diálogo de actualización en curso.
* El actualizador ahora valida que el archivo descargado no haya sido manipulado.

##### Vista web
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Compatibilidad con libros DAISY 2.0.
* Compatibilidad con la reproducción de audio DAISY 2.02.

##### Audiolibros
* La posibilidad de reproducir audiolibros, actualmente con compatibilidad tanto con audio DAISY (incluido audio + texto DAISY) como con archivos zip de ficheros de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar la narración, avanzar y retroceder, y ajustar la cantidad de avance.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de avance del audio y elegir si al avanzar más allá del final de un capítulo se continúa en el siguiente.

##### Documentos CHM
* Compatibilidad con listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora admiten tablas.

#### Corregido

##### General
* Los documentos codificados en codificaciones CJK antiguas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de mostrarse como un montón de mojibake.
* «Reabrir el último cerrado» intentando reabrir el archivo léeme incluido.
* Que tu pestaña seleccionada no recibiera el foco correctamente después de reiniciar Paperback.
* El manejo por parte de Paperback de archivos en unidades de red de Windows: al pulsar mostrar archivo en la carpeta ahora se enfoca correctamente el archivo en el almacenamiento en red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán a la fuerza al restaurar documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir la carpeta contenedora ahora enfoca el archivo indicado en el explorador.
* Abrir el archivo léeme ahora respetará el idioma que hayas seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta densidad de píxeles.
* El menú ahora se actualiza correctamente, y el foco se mueve al control de texto, al abrir la ayuda en Paperback.
* Se ha cambiado a un método de IPC mucho más seguro en Windows.
* El título del documento activo ahora se leerá al cambiar entre pestañas.
* Se ha reducido el uso de memoria en documentos grandes al reducir a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Que Escape no cerrara los diálogos Información del documento y Todos los documentos.
* Que la barra de título no se actualizara después de cerrar un documento desde el diálogo Todos los documentos.
* Readme.html ya no se añadirá a tu lista de todos los documentos cuando se abra mediante Shift+F1.
* Eliminar documentos del diálogo de recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* Que la navegación por páginas anunciara texto de línea incorrecto en algunas situaciones.
* Que Ir a línea, Ir a página e Ir a porcentaje colocaran el cursor en la posición equivocada en documentos grandes.
* Que Buscar y Buscar siguiente no respetaran la ventana del documento cargado en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deberían reproducirse exclusivamente cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Que aplicar el ajuste de línea te llevara al inicio del documento.

##### Vista web
* Que el diálogo de la vista web no fuera redimensionable y apareciera con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web incrustada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código Markdown en las notas de la versión.

##### Libros DAISY
* Que los libros DAISY mostraran información incorrecta en la barra de estado.
* La carga de libros DAISY con declaraciones de codificación erróneas.

##### Documentos RTF
* El análisis de documentos RTF con caracteres no latinos.
* Los grupos `\pict` de RTF, de modo que los datos de imágenes incrustadas ya no se filtren al texto del documento.

##### Libros Mobi/AZW3
* Que los anclajes filepos en los libros Mobi dividieran las etiquetas HTML e introdujeran basura en el texto del libro.
* Los enlaces en libros Mobi antiguos.
* Se ha mejorado enormemente el análisis de AZW3.

##### Documentos de Word
* Que los documentos de Word con nombres de estilo específicos de la configuración regional no renderizaran correctamente sus encabezados.

##### Documentos HTML/XHTML
* Que los elementos dl, dt y dd no produjeran saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos o marcadores ya no provocarán el cierre de Paperback al abrirlos.

### Versión 0.8.5
* Se añadió compatibilidad con páginas en los libros epub.
* Se añadió compatibilidad con documentos de Microsoft Office cifrados. Actualmente se admiten Word antiguo, Word moderno y PowerPoint moderno, con PowerPoint antiguo previsto para el futuro.
* ¡Se añadió compatibilidad con documentos antiguos de Microsoft Word (*.doc)!
* ¡Se añadió compatibilidad con presentaciones antiguas de PowerPoint (*.ppt)!
* ¡Se añadió compatibilidad con libros mobi y AZW3!
* ¡Se añadió compatibilidad con archivos PDF etiquetados!
* Se añadió el atajo ctrl+q para salir de la aplicación.
* ¡Se añadió compatibilidad con libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo de las imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora admiten correctamente la navegación por enlaces internos.
* Se corrigió que los sonidos de marcador se activaran al inicio del párrafo en lugar de en la posición del marcador.
* Se corrigió que ir a página se desviara en 1.
* Se corrigió que la tecla Escape no funcionara para cerrar el diálogo Abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o con la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento equivocado al abrir documentos desde la línea de comandos.
* Los PDF que solo contienen imágenes se detectan de nuevo y te avisan de su existencia.
* Ahora es posible navegar por imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará la configuración de modo oscuro de tu aplicación.
* Se eliminó la compatibilidad con DAISY XML, ya que ya no es necesaria.
* Se volvió a la navegación nativa de Win32 por primera letra en el árbol de la tabla de contenido.
* El diálogo de error de carga ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y con mayor fluidez.

### Versión 0.8.2
* ¡Se añadió compatibilidad con páginas en los documentos RTF!
* Se corrigió un error por el que al abrir la vista web en epubs que contenían enlaces externos estos se activaban automáticamente.
* Se corrigió un error por el que el analizador de RTF no ponía un espacio entre palabras en casos raros.
* Se corrigió que los párrafos se dividieran en varias líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen compatibilidad básica con la navegación por enlaces y encabezados!
* Las tabulaciones y los saltos de línea de RTF ahora se renderizan exactamente como aparecen en el documento.
* Se volvió a la probada y fiable biblioteca pdfium para analizar los PDF, haciendo que el renderizado de PDF vuelva a ser mucho más fiable.

### Versión 0.8.1
* Se añadió Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora permite seleccionar varios documentos para abrirlos a la vez.
* Se corrigieron algunos errores del analizador de RTF.
* Se corrigió que las rutas de archivo con caracteres no ASCII (como la š, č, ć, ž del bosnio) se corrompieran al abrir un archivo mediante una segunda instancia de Paperback.
* Se corrigió que el texto de los PDF se leyera en el orden equivocado, y el espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en los diálogos de confirmación.

### Versión 0.8.0
* ¡Se añadieron las traducciones al japonés, chino simplificado y vietnamita!
* ¡Se añadió un actualizador automático que ahora reemplazará la versión de Paperback que tengas instalada en lugar de solo descargar la nueva versión!
* ¡Se añadió una respuesta sonora opcional al llegar a un marcador o una nota, gracias a Andre Louis por los sonidos!
* ¡Se añadió compatibilidad con documentos RTF!
* Se añadió compatibilidad con documentos DAISY XML.
* ¡Se añadió compatibilidad con archivos Flat Open Document Text!
* ¡Se añadió compatibilidad con presentaciones Flat Open Document!
* Se añadió compatibilidad con separadores mediante s y shift+s.
* Cualquier movimiento de más de 300 caracteres se añadirá ahora automáticamente a tu historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en la vista web.
* Se corrigió que las tablas no se renderizaran correctamente en los archivos Markdown.
* Los PDF que solo contienen imágenes ahora te avisarán de su existencia cuando intentes cargar uno.
* Ahora es posible buscar nuevas compilaciones de desarrollo en lugar de versiones estables al comprobar si hay actualizaciones.
* Se incrusta correctamente la información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar su uso y navegación.
* Se cambió a Hayro para analizar los PDF, lo que aporta más fiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga los documentos más rápido y es más fácil de mantener y ampliar.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se añadió compatibilidad con tablas para documentos basados en HTML y XHTML! Navega entre las tablas usando T y Shift+T, y pulsa Enter para ver una en una vista web.
* ¡Se añadió una función básica de renderizado web! Pulsa Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formatos complejos o ejemplos de código.
* ¡Se añadió una traducción al ruso, gracias a Ruslan Gulmagomedov!
* Se añadió un botón Borrar todo al diálogo Todos los documentos.
* El comprobador de actualizaciones ahora muestra las notas de la versión cuando hay una nueva disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de los botones Sí/No en los diálogos de confirmación.
* Se corrigió la carga de configuraciones al ejecutar como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de la tabla de contenido en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenido.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que las tablas de contenido de los epub te llevaran ocasionalmente al elemento equivocado.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió un error de desfase de uno en la navegación por enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco al final de sus líneas.
* Se corrigieron varios problemas de los analizadores.
* Los elementos de menú relacionados con marcadores, así como la lista de elementos, ahora se desactivan correctamente cuando no hay ningún documento abierto.
* Se mejoró el manejo de listas en varios formatos de documento.
* Se mejoró el flujo de trabajo de traducción para los colaboradores.
* Muchas refactorizaciones internas, trasladando la mayor parte de la lógica de negocio de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se añadió compatibilidad con PDF protegidos con contraseña!
* Se añadió una función muy básica de ir a la posición anterior/siguiente. Si pulsas Enter en un enlace interno y esto mueve el cursor, esa posición se recordará ahora y podrás navegar a ella con alt+flechas izquierda/derecha.
* ¡Se añadió una lista de elementos! Actualmente solo muestra un árbol con todos los encabezados de tu documento o una lista de enlaces, pero hay planes para ampliarla en el futuro.
* Se añadió una opción para iniciar Paperback maximizado por defecto.
* Se corrigió que los enlaces en algunos documentos Epub no funcionaran correctamente.
* Se corrigió el análisis de tablas de contenido de Epub que contienen rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título ni autor.
* Se corrigió que los títulos de algunos capítulos de epub no aparecieran correctamente en el diálogo de la tabla de contenido.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones Aceptar/Cancelar en el diálogo de la tabla de contenido.
* Se mejoró el manejo de los encabezados en los documentos de Word.
* Ahora recibirás una respuesta hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se ha añadido al diálogo de opciones una nueva opción para mostrar el menú Ir en una forma mucho más compacta, activada por defecto.
* Se añadió una opción para que la navegación por elementos estructurales sea cíclica.
* Se añadió una opción al menú Herramientas para abrir la carpeta contenedora del documento actualmente enfocado.
* Se añadió un sistema de actualización bastante sencillo, pero muy eficaz.
* Se añadió una función básica de temporizador de sueño, accesible con Ctrl+Shift+S.
* ¡Se añadió compatibilidad con el análisis de libros electrónicos FB2!
* ¡Se añadió compatibilidad con el análisis de presentaciones OpenDocument!
* ¡Se añadió compatibilidad con el análisis de archivos OpenDocument Text!
* Los marcadores ahora se pueden crear para marcar una línea entera o solo un texto especificado. Si no tienes ninguna selección activa al colocar un marcador, el comportamiento es como antes de 0.6, y marcará toda la línea. Sin embargo, si seleccionas algo de texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas! Navega entre los marcadores que contienen notas con N y Shift+N, o abre el diálogo de marcadores con todos los marcadores, solo las notas o solo los que no son notas mediante atajos específicos.
* Los marcadores en el diálogo de marcadores ya no tendrán un molesto prefijo «marcador x».
* Los libros Epub que contienen contenido HTML que finge ser XML ahora se manejarán correctamente.
* Se corrigió la carga de documentos Markdown grandes.
* Se corrigió que al pulsar espacio en la vista de árbol de la tabla de contenido se activara el botón Aceptar.
* Se corrigió el manejo de espacios en blanco al principio de las etiquetas pre tanto en documentos HTML como XHTML.
* Se corrigió que el control de texto a veces no recuperara el foco al volver a la ventana de Paperback.
* Se corrigió que el campo de texto del diálogo Ir a porcentaje no actualizara el valor del deslizador.
* Se corrigió el renderizado de ID HTML personalizados en documentos Markdown.
* El HTML dentro de los bloques de código de Markdown ahora se renderizará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras hay una instancia de Paperback en ejecución, ya no recibirás un error si cargar tu documento tarda más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde el diálogo de marcadores.
* Ahora es posible importar y exportar tus marcadores y tu posición de lectura de un documento concreto. El archivo generado se nombra según el archivo con la extensión .paperback. Si se encuentra un archivo así en el mismo directorio que un archivo al cargarlo, se cargará automáticamente. Si no, puedes importarlos manualmente usando un elemento del menú Herramientas.
* ¡Los enlaces dentro de los documentos ahora son totalmente compatibles! Usa k y shift+k para avanzar y retroceder por ellos, y pulsa Enter para abrir/activar uno.
* Muchas refactorizaciones internas, que hacen la aplicación más rápida y el binario más pequeño.
* El contenido Markdown ahora se preprocesa para que cumpla con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Usa L y Shift+L para ir por las listas en sí, e I y Shift+I para recorrer los elementos de lista.
* La tecla Suprimir del teclado numérico ahora funciona para eliminar documentos de la barra de pestañas, además de la tecla Suprimir normal.
* ¡Paperback ahora puede minimizarse opcionalmente a la bandeja del sistema! Esta opción está desactivada por defecto, pero al activarla la opción minimizar del menú del sistema colocará Paperback en tu bandeja, pudiendo restaurarlo haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que admite es actualmente bastante reducida, pero crece constantemente.
* ¡Paperback ya tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenido básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el archivo léeme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha ampliado enormemente! En lugar de mostrarte simplemente los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, y el resto de los documentos que hayas abierto alguna vez serán accesibles mediante un pequeño diálogo.
* Diversas pequeñas mejoras en los analizadores en general, incluyendo poner una línea en blanco entre las diapositivas de las presentaciones PPTX, corregir el manejo de los saltos de línea dentro de los párrafos en los documentos de Word y añadir viñetas a los elementos de lista.

### Versión 0.5.0
* ¡Se añadió compatibilidad con documentos de Microsoft Word!
* ¡Se añadió compatibilidad con presentaciones de PowerPoint!
* Se corrigió que ciertos elementos de menú no se desactivaran sin documentos abiertos.
* Se corrigió la orientación del deslizador de Ir a porcentaje.
* Se corrigió la tabla de contenido en libros Epub con rutas de archivo o ID de fragmento codificados en URL.
* Se corrigió que se eliminaran espacios en blanco de los encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora admiten la función de tabla de contenido! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenido a partir de la estructura de los encabezados de tu documento, y te la mostrará en el diálogo de ctrl+t.
* Los documentos HTML ahora tendrán el título establecido en la etiqueta title, si existe. Si no, seguirán usando el nombre del archivo sin la extensión.
* Se cambió de UniversalSpeech al uso de una región activa para comunicar el habla. Esto significa que ya no se distribuyen DLL de lectores de pantalla junto con el programa, y ahora se admitirán más lectores de pantalla, como el Narrador de Microsoft.
* Se cambiaron las bibliotecas zip para permitir abrir una mayor variedad de libros epub.
* El diálogo que te pregunta si quieres abrir tu documento como texto sin formato se ha rehecho por completo, y ahora te permite abrir tu documento como texto sin formato, HTML o Markdown.
* El diálogo Ir a porcentaje ahora incluye un campo de texto que te permite introducir manualmente un porcentaje al que saltar.
* El analizador de HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenido de los libros Epub volverá a conservarse exactamente.
* El espacio de no separación Unicode ahora se tiene en cuenta al eliminar líneas en blanco.
* Ya no se te preguntará cómo quieres abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se añadió un icono opcional en el menú de inicio al instalador.
* La tabla de contenido debería ser ahora más limpia en algunos casos; por ejemplo, si tienes un elemento hijo y otro padre con el mismo texto en la misma posición, ahora solo verás el elemento padre.
* Se corrigió la tabla de contenido en ciertos documentos CHM.
* Se corrigió la tabla de contenido en libros Epub 3 con rutas absolutas.
* Los documentos CHM ahora deberían mostrar su título tal como está establecido en el archivo de metadatos.

### Versión 0.4.0
* ¡Se añadió compatibilidad con archivos CHM!
* ¡Se añadió compatibilidad con marcadores! Puedes tener tantos marcadores repartidos en tantos documentos como quieras. Puedes avanzar y retroceder por ellos con b y shift+b, establecer uno con control+shift+b y abrir un diálogo para saltar a un marcador concreto con control+b.
* ¡Se añadió un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Program Files y configurará automáticamente las asociaciones de archivos.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto.
* Se añadió mucha más información a la barra de estado. Ahora te mostrará tu línea actual, el carácter y el porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento por porcentaje ahora se gestiona mediante su propio diálogo basado en un deslizador, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor por defecto.
* La lógica de guardado de la posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que tenías enfocado al cerrar Paperback ahora se recuerda entre reinicios de la aplicación.
* La entrada en los diálogos Ir a línea e Ir a página ahora debería depurarse de forma más estricta.
* Se corrigió la navegación por la tabla de contenido en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenido en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación por encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigió que los elementos anidados de la tabla de contenido en libros Epub colocaran el cursor en la posición equivocada.
* Se corrigió un cierre inesperado al salir de la aplicación en ciertos casos.
* ¡Se añadió una casilla de verificación en el diálogo de opciones para activar o desactivar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento Donar del menú Ayuda o mediante el enlace «sponsor this project» al final de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambiaron las bibliotecas de PDF a la utilizada en Chromium, lo que aporta un análisis de PDF mucho más fiable en general.
* Ahora solo puedes tener una instancia de Paperback en ejecución a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya está en ejecución abrirá ese documento en la instancia que ya está funcionando.
* Ahora puedes pulsar Suprimir en un documento del control de pestañas para cerrarlo.

### Versión 0.2.1
* Se añadió el número total de páginas a la etiqueta de página en el diálogo Ir a página.
* Se permite tabular desde el contenido del documento hasta tu lista de documentos abiertos.
* Se corrigió que las combinaciones de teclas de encabezados a veces abrieran documentos recientes si tenías suficientes.
* Paperback ahora eliminará los guiones suaves innecesarios de la salida de texto.
* Se corrigió que la navegación por encabezados a veces te situara en el carácter equivocado.

### Versión 0.2.0
* ¡Se añadió compatibilidad con documentos Markdown!
* ¡Se añadió compatibilidad con documentos PDF, incluida la posibilidad de navegar entre páginas!
* Se añadieron combinaciones de teclas para navegar por encabezados en contenido HTML, incluidos libros epub y documentos Markdown. Estas combinaciones se diseñaron para funcionar de forma similar a un lector de pantalla.
* Se corrigió la carga de epubs con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado en ellos.
* Ahora se pronuncia un mensaje si el documento no admite tabla de contenido o secciones, en lugar de desactivar los elementos de menú.
* ¡Se añadió un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y al pulsar Enter en uno se abrirá para su lectura.
* Se reescribió por completo el diálogo Buscar, haciéndolo mucho más sencillo de usar, ¡y añadiendo además un historial de tus últimas 25 búsquedas y compatibilidad con expresiones regulares!
* Los documentos abiertos anteriormente ahora se recuerdan entre reinicios de la aplicación. Esto es configurable mediante el nuevo elemento Opciones del menú Herramientas.
* Se añadió shift+f1 para abrir el archivo léeme directamente en Paperback.

### Versión 0.1.0
* Lanzamiento inicial.
