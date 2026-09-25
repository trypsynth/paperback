<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

# Paperback – Version 1.0

## Einführung

Paperback ist ein leichter, schneller und barrierefreier Reader für E-Books, Dokumente und Hörbücher für alle – von Gelegenheitslesern bis hin zu Power-Usern. Das Programm wurde für Barrierefreiheit mit Bildschirmlesern, hohe Geschwindigkeit und ein aufgeräumtes Benutzererlebnis konzipiert.

## Systemanforderungen

Paperback läuft auf Windows 10/11, allen modernen Versionen von ARM macOS, Linux, iOS 17 und höher sowie Android 7 und höher. Die iOS- und Android-Apps sind im App Store und Google Play verfügbar.

## Funktionen

* Vollständig eigenständig, keine zusätzliche Software ist erforderlich, um zu beginnen.
* Unglaublich schnell, auch auf älterer Hardware.
* Einfache Registerkartenoberfläche, mit der Sie beliebig viele Dokumente nebeneinander öffnen können.
* Speichert Ihre genaue Leseposition für jedes geöffnete Dokument.
* Optional merkt sich das Programm, welche Dokumente Sie beim Schließen offen hatten, und stellt diese beim nächsten Start wieder her.
* Enthält Navigationsfunktionen, ähnlich denen im Web-Browsing-Modus vieler Bildschirmleser, um schnell und einfach durch Dokumente zu navigieren.
* Enthält einen robusten Suchdialog mit Funktionen wie Verlauf und Unterstützung für reguläre Ausdrücke.
* Kann vollständig tragbar ausgeführt oder mit automatisch eingerichteten Dateizuordnungen installiert werden.
* Unterstützt eine große Auswahl an gängigen Dateiformaten.
* Spielt Hörbücher mit verstellbarer Geschwindigkeit und Lesezeichen ab, die die genaue Zeit speichern.
* Liest gescannte PDF-Seiten mit der in Windows und macOS integrierten OCR.
* Lesezeichen und Notizen, damit Sie Ihren Ort markieren und später dorthin zurückkehren können.
* Jede Tastaturkombination kann geändert werden.
* Wird mit `pb`, einem Befehlszeilentool, geliefert, das jedes unterstützte Dokument in HTML, Markdown oder reine Text umwandelt.

## Kompatibilität mit Bildschirmlesern

Paperback funktioniert gut mit allen gängigen Bildschirmlesern. Es gibt jedoch zwei bekannte Probleme für JAWS-Benutzer.

### JAWS und Braille-Displays

Wenn Sie JAWS mit einem Braille-Display verwenden, kann es vorkommen, dass lange Absätze beim Blättern mit den Navigationstasten Ihres Displays abgeschnitten werden. Der Befehl zum Lesen des aktuellen Absatzes ist ebenfalls betroffen. Dies ist ein Fehler in JAWs Umgang mit dem RICHEDIT50W-Textsteuerelement und nicht etwas im Paperback selbst – ein Fehler, dessen Behebung lange dauerte, angesichts Visperos Begeisterung für die Reaktion auf Probleme mit Open-Source-Software.

Das Workaround, das nach Monaten des Wartens durch die JAWS-Diskussionsgruppe gefunden wurde, besteht darin, `paperback.jcf` zu bearbeiten und „Braille Presentation and Panning

## Tastenkombinationen

Paperback ist für die Verwendung mit der Tastatur ausgelegt. Hier sind die aktuellen Tastenkombinationen.

Die folgenden Tastenkombinationen gelten für Windows. Falls macOS abweicht, wird das Äquivalent in Klammern angegeben — hauptsächlich weil Ctrl+G, Ctrl+W und Alt+Left/Right auf dieser Plattform bereits von anderen System- oder App-Konventionen belegt sind.

### Datei-Menü

* `Ctrl+O`: Ein Dokument öffnen.
* `Ctrl+F4` (macOS: `Cmd+W`): Das aktuelle Dokument schließen.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle offenen Dokumente schließen.
* `Ctrl+Shift+T`: Das zuletzt geschlossene Dokument erneut öffnen.
* `Ctrl+R`: Den Dialog "Alle Dokumente" anzeigen (aus den zuletzt verwendeten Dokumenten).
* `Ctrl+Q`: Beenden (nur Windows; auf macOS befindet sich dies stattdessen im App-Menü).

### Navigations-Menü

