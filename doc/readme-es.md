<!-- machine-translated from doc/readme.md (source-hash: efe922e94821c70e); please review and edit as needed -->

# Paperback - versión 0.9.2

## Introducción

Paperback es un lector de ebooks y documentos ligero, rápido y accesible para todos, desde lectores ocasionales hasta usuarios expertos. Está diseñado para ser accesible con lectores de pantalla, ofrecer velocidades rápidas y una experiencia sin bloatware.

## Requisitos del sistema

Paperback actualmente se ejecuta en Windows 10/11 y todas las versiones modernas de ARM macOS. Las aplicaciones nativas para iOS y Android están en desarrollo activo, con compilaciones de prueba públicas previstas poco después del lanzamiento de la versión 0.9.0 de escritorio, antes de un lanzamiento unificado 1.0 que cubrirá las cuatro plataformas.

## Características

* Completamente independiente, no requiere que instales ningún software en tu ordenador para comenzar a leer.
* Increíblemente rápido, incluso en hardware antiguo.
* Interfaz simple con pestañas, que te permite abrir tantos documentos como desees lado a lado.
* Guarda tu posición exacta de lectura en cada documento que abras.
* Opcionalmente recuerda qué documentos tenías abiertos cuando cerraste el programa y los restaura en el siguiente inicio.
* Incluye funcionalidad de navegación similar a la que se encuentra en el modo de navegación web de muchos lectores de pantalla para navegar rápida y fácilmente a través de documentos.
* Incluye un diálogo de búsqueda robusto, con características como historial y compatibilidad con expresiones regulares.
* Puede ejecutarse de forma completamente portátil o instalarse con asociaciones de archivo configuradas automáticamente.
* Admite una amplia variedad de formatos de archivo comunes.

## Compatibilidad con lectores de pantalla

Paperback funciona bien con todos los lectores de pantalla principales. Sin embargo, existe un problema conocido para los usuarios de JAWS.

### JAWS y pantallas braille

Si utilizas JAWS con una pantalla braille, es posible que encuentres que los párrafos largos se truncan al desplazarse hacia adelante con las teclas de navegación de tu pantalla. El comando de lectura de párrafo actual también se ve afectado. Este es un error en la forma en que JAWS maneja el control de texto RICHEDIT50W, no algo en Paperback en sí, y uno que tomó bastante tiempo encontrar una solución dado el entusiasmo de Vispero por responder a problemas con software de código abierto.

La solución, finalmente encontrada a través del grupo de discusión de JAWS después de meses de espera, es editar `paperback.jcf` y establecer "Braille Presentation and Panning" en "Always use DOM if available". También querrás habilitar "Pan Text by Paragraph", de lo contrario tu pantalla permanecerá en el párrafo activo en lugar de avanzar. Con ambos ajustes en su lugar, el desplazamiento debe funcionar correctamente.

## Tipos de archivo actualmente compatibles

Paperback admite los siguientes formatos y extensiones:

* Archivos de ayuda CHM (`.chm`)
* Libros DAISY (`.opf`, `.zip`)
* Libros EPUB (`.epub`)
* Ebooks FB2 (`.fb2`)
* Documentos HTML (`.htm`, `.html`, `.xhtml`)
* Documentos Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Documentos Microsoft Word (`.docx`, `.docm`, `.doc`)
* Libros MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Presentaciones OpenDocument (`.odp`, `.fodp`)
* Archivos de texto OpenDocument (`.odt`, `.fodt`)
* Documentos PDF (`.pdf`)
* Presentaciones PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Documentos RTF (`.rtf`)
* Archivos de texto plano y registros (`.txt`, `.log`)

## Atajos de teclado

Paperback está diseñado para uso prioritario con teclado. Aquí se encuentran los atajos actuales.

Los atajos que se muestran a continuación son para Windows. Donde macOS difiere, el equivalente se indica entre paréntesis — principalmente porque `Ctrl+G`, `Ctrl+W` y `Alt+Left`/`Right` ya están utilizados por otras convenciones del sistema u otra aplicación en esa plataforma.

### Menú Archivo

* `Ctrl+O`: Abrir un documento.
* `Ctrl+F4` (macOS: `Cmd+W`): Cerrar el documento actual.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Cerrar todos los documentos abiertos.
* `Ctrl+Shift+T`: Reabrir el último documento cerrado.
* `Ctrl+R`: Mostrar el diálogo "Todos los documentos" (desde Documentos recientes).
* `Ctrl+Q`: Salir (solo Windows; en macOS se encuentra en el menú de la aplicación).

### Menú Ir

