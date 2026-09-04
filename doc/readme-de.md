# Paperback - Version 0.9.2

## Einführung

Paperback ist ein schlanker, schneller und barrierefreier E-Book- und Dokumenten-Reader für alle – vom Gelegenheitsleser bis zum intensiven Power-User. Er ist auf Barrierefreiheit für Screenreader, hohe Geschwindigkeit und ein schnörkelloses Erlebnis ausgelegt.

## Systemanforderungen

Paperback läuft derzeit unter Windows, macOS, iOS und Android.

## Funktionen

* Völlig eigenständig – es muss keine Software auf dem Computer installiert werden, um mit dem Lesen zu beginnen.
* Unglaublich schnell, selbst auf älterer Hardware.
* Einfache Benutzeroberfläche mit Registerkarten, mit der beliebig viele Dokumente nebeneinander geöffnet werden können.
* Speichert die genaue Leseposition in jedem geöffneten Dokument.
* Merkt sich optional, welche Dokumente beim Schließen des Programms geöffnet waren, und stellt sie beim nächsten Start wieder her.
* Enthält Navigationsfunktionen, ähnlich wie im Web-Browsing-Modus vieler Screenreader, um schnell und einfach durch Dokumente zu navigieren.
* Enthält einen leistungsstarken Suchdialog mit Funktionen wie Verlauf und Unterstützung für reguläre Ausdrücke.
* Kann vollständig portabel ausgeführt oder mit automatisch eingerichteten Dateizuordnungen installiert werden.
* Unterstützt eine riesige Auswahl gängiger Dateiformate.

## Kompatibilität mit Bildschirmleseprogrammen

Paperback funktioniert gut mit allen gängigen Bildschirmleseprogrammen. Es gibt jedoch ein bekanntes Problem für JAWS-Nutzer.

### JAWS und Braillezeilen

Wird JAWS mit einer Braillezeile verwendet, kann es vorkommen, dass lange Absätze abgeschnitten werden, wenn man mit den Navigationstasten der Braillezeile vorwärts blättert. Der Befehl „Aktuellen Absatz vorlesen“ ist ebenfalls davon betroffen. Dies ist ein Fehler in der Verarbeitung des RICHEDIT50W-Textsteuerelements durch JAWS, kein Problem in Paperback selbst, und es hat ziemlich lange gedauert, bis eine Lösung gefunden wurde – angesichts Visperos Eifer, auf Probleme mit Open-Source-Software zu reagieren.

Die Umgehungslösung, die nach monatelanger Wartezeit schließlich über die JAWS-Diskussionsgruppe bekannt wurde, besteht darin, die Datei `paperback.jcf` zu bearbeiten und unter „Braille-Darstellung und -Navigation“ die Option „Immer DOM verwenden, falls verfügbar“ zu aktivieren. Außerdem sollte „Text absatzweise navigieren“ aktiviert sein, da das Display sonst beim aktiven Absatz stehen bleibt, anstatt weiterzuspringen. Wenn beide Einstellungen vorgenommen sind, sollte die Navigation korrekt funktionieren.

## Derzeit unterstützte Dateitypen

Paperback unterstützt die folgenden Formate und Dateiendungen:

* CHM-Hilfedateien (`.chm`)
* DAISY-Bücher (`.opf`, `.zip`)
* EPUB-Bücher (`.epub`)
* FB2-E-Books (`.fb2`)
* HTML-Dokumente (`.htm`, `.html`, `.xhtml`)
* Markdown-Dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-Dokumente (`.docx`, `.docm`, `.doc`)
* MOBI/Kindle-Bücher (`.mobi`, `.azw`, `.azw3`)
* OpenDocument-Präsentationen (`.odp`, `.fodp`)
* OpenDocument-Textdateien (`.odt`, `.fodt`)
* PDF-Dokumente (`.pdf`)
* PowerPoint-Präsentationen (`.pptx`, `.pptm`, `.ppt`)
* RTF-Dokumente (`.rtf`)
* Nur-Text- und Protokolldateien (`.txt`, `.log`)

## Tastaturkürzel

Paperback ist für die Bedienung vorrangig über die Tastatur konzipiert. Hier sind die aktuellen Tastaturkürzel.

Die folgenden Tastaturkürzel gelten für Windows. Wo es bei macOS Abweichungen gibt, ist die entsprechende Tastenkombination in Klammern angegeben – hauptsächlich, weil Strg+G, Strg+W und Alt+Pfeil links/rechts auf dieser Plattform bereits von anderen System- oder App-Konventionen belegt sind.

### Menü „Datei“

* `Strg+O`: Ein Dokument öffnen.
* `Strg+F4` (macOS: `Cmd+W`): Das aktuelle Dokument schließen.
* `Strg+Umschalt+F4` (macOS: `Cmd+Umschalt+W`): Alle geöffneten Dokumente schließen.
* `Strg+Umschalt+T`: Das zuletzt geschlossene Dokument wieder öffnen.
* `Strg+R`: Den Dialog „Alle Dokumente“ anzeigen (aus den zuletzt verwendeten Dokumenten).
* `Strg+Q`: Beenden (nur Windows; unter macOS findest du diese Option stattdessen im App-Menü).

### Menü „Gehe zu“

