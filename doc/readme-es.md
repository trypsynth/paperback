<!-- machine-translated from doc/readme.md (source-hash: 197dbd0c570ba62e); please review and edit as needed -->

# Edición en rústica - versión 0.8.5 {#paperback---version-0.8.5}

## Introducción {#introduction}

Paperback es un lector de libros electrónicos y documentos ligero,
rápido y accesible para todo el mundo, desde lectores ocasionales hasta
usuarios avanzados. Está diseñado para ser compatible con lectores de
pantalla, ofrecer una gran velocidad y una experiencia sin elementos
superfluos.

## Requisitos del sistema {#system-requirements}

Actualmente, Paperback funciona en Windows, macOS, iOS y Android.

## Características {#features}

-   Totalmente autónomo, no requiere instalar ningún software en tu
    ordenador para empezar a leer.
-   Increíblemente rápido, incluso en equipos antiguos.
-   Interfaz sencilla con pestañas, que te permite abrir tantos
    documentos como quieras en paralelo.
-   Guarda tu posición exacta de lectura en todos los documentos que
    abrís.
-   Opcionalmente, recuerda qué documentos tenías abiertos cuando
    cerraste el programa y los restaura la próxima vez que lo inicies.
-   Incluye una funcionalidad de navegación similar a la que se
    encuentra en el modo de navegación web de muchos lectores de
    pantalla para desplazarse rápida y fácilmente por los documentos.
-   Incluye un sólido cuadro de diálogo de búsqueda, con funciones como
    el historial y compatibilidad con expresiones regulares.
-   Se puede ejecutar de forma totalmente portátil o instalarse con las
    asociaciones de archivos configuradas automáticamente.
-   Es compatible con una amplia gama de formatos de archivo habituales.

## Compatibilidad con lectores de pantalla {#screen-reader-compatibility}

Paperback funciona bien con todos los principales lectores de pantalla.
Sin embargo, existe un problema conocido para los usuarios de JAWS.

### JAWS y pantallas braille {#jaws-and-braille-displays}

Si utilizas JAWS con un display braille, es posible que observes que los
párrafos largos se truncan al desplazarte hacia adelante con las teclas
de navegación de tu display. El comando «leer párrafo actual» también se
ve afectado. Se trata de un error en la gestión que hace JAWS del
control de texto RICHEDIT50W, no de algo propio de Paperback, y para el
que tardó bastante tiempo en aparecer una solución, dado el entusiasmo
de Vispero por responder a los problemas del software de código abierto.

La solución provisional, que finalmente salió a la luz a través del
grupo de debate de JAWS tras meses de espera, consiste en editar
`paperback.jcf` y configurar «Presentación y desplazamiento en braille»
en «Usar siempre DOM si está disponible». También te conviene activar
«Desplazar el texto por párrafos»; de lo contrario, tu pantalla se
quedará en el párrafo activo en lugar de avanzar. Con ambos ajustes
configurados, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente compatibles {#currently-supported-file-types}

Paperback es compatible con los siguientes formatos y extensiones:

-   Archivos de ayuda CHM (`.chm`)
-   libros DAISY (`.opf`, `.zip`)
-   Libros EPUB (`.epub`)
-   Libros electrónicos FB2 (`.fb2`)
-   Documentos HTML (`.htm`, `.html`, `.xhtml`)
-   Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Documentos de Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Presentaciones OpenDocument (`.odp`, `.fodp`)
-   Archivos de texto de OpenDocument (`.odt`, `.fodt`)
-   Documentos PDF (`.pdf`)
-   Presentaciones de PowerPoint (`.pptx`, `.pptm`, `.ppt`)
-   Documentos RTF (`.rtf`)
-   Texto sin formato y archivos de registro (`.txt`, `.log`)

## Atajos de teclado {#keyboard-shortcuts}

Paperback está diseñado para utilizarse principalmente con el teclado. A
continuación se indican los atajos actuales.

Los atajos que aparecen a continuación son para Windows. Cuando hay
diferencias con macOS, el equivalente se indica entre paréntesis,
principalmente porque Ctrl+G, Ctrl+W y Alt+Izquierda/Derecha ya están
ocupados por otras convenciones del sistema o de las aplicaciones en esa
plataforma.

### Menú «Archivo» {#file-menu}

-   `Ctrl+O`: Abrir un documento.
-   `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos
    abiertos.
-   `Ctrl+Shift+T`: Volver a abrir el último documento cerrado.
-   `Ctrl+R`: Mostrar el cuadro de diálogo «Todos los documentos» (desde
    «Documentos recientes» ).
-   `Ctrl+Q`: Salir (solo en Windows; en macOS se encuentra en el menú
    de la aplicación).

### Menú Ir {#go-menu}

-   `Ctrl+F`: Mostrar el cuadro de diálogo «Buscar».
-   `F3` (macOS: `Cmd+G`): Buscar el siguiente.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar el anterior.
-   `Ctrl+G` (macOS: `Cmd+L`): Ir a la línea.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir al porcentaje.
-   `Ctrl+P`: Ir a la página (si el documento actual lo admite).
-   `Alt+Left` (macOS: `Cmd+[`): Retroceder en el historial de
    navegación.
-   `Alt+Right` (macOS: `Cmd+]`): Avanzar en el historial de navegación.
-   `[`: Sección anterior.
-   `]`: Siguiente sección.
-   `Shift+H`: Encabezado anterior.
-   `H`: Título siguiente.
-   `Shift+1` hasta `Shift+6`: Encabezado anterior en los niveles 1-6.
-   `1` a través de `6`: Siguiente encabezado de nivel 1-6.
-   `Shift+P`: Página anterior.
-   `P`: Página siguiente.
-   `Shift+B`: Marcador anterior.
-   `B`: Marcador siguiente.
-   `Shift+N`: Nota anterior.
-   `N`: Nota siguiente.
-   `Ctrl+B`: Ir a todos los marcadores y notas.
-   `Ctrl+Alt+B`: Ir solo a los marcadores.
-   `Ctrl+Alt+M`: Ir solo a las notas.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, es decir, la tecla Control
    física en lugar de Cmd): Ver el texto de la nota en la posición
    actual.
-   `Shift+K`: Enlace anterior.
-   `K`: Enlace siguiente.
-   `Shift+G`: Imagen anterior.
-   `G`: Imagen siguiente.
-   `Shift+F`: Figura anterior.
-   `F`: Figura siguiente.
-   `Shift+T`: Tabla anterior.
-   `T`: Tabla siguiente.
-   `Shift+S`: Separador anterior.
-   `S`: Separador siguiente.
-   `Shift+L`: Lista anterior.
-   `L`: Lista siguiente.
-   `Shift+I`: Elemento anterior de la lista.
-   `I`: Elemento siguiente de la lista.
-   `Shift+,`: Ir al principio del contenedor actual (lista o tabla).
-   `,`: Ir más allá del final del contenedor actual (lista o tabla).

### Menú «Herramientas» {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en
    lugar de Cmd): Mostrar el recuento de palabras del documento actual.
-   `Ctrl+I`: Mostrar información del documento.
-   `Ctrl+T`: Mostrar la tabla de contenidos.
-   `F7`: Mostrar la lista de elementos.
-   `Ctrl+Shift+C`: Abrir la carpeta que lo contiene.
-   `Ctrl+Shift+V`: Abrir el contenido actual en la vista web.
-   `Ctrl+U`: Ver el código fuente del documento en una nueva pestaña.
-   `Ctrl+Shift+E`: Exportar los datos del documento (`.paperback`).
-   `Ctrl+Shift+I`: Importar los datos del documento (`.paperback`).
-   `Ctrl+E`: Exportar el documento actual a texto sin formato.
-   `Ctrl+Shift+B`: Añadir o eliminar un marcador en la selección o
    posición del cursor actual.
-   `Ctrl+Shift+N`: Añadir o editar una nota de marcador en la selección
    o posición del cursor actual.
-   `Ctrl+Alt+W`: Activar o desactivar el ajuste de línea.
-   `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de la
    aplicación ).