* `Ctrl+F`: Mostrar el diálogo Buscar.
* `F3` (macOS: `Cmd+G`): Buscar siguiente.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Buscar anterior.
* `Ctrl+G` (macOS: `Cmd+L`): Ir a línea.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ir a porcentaje.
* `Ctrl+P`: Ir a página (cuando lo admita el documento actual).
* `=`: Anunciar su porcentaje de lectura actual.
* `Alt+Left` (macOS: `Cmd+[`): Retroceder en el historial de navegación.
* `Alt+Right` (macOS: `Cmd+]`): Avanzar en el historial de navegación.
* `[`: Sección anterior.
* `]`: Sección siguiente.
* `Shift+H`: Encabezado anterior.
* `H`: Encabezado siguiente.
* `Shift+1` a `Shift+6`: Encabezado anterior en nivel 1-6.
* `1` a `6`: Encabezado siguiente en nivel 1-6.
* `Shift+P`: Página anterior.
* `P`: Página siguiente.
* `Shift+B`: Marcador anterior.
* `B`: Marcador siguiente.
* `/`: Establecer su marcador temporal.
* `\`: Saltar a su marcador temporal.
* `Shift+N`: Nota anterior.
* `N`: Nota siguiente.
* `Ctrl+B`: Saltar a todos los marcadores y notas.
* `Ctrl+Alt+B`: Saltar solo a marcadores.
* `Ctrl+Alt+M`: Saltar solo a notas.
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
* `Ctrl+Shift+V`: Abrir contenido actual en Vista Web.
* `Ctrl+U`: Ver la fuente del documento en una pestaña nueva.
* `Ctrl+Shift+E`: Exportar datos del documento (`.paperback`).
* `Ctrl+Shift+I`: Importar datos del documento (`.paperback`).
* `Ctrl+E`: Exportar el documento actual a texto plano.
* `Ctrl+Shift+B`: Alternar marcador en la selección/cursor actual.
* `Ctrl+Shift+N`: Agregar o editar nota de marcador en la selección/cursor actual.
* `Ctrl+Alt+W`: Alternar ajuste de palabras.
* `Ctrl+Space`: Reproducir/pausar narración de audio.
* `'`: Avanzar la narración de audio.
* `;`: Retroceder la narración de audio.
* `Ctrl+'`: Aumentar la cantidad de búsqueda de audio.
* `Ctrl+;`: Disminuir la cantidad de búsqueda de audio.
* `F11` (macOS: `RawCtrl+Ctrl+F`, es decir, Control+Command+F): Alternar pantalla completa.
* `Ctrl+,`: Abrir opciones (macOS: Preferencias, en el menú de la aplicación).
* `Ctrl+Shift+S`: Alternar temporizador de reposo.

### Menú Ayuda

* `Ctrl+F1`: Mostrar diálogo Acerca de.
* `F1`: Ver ayuda en su navegador predeterminado.
* `Shift+F1`: Ver ayuda en Paperback.
* `Ctrl+Shift+U`: Buscar actualizaciones.
* `Ctrl+D`: Abrir la página de donación en su navegador predeterminado.

### Teclas adicionales de vista de documento

* `Delete` / `Numpad Delete` en el control de pestañas: Cerrar la pestaña de documento seleccionada.
* `Enter` o `Space` en el texto del documento: Activar enlace en el cursor, o abrir una vista de tabla cuando está en un marcador de tabla.
* `Shift+F10` o la tecla Menú/Aplicación en el texto del documento: Abrir el menú contextual.

## Idiomas admitidos

Paperback se traduce a muchos idiomas diferentes, y se siguen agregando más. A continuación se presenta una lista completa.

Para aprender cómo contribuir, consulte nuestra [Guía de traducción](translating.md).

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
Las siguientes personas han realizado donaciones de algún tamaño al desarrollo de Paperback. Si realiza una donación, su nombre no se agregará automáticamente aquí; solo agrego personas que desean que su donación sea pública.

Nota: considero que ser patrocinador público de GitHub es motivo para inclusión automática en esta lista.

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
* Los audiolibros ahora nombran el archivo mientras los recorres por sección.
* Los audiolibros ahora reportan su duración real, en lugar de pretender que cada archivo dura 24 horas.
* Cerrar Web View con Escape ya no muestra una alerta de depuración después de haber seguido un enlace dentro de él.
* Copiar después de Seleccionar todo ahora te da el documento completo, en lugar de solo la parte cargada actualmente.
* Buscar ahora va directo a la línea que encontró, en lugar de hacer que tu lector de pantalla lea la ventana nuevamente mientras el enfoque regresa al libro.
* Se corrigieron los EPUB que contienen un bloque ZIP64 extraño rechazando abrirse con "Invalid local file header".
* Se corrigieron los documentos largos que retrocedían a su inicio mientras un lector de pantalla los leía continuamente.
* Los enlaces en WebView ahora te llevan a la sección a la que apuntan, en lugar de fallar con "File not found".
* El anuncio automático "Documento recargado" ya no corta tu lector de pantalla a mitad de oración, esperando a que termine lo que estaba diciendo.
* La pestaña General del diálogo Configuración ahora tabula a través de sus opciones en el orden en que aparecen en pantalla, con el canal de actualización directamente después de la opción de verificación de actualizaciones.
* Windows ahora siempre mostrará "Paperback" en el menú Abrir con, en lugar de la línea de etiqueta completa del programa.
* Conteo de palabras e Información del documento ahora muestran cuántos archivos contiene un audiolibro y cuánto tiempo dura en total.