* `Strg+F`: Den Suchdialog anzeigen.
* `F3` (macOS: `Cmd+G`): Nächstes Suchergebnis.
* `Umschalt+F3` (macOS: `Cmd+Umschalt+G`): Vorheriges Suchergebnis.
* `Strg+G` (macOS: `Cmd+L`): Zur Zeile springen.
* `Strg+Umschalt+G` (macOS: `Cmd+Umschalt+L`): Zur Prozentangabe springen.
* `Strg+P`: Zur Seite springen (sofern vom aktuellen Dokument unterstützt).
* `Alt+Pfeil nach links` (macOS: `Cmd+[`): Im Navigationsverlauf zurückgehen.
* `Alt+Pfeil nach rechts` (macOS: `Cmd+]`): Im Navigationsverlauf vorwärtsgehen.
* `[`: Vorheriger Abschnitt.
* `]`: Nächster Abschnitt.
* `Umschalt+H`: Vorherige Überschrift.
* `H`: Nächste Überschrift.
* `Umschalt+1` bis `Umschalt+6`: Vorherige Überschrift der Ebenen 1–6.
* `1` bis `6`: Nächste Überschrift der Ebenen 1–6.
* `Umschalt+P`: Vorherige Seite.
* `P`: Nächste Seite.
* `Umschalt+B`: Vorheriges Lesezeichen.
* `B`: Nächstes Lesezeichen.
* `Umschalt+N`: Vorherige Notiz.
* `N`: Nächste Notiz.
* `Strg+B`: Zu allen Lesezeichen und Notizen springen.
* `Strg+Alt+B`: Nur zu Lesezeichen springen.
* `Strg+Alt+M`: Nur zu Notizen springen.
* `Strg+Umschalt+W` (macOS: `RawStrg+Umschalt+W`, d. h. die physische Strg-Taste statt Cmd): Notiztext an der aktuellen Position anzeigen.
* `Umschalt+K`: Vorheriger Link.
* `K`: Nächster Link.
* `Umschalt+G`: Vorheriges Bild.
* `G`: Nächstes Bild.
* `Umschalt+F`: Vorherige Abbildung.
* `F`: Nächste Abbildung.
* `Umschalt+T`: Vorherige Tabelle.
* `T`: Nächste Tabelle.
* `Umschalt+S`: Vorherige Trennlinie.
* `S`: Nächste Trennlinie.
* `Umschalt+L`: Vorherige Liste.
* `L`: Nächste Liste.
* `Umschalt+I`: Vorheriger Listeneintrag.
* `I`: Nächster Listeneintrag.
* `Umschalt+,`: Zum Anfang des aktuellen Containers (Liste oder Tabelle) springen.
* `,`: Über das Ende des aktuellen Containers (Liste oder Tabelle) hinausspringen.

### Menü „Extras“

* `Strg+W` (macOS: `RawStrg+W`, d. h. die physische Strg-Taste statt Cmd): Wortanzahl für das aktuelle Dokument anzeigen.
* `Strg+I`: Dokumentinformationen anzeigen.
* `Strg+T`: Inhaltsverzeichnis anzeigen.
* `F7`: Elementliste anzeigen.
* `Strg+Umschalt+C`: Übergeordneten Ordner öffnen.
* `Strg+Umschalt+V`: Aktuellen Inhalt in der Webansicht öffnen.
* `Strg+U`: Dokumentquelle in einem neuen Tab anzeigen.
* `Strg+Umschalt+E`: Dokumentdaten exportieren (`.paperback`).
* `Strg+Umschalt+I`: Dokumentdaten importieren (`.paperback`).
* `Strg+E`: Aktuelles Dokument als reinen Text exportieren.
* `Strg+Umschalt+B`: Lesezeichen an der aktuellen Auswahl/Cursorposition setzen/aufheben.
* `Strg+Umschalt+N`: Lesezeichen-Notiz an der aktuellen Auswahl/Cursorposition hinzufügen oder bearbeiten.
* `Strg+Alt+W`: Zeilenumbruch ein- oder ausschalten.
* `Strg+,`: Optionen öffnen (macOS: Einstellungen, im App-Menü).
* `Strg+Umschalt+S`: Einschlaf-Timer umschalten.

### Hilfe-Menü

* `Strg+F1`: „Über“-Dialog anzeigen.
* `F1`: Hilfe im Standardbrowser anzeigen.
* `Umschalt+F1`: Hilfe in Paperback anzeigen.
* `Strg+Umschalt+U`: Nach Updates suchen.
* `Strg+D`: Die Spendenseite im Standardbrowser öffnen.

### Weitere Tasten für die Dokumentansicht

* `Entf` / `Entf auf dem Ziffernblock` im Registerkartenbereich: Schließe die ausgewählte Dokumentregisterkarte.
* `Enter` oder `Leertaste` im Dokumenttext: Aktiviere den Link an der Cursorposition oder öffne eine Tabellenansicht, wenn du dich auf einer Tabellenmarkierung befindest.
* `Umschalt+F10` oder die Menü-/Anwendungstaste im Dokumenttext: Öffne das Kontextmenü.

## Unterstützte Sprachen

Paperback wurde in viele verschiedene Sprachen übersetzt, und es kommen ständig neue hinzu. Eine vollständige Liste findet sich weiter unten.

Um zu erfahren, wie du mitwirken kannst, lies bitte unseren [Übersetzungsleitfaden](translating.md).

* Bosnisch
* Tschechisch
* Niederländisch
* Finnisch
* Französisch
* Deutsch
* Japanisch
* Polnisch
* Portugiesisch (Brasilien)
* Russisch
* Vereinfachtes Chinesisch
* Serbisch
* Spanisch
* Vietnamesisch

## Danksagungen
### Entwicklung
* Quin Gillespie: Hauptentwickler und Projektgründer.
* Aryan Choudhary: Hauptmitwirkender.

### Spenden
Die folgenden Personen haben Spenden in nennenswertem Umfang für die Entwicklung von Paperback geleistet. Wenn du eine Spende tätigst, wird dein Name nicht automatisch hier hinzugefügt; ich füge nur Personen hinzu, die möchten, dass ihre Spende öffentlich bekannt gegeben wird.

Hinweis: Ein öffentlicher GitHub-Sponsor gilt für mich als Grund für die automatische Aufnahme in diese Liste.

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
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Änderungsprotokoll