* `Ctrl+F`: Den Suchedialog anzeigen.
* `F3` (macOS: `Cmd+G`): Nächstes Vorkommen suchen.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorheriges Vorkommen suchen.
* `Ctrl+G` (macOS: `Cmd+L`): Zur Zeile springen.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Zum Prozentsatz springen.
* `Ctrl+P`: Zur Seite springen (wenn vom aktuellen Dokument unterstützt).
* `=`: Ihren aktuellen Leseprozentsatz und die Seite ankündigen, z. B. "15%, Seite 30". Die Seite wird bei Dokumenten ohne Seitenzahlen weggelassen.
* `Alt+Left` (macOS: `Cmd+[`): In der Navigationshistorie zurückgehen.
* `Alt+Right` (macOS: `Cmd+]`): In der Navigationshistorie vorwärts gehen.
* `[`: Vorheriger Abschnitt.
* `]`: Nächster Abschnitt.
* `Shift+H`: Vorheriges Überschrift.
* `H`: Nächstes Überschrift.
* `Shift+1` bis `Shift+6`: Vorherige Überschrift auf Ebene 1–6.
* `1` bis `6`: Nächste Überschrift auf Ebene 1–6.
* `Shift+P`: Vorherige Seite.
* `P`: Nächste Seite.
* `Shift+B`: Vorheriges Lesezeichen.
* `B`: Nächstes Lesezeichen.
* `/`: Temporäres Lesezeichen setzen.
* `\`: Zum temporären Lesezeichen springen.
* `Shift+N`: Vorherige Notiz.
* `N`: Nächste Notiz.
* `Ctrl+B`: Zu allen Lesezeichen und Notizen springen.
* `Ctrl+Alt+B`: Nur zu Lesezeichen springen.
* `Ctrl+Alt+M`: Nur zu Notizen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d. h. die physikalische Strg-Taste statt Cmd): Notiztext an der aktuellen Position anzeigen.
* `Shift+K`: Vorheriger Link.
* `K`: Nächster Link.
* `Shift+G`: Vorheriges Bild.
* `G`: Nächstes Bild.
* `Shift+F`: Vorherige Abbildung.
* `F`: Nächste Abbildung.
* `Shift+T`: Vorherige Tabelle.
* `T`: Nächste Tabelle.
* `Shift+M`: Vorherige Formel.
* `M`: Nächste Formel.
* `Shift+S`: Vorheriger Trennzeichen.
* `S`: Nächster Trennzeichen.
* `Shift+L`: Vorherige Liste.
* `L`: Nächste Liste.
* `Shift+I`: Vorheriges Listenelement.
* `I`: Nächstes Listenelement.
* `Shift+,`: Zum Anfang des aktuellen Containers (Liste oder Tabelle) gehen.
* `,`: Nach dem Ende des aktuellen Containers (Liste oder Tabelle) gehen.

### Werkzeuge-Menü

* `Ctrl+W` (macOS: `RawCtrl+W`, d. h. die physikalische Strg-Taste statt Cmd): Wortanzahl für das aktuelle Dokument anzeigen.
* `Ctrl+I`: Dokumentinfo anzeigen.
* `Ctrl+T`: Inhaltsverzeichnis anzeigen.
* `F7`: Elementliste anzeigen.
* `Ctrl+Shift+C`: Enthaltenden Ordner öffnen.
* `Ctrl+Shift+V`: Aktuellen Inhalt in Web View öffnen.
* `Ctrl+U`: Dokumentquelle in einem neuen Tab anzeigen.
* `Ctrl+Shift+E`: Dokumentdaten exportieren (`.paperback`).
* `Ctrl+Shift+I`: Dokumentdaten importieren (`.paperback`).
* `Ctrl+E`: Aktuelles Dokument als Klartext exportieren.
* `Ctrl+Shift+B`: Lesezeichen an der aktuellen Auswahl/dem Cursor umschalten.
* `Ctrl+Shift+N`: Lesezeichennotiz an der aktuellen Auswahl/dem Cursor hinzufügen oder bearbeiten.
* `Ctrl+Alt+W`: Wortumbruch umschalten.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, d. h. die physikalische Strg-Taste, da Cmd+Space Spotlight öffnet): Audioerzählung wiedergeben/pausieren.
* `'`: Audioerzählung vorwärts spulen.
* `;`: Audioerzählung rückwärts spulen.
* `Shift+'`: Spulbetrag der Audioerzählung erhöhen.
* `Shift+;`: Spulbetrag der Audioerzählung verringern.
* `Ctrl+Shift+.`: Audioerzählung beschleunigen.
* `Ctrl+Shift+,`: Audioerzählung verlangsamen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d. h. Strg+Befehl+F): Vollbildmodus umschalten.
* `Ctrl+,`: Einstellungen öffnen (macOS: im App-Menü).
* `Ctrl+Shift+S`: Schlaf-Timer umschalten.
* `Ctrl+Shift+O`: Eine Reihe gescannter PDF-Seiten mit OCR erkennen.
* `Alt+F9` (macOS: `Cmd+F9`): Den Anfang einer Auswahl markieren, um alles von hier bis zu der Stelle, zu der Sie gelangen, auf einmal kopieren zu können.
* `Alt+F10` (macOS: `Cmd+F10`): Alles vom markierten Anfang der Auswahl bis zur aktuellen Position kopieren.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Zum markierten Anfang der Auswahl zurückspringen und die Markierung beibehalten.

### Hilfemenü

* `Ctrl+F1`: Dialog "Über" anzeigen.
* `F1`: Hilfe in Ihrem Standardbrowser anzeigen.
* `Shift+F1`: Hilfe in Paperback anzeigen.
* `Ctrl+Shift+U`: Nach Updates suchen.
* `Ctrl+D`: Spendenseite in Ihrem Standardbrowser öffnen.

### Zusätzliche Tasten in der Dokumentansicht

* `Delete` / `Numpad Delete` in der Tab-Steuerung: Die ausgewählte Dokumentregisterkarte schließen.
* `Enter` oder `Space` im Dokumenttext: Einen Link folgen oder eine Tabellen- oder Formelansicht am Cursor öffnen.
* `Enter` auf einer gescannten PDF-Seite: Die Seite mit OCR erkennen.
* `Shift+F10` oder die Menü-/Anwendungstaste im Dokumenttext: Das Kontextmenü öffnen.

## iOS und Android

Die iOS- und Android-Apps nutzen dasselbe Lesemodul wie die Desktop-Version, daher öffnen sie dieselben Formate und merken sich deine Position auf dieselbe Weise. Sie sind für die Verwendung mit VoiceOver unter iOS und TalkBack unter Android konzipiert.

### Dokumente öffnen

* Nutze die Schaltfläche „Buch öffnen

## Credits
### Development
* Quin Gillespie: Hauptentwickler und Projektgründer.
* Aryan Choudhary: Hauptmitwirkender.

### Donations
Die folgenden Personen haben Spenden verschiedener Größe zur Entwicklung von Paperback geleistet. Wenn Sie eine Spende tätigen, wird Ihr Name nicht automatisch hier hinzugefügt. Ich füge nur Personen hinzu, die möchten, dass ihre Spende öffentlich bekannt gemacht wird.

Hinweis: Ich betrachte einen öffentlichen GitHub-Sponsor als automatischen Grund für die Aufnahme in diese Liste.

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

## Changelog

### Version 1.0

1.0 ist die erste Veröffentlichung auf allen fünf Plattformen: Windows, macOS, Linux, iOS und Android, mit den iOS- und Android-Apps im App Store und Google Play.

#### Added

##### General
* Linux-Unterstützung als AppImage oder tar.gz mit Desktop-Integration, sodass Dokumente aus Ihrem Dateimanager geöffnet werden.
* Markieren Sie den Anfang einer Auswahl mit `Alt+F9`, kopieren Sie alles von dort bis zu Ihrer aktuellen Position mit `Alt+F10`, und kehren Sie zur Markierung mit `Alt+Shift+F9` zurück, um lange Textabschnitte zu kopieren, ohne sich durch sie hindurch zu pfeilen. Alle drei befinden sich unter Tools > Select and copy.
* Die Verknüpfung `=` kündigt nun neben dem Prozentsatz auch die Seite an, z. B. "15%, page 30", und bleibt wie zuvor für Dokumente ohne Seitenzahlen.
* Das Infofeld zeigt nun die Lizenz von Paperback und jeden Übersetzer.
* Eine ukrainische Übersetzung.