### Versión 0.9.1
* Los sonidos de marcador y nota ahora se reproducen en macOS.
* Los libros DAISY ahora reproducen su audio en macOS, en lugar de abrir y rastrear su línea de tiempo en silencio.
* Se corrigieron las comillas rizadas, guiones largos y caracteres similares desapareciendo de documentos RTF, uniendo las palabras circundantes al hacerlo.
* Se corrigieron las imágenes RTF filtrando sus datos sin procesar en el documento como texto desordenado.
* Se corrigió el submenú Documentos recientes manteniendo entradas obsoletas hasta que algo más sucediera para reconstruirlo.
* Los aceleradores de teclado están de vuelta en todas las traducciones, por lo que los menús de ruso tienen acceso de teclado nuevamente.
* Los documentos CHM grandes ahora se abren hasta siete veces más rápido.
* Los documentos abiertos ahora se registran con Windows, por lo que aparecen en la lista de saltos de la barra de tareas y en la lista reciente del menú Inicio.
* Options ha sido renombrado a Settings, coincidiendo con las aplicaciones móviles y, en macOS, la convención de la plataforma.
* Paperback ahora recuerda su posición de ventana, tamaño y estado maximizado entre ejecuciones.
* Los formularios plurales ahora se traducen, por lo que los mensajes que cuentan cosas se leen correctamente en idiomas que necesitan más de una forma.
* Seleccionar el ncc.html de un libro DAISY ahora abre el audiolibro completo en lugar de solo su texto.
* Los nombres de acciones del diálogo Personalizar atajos de teclado ahora pueden ser traducidos.
* El título del documento ahora viene primero en la barra de título, para que los libros abiertos se distingan en la barra de tareas y Alt+Tab.
* El diálogo de actualización ahora está traducido.

### Versión 0.9.0

#### Agregado

##### General
* Una herramienta CLI, llamada pb, para convertir rápidamente cualquiera de los formatos soportados por Paperback a HTML, Markdown o texto plano.
* Una opción para recargar documentos que han sido modificados por otros programas en disco.
* Una opción Ver origen para abrir el código fuente de un documento en una nueva pestaña, útil para editar Markdown, por ejemplo.
* El texto del documento ahora está paginado, lo que significa que puedes cargar libros con decenas de millones de palabras en solo un par de segundos. Por favor reporta cualquier rareza encontrada con esto.

##### Soporte de plataforma
* ¡Soporte para Windows ARM64!
* ¡Soporte nativo para macOS!
* Un toggle de pantalla completa.

##### Diálogo Todos los documentos
* Un botón ubicar para localizar libros faltantes que acaban de cambiar su ruta.
* Un filtro de estado y barra de estado, para que puedas filtrar por estado del documento y ver cuántos documentos se muestran y seleccionan.
* El atajo `Ctrl+Shift+A` para deseleccionar todos los documentos.

##### Opciones y legibilidad
* Una pestaña de legibilidad, con las siguientes opciones:
    * Ajuste de línea (movido desde general);
    * Renderizar tablas en línea (nuevo en esta versión, ver abajo);
    * Fuente;
    * Color de fondo;
    * Espaciado de líneas;
    * Espaciado de párrafos;
    * Espaciado de letras;
    * Alineación de texto.
* Un elemento de menú de ajuste de línea y su correspondiente tecla de acceso rápido.
* Un toggle para determinar cómo quieres que se muestren las tablas, y unificar cómo se muestran las tablas en todos los documentos.

##### Navegación
* Soporte para navegar por contenedor.
* Una opción para mover automáticamente el cursor al inicio de la línea al navegar entre líneas, similar al modo exploración en lectores de pantalla.
* El atajo de teclado de iguales para anunciar tu porcentaje actual a través de un documento.

##### Marcadores
* Marcadores temporales: puedes tener uno por documento, y persisten. Usa barra inclinada para establecer uno e barra invertida para saltar a él.

##### Conteo de palabras
* Tiempo de lectura estimado en el diálogo de conteo de palabras, así como la capacidad de establecer tu velocidad de lectura para hacer esta métrica realmente útil.
* Si una selección está activa cuando abres el diálogo de conteo de palabras, ahora se mostrará cuántas palabras has seleccionado.

##### Atajos de teclado
* La capacidad de personalizar cada atajo de teclado en la aplicación a través de un diálogo simple.
* Un atajo de teclado configurable para restaurar Paperback desde la bandeja del sistema.

##### Idiomas
* Holandés, finlandés y polaco.

##### Exportar
* Se expandió el elemento de menú exportar para permitir exportar a HTML y Markdown, además de texto plano.

##### Actualizador
* Un botón cancelar al diálogo de actualización en progreso.
* El actualizador ahora valida que el archivo descargado no haya sido manipulado.

##### Web View
* La vista web ahora se abre en tu posición de lectura actual.

##### Libros DAISY
* Soporte para libros DAISY 2.0.
* Soporte para reproducción de audio DAISY 2.02.