### Version 0.9.0 (noch nicht veröffentlicht)
* Dem Dialogfeld „Aktualisierung läuft“ wurde eine Schaltfläche „Abbrechen“ hinzugefügt.
* Es wurde ein CLI-Tool namens „pb“ hinzugefügt, mit dem sich alle von Paperback unterstützten Formate schnell in HTML, Markdown oder einfachen Text konvertieren lassen.
* Es wurde ein konfigurierbarer Tastaturbefehl hinzugefügt, um Paperback über die Taskleiste wiederherzustellen.
* Im Dialog „Alle Dokumente“ wurde eine Suchschaltfläche hinzugefügt, um fehlende Bücher zu finden, deren Pfad sich gerade geändert hat.
* Im Optionsdialog wurde die Registerkarte „Lesbarkeit“ mit den folgenden Optionen hinzugefügt:
    * Zeilenumbruch (aus dem Bereich „Allgemein“ verschoben);
    * Tabellen inline darstellen (neu in dieser Version, siehe unten);
    * Schriftart;
    * Hintergrundfarbe;
    * Zeilenabstand;
    * Absatzabstand;
    * Zeichenabstand;
    * Textausrichtung.
* Es wurde ein Umschalter hinzugefügt, mit dem festgelegt werden kann, wie Tabellen angezeigt werden sollen, und die Darstellung von Tabellen wurde dokumentübergreifend vereinheitlicht.
* Es wurde die Option „Quelltext anzeigen“ hinzugefügt, um den Quelltext eines Dokuments in einem neuen Tab zu öffnen – nützlich zum Beispiel für die Bearbeitung von Markdown.
* Im Dialogfeld zur Wortzählung wurde die geschätzte Lesezeit hinzugefügt sowie die Möglichkeit, die Lesegeschwindigkeit einzustellen, damit diese Angabe tatsächlich nützlich ist.
* Unterstützung für ARM64 unter Windows hinzugefügt!
* Unterstützung für Android hinzugefügt!
* Unterstützung für iOS hinzugefügt!
* Unterstützung für macOS hinzugefügt!
* Neue Sprachen hinzugefügt: Niederländisch, Finnisch und Polnisch.
* Unterstützung für die Navigation nach Containern hinzugefügt.
* Unterstützung für Listen, Listenelemente, Abbildungen und Bilder in CHM-Dokumenten hinzugefügt.
* Ein Menüpunkt für den Zeilenumbruch und ein entsprechender Tastenbefehl wurden hinzugefügt.
* Lesezeichen-/Notiz-Sounds sollten nun korrekt und ausschließlich abgespielt werden, wenn du über ein Wort navigierst, das ein solches enthält.
* Der Export-Menüpunkt wurde erweitert, sodass nun zusätzlich zu reinem Text auch in HTML und Markdown exportiert werden kann.
* Es wurde behoben, dass man beim Anwenden des Zeilenumbruchs an den Anfang des Dokuments gesprungen ist.
* Es wurde behoben, dass DAISY-Bücher falsche Informationen in der Statusleiste anzeigten.
* Es wurde behoben, dass die Elemente „dl“, „dt“ und „dd“ in XHTML-Dokumenten keine Zeilenumbrüche erzeugten.
* Es wurde behoben, dass die Escape-Taste die Dialoge „Dokumenteninfo“ und „Alle Dokumente“ nicht schloss.
* Es wurde behoben, dass „filepos“-Anker in Mobi-Büchern HTML-Tags aufspalteten und unerwünschte Zeichen in den Buchtext einfügten.
* Es wurde eine Verzögerung behoben, die auftrat, wenn man sich in großen Dokumenten dem Ende des Textfeldes näherte.
* Es wurden Links in älteren Mobi-Büchern korrigiert.
* Das Laden von DAISY-Büchern mit fehlerhaften Kodierungsangaben wurde behoben.
* Das Parsen von RTF-Dokumenten mit nicht-lateinischen Zeichen wurde behoben.
* Es wurde behoben, dass sich die Titelleiste nach dem Schließen eines Dokuments über den Dialog „Alle Dokumente“ nicht aktualisierte.
* Es wurde behoben, dass der Webview-Dialog nicht in der Größe angepasst werden konnte und zunächst in einer sehr kleinen Größe erschien.
* Es wurde behoben, dass in Word-Dokumenten mit lokalisierungsspezifischen Stilnamen die Überschriften nicht richtig dargestellt wurden.
* Es wurde behoben, dass der ausgewählte Reiter nach dem Neustart von Paperback nicht richtig fokussiert wurde.
* Wenn beim Öffnen des Wortzähl-Dialogs eine Auswahl aktiv ist, wird nun angezeigt, wie viele Wörter ausgewählt sind.
* Die Verarbeitung von Dateien auf Windows-Netzlaufwerken durch Paperback wurde verbessert: Beim Klicken auf „Datei im Ordner anzeigen“ wird nun die Datei auf dem Netzlaufwerk korrekt fokussiert, und die Pfade enthalten keine seltsamen Zeichen mehr.
* Das Parsen von AZW3-Dateien wurde erheblich verbessert.
* Wir sind von chmlib auf unseren eigenen, rein in Rust geschriebenen CHM-Dateireader umgestiegen.
* Auf dem Desktop werden .paperback-Dateien bei der Dokumentwiederherstellung nicht mehr zwangsweise geladen. Stattdessen wird um Bestätigung gebeten, wenn die Datei gefunden wird.
* Paperback greift nun bei falsch getaggten PDFs auf die Extraktion von reinem Text zurück.
* „Enthaltenden Ordner öffnen“ setzt nun den Fokus auf die angegebene Datei im Explorer.
* Beim Öffnen der Readme-Datei wird nun die ausgewählte Sprache berücksichtigt.
* PowerPoint-Dokumente unterstützen nun Tabellen.
* Das Menü wird beim Öffnen der Hilfe in Paperback korrekt aktualisiert und der Fokus auf das Textfeld gesetzt.
* „Readme.html“ wird nicht mehr zur Liste „Alle Dokumente“ hinzugefügt, wenn es über Umschalt+F1 geöffnet wurde.
* Das Entfernen von Dokumenten aus dem „Zuletzt verwendet“-Dialog schließt nun auch den aktiven Tab.
* Unter Windows wurde auf eine wesentlich sicherere Methode der interprozessualen Kommunikation (IPC) umgestellt.
* Der Titel des aktiven Dokuments wird nun beim Wechseln zwischen den Registerkarten vorgelesen.
* Der Updater zeigt nun den Inhalt von Markdown-Code-Tags in den Versionshinweisen korrekt an.
* Der Updater überprüft nun, ob die heruntergeladene Datei nicht manipuliert wurde.
* Die Webansicht wird nun an der aktuellen Leseposition geöffnet.
* Der Suchfilter im Dialog „Alle Dokumente“ bleibt nun auch nach dem Entfernen eines Dokuments erhalten.