##### New Formats
* Comic-Book-Archive (`.cbz`).
* M4B-Hörbücher, aufgeteilt in ihre Kapitel.
* Man-Pages, sowohl `man` als auch BSD `mdoc`, gezippt oder nicht.
* MP3-Hörbücher, aufgeteilt in Kapitel, wenn die Datei diese enthält.
* reStructuredText-Dokumente.
* Windows-Write-Dateien (`.wri`).
* WinHelp-Dateien (`.hlp`).
* Word-6- und Word-95-Dokumente.

##### OCR
* Gescannte PDF-Seiten können nun mit der in Windows und macOS integrierten OCR erkannt werden. Drücken Sie `Enter` auf einer gescannten Seite, um sie zu erkennen, oder verwenden Sie Batch OCR (`Ctrl+Shift+O`) für einen Seitenbereich.

##### Navigation
* MathML-Formeln in EPUB und HTML werden mit MathCAT als AsciiMath dargestellt. Verwenden Sie `M` oder `Shift+M`, um durch Formeln zu navigieren, dann `Enter` oder `Space`, um das Original-MathML in Formula View zu öffnen.
* Eine Find-All-Schaltfläche im Find-Dialog, die jede Zeile mit einem Treffer auflistet, damit Sie direkt zur gewünschten springen können.
* Tables-, Lists- und Pages-Ansichten in der Elementliste (`F7`).
* Go to Line, Go to Page und Go to Percent akzeptieren nun `+n` und `-n`, um relativ zu Ihrer aktuellen Position zu bewegen.
* EPUB-, MOBI- und CHM-Bücher ohne eigene Überschriften erhalten nun Navigationen von ihrem Inhaltsverzeichnis.
* KF8- (AZW3-) Bücher unterstützen nun Abschnittsnavigation.
* EPUB-Seiten, die nur ein Bild sind, zeigen nun eine Zeile dafür an, sodass Sie auf ihnen landen können, anstatt einfach vorbeizuspringen.

##### Audio Books
* Steuerelemente für Wiedergabegeschwindigkeit von halber Geschwindigkeit bis dreifacher Geschwindigkeit. Verwenden Sie `Ctrl+Shift+.` und `Ctrl+Shift+,` oder das Tools-Menü.
* Lesezeichen und Notizen in reinen Audio-Büchern merken sich nun die genaue Zeit, an der Sie sie gesetzt haben.
* Nächste und vorherige Position (`Alt+Left` und `Alt+Right`) funktionieren nun in Audio-Büchern.
* Der Fortschritt durch ein Hörbuch wird nun anhand seiner Aufnahme gemessen, sodass Go to Percent und die Statusleiste übereinstimmen, wie weit Sie wirklich sind.

##### Recent Documents
* Ein Clear Recent Documents-Element im Submenu Recent Documents.

##### PDF Documents
* Eine Einstellung, um jede Zeile einer PDF separat zu halten, anstatt sie in Absätze einzufügen.
* Bilder und Abbildungen in PDFs werden nun angekündigt.
* PDFs, die Lesestruktur haben, aber keine ihrer Bilder kennzeichnen, kündigen diese Bilder nun an, anstatt sie ganz aus dem Buch auszulassen.

##### Web View
* Jedes Dokument kann nun in der Web-Ansicht geöffnet werden, nicht nur EPUB, HTML und Markdown.

##### Readability
* Überschriften werden nun in einer Größe gezeichnet, die ihrer Ebene entspricht, und Bilder und Tabellen sind vom umgebenden Text getrennt.

##### pb
* `pb --list-formats` listet jedes Format auf, das pb lesen kann.
* pb sagt nun, welche Datei es nicht lesen konnte und warum.

#### Behoben

##### Allgemein
* Ein beim Start erneut geöffnetes Buch wird nun gleich abgespielt, anstatt stumm zu bleiben, bis es geschlossen und erneut geöffnet wurde.
* Ein Dokument, dessen Datei nicht mehr vorhanden ist, kann nun aus „Alle Dokumente

### Version 0.9.2
* Audiobücher lassen Ihre Bildschirmlese-Software nicht mehr eine Reihe von Leerzeichen vorlesen, wenn Sie das Textfeld fokussieren.
* Audiobücher benennen nun die Datei, während Sie sie abschnittweise durchlaufen.
* Audiobücher melden nun ihre echte Länge, anstatt zu behaupten, dass jede Datei 24 Stunden läuft.
* Das Schließen der Webansicht mit Escape wirft nach dem Folgen eines Links nicht mehr eine Debug-Warnung auf.
* Das Kopieren nach „Alles auswählen

#### Hinzugefügt

##### Allgemein
* Ein CLI-Tool namens pb, um schnell alle unterstützten Formate von Paperback in HTML, Markdown oder Klartext zu konvertieren.
* Eine Option zum Neuladen von Dokumenten, die von anderen Programmen auf der Festplatte geändert wurden.
* Eine Option zum Anzeigen der Quelle, um die Quelle eines Dokuments in einem neuen Tab zu öffnen, nützlich beispielsweise zum Bearbeiten von Markdown.
* Dokumenttext ist nun paginiert, was bedeutet, dass Sie Bücher mit Millionen von Wörtern in nur wenigen Sekunden laden können. Bitte melden Sie alle Anomalien, die Sie damit finden.

##### Plattformunterstützung
* ARM64-Windows-Unterstützung!
* Native macOS-Unterstützung!
* Ein Vollbildumschalter.

##### Dialog "Alle Dokumente"
* Eine Schaltfläche zum Lokalisieren, um fehlende Bücher zu finden, die gerade ihren Pfad geändert haben.
* Ein Statusfilter und eine Statusleiste, mit denen Sie nach Dokumentstatus filtern und sehen können, wie viele Dokumente angezeigt und ausgewählt sind.
* Die Tastenkombination `Ctrl+Shift+A` zum Deselektieren aller Dokumente.

##### Optionen und Lesbarkeit
* Ein Reiter zur Lesbarkeit mit den folgenden Optionen:
    * Wortumbruch (aus "Allgemein" verschoben);
    * Tabellen inline rendern (neu in dieser Version, siehe unten);
    * Schriftart;
    * Hintergrundfarbe;
    * Zeilenabstand;
    * Absatzabstand;
    * Buchstabenabstand;
    * Textausrichtung.
* Ein Menüelement für Wortumbruch und eine anschließende Tastenkombination.
* Ein Umschalter, um zu bestimmen, wie Tabellen angezeigt werden sollen, und einheitliche Anzeige von Tabellen über alle Dokumente hinweg.

##### Navigation
* Unterstützung für die Navigation nach Container.
* Eine Option, um den Cursor beim Navigieren zwischen Zeilen automatisch an den Anfang der Zeile zu verschieben, ähnlich wie der Browsermodus in Bildschirmleseprogrammen.
* Die Tastenkombination Gleichheitszeichen, um Ihren aktuellen Prozentsatz im Dokument anzusagen.