##### Audiolibros
* La capacidad de reproducir audiolibros, actualmente soportando tanto DAISY audio (incluyendo DAISY audio + texto) como archivos zip de audio.
* Atajos de teclado e elementos de menú para reproducir/pausar narración, avanzar y retroceder, y ajustar la cantidad de búsqueda.
* Opciones para sincronizar el cursor de lectura a la reproducción de audio, establecer la cantidad de búsqueda de audio y elegir si la búsqueda pasada el final de un capítulo continúa en el siguiente.

##### Documentos CHM
* Soporte para listas, elementos de lista, figuras e imágenes.

##### PowerPoint
* Los documentos PowerPoint ahora soportan tablas.

#### Correcciones

##### General
* Los documentos codificados en codificaciones CJK heredadas, como GBK, Big5 y Shift_JIS, ahora se renderizarán correctamente en lugar de un montón de mojibake.
* "Reabre el último cerrado" intentando reabre el léame incluido.
* Tu pestaña seleccionada no enfocándose correctamente después de reiniciar Paperback.
* El manejo de Paperback de archivos en unidades de red de Windows: presionar mostrar archivo en carpeta ahora enfoca correctamente el archivo en el almacenamiento de red, y las rutas ya no contienen caracteres extraños.
* Los archivos .paperback ya no se cargarán forzadamente en la restauración de documentos; en su lugar, se te pedirá confirmación cuando se encuentre uno.
* Abrir carpeta contenedora ahora enfoca el archivo dado en el explorador.
* Abrir el léame ahora respetará tu idioma seleccionado.
* La interfaz de usuario de Paperback ahora se escalará correctamente en pantallas de alta resolución.
* El menú ahora se actualiza correctamente, y el enfoque se mueve al control de texto, al abrir ayuda en Paperback.
* Se cambió a un método mucho más seguro de IPC en Windows.
* El título del documento activo ahora se lee al cambiar entre pestañas.
* Se redujo el uso de memoria en documentos grandes reduciendo a la mitad el tamaño de las tablas de índice internas por carácter.

##### Diálogo Todos los documentos
* Escape no cerrando los diálogos Información del documento y Todos los documentos.
* La barra de título no actualizándose después de cerrar un documento desde el diálogo de todos los documentos.
* Readme.html ya no se añadirá a tu lista de todos los documentos cuando se abra vía Shift+F1.
* Remover documentos del diálogo recientes ahora también cerrará su pestaña activa.
* Tu filtro de búsqueda ahora se preserva después de remover un documento.

##### Navegación
* Navegación de página anunciando texto de línea incorrecto en algunas situaciones.
* Ir a línea, Ir a página e Ir a porcentaje colocando tu cursor en la posición incorrecta en documentos grandes.
* Buscar y Buscar siguiente no respetando la ventana de documento cargada en documentos grandes.

##### Marcadores
* Los sonidos de marcador/nota ahora deberían reproducirse exclusivamente cuando navegas sobre una palabra que contiene uno.

##### Legibilidad
* Aplicar ajuste de línea disparándote al inicio de tu documento.

##### Web View
* El diálogo de vista web no siendo redimensionable y surgiendo en un tamaño inicial muy pequeño.
* Las imágenes ahora deberían mostrarse correctamente en la vista web integrada.

##### Actualizador
* El actualizador ahora muestra correctamente el contenido de las etiquetas de código markdown en las notas de versión.

##### Libros DAISY
* Los libros DAISY mostrando información incorrecta en la barra de estado.
* Cargando libros DAISY con declaraciones de codificación falsas.

##### Documentos RTF
* Analizando documentos RTF con caracteres no latinos en ellos.
* Grupos RTF `\pict` para que los datos de imagen incrustados ya no se filtren en el texto del documento.

##### Libros Mobi/AZW3
* Los anclajes filepos en libros Mobi dividiendo etiquetas HTML y poniendo basura en el texto del libro.
* Enlaces en libros Mobi heredados.
* Análisis AZW3 muy mejorado.

##### Documentos Word
* Documentos Word con nombres de estilos específicos de locale no renderizando sus encabezados correctamente.

##### Documentos HTML/XHTML
* Los elementos dl, dt y dd no produciendo saltos de línea en documentos XHTML.

##### Documentos PDF
* Paperback ahora recurre a la extracción de texto plano para PDFs etiquetados falsamente.
* Los documentos PDF que contienen caracteres de control en sus títulos y/o marcadores ya no harán que Paperback se bloquee al abrirse.