### Version 0.8.5
* Unterstützung für Seiten in EPUB-Büchern hinzugefügt.
* Unterstützung für verschlüsselte Microsoft-Office-Dokumente hinzugefügt. Derzeit werden „Legacy Word“, „Modern Word“ und „Modern PowerPoint“ unterstützt; die Unterstützung für „Legacy PowerPoint“ ist für die Zukunft geplant.
* Unterstützung für ältere Microsoft-Word-Dokumente (*.doc) hinzugefügt!
* Unterstützung für ältere PowerPoint-Präsentationen (*.ppt) hinzugefügt!
* Unterstützung für Mobi- und AZW3-Bücher hinzugefügt!
* Unterstützung für getaggte PDF-Dateien hinzugefügt!
* Die Tastenkombination Strg+Q zum Beenden der App hinzugefügt.
* Unterstützung für gezippte Bücher von Bookshare (sowohl DAISY als auch Word) hinzugefügt!
* Der Alt-Text für eingebettete Bilder sollte nun korrekt angezeigt werden.
* CHM-Dokumente unterstützen nun die Navigation über interne Links korrekt.
* Das Problem behoben, dass Lesezeichen-Töne am Absatzanfang statt an der Position des Lesezeichens ausgelöst wurden.
* Das Problem behoben, dass die Seitennavigation um 1 versetzt war.
* Es wurde behoben, dass die Escape-Taste nicht funktionierte, um den „Öffnen als“-Dialog zu schließen.
* Es wurde behoben, dass das Kontextmenü des Readers bei Rechtsklick oder beim Drücken der Anwendungstaste nicht angezeigt wurde.
* Es wurde behoben, dass beim Öffnen von Dokumenten über die Befehlszeile manchmal das falsche Dokument fokussiert wurde.
* PDFs, die nur aus Bildern bestehen, werden wieder erkannt und es wird auf deren Vorhandensein hingewiesen.
* Es ist nun möglich, mit „g/Umschalt+g“ bzw. „f/Umschalt+f“ durch Bilder und Abbildungen zu navigieren.
* Paperback berücksichtigt nun die Einstellung für den Dunkelmodus der Anwendung.
* Die DAISY-XML-Unterstützung wurde entfernt, da sie nicht mehr benötigt wird.
* Es wurde wieder auf die native Win32-Navigation nach Anfangsbuchstaben im Inhaltsverzeichnisbaum umgestellt.
* Der Fehlerdialog beim Laden zeigt nun detailliertere Fehlermeldungen an.
* Die Webansicht öffnet sich nun deutlich schneller und flüssiger.

### Version 0.8.2
* Unterstützung für Seiten in RTF-Dokumenten hinzugefügt!
* Ein Fehler wurde behoben, durch den das Öffnen der Webansicht in ePubs mit externen Links diese automatisch aktivierte.
* Ein Fehler wurde behoben, durch den der RTF-Parser in seltenen Fällen kein Leerzeichen zwischen Wörtern einfügte.
* Es wurde behoben, dass Absätze in einigen PDF-Dokumenten in mehrere kurze Zeilen aufgeteilt wurden.
* PDF-Dokumente unterstützen nun grundlegende Navigation über Links und Überschriften!
* RTF-Tabulatoren und Zeilenvorschübe werden nun genau so dargestellt, wie sie im Dokument erscheinen.
* Wir sind wieder zur bewährten „pdfium“-Bibliothek für das Parsen von PDFs zurückgekehrt, wodurch die PDF-Darstellung wieder deutlich zuverlässiger ist.

### Version 0.8.1
* Die Tastenkombination Strg+Umschalt+T wurde hinzugefügt, um das zuletzt geschlossene Dokument wieder zu öffnen.
* Im Dialog „Alle Dokumente“ lassen sich nun mehrere Dokumente auswählen, um sie gleichzeitig zu öffnen.
* Ein paar Fehler beim RTF-Parser wurden behoben.
* Es wurde behoben, dass Dateipfade mit Nicht-ASCII-Zeichen (wie z. B. bosnische š, č, ć, ž) beim Öffnen einer Datei über eine zweite Paperback-Instanz beschädigt wurden.
* Es wurde behoben, dass PDF-Text in der falschen Reihenfolge gelesen wurde und die Abstände um großgeschriebene Wörter falsch waren.
* Das langsame Laden von Dokumenten beim Öffnen großer Dateien wurde behoben.
* Die Lokalisierung der „Ja/Nein“-Schaltflächen in Bestätigungsdialogen wurde korrigiert.