##### Lesezeichen
* Temporäre Lesezeichen: Sie können eines pro Dokument haben, und diese bleiben erhalten. Verwenden Sie Schrägstrich, um einen zu setzen, und umgekehrten Schrägstrich, um zu diesem zu springen.

##### Wortanzahl
* Geschätzte Lesezeit im Dialog "Wortanzahl" sowie die Möglichkeit, Ihre Lesegeschwindigkeit festzulegen, um diese Metrik tatsächlich nützlich zu machen.
* Wenn eine Auswahl aktiv ist, wenn Sie den Dialog "Wortanzahl" öffnen, wird nun angezeigt, wie viele Wörter Sie ausgewählt haben.

##### Tastaturkürzel
* Die Möglichkeit, alle Tastaturkürzel in der App über einen einfachen Dialog anzupassen.
* Ein anpassbares Tastaturkürzel, um Paperback aus der Taskleiste wiederherzustellen.

##### Sprachen
* Niederländisch, Finnisch und Polnisch.

##### Export
* Das Menüelement "Exportieren" wurde erweitert, um neben Klartext auch den Export als HTML und Markdown zu ermöglichen.

##### Updater
* Eine Schaltfläche "Abbrechen" zum Dialog "Update läuft".
* Der Updater validiert nun, dass die heruntergeladene Datei nicht manipuliert wurde.

##### Webansicht
* Die Webansicht wird nun an Ihrer aktuellen Leseposition geöffnet.

##### DAISY-Bücher
* Unterstützung für DAISY-2.0-Bücher.
* Unterstützung für DAISY-2.02-Audiowiedergabe.

##### Hörbücher
* Die Möglichkeit, Hörbücher abzuspielen, derzeit mit Unterstützung von DAISY-Audio (einschließlich DAISY-Audio + Text) und ZIP-Dateien mit Audiodateien.
* Tastaturkürzel und Menüelemente zum Abspielen/Pausieren der Erzählung, zum Vor- und Zurückspulen sowie zum Anpassen der Spulmenge.
* Optionen zum Synchronisieren der Lesemarke mit der Audiowiedergabe, zum Festlegen der Audiospulmenge und zum Auswählen, ob das Spulen über das Ende eines Kapitels hinaus in das nächste Kapitel fortgesetzt wird.

##### CHM-Dokumente
* Unterstützung für Listen, Listenelemente, Abbildungen und Bilder.

##### PowerPoint
* PowerPoint-Dokumente unterstützen nun Tabellen.

#### Behoben

##### Allgemein
* Dokumente mit Kodierung in älteren CJK-Zeichensätzen wie GBK, Big5 und Shift_JIS werden nun korrekt dargestellt statt als Zeichensalat.
* "Zuletzt geschlossene wiederherstellen" versuchte, die gebündelte Readme zu öffnen.
* Der ausgewählte Tab erhielt nach dem Neustart von Paperback keinen korrekten Fokus.
* Paperbacks Verarbeitung von Dateien auf Windows-Netzlaufwerken: Das Anzeigen der Datei im Ordner fokussiert nun die Datei auf dem Netzwerkspeicher korrekt, und die Pfade enthalten keine seltsamen Zeichen mehr.
* .paperback-Dateien werden bei der Dokumentwiederherstellung nicht mehr erzwungen geladen; stattdessen werden Sie um Bestätigung gefragt, wenn eine gefunden wird.
* "Enthaltenden Ordner öffnen" fokussiert die angegebene Datei nun im Explorer.
* Das Öffnen der Readme berücksichtigt nun die gewählte Sprache.
* Die Benutzeroberfläche von Paperback wird nun auf hochauflösenden Displays korrekt skaliert.
* Das Menü wird nun korrekt aktualisiert und der Fokus wechselt zum Textsteuerelement, wenn die Hilfe in Paperback geöffnet wird.
* Wechsel zu einer viel sichereren Methode der IPC unter Windows.
* Der aktive Dokumenttitel wird nun vorgelesen, wenn zwischen Registerkarten gewechselt wird.
* Speichernutzung bei großen Dokumenten reduziert, indem die Größe der internen Zeichenindextabellen halbiert wurde.

##### Dialog "Alle Dokumente"
* Escape schloss die Dialoge "Dokumentinfo" und "Alle Dokumente" nicht.
* Die Titelleiste wurde nach dem Schließen eines Dokuments aus dem Dialog "Alle Dokumente" nicht aktualisiert.
* Readme.html wird nicht mehr zu Ihrer Liste "Alle Dokumente" hinzugefügt, wenn es über `Shift+F1` geöffnet wird.
* Das Entfernen von Dokumenten aus dem Dialog "Zuletzt verwendet" schließt nun auch deren aktiven Tab.
* Der Suchfilter wird nun nach dem Entfernen eines Dokuments beibehalten.

##### Navigation
* Seitennavigation gab in einigen Situationen falschen Zeilentext an.
* "Zu Zeile", "Zu Seite" und "Zu Prozent" platzierten den Cursor bei großen Dokumenten an der falschen Position.
* "Suchen" und "Nächste suchen" berücksichtigten das geladene Dokumentfenster bei großen Dokumenten nicht.

##### Lesezeichen
* Lesezeichen-/Notiztöne sollten nun nur noch abgespielt werden, wenn Sie über ein Wort mit einem Lesezeichen navigieren.

##### Lesbarkeit
* Das Anwenden von Zeilenumbruch versendete Sie zum Anfang des Dokuments.

##### Web-Ansicht
* Der Webview-Dialog war nicht in der Größe veränderbar und wurde in sehr kleiner Größe angezeigt.
* Bilder sollten nun in der eingebetteten Webansicht korrekt angezeigt werden.

##### Updater
* Der Updater zeigt nun den Inhalt von Markdown-Code-Tags in Versionshinweisen korrekt an.

##### DAISY-Bücher
* DAISY-Bücher zeigten in der Statusleiste falsche Informationen an.
* Laden von DAISY-Büchern mit falschen Zeichenkodierungsdeklarationen.

##### RTF-Dokumente
* Analyse von RTF-Dokumenten mit nicht-lateinischen Zeichen.
* RTF-Gruppen `\pict`, damit eingebettete Bilddaten nicht mehr in den Dokumenttext gelangen.

