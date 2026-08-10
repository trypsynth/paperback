<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Taschenbuch -- Version 0.8.5 {#paperback---version-0.8.5}

## Einführung {#introduction}

Paperback ist ein schlanker, schneller und barrierefreier E-Book- und
Dokumenten-Reader für alle, vom Gelegenheitsleser bis zum intensiven
Power-User. Er ist auf Barrierefreiheit für Screenreader, hohe
Geschwindigkeit und ein übersichtliches, unüberladenes Erlebnis
ausgelegt.

## Systemanforderungen {#system-requirements}

Paperback läuft derzeit unter Windows, macOS, iOS und Android.

## Funktionen {#features}

-   Völlig eigenständig -- es muss keine Software auf Ihrem Computer
    installiert werden, um mit dem Lesen zu beginnen.
-   Unglaublich schnell, selbst auf älterer Hardware.
-   Einfache Benutzeroberfläche mit Registerkarten, mit der Sie so viele
    Dokumente wie Sie möchten nebeneinander öffnen können.
-   Speichert Ihre genaue Leseposition in jedem Dokument, das Sie
    öffnen.
-   Speichert optional, welche Dokumente Sie beim Schließen des
    Programms geöffnet hatten, und stellt diese beim nächsten Start
    wieder her.
-   Enthält Navigationsfunktionen, die denen im Web-Browsing-Modus
    vieler Bildschirmleseprogramme ähneln, um schnell und einfach durch
    Dokumente zu navigieren.
-   Enthält einen leistungsstarken Suchdialog mit Funktionen wie einem
    Suchverlauf und Unterstützung für reguläre Ausdrücke.
-   Kann vollständig portabel ausgeführt oder mit automatisch
    eingerichteten Dateizuordnungen installiert werden.
-   Unterstützt eine riesige Auswahl gängiger Dateiformate.

## Kompatibilität mit Bildschirmleseprogrammen {#screen-reader-compatibility}

Paperback funktioniert gut mit allen gängigen Bildschirmleseprogrammen.
Es gibt jedoch ein bekanntes Problem für JAWS-Nutzer.

### JAWS und Braillezeilen {#jaws-and-braille-displays}

Wenn Sie JAWS mit einer Braillezeile verwenden, kann es vorkommen, dass
lange Absätze abgeschnitten werden, wenn Sie mit den Navigationstasten
Ihrer Braillezeile vorwärts blättern. Der Befehl „Aktuellen Absatz
lesen" ist ebenfalls davon betroffen. Dies ist ein Fehler in der
Verarbeitung des RICHEDIT50W-Textsteuerelements durch JAWS, nicht ein
Problem in Paperback selbst, und es dauerte ziemlich lange, bis eine
Lösung gefunden wurde -- trotz Visperos Eifer, auf Probleme mit
Open-Source-Software zu reagieren.

Die Umgehungslösung, die schließlich nach monatelanger Wartezeit über
die JAWS-Diskussionsgruppe bekannt wurde, besteht darin, `paperback.jcf`
und die Option „Braille-Darstellung und -Verschiebung" auf „Immer DOM
verwenden, falls verfügbar" einzustellen. Außerdem sollten Sie „Text
nach Absätzen verschieben" aktivieren, da Ihr Bildschirm sonst auf dem
aktiven Absatz verbleibt, anstatt weiterzuspringen. Wenn beide
Einstellungen vorgenommen wurden, sollte die Textverschiebung korrekt
funktionieren.

## Derzeit unterstützte Dateiformate {#currently-supported-file-types}

Paperback unterstützt die folgenden Formate und Dateiendungen:

-   CHM-Hilfedateien (`.chm`)
-   DAISY-Bücher (`.opf`, `.zip`)
-   EPUB-Bücher (`.epub`)
-   FB2-E-Books (`.fb2`)
-   HTML-Dokumente (`.htm`, `.html`, `.xhtml`)
-   Markdown-Dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Microsoft Word-Dokumente (`.docx`, `.docm`, `.doc`)
-   MOBI-/Kindle-Bücher (`.mobi`, `.azw`, `.azw3`)
-   OpenDocument-Präsentationen (`.odp`, `.fodp`)
-   OpenDocument-Textdateien (`.odt`, `.fodt`)
-   PDF-Dokumente (`.pdf`)
-   PowerPoint-Präsentationen (`.pptx`, `.pptm`, `.ppt`)
-   RTF-Dokumente (`.rtf`)
-   Reiner Text und Protokolldateien (`.txt`, `.log`)

## Tastaturkürzel {#keyboard-shortcuts}

Paperback ist für die Bedienung vorrangig über die Tastatur konzipiert.
Hier sind die aktuellen Tastenkombinationen.

Die folgenden Tastenkombinationen gelten für Windows. Bei Abweichungen
unter macOS wird die entsprechende Tastenkombination in Klammern
angegeben -- hauptsächlich, weil Strg+G, Strg+W und Alt+Pfeil
links/rechts auf dieser Plattform bereits durch andere System- oder
Anwendungskonventionen belegt sind.

### Menü „Datei" {#file-menu}

-   `Ctrl+O`: Ein Dokument öffnen.
-   `Ctrl+F4` (macOS: `Cmd+W`): Das aktuelle Dokument schließen.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Schließen Sie alle
    geöffneten Dokumente.
-   `Ctrl+Shift+T`: Das zuletzt geschlossene Dokument erneut öffnen.
-   `Ctrl+R`: Zeige den Dialog „Alle Dokumente" an (aus „Zuletzt
    geöffnete Dokumente").
-   `Ctrl+Q`: Beenden (nur Windows; unter macOS befindet sich diese
    Option stattdessen im App-Menü).

### Menü „Gehe zu" {#go-menu}

-   `Ctrl+F`: Zeige den Suchdialog an.
-   `F3` (macOS: `Cmd+G`): Nächste Suche.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Vorheriges Suchen.
-   `Ctrl+G` (macOS: `Cmd+L`): Zur Zeile springen.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Zu Prozent springen.
-   `Ctrl+P`: Zur Seite springen (sofern vom aktuellen Dokument
    unterstützt).
-   `Alt+Left` (macOS: `Cmd+[`): Gehe im Navigationsverlauf zurück.
-   `Alt+Right` (macOS: `Cmd+]`): Im Navigationsverlauf vorwärts
    springen.
-   `[`: Vorheriger Abschnitt.
-   `]`: Nächster Abschnitt.
-   `Shift+H`: Vorherige Überschrift.
-   `H`: Nächste Überschrift.
-   `Shift+1` bis `Shift+6`: Vorherige Überschrift auf Ebene 1--6.
-   `1` bis `6`: Nächste Überschrift auf Ebene 1--6.
-   `Shift+P`: Vorherige Seite.
-   `P`: Nächste Seite.
-   `Shift+B`: Vorheriges Lesezeichen.
-   `B`: Nächstes Lesezeichen.
-   `Shift+N`: Vorherige Notiz.
-   `N`: Nächste Notiz.
-   `Ctrl+B`: Zu allen Lesezeichen und Notizen springen.
-   `Ctrl+Alt+B`: Nur zu Lesezeichen springen.
-   `Ctrl+Alt+M`: Nur zu Notizen springen.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d. h. die physische
    Control-Taste anstelle von Cmd): Notiztext an der aktuellen Position
    anzeigen.