### Versión 0.8.5
* Se agregó soporte de página a libros epub.
* Se agregó soporte para documentos cifrados de Microsoft Office. Actualmente se soportan Word heredado, Word moderno y Powerpoint moderno, con Powerpoint heredado planeado para el futuro.
* ¡Se agregó soporte para documentos heredados de Microsoft Word!
* ¡Se agregó soporte para presentaciones heredadas de Powerpoint!
* ¡Se agregó soporte para libros mobi y AZW3!
* ¡Se agregó soporte para archivos PDF etiquetados!
* Se agregó el atajo ctrl+q para salir de la aplicación.
* ¡Se agregó soporte para libros comprimidos de Bookshare (tanto DAISY como Word)!
* El texto alternativo para imágenes incrustadas ahora debería mostrarse correctamente.
* Los documentos CHM ahora soportan correctamente la navegación de enlaces internos.
* Se corrigió que ir a página estuviera desplazado por 1.
* Se corrigió que la tecla escape no funcionara para cerrar el diálogo abrir como.
* Se corrigió que el menú contextual del lector no apareciera al hacer clic derecho o presionar la tecla Aplicaciones.
* Se corrigió que a veces se enfocara el documento incorrecto al abrir documentos desde la línea de comandos.
* Los PDF solo de imagen se detectan nuevamente y te alertan de su existencia.
* Ahora es posible navegar a través de imágenes y figuras con g/shift+g y f/shift+f, respectivamente.
* Paperback ahora respetará tu configuración de modo oscuro de la aplicación.
* Se eliminó el soporte DAISY XML, ya que ya no es necesario.
* Se volvió al navegador nativo de Win32 de primera letra en el árbol de contenido.
* El diálogo de carga de errores ahora muestra mensajes de error más detallados.
* La vista web ahora se abrirá mucho más rápido y suave.

### Versión 0.8.2
* ¡Se agregó soporte de página a documentos RTF!
* Se corrigió un error donde abrir la vista web en epubs que contienen enlaces externos los activaría automáticamente.
* Se corrigió un error donde el analizador RTF no pondría un espacio entre palabras en casos raros.
* Se corrigieron párrafos divididos en múltiples líneas cortas en algunos documentos PDF.
* ¡Los documentos PDF ahora tienen soporte básico de navegación de enlaces y encabezados!
* Las pestañas RTF y saltos de línea ahora se renderan exactamente como aparecen en el documento.
* Se volvió a la biblioteca pdfium probada y verdadera para analizar PDF, haciendo que la renderización de PDF sea mucho más confiable nuevamente.

### Versión 0.8.1
* Se agregó Ctrl+Shift+T para reabre el último documento cerrado.
* El diálogo Todos los documentos ahora soporta seleccionar múltiples documentos para abrir a la vez.
* Se corrigieron algunos errores con el analizador RTF.
* Se corrigieron rutas de archivo que contienen caracteres no ASCII (como š, č, ć, ž bosnios) corrompidas al abrir un archivo a través de una segunda instancia de Paperback.
* Se corrigió el texto PDF siendo leído en el orden incorrecto, y espaciado incorrecto alrededor de palabras en mayúsculas.
* Se corrigió carga lenta de documentos al abrir archivos grandes.
* Se corrigió la localización de los botones Sí/No en diálogos de confirmación.

### Versión 0.8.0
* ¡Se agregaron traducciones al japonés, chino simplificado y vietnamita!
* ¡Se agregó un actualizador automático que ahora reemplazará tu versión actualmente instalada de Paperback en lugar de solo descargar la nueva versión!
* ¡Se agregó retroalimentación de sonido opcional para llegar a un marcador o una nota, gracias Andre Louis por los sonidos!
* ¡Se agregó soporte para documentos RTF!
* Se agregó soporte para documentos DAISY XML.
* ¡Se agregó soporte para archivos de Documento de texto abierto plano!
* ¡Se agregó soporte para presentaciones de documento abierto plano!
* Se agregó soporte para separadores con s y shift+s.
* Cualquier movimiento mayor de 300 caracteres ahora agregará automáticamente a tu historial de navegación.
* Se corrigió la restauración de la ventana de Paperback desde la bandeja del sistema.
* Se corrigieron documentos Markdown mostrando texto sin procesar en lugar de HTML renderizado en Web View.
* Se corrigieron las tablas no renderándose correctamente en archivos Markdown.
* Los PDF solo de imagen ahora te advertirán de su existencia cuando intentes cargar uno.
* Se incrustó correctamente información de versión en el ejecutable de Paperback.
* Se dividió el diálogo de opciones en pestañas para facilidad de uso y navegación.
* Se cambió a Hayro para analizar PDF, lo que lleva a más confiabilidad, velocidad y menos DLL.
* Se reescribió toda la aplicación en Rust. La nueva base de código es más segura, carga documentos más rápido y es más fácil de mantener y extender.
* El menú contextual del control de texto ahora incluirá acciones específicas del lector en lugar de elementos genéricos como cortar y pegar.