##### Mobi/AZW3-Bücher
* Filepos-Anker in Mobi-Büchern, die HTML-Tags aufteilen und Müll in den Buchtext einbringen.
* Links in älteren Mobi-Büchern.
* Erheblich verbesserte AZW3-Analyse.

##### Word-Dokumente
* Word-Dokumente mit gebietsspezifischen Stilnamen zeigten ihre Überschriften nicht korrekt an.

##### HTML/XHTML-Dokumente
* dl-, dt- und dd-Elemente erzeugten in XHTML-Dokumenten keine Zeilenumbrüche.

##### PDF-Dokumente
* Paperback greift nun auf einfache Textextraktion für falsch gekennzeichnete PDFs zurück.
* PDF-Dokumente mit Steuerzeichen in ihren Titeln und/oder Lesezeichen stürzen Paperback beim Öffnen nicht mehr ab.

### Version 0.8.5
* Seitenunterstützung für EPUB-Bücher hinzugefügt.
* Unterstützung für verschlüsselte Microsoft Office-Dokumente hinzugefügt. Derzeit werden ältere Word-, moderne Word- und moderne PowerPoint-Versionen unterstützt, wobei ältere PowerPoint-Versionen für die Zukunft geplant sind.
* Unterstützung für ältere Microsoft Word-Dokumente hinzugefügt!
* Unterstützung für ältere PowerPoint-Präsentationen hinzugefügt!
* Unterstützung für Mobi- und AZW3-Bücher hinzugefügt!
* Unterstützung für gekennzeichnete PDF-Dateien hinzugefügt!
* Tastenkombination `Ctrl+Q` zum Beenden der App hinzugefügt.
* Unterstützung für gezippte Bücher von Bookshare (sowohl DAISY als auch Word) hinzugefügt!
* Alt-Text für eingebettete Bilder sollte nun korrekt angezeigt werden.
* CHM-Dokumente unterstützen nun korrekt die Navigation mit internen Links.
* Fehler bei "Zu Seite" behoben, der um 1 falsch war.
* Fehler behoben, dass die Escape-Taste nicht zum Schließen des Dialogs "Als öffnen" funktionierte.
* Fehler behoben, dass das Kontextmenü des Lesers bei Rechtsklick oder der Anwendungstaste nicht angezeigt wurde.
* Fehler behoben, dass beim Öffnen von Dokumenten über die Befehlszeile manchmal das falsche Dokument fokussiert wurde.
* PDF-Dateien mit nur Bildern werden erneut erkannt und warnen Sie vor ihrer Existenz.
* Es ist nun möglich, mit `g`/`Shift+G` und `f`/`Shift+F` durch Bilder und Abbildungen zu navigieren.
* Paperback berücksichtigt nun die Einstellung Ihres Anwendungs-Dunkelmodus.
* DAISY XML-Unterstützung entfernt, da sie nicht mehr benötigt wird.
* Zurück zur nativen Win32-Navigation nach dem ersten Buchstaben im Inhaltsverzeichnisbaum.
* Der Fehlerladedialog zeigt nun detailliertere Fehlermeldungen an.
* Die Webansicht wird nun viel schneller und reibungsloser geöffnet.

### Version 0.8.2
* Seitenunterstützung für RTF-Dokumente hinzugefügt!
* Fehler behoben, bei dem das Öffnen der Webansicht in EPUBs mit externen Links diese automatisch aktivieren würde.
* Fehler behoben, bei dem der RTF-Parser in seltenen Fällen keinen Platz zwischen Wörtern einsetzen würde.
* Fehler behoben, bei dem Absätze in einigen PDF-Dokumenten in mehrere kurze Zeilen aufgeteilt wurden.
* PDF-Dokumente haben nun grundlegende Unterstützung für Link- und Überschriftennavigation!
* RTF-Tabulatoren und Zeilenumbrüche werden nun genau so dargestellt, wie sie im Dokument erscheinen.
* Zurück zur bewährten pdfium-Bibliothek zum Analysieren von PDFs, wodurch PDF-Rendering wieder viel zuverlässiger wird.

### Version 0.8.1
* `Ctrl+Shift+T` zum Wiederherstellen des zuletzt geschlossenen Dokuments hinzugefügt.
* Der Dialog "Alle Dokumente" unterstützt nun das gleichzeitige Auswählen mehrerer Dokumente zum Öffnen.
* Mehrere Fehler im RTF-Parser behoben.
* Fehler behoben, bei dem Dateipfade mit nicht-ASCII-Zeichen (wie bosnisches š, č, ć, ž) beschädigt wurden, wenn eine Datei über eine zweite Paperback-Instanz geöffnet wurde.
* Fehler behoben, dass PDF-Text in der falschen Reihenfolge vorgelesen wurde und falscher Abstand um großgeschriebene Wörter vorhanden war.
* Fehler behoben, das langsame Laden von Dokumenten beim Öffnen großer Dateien.
* Lokalisierung der Ja-/Nein-Schaltflächen in Bestätigungsdialogen behoben.

### Version 0.8.0
* Japanische, vereinfachte chinesische und vietnamesische Übersetzungen hinzugefügt!
* Einen automatischen Updater hinzugefügt, der nun deine aktuell installierte Version von Paperback ersetzt, anstatt nur die neue Version herunterzuladen!
* Optionales Soundfeedback beim Erreichen einer Lesezeichen- oder Notizmarke hinzugefügt, Dank an Andre Louis für die Sounds!
* RTF-Dokumentunterstützung hinzugefügt!
* Unterstützung für DAISY-XML-Dokumente hinzugefügt.
* Unterstützung für Flat Open Document Text-Dateien hinzugefügt!
* Unterstützung für Flat Open Document-Präsentationen hinzugefügt!
* Unterstützung für Trennzeichen mit `s` und `shift+s` hinzugefügt.
* Jede Bewegung größer als 300 Zeichen wird nun automatisch zu deinem Navigationsverlauf hinzugefügt.
* Fenster von Paperback aus dem Systemtray wiederherstellen behoben.
* Markdown-Dokumente zeigen nun korrekt gerendertes HTML in der Webansicht statt rohem Text an.
* Tabellen werden nun in Markdown-Dateien korrekt dargestellt.
* Reiner Bilder-PDFs warnen dich nun, wenn du versuchst, eine zu laden.
* Versionsinformationen korrekt in die ausführbare Datei von Paperback eingebettet.
* Optionsdialog in Reiter aufgeteilt für einfachere Bedienung und Navigation.
* Zu Hayro für PDF-Parsing gewechselt, was zu höherer Zuverlässigkeit, schnellerer Geschwindigkeit und weniger DLLs führt.
* Gesamte App in Rust neu geschrieben. Die neue Codebasis ist sicherer, lädt Dokumente schneller und ist einfacher zu warten und zu erweitern.
* Das Kontextmenü des Textfelds enthält nun leserspezifische Aktionen statt generischer Elemente wie Ausschneiden und Einfügen.