-   `Shift+K`: Vorheriger Link.
-   `K`: Nächster Link.
-   `Shift+G`: Vorheriges Bild.
-   `G`: Nächstes Bild.
-   `Shift+F`: Vorherige Abbildung.
-   `F`: Nächste Abbildung.
-   `Shift+T`: Vorherige Tabelle.
-   `T`: Nächste Tabelle.
-   `Shift+S`: Vorheriges Trennzeichen.
-   `S`: Nächstes Trennzeichen.
-   `Shift+L`: Vorherige Liste.
-   `L`: Nächste Liste.
-   `Shift+I`: Vorheriger Listenpunkt.
-   `I`: Nächstes Listenelement.
-   `Shift+,`: Zum Anfang des aktuellen Containers (Liste oder Tabelle)
    springen.
-   `,`: Über das Ende des aktuellen Containers (Liste oder Tabelle)
    hinausgehen.

### Menü „Extras" {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, d. h. die physische Control-Taste
    anstelle von Cmd): Wortanzahl für das aktuelle Dokument anzeigen.
-   `Ctrl+I`: Dokumentinformationen anzeigen.
-   `Ctrl+T`: Inhaltsverzeichnis anzeigen.
-   `F7`: Elementliste anzeigen.
-   `Ctrl+Shift+C`: Enthaltenden Ordner öffnen.
-   `Ctrl+Shift+V`: Aktuellen Inhalt in der Webansicht öffnen.
-   `Ctrl+U`: Dokumentquelle in einem neuen Tab anzeigen.
-   `Ctrl+Shift+E`: Dokumentdaten exportieren (`.paperback`).
-   `Ctrl+Shift+I`: Dokumentdaten importieren (`.paperback`).
-   `Ctrl+E`: Aktuelles Dokument als reinen Text exportieren.
-   `Ctrl+Shift+B`: Lesezeichen an der aktuellen Auswahl/Cursorposition
    setzen.
-   `Ctrl+Shift+N`: Lesezeichen-Notiz an der aktuellen
    Auswahl/Cursorposition hinzufügen oder bearbeiten.
-   `Ctrl+Alt+W`: Zeilenumbruch umschalten.
-   `Ctrl+,`: Optionen öffnen (macOS: Einstellungen, im App-Menü ).
-   `Ctrl+Shift+S`: Schlaf-Timer ein- oder ausschalten.

### Hilfe-Menü {#help-menu}

-   `Ctrl+F1`: „Über"-Dialogfeld anzeigen.
-   `F1`: Hilfe im Standardbrowser anzeigen.
-   `Shift+F1`: Hilfe in Paperback anzeigen.
-   `Ctrl+Shift+U`: Nach Updates suchen.
-   `Ctrl+D`: Die Spendenseite in Ihrem Standard-Browser öffnen.

### Zusätzliche Tasten zur Dokumentanzeige im {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` im Registerkartensteuerelement: Schließen
    Sie die Registerkarte des ausgewählten Dokuments.
-   `Enter` oder `Space` im Dokumenttext: Aktivieren Sie den Link an der
    Cursorposition oder öffnen Sie eine Tabellenansicht, wenn sich der
    Cursor auf einer Tabellenmarkierung befindet.
-   `Shift+F10` oder die Menü-/Anwendungstaste im Dokumenttext: Öffnen
    Sie das Kontextmenü.

## Unterstützte Sprachen {#supported-languages}

Paperback wurde in viele verschiedene Sprachen übersetzt, und es kommen
ständig weitere hinzu. Eine vollständige Liste finden Sie weiter unten.

Um zu erfahren, wie Sie mitwirken können, lesen Sie bitte unseren
[Übersetzungsleitfaden](translating.md).

-   Bosnisch
-   Tschechisch
-   Niederländisch
-   Finnisch
-   Französisch
-   Deutsch
-   Japanisch
-   Polnisch
-   Portugiesisch (Brasilien)
-   Russisch
-   Vereinfachtes Chinesisch
-   Serbisch
-   Spanisch
-   Vietnamesisch

## Impressum {#credits}

### Entwicklung {#development}

-   Quin Gillespie: Hauptentwickler und Projektgründer.
-   Aryan Choudhary: Hauptmitwirkender.

### Spenden {#donations}

Die folgenden Personen haben Spenden in nennenswerter Höhe für die
Entwicklung von Paperback geleistet. Wenn Sie eine Spende tätigen, wird
Ihr Name nicht automatisch hier aufgeführt; ich füge nur Personen hinzu,
die möchten, dass ihre Spende öffentlich bekannt gegeben wird.