### Versión 0.7.0
* ¡Se agregó soporte de tabla para documentos basados en HTML y XHTML! Navega entre tablas usando T y Shift+T, y presiona Intro para ver una en una vista web.
* ¡Se agregó una función básica de renderizado web! Presiona Ctrl+Shift+V para abrir la sección actual de tu documento en un renderizador basado en web, útil para contenido como formateo complejo o muestras de código.
* ¡Se agregó una traducción al ruso, gracias Ruslan Gulmagomedov!
* Se agregó un botón Limpiar todo al diálogo Todos los documentos.
* El verificador de actualización ahora muestra notas de versión cuando hay una nueva versión disponible.
* Se corrigió la restauración de la ventana desde la bandeja del sistema.
* Se corrigieron las traducciones de botones Sí/No en diálogos de confirmación.
* Se corrigió la carga de configuraciones al ejecutar como administrador.
* Se corrigió el manejo de comentarios en documentos XML y HTML.
* Se corrigió el análisis de TOC en libros Epub 2.
* Se corrigió la navegación al siguiente elemento con la misma letra en la tabla de contenidos.
* Se corrigió que el diálogo de búsqueda no se ocultara correctamente al usar los botones siguiente/anterior.
* Se corrigió que los TOC de epub ocasionalmente te llevaran al elemento incorrecto.
* Se corrigieron varios problemas de manejo de espacios en blanco en XML, HTML y etiquetas pre.
* Se corrigió error desplazado por uno en navegación de enlaces.
* Se corrigió que algunos libros tuvieran espacios en blanco al final en sus líneas.
* Se corrigieron varios problemas del analizador.
* Los elementos de menú relacionados con marcadores así como la lista de elementos ahora se deshabilitan correctamente cuando ningún documento está abierto.
* Se mejoró el manejo de listas en varios formatos de documentos.
* Se mejoró el flujo de trabajo de traducción para contribuidores.
* Muchos refactores internos, moviendo la mayoría de la lógica empresarial de la aplicación de C++ a Rust para mejorar el rendimiento y la mantenibilidad.

### Versión 0.6.1
* ¡Se agregó soporte para PDF protegido con contraseña!
* Se agregó una función muy básica de ir a posición anterior/siguiente. Si presionas enter en un enlace interno y mueve tu cursor, esa posición ahora será recordada, y pueda ser navegada con flechas alt+izquierda/derecha.
* ¡Se agregó una lista de elementos! Actualmente solo muestra un árbol de todos los encabezados en tu documento o una lista de enlaces, pero hay planes para expandirlo en el futuro.
* Se agregó una opción para iniciar Paperback en modo maximizado por defecto.
* Se corrigieron enlaces en algunos documentos Epub no funcionando correctamente.
* Se corrigió el análisis de TOC de Epub que contienen rutas relativas.
* Se corrigieron algunos documentos epub no mostrando un título o autor.
* Se corrigieron los títulos de algunos capítulos de epub no apareciendo correctamente en el diálogo TOC.
* Se corrigió que no pudieras usar la barra espaciadora para activar los botones OK/cancelar en el diálogo TOC.
* Se mejoró el manejo de encabezados en documentos Word.
* Ahora recibirás retroalimentación hablada si la lista de documentos recientes está vacía cuando intentes abrir el diálogo.