### Version 0.7.0
* Tabellenunterstützung für HTML- und XHTML-basierte Dokumente hinzugefügt! Navigiere zwischen Tabellen mit `T` und `Shift+T`, und drücke `Enter` um eine in einer Webansicht anzuzeigen.
* Eine grundlegende Webrendering-Funktion hinzugefügt! Drücke `Ctrl+Shift+V` um den aktuellen Abschnitt deines Dokuments in einem webbasierten Renderer zu öffnen, nützlich für Inhalte wie komplexe Formatierung oder Codebeispiele.
* Russische Übersetzung hinzugefügt, Dank an Ruslan Gulmagomedov!
* Eine Schaltfläche "Alles löschen" zum Dialog "Alle Dokumente" hinzugefügt.
* Der Update-Checker zeigt nun Versionsinformationen an, wenn eine neue Version verfügbar ist.
* Fenster aus dem Systemtray wiederherstellen behoben.
* Ja/Nein-Schaltflächenübersetzungen in Bestätigungsdialogen behoben.
* Laden von Konfigurationen bei Ausführung als Administrator behoben.
* Kommentarbehandlung in XML- und HTML-Dokumenten behoben.
* TOC-Parsing in Epub 2-Büchern behoben.
* Navigation zum nächsten Element mit dem gleichen Buchstaben im Inhaltsverzeichnis behoben.
* Such-Dialog versteckt sich nun korrekt, wenn die Tasten "Nächstes/Vorheriges" verwendet werden.
* Epub-TOC wirft dich nun nicht mehr gelegentlich zum falschen Element.
* Verschiedene Whitespace-Handhabungsprobleme in XML-, HTML- und Pre-Tags behoben.
* Off-by-One-Fehler in der Linknavigation behoben.
* Einige Bücher haben nun kein Whitespace mehr am Ende ihrer Zeilen.
* Verschiedene Parser-Probleme behoben.
* Lesezeichen-bezogene Menüelemente sowie die Elementeliste sind nun korrekt deaktiviert, wenn kein Dokument offen ist.
* Listenbehandlung in verschiedenen Dokumentformaten verbessert.
* Übersetzungs-Arbeitsablauf für Mitwirkende verbessert.
* Viele interne Umstrukturierungen, Verschiebung der Mehrheit der Geschäftslogik der Anwendung von C++ zu Rust für verbesserte Leistung und Wartbarkeit.

### Version 0.6.1
* Unterstützung für passwortgeschützte PDFs hinzugefügt!
* Eine sehr grundlegende Funktion zum Navigieren zur vorherigen/nächsten Position hinzugefügt. Wenn du `Enter` auf einen internen Link drückst und dieser deinen Cursor verschiebt, wird diese Position nun gespeichert und kann mit `alt+left`/`right arrow` navigiert werden.
* Eine Elementeliste hinzugefügt! Derzeit zeigt sie nur einen Baum aller Überschriften in deinem Dokument oder eine Liste von Links, aber es gibt Pläne, dies in Zukunft zu erweitern.
* Eine Option hinzugefügt, um Paperback standardmäßig im maximierten Modus zu starten.
* Links in einigen Epub-Dokumenten funktionieren nun korrekt.
* Parsing von Epub-TOCs mit relativen Pfaden behoben.
* Einige Epub-Dokumente zeigen nun korrekt einen Titel oder Autor an.
* Die Titel einiger Epub-Kapitel werden nun korrekt im TOC-Dialog angezeigt.
* Du kannst nun die Leertaste verwenden, um die OK/Abbrechen-Schaltflächen im TOC-Dialog zu aktivieren.
* Behandlung von Überschriften in Word-Dokumenten verbessert.
* Du erhältst nun Sprachfeedback, wenn die Liste der letzten Dokumente leer ist und du versuchst, den Dialog anzuzeigen.