Hinweis: Ich betrachte einen öffentlichen GitHub-Sponsor als Grund für
die automatische Aufnahme in diese Liste.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Felix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodriguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Perez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Änderungsprotokoll {#changelog}

### Version 0.9.0 (noch nicht veröffentlicht) {#version-0.9.0-unreleased}

-   Dem Dialogfeld „Aktualisierung läuft" wurde eine Schaltfläche
    „Abbrechen" hinzugefügt.
-   Ein CLI-Tool namens „pb" wurde hinzugefügt, um alle von Paperback
    unterstützten Formate schnell in HTML, Markdown oder einfachen Text
    zu konvertieren.
-   Es wurde eine konfigurierbare Tastenkombination hinzugefügt, um
    Paperback über die Systemleiste wiederherzustellen.
-   Im Dialog „Alle Dokumente" wurde eine Schaltfläche „Suchen"
    hinzugefügt, um fehlende Bücher zu finden, deren Pfad sich gerade
    geändert hat.
-   Im Optionsdialog wurde die Registerkarte „Lesbarkeit" mit den
    folgenden Optionen hinzugefügt:
    -   Zeilenumbruch (aus dem Bereich „Allgemein" verschoben);
    -   Tabellen inline darstellen (neu in dieser Version, siehe unten);
    -   Schriftart;
    -   Hintergrundfarbe;
    -   Zeilenabstand;
    -   Absatzabstand;
    -   Buchstabenabstand;
    -   Textausrichtung.
-   Es wurde eine Umschaltfunktion hinzugefügt, mit der Sie festlegen
    können, wie Tabellen angezeigt werden sollen, und die Darstellung
    von Tabellen wurde dokumentübergreifend vereinheitlicht.
-   Es wurde eine Option „Quelltext anzeigen" hinzugefügt, um den
    Quelltext eines Dokuments in einem neuen Tab zu öffnen, was
    beispielsweise für die Bearbeitung von Markdown nützlich ist.
-   Im Dialogfeld zur Wortzählung wurde die geschätzte Lesezeit
    hinzugefügt sowie die Möglichkeit, Ihre Lesegeschwindigkeit
    festzulegen, damit diese Angabe tatsächlich nützlich ist.
-   Unterstützung für ARM64 unter Windows hinzugefügt!
-   Android-Unterstützung hinzugefügt!
-   Unterstützung für iOS hinzugefügt!
-   macOS-Unterstützung hinzugefügt!
-   Neue Sprachen hinzugefügt: Niederländisch, Finnisch und Polnisch.
-   Unterstützung für die Navigation nach Containern hinzugefügt.
-   Unterstützung für Listen, Listenelemente, Abbildungen und Bilder in
    CHM-Dokumenten hinzugefügt.
-   Ein Menüpunkt für den Zeilenumbruch sowie ein entsprechender
    Tastenbefehl wurden hinzugefügt.
-   Lesezeichen-/Notiz-Sounds sollten nun korrekt und ausschließlich
    abgespielt werden, wenn Sie mit der Maus über ein Wort fahren, das
    ein solches enthält.
-   Dokumente, die in älteren CJK-Kodierungen wie GBK, Big5 und
    Shift_JIS kodiert sind, werden nun korrekt dargestellt und nicht
    mehr als eine Ansammlung von Zeichenkaos.
-   Der Menüpunkt „Exportieren" wurde erweitert, sodass nun neben reinem
    Text auch in HTML und Markdown exportiert werden kann.
-   Das Problem behoben, dass das Anwenden des Zeilenumbruchs den Nutzer
    an den Anfang des Dokuments zurückwarf.
-   Es wurde behoben, dass Daisy-Bücher falsche Informationen in der
    Statusleiste anzeigten.
-   Es wurde behoben, dass die Elemente „dl", „dt" und „dd" in XHTML-
    Dokumenten keine Zeilenumbrüche erzeugten.
-   Es wurde behoben, dass die Escape-Taste die Dialoge „Dokumentinfo"
    und „Alle Dokumente" nicht schloss.
-   Es wurde behoben, dass „filepos"-Anker in Mobi-Büchern HTML-Tags
    aufteilten und Fehler im Buchtext verursachten.
-   Es wurde ein Verzögerungsproblem behoben, das auftrat, wenn man sich
    in großen Dokumenten dem Ende des Textfelds näherte.
-   Es wurde ein Problem mit Links in älteren Mobi-Büchern behoben.
-   Das Laden von DAISY-Büchern mit fehlerhaften Kodierungsangaben wurde
    behoben.
-   Es wurde behoben, dass bei der Seitennavigation in manchen
    Situationen falscher Zeilentext angesagt wurde.
-   Das Parsen von RTF-Dokumenten mit nicht-lateinischen Zeichen wurde
    korrigiert.
-   Es wurde behoben, dass die Funktion „Zuletzt geschlossenes Dokument
    erneut öffnen" versuchte, die mitgelieferte README-Datei erneut zu
    öffnen.
-   Es wurde behoben, dass die Titelleiste nach dem Schließen eines
    Dokuments über den Dialog „Alle Dokumente" nicht aktualisiert wurde.
-   Es wurde behoben, dass der Webview-Dialog nicht in der Größe
    angepasst werden konnte und mit einer sehr kleinen Anfangsgröße
    angezeigt wurde.
-   Es wurde behoben, dass in Word-Dokumenten mit
    lokalisierungsspezifischen Stilnamen die Überschriften nicht korrekt
    dargestellt wurden.
-   Es wurde behoben, dass die ausgewählte Registerkarte nach dem
    Neustart von Paperback nicht richtig fokussiert wurde.
-   Wenn beim Öffnen des Wortzählungsdialogs eine Auswahl aktiv ist,
    wird nun angezeigt, wie viele Wörter Sie ausgewählt haben.
-   Bilder sollten nun in der eingebetteten Webansicht korrekt angezeigt
    werden.
-   Die Verarbeitung von Dateien auf Windows-Netzlaufwerken durch
    Paperback wurde verbessert: Durch Klicken auf „Datei im Ordner
    anzeigen" wird nun die Datei auf dem Netzwerkspeicher korrekt
    fokussiert, und die Pfade enthalten keine seltsamen Zeichen mehr.
-   Das Parsen von AZW3-Dateien wurde erheblich verbessert.
-   Umstellung von chmlib auf unseren eigenen, rein in Rust
    geschriebenen CHM-Dateireader.
-   Auf dem Desktop werden .paperback-Dateien bei der Wiederherstellung
    von Dokumenten nicht mehr zwangsweise geladen. Stattdessen werden
    Sie um Bestätigung gebeten, wenn die Datei gefunden wird.
-   Paperback greift nun bei falsch getaggten PDFs auf die Extraktion
    von Klartext zurück.
-   Beim Öffnen des enthaltenen Ordners wird nun die angegebene Datei im
    Explorer fokussiert.
-   Beim Öffnen der Readme-Datei wird nun die von Ihnen ausgewählte
    Sprache berücksichtigt.
-   PowerPoint-Dokumente unterstützen nun Tabellen.
-   Das Menü wird korrekt aktualisiert und der Fokus auf das Textfeld
    gesetzt, wenn die Hilfe in Paperback geöffnet wird.
-   „Readme.html" wird nicht mehr zur Liste „Alle Dokumente"
    hinzugefügt, wenn es über Umschalt+F1 geöffnet wird.
-   Das Entfernen von Dokumenten aus dem Dialog „Zuletzt verwendet"
    schließt nun auch deren aktiven Reiter.
-   Unter Windows wurde auf eine wesentlich sicherere Methode der IPC
    umgestellt.
-   Der Titel des aktiven Dokuments wird nun beim Wechseln zwischen
    Registerkarten vorgelesen.
-   Der Updater zeigt nun den Inhalt von Markdown-Code-Tags in den
    Versionshinweisen korrekt an.
-   Der Updater überprüft nun, ob die heruntergeladene Datei nicht
    manipuliert wurde.
-   Die Webansicht wird nun an Ihrer aktuellen Leseposition geöffnet.
-   Ihr Suchfilter im Dialog „Alle Dokumente" bleibt nun auch nach dem
    Entfernen eines Dokuments erhalten.

### Version 0.8.5

-   Unterstützung für Seiten in EPUB-Büchern hinzugefügt.
-   Unterstützung für verschlüsselte Microsoft-Office-Dokumente
    hinzugefügt. Derzeit werden ältere Word-Versionen, moderne
    Word-Versionen und moderne PowerPoint-Versionen unterstützt; die
    Unterstützung für ältere PowerPoint-Versionen ist für die Zukunft
    geplant.
-   Unterstützung für ältere Microsoft-Word-Dokumente (\*.doc)
    hinzugefügt!
-   Unterstützung für ältere PowerPoint-Präsentationen (\*.ppt)
    hinzugefügt!
-   Unterstützung für Mobi- und AZW3-Bücher hinzugefügt!
-   Unterstützung für getaggte PDF-Dateien hinzugefügt!
-   Die Tastenkombination Strg+Q zum Beenden der App wurde hinzugefügt.
-   Unterstützung für gezippte Bücher von Bookshare (sowohl DAISY als
    auch Word) hinzugefügt!
-   Alternativtext für eingebettete Bilder sollte nun korrekt angezeigt
    werden.
-   CHM-Dokumente unterstützen nun die Navigation über interne Links
    ordnungsgemäß.
-   Es wurde behoben, dass Lesezeichen-Töne am Absatzanfang statt an der
    Position des Lesezeichens ausgelöst wurden.
-   Es wurde behoben, dass die Seitennavigation um 1 versetzt war.
-   Es wurde behoben, dass die Escape-Taste nicht zum Schließen des
    „Öffnen als"-Dialogfelds funktionierte.
-   Es wurde behoben, dass das Kontextmenü des Readers bei einem
    Rechtsklick oder beim Drücken der „Anwendungen"-Taste nicht
    angezeigt wurde.
-   Es wurde behoben, dass beim Öffnen von Dokumenten über die
    Befehlszeile manchmal das falsche Dokument im Fokus stand.
-   Reine Bild-PDFs werden wieder erkannt und Sie werden auf deren
    Vorhandensein hingewiesen.
-   Es ist nun möglich, mit g/Umschalt+g bzw. f/Umschalt+f durch Bilder
    und Abbildungen zu navigieren.
-   Paperback berücksichtigt nun die Einstellung für den Dunkelmodus
    Ihrer Anwendung.
-   Die DAISY-XML-Unterstützung wurde entfernt, da sie nicht mehr
    benötigt wird.
-   Es wurde wieder auf die native Win32-Navigation nach
    Anfangsbuchstaben im Inhaltsverzeichnisbaum umgestellt.
-   Der Fehlerdialog beim Laden zeigt nun detailliertere Fehlermeldungen
    an.
-   Die Webansicht öffnet sich nun deutlich schneller und flüssiger.

### Version 0.8.2

-   Unterstützung für Seiten in RTF-Dokumenten hinzugefügt!
-   Ein Fehler wurde behoben, durch den beim Öffnen der Webansicht in
    EPUBs mit externen Links diese automatisch aktiviert wurden.
-   Es wurde ein Fehler behoben, durch den der RTF-Parser in seltenen
    Fällen kein Leerzeichen zwischen den Wörtern einfügte.
-   Es wurde ein Fehler behoben, durch den Absätze in einigen
    PDF-Dokumenten in mehrere kurze Zeilen aufgeteilt wurden.
-   PDF-Dokumente verfügen nun über grundlegende Unterstützung für die
    Navigation über Links und Überschriften!
-   RTF-Tabulatoren und Zeilenvorschübe werden nun genau so dargestellt,
    wie sie im Dokument erscheinen.
-   Es wurde wieder auf die bewährte „pdfium"-Bibliothek zum Parsen von
    PDFs umgestellt, wodurch die PDF-Darstellung wieder wesentlich
    zuverlässiger ist.

### Version 0.8.1

-   Die Tastenkombination Strg+Umschalt+T wurde hinzugefügt, um das
    zuletzt geschlossene Dokument erneut zu öffnen.
-   Der Dialog „Alle Dokumente" unterstützt nun die Auswahl mehrerer
    Dokumente, die gleichzeitig geöffnet werden können.
-   Einige Fehler im RTF-Parser wurden behoben.
-   Es wurde ein Problem behoben, bei dem Dateipfade mit
    Nicht-ASCII-Zeichen (wie z. B. die bosnischen Zeichen š, č, ć, ž)
    beim Öffnen einer Datei über eine zweite Paperback-Instanz
    beschädigt wurden.
-   Es wurde behoben, dass PDF-Text in falscher Reihenfolge gelesen
    wurde und dass die Abstände um großgeschriebene Wörter herum falsch
    waren.
-   Das langsame Laden von Dokumenten beim Öffnen großer Dateien wurde
    behoben.
-   Die Lokalisierung der Schaltflächen „Ja"/„Nein" in
    Bestätigungsdialogen wurde korrigiert.

### Version 0.8.0

-   Übersetzungen für Japanisch, vereinfachtes Chinesisch und
    Vietnamesisch hinzugefügt!
-   Es wurde ein automatischer Updater hinzugefügt, der nun Ihre aktuell
    installierte Version von Paperback ersetzt, anstatt nur die neue
    Version herunterzuladen!
-   Optionale akustische Rückmeldung beim Erreichen eines Lesezeichens
    oder einer Notiz hinzugefügt, vielen Dank an Andre Louis für die
    Sounds!
-   Unterstützung für RTF-Dokumente hinzugefügt!
-   Unterstützung für DAISY-XML-Dokumente hinzugefügt.
-   Unterstützung für Flat Open Document-Textdateien hinzugefügt!
-   Unterstützung für „Flat Open Document"-Präsentationen hinzugefügt!
-   Unterstützung für Trennzeichen mit „s" und „Umschalt+s" hinzugefügt.
-   Jede Bewegung von mehr als 300 Zeichen wird nun automatisch zu Ihrem
    Navigationsverlauf hinzugefügt.
-   Das Wiederherstellen des Paperback-Fensters aus der Taskleiste wurde
    korrigiert.
-   Es wurde behoben, dass Markdown-Dokumente in der Webansicht Rohtext
    anstelle von gerendertem HTML anzeigten.
-   Es wurde behoben, dass Tabellen in Markdown-Dateien nicht korrekt
    dargestellt wurden.
-   Bei reinen Bild-PDFs werden Sie nun gewarnt, wenn Sie versuchen,
    eine solche Datei zu laden.
-   Es ist nun möglich, bei der Suche nach Updates nach neuen
    Entwicklerversionen anstelle von stabilen Versionen zu suchen.
-   Versionsinformationen wurden ordnungsgemäß in die ausführbare Datei
    von Paperback eingebettet.
-   Der Optionsdialog wurde zur Vereinfachung der Bedienung und
    Navigation in Registerkarten unterteilt.
-   Wechsel zu Hayro für das Parsen von PDFs, was zu mehr
    Zuverlässigkeit, Geschwindigkeit und weniger DLLs führt.
-   Die gesamte App wurde in Rust neu geschrieben. Die neue Codebasis
    ist sicherer, lädt Dokumente schneller und lässt sich leichter
    warten und erweitern.
-   Das Kontextmenü des Textsteuerelements enthält nun leserspezifische
    Aktionen anstelle von allgemeinen Einträgen wie „Ausschneiden" und
    „Einfügen".

### Version 0.7.0

-   Unterstützung für Tabellen in HTML- und XHTML-basierten Dokumenten
    hinzugefügt! Navigieren Sie mit T und Umschalt+T zwischen Tabellen
    und drücken Sie die Eingabetaste, um eine Tabelle in einer
    Webansicht anzuzeigen.
-   Eine grundlegende Web-Rendering-Funktion wurde hinzugefügt! Drücke
    Strg+Umschalt+V, um den aktuellen Abschnitt deines Dokuments in
    einem webbasierten Renderer zu öffnen -- nützlich für Inhalte wie
    komplexe Formatierungen oder Code-Beispiele.
-   Eine russische Übersetzung wurde hinzugefügt -- vielen Dank an
    Ruslan Gulmagomedov!
-   Im Dialogfeld „Alle Dokumente" wurde die Schaltfläche „Alle löschen"
    hinzugefügt.
-   Der Update-Checker zeigt nun Versionshinweise an, wenn eine neue
    Version verfügbar ist.
-   Das Wiederherstellen des Fensters aus der Taskleiste wurde
    korrigiert.
-   Die Übersetzungen der „Ja/Nein"-Schaltflächen in
    Bestätigungsdialogen wurden korrigiert.
-   Das Laden von Konfigurationen bei Ausführung als Administrator wurde
    korrigiert.
-   Die Verarbeitung von Kommentaren in XML- und HTML-Dokumenten wurde
    korrigiert.
-   Das Parsen des Inhaltsverzeichnisses in Epub-2-Büchern wurde
    korrigiert.
-   Es wurde ein Fehler behoben, der das Navigieren zum nächsten Eintrag
    mit demselben Buchstaben im Inhaltsverzeichnis verhinderte.
-   Es wurde behoben, dass sich der Suchdialog bei Verwendung der
    Schaltflächen „Weiter" und „Zurück" nicht ordnungsgemäß ausblendete.
-   Es wurde behoben, dass EPUB-Inhaltsverzeichnisse gelegentlich zum
    falschen Eintrag führten.
-   Verschiedene Probleme bei der Verarbeitung von Leerzeichen in XML-,
    HTML- und \`pre\`-Tags wurden behoben.
-   Ein „Off-by-one"-Fehler bei der Link-Navigation wurde behoben.
-   Es wurde behoben, dass einige Bücher am Zeilenende Leerzeichen
    aufwiesen.
-   Verschiedene Parser-Probleme wurden behoben.
-   Menüelemente im Zusammenhang mit Lesezeichen sowie die Elementliste
    werden nun korrekt deaktiviert, wenn kein Dokument geöffnet ist.
-   Die Verarbeitung von Listen in verschiedenen Dokumentformaten wurde
    verbessert.
-   Der Übersetzungs-Workflow für Mitwirkende wurde verbessert.
-   Zahlreiche interne Umstrukturierungen, bei denen der Großteil der
    Geschäftslogik der Anwendung von C++ nach Rust verlagert wurde, um
    die Leistung und Wartbarkeit zu verbessern.

### Version 0.6.1

-   Unterstützung für passwortgeschützte PDF-Dateien hinzugefügt!
-   Eine sehr einfache Funktion zum Springen zur vorherigen/nächsten
    Position wurde hinzugefügt. Wenn Sie bei einem internen Link die
    Eingabetaste drücken und sich der Cursor dadurch bewegt, wird diese
    Position nun gespeichert und kann mit Alt+Pfeil nach links/rechts
    angesteuert werden.
-   Eine Elementliste wurde hinzugefügt! Derzeit zeigt sie nur eine
    Baumstruktur aller Überschriften in Ihrem Dokument oder eine Liste
    von Links an, es ist jedoch geplant, sie in Zukunft zu erweitern.
-   Es wurde eine Option hinzugefügt, um Paperback standardmäßig im
    maximierten Modus zu starten.
-   Es wurde ein Fehler behoben, durch den Links in einigen
    EPUB-Dokumenten nicht richtig funktionierten.
-   Das Parsen von EPUB-Inhaltsverzeichnissen mit relativen Pfaden wurde
    korrigiert.
-   Es wurde behoben, dass bei einigen EPUB-Dokumenten kein Titel oder
    Autor angezeigt wurde.
-   Es wurde behoben, dass die Titel einiger EPUB-Kapitel im
    Inhaltsverzeichnis-Dialogfeld nicht korrekt angezeigt wurden.
-   Es wurde behoben, dass die Leertaste nicht zum Aktivieren der
    Schaltflächen „OK"/„Abbrechen" im Inhaltsverzeichnis-Dialogfeld
    verwendet werden konnte.
-   Die Verarbeitung von Überschriften in Word-Dokumenten wurde
    verbessert.
-   Sie erhalten nun eine Sprachansage, wenn die Liste der zuletzt
    geöffneten Dokumente leer ist, wenn Sie versuchen, den Dialog
    aufzurufen.

### Version 0.6.0

-   Eine neue Option, um das „Gehe zu"-Menü in einer wesentlich
    kompakteren Form anzuzeigen, wurde dem Optionsdialog hinzugefügt und
    ist standardmäßig aktiviert.
-   Es wurde eine Option hinzugefügt, um die Navigation anhand von
    Strukturelementen umbrechen zu lassen.
-   Dem Menü „Extras" wurde eine Option hinzugefügt, um den
    übergeordneten Ordner des aktuell ausgewählten Dokuments zu öffnen.
-   Ein recht einfaches, aber sehr effektives Aktualisierungssystem
    wurde hinzugefügt.
-   Eine einfache Sleep-Timer-Funktion wurde hinzugefügt, die über
    Strg+Umschalt+S aufgerufen werden kann.
-   Unterstützung für das Parsen von FB2-E-Books hinzugefügt!
-   Unterstützung für das Parsen von OpenDocument-Präsentationen
    hinzugefügt!
-   Unterstützung für das Parsen von OpenDocument-Textdateien
    hinzugefügt!
-   Lesezeichen können nun gesetzt werden, um eine ganze Zeile zu
    markieren oder nur einen bestimmten Textabschnitt. Wenn beim Setzen
    eines Lesezeichens keine Auswahl aktiv ist, verhält es sich wie vor
    Version 0.6 und markiert die gesamte Zeile. Wenn Sie jedoch Text
    auswählen, wird nur dieser Text in das Lesezeichen aufgenommen.
-   Lesezeichen können nun optional mit Textnotizen versehen werden!
    Navigiere mit N und Umschalt+N zwischen Lesezeichen, die Notizen
    enthalten, oder rufe den Lesezeichendialog mit allen Lesezeichen,
    nur Notizen oder nur Nicht-Notizen auf, die mit bestimmten
    Tastenkombinationen ausgewählt wurden.
-   Lesezeichen im Lesezeichendialog haben nun kein störendes Präfix
    „Lesezeichen x" mehr.
-   Epub-Bücher, die HTML-Inhalte enthalten, die als XML getarnt sind,
    werden nun korrekt verarbeitet.
-   Das Laden großer Markdown-Dokumente wurde korrigiert.
-   Es wurde behoben, dass das Drücken der Leertaste in der Baumansicht
    des Inhaltsverzeichnisses die Schaltfläche „OK" aktivierte.
-   Die Behandlung von Leerzeichen am Anfang von \`pre\`-Tags wurde
    sowohl in HTML- als auch in XHTML-Dokumenten korrigiert.
-   Es wurde behoben, dass das Textsteuerelement manchmal den Fokus
    nicht zurückerhielt, wenn man zum Fenster von Paperback
    zurückkehrte.
-   Es wurde behoben, dass das Textfeld im Dialogfeld „Zu Prozent
    springen" den Wert des Schiebereglers nicht aktualisierte.
-   Die Darstellung benutzerdefinierter HTML-IDs in Markdown-Dokumenten
    wurde korrigiert.
-   HTML innerhalb von Markdown-Codeblöcken wird nun korrekt
    dargestellt.
-   Wenn du ein Buch mit einem Befehlszeilenparameter lädst, während
    eine bestehende Paperback-Instanz läuft, erhältst du keinen Fehler
    mehr, wenn das Laden deines Dokuments länger als 5 Sekunden dauert.
-   Wenn du Paperback als Administrator ausführst, wird die
    Konfiguration nun korrekt geladen und gespeichert.
-   Es ist nun möglich, ein Lesezeichen direkt aus dem
    Lesezeichen-Dialog heraus zu löschen.
-   Es ist nun möglich, Ihre Lesezeichen und die Leseposition für ein
    bestimmtes Dokument zu importieren und zu exportieren. Die
    generierte Datei wird nach der Datei benannt und erhält die
    Erweiterung .paperback. Wird eine solche Datei beim Laden im selben
    Verzeichnis wie die zu ladende Datei gefunden, wird sie automatisch
    geladen. Andernfalls können Sie sie manuell über einen Eintrag im
    Menü „Extras" importieren.
-   Links innerhalb von Dokumenten werden nun vollständig unterstützt!
    Verwenden Sie „k" und „Umschalt+k", um vorwärts und rückwärts durch
    sie zu navigieren, und drücken Sie die Eingabetaste, um einen Link
    zu öffnen bzw. zu aktivieren.
-   Zahlreiche interne Umstrukturierungen, die die App schneller und die
    Binärdatei kleiner machen.
-   Markdown-Inhalte werden nun vor der Darstellung vorverarbeitet, um
    CommonMark-konform zu sein.
-   Die Navigation durch Listen und deren Elemente wird nun vollständig
    unterstützt! Verwenden Sie „L" und „Umschalt+L", um durch die Listen
    selbst zu navigieren, sowie „I" und „Umschalt+I", um durch die
    Listenelemente zu navigieren.
-   Die Löschtaste auf dem Ziffernblock funktioniert nun zusätzlich zur
    normalen Löschtaste, um Dokumente aus der Registerkartenleiste zu
    entfernen.
-   Paperback lässt sich nun optional in die Taskleiste minimieren!
    Diese Option ist standardmäßig deaktiviert, aber wenn du sie
    aktivierst, wird Paperback durch die Minimieren-Option im Systemmenü
    in die Taskleiste verschoben und kann durch Klicken auf das
    angezeigte Symbol wiederhergestellt werden.
-   Paperback ist nun vollständig übersetzbar! Die Liste der
    unterstützten Sprachen ist derzeit noch recht klein, wächst aber
    stetig!
-   Paperback hat nun eine offizielle Website unter
    [paperback.dev](https://paperback.dev)!
-   PPTX-Dokumente zeigen nun ein einfaches Inhaltsverzeichnis an, das
    alle Folien enthält.
-   Der vollständige Pfad zum geöffneten Dokument wird nun im
    Dokument-Info-Dialog angezeigt.
-   Das Installationsprogramm enthält nun eine Option, um die
    Readme-Datei nach der Installation in Ihrem Browser anzuzeigen.
-   Die Liste der zuletzt geöffneten Dokumente wurde erheblich
    erweitert! Anstatt Ihnen lediglich die letzten 10 Dokumente
    anzuzeigen, die Sie geöffnet haben, wird nun eine anpassbare Anzahl
    angezeigt, wobei die übrigen Dokumente, die Sie jemals geöffnet
    haben, über einen kleinen Dialog zugänglich sind.
-   Verschiedene kleinere Verbesserungen an den Parsern in allen
    Bereichen, darunter das Einfügen einer Leerzeile zwischen den Folien
    in PPTX-Präsentationen, die Korrektur der Zeilenumbruchbehandlung
    innerhalb von Absätzen in Word-Dokumenten und das Hinzufügen von
    Aufzählungszeichen zu Listenelementen.

### Version 0.5.0

-   Unterstützung für Microsoft Word-Dokumente hinzugefügt!
-   Unterstützung für PowerPoint-Präsentationen hinzugefügt!
-   Es wurde behoben, dass bestimmte Menüelemente nicht deaktiviert
    wurden, wenn keine Dokumente geöffnet waren.
-   Die Ausrichtung des Schiebereglers „Zu-Prozent-Sprung" wurde
    korrigiert.
-   Das Inhaltsverzeichnis in EPUB-Büchern mit URL-kodierten Dateipfaden
    und/oder Fragment-IDs wurde korrigiert.
-   Es wurde behoben, dass Leerzeichen auf seltsame Weise aus
    XHTML-Überschriften entfernt wurden.
-   Die Behandlung von Leerzeichen innerhalb verschachtelter
    \`pre\`-Tags in HTML- Dokumenten wurde korrigiert.
-   HTML- und Markdown-Dokumente unterstützen nun die
    Inhaltsverzeichnis-Funktion ! Wenn Sie ein HTML-/Markdown-Dokument
    laden, erstellt Paperback ein eigenes Inhaltsverzeichnis anhand der
    Struktur der Überschriften in Ihrem Dokument und zeigt es Ihnen im
    Strg+T-Dialog an.
-   HTML-Dokumente verwenden nun den im „title"-Tag festgelegten Titel,
    sofern dieser vorhanden ist. Andernfalls wird weiterhin der
    Dateiname ohne die Erweiterung verwendet.
-   Es wurde von UniversalSpeech auf die Verwendung eines Live-Bereichs
    zur Sprachausgabe umgestellt. Das bedeutet, dass keine
    Screenreader-DLLs mehr zusammen mit dem Programm ausgeliefert werden
    und nun mehr Screenreader unterstützt werden, wie beispielsweise
    Microsoft Narrator.
-   Die ZIP-Bibliotheken wurden umgestellt, um das Öffnen einer größeren
    Auswahl an EPUB- Büchern zu ermöglichen.
-   Der Dialog, in dem Sie gefragt werden, ob Sie Ihr Dokument als
    einfachen Text öffnen möchten, wurde komplett überarbeitet und
    ermöglicht es Ihnen nun, Ihr Dokument als einfachen Text, HTML oder
    Markdown zu öffnen.
-   Der Dialog „Zu Prozent springen" enthält nun ein Textfeld, in das
    Sie manuell einen Prozentsatz eingeben können, zu dem Sie springen
    möchten.
-   Der HTML-Parser erkennt nun „dd", „dt" und „dl" als Listenelemente.
-   Das Inhaltsverzeichnis in EPUB-Büchern wird wieder exakt
    beibehalten.
-   Das Unicode-Schusstrichzeichen wird nun beim Entfernen von
    Leerzeilen berücksichtigt.
-   Sie werden nicht mehr bei jedem Laden einer nicht erkannten Datei
    gefragt, wie Sie diese öffnen möchten, sondern nur noch beim ersten
    Mal.

### Version 0.4.1

-   Dem Installationsprogramm wurde ein optionales Startmenü-Symbol
    hinzugefügt.
-   Das Inhaltsverzeichnis sollte nun in einigen Fällen übersichtlicher
    sein; wenn Sie beispielsweise ein untergeordnetes und ein
    übergeordnetes Element mit demselben Text an derselben Position
    haben, wird nun nur noch das übergeordnete Element angezeigt.
-   Das Inhaltsverzeichnis in bestimmten CHM-Dokumenten wurde
    korrigiert.
-   Das Inhaltsverzeichnis in Epub-3-Büchern mit absoluten Pfaden wurde
    korrigiert.
-   CHM-Dokumente sollten nun den Titel anzeigen, der in der
    Metadaten-Datei festgelegt ist.

### Version 0.4.0

-   Unterstützung für CHM-Dateien hinzugefügt!
-   Unterstützung für Lesezeichen hinzugefügt! Sie können so viele
    Lesezeichen in so vielen Dokumenten setzen, wie Sie möchten. Mit „b"
    und „Umschalt+b" können Sie zwischen den Lesezeichen hin- und
    herspringen, mit „Strg+Umschalt+b" ein Lesezeichen setzen und mit
    „Strg+b" ein Dialogfeld aufrufen, um zu einem bestimmten Lesezeichen
    zu springen.
-   Neben der portablen ZIP-Datei wurde ein Installationsprogramm
    hinzugefügt! Das Installationsprogramm installiert Paperback in
    Ihrem Verzeichnis „Programme" und richtet automatisch die
    Dateizuordnungen für Sie ein.
-   Textdateien mit BOMs sollten nun korrekt dekodiert werden, und die
    BOM wird auch nicht mehr am Anfang des Textes angezeigt.
-   Die Statusleiste wurde um weitaus mehr Informationen erweitert. Sie
    zeigt dir nun die aktuelle Zeile, das aktuelle Zeichen und den
    Lesefortschritt in Prozent an.
-   HTML-Kommentare sowie der Inhalt von Skript- und Stil-Tags werden in
    der Textausgabe nicht mehr angezeigt.
-   Wenn du Paperback über die Befehlszeile einen relativen Pfad
    übergibst, wird dieser nun korrekt aufgelöst.
-   Die prozentuale Verschiebung wird nun über einen eigenen, auf einem
    Schieberegler basierenden Dialog gehandhabt, der mit Strg+Umschalt+g
    aufgerufen werden kann.
-   Dokumente ohne bekannten Titel oder Autor erhalten nun immer einen
    Standardwert.
-   Die Logik zum Speichern der Position ist nun wesentlich
    intelligenter und sollte nur dann auf die Festplatte schreiben, wenn
    es absolut notwendig ist.
-   Das Dokument, das beim Schließen von Paperback im Fokus stand, wird
    nun auch nach einem Neustart der Anwendung beibehalten.
-   Eingaben in den Dialogen „Zur Zeile springen" und „Zur Seite
    springen" sollten nun strenger überprüft werden.
-   Die Navigation im Inhaltsverzeichnis von EPUB-3-Büchern mit
    relativen Pfaden in ihren Manifesten wurde korrigiert.

### Version 0.3.0

-   Das Inhaltsverzeichnis in EPUB-Büchern mit URL-kodierten Manifesten
    wurde korrigiert.
-   Die Navigation durch Überschriften in HTML-Dokumenten, die
    Multibyte- Unicode-Zeichen enthalten, wurde korrigiert.
-   Die hohe CPU-Auslastung in Dokumenten mit langen Titeln, die auf
    eine Regression in wxWidgets zurückzuführen war, wurde behoben.
-   Das Laden von UTF-8-Textdateien wurde korrigiert.
-   Es wurde ein Problem behoben, bei dem verschachtelte
    Inhaltsverzeichnis-Einträge in EPUB-Büchern den Cursor an die
    falsche Position setzten.
-   Ein Absturz beim Beenden der Anwendung in bestimmten Fällen wurde
    behoben.
-   Im Optionsdialog wurde ein Kontrollkästchen hinzugefügt, um den
    Zeilenumbruch zu aktivieren oder zu deaktivieren!
-   Es ist nun möglich, die Entwicklung von Paperback zu unterstützen,
    entweder über den neuen Menüpunkt „Spenden" im Hilfemenü oder über
    den Link „Dieses Projekt sponsern" unten auf der Hauptseite des
    GitHub-Repositorys.
-   Markdown-Dokumente haben nun immer einen Titel, und Paperback sollte
    nun in der Lage sein, praktisch jede Markdown-Datei zu laden.
-   PDF-Dokumente haben nun immer einen Titel, auch wenn die Metadaten
    fehlen.
-   Die PDF-Bibliotheken wurden auf die in Chromium verwendete
    umgestellt, was zu einer deutlich zuverlässigeren PDF-Analyse in
    allen Bereichen führt.
-   Es kann nun jeweils nur eine Instanz von Paperback gleichzeitig
    ausgeführt werden. Wenn Sie „paperback.exe" mit einem Dateinamen
    ausführen, während das Programm bereits läuft, wird das Dokument in
    der bereits laufenden Instanz geöffnet.
-   Sie können nun im Register-Bereich die Entf-Taste bei einem Dokument
    drücken, um es zu schließen.

### Version 0.2.1

-   Die Gesamtseitenzahl wurde zur Seitenbezeichnung im Dialogfeld „Zur
    Seite springen" hinzugefügt.
-   Es ist nun möglich, per Tabulator vom Dokumentinhalt zu Ihrer Liste
    der geöffneten Dokumente zu wechseln.
-   Es wurde behoben, dass die Tastenkombinationen für Überschriften
    manchmal zuletzt geöffnete Dokumente öffneten, wenn genügend davon
    vorhanden waren.
-   Paperback entfernt nun unnötige weiche Bindestriche aus der
    Textausgabe.
-   Es wurde behoben, dass die Navigation in Überschriften manchmal zum
    falschen Zeichen führte.

### Version 0.2.0

-   Unterstützung für Markdown-Dokumente hinzugefügt!
-   Unterstützung für PDF-Dokumente hinzugefügt, einschließlich der
    Möglichkeit, zwischen den Seiten zu navigieren!
-   Tastenkombinationen für die Navigation anhand von Überschriften in
    HTML-Inhalten hinzugefügt, einschließlich EPUB-Büchern und
    Markdown-Dokumenten. Diese Tastenkombinationen wurden so konzipiert,
    dass sie ähnlich wie ein Screenreader funktionieren.
-   Das Laden von EPUBs mit URL-kodierten Dateinamen in ihren Manifesten
    wurde behoben.
-   Das Laden von EPUB-3-Büchern mit darin eingebettetem XHTML wurde
    korrigiert.
-   Es wird nun eine Ansage ausgegeben, wenn das Dokument kein
    Inhaltsverzeichnis oder keine Abschnitte unterstützt, anstatt die
    Menüelemente zu deaktivieren.
-   Ein Menü „Zuletzt geöffnete Dokumente" wurde hinzugefügt! Es
    speichert derzeit Ihre letzten 10 geöffneten Dokumente, und durch
    Drücken der Eingabetaste bei einem davon wird dieses zum Lesen
    geöffnet.
-   Der Suchdialog wurde komplett überarbeitet, wodurch er nun
    wesentlich einfacher zu bedienen ist, und es wurden zudem ein
    Verlauf der letzten 25 Suchanfragen sowie Unterstützung für reguläre
    Ausdrücke hinzugefügt!
-   Zuvor geöffnete Dokumente bleiben nun auch nach einem Neustart der
    Anwendung erhalten. Dies lässt sich über den neuen Menüpunkt
    „Optionen" im Menü „Extras" konfigurieren.
-   Die Tastenkombination Umschalt+F1 wurde hinzugefügt, um die
    Readme-Datei direkt in Paperback selbst zu öffnen.

### Version 0.1.0

-   Erstveröffentlichung.