-   `Ctrl+Shift+S`: Activar o desactivar el temporizador de suspensión.

### Menú Ayuda {#help-menu}

-   `Ctrl+F1`: Mostrar el cuadro de diálogo «Acerca de».
-   `F1`: Ver la ayuda en tu navegador predeterminado.
-   `Shift+F1`: Ver la ayuda en Paperback.
-   `Ctrl+Shift+U`: Buscar actualizaciones.
-   `Ctrl+D`: Abrir la página de donaciones en tu navegador
    predeterminado.

### Teclas adicionales para la visualización de documentos {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` en el control de pestañas: Cierra la
    pestaña del documento seleccionado.
-   `Enter` o `Space` en el texto del documento: Activar el enlace
    situado en el cursor o abrir una vista de tabla cuando se encuentre
    sobre un marcador de tabla .
-   `Shift+F10` o la tecla Menú/Aplicación en el texto del documento :
    abre el menú contextual.

## Idiomas compatibles {#supported-languages}

Paperback está traducido a muchos idiomas diferentes, y se van añadiendo
más continuamente. A continuación se incluye una lista completa.

Para saber cómo colaborar, lee nuestra [Guía de
traducción](translating.md).

-   Bosnio
-   checo
-   Neerlandés
-   Finlandés
-   Francés
-   Alemán
-   Japonés
-   Polaco
-   Portugués (Brasil)
-   Ruso
-   Chino simplificado
-   Serbio
-   Español
-   Vietnamita

## Créditos {#credits}

### Desarrollo {#development}

-   Quin Gillespie: desarrollador principal y fundador del proyecto.
-   Aryan Choudhary: colaborador principal.

### Donaciones {#donations}

Las siguientes personas han realizado donaciones de cierta cuantía para
el desarrollo de Paperback. Si realizas una donación, tu nombre no
aparecerá automáticamente aquí; solo incluyo a aquellas personas que
desean que su donación se haga pública.

