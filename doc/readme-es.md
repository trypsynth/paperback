<!-- machine-translated from doc/readme.md (source-hash: 13c58fb50049f608); please review and edit as needed -->

# Paperback - versión 0.9.1

## Introducción

Paperback es un lector ligero, rápido y accesible de libros electrónicos y documentos para todos, desde lectores ocasionales hasta usuarios avanzados. Está diseñado para la accesibilidad con lectores de pantalla, la velocidad y una experiencia libre de innecesarios.

## Requisitos del sistema

Paperback se ejecuta actualmente en Windows 10/11 y todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba pública previstas poco después del lanzamiento de escritorio 0.9.0, antes de un lanzamiento unificado 1.0 que cubra las cuatro plataformas.

## Características

* Completamente independiente, sin requerir la instalación de ningún software en tu ordenador para empezar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz con pestañas simple, que te permite abrir tantos documentos como desees uno al lado del otro.
* Guarda tu posición exacta de lectura en cada documento que abres.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura al siguiente inicio.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de documentos.
* Incluye un robusto diálogo de búsqueda, con características como historial y soporte de expresiones regulares.
* Puede ejecutarse completamente de forma portátil o instalarse con las asociaciones de archivos configuradas automáticamente.
* Soporta una amplia variedad de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, hay un problema conocido para los usuarios de JAWS.

### JAWS y pantallas Braille

Si usas JAWS con una pantalla Braille, puedes encontrar que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de tu pantalla. El comando de lectura del párrafo actual también se ve afectado. Este es un error en el manejo de JAWS del control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tardó bastante tiempo en encontrar una solución dado el entusiasmo de Vispero por responder a problemas con software de código abierto.

La solución alternativa, finalmente encontrada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla se mantendrá en el párrafo activo en lugar de avanzar. Con ambas configuraciones en su lugar, el desplazamiento debería funcionar correctamente.

## Tipos de archivo actualmente soportados

Paperback soporta los siguientes formatos y extensiones:

* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Libros electrónicos FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto sin formato y de registro (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para uso prioritario con teclado. Aquí están los atajos actuales.

Los atajos que aparecen a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left/Right` ya están asignados por otras convenciones del sistema u otras aplicaciones en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabre el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS está en el menú de la aplicación).

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
* `\`: Saltar a tu marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Saltar a todos los marcadores y notas.
* `Ctrl+Alt+B`: Saltar solo a marcadores.
* `Ctrl+Alt+M`: Saltar solo a notas.
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

* `Ctrl+W` (macOS: `RawCtrl+W`, es decir, la tecla Control física en lugar de Cmd): Mostrar recuento de palabras del documento actual.
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
* `Ctrl+Shift+N`: Agregar o editar nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar ajuste de palabras.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Avanzar búsqueda de narración de audio.
* `;`: Retroceder búsqueda de narración de audio.
* `Ctrl+'`: Aumentar la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuir la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Alternar temporizador de sueño.

### Menú Ayuda

* `Ctrl+F1`: Mostrar diálogo Acerca de.
* `F1`: Ver ayuda en tu navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Verificar actualizaciones.
* `Ctrl+D`: Abrir la página de donación en tu navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestaña: Cerrar la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activar enlace en el cursor, o abrir una vista de tabla cuando estés en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas admitidos

Paperback se traduce a muchos idiomas diferentes, y se agregan más continuamente. A continuación se incluye una lista completa.

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
* Vietnamita

## Créditos
### Desarrollo
* Quin Gillespie: desarrollador principal y fundador del proyecto.
* Aryan Choudhary: principal colaborador.

### Donaciones
Las siguientes personas han hecho donaciones de algún tipo al desarrollo de Paperback. Si haces una donación, tu nombre no se agregará automáticamente aquí, solo agrego personas que desean que su donación sea pública.

Nota: Considero un patrocinio público de GitHub como motivo para inclusión automática en esta lista.

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
* Cerrar Web View con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de él.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directamente a la línea encontrada, en lugar de hacerte esperar a que el lector de pantalla lea la ventana nuevamente mientras el foco vuelve al libro.
* Se corrigió el problema de que los EPUB con un bloque ZIP64 extraviado se negaran a abrir con "Invalid local file header".
* Se corrigió el problema de que los documentos largos volvieran al inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en Web View ahora te llevan a la sección a la que apuntan, en lugar de fallar con "File not found".
* El anuncio automático "Document reloaded" ya no interrumpe a tu lector de pantalla a mitad de frase, sino que espera a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora navega por sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificar actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la etiqueta completa del programa.
* Recuento de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcapáginas y notas ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrir y rastrear su línea de tiempo en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares desapareciendo de documentos RTF, juntando las palabras circundantes.
* Se corrigió el problema de que las imágenes RTF filtraran datos sin procesar en el documento como texto ilegible.
* Se corrigió el submenú Documentos recientes que mantenía entradas antiguas hasta que algo más sucediera para reconstruirlo.
* Los aceleradores de teclado están de vuelta en cada traducción, así que los menús de ruso tienen acceso de teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Opciones ha sido renombrado a Configuración, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda la posición, tamaño y estado maximizado de su ventana entre ejecuciones.
* Las formas plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora se pueden traducir.
* El título del documento ahora viene primero en la barra de título, por lo que los libros abiertos se pueden distinguir en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Añadido

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos compatibles de Paperback a HTML, Markdown o texto sin formato.
* Una opción para recargar documentos que han sido modificados por otros programas en el disco.
* Una opción Ver fuente para abrir la fuente de un documento en una pestaña nueva, útil para editar Markdown, por ejemplo.
* El texto del documento ahora se pagina, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor reporta cualquier cosa extraña encontrada con esto.

##### Compatibilidad de plataformas
* ¡Compatibilidad con Windows ARM64!
* ¡Compatibilidad nativa con macOS!
* Un toggle de pantalla completa.

##### Diálogo Todos los documentos
* Un botón de localización para localizar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y una barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y se seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de línea;
    * Espaciado de párrafo;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y un atajo de tecla posterior.
* Un toggle para determinar cómo deseas que se muestren las tablas, y unificó cómo se muestran las tablas en documentos.

##### Navegación
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo navegación en lectores de pantalla.
* El atajo de teclado de signo igual para anunciar tu porcentaje actual en un documento.

##### Marcapáginas
* Marcapáginas temporales: puedes tener uno por documento, y persisten. Usa barra inclinada para establecer uno e barra invertida para saltar a él.

##### Recuento de palabras
* Tiempo de lectura estimado en el diálogo de recuento de palabras, así como la capacidad de establecer tu velocidad de lectura para hacer que esta métrica sea realmente útil.
* Si hay una selección activa cuando abres el diálogo de recuento de palabras, ahora se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento de menú exportar para permitir exportar a HTML y Markdown, además de texto sin formato.

##### Actualizador
* Un botón de cancelación al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido modificado.

##### Web View
* Web View ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente soportando tanto DAISY de audio (incluyendo DAISY de audio + texto) como archivos ZIP de audio.
* Atajos de teclado y elementos de menú para reproducir/pausar narración, buscar adelante y atrás, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura con la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si buscar más allá del final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos de PowerPoint ahora soportan tablas.

#### Corregido

##### General
* Los documentos codificados en conjuntos de caracteres CJK heredados, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de como un montón de mojibake.
* "Reopen last closed" intentando reabrir el readme incluido.
* Tu pestaña seleccionada no se enfocaba correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzosamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el readme ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escala correctamente en pantallas de alta DPI.
* El menú ahora se actualiza correctamente, y el foco se mueve al control de texto, cuando se abre ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se lee al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes reduciendo a la mitad el tamaño de las tablas de índice interno por carácter.

##### Diálogo Todos los documentos
* Escape no cerraba los diálogos de Información del documento y Todos los documentos.
* La barra de título no se actualizaba después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se agregará a tu lista de todos los documentos cuando se abra mediante Shift+F1.
* Eliminar documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se conserva después de eliminar un documento.

##### Navegación
* La navegación de página anunciaba texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocaban tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetaban la ventana del documento cargado en documentos grandes.

##### Marcapáginas
* Los sonidos de marcapáginas/notas ahora deberían reproducirse correctamente en exclusiva cuando navegues sobre una palabra que contenga uno.

##### Legibilidad
* Aplicar ajuste de línea te disparaba al inicio de tu documento.

##### Web View
* El diálogo Web View no era redimensionable y aparecía con un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en Web View integrado.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código de markdown en las notas de la versión.

##### Libros DAISY
* Los libros DAISY mostraban información incorrecta en la barra de estado.
* Cargar libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Análisis de documentos RTF con caracteres no latinos.
* Grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes de posición de archivo en libros Mobi dividen etiquetas HTML e insertaban basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis de AZW3 enormemente mejorado.

##### Documentos Word
* Los documentos de Word con nombres de estilo específicos de la configuración regional no renderizaban sus encabezados correctamente.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no producían saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora se repliega a la extracción de texto sin formato para PDF etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcapáginas ya no causarán el cierre de Paperback al abrirse.

### Versión 0.8.5
* Se agregó compatibilidad de página a libros epub.
* Se agregó soporte para documentos de Microsoft Office cifrados. Actualmente se soportan Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos heredados de Microsoft Word!
* ¡Se agregó soporte para presentaciones heredadas de Powerpoint!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo `ctrl+q` para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió que ir a página estuviera desactivado por 1.
* Se corrigió que la tecla de escape no funcionara para cerrar el diálogo abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF solo de imagen una vez más se detectan y te alertan de su existencia.
* Ahora es posible navegar por imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte XML DAISY, ya que ya no es necesario.
* Se cambió de nuevo a la navegación de primera letra Win32 nativa en el árbol de tabla de contenidos.
* El diálogo de error al cargar ahora muestra mensajes de error más detallados.
* Web View ahora se abrirá mucho más rápido y suave.

### Versión 0.8.2
* ¡Se agregó compatibilidad de página a documentos RTF!
* Se corrigió un error donde abrir Web View en epub que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no ponía un espacio entre palabras en casos raros.
* Se corrigieron los párrafos que se dividían en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las pestañas RTF y saltos de línea ahora se renderizan exactamente como aparecen en el documento.
* Se cambió de nuevo a la biblioteca pdfium probada y comprobada para analizar PDF, haciendo que la representación de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabrir el último documento cerrado.
* El diálogo Todos los documentos ahora soporta seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigió que las rutas de archivo que contienen caracteres no ASCII (como el bosnio š, č, ć, ž) se corrompieran al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió que el texto PDF se leyera en el orden incorrecto y espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió la carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional para alcanzar un marcapáginas o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó soporte para documentos RTF!
* Se agregó soporte para documentos XML DAISY.
* ¡Se agregó soporte para archivos de texto de documento abierto plano!
* ¡Se agregó soporte para presentaciones de documento abierto plano!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento superior a 300 caracteres ahora agregará automáticamente al historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigió que los documentos Markdown mostraran texto sin procesar en lugar de HTML renderizado en Web View.
* Se corrigió que las tablas no se renderizaran correctamente en archivos Markdown.
* Los PDF solo de imagen ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrustó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilitar su uso y navegación.
* Se cambió a Hayro para analizar PDF, lo que lleva a más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó compatibilidad de tabla para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Enter para ver una en un renderizador web.
* ¡Se agregó una característica básica de representación web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formato complejo o ejemplos de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Limpiar todo al diálogo Todos los documentos.
* El verificador de actualizaciones ahora muestra notas de la versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigió la traducción de botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones cuando se ejecuta como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que los TOC de epub ocasionalmente te llevaran al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió el error de desactivación por uno en la navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco finales en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos relacionados con marcapáginas en el menú, así como la lista de elementos, ahora se deshabilitan correctamente cuando no hay documento abierto.
* Se mejoró el manejo de listas en varios formatos de documentos.
* Se mejoró el flujo de trabajo de traducción para colaboradores.
* Muchas refactorizaciones internas, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte para PDF protegidas con contraseña!
* Se agregó una característica muy básica de ir a posición anterior/siguiente. Si presionas enter en un enlace interno y mueve el cursor, esa posición ahora será recordada, y se puede navegar con flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado de forma predeterminada.
* Se corrigió que los enlaces en algunos documentos Epub no funcionaran correctamente.
* Se corrigió el análisis de Epub TOC que contiene rutas relativas.
* Se corrigió que algunos documentos epub no mostraran título o autor.
* Se corrigió que los títulos de algunos capítulos de epub no aparecieran correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos de Word.
* Ahora obtendrás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes traer el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú ir en una forma mucho más compacta al diálogo de opciones, activada de forma predeterminada.
* Se agregó una opción para que la navegación por elementos estructurales se envuelva.
* Se agregó una opción al menú de herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* ¡Se agregó un sistema de actualización bastante simple, pero muy efectivo!
* ¡Se agregó una característica básica de temporizador de sueño, accesible con Ctrl+Shift+S!
* ¡Se agregó soporte para analizar libros de FB2!
* ¡Se agregó soporte para analizar presentaciones de OpenDocument!
* ¡Se agregó soporte para analizar archivos de texto de OpenDocument!
* Los marcapáginas ahora se pueden hacer para marcar una línea completa o para marcar solo texto especificado. Si no tienes ninguna selección activa cuando colocas un marcapáginas, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas texto, solo ese texto se incluirá en el marcapáginas.
* ¡Los marcapáginas ahora pueden tener notas de texto opcionales adjuntas! Navega entre marcapáginas que contienen notas con N y Shift+N, o abre el diálogo de marcapáginas con todos los marcapáginas, solo notas o solo no notas seleccionados con teclas de acceso rápido específicas.
* Los marcapáginas en el diálogo de marcapáginas ya no tendrán un prefijo molesto "bookmark x".
* Los libros Epub que contienen contenido HTML pretendiendo ser XML ahora se manejarán correctamente.
* Se corrigió la carga de documentos Markdown grandes.
* Se corrigió presionar espacio en la vista de árbol de tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al comienzo de etiquetas pre en documentos HTML y XHTML.
* Se corrigió que el control de texto no recuperara el foco a veces al volver a la ventana de Paperback.
* Se corrigió que el campo de texto en el diálogo ir a porcentaje no actualizara el valor del control deslizante.
* Se corrigió la representación de IDs HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderizará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras una instancia existente de Paperback se está ejecutando, ya no obtendrás un error si cargar tu documento tarda más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcapáginas directamente desde el diálogo de marcapáginas.
* Ahora es posible importar y exportar tus marcapáginas y posición de lectura para un documento particular. El archivo generado se nombra según el archivo con una extensión .paperback. Si se encuentra tal archivo en el mismo directorio que un archivo mientras lo cargas, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente compatibles! Usa k y shift+k para moverte hacia adelante y hacia atrás a través de ellos, y presiona enter para abrir/activar uno.
* Muchas refactorizaciones internas, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido de Markdown ahora se preprocesa para ser compatible con CommonMark antes de renderizar.
* ¡La navegación por listas y sus elementos ahora es totalmente compatible! Usa L y Shift+L para ir por las listas mismas, e I y Shift+I para recorrer elementos de lista.
* La tecla Supr del teclado numérico ahora funciona para eliminar documentos de la barra de pestañas además del Supr normal.
* ¡Paperback ahora puede minimizarse opcionalmente a la bandeja del sistema! Esta opción está desactivada de forma predeterminada, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que soporta es actualmente bastante pequeña, pero está creciendo constantemente.
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el readme en tu navegador después de la instalación.
* ¡La lista de documentos recientes se ha expandido drásticamente! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora te mostrará un número personalizable, siendo el resto de los documentos que has abierto alguna vez accesibles a través de un pequeño diálogo.
* Varias pequeñas mejoras en los analizadores en general, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de nuevas líneas dentro de párrafos en documentos de Word, y agregar puntos de viñeta a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte para documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos del menú que no se deshabilitaban sin documentos abiertos.
* Se corrigió la orientación del control deslizante ir a porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas en URL y/o IDs de fragmento.
* Se corrigió el manejo de espacios en blanco en encabezados XHTML de formas raras.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora soportan la característica de tabla de contenidos! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos a partir de la estructura de los encabezados en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título como se establece en la etiqueta de título, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar discurso. Esto significa que no se envían DLL de lector de pantalla junto con el programa, y más lectores de pantalla ahora serán soportados, como Microsoft Narrator.
* Se cambió la biblioteca zip para permitir abrir una gama más amplia de libros epub.
* El diálogo que te pregunta si quieres abrir tu documento como texto sin formato ha sido completamente reescrito, y ahora te permite abrir tu documento como texto sin formato, HTML o Markdown.
* El diálogo ir a porcentaje ahora incluye un campo de texto que te permite ingresar manualmente un porcentaje para saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub ahora será preservada exactamente.
* El espacio sin salto unicode ahora se considera al eliminar líneas en blanco.
* Ya no se te preguntará cómo quieres abrir un archivo no reconocido cada vez que lo cargues, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono de menú de inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo si tienes un elemento hijo y padre con el mismo texto en la misma posición ahora solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas.
* Los documentos CHM ahora deberían mostrar su título como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcapáginas! Puedes tener tantos marcapáginas como desees en tantos documentos como desees. Puedes saltar hacia adelante y hacia atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y abrir un diálogo para saltar a un marcapáginas específico con control+b.
* ¡Se agregó un instalador junto con el archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente asociaciones de archivo por ti.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al principio del texto tampoco.
* Se agregó información mucho más completa a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de las etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora es manejado por su propio diálogo basado en deslizador, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán un valor predeterminado.
* La lógica de guardar posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento que enfocaste cuando cerraste Paperback ahora se recuerda en reinicios de aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería sanitizarse más estrictamente.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados en URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el alto uso de CPU en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigió que los elementos anidados de TOC en libros Epub pusieran el cursor en la posición incorrecta.
* Se corrigió un cierre de aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace patrocina este proyecto al final de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería ser capaz de cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si los metadatos faltan.
* Se cambió de biblioteca de PDF a la utilizada en Chromium, lo que lleva a análisis de PDF mucho más confiables en general.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya ejecutándose.
* Ahora puedes presionar eliminar en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Se permite tabular desde el contenido del documento a tu lista de documentos abiertos.
* Se corrigió que los atajos de encabezado a veces abrieran documentos recientes si tenías suficientes de ellos.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió que la navegación de encabezados a veces te pusiera en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de teclas para navegar por encabezados en contenido HTML, incluyendo libros epub y documentos markdown. Estas pulsaciones de teclas fueron diseñadas para funcionar de manera similar a un lector de pantalla.
* Se corrigió la carga de epub con nombres de archivo codificados en URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro de ellos.
* Ahora se habla un mensaje si el documento no soporta una tabla de contenidos o secciones, en lugar de deshabilitar los elementos del menú.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar enter en uno lo abrirá para lectura.
* ¡Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras también se agregaba un historial de tus últimas 25 búsquedas y soporte para expresiones regulares!
* Los documentos abiertos anteriormente ahora se recuerdan en reinicios de aplicación. Esto es configurable a través del nuevo elemento opciones en el menú de herramientas.
* Se agregó shift+f1 para abrir el readme directamente en Paperback.

### Versión 0.1.0
* Versión inicial.