### Version 0.8.0
* Übersetzungen für Japanisch, vereinfachtes Chinesisch und Vietnamesisch hinzugefügt!
* Ein automatischer Updater wurde hinzugefügt, der nun die aktuell installierte Version von Paperback ersetzt, anstatt nur die neue Version herunterzuladen!
* Optionale akustische Rückmeldung beim Erreichen eines Lesezeichens oder einer Notiz hinzugefügt – danke an Andre Louis für die Sounds!
* Unterstützung für RTF-Dokumente hinzugefügt!
* Unterstützung für DAISY-XML-Dokumente hinzugefügt.
* Unterstützung für Flat Open Document-Textdateien hinzugefügt!
* Unterstützung für Flat Open Document-Präsentationen hinzugefügt!
* Unterstützung für Trennzeichen mit „s“ und „Umschalt+s“ hinzugefügt.
* Jede Bewegung von mehr als 300 Zeichen wird nun automatisch dem Navigationsverlauf hinzugefügt.
* Das Wiederherstellen des Paperback-Fensters aus der Taskleiste wurde korrigiert.
* Das Problem, dass Markdown-Dokumente in der Webansicht Quelltext statt gerenderten HTML-Codes anzeigten, wurde behoben.
* Es wurde behoben, dass Tabellen in Markdown-Dateien nicht korrekt dargestellt wurden.
* Bei reinen Bild-PDFs wird nun gewarnt, wenn man versucht, eine solche Datei zu laden.
* Es ist nun möglich, bei der Suche nach Updates nach neuen Entwickler-Builds statt nach stabilen Versionen zu suchen.
* Versionsinformationen wurden korrekt in die Paperback-Ausführungsdatei eingebettet.
* Der Optionsdialog wurde zur besseren Bedienbarkeit und Navigation in Registerkarten unterteilt.
* Für das Parsen von PDFs wurde auf Hayro umgestellt, was zu mehr Zuverlässigkeit, höherer Geschwindigkeit und weniger DLLs führt.
* Die gesamte App wurde in Rust neu geschrieben. Die neue Codebasis ist sicherer, lädt Dokumente schneller und lässt sich leichter warten und erweitern.
* Das Kontextmenü des Textsteuerelements enthält nun leserspezifische Aktionen anstelle von allgemeinen Einträgen wie „Ausschneiden“ und „Einfügen“.

### Version 0.7.0
* Unterstützung für Tabellen in HTML- und XHTML-basierten Dokumenten hinzugefügt! Navigiere mit T und Umschalt+T zwischen den Tabellen und drücke die Eingabetaste, um eine Tabelle in einer Webansicht anzuzeigen.
* Eine grundlegende Web-Rendering-Funktion hinzugefügt! Drücke Strg+Umschalt+V, um den aktuellen Abschnitt des Dokuments in einem webbasierten Renderer zu öffnen – nützlich für Inhalte wie komplexe Formatierungen oder Code-Beispiele.
* Eine russische Übersetzung wurde hinzugefügt – danke an Ruslan Gulmagomedov!
* Im Dialog „Alle Dokumente“ wurde eine Schaltfläche „Alle löschen“ hinzugefügt.
* Der Update-Checker zeigt nun Versionshinweise an, wenn eine neue Version verfügbar ist.
* Das Wiederherstellen des Fensters aus der Taskleiste wurde korrigiert.
* Die Übersetzungen der „Ja/Nein“-Schaltflächen in Bestätigungsdialogen wurden korrigiert.
* Das Laden von Konfigurationen bei Ausführung als Administrator wurde korrigiert.
* Die Verarbeitung von Kommentaren in XML- und HTML-Dokumenten wurde korrigiert.
* Das Parsen des Inhaltsverzeichnisses in Epub-2-Büchern wurde korrigiert.
* Das Navigieren zum nächsten Eintrag mit demselben Buchstaben im Inhaltsverzeichnis wurde korrigiert.
* Der Suchdialog wurde korrigiert, damit er bei Verwendung der Schaltflächen „Weiter“/„Zurück“ nun korrekt ausgeblendet wird.
* Es wurde behoben, dass Epub-Inhaltsverzeichnisse gelegentlich zum falschen Eintrag führten.
* Verschiedene Probleme bei der Behandlung von Leerzeichen in XML-, HTML- und `pre`-Tags wurden behoben.
* Ein „Off-by-One“-Fehler bei der Link-Navigation wurde behoben.
* Es wurde behoben, dass einige Bücher am Zeilenende Leerzeichen hatten.
* Verschiedene Parser-Probleme wurden behoben.
* Lesezeichenbezogene Menüpunkte sowie die Elementliste werden nun korrekt deaktiviert, wenn kein Dokument geöffnet ist.
* Die Listenverarbeitung in verschiedenen Dokumentformaten wurde verbessert.
* Der Übersetzungs-Workflow für Mitwirkende wurde verbessert.
* Zahlreiche interne Umstrukturierungen: Der Großteil der Geschäftslogik der Anwendung wurde von C++ nach Rust verlagert, um die Leistung und Wartbarkeit zu verbessern.