### Versión 0.6.0
* Se agregó una nueva opción para mostrar el menú go en una forma mucho más compacta al diálogo de opciones, marcado por defecto.
* Se agregó una opción para hacer que la navegación por elementos estructurales se envuelva.
* Se agregó una opción al menú de herramientas para abrir la carpeta contenedora del documento enfocado actualmente.
* ¡Se agregó un sistema de actualización bastante simple, pero muy efectivo!
* ¡Se agregó una función básica de temporizador de sueño, accesible con Ctrl+Shift+S!
* ¡Se agregó soporte para analizar libros electrónicos FB2!
* ¡Se agregó soporte para analizar presentaciones OpenDocument!
* ¡Se agregó soporte para analizar archivos de Texto OpenDocument!
* Los marcadores ahora pueden marcarse para marcar una línea completa, o para marcar solo algo de texto especificado. Si no tienes selección activa al colocar un marcador, el comportamiento es como pre-0.6, y marcará la línea completa. Sin embargo, si seleccionas algo de texto, solo ese texto se incluirá en el marcador.
* ¡Los marcadores ahora pueden tener notas de texto opcionales adjuntas a ellos! Navega entre marcadores que contienen notas con N y Shift+N, o abre el diálogo de marcadores con todos los marcadores, solo notas, o solo no-notas seleccionados con teclas de acceso rápido específicas.
* Los marcadores en el diálogo de marcadores ya no tendrán un prefijo "marcador x" molesto.
* Los libros Epub que contienen contenido HTML pretendiendo ser XML ahora se manejarán correctamente.
* Se corrigió la carga de documentos Markdown grandes.
* Se corrigió presionar espacio en el árbol de vista de tabla de contenidos activando el botón OK.
* Se corrigió el manejo de espacios en blanco al inicio de etiquetas pre en documentos HTML y XHTML.
* Se corrigió el control de texto no recuperando enfoque a veces cuando regresabas a la ventana de Paperback.
* Se corrigió el campo de texto en el diálogo de porcentaje no actualizando el valor del slider.
* Se corrigió la renderización de ID HTML personalizados en documentos Markdown.
* El HTML dentro de bloques de código Markdown ahora se renderará correctamente.
* Si cargas un libro con un parámetro de línea de comandos mientras una instancia existente de Paperback se está ejecutando, ya no recibirás un error si cargar tu documento tarda más de 5 segundos.
* Si ejecutas Paperback como administrador, la configuración ahora se cargará y guardará correctamente.
* Ahora es posible eliminar un marcador directamente desde dentro del diálogo de marcadores.
* Ahora es posible importar y exportar tus marcadores y posición de lectura para un documento en particular. El archivo generado se nombra después del archivo con una extensión .paperback. Si se encuentra tal archivo en el mismo directorio que un archivo al cargarlo, se cargará automáticamente. De lo contrario, puedes importarlos manualmente usando un elemento en el menú de herramientas.
* ¡Los enlaces dentro de documentos ahora son totalmente soportados! Usa k y shift+k para moverte adelante y atrás a través de ellos, y presiona enter para abrir/activar uno.
* Muchos refactores internos, haciendo la aplicación más rápida y el binario más pequeño.
* El contenido de Markdown ahora se preprocesa para cumplir con CommonMark antes de renderizarse.
* ¡La navegación por listas y sus elementos ahora es totalmente soportada! Usa L y Shift+L para ir por las propias listas, e I y Shift+I para recorrer elementos de lista.
* Ahora el suprimir de teclado numérico funciona para remover documentos de la barra de pestañas además del suprimir normal.
* ¡Paperback ahora puede minimizarse opcionales a tu bandeja del sistema! Esta opción está desactivada por defecto, pero activarla hará que la opción minimizar en el menú del sistema ponga Paperback en tu bandeja, pudiendo ser restaurado haciendo clic en el icono generado.
* ¡Paperback ahora es totalmente traducible! La lista de idiomas que soporta es actualmente bastante pequeña, pero crece constantemente!
* ¡Paperback ahora tiene un sitio web oficial, en [paperback.dev](https://paperback.dev)!
* Los documentos PPTX ahora mostrarán una tabla de contenidos básica, que contiene todas las diapositivas.
* La ruta completa al documento abierto ahora se mostrará en el diálogo de información del documento.
* El instalador ahora incluye una opción para ver el léame en tu navegador después de la instalación.
* ¡La lista de documentos recientes ha sido dramáticamente expandida! En lugar de simplemente mostrarte los últimos 10 documentos que abriste, ahora mostrará un número personalizable, con el resto de los documentos que jamás has abierto siendo accesibles a través de un pequeño diálogo.
* Varias pequeñas mejoras a los analizadores en toda la junta, incluyendo poner una línea en blanco entre diapositivas en presentaciones PPTX, corregir el manejo de nueva línea dentro de párrafos en documentos word, y agregar puntos de viñeta a elementos de lista.

### Versión 0.5.0
* ¡Se agregó soporte para documentos de Microsoft Word!
* ¡Se agregó soporte para presentaciones de PowerPoint!
* Se corrigieron ciertos elementos de menú no siendo deshabilitados sin documentos abiertos.
* Se corrigió la orientación del slider de porcentaje.
* Se corrigió la tabla de contenidos en libros Epub con rutas de archivo codificadas por URL y/o ID de fragmento.
* Se corrigieron espacios en blanco siendo despojados de encabezados XHTML de formas extrañas.
* Se corrigió el manejo de espacios en blanco dentro de etiquetas pre anidadas en documentos HTML.
* ¡Los documentos HTML y Markdown ahora soportan la función de tabla de contenidos! Cuando cargas un documento HTML/Markdown, Paperback construirá su propia tabla de contenidos de la estructura de los encabezados en tu documento, y te la mostrará en el diálogo ctrl+t.
* Los documentos HTML ahora tendrán el título como se establece en la etiqueta title, si existe. De lo contrario, continuarán usando el nombre de archivo sin la extensión.
* Se cambió de UniversalSpeech a usar una región activa para reportar discurso. Esto significa que no se envían DLL de lector de pantalla junto con el programa, y más lectores de pantalla ahora serán soportados, como Microsoft Narrator.
* Se cambió de biblioteca zip para permitir abrir una gama más amplia de libros epub.
* El diálogo pidiéndote si quieres abrir tu documento como texto plano ha sido completamente rehecho, y ahora te permite abrir tu documento como texto plano, HTML o Markdown.
* El diálogo de porcentaje ahora incluye un campo de texto permitiéndote ingresar manualmente un porcentaje para saltar.
* El analizador HTML ahora reconocerá dd, dt y dl como elementos de lista.
* La tabla de contenidos en libros Epub se preservará exactamente nuevamente.
* El espacio unicode sin ruptura ahora se considera al despojar líneas en blanco.
* Ya no se te preguntará cómo quieres abrir un archivo no reconocido cada vez que lo cargas, solo la primera vez.

### Versión 0.4.1
* Se agregó un icono del menú inicio opcional al instalador.
* La tabla de contenidos ahora debería ser más limpia en algunos casos, por ejemplo si tienes un elemento hijo y padre con el mismo texto en la misma posición solo verás el elemento padre.
* Se corrigió la tabla de contenidos en ciertos documentos CHM.
* Se corrigió la tabla de contenidos en libros Epub 3 con rutas absolutas en ellos.
* Los documentos CHM ahora deberían mostrar su título como se establece en el archivo de metadatos.

### Versión 0.4.0
* ¡Se agregó soporte para archivos CHM!
* ¡Se agregó soporte para marcadores! Puedes tener tantos marcadores en tantos documentos como quieras. Puedes saltar adelante y atrás a través de ellos con b y shift+b, establecer uno con control+shift+b, y abrir un diálogo para saltar a un marcador específico con control+b.
* ¡Se agregó un instalador junto al archivo zip portátil! El instalador instalará Paperback en tu directorio Archivos de programa, y configurará automáticamente asociaciones de archivos para ti.
* Los archivos de texto con BOM ahora deberían decodificarse correctamente, y el BOM ya no se mostrará al inicio del texto tampoco.
* Se agregó mucha más información a la barra de estado. Ahora te mostrará tu línea actual, carácter y porcentaje de lectura.
* Los comentarios HTML, así como el contenido de etiquetas script y style, ya no se mostrarán en la salida de texto.
* Si pasas una ruta relativa a Paperback en la línea de comandos, ahora la resolverá correctamente.
* El movimiento de porcentaje ahora se maneja por su propio diálogo basado en slider, accesible con control+shift+g.
* Los documentos sin títulos o autores conocidos ahora siempre tendrán uno por defecto.
* La lógica de guardado de posición ahora es mucho más inteligente y solo debería escribir en el disco cuando sea absolutamente necesario.
* El documento en el que tenías enfoque cuando cerraste Paperback ahora es recordado entre reinicios de la aplicación.
* La entrada en los diálogos ir a línea e ir a página ahora debería ser desinfectada más estrictamente.
* Se corrigió la navegación de tabla de contenidos en libros epub 3 con rutas relativas en sus manifiestos.

### Versión 0.3.0
* Se corrigió la tabla de contenidos en libros epub con manifiestos codificados por URL.
* Se corrigió la navegación de encabezados en documentos HTML que contienen caracteres Unicode multibyte.
* Se corrigió el uso de CPU alto en documentos con títulos largos debido a una regresión en wxWidgets.
* Se corrigió la carga de archivos de texto UTF-8.
* Se corrigieron elementos TOC anidados en libros Epub colocando tu cursor en la posición incorrecta.
* Se corrigió un bloqueo al salir de la aplicación en ciertos casos.
* ¡Se agregó una casilla de verificación en el diálogo de opciones para habilitar o deshabilitar el ajuste de línea!
* Ahora es posible donar al desarrollo de Paperback, ya sea a través del nuevo elemento donar en el menú de ayuda o a través del enlace de proyecto patrocinador en la parte inferior de la página principal del repositorio de GitHub.
* Los documentos Markdown ahora siempre tendrán un título, y Paperback ahora debería poder cargar prácticamente cualquier archivo Markdown.
* Los documentos PDF ahora siempre tendrán un título, incluso si faltan los metadatos.
* Se cambió de biblioteca PDF a la usada en Chromium, lo que lleva a un análisis de PDF mucho más confiable en toda la junta.
* Ahora solo puedes tener una instancia de Paperback ejecutándose a la vez. Ejecutar paperback.exe con un nombre de archivo mientras ya se está ejecutando abrirá ese documento en la instancia ya en ejecución.
* Ahora puedes presionar eliminar en un documento en el control de pestaña para cerrarlo.

### Versión 0.2.1
* Se agregó el número total de páginas a la etiqueta de página en el diálogo ir a página.
* Se permite tabulación desde el contenido del documento a tu lista de documentos abiertos.
* Se corrigieron algunos errores donde las pulsaciones de encabezados a veces abrían documentos recientes si tenías suficientes de ellos.
* Paperback ahora eliminará guiones blandos innecesarios de la salida de texto.
* Se corrigió la navegación de encabezados a veces poniéndote en el carácter incorrecto.

### Versión 0.2.0
* ¡Se agregó soporte para documentos markdown!
* ¡Se agregó soporte para documentos PDF, incluyendo la capacidad de navegar entre páginas!
* Se agregaron pulsaciones de teclado para navegar por encabezados en contenido HTML, incluyendo libros epub y documentos markdown. Estas pulsaciones de teclado fueron diseñadas para funcionar similar a un lector de pantalla.
* Se corrigió la carga de epubs con nombres de archivo codificados por URL en sus manifiestos.
* Se corrigió la carga de libros epub 3 con XHTML incrustado dentro de ellos.
* Un mensaje ahora se habla si el documento no soporta una tabla de contenidos o secciones, como opuesto a los elementos de menú siendo deshabilitados.
* ¡Se agregó un menú de documentos recientes! Actualmente almacena tus últimos 10 documentos abiertos, y presionar enter en uno lo abrirá para lectura.
* ¡Se reescribió completamente el diálogo Buscar, haciéndolo mucho más simple de usar, mientras también se agregó un historial de tus últimas 25 búsquedas y soporte de expresiones regulares!
* Los documentos previamente abiertos ahora son recordados entre reinicios de la aplicación. Esto es configurable a través del nuevo elemento de opciones en el menú de herramientas.
* Se agregó shift+f1 para abrir el léame directamente en Paperback mismo.

### Versión 0.1.0
* Lanzamiento inicial.