Nota: Considero que ser patrocinador público en GitHub es motivo para la
inclusión automática en esta lista.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Felix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodríguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Pérez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Registro de cambios {#changelog}

### Versión 0.9.0 (sin publicar) {#version-0.9.0-unreleased}

-   Se ha añadido un botón de cancelación al cuadro de diálogo de
    actualización en curso.
-   Se ha añadido una herramienta de línea de comandos, llamada «pb»,
    para convertir rápidamente cualquiera de los formatos compatibles
    con Paperback a HTML, Markdown o texto sin formato.
-   Se ha añadido un atajo de teclado configurable para restaurar
    Paperback desde la bandeja del sistema.
-   Se ha añadido un botón «Buscar» al cuadro de diálogo «Todos los
    documentos» para localizar los libros que faltan y cuya ruta acaba
    de cambiar.
-   Se ha añadido la tecla = para ver el porcentaje del libro que has
    leído hasta el momento.
-   Se ha añadido una opción para mover el cursor de texto al principio
    de la línea al navegar, de forma similar al modo de navegación de
    algunos lectores de pantalla.
-   Se ha añadido una pestaña de «Legibilidad» al cuadro de diálogo de
    opciones, con las siguientes opciones:
    -   Ajustar al ancho de línea (trasladado desde «General»);
    -   Mostrar tablas en línea (novedad de esta versión, ver más
        abajo);
    -   Fuente;
    -   Color de fondo;
    -   Interlineado;
    -   Espaciado entre párrafos;
    -   Espaciado entre letras;
    -   Alineación del texto.
-   Se ha añadido un botón para elegir cómo se muestran las tablas y se
    ha unificado la forma en que se muestran las tablas en todos los
    documentos.
-   Se ha añadido la opción «Ver código fuente» para abrir el código
    fuente de un documento en una nueva pestaña, lo cual resulta útil,
    por ejemplo, para editar Markdown.
-   Se ha añadido una opción de menú para el ajuste de línea y la
    correspondiente tecla de acceso rápido.
-   Se ha añadido el tiempo estimado de lectura al cuadro de diálogo de
    recuento de palabras, así como la posibilidad de configurar tu
    velocidad de lectura para que esta métrica resulte realmente útil.
-   ¡Se ha añadido compatibilidad con ARM64 en Windows!
-   ¡Se ha añadido compatibilidad con Android!
-   ¡Se ha añadido compatibilidad con iOS!
-   ¡Se ha añadido compatibilidad con macOS!
-   Se han añadido nuevos idiomas: neerlandés, finés y polaco.
-   Se ha añadido compatibilidad con la navegación por contenedores.
-   Se ha añadido compatibilidad con listas, elementos de lista, figuras
    e imágenes en documentos CHM .
-   Se han añadido marcadores temporales: pulsa la barra para crear uno
    y la barra invertida para saltar a él.
-   Los sonidos de marcadores y notas ahora deberían reproducirse
    correctamente y de forma exclusiva cuando navegues sobre una palabra
    que contenga uno.
-   Los documentos codificados en codificaciones CJK heredadas, como
    GBK, Big5 y Shift_JIS, ahora se mostrarán correctamente en lugar de
    aparecer como un montón de caracteres ilegibles.
-   Los documentos cuyo contenido cambie en el disco ahora pueden, de
    forma opcional, recargarse automáticamente con el nuevo contenido.
-   Se ha ampliado la opción del menú de exportación para permitir la
    exportación a HTML y Markdown, además de a texto sin formato.
-   Se ha corregido el error por el que al aplicar el ajuste de línea se
    volvía al principio del documento.
-   Se ha corregido el error por el que los libros «Daisy» mostraban
    información incorrecta en la barra de estado.
-   Se ha corregido el error por el que los elementos dl, dt y dd no
    generaban saltos de línea en los documentos XHTML .
-   Se ha corregido el error por el que la tecla Escape no cerraba los
    cuadros de diálogo «Información del documento» y «Todos los
    documentos». Se ha corregido el error por el que los enlaces
    «filepos» en los libros Mobi dividían las etiquetas HTML e
    insertaban
-   Se ha corregido un error por el que los enlaces «filepos» en libros
    Mobi dividían las etiquetas HTML e insertaban caracteres indeseados
    en el texto del libro.
-   Se ha corregido el retraso al acercarse al final del campo de texto
    en documentos de gran tamaño.
-   Se han corregido los enlaces en libros Mobi antiguos.
-   Se ha corregido la carga de libros DAISY con declaraciones de
    codificación erróneas.
-   Se ha corregido un error por el que la navegación por páginas
    mostraba texto de línea incorrecto en algunas situaciones.
-   Se ha corregido el análisis de documentos RTF con caracteres no
    latinos y caracteres de escape Unicode «?» en su interior.
-   Se ha corregido el error por el que la opción «Reabrir el último
    cerrado» intentaba reabrir el archivo «readme» incluido.
-   Se ha corregido el error por el que la barra de título no se
    actualizaba tras cerrar un documento desde el cuadro de diálogo
    «Todos los documentos».
-   Se ha corregido el problema por el que el cuadro de diálogo de la
    vista web no se podía redimensionar y aparecía con un tamaño inicial
    muy pequeño.
-   Se ha corregido un error por el que los documentos de Word con
    nombres de estilos específicos de la configuración regional no
    mostraban correctamente sus encabezados.
-   Se ha corregido el problema por el que la pestaña seleccionada no
    recibía el foco correctamente tras reiniciar Paperback.
-   Si hay una selección activa al abrir el cuadro de diálogo de
    recuento de palabras, ahora se mostrará el número de palabras
    seleccionadas.
-   Las imágenes ahora deberían mostrarse correctamente en la vista web
    incrustada.
-   Se ha mejorado la gestión de archivos de Paperback en unidades de
    red de Windows: al pulsar «Mostrar archivo en la carpeta», ahora se
    selecciona correctamente el archivo en el almacenamiento de red, y
    las rutas ya no contienen caracteres extraños. Se ha mejorado
    considerablemente el análisis de AZW3.
-   Se ha mejorado considerablemente el análisis de AZW3.
-   Se ha dejado de utilizar chmlib para pasar a nuestro propio lector
    de archivos CHM escrito íntegramente en Rust.
-   En el escritorio, los archivos .paperback ya no se cargarán de forma
    forzada al restaurar un documento. En su lugar, se te pedirá
    confirmación cuando se encuentre el archivo.
-   Paperback recurre ahora a la extracción de texto sin formato para
    los archivos PDF con etiquetas erróneas .
-   Al abrir la carpeta que contiene el archivo, ahora se selecciona
    dicho archivo en el Explorador.
-   Al abrir el archivo «readme», ahora se respetará el idioma
    seleccionado.
-   Los documentos de PowerPoint ahora admiten tablas.
-   Actualización correcta del menú y establecimiento del foco en el
    control de texto al abrir la ayuda en Paperback.
-   El archivo «Readme.html» ya no se añadirá a la lista de «Todos los
    documentos» cuando se abra mediante Mayús+F1.
-   Al eliminar documentos del cuadro de diálogo «Recientes», ahora
    también se cerrará su pestaña activa.
-   Se ha cambiado a un método de comunicación entre procesos (IPC)
    mucho más seguro en Windows.
-   Ahora se leerá el título del documento activo al cambiar entre
    pestañas.
-   El actualizador ahora muestra correctamente el contenido de las
    etiquetas de código Markdown en las notas de la versión.
-   El programa de actualización comprueba ahora que el archivo
    descargado no haya sido alterado.
-   La vista web se abre ahora en la posición de lectura actual.
-   Tu filtro de búsqueda en el cuadro de diálogo «Todos los documentos»
    se conserva ahora tras eliminar un documento.

### Versión 0.8.5 {#version-0.8.5}

-   Se ha añadido compatibilidad con páginas para los libros en formato
    ePub.
-   Se ha añadido compatibilidad con documentos cifrados de Microsoft
    Office. Actualmente se admiten Word clásico, Word moderno y
    PowerPoint moderno, y está prevista la compatibilidad con PowerPoint
    clásico en el futuro.
-   ¡Se ha añadido compatibilidad con documentos antiguos de Microsoft
    Word (\*.doc)!
-   ¡Se ha añadido compatibilidad con presentaciones antiguas de
    PowerPoint (\*.ppt)!
-   ¡Se ha añadido compatibilidad con libros en formato mobi y AZW3!
-   ¡Se ha añadido compatibilidad con archivos PDF etiquetados!
-   Se ha añadido el atajo de teclado Ctrl+Q para salir de la
    aplicación.
-   ¡Se ha añadido compatibilidad con libros comprimidos de Bookshare
    (tanto DAISY como Word)!
-   El texto alternativo de las imágenes incrustadas ahora debería
    mostrarse correctamente.
-   Los documentos CHM ahora admiten correctamente la navegación
    mediante enlaces internos.
-   Se ha corregido el error por el que los sonidos de los marcadores se
    activaban al inicio del párrafo en lugar de en la posición del
    marcador.
-   Se ha corregido el error por el que la función «Ir a la página» se
    desviaba en 1.
-   Se ha corregido el error por el que la tecla Esc no cerraba el
    cuadro de diálogo «Abrir como».
-   Se ha corregido el error por el que el menú contextual del lector no
    aparecía al hacer clic con el botón derecho o al pulsar la tecla
    «Aplicaciones».
-   Se ha corregido el error por el que, en ocasiones, se seleccionaba
    el documento equivocado al abrir documentos desde la línea de
    comandos.
-   Los archivos PDF que solo contienen imágenes vuelven a detectarse y
    se te avisa de su existencia.
-   Ahora es posible navegar por imágenes y figuras con g/Shift+g y
    f/Shift+f, respectivamente.
-   Paperback respetará ahora la configuración del modo oscuro de tu
    aplicación.
-   Se ha eliminado la compatibilidad con DAISY XML, ya que ya no es
    necesaria.
-   Se ha vuelto a la navegación nativa de Win32 por la primera letra en
    el árbol de la tabla de contenidos.
-   El cuadro de diálogo de error al cargar ahora muestra mensajes de
    error más detallados.
-   La vista web se abrirá ahora mucho más rápido y con mayor fluidez.

### Versión 0.8.2 {#version-0.8.2}

-   ¡Se ha añadido compatibilidad con páginas a los documentos RTF!
-   Se ha corregido un error por el que, al abrir la vista web en
    archivos EPUB que contenían enlaces externos, estos se activaban
    automáticamente.
-   Se ha corregido un error por el que, en contadas ocasiones, el
    analizador de RTF no insertaba un espacio entre las palabras .
-   Se ha corregido el problema por el que los párrafos se dividían en
    varias líneas cortas en algunos documentos PDF.
-   ¡Los documentos PDF ahora admiten la navegación básica por enlaces y
    encabezados !
-   Las tabulaciones y los saltos de línea RTF ahora se representan
    exactamente tal y como aparecen en el documento.
-   Se ha vuelto a utilizar la biblioteca «pdfium», de probada eficacia,
    para el análisis de archivos PDF, lo que hace que la visualización
    de los PDF vuelva a ser mucho más fiable.

### Versión 0.8.1 {#version-0.8.1}

-   Se ha añadido la combinación Ctrl+Mayús+T para volver a abrir el
    último documento cerrado.
-   El cuadro de diálogo «Todos los documentos» ahora permite
    seleccionar varios documentos para abrirlos a la vez.
-   Se han corregido algunos errores del analizador RTF.
-   Se ha corregido el problema por el que las rutas de archivo que
    contenían caracteres no ASCII (como las letras bosnias š, č, ć, ž)
    se corrompían al abrir un archivo a través de una segunda instancia
    de Paperback .
-   Se ha corregido el problema por el que el texto de los PDF se leía
    en el orden incorrecto y el espaciado alrededor de las palabras en
    mayúsculas era incorrecto.
-   Se ha corregido la lentitud en la carga de documentos al abrir
    archivos de gran tamaño.
-   Se ha corregido la localización de los botones «Sí»/«No» en los
    cuadros de diálogo de confirmación. Versión 0.8.0

### Versión 0.8.0 {#version-0.8.0}

-   ¡Se han añadido traducciones al japonés, al chino simplificado y al
    vietnamita !
-   Se ha añadido un actualizador automático que ahora sustituirá la
    versión actualmente instalada de Paperback, en lugar de limitarse a
    descargar la nueva versión.
-   Se ha añadido una señal sonora opcional al llegar a un marcador o a
    una nota; ¡gracias a Andre Louis por los sonidos!
-   ¡Se ha añadido compatibilidad con documentos RTF!
-   Se ha añadido compatibilidad con documentos DAISY XML.
-   ¡Se ha añadido compatibilidad con archivos de texto Flat Open
    Document!
-   ¡Se ha añadido compatibilidad con presentaciones Flat Open Document!
-   Se ha añadido compatibilidad con los separadores mediante «s» y
    «Shift+s».
-   Cualquier desplazamiento de más de 300 caracteres se añadirá ahora
    automáticamente a tu historial de navegación.
-   Se ha corregido la restauración de la ventana de Paperback desde la
    bandeja del sistema.
-   Se ha corregido el error por el que los documentos Markdown
    mostraban texto sin formato en lugar de HTML renderizado en la vista
    web.
-   Se ha corregido el error por el que las tablas no se mostraban
    correctamente en los archivos Markdown.
-   Los archivos PDF que solo contienen imágenes ahora te avisarán de su
    existencia cuando intentes cargar uno.
-   Ahora es posible buscar nuevas versiones de desarrollo en lugar de
    versiones estables al comprobar si hay actualizaciones.
-   Se ha integrado correctamente la información de la versión en el
    ejecutable de Paperback.
-   Se ha dividido el cuadro de diálogo de opciones en pestañas para
    facilitar su uso y navegación.
-   Se ha cambiado a Hayro para el análisis de archivos PDF, lo que
    aporta mayor fiabilidad, velocidad y un menor número de DLL.
-   Se ha reescrito toda la aplicación en Rust. El nuevo código es más
    seguro, carga los documentos más rápido y es más fácil de mantener y
    ampliar.
-   El menú contextual del control de texto incluirá ahora acciones
    específicas del lector en lugar de elementos genéricos como cortar y
    pegar.

### Versión 0.7.0 {#version-0.7.0}

-   ¡Se ha añadido compatibilidad con tablas para documentos basados en
    HTML y XHTML! Navega entre tablas con las teclas T y Mayús+T, y
    pulsa Intro para ver una de ellas en una vista web.
-   ¡Se ha añadido una función básica de visualización web! Pulsa
    Ctrl+Mayús+V para abrir la sección actual de tu documento en un
    visor web, algo útil para contenidos como formatos complejos o
    ejemplos de código.
-   Se ha añadido una traducción al ruso. ¡Gracias, Ruslan Gulmagomedov!
-   Se ha añadido un botón «Borrar todo» al cuadro de diálogo «Todos los
    documentos».
-   El verificador de actualizaciones ahora muestra las notas de la
    versión cuando hay una nueva versión disponible.
-   Se ha corregido la restauración de la ventana desde la bandeja del
    sistema.
-   Se han corregido las traducciones de los botones «Sí» y «No» en los
    cuadros de diálogo de confirmación.
-   Se ha corregido la carga de configuraciones al ejecutar el programa
    como administrador.
-   Se ha corregido el manejo de comentarios en documentos XML y HTML.
-   Se ha corregido el análisis de la tabla de contenidos en libros Epub
    2.
-   Se ha corregido la navegación al siguiente elemento con la misma
    letra en la tabla de contenidos.
-   Se ha corregido el error por el que el cuadro de diálogo de búsqueda
    no se ocultaba correctamente al utilizar los botones «Siguiente» y
    «Anterior».
-   Se ha corregido el problema por el que, en ocasiones, las tablas de
    contenido de los libros EPUB llevaban al elemento equivocado.
-   Se han corregido varios problemas relacionados con el manejo de los
    espacios en blanco en las etiquetas XML, HTML y pre .
-   Se ha corregido un error de «off-by-one» en la navegación por
    enlaces.
-   Se ha corregido el problema por el que algunos libros presentaban
    espacios en blanco al final de las líneas.
-   Se han corregido varios problemas del analizador sintáctico.
-   Los elementos del menú relacionados con los marcadores, así como la
    lista de elementos, ahora se desactivan correctamente cuando no hay
    ningún documento abierto.
-   Se ha mejorado el manejo de listas en diversos formatos de
    documento.
-   Se ha mejorado el flujo de trabajo de traducción para los
    colaboradores.
-   Se han realizado numerosas refactorizaciones internas, trasladando
    la mayor parte de la lógica de negocio de la aplicación de C++ a
    Rust para mejorar el rendimiento y la facilidad de mantenimiento.

### Versión 0.6.1 {#version-0.6.1}

-   ¡Se ha añadido compatibilidad con archivos PDF protegidos con
    contraseña!
-   Se ha añadido una función muy básica para ir a la posición anterior
    o siguiente. Si pulsas Intro en un enlace interno y el cursor se
    desplaza, esa posición ahora se recordará y podrás navegar hasta
    ella con las teclas Alt + flechas izquierda/derecha.
-   ¡Se ha añadido una lista de elementos! Actualmente solo muestra un
    árbol con todos los encabezados del documento o una lista de
    enlaces, pero hay planes para ampliarla en el futuro.
-   Se ha añadido una opción para iniciar Paperback en modo maximizado
    de forma predeterminada.
-   Se ha corregido un error por el que los enlaces de algunos
    documentos EPUB no funcionaban correctamente.
-   Se ha corregido el análisis de las tablas de contenido de los
    archivos EPUB que contienen rutas relativas.
-   Se ha corregido el error por el que algunos documentos EPUB no
    mostraban el título ni el autor.
-   Se ha corregido un error por el que los títulos de algunos capítulos
    de EPUB no se mostraban correctamente en el cuadro de diálogo de la
    tabla de contenidos.
-   Se ha corregido el problema por el que no se podía utilizar la barra
    espaciadora para activar los botones «Aceptar»/«Cancelar» en el
    cuadro de diálogo de la tabla de contenidos.
-   Se ha mejorado el manejo de los encabezados en los documentos de
    Word.
-   Ahora recibirás un aviso de voz si la lista de documentos recientes
    está vacía al intentar abrir el cuadro de diálogo.

### Versión 0.6.0 {#version-0.6.0}

-   Se ha añadido al cuadro de diálogo de opciones una nueva opción para
    mostrar el menú «Ir a» en un formato mucho más compacto, activada
    por defecto.
-   Se ha añadido una opción para que la navegación por elementos
    estructurales se ajuste a la línea.
-   Se ha añadido una opción al menú «Herramientas» para abrir la
    carpeta que contiene el documento actualmente seleccionado.
-   Se ha añadido un sistema de actualización bastante sencillo, pero
    muy eficaz.
-   Se ha añadido una función básica de temporizador de suspensión,
    accesible con Ctrl+Mayús+S.
-   ¡Se ha añadido compatibilidad con el análisis de libros electrónicos
    en formato FB2!
-   ¡Se ha añadido compatibilidad con el análisis de presentaciones
    OpenDocument!
-   ¡Se ha añadido compatibilidad con el análisis de archivos de texto
    OpenDocument!
-   Ahora se pueden crear marcadores para marcar una línea completa o
    para marcar solo un texto específico. Si no tienes ninguna selección
    activa al colocar un marcador, el comportamiento es como en las
    versiones anteriores a la 0.6, y se marcará la línea completa. Sin
    embargo, si seleccionas algo de texto, solo ese texto se incluirá en
    el marcador.
-   ¡Ahora se pueden adjuntar notas de texto opcionales a los
    marcadores! Navega entre los marcadores que contienen notas con N y
    Mayús+N, o abre el cuadro de diálogo de marcadores con todos los
    marcadores, solo las notas o solo los que no sean notas
    seleccionados mediante teclas de acceso rápido específicas.
-   Los marcadores del cuadro de diálogo de marcadores ya no tendrán el
    molesto prefijo «marcador x».
-   Los libros en formato ePub que contengan contenido HTML que simule
    ser XML ahora se gestionarán correctamente.
-   Se ha corregido la carga de documentos Markdown de gran tamaño.
-   Se ha corregido el error por el que al pulsar la barra espaciadora
    en la vista de árbol de la tabla de contenidos se activaba el botón
    «Aceptar».
-   Se ha corregido el manejo de los espacios en blanco al principio de
    las etiquetas «pre» tanto en documentos HTML como en XHTML.
-   Se ha corregido el error por el que, en ocasiones, el control de
    texto no recuperaba el foco al volver a la ventana de Paperback.
-   Se ha corregido un error por el que el campo de texto del cuadro de
    diálogo «Ir al porcentaje» no actualizaba el valor del control
    deslizante.
-   Se ha corregido la representación de los identificadores HTML
    personalizados en documentos Markdown.
-   El código HTML dentro de los bloques de código Markdown ahora se
    mostrará correctamente.
-   Si se carga un libro con un parámetro de línea de comandos mientras
    hay una instancia de Paperback en ejecución, ya no aparecerá un
    error si la carga del documento tarda más de 5 segundos.
-   Si se ejecuta Paperback como administrador, la configuración ahora
    se cargará y guardará correctamente.
-   Ahora es posible eliminar un marcador directamente desde el cuadro
    de diálogo de marcadores.
-   Ahora es posible importar y exportar tus marcadores y la posición de
    lectura de un documento concreto. El archivo generado recibe el
    nombre del archivo con la extensión .paperback. Si se encuentra un
    archivo de este tipo en el mismo directorio que el archivo que se
    está cargando, se cargará automáticamente. De lo contrario, puedes
    importarlos manualmente mediante una opción del De lo contrario,
    puedes importarlos manualmente mediante una opción del menú
    «Herramientas».
-   ¡Ahora se admiten plenamente los enlaces dentro de los documentos!
    Utiliza k y shift+k para avanzar y retroceder por ellos, y pulsa
    Intro para abrir o activar uno.
-   Se han realizado numerosas refactorizaciones internas, lo que hace
    que la aplicación sea más rápida y el binario más pequeño.
-   El contenido Markdown ahora se preprocesa para que cumpla con el
    estándar CommonMark antes de su visualización.
-   ¡Ahora se admite plenamente la navegación por listas y sus
    elementos! Utiliza L y Mayús+L para desplazarte por las propias
    listas, e I y Mayús+I para desplazarte por los elementos de la
    lista.
-   Ahora, la tecla «Suprimir» del teclado numérico sirve para eliminar
    documentos de la barra de pestañas, además de la función habitual de
    «Suprimir».
-   ¡Ahora Paperback se puede minimizar opcionalmente en la bandeja del
    sistema! Esta opción está desactivada por defecto, pero al
    activarla, la opción de minimizar del menú del sistema colocará
    Paperback en la bandeja, desde donde se podrá restaurar haciendo
    clic en el icono que aparece.
-   ¡Paperback ya es totalmente traducible! La lista de idiomas que
    admite es bastante reducida por el momento, ¡pero no deja de crecer!
-   ¡Paperback ya tiene una página web oficial, en
    [paperback.dev](https://paperback.dev)!
-   Los documentos PPTX mostrarán ahora una tabla de contenidos básica
    que incluye todas las diapositivas.
-   Ahora se mostrará la ruta completa del documento abierto en el
    cuadro de diálogo de información del documento.
-   El instalador incluye ahora una opción para ver el archivo «Léeme»
    en tu navegador tras la instalación.
-   ¡La lista de documentos recientes se ha ampliado considerablemente!
    En lugar de mostrarte simplemente los últimos 10 documentos que has
    abierto, ahora te mostrará un número personalizable, y podrás
    acceder al resto de los documentos que hayas abierto alguna vez a
    través de un pequeño cuadro de diálogo.
-   Se han introducido varias pequeñas mejoras en los analizadores en
    general, entre las que se incluyen la inserción de una línea en
    blanco entre diapositivas en presentaciones PPTX, la corrección del
    manejo de los saltos de línea dentro de los párrafos en documentos
    de Word y la adición de viñetas a los elementos de las listas.

### Versión 0.5.0 {#version-0.5.0}

-   ¡Se ha añadido compatibilidad con documentos de Microsoft Word!
-   ¡Se ha añadido compatibilidad con presentaciones de PowerPoint!
-   Se ha corregido el error por el que algunos elementos del menú no se
    desactivaban cuando no había ningún documento abierto.
-   Se ha corregido la orientación del control deslizante de porcentaje.
-   Se ha corregido la tabla de contenidos en libros EPUB con rutas de
    archivo codificadas en URL y/o identificadores de fragmentos.
-   Se ha corregido la eliminación de espacios en blanco de los
    encabezados XHTML de forma anómala.
-   Se ha corregido el manejo de los espacios en blanco dentro de las
    etiquetas \`pre\` anidadas en documentos HTML.
-   ¡Los documentos HTML y Markdown ahora admiten la función de índice !
    Cuando cargues un documento HTML/Markdown, Paperback generará su
    propio índice a partir de la estructura de los encabezados de tu
    documento, y te lo mostrará en el cuadro de diálogo de Ctrl+T.
-   Los documentos HTML tendrán ahora el título tal y como se ha
    establecido en la etiqueta «title», si existe. De lo contrario,
    seguirán utilizando el nombre del archivo sin la extensión.
-   Se ha pasado de UniversalSpeech a utilizar una región activa para
    generar la lectura en voz alta. Esto significa que ya no se incluyen
    DLL de lectores de pantalla junto con el programa, y ahora se
    admitirán más lectores de pantalla, como Microsoft Narrator.
-   Se han cambiado las bibliotecas ZIP para permitir abrir una gama más
    amplia de libros EPUB.
-   Se ha rediseñado por completo el cuadro de diálogo que pregunta si
    deseas abrir el documento como texto sin formato, y ahora te permite
    abrirlo como texto sin formato, HTML o Markdown.
-   El cuadro de diálogo «Ir al porcentaje» incluye ahora un campo de
    texto que permite introducir manualmente el porcentaje al que se
    desea saltar.
-   El analizador de HTML ahora reconocerá dd, dt y dl como elementos de
    lista.
-   La tabla de contenidos de los libros en formato ePub se conservará
    de nuevo exactamente.
-   Ahora se tiene en cuenta el espacio no separable Unicode al eliminar
    las líneas en blanco.
-   Ya no se te preguntará cómo quieres abrir un archivo no reconocido
    cada vez que lo cargues, sino solo la primera vez.

### Versión 0.4.1 {#version-0.4.1}

-   Se ha añadido un icono opcional en el menú de inicio al instalador.
-   La tabla de contenidos debería aparecer ahora más clara en algunos
    casos; por ejemplo, si tienes un elemento secundario y uno principal
    con el mismo texto en la misma posición, ahora solo verás el
    elemento principal.
-   Se ha corregido la tabla de contenidos en determinados documentos
    CHM.
-   Se ha corregido la tabla de contenidos en libros Epub 3 que
    contienen rutas absolutas en su interior.
-   Los documentos CHM ahora deberían mostrar el título tal y como se ha
    establecido en el archivo de metadatos .

### Versión 0.4.0 {#version-0.4.0}

-   ¡Se ha añadido compatibilidad con archivos CHM!
-   ¡Se ha añadido compatibilidad con marcadores! Puedes tener tantos
    marcadores en tantos documentos como desees. Puedes avanzar y
    retroceder entre ellos con las teclas «b» y «Shift+b», crear uno con
    «Control+Shift+b» y abrir un cuadro de diálogo para saltar a un
    marcador específico con «Control+b».
-   ¡Se ha añadido un instalador junto con el archivo ZIP portátil! El
    instalador instalará Paperback en tu directorio «Archivos de
    programa» y configurará automáticamente las asociaciones de archivos
    por ti.
-   Los archivos de texto con BOM ahora deberían descodificarse
    correctamente, y el BOM ya no se mostrará al principio del texto.
-   Se ha añadido mucha más información a la barra de estado. Ahora te
    mostrará la línea actual, el carácter y el porcentaje leído.
-   Los comentarios HTML, así como el contenido de las etiquetas de
    script y style, ya no se mostrarán en la salida de texto.
-   Si se pasa una ruta relativa a Paperback en la línea de comandos,
    ahora la resolverá correctamente.
-   El desplazamiento porcentual se gestiona ahora mediante su propio
    cuadro de diálogo basado en un control deslizante, al que se accede
    con Control+Mayús+G.
-   Los documentos sin título ni autor conocidos tendrán ahora siempre
    un valor por defecto.
-   La lógica de guardado de la posición es ahora mucho más inteligente
    y solo debería escribir en el disco cuando sea absolutamente
    necesario.
-   El documento en el que tenías el foco al cerrar Paperback ahora se
    recuerda tras reiniciar la aplicación.
-   La entrada en los cuadros de diálogo «Ir a la línea» e «Ir a la
    página» ahora se depura de forma más estricta.
-   Se ha corregido la navegación por la tabla de contenidos en libros
    EPUB 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0 {#version-0.3.0}

-   Se ha corregido la tabla de contenidos en libros EPUB con
    manifiestos codificados en URL.
-   Se ha corregido la navegación por encabezados en documentos HTML que
    contienen caracteres Unicode multibyte.
-   Se ha corregido el elevado consumo de CPU en documentos con títulos
    largos debido a una regresión en wxWidgets.
-   Se ha corregido la carga de archivos de texto UTF-8.
-   Se han corregido los elementos anidados de la tabla de contenidos en
    libros EPUB que situaban el cursor en una posición incorrecta.
-   Se ha corregido un fallo al salir de la aplicación en determinados
    casos.
-   Se ha añadido una casilla de selección en el cuadro de diálogo de
    opciones para activar o desactivar el ajuste de línea.
-   Ahora es posible hacer donaciones para el desarrollo de Paperback,
    ya sea a través de la nueva opción «Donar» del menú de ayuda o
    mediante el enlace «Patrocina este proyecto» situado en la parte
    inferior de la página principal del repositorio de GitHub.
-   Los documentos Markdown ahora siempre tendrán un título, y Paperback
    debería poder cargar prácticamente cualquier archivo Markdown.
-   Los documentos PDF ahora siempre tendrán un título, incluso si
    faltan los metadatos. Se han cambiado las bibliotecas de PDF por las
    que se utilizan en Chromium, lo que ha dado lugar a un
-   Se ha cambiado la biblioteca de PDF por la que se utiliza en
    Chromium, lo que ha dado lugar a un análisis de PDF mucho más fiable
    en todos los aspectos.
-   Ahora solo puede haber una instancia de Paperback en ejecución a la
    vez. Si ejecutas paperback.exe con un nombre de archivo mientras ya
    está en ejecución, ese documento se abrirá en la instancia que ya
    está en ejecución.
-   Ahora puedes pulsar la tecla «Suprimir» sobre un documento en el
    control de pestañas para cerrarlo. Versión 0.2.1

### Versión 0.2.1 {#version-0.2.1}

-   Se ha añadido el número total de páginas a la etiqueta de página en
    el cuadro de diálogo «Ir a la página».
-   Ahora se puede navegar con la tecla Tab desde el contenido del
    documento hasta la lista de documentos abiertos.
-   Se ha corregido el error por el que, en ocasiones, al pulsar las
    teclas de encabezado se abrían documentos recientes si había
    suficientes.
-   Paperback eliminará ahora los guiones no obligatorios del texto
    resultante.
-   Se ha corregido un error por el que la navegación por encabezados a
    veces te llevaba al carácter equivocado.

### Versión 0.2.0 {#version-0.2.0}

-   ¡Se ha añadido compatibilidad con documentos Markdown!
-   Se ha añadido compatibilidad con documentos PDF, incluida la
    posibilidad de navegar entre páginas.
-   Se han añadido atajos de teclado para navegar por los encabezados en
    contenido HTML, incluidos libros en formato ePub y documentos
    Markdown. Estos atajos se han diseñado para funcionar de forma
    similar a un lector de pantalla.
-   Se ha corregido la carga de libros EPUB con nombres de archivo
    codificados como URL en sus manifiestos.
-   Se ha corregido la carga de libros EPUB 3 con XHTML incrustado en su
    interior.
-   Ahora se reproduce un mensaje si el documento no admite una tabla de
    contenidos o secciones, en lugar de desactivar las opciones del
    menú.
-   ¡Se ha añadido un menú de documentos recientes! Actualmente almacena
    los últimos 10 documentos abiertos, y al pulsar Intro sobre uno de
    ellos, este se abrirá para su lectura.
-   Se ha reescrito por completo el cuadro de diálogo «Buscar», lo que
    lo hace mucho más sencillo de usar, además de añadir un historial de
    tus últimas 25 búsquedas y compatibilidad con expresiones regulares.
-   Ahora se recuerdan los documentos abiertos anteriormente incluso
    tras reiniciar la aplicación. Esto se puede configurar a través de
    la nueva opción del menú «Herramientas».
-   Se ha añadido la combinación Mayús+F1 para abrir el archivo «Léeme»
    directamente en la propia aplicación Paperback.

### Versión 0.1.0 {#version-0.1.0}

-   Lanzamiento inicial.