### Version 0.6.1
* Unterstützung für passwortgeschützte PDFs hinzugefügt!
* Eine sehr einfache Funktion zum Springen zur vorherigen/nächsten Position hinzugefügt. Wenn bei einem internen Link die Eingabetaste gedrückt wird und der Cursor springt, wird diese Position nun gespeichert und es kann mit Alt+Pfeil nach links/rechts dorthin navigiert werden.
* Eine Elementliste wurde hinzugefügt! Derzeit zeigt sie nur eine Baumstruktur aller Überschriften im Dokument oder eine Liste von Links an, es ist jedoch geplant, sie in Zukunft zu erweitern.
* Es wurde eine Option hinzugefügt, um Paperback standardmäßig im maximierten Modus zu starten.
* Es wurde behoben, dass Links in einigen EPUB-Dokumenten nicht richtig funktionierten.
* Das Parsen von EPUB-Inhaltsverzeichnissen mit relativen Pfaden wurde korrigiert.
* Es wurde behoben, dass bei einigen EPUB-Dokumenten kein Titel oder Autor angezeigt wurde.
* Es wurde behoben, dass die Titel einiger EPUB-Kapitel im Inhaltsverzeichnis-Dialogfeld nicht richtig angezeigt wurden.
* Es wurde behoben, dass die Leertaste nicht zum Aktivieren der OK-/Abbrechen-Schaltflächen im Inhaltsverzeichnis-Dialogfeld verwendet werden konnte.
* Die Verarbeitung von Überschriften in Word-Dokumenten wurde verbessert.
* Es erfolgt nun eine Sprachansage, wenn die Liste der zuletzt geöffneten Dokumente leer ist, wenn versucht wird, den Dialog aufzurufen.