### Version 0.6.0
* Eine neue Option zum Anzeigen des Go-Menüs in einer viel kompakteren Form wurde zum Optionsdialog hinzugefügt und ist standardmäßig aktiviert.
* Eine Option zum Umhüllen der Navigation nach strukturellen Elementen hinzugefügt.
* Eine Option zum Tools-Menü hinzugefügt, um den Ordner des aktuell fokussierten Dokuments zu öffnen.
* Ein ziemlich einfaches, aber sehr effektives Aktualisierungssystem hinzugefügt.
* Eine grundlegende Sleep-Timer-Funktion hinzugefügt, zugänglich mit `Ctrl+Shift+S`.
* Unterstützung für das Parsing von FB2-eBooks hinzugefügt!
* Unterstützung für das Parsing von OpenDocument-Präsentationen hinzugefügt!
* Unterstützung für das Parsing von OpenDocument-Textdateien hinzugefügt!
* Lesezeichen können nun eine ganze Zeile mit einem Lesezeichen versehen oder nur einen bestimmten Text markieren. Wenn keine Auswahl aktiv ist, wenn Sie ein Lesezeichen setzen, verhalten sich die Dinge wie vor 0.6, und die ganze Zeile wird markiert. Wenn Sie jedoch Text auswählen, wird nur dieser Text im Lesezeichen enthalten sein.
* Lesezeichen können nun optionale Textnotizen haben! Navigieren Sie zwischen Lesezeichen mit Notizen mit `N` und `Shift+N`, oder öffnen Sie den Lesezeichen-Dialog mit allen Lesezeichen, nur Notizen oder nur Nicht-Notizen mit spezifischen Hotkeys.
* Lesezeichen im Lesezeichen-Dialog haben nun kein lästiges Präfix "Lesezeichen x" mehr.
* EPUB-Bücher mit HTML-Inhalt, der so tut, als wäre er XML, werden nun ordnungsgemäß behandelt.
* Das Laden großer Markdown-Dokumente behoben.
* Das Drücken der Leertaste in der Inhaltsverzeichnisstrukturansicht, die die OK-Schaltfläche aktiviert, behoben.
* Die Leerzeichenbehandlung am Anfang von Pre-Tags in HTML- und XHTML-Dokumenten behoben.
* Das Textfeld regained Fokus nicht manchmal bei der Rückkehr zum Paperback-Fenster behoben.
* Das Textfeld im Dialog "Zu Prozentsatz gehen" aktualisiert nicht den Wert des Schiebereglers behoben.
* Das Rendering von benutzerdefinierten HTML-IDs in Markdown-Dokumenten behoben.
* HTML in Markdown-Codeblöcken wird nun ordnungsgemäß gerendert.
* Wenn Sie ein Buch mit einem Befehlszeilenparameter laden, während eine vorhandene Paperback-Instanz ausgeführt wird, erhalten Sie keinen Fehler mehr, wenn das Laden des Dokuments länger als 5 Sekunden dauert.
* Wenn Paperback als Administrator ausgeführt wird, wird die Konfiguration nun ordnungsgemäß geladen und gespeichert.
* Es ist nun möglich, ein Lesezeichen direkt aus dem Lesezeichen-Dialog zu löschen.
* Es ist nun möglich, Ihre Lesezeichen und Leseposition für ein bestimmtes Dokument zu importieren und zu exportieren. Die generierte Datei wird nach der Datei mit einer `.paperback`-Erweiterung benannt. Wenn eine solche Datei beim Laden im selben Verzeichnis wie eine Datei gefunden wird, wird sie automatisch geladen. Andernfalls können Sie sie manuell über ein Element im Tools-Menü importieren.
* Links in Dokumenten werden nun vollständig unterstützt! Verwenden Sie `k` und `shift+k`, um vorwärts und rückwärts durch sie zu navigieren, und drücken Sie die Eingabetaste, um einen zu öffnen/aktivieren.
* Viele interne Umstrukturierungen, die die App schneller und die Binärdatei kleiner machen.
* Markdown-Inhalte werden nun vorverarbeitet, um CommonMark-konform zu sein, bevor sie gerendert werden.
* Die Navigation nach Listen und deren Elementen wird nun vollständig unterstützt! Verwenden Sie `L` und `Shift+L`, um nach Listen selbst zu navigieren, und `I` und `Shift+I`, um durch Listenelemente zu navigieren.
* Numpad-Löschen funktioniert nun zum Entfernen von Dokumenten aus der Registerkarte zusätzlich zum normalen Löschen.
* Paperback kann sich nun optional in Ihr Systemfach minimieren! Diese Option ist standardmäßig ausgeschaltet, aber das Aktivieren führt dazu, dass die Minimierungsoption im Systemmenü Paperback in Ihr Fach verschiebt, wo es durch Klicken auf das erzeugte Symbol wiederhergestellt werden kann.
* Paperback ist nun vollständig übersetzbar! Die Liste der derzeit unterstützten Sprachen ist noch ziemlich klein, wächst aber ständig!
* Paperback hat nun eine offizielle Website unter [paperback.dev](https://paperback.dev)!
* PPTX-Dokumente zeigen nun ein grundlegendes Inhaltsverzeichnis mit allen Folien.
* Der vollständige Pfad zum geöffneten Dokument wird nun im Dokument-Info-Dialog angezeigt.
* Das Installationsprogramm enthält nun eine Option zum Anzeigen der Readme-Datei in Ihrem Browser nach der Installation.
* Die Liste der letzten Dokumente wurde drastisch erweitert! Anstatt einfach die letzten 10 geöffneten Dokumente anzuzeigen, zeigt es nun eine anpassbare Anzahl, wobei auf die übrigen Dokumente, die Sie je geöffnet haben, über einen kleinen Dialog zugegriffen werden kann.
* Verschiedene kleine Verbesserungen an den Parsern insgesamt, einschließlich eines Zeilenumbruchs zwischen Folien in PPTX-Präsentationen, Behebung der Zeilenumbruchbehandlung in Absätzen in Word-Dokumenten und Hinzufügen von Aufzählungszeichen zu Listenelementen.

### Version 0.5.0
* Microsoft Word-Dokumente werden jetzt unterstützt!
* PowerPoint-Präsentationen werden jetzt unterstützt!
* Bestimmte Menüelemente werden jetzt ordnungsgemäß deaktiviert, wenn keine Dokumente geöffnet sind.
* Die Ausrichtung des Prozentsatz-Schiebereglers wurde korrigiert.
* Das Inhaltsverzeichnis in EPUB-Büchern mit URL-codierten Dateipfaden und/oder Fragment-IDs wurde korrigiert.
* Whitespace in XHTML-Überschriften wird nicht mehr auf merkwürdige Weise entfernt.
* Die Whitespace-Behandlung innerhalb verschachtelter pre-Tags in HTML-Dokumenten wurde korrigiert.
* HTML- und Markdown-Dokumente unterstützen jetzt das Inhaltsverzeichnis-Feature! Wenn Sie ein HTML-/Markdown-Dokument laden, erstellt Paperback automatisch ein Inhaltsverzeichnis aus der Struktur der Überschriften in Ihrem Dokument und zeigt es Ihnen im `Ctrl+T`-Dialog an.
* HTML-Dokumente zeigen jetzt den Titel an, der im title-Tag gesetzt ist, falls dieser vorhanden ist. Andernfalls verwenden sie weiterhin den Dateinamen ohne die Erweiterung.
* Von UniversalSpeech zu einer Live-Region für die Sprachausgabe gewechselt. Das bedeutet, dass keine Screen-Reader-DLLs mehr mit dem Programm ausgeliefert werden, und mehr Screen Reader werden jetzt unterstützt, z. B. Microsoft Narrator.
* Zip-Bibliotheken gewechselt, um eine breitere Palette von EPUB-Büchern öffnen zu können.
* Der Dialog, in dem Sie gefragt werden, ob Sie Ihr Dokument als Klartext öffnen möchten, wurde vollständig überarbeitet und ermöglicht Ihnen jetzt, Ihr Dokument als Klartext, HTML oder Markdown zu öffnen.
* Der Prozentsatz-Dialog enthält jetzt ein Textfeld, mit dem Sie manuell einen Prozentsatz eingeben können, um zu diesem zu springen.
* Der HTML-Parser erkennt jetzt dd, dt und dl als Listenelemente.
* Das Inhaltsverzeichnis in EPUB-Büchern wird jetzt wieder genau beibehalten.
* Das Unicode-Leerzeichen ohne Umbruch wird jetzt beim Entfernen von Leerzeilen berücksichtigt.
* Sie werden nicht mehr gefragt, wie Sie eine unbekannte Datei öffnen möchten, jedes Mal wenn Sie diese laden, sondern nur beim ersten Mal.

### Version 0.4.1
* Ein optionales Startmenü-Symbol wurde zum Installationsprogramm hinzugefügt.
* Das Inhaltsverzeichnis sollte jetzt in einigen Fällen übersichtlicher sein. Wenn Sie beispielsweise ein untergeordnetes Element und ein übergeordnetes Element mit dem gleichen Text an der gleichen Position haben, wird Ihnen nur das übergeordnete Element angezeigt.
* Das Inhaltsverzeichnis in bestimmten CHM-Dokumenten wurde korrigiert.
* Das Inhaltsverzeichnis in EPUB-3-Büchern mit absoluten Pfaden darin wurde korrigiert.
* CHM-Dokumente zeigen jetzt ihren Titel an, wie er in der Metadatendatei gesetzt ist.

### Version 0.4.0
* CHM-Dateien werden jetzt unterstützt!
* Lesezeichen-Unterstützung wurde hinzugefügt! Sie können beliebig viele Lesezeichen in beliebig vielen Dokumenten haben. Sie können mit `B` und `Shift+B` vorwärts und rückwärts durch sie navigieren, eins mit `Ctrl+Shift+B` setzen und mit `Ctrl+B` einen Dialog öffnen, um zu einem bestimmten Lesezeichen zu springen.
* Ein Installationsprogramm wurde zusammen mit der tragbaren ZIP-Datei hinzugefügt! Das Installationsprogramm wird Paperback in Ihr Program Files-Verzeichnis installieren und automatisch Dateiverknüpfungen für Sie einrichten.
* Textdateien mit BOMs werden jetzt ordnungsgemäß dekodiert, und die BOM wird am Anfang des Textes nicht mehr angezeigt.
* Viel mehr Informationen wurden zur Statusleiste hinzugefügt. Sie zeigt jetzt Ihre aktuelle Zeile, Ihr Zeichen und Ihren Leseprozentsatz an.
* HTML-Kommentare sowie der Inhalt von script- und style-Tags werden nicht mehr in der Textausgabe angezeigt.
* Wenn Sie einen relativen Pfad an Paperback in der Befehlszeile übergeben, wird dieser jetzt ordnungsgemäß aufgelöst.
* Die prozentuale Bewegung wird jetzt durch einen eigenen Schieberegler-basierten Dialog behandelt, auf den Sie mit `Ctrl+Shift+G` zugreifen können.
* Dokumente ohne bekannte Titel oder Autoren haben jetzt immer einen Standard.
* Die Logik zum Speichern der Position ist jetzt viel intelligenter und sollte nur bei absoluter Notwendigkeit auf die Festplatte schreiben.
* Das Dokument, das Sie beim Schließen von Paperback fokussiert hatten, wird jetzt über Neustarts der Anwendung hinweg gespeichert.
* Die Eingabe in die Dialoge "Gehe zu Zeile" und "Gehe zu Seite" wird jetzt strenger bereinigt.
* Die Navigation des Inhaltsverzeichnisses in EPUB-3-Büchern mit relativen Pfaden in ihren Manifesten wurde korrigiert.

### Version 0.3.0
* Das Inhaltsverzeichnis in EPUB-Büchern mit URL-codierten Manifesten wurde korrigiert.
* Die Überschriftennavigation in HTML-Dokumenten mit mehrbytigen Unicode-Zeichen wurde korrigiert.
* Hohe CPU-Auslastung in Dokumenten mit langen Titeln aufgrund einer Regression in wxWidgets wurde korrigiert.
* Das Laden von UTF-8-Textdateien wurde korrigiert.
* Verschachtelte TOC-Elemente in EPUB-Büchern, die Ihren Cursor an die falsche Position brachten, wurden korrigiert.
* Ein Absturz beim Beenden der Anwendung in bestimmten Fällen wurde korrigiert.
* Ein Kontrollkästchen im Optionsdialog wurde hinzugefügt, um Zeilenumbruch zu aktivieren oder zu deaktivieren!
* Es ist jetzt möglich, zur Entwicklung von Paperback beizutragen, entweder über das neue Spenden-Element im Hilfemenü oder über den Link "Dieses Projekt sponsern" am unteren Rand der Hauptseite des GitHub-Repositorys.
* Markdown-Dokumente haben jetzt immer einen Titel, und Paperback sollte jetzt praktisch jede Markdown-Datei laden können.
* PDF-Dokumente haben jetzt immer einen Titel, auch wenn die Metadaten fehlen.
* PDF-Bibliotheken zu denen gewechselt, die in Chromium verwendet werden, was zu viel zuverlässigerem PDF-Parsing insgesamt führt.
* Sie können jetzt nur eine Instanz von Paperback gleichzeitig ausführen. Die Ausführung von paperback.exe mit einem Dateinamen, während bereits eine Instanz läuft, öffnet dieses Dokument in der bereits laufenden Instanz.
* Sie können jetzt `Delete` auf ein Dokument in der Registerkarten-Steuerung drücken, um es zu schließen.

### Version 0.2.1
* Die Gesamtanzahl der Seiten wurde zum Seitenlabel im Dialog "Gehe zu Seite" hinzugefügt.
* Registerkartenverweis vom Dokumentinhalt zur Liste der geöffneten Dokumente ermöglicht.
* Die Tastenkombinationen für die Überschriftennavigation öffnen manchmal die zuletzt geöffneten Dokumente, wenn Sie genug von ihnen haben, wurde korrigiert.
* Paperback entfernt jetzt unnötige Trennstriche aus der Textausgabe.
* Die Überschriftennavigation, die Sie manchmal auf das falsche Zeichen brachte, wurde korrigiert.

### Version 0.2.0
* Markdown-Dokumentunterstützung hinzugefügt!
* PDF-Dokumentunterstützung hinzugefügt, einschließlich der Möglichkeit, zwischen Seiten zu navigieren!
* Tastaturbefehle zum Navigieren nach Überschriften in HTML-Inhalten hinzugefügt, einschließlich EPUB-Büchern und Markdown-Dokumenten. Diese Tastaturbefehle wurden ähnlich wie ein Bildschirmleser konzipiert.
* Laden von EPUBs mit URL-codierten Dateinamen in ihren Manifesten behoben.
* Laden von EPUB-3-Büchern mit eingebettetem XHTML behoben.
* Eine Nachricht wird jetzt gesprochen, wenn das Dokument kein Inhaltsverzeichnis oder Abschnitte unterstützt, anstatt dass die Menüelemente deaktiviert werden.
* Menü mit kürzlich geöffneten Dokumenten hinzugefügt! Es speichert derzeit die letzten 10 geöffneten Dokumente, und durch Drücken der Eingabetaste auf einem davon wird es zum Lesen geöffnet.
* Dialogfeld „Suchen