### Version 0.6.0
* Im Optionsdialog wurde eine neue Option hinzugefügt, um das „Gehe zu“-Menü in einer wesentlich kompakteren Form anzuzeigen; diese ist standardmäßig aktiviert.
* Es wurde eine Option hinzugefügt, mit der die Navigation nach Strukturelementen umgebrochen wird.
* Im Menü „Extras“ wurde eine Option hinzugefügt, um den übergeordneten Ordner des aktuell fokussierten Dokuments zu öffnen.
* Ein recht einfaches, aber sehr effektives Aktualisierungssystem wurde hinzugefügt.
* Eine einfache Sleep-Timer-Funktion wurde hinzugefügt, die über Strg+Umschalt+S aufgerufen werden kann.
* Unterstützung für das Parsen von FB2-E-Books wurde hinzugefügt!
* Unterstützung für das Parsen von OpenDocument-Präsentationen wurde hinzugefügt!
* Unterstützung für das Parsen von OpenDocument-Textdateien hinzugefügt!
* Lesezeichen können nun so gesetzt werden, dass sie eine ganze Zeile markieren oder nur einen bestimmten Textabschnitt. Wenn beim Setzen eines Lesezeichens keine Auswahl aktiv ist, verhält es sich wie vor Version 0.6 und markiert die gesamte Zeile. Wenn man jedoch Text auswählt, wird nur dieser Text im Lesezeichen enthalten sein.
* Lesezeichen können nun optional mit Textnotizen versehen werden! Mit N und Umschalt+N lässt sich zwischen Lesezeichen mit Notizen navigieren oder den Lesezeichendialog mit bestimmten Tastenkombinationen öffnen, wobei alle Lesezeichen, nur Notizen oder nur Lesezeichen ohne Notizen ausgewählt sind.
* Lesezeichen im Lesezeichendialog haben nun kein lästiges „Lesezeichen x“-Präfix mehr.
* EPUB-Bücher mit HTML-Inhalten, die als XML getarnt sind, werden nun korrekt verarbeitet.
* Das Laden großer Markdown-Dokumente wurde korrigiert.
* Es wurde behoben, dass das Drücken der Leertaste in der Baumansicht des Inhaltsverzeichnisses die OK-Schaltfläche aktivierte.
* Die Behandlung von Leerzeichen am Anfang von `pre`-Tags wurde sowohl in HTML- als auch in XHTML-Dokumenten korrigiert.
* Es wurde behoben, dass das Textfeld manchmal nicht wieder den Fokus erhielt, wenn man zum Paperback-Fenster zurückkehrte.
* Es wurde behoben, dass das Textfeld im Dialog „Zu Prozent springen“ den Wert des Schiebereglers nicht aktualisierte.
* Die Darstellung benutzerdefinierter HTML-IDs in Markdown-Dokumenten wurde korrigiert.
* HTML innerhalb von Markdown-Codeblöcken wird nun korrekt dargestellt.
* Wenn ein Buch mit einem Befehlszeilenparameter geladen wird, während bereits eine Paperback-Instanz läuft, erhält man keinen Fehler mehr, falls das Laden des Dokuments länger als 5 Sekunden dauert.
* Wird Paperback als Administrator ausgeführt, wird die Konfiguration nun korrekt geladen und gespeichert.
* Es ist nun möglich, ein Lesezeichen direkt aus dem Lesezeichen-Dialog heraus zu löschen.
* Es ist nun möglich, Lesezeichen und die Leseposition für ein bestimmtes Dokument zu importieren und zu exportieren. Die generierte Datei wird nach der Datei benannt und hat die Endung .paperback. Wird beim Laden einer Datei eine solche Datei im selben Verzeichnis gefunden, wird sie automatisch geladen. Ansonsten kann sie manuell über einen Eintrag im Menü „Extras“ importiert werden.
* Links innerhalb von Dokumenten werden jetzt vollständig unterstützt! Mit „k“ und „Umschalt+k“ kann man vor- und zurückblättern, und mit der Eingabetaste wird ein Link geöffnet bzw. aktiviert.
* Viele interne Umstrukturierungen, die die App schneller und die Binärdatei kleiner machen.
* Markdown-Inhalte werden nun vor der Darstellung vorverarbeitet, um CommonMark-konform zu sein.
* Die Navigation über Listen und deren Einträge wird nun vollständig unterstützt! Die Navigation erfolgt mit „L“ und „Umschalt+L“, um durch die Listen selbst zu blättern, und „I“ sowie „Umschalt+I“, um durch die Listeneinträge zu blättern.
* Die Löschtaste auf dem Ziffernblock funktioniert nun zusätzlich zur normalen Löschtaste, um Dokumente aus der Registerkartenleiste zu entfernen.
* Paperback lässt sich nun optional in die Taskleiste minimieren! Diese Option ist standardmäßig deaktiviert, aber bei aktivierter Option wird Paperback über die Minimieren-Option im Systemmenü in die Taskleiste verschoben und kann durch Klicken auf das angezeigte Symbol wiederhergestellt werden.
* Paperback ist nun vollständig übersetzbar! Die Liste der unterstützten Sprachen ist derzeit noch recht klein, wächst aber stetig!
* Paperback hat jetzt eine offizielle Website unter [paperback.dev](https://paperback.dev)!
* PPTX-Dokumente zeigen nun ein einfaches Inhaltsverzeichnis an, das alle Folien enthält.
* Der vollständige Pfad zum geöffneten Dokument wird nun im Dokument-Info-Dialog angezeigt.
* Das Installationsprogramm enthält jetzt eine Option, um die Readme-Datei nach der Installation im Browser anzuzeigen.
* Die Liste der zuletzt geöffneten Dokumente wurde erheblich erweitert! Anstatt nur die letzten 10 geöffneten Dokumente anzuzeigen, wird nun eine anpassbare Anzahl angezeigt, wobei über einen kleinen Dialog auf alle anderen Dokumente zugegriffen werden kann, die jemals geöffnet wurden.
* Verschiedene kleine Verbesserungen an den Parsern insgesamt, darunter das Einfügen einer Leerzeile zwischen den Folien in PPTX-Präsentationen, die Korrektur der Zeilenumbruchbehandlung innerhalb von Absätzen in Word-Dokumenten und das Hinzufügen von Aufzählungspunkten zu Listenelementen.

### Version 0.5.0
* Unterstützung für Microsoft Word-Dokumente hinzugefügt!
* Unterstützung für PowerPoint-Präsentationen hinzugefügt!
* Es wurde behoben, dass bestimmte Menüpunkte nicht deaktiviert wurden, wenn keine Dokumente geöffnet waren.
* Die Ausrichtung des Schiebereglers „Prozent“ wurde korrigiert.
* Das Inhaltsverzeichnis in EPUB-Büchern mit URL-kodierten Dateipfaden und/oder Fragment-IDs wurde korrigiert.
* Es wurde behoben, dass Leerzeichen auf seltsame Weise aus XHTML-Überschriften entfernt wurden.
* Die Behandlung von Leerzeichen innerhalb verschachtelter `pre`-Tags in HTML-Dokumenten wurde korrigiert.
* HTML- und Markdown-Dokumente unterstützen jetzt die Inhaltsverzeichnis-Funktion! Wenn ein HTML-/Markdown-Dokument geladen wird, erstellt Paperback anhand der Struktur der Überschriften im Dokument ein eigenes Inhaltsverzeichnis und zeigt es im Strg+T-Dialog an.
* HTML-Dokumente erhalten nun den im „title“-Tag festgelegten Titel, sofern vorhanden. Andernfalls wird weiterhin der Dateiname ohne Erweiterung verwendet.
* Wir sind von UniversalSpeech auf die Verwendung einer Live-Region zur Sprachausgabe umgestiegen. Das bedeutet, dass keine Screenreader-DLLs mehr mit dem Programm ausgeliefert werden und nun mehr Screenreader unterstützt werden, wie zum Beispiel Microsoft Narrator.
* Die ZIP-Bibliotheken wurden gewechselt, um das Öffnen einer größeren Auswahl an EPUB-Büchern zu ermöglichen.
* Der Dialog, in dem gefragt wird, ob das Dokument als reiner Text geöffnet werden soll, wurde komplett überarbeitet und ermöglicht es nun, das Dokument als reinen Text, HTML oder Markdown zu öffnen.
* Der Dialog „Zu Prozent springen“ enthält nun ein Textfeld, in das manuell ein Prozentsatz eingegeben werden kann, zu dem man springen möchte.
* Der HTML-Parser erkennt nun „dd“, „dt“ und „dl“ als Listenelemente.
* Das Inhaltsverzeichnis in EPUB-Büchern wird wieder exakt beibehalten.
* Das Unicode-Schusstrichzeichen wird nun beim Entfernen von Leerzeilen berücksichtigt.
* Es wird nicht mehr bei jedem Laden einer nicht erkannten Datei gefragt, wie sie geöffnet werden soll, sondern nur noch beim ersten Mal.

### Version 0.4.1
* Dem Installationsprogramm wurde ein optionales Startmenü-Symbol hinzugefügt.
* Das Inhaltsverzeichnis sollte nun in einigen Fällen übersichtlicher sein; wenn beispielsweise ein untergeordnetes und ein übergeordnetes Element mit demselben Text an derselben Position existiert, wird nun nur noch das übergeordnete Element angezeigt.
* Das Inhaltsverzeichnis in bestimmten CHM-Dokumenten wurde korrigiert.
* Das Inhaltsverzeichnis in EPUB-3-Büchern mit absoluten Pfaden wurde korrigiert.
* CHM-Dokumente sollten nun den Titel anzeigen, der in der Metadaten-Datei festgelegt ist.

### Version 0.4.0
* Unterstützung für CHM-Dateien hinzugefügt!
* Lesezeichen-Unterstützung hinzugefügt! Man kann so viele Lesezeichen in so vielen Dokumenten setzen, wie man möchte. Mit „b“ und „Umschalt+b“ lässt sich zwischen ihnen hin- und herspringen, mit „Strg+Umschalt+b“ ein Lesezeichen setzen und mit „Strg+b“ einen Dialog aufrufen, um zu einem bestimmten Lesezeichen zu springen.
* Neben der portablen ZIP-Datei wurde ein Installationsprogramm hinzugefügt! Das Installationsprogramm installiert Paperback im „Programme“-Ordner und richtet die Dateizuordnungen automatisch ein.
* Textdateien mit BOMs sollten nun korrekt dekodiert werden, und die BOM wird auch nicht mehr am Anfang des Textes angezeigt.
* Die Statusleiste wurde um weitaus mehr Informationen erweitert. Sie zeigt nun die aktuelle Zeile, das aktuelle Zeichen und den Lesefortschritt in Prozent an.
* HTML-Kommentare sowie der Inhalt von Skript- und Stil-Tags werden in der Textausgabe nicht mehr angezeigt.
* Wenn Paperback über die Befehlszeile ein relativer Pfad übergeben wird, wird dieser nun korrekt aufgelöst.
* Die prozentuale Navigation wird nun über einen eigenen, auf einem Schieberegler basierenden Dialog abgewickelt, der mit Strg+Umschalt+g aufgerufen werden kann.
* Dokumente ohne bekannten Titel oder Autor erhalten nun immer einen Standardwert.
* Die Logik zum Speichern der Position ist nun viel intelligenter und sollte nur dann auf die Festplatte schreiben, wenn es absolut notwendig ist.
* Das Dokument, das beim Schließen von Paperback im Fokus war, wird nun auch nach einem Neustart der Anwendung beibehalten.
* Eingaben in den Dialogen „Zur Zeile springen“ und „Zur Seite springen“ sollten nun strenger überprüft werden.
* Die Navigation im Inhaltsverzeichnis von EPUB-3-Büchern mit relativen Pfaden in ihren Manifesten wurde korrigiert.

### Version 0.3.0
* Das Inhaltsverzeichnis in EPUB-Büchern mit URL-kodierten Manifesten wurde korrigiert.
* Die Navigation durch Überschriften in HTML-Dokumenten, die Multibyte-Unicode-Zeichen enthalten, wurde korrigiert.
* Die hohe CPU-Auslastung in Dokumenten mit langen Titeln, die auf eine Regression in wxWidgets zurückzuführen war, wurde behoben.
* Das Laden von UTF-8-Textdateien wurde korrigiert.
* Es wurde behoben, dass verschachtelte Inhaltsverzeichnis-Einträge in EPUB-Büchern den Cursor an die falsche Position setzten.
* Ein Absturz beim Beenden der App in bestimmten Fällen wurde behoben.
* Im Optionsdialog wurde ein Kontrollkästchen hinzugefügt, um den Zeilenumbruch zu aktivieren oder zu deaktivieren!
* Es ist nun möglich, für die Entwicklung von Paperback zu spenden, entweder über den neuen Punkt „Spenden“ im Hilfemenü oder über den Link „Dieses Projekt sponsern“ unten auf der Hauptseite des GitHub-Repositorys.
* Markdown-Dokumente haben nun immer einen Titel, und Paperback sollte nun praktisch jede Markdown-Datei laden können.
* PDF-Dokumente haben nun immer einen Titel, auch wenn die Metadaten fehlen.
* Die PDF-Bibliotheken wurden auf die in Chromium verwendete umgestellt, was zu einer deutlich zuverlässigeren PDF-Auswertung in allen Bereichen führt.
* Es kann jetzt nur noch eine Instanz von Paperback gleichzeitig ausgeführt werden. Wenn „paperback.exe“ mit einem Dateinamen ausgeführt wird, während das Programm bereits läuft, wird dieses Dokument in der bereits laufenden Instanz geöffnet.
* Man kann jetzt die Entf-Taste bei einem Dokument im Registerkarten-Control drücken, um es zu schließen.

### Version 0.2.1
* Die Gesamtseitenzahl wurde zur Seitenbezeichnung im Dialog „Zur Seite springen“ hinzugefügt.
* Es kann nun mit der Tabulatortaste vom Dokumentinhalt zur Liste geöffneter Dokumente gewechselt werden.
* Es wurde behoben, dass die Tastenkombinationen für Überschriften manchmal zuletzt geöffnete Dokumente öffneten, wenn es genug davon gab.
* Paperback entfernt nun unnötige weiche Bindestriche aus der Textausgabe.
* Es wurde behoben, dass die Navigation über Überschriften manchmal zum falschen Zeichen führte.

### Version 0.2.0
* Unterstützung für Markdown-Dokumente hinzugefügt!
* Unterstützung für PDF-Dokumente hinzugefügt, einschließlich der Möglichkeit, zwischen den Seiten zu navigieren!
* Tastenkombinationen für die Navigation über Überschriften in HTML-Inhalten hinzugefügt, einschließlich EPUB-Büchern und Markdown-Dokumenten. Diese Tastenkombinationen wurden so konzipiert, dass sie ähnlich wie ein Screenreader funktionieren.
* Das Laden von EPUBs mit URL-kodierten Dateinamen in ihren Manifesten wurde korrigiert.
* Das Laden von EPUB-3-Büchern mit darin eingebettetem XHTML wurde korrigiert.
* Es wird nun eine Ansage ausgegeben, wenn das Dokument kein Inhaltsverzeichnis oder keine Abschnitte unterstützt, anstatt die Menüoptionen zu deaktivieren.
* Ein Menü „Zuletzt geöffnete Dokumente“ wurde hinzugefügt! Es speichert derzeit die letzten 10 geöffneten Dokumente, und wenn bei einem davon die Eingabetaste gedrückt wird, wird es zum Lesen geöffnet.
* Der Suchdialog wurde komplett überarbeitet, sodass er nun viel einfacher zu bedienen ist. Außerdem wurden ein Verlauf der letzten 25 Suchanfragen und die Unterstützung für reguläre Ausdrücke hinzugefügt!
* Zuvor geöffnete Dokumente bleiben nun auch nach einem Neustart der Anwendung erhalten. Dies lässt sich über den neuen Optionseintrag im Menü „Extras“ konfigurieren.
* Die Tastenkombination Umschalt+F1 wurde hinzugefügt, um die Readme-Datei direkt in Paperback selbst zu öffnen.

### Version 0.1.0
* Erste Veröffentlichung.
