<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

# Paperback - Version 1.0

## Einführung

Paperback ist ein schlanker, schneller und zugänglicher Reader für eBooks, Dokumente und Hörbücher für alle – von Gelegenheitsleser bis zu intensiven Powerusern. Er ist für Barrierefreiheit mit Bildschirmlesern, hohe Geschwindigkeit und ein bloatfreies Erlebnis konzipiert.

## Systemanforderungen

Paperback läuft auf Windows 10/11, allen modernen ARM-Versionen von macOS, Linux, iOS 17 und neuer sowie Android 7 und neuer. Die iOS- und Android-Apps sind im App Store und Google Play verfügbar.

## Funktionen

* Vollständig eigenständig – du musst keine Software auf deinem Computer installieren, um mit dem Lesen zu beginnen.
* Unglaublich schnell, auch auf älterer Hardware.
* Einfache Oberfläche mit Reitern, mit der du beliebig viele Dokumente nebeneinander öffnen kannst.
* Speichert die genaue Leseposition für jedes Dokument, das du öffnest.
* Merkt sich optional, welche Dokumente du beim Schließen des Programms geöffnet hattest, und stellt sie beim nächsten Start wieder her.
* Enthält Navigationsfunktionen ähnlich denen im Web-Browsing-Modus vieler Bildschirmleser, um schnell und mühelos durch Dokumente zu navigieren.
* Verfügt über einen robusten Suchdialog mit Funktionen wie Verlauf und Unterstützung für reguläre Ausdrücke.
* Kann vollständig portabel ausgeführt oder mit automatisch eingerichteten Dateizuordnungen installiert werden.
* Unterstützt eine riesige Auswahl an gängigen Dateiformaten.
* Spielt Audiobooks mit verstellbarer Geschwindigkeit und Lesezeichen ab, die die genaue Zeit speichern.
* Liest gescannte PDF-Seiten mit der in Windows und macOS integrierten Texterkennung.
* Lesezeichen und Notizen, damit du deine Position markieren und später dorthin zurückkehren kannst.
* Jede Tastenkombination kann geändert werden.
* Kommt mit `pb`, einem Befehlszeilentool, das beliebige unterstützte Dokumente in HTML, Markdown oder einfachen Text konvertiert.

## Bildschirmleser-Kompatibilität

Paperback funktioniert gut mit allen gängigen Bildschirmlesern. Es gibt jedoch zwei bekannte Probleme für JAWS-Nutzer.

### JAWS und Braille-Displays

Bei Verwendung von JAWS mit einem Braille-Display kann es vorkommen, dass lange Absätze abgeschnitten werden, wenn du mit den Navigationstasten deines Displays vorwärts blätterst. Auch der Befehl zum Vorlesen des aktuellen Absatzes ist betroffen. Dies ist ein Fehler in der Behandlung des RICHEDIT50W-Textsteuerelements durch JAWS, nicht etwas in Paperback selbst, und ein Fehler, für den eine Lösung aufgrund von Visperos mangelndem Engagement bei der Reaktion auf Probleme mit Open-Source-Software lange dauerte.

Der Workaround, der schließlich nach monatelangem Warten durch die JAWS-Diskussionsgruppe gefunden wurde, besteht darin, `paperback.jcf` zu bearbeiten und „Braille Presentation and Panning" auf „Always use DOM if available" einzustellen. Du solltest auch „Pan Text by Paragraph" aktivieren, sonst bleibt dein Display beim aktiven Absatz stehen, anstatt weiterzugehen. Mit beiden Einstellungen sollte das Blättern ordnungsgemäß funktionieren.

### JAWS und Paperbacks Meldungen

Paperback gibt Meldungen wie „Keine Seiten." oder „Dieses Dokument hat keine Audioinhalte." als Barrierefreiheitsbenachrichtigungen aus, womit Bildschirmleser diese über ihre anderen Ausgaben hinweg sprechen können. JAWS berücksichtigt diese nur, wenn „Enable accessible notification events" für die Anwendung eingeschaltet ist, und auf manchen Computern ist das nicht der Fall.

Wenn JAWS nichts sagt, wenn du eine Taste drückst, die etwas melden sollte, öffne das Settings Center mit Paperback im Vordergrund (`Insert+6`), suche nach „notification", und aktiviere „Enable accessible notification events". Das schreibt die Einstellung in `paperback.jcf`, sodass sie nur auf Paperback zutrifft.

## Derzeit unterstützte Dateitypen

Paperback unterstützt die folgenden Formate und Dateitypen:

* Comic-Book-Archive (`.cbz`)
* CHM-Hilfedateien (`.chm`)
* DAISY-Bücher (`.opf`, `.zip`)
* EPUB-Bücher (`.epub`)
* FB2-eBooks (`.fb2`)
* HTML-Dokumente (`.htm`, `.html`, `.xhtml`)
* Manualseiten, sowohl `man` als auch BSD `mdoc` (`.1` bis `.9`, `.man`, `.roff` und die komprimierten Versionen von jedem)
* Markdown-Dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft-Word-Dokumente (`.docx`, `.docm`, `.doc`)
* M4B-Hörbücher (`.m4b`)
* MOBI-/Kindle-Bücher (`.mobi`, `.azw`, `.azw3`)
* MP3-Hörbücher (`.mp3`)
* OpenDocument-Präsentationen (`.odp`, `.fodp`)
* OpenDocument-Textdateien (`.odt`, `.fodt`)
* PDF-Dokumente (`.pdf`)
* PowerPoint-Präsentationen (`.pptx`, `.pptm`, `.ppt`)
* reStructuredText-Dokumente (`.rst`, `.rest`)
* RTF-Dokumente (`.rtf`)
* Windows-Write-Dokumente (`.wri`)
* WinHelp-Dateien (`.hlp`)
* Reine Textdateien und Logdateien (`.txt`, `.log`)

## Tastenkombinationen

Paperback ist für die Verwendung mit der Tastatur ausgerichtet. Hier sind die aktuellen Tastenkombinationen.

Die folgenden Tastenkombinationen sind für Windows. Wo sich macOS unterscheidet, ist das Äquivalent in Klammern vermerkt — hauptsächlich, weil `Ctrl+G`, `Ctrl+W` und `Alt+Left`/`Alt+Right` auf dieser Plattform bereits von anderen System- oder App-Konventionen belegt sind.

### Menü „Datei"

* `Ctrl+O`: Öffnet ein Dokument.
* `Ctrl+F4` (macOS: `Cmd+W`): Schließt das aktuelle Dokument.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Schließt alle offenen Dokumente.
* `Ctrl+Shift+T`: Öffnet das zuletzt geschlossene Dokument erneut.
* `Ctrl+R`: Zeigt den Dialog „Alle Dokumente" an (aus „Zuletzt verwendet").
* `Ctrl+Q`: Beendet Paperback (nur Windows; auf macOS befindet sich diese Option stattdessen im App-Menü).

### Menü „Gehe zu"

* `Ctrl+F`: Suchen-Dialog anzeigen.
* `F3` (macOS: `Cmd+G`): Nächstes Ergebnis suchen.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorheriges Ergebnis suchen.
* `Ctrl+G` (macOS: `Cmd+L`): Zur Zeile gehen.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Zum Prozentsatz gehen.
* `Ctrl+P`: Zur Seite gehen (wenn vom aktuellen Dokument unterstützt).
* `=`: Ankündigung des aktuellen Leseprozentsatzes und der Seitenzahl, z. B. „15 %, Seite 30". Die Seitenzahl wird bei Dokumenten ohne Seitenzahlen weggelassen.
* `Alt+Left` (macOS: `Cmd+[`): Zurück in der Navigationshistorie.
* `Alt+Right` (macOS: `Cmd+]`): Vorwärts in der Navigationshistorie.
* `[`: Vorheriger Abschnitt.
* `]`: Nächster Abschnitt.
* `Shift+H`: Vorherige Überschrift.
* `H`: Nächste Überschrift.
* `Shift+1` bis `Shift+6`: Vorherige Überschrift der Ebene 1–6.
* `1` bis `6`: Nächste Überschrift der Ebene 1–6.
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
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d. h. die physische Steuerungstaste statt Cmd): Notiztext an der aktuellen Position anzeigen.
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
* `Shift+S`: Vorheriger Trenner.
* `S`: Nächster Trenner.
* `Shift+L`: Vorherige Liste.
* `L`: Nächste Liste.
* `Shift+I`: Vorheriges Listenelement.
* `I`: Nächstes Listenelement.
* `Shift+,`: Zum Anfang des aktuellen Containers (Liste oder Tabelle) gehen.
* `,`: Über das Ende des aktuellen Containers (Liste oder Tabelle) hinausgehen.

### Menü „Extras"

* `Ctrl+W` (macOS: `RawCtrl+W`, d. h. die physische Strg-Taste statt Cmd): Wortanzahl für das aktuelle Dokument anzeigen.
* `Ctrl+I`: Dokumentinfo anzeigen.
* `Ctrl+T`: Inhaltsverzeichnis anzeigen.
* `F7`: Elementeliste anzeigen.
* `Ctrl+Shift+C`: Enthaltenden Ordner öffnen.
* `Ctrl+Shift+V`: Aktuellen Inhalt in der Webansicht öffnen.
* `Ctrl+U`: Dokumentquelle in einem neuen Tab anzeigen.
* `Ctrl+Shift+E`: Dokumentdaten exportieren (`.paperback`).
* `Ctrl+Shift+I`: Dokumentdaten importieren (`.paperback`).
* `Ctrl+E`: Aktuelles Dokument als Klartext exportieren.
* `Ctrl+Shift+B`: Lesezeichen bei der aktuellen Auswahl/dem Cursor umschalten.
* `Ctrl+Shift+N`: Lesezeichennotiz bei der aktuellen Auswahl/dem Cursor hinzufügen oder bearbeiten.
* `Ctrl+Alt+W`: Zeilenumbruch umschalten.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, d. h. die physische Strg-Taste, da Cmd+Space Spotlight öffnet): Audioerzählung abspielen/pausieren.
* `'`: Audioerzählung vorwärts spulen.
* `;`: Audioerzählung rückwärts spulen.
* `Shift+'`: Spulmenge der Audioerzählung erhöhen.
* `Shift+;`: Spulmenge der Audioerzählung verringern.
* `Ctrl+Shift+.`: Audioerzählung beschleunigen.
* `Ctrl+Shift+,`: Audioerzählung verlangsamen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d. h. Strg+Cmd+F): Vollbild umschalten.
* `Ctrl+,`: Einstellungen öffnen (macOS: im App-Menü).
* `Ctrl+Shift+S`: Schlaf-Timer umschalten.
* `Ctrl+Shift+O`: Einen Bereich gescannter PDF-Seiten mit Texterkennung erkennen.
* `Alt+F9` (macOS: `Cmd+F9`): Markiere den Anfang einer Auswahl, sodass alles von hier bis dorthin in einem Zug kopiert werden kann.
* `Alt+F10` (macOS: `Cmd+F10`): Alles vom markierten Anfang der Auswahl bis zur aktuellen Position kopieren.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Springe zurück zum markierten Anfang der Auswahl und lasse die Markierung bestehen.

### Menü „Hilfe"

* `Ctrl+F1`: Zeigt den Dialog „Über" an.
* `F1`: Öffnet die Hilfe im Standard-Browser.
* `Shift+F1`: Zeigt die Hilfe in Paperback an.
* `Ctrl+Shift+U`: Prüft auf Aktualisierungen.
* `Ctrl+D`: Öffnet die Spendenseite im Standard-Browser.

### Zusätzliche Tastenkombinationen für die Dokumentanzeige

* `Delete` / `Numpad Delete` auf dem Reiter-Steuerelement: Schließt den ausgewählten Dokumentreiter.
* `Enter` oder `Space` im Dokumenttext: Folgt einem Link oder öffnet eine Tabellen- oder Formelansicht an der Cursorposition.
* `Enter` auf einer gescannten PDF-Seite: Erkennt die Seite mit Texterkennung.
* `Shift+F10` oder die Menü-/Anwendungstaste im Dokumenttext: Öffnet das Kontextmenü.

## iOS und Android

Die Apps für iOS und Android nutzen die gleiche Lese-Engine wie die Desktop-Version, öffnen daher die gleichen Formate und speichern die Leseposition auf die gleiche Weise. Sie sind für die Verwendung mit VoiceOver auf iOS und TalkBack auf Android konzipiert.

### Dokumente öffnen

* Nutze die Schaltfläche „Buch öffnen", oder öffne ein Dokument aus der Datei-App oder einer anderen App und wähle Paperback.
* Auf Android kannst du stattdessen den In-App-Dateibrowser in den Einstellungen aktivieren. Er benötigt die Berechtigung für den Zugriff auf alle Dateien und öffnet große Dateien direkt, anstatt sie zuerst zu kopieren.
* Halte die Schaltfläche „Buch öffnen" gedrückt, um die Daten eines Dokuments (`.paperback`) zu importieren oder zu exportieren – dieselben Dateien, die die Desktop-App verwendet.

### Lesen und Anhören

Jede App hat zwei Möglichkeiten, ein Dokument zu lesen. Im Textmodus liest der Bildschirmleser den Text vor. Im Modus „Vorlesen" liest Paperback den Text mit der Stimme vor, die du in den Einstellungen wählst, und spielt im Hintergrund sowie vom Sperrbildschirm weiter. Wechsle zwischen ihnen im Menü „Weitere Optionen".

Audiobücher wie DAISY, M4B und MP3-Bücher spielen stattdessen ihre eigene Aufnahme ab.

### Die Lesezeile

Die Leiste am unteren Rand des Bildschirms enthält von links nach rechts:

* Die Navigationseinheit, beispielsweise Absatz, Überschrift, Seite oder Link. Wische nach oben oder unten darauf, um sie zu ändern.
* Schaltflächen für Zurück, Abspielen und Weiter. Zurück und Weiter bewegen sich um die Navigationseinheit.
* Die Sprechgeschwindigkeit. Wische nach oben oder unten darauf, um zu ändern, wie schnell Paperback vorliest.

Du kannst auch nach oben oder unten auf die Abspielen-Schaltfläche wischen, um dich um die Navigationseinheit zu bewegen, ohne die Schaltflächen für Zurück und Weiter erreichen zu müssen. Falls du nur das nutzt, entfernt die Einstellung Schaltflächen für Zurück und Weiter ausblenden diese aus dem Weg deines Bildschirmlesers. Die Einstellung Wischen nach oben bewegt vorwärts bestimmt, in welche Richtung ein Wisch führt.

### Weitere Optionen

Das Menü „Weitere Optionen" enthält alles Übrige. Einige Einträge verhalten sich auf den einzelnen Apps etwas anders.

* **Zu Vorlesemodus oder zu Textmodus wechseln:** wechselt zwischen dem Modus „Vorlesen" und dem Textmodus (siehe oben). Im Textmodus startet und pausiert ein Eintrag „Vorlesen" die Wiedergabe, ohne den Textmodus zu verlassen.
* **Inhaltsverzeichnis:** zeigt die Kapitel des Buchs und öffnet das Kapitel, das du gerade liest. Wähle ein Kapitel, um direkt dorthin zu springen. Einträge mit Unterkapiteln können mit den Aktionen des Bildschirmlesers aus- und eingeklappt werden.
* **Elemente:** eine Liste der Überschriften oder Links des Dokuments. Wechsle zwischen beiden mit der Typauswahl unter iOS oder den Registerkarten unter Android, und wähle dann einen Eintrag, um dorthin zu springen.
* **Suchen:** gib ein, was zu suchen ist, oder wähle eine frühere Suche aus dem Suchverlauf, und entscheide, ob Groß- und Kleinschreibung beachtet werden soll, nur ganze Wörter gesucht werden sollen, oder ein regulärer Ausdruck verwendet werden soll. „Vorherige suchen" und „Nächste suchen" springen zu einer Übereinstimmung und geben an, wo sie gefunden wurde. Die Suche bleibt offen, sodass du weitermachen kannst. Im Vorlesemodus wird die Suche auch als Navigationselement in der Leiste angezeigt, sodass du die Übereinstimmungen auch von dort aus durchgehen kannst.
* **Gehe zu:** springe zu einer Zeile, einer Seite oder einer Prozentzahl im Dokument. Wähle mit der Modusauswahl aus.
* **Zuletzt geöffnete Dokumente:** alle Dokumente, die du geöffnet hast, jeweils gekennzeichnet als zurzeit offen, geschlossen oder Datei fehlt. Jedes Dokument hat zwei Aktionen des Bildschirmlesers: „Entfernen" nimmt es aus der Liste, und „Suchen" ermöglicht es dir, ein Dokument zu finden, dessen Datei verschoben wurde. „Zuletzt geöffnete Dokumente löschen" leert die Liste, ohne Dokumente zu löschen.
* **Wortanzahl:** die Anzahl der Wörter im Dokument.
* **Dokument-Info:** Titel, Autor, Dateiname und unter iOS auch die Zeilen- und Zeichenanzahl.
* **Exportieren:** speichert das Dokument als reinen Text, HTML oder Markdown.
* **Schlaf-Timer:** beendet die Wiedergabe nach 5, 10, 15, 30, 45 oder 60 Minuten oder zu einer selbst festgelegten Zeit. Öffne es erneut während der Ausführung, um zu sehen, wie lange es noch dauert, oder um es abzubrechen.
* **Hilfe:** öffnet diese Anleitung.
* **Einstellungen:**
    * **Vorlesen:** die Stimme, die Sprechgeschwindigkeit und die Tonhöhe, eine Schaltfläche „Beispiel abspielen", um sie zu hören, und die Pause zwischen Absätzen. Android ermöglicht dir auch die Auswahl der Sprach-Engine. Unter iOS ist dies auch der Ort, an dem sich das Sprachwörterbuch befindet: Regeln, die ändern, wie Wörter gesprochen werden, für jede Stimme oder nur für einige.
    * **Lesbarkeit:** Textgröße, Zeilenabstand, Absatzabstand, Ausrichtung und hochkontrastiger Text. iOS hat auch helles und dunkles Erscheinungsbild.
    * **Verhalten:** ob deine Dokumente erneut geöffnet werden, wenn die App startet, in welche Richtung ein Wischen auf die Wiedergabeschaltfläche verläuft, und ob die Schaltflächen „Zurück" und „Vorwärts" ausgeblendet werden sollen. Android hat auch den integrierten Dateibrowser hier.

### Tastaturen und Headsets

Mit einer Tastatur funktionieren die Desktop-Tastenkombinationen zum Öffnen von Büchern, zuletzt geöffneten Dokumenten, Suchen, Gehe zu, dem Inhaltsverzeichnis, Wortanzahl, Dokumentinformationen, Exportieren und dem Schlaf-Timer, wobei `Cmd` auf iOS anstelle von `Ctrl` verwendet wird. Dasselbe gilt für die Einzelbuchstaben-Tasten zum Navigieren nach Überschrift, Seite, Link und mehr, und `Space` startet und pausiert die Wiedergabe. Auf iOS erreichen die Einzelbuchstaben-Tasten Paperback nur, wenn VoiceOvers Single-Letter Quick Nav ausgeschaltet ist.

Auf Android startet und pausiert eine Kopfhörer-Taste mit einem Druck, springt vorwärts mit zwei und geht mit drei zurück.

## Unterstützte Sprachen

Paperback ist in viele verschiedene Sprachen übersetzt, und ständig kommen neue hinzu. Eine vollständige Liste folgt unten.

Um zu erfahren, wie du beitragen kannst, lies bitte unseren [Übersetzungsleitfaden](translating.md).

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
* Ukrainisch
* Vietnamesisch

## Mitwirkende

### Entwicklung
* Quin Gillespie: Hauptentwickler und Projektgründer.
* Aryan Choudhary: Hauptbeitragender.

### Spenden
Die folgenden Personen haben Spenden in unterschiedlicher Höhe zur Entwicklung von Paperback beigetragen. Wenn du eine Spende machst, wird dein Name nicht automatisch hier hinzugefügt – ich füge nur Personen hinzu, die ihre Spende veröffentlicht sehen möchten.

Hinweis: Ich betrachte ein öffentliches GitHub-Sponsoring als Grund für die automatische Aufnahme in diese Liste.

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

## Änderungsprotokoll

### Version 1.0

1.0 ist die erste Veröffentlichung auf allen fünf Plattformen: Windows, macOS, Linux, iOS und Android, mit den iOS- und Android-Apps im App Store und Google Play.

#### Hinzugefügt

##### Allgemein
* Linux-Unterstützung als AppImage oder tar.gz mit Desktopintegration, sodass sich Dokumente aus dem Dateimanager öffnen lassen.
* Markiere den Anfang einer Auswahl mit `Alt+F9`, kopiere alles von dort bis zu deiner aktuellen Position mit `Alt+F10`, und kehre zur Markierung mit `Alt+Shift+F9` zurück – zum Kopieren langer Textabschnitte ohne Shift-Pfeiltasten. Alle drei Funktionen findest du unter Menü „Extras" > Menü „Auswahl und Kopieren".
* Die Tastenkombination `=` gibt nun die Seite sowie den Prozentsatz an, z. B. „15%, Seite 30", und verhält sich bei Dokumenten ohne Seitenzahlen wie zuvor.
* Das Fenster „Über" zeigt nun die Lizenz von Paperback und alle Übersetzer.
* Eine ukrainische Übersetzung.

##### Neue Formate
* Comic-Buch-Archive (`.cbz`).
* M4B-Hörbücher, aufgeteilt in ihre Kapitel.
* Handbuchseiten, sowohl `man` als auch BSD `mdoc`, komprimiert oder nicht.
* MP3-Hörbücher, aufgeteilt in Kapitel, wenn die Datei sie enthält.
* reStructuredText-Dokumente.
* Windows-Write-Dateien (`.wri`).
* WinHelp-Dateien (`.hlp`).
* Word 6 und Word 95-Dokumente.

##### Texterkennung
* Gescannte PDF-Seiten können nun mit der in Windows und macOS integrierten Texterkennung erkannt werden. Drücke `Enter` auf einer gescannten Seite, um sie zu erkennen, oder verwende Texterkennung (Stapelverarbeitung) (`Ctrl+Shift+O`) für mehrere Seiten.

##### Navigation
* MathML-Formeln in EPUB und HTML werden mit MathCAT als AsciiMath dargestellt. Nutze `M` oder `Shift+M` zum Navigieren in Formeln, dann `Enter` oder `Space` um die ursprüngliche MathML in der Formelansicht zu öffnen.
* Ein Schaltfläche „Alle suchen" im Suchdialog, die jede Zeile mit einem Treffer auflistet, so dass du direkt zur gewünschten springen kannst.
* Tabellenansicht, Listenansicht und Seitenansicht in der Elementenliste (`F7`).
* „Gehe zu Zeile", „Gehe zu Seite" und „Gehe zu Prozent" nehmen jetzt `+n` und `-n` an, um relativ zur aktuellen Position zu navigieren.
* EPUB-, MOBI- und CHM-Bücher ohne eigene Überschriften erhalten jetzt Überschrift-Navigation aus ihrem Inhaltsverzeichnis.
* KF8-(AZW3-)Bücher unterstützen jetzt Abschnittsnavigation.
* EPUB-Seiten, die nur aus einem Bild bestehen, zeigen jetzt eine Zeile dafür an, so dass du dort landen kannst, statt direkt vorbeizuspringen.

##### Hörbücher
* Wiedergabegeschwindigkeit lässt sich von halber Geschwindigkeit bis zur dreifachen Geschwindigkeit steuern. Verwende `Ctrl+Shift+.` und `Ctrl+Shift+,`, oder das Menü „Werkzeuge".
* Lesezeichen und Notizen in reinen Hörbüchern merken sich jetzt die genaue Zeit, zu der sie gesetzt wurden.
* Nächste und vorherige Position (`Alt+Left` und `Alt+Right`) funktionieren jetzt in Hörbüchern.
* Der Fortschritt durch ein Hörbuch wird jetzt an seiner Aufnahme gemessen, sodass „Gehe zu Prozent" und die Statusleiste zeigen, wie weit du wirklich bist.

##### Zuletzt geöffnete Dokumente
* Ein eindeutiger Eintrag für zuletzt geöffnete Dokumente im Untermenü „Zuletzt geöffnete Dokumente".

##### PDF-Dokumente
* Eine Einstellung, um jede Zeile einer PDF getrennt zu halten, statt sie zu Absätzen zusammenzufassen.
* Bilder und Abbildungen in PDFs werden jetzt angekündigt.
* PDFs, die eine Lesestruktur enthalten, aber ihre Bilder nicht taggen, kündigen diese Bilder jetzt an, statt sie ganz aus dem Buch auszulassen.

##### Web View
* Jedes Dokument kann nun in der Webansicht geöffnet werden, nicht nur EPUB, HTML und Markdown.

##### Lesbarkeit
* Überschriften werden jetzt in einer Größe dargestellt, die ihrer Ebene entspricht, und Bilder sowie Tabellen sind vom umgebenden Text abgesetzt.

##### pb
* `pb --list-formats` zeigt alle Formate auf, die pb lesen kann.
* pb gibt nun an, welche Datei nicht gelesen werden konnte und warum.

#### Behoben

##### Allgemein
* Ein Buch, das beim Start erneut geöffnet wird, liest sich jetzt sofort vor, statt bis zur Schließung und erneuten Öffnung stumm zu bleiben.
* Ein Dokument, dessen Datei verloren gegangen ist, kann jetzt aus „Alle Dokumente" entfernt werden, statt in der Liste zu bleiben, obwohl du dies bestätigst.
* Absturz beim Schließen von Paperback behoben.
* Das Schließen von Paperback blendet das Fenster jetzt sofort aus, statt es auf dem Bildschirm zu lassen, während es speichert.
* Große Bücher mit wenig Formatierung öffnen sich jetzt etwa halb so schnell.
* Meldungen, die aus einem Menü gewählt werden, wie „Dieses Dokument hat keinen Audio", werden vom Bildschirmleser nicht mehr abgeschnitten, bevor du sie hörst.
* Das Öffnen eines Dokuments lässt „Zuletzt geschlossenes erneut öffnen" nicht mehr aktiviert, wenn es nichts zu öffnen gibt.
* Paperback versucht nicht mehr, Dokumente aus der Liste der zuletzt geöffneten Dateien erneut aufzurufen, die verloren gegangen sind, und begrenzt die Anzahl der gespeicherten zuletzt geöffneten Dokumente.
* Die alte INI-Einstellungsdatei wird jetzt gelöscht, nachdem sie in das neue Format übertragen wurde.
* Die Titel der Schrift- und Farbdialoge sowie das Menü „Exportieren unter" im Vietnamesischen sind jetzt übersetzt.
* Nach einer Aktualisierung wird das neu gestartete Fenster jetzt nach vorne gebracht, statt es hinter jedem anderen Fenster in `Alt+Tab` zu lassen.
* Zeilenumbruch wird jetzt sofort bei großen Dokumenten angewendet, statt die ganze Sache erneut zu laden.

##### Navigation
* `Alt+Left` geht jetzt zur Stelle zurück, von der du gesprungen bist, statt zu einer älteren Position.
* Lesezeichen-Sounds spielen jetzt nur ab, wenn du über ein Lesezeichen fährst, nicht wenn du auf der Zeile landest, auf der es sich befindet.
* Beim Schließen des Inhaltsverzeichnisses, der Elementeliste und der Gehe-zu-Dialoge wirst du direkt zur Zeile gebracht, auf der du landest, statt dass der Bildschirmleser das Fenster nochmal vorliest.
* Gehe zu Zeile, Gehe zu Seite und Gehe zu Prozent lehnen jetzt Nummern außerhalb des Dokuments ab, statt stillschweigend woanders hinzugehen.
* NVDA schneidet die Ankündigung nicht mehr ab, wenn ein Dokument keine Seiten hat.
* OK im Inhaltsverzeichnis ohne Bewegung zu drücken geht jetzt zum bereits ausgewählten Eintrag.
* Das Inhaltsverzeichnis, die Elementeliste und die Lesezeichenliste laggen oder frieren nicht mehr bei Büchern mit Tausenden von Einträgen.
* Auf und Ab Pfeiltasten merken sich jetzt ihre Spalte pro Dokument, statt sie beim Wechsel von Registerkarten mitzunehmen.

##### Hörbücher
* Die Audiowiedergabe verwendet jetzt `Control+Space` auf macOS, da `Command+Space` zu Spotlight gehört.

##### PDF-Dokumente
* PDF-Dateien, die aus Apple Pages exportiert wurden und als einfacher Text gelesen wurden, zeigen nun die Überschriften und Listen an, mit denen sie verfasst wurden.
* PDF-Absätze und Überschriften, die bei jeder Zeile aufgespalten wurden, und Wörter, die an Leerzeichen aufgespalten wurden, werden nun korrekt dargestellt.
* Nummerierte PDF-Überschriften, die zusammenliefen, werden nun als separate Überschriften behandelt.
* PDFs, deren Strukturbaum zu keinem Text führte und leer angezeigt wurden, werden nun korrekt geöffnet.
* Zeilen in einer nichtproportionalen Schriftart, etwa Code, werden nicht mehr zu Absätzen zusammengefasst.
* Seitenkopfzeilen und Fußzeilen werden in unmarkierten PDFs nicht mehr auf jeder Seite vorgelesen.
* PDFs, die ihre Seitenkopfzeilen und Fußzeilen als normalen Text markieren, wiederholen nun nicht mehr den Titel und die Seitennummer zwischen zwei Absätzen auf jeder Seite.
* PDFs zeigen nun ihren eigentlichen Titel anstelle des Dateinamens an.

##### MOBI/AZW3-Bücher
* Große MOBI-Bücher erschöpfen den Speicher nicht mehr und werden nicht mehr nach 20 MB abgeschnitten.
* MOBI- und AZW3-Bücher öffnen sich nun viel schneller.
* Ein Problem behoben, bei dem MOBI-Bücher ihre Kapitelliste verloren.
* Ein Problem behoben, bei dem Text in MOBI-Büchern beim Übergang von einem Datensatz zum nächsten unleserlich wurde.

##### Web View
* Die Web-Ansicht lädt große Bücher nicht mehr komplett auf einmal.
* Die Web-Ansicht zeigt Dokumente vollständig an, wenn der Reader sie vollständig anzeigt, statt nur einen Ausschnitt davon.

##### Andere Formate
* FictionBook-Bücher (.fb2) mit windows-1251-Codierung, bei denen es sich um die meisten handelt, werden jetzt geöffnet statt nicht gelesen zu werden.
* FictionBook-Bücher, die einen Namespace oder eine HTML-Entity verwenden, die sie nie deklariert haben, werden jetzt geöffnet statt als beschädigt abgelehnt zu werden.
* Bücher in veralteten Codierungen werden jetzt viel schneller geöffnet.
* Einige chinesische Textdateien werden nicht mehr als unleserlicher Text geöffnet.
* Passwortgeschützte OpenDocument-Dateien fragen jetzt nach ihrem Passwort statt als beschädigt gemeldet zu werden.
* Passwortgeschützte PowerPoint-Dateien im alten Format werden jetzt geöffnet, und Folien im alten PowerPoint-Format verlieren ihren Text nicht mehr.
* Nur-Text-Dateien, die mit einer `.rtf`-Erweiterung gespeichert wurden, werden jetzt als Text geöffnet statt mit einem Fehler abzubrechen.
* RTF-Steuerwörter werden nicht mehr als Text angezeigt.

#### iOS und Android

Die iOS- und Android-Apps öffnen alle Formate, die die Desktop-Version öffnet, und bieten:

* Vorlesen mit wählbarer Stimme, Sprechgeschwindigkeit und Tonhöhe, einer Sprachgeschwindigkeitsregelung direkt in der Leiste und optionalen Pausen zwischen Absätzen.
* Wiedergabe von DAISY-, M4B- und MP3-Hörbüchern, die im Hintergrund und vom Sperrbildschirm aus weiterlaufen.
* Navigation nach Überschriften, Seiten, Links, Tabellen, Listen und mehr über die Leiste, sowie Inhaltsverzeichnis und Suchen.
* Ein Schlaf-Timer, Wortanzahl und Dokumentexport, plus ein Sprachverzeichnis unter iOS. Unter iOS erfolgt der Export über das Freigabemenü, sodass ein Buch in einer anderen App oder unter „Dateien" in einem anderen Format oder genau wie vorhanden weitergegeben werden kann.
* Optionen für Textgröße, Zeilenabstand und hochkontrastreiche Texte.
* Tastenkombinationen, die denen der Desktop-Version entsprechen.

### Version 0.9.2
* Audiobooks lassen deinen Bildschirmleser nicht mehr eine Folge von Leerzeichen vorlesen, wenn du das Textfeld fokussierst.
* Audiobooks benennen die Datei nun, während du sie nach Abschnitten durchschreitest.
* Audiobooks geben die echte Länge an, statt zu behaupten, dass jede Datei darin 24 Stunden läuft.
* Wenn du die Web View mit Escape schließt, zeigt sich keine Debug-Meldung mehr, nachdem du einem Link darin gefolgt bist.
* Nach „Alles auswählen" und Kopieren erhältst du das ganze Dokument, statt nur des gerade geladenen Teils.
* „Suchen" springt direkt zur gefundenen Zeile, statt dass der Bildschirmleser das Fenster erneut vorliest, während der Fokus zum Buch zurückkehrt.
* EPUBs mit einem verirrten ZIP64-Block, die mit „Invalid local file header" nicht geöffnet werden konnten, funktionieren nun.
* Lange Dokumente springen nicht mehr an ihren Anfang zurück, während ein Bildschirmleser durchgehend darin liest.
* Links in der WebView bringen dich zum Abschnitt, auf den sie zeigen, statt mit „File not found" zu fehlschlagen.
* Die automatische Ankündigung „Dokument neu geladen" unterbricht deinen Bildschirmleser nicht mehr mitten im Satz, sondern wartet, bis er fertig ist.
* Der Reiter „Allgemein" im Einstellungsdialog wechselt nun in der Reihenfolge durch seine Optionen, wie sie auf dem Bildschirm erscheinen, mit dem Update-Kanal direkt nach der Option „Auf Updates prüfen".
* Windows zeigt nun immer „Paperback" im Menü „Öffnen mit" an, statt des vollständigen Tagline des Programms.
* „Wortanzahl" und „Dokumentinfo" zeigen nun, wie viele Dateien ein Audiobook enthält, und wie lange es insgesamt läuft.

### Version 0.9.1
* Lesezeichen- und Notiz-Töne werden jetzt unter macOS wiedergegeben.
* DAISY-Bücher geben ihre Audio unter macOS wieder, statt sie zu öffnen und ihre Zeitleiste stumm abzuspielen.
* Fehler behoben: Lockige Anführungszeichen, Gedankenstriche und ähnliche Zeichen verschwinden nicht mehr aus RTF-Dokumenten und führen nicht mehr dazu, dass benachbarte Wörter zusammenlaufen.
* Fehler behoben: RTF-Bilder geben ihre Rohdaten nicht mehr als Zeichensalat in das Dokument ab.
* Fehler behoben: Das Untermenü „Zuletzt verwendet" behielt veraltete Einträge bei, bis ein anderes Ereignis zu ihrem Neuaufbau führte.
* Tastenkombinationen sind wieder in jeder Übersetzung vorhanden, sodass die Menüs Russlands wieder Tastaturzugriff haben.
* Große CHM-Dokumente öffnen sich jetzt bis zu siebenmal schneller.
* Geöffnete Dokumente werden nun bei Windows registriert, sodass sie in der Sprungliste der Taskleiste und in der Liste „Zuletzt verwendet" des Startmenüs angezeigt werden.
* „Einstellungen" hat den Namen „Optionen" ersetzt und entspricht damit den mobilen Apps und auf macOS der Plattformkonvention.
* Paperback speichert seine Fensterposition, Größe und den maximierten Zustand jetzt zwischen den Ausführungen.
* Pluralformen werden jetzt übersetzt, sodass Meldungen, die Dinge zählen, in Sprachen, die mehr als eine Form benötigen, ordnungsgemäß gelesen werden.
* Die Auswahl der Datei ncc.html eines DAISY-Buches öffnet jetzt das komplette Hörbuch statt nur seinen Text.
* Die Aktionsnamen im Dialog „Tastenkombinationen anpassen" können jetzt übersetzt werden.
* Der Dokumenttitel steht jetzt an erster Stelle in der Titelleiste, sodass offene Bücher in der Taskleiste und bei Alt+Tab unterschieden werden können.
* Der Aktualisierungsdialog wird jetzt übersetzt.

### Version 0.9.0

#### Hinzugefügt

##### Allgemein
* Ein CLI-Tool namens pb, um schnell alle von Paperback unterstützten Formate in HTML, Markdown oder reinen Text umzuwandeln.
* Eine Option zum Neuladen von Dokumenten, die von anderen Programmen auf der Festplatte geändert wurden.
* Eine Option „Quelle anzeigen", um die Quelle eines Dokuments in einem neuen Tab zu öffnen, nützlich beispielsweise zum Bearbeiten von Markdown.
* Dokumenttext wird jetzt paginiert, sodass du Bücher mit Millionen von Wörtern in nur wenigen Sekunden laden kannst. Bitte berichte über jede Besonderheit, die du damit findest.

##### Plattformunterstützung
* ARM64-Windows-Unterstützung!
* Native macOS-Unterstützung!
* Ein Vollbildmodus-Schalter.

##### Dialogfeld „Alle Dokumente"
* Eine Suchschaltfläche, um vermisste Bücher zu finden, deren Pfad sich gerade geändert hat.
* Ein Statusfilter und eine Statusleiste, um nach Dokumentstatus zu filtern und zu sehen, wie viele Dokumente angezeigt und ausgewählt sind.
* Die Tastenkombination `Ctrl+Shift+A` zum Abwählen aller Dokumente.

##### Optionen und Lesbarkeit
* Eine Registerkarte Lesbarkeit mit den folgenden Optionen:
    * Zeilenumbruch (aus Allgemein verschoben);
    * Tabellen inline darstellen (neu in dieser Version, siehe unten);
    * Schrift;
    * Hintergrundfarbe;
    * Zeilenabstand;
    * Absatzabstand;
    * Zeichenabstand;
    * Textausrichtung.
* Ein Menüelement für Zeilenumbruch und die entsprechende Tastenkombination.
* Eine Umschaltfläche zur Bestimmung der Tabellenanzeige und einheitliche Tabellenformatierung über Dokumente hinweg.

##### Navigation
* Unterstützung für die Navigation nach Behältern.
* Eine Option zum automatischen Bewegen des Cursors an den Anfang der Zeile bei der Navigation zwischen Zeilen, ähnlich wie der Lesemodus in Bildschirmlesern.
* Die Tastenkombination `=` zum Ankündigen des aktuellen Prozentsatzes im Dokument.

##### Lesezeichen
* Temporäre Lesezeichen: Du kannst eines pro Dokument haben, und es bleibt erhalten. Nutze Schrägstrich zum Setzen und Backslash zum Springen.

##### Wortanzahl
* Geschätzte Lesezeit im Wortanzahl-Dialog sowie die Möglichkeit, deine Lesegeschwindigkeit einzustellen, um diese Metrik tatsächlich nutzen zu können.
* Wenn eine Auswahl aktiv ist, wenn du den Wortanzahl-Dialog öffnest, wird nun angezeigt, wie viele Wörter du ausgewählt hast.

##### Tastenkombinationen
* Die Möglichkeit, jede Tastenkombination in der App über einen einfachen Dialog anzupassen.
* Eine konfigurierbare Tastenkombination, um Paperback aus der Taskleiste wiederherzustellen.

##### Sprachen
* Niederländisch, Finnisch und Polnisch.

##### Export
* Das Menüelement „Exportieren" wurde erweitert, um neben Klartext auch den Export nach HTML und Markdown zu ermöglichen.

##### Aktualisierung
* Eine Schaltfläche zum Abbrechen im Dialog für laufende Aktualisierungen.
* Die Aktualisierung validiert nun, dass die heruntergeladene Datei nicht manipuliert wurde.

##### Webansicht
* Die Webansicht wird nun an deiner aktuellen Leseposition geöffnet.

##### DAISY-Bücher
* Unterstützung für DAISY 2.0-Bücher.
* Unterstützung für DAISY 2.02-Audiowiedergabe.

##### Hörbücher
* Die Möglichkeit, Hörbücher abzuspielen, unterstützt derzeit sowohl DAISY-Audio (einschließlich DAISY-Audio + Text) als auch ZIP-Archive mit Audiodateien.
* Tastenkombinationen und Menüelemente zum Abspielen/Pausieren der Narration, zum Spulen vorwärts und rückwärts sowie zum Anpassen der Spulmenge.
* Optionen zum Synchronisieren der Leseposition mit der Audiowiedergabe, zum Festlegen der Audio-Spulmenge und zum Auswählen, ob das Spulen über das Ende eines Kapitels hinaus in das nächste Kapitel fortgesetzt wird.

##### CHM-Dokumente
* Unterstützung für Listen, Listenelemente, Abbildungen und Bilder.

##### PowerPoint
* PowerPoint-Dokumente unterstützen nun Tabellen.

#### Behoben

##### Allgemein
* Dokumente in veralteten CJK-Kodierungen wie GBK, Big5 und Shift_JIS werden jetzt korrekt angezeigt statt als Zeichensalat.
* „Letztes geschlossenes Dokument erneut öffnen" versucht nicht mehr, die gebündelte Readme zu öffnen.
* Der ausgewählte Tab wird nach dem Neustart von Paperback jetzt richtig fokussiert.
* Paperbacks Umgang mit Dateien auf Windows-Netzlaufwerken: „Im Ordner anzeigen" fokussiert die Datei jetzt richtig auf dem Netzwerkspeicher, und die Pfade enthalten keine merkwürdigen Zeichen mehr.
* .paperback-Dateien werden bei der Dokumentwiederherstellung nicht mehr zwangsweise geladen, stattdessen wirst du um Bestätigung gefragt, wenn eine gefunden wird.
* „Enthaltenden Ordner öffnen" fokussiert die Datei jetzt im Explorer.
* Das Öffnen der Readme respektiert jetzt deine ausgewählte Sprache.
* Die Benutzeroberfläche von Paperback wird jetzt auf hochauflösenden Displays richtig skaliert.
* Das Menü wird jetzt richtig aktualisiert und der Fokus wechselt zum Textsteuerelement, wenn die Hilfe in Paperback geöffnet wird.
* Wechsel zu einer viel sichereren Methode für IPC unter Windows.
* Der Titel des aktiven Dokuments wird jetzt verlesen, wenn zwischen Tabs gewechselt wird.
* Reduzierte Speichernutzung bei großen Dokumenten durch Halbierung der Größe der internen Zeichenindextabellen.

##### Dialog „Alle Dokumente"
* Escape schließt die Dialoge „Dokumentinformationen" und „Alle Dokumente" nicht.
* Die Titelleiste wird nicht aktualisiert, nachdem ein Dokument aus dem Dialog „Alle Dokumente" geschlossen wurde.
* Readme.html wird nicht mehr zur Liste „Alle Dokumente" hinzugefügt, wenn es über `Shift+F1` geöffnet wird.
* Das Entfernen von Dokumenten aus dem Dialog „Zuletzt verwendet" schließt nun auch deren aktive Registerkarte.
* Der Suchfilter bleibt nach dem Entfernen eines Dokuments erhalten.

##### Navigation
* Seitennavigation gibt in manchen Fällen falschen Zeilentext an.
* „Gehe zu Zeile", „Gehe zu Seite" und „Gehe zu Prozent" platzieren den Cursor in großen Dokumenten an der falschen Position.
* „Suchen" und „Suchen (nächstes)" beachten in großen Dokumenten das geladene Dokumentfenster nicht.

##### Lesezeichen
* Die Sounds für Lesezeichen und Notizen sollten jetzt korrekt nur abspielen, wenn du über ein Wort mit einem solchen navigierst.

##### Lesbarkeit
* Das Anwenden des Zeilenumbruchs führt dich zum Anfang des Dokuments.

##### Web View
* Der Webview-Dialog konnte nicht in der Größe verändert werden und öffnete sich in einer sehr kleinen Anfangsgröße.
* Bilder sollten sich nun ordnungsgemäß in der eingebetteten Webview anzeigen.

##### Updater
* Das Updater-Programm zeigt den Inhalt von Markdown-Code-Tags in Versionshinweisen nun korrekt an.

##### DAISY-Bücher
* DAISY-Bücher zeigen falsche Informationen in der Statusleiste.
* Laden von DAISY-Büchern mit ungültigen Kodierungsdeklarationen.

##### RTF-Dokumente
* Analyse von RTF-Dokumenten mit Zeichen außerhalb des lateinischen Alphabets.
* RTF-`\pict`-Gruppen, sodass eingebettete Bilddaten nicht länger in den Dokumenttext gelangen.

##### Mobi/AZW3-Bücher
* Filepos-Anker in Mobi-Büchern teilen HTML-Tags auf und fügen Müll in den Buchtext ein.
* Links in älteren Mobi-Büchern.
* Wesentlich verbessertes Parsing von AZW3-Dateien.

##### Word-Dokumente
* Word-Dokumente mit gebietsschemaspezifischen Stilnamen, deren Überschriften nicht korrekt dargestellt werden.

##### HTML/XHTML-Dokumente
* `dl`-, `dt`- und `dd`-Elemente erzeugen keine Zeilenumbrüche in XHTML-Dokumenten.

##### PDF-Dokumente
* Paperback greift nun auf reine Textextraktion für falsch gekennzeichnete PDFs zurück.
* PDF-Dokumente, die Steuerzeichen in ihren Titeln und/oder Lesezeichen enthalten, führen beim Öffnen nicht mehr zu einem Absturz von Paperback.

### Version 0.8.5
* Seitenunterstützung für EPUB-Bücher hinzugefügt.
* Unterstützung für verschlüsselte Microsoft Office-Dokumente hinzugefügt. Derzeit werden Legacy Word, modernes Word und modernes Powerpoint unterstützt, Legacy Powerpoint ist für die Zukunft geplant.
* Unterstützung für Legacy Microsoft Word-Dokumente hinzugefügt!
* Unterstützung für Legacy Powerpoint-Präsentationen hinzugefügt!
* Unterstützung für MOBI- und AZW3-Bücher hinzugefügt!
* Unterstützung für gekennzeichnete PDF-Dateien hinzugefügt!
* Die `Ctrl+Q`-Tastenkombination zum Beenden der App hinzugefügt.
* Unterstützung für gezippte Bücher von Bookshare hinzugefügt (sowohl DAISY als auch Word)!
* Alternativtext für eingebettete Bilder wird nun korrekt angezeigt.
* CHM-Dokumente unterstützen nun ordnungsgemäß die Navigation über interne Links.
* Problem behoben: „Gehe zu Seite" war um 1 verschoben.
* Problem behoben: Die Escape-Taste funktioniert nicht zum Schließen des Dialogs „Öffnen unter".
* Problem behoben: Das Kontextmenü des Readers wird bei Rechtsklick oder der Anwendungstaste nicht angezeigt.
* Problem behoben: Beim Öffnen von Dokumenten aus der Befehlszeile wird manchmal das falsche Dokument fokussiert.
* Nur-Bild-PDFs werden wieder erkannt und du wirst auf ihr Vorhandensein hingewiesen.
* Es ist nun möglich, durch Bilder und Abbildungen mit `G`/`Shift+G` bzw. `F`/`Shift+F` zu navigieren.
* Paperback respektiert nun deine Einstellung für den Dunkelmodus der Anwendung.
* DAISY XML-Unterstützung entfernt, da sie nicht mehr erforderlich ist.
* Zurückgewechselt zur nativen Win32-Navigation nach erstem Buchstaben im Inhaltsverzeichnis-Baum.
* Der Fehlerlade-Dialog zeigt nun ausführlichere Fehlermeldungen.
* Die Webansicht wird nun viel schneller und flüssiger geöffnet.

### Version 0.8.2
* Seitenunterstützung für RTF-Dokumente hinzugefügt!
* Ein Fehler behoben, bei dem das Öffnen der Webansicht in EPUBs mit externen Links diese automatisch aktivieren würde.
* Ein Fehler behoben, bei dem der RTF-Parser in seltenen Fällen keinen Platz zwischen Wörtern einfügen würde.
* Absätze werden in einigen PDF-Dokumenten nicht mehr in mehrere kurze Zeilen aufgeteilt.
* PDF-Dokumente haben jetzt grundlegende Unterstützung für Link- und Überschriftennavigation!
* RTF-Tabulatoren und Zeilenumbrüche werden jetzt genau so dargestellt, wie sie im Dokument erscheinen.
* Zurück zu der bewährten pdfium-Bibliothek für die PDF-Analyse gewechselt, wodurch die PDF-Darstellung wieder viel zuverlässiger ist.

### Version 0.8.1
* `Ctrl+Shift+T` hinzugefügt zum Erneut-Öffnen des zuletzt geschlossenen Dokuments.
* Der Dialog „Alle Dokumente" unterstützt jetzt das Auswählen mehrerer Dokumente zum gleichzeitigen Öffnen.
* Mehrere Fehler im RTF-Parser behoben.
* Dateipfade mit Nicht-ASCII-Zeichen (wie bosnisches š, č, ć, ž) werden nicht mehr beschädigt, wenn eine Datei über eine zweite Paperback-Instanz geöffnet wird.
* PDF-Text wird nicht mehr in falscher Reihenfolge gelesen, und die Abstände um großgeschriebene Wörter sind korrekt.
* Langsames Laden von Dokumenten beim Öffnen großer Dateien behoben.
* Lokalisierung der Ja/Nein-Schaltflächen in Bestätigungsdialogen behoben.

### Version 0.8.0
* Japanische, vereinfachte chinesische und vietnamesische Übersetzungen hinzugefügt!
* Automatische Aktualisierung hinzugefügt, die jetzt deine aktuell installierte Version von Paperback ersetzt, anstatt nur die neue Version herunterzuladen!
* Optionales Soundfeedback beim Erreichen eines Lesezeichens oder einer Notiz hinzugefügt, danke Andre Louis für die Sounds!
* RTF-Dokumentunterstützung hinzugefügt!
* Unterstützung für DAISY-XML-Dokumente hinzugefügt.
* Unterstützung für Flat-Open-Document-Text-Dateien hinzugefügt!
* Unterstützung für Flat-Open-Document-Präsentationen hinzugefügt!
* Unterstützung für Trennzeichen mit `s` und `Shift+s` hinzugefügt.
* Bewegungen über 300 Zeichen hinaus werden jetzt automatisch zur Navigationschronik hinzugefügt.
* Wiederherstellen von Paperbacks Fenster aus dem Systembereich behoben.
* Markdown-Dokumente zeigen jetzt in der Web View gerendertes HTML an statt Rohtext.
* Tabellen werden jetzt ordnungsgemäß in Markdown-Dateien gerendert.
* Nur-Bild-PDFs werden dich jetzt warnen, wenn du versuchst, eine zu laden.
* Versionsinformationen ordnungsgemäß in die Paperback-Programmdatei eingebettet.
* Optionsdialog in Reiter aufgeteilt zur einfacheren Nutzung und Navigation.
* Zu Hayro für PDF-Analyse gewechselt, was zu mehr Zuverlässigkeit, Geschwindigkeit und weniger DLLs führt.
* Die gesamte App in Rust neu geschrieben. Die neue Codebasis ist sicherer, lädt Dokumente schneller und ist leichter zu warten und zu erweitern.
* Das Kontextmenü des Text-Steuerelements enthält jetzt leserspezifische Aktionen statt generischer Elemente wie Ausschneiden und Einfügen.

### Version 0.7.0
* Tabellenunterstützung für HTML- und XHTML-basierte Dokumente hinzugefügt! Navigiere zwischen Tabellen mit T und Shift+T, und drücke Enter, um eine in einer Webansicht anzuzeigen.
* Grundlegende Webrendering-Funktion hinzugefügt! Drücke `Ctrl+Shift+V`, um den aktuellen Abschnitt deines Dokuments in einem webbasierten Renderer zu öffnen, nützlich für Inhalte wie komplexe Formatierungen oder Code-Beispiele.
* Russische Übersetzung hinzugefügt, danke Ruslan Gulmagomedov!
* Schaltfläche „Alle löschen" zum Dialog „Alle Dokumente" hinzugefügt.
* Der Update-Checker zeigt jetzt Versionshinweise an, wenn eine neue Version verfügbar ist.
* Wiederherstellen des Fensters aus der Taskleiste behoben.
* Ja/Nein-Schaltflächenübersetzungen in Bestätigungsdialogen behoben.
* Laden von Konfigurationen bei Ausführung als Administrator behoben.
* Kommentarverarbeitung in XML- und HTML-Dokumenten behoben.
* Inhaltsverzeichnis-Parsing in EPUB-2-Büchern behoben.
* Navigation zum nächsten Element mit demselben Buchstaben im Inhaltsverzeichnis behoben.
* Suchdialog versteckt sich jetzt ordnungsgemäß mit den Schaltflächen „Weiter"/„Zurück".
* Inhaltsverzeichnis von EPUB wirft dich gelegentlich zum falschen Element – behoben.
* Verschiedene Probleme bei der Leerzeichenbehandlung in XML, HTML und `pre`-Tags behoben.
* Fehler um eins bei der Link-Navigation behoben.
* Einige Bücher mit nachfolgenden Leerzeichen in ihren Zeilen behoben.
* Verschiedene Parser-Probleme behoben.
* Lesezeichen-bezogene Menüelemente sowie die Elementliste werden jetzt ordnungsgemäß deaktiviert, wenn kein Dokument geöffnet ist.
* Listenverarbeitung in verschiedenen Dokumentformaten verbessert.
* Übersetzungs-Arbeitsablauf für Mitwirkende verbessert.
* Viele interne Umstrukturierungen, wobei der Großteil der Geschäftslogik der Anwendung von C++ zu Rust verschoben wurde, um Leistung und Wartbarkeit zu verbessern.

### Version 0.6.1
* Passwortgeschützte PDF-Unterstützung hinzugefügt!
* Ein sehr grundlegendes Feature zum Navigieren zur vorherigen/nächsten Leseposition hinzugefügt. Wenn du die Eingabetaste auf einem internen Link drückst und dies deinen Cursor bewegt, wird diese Position nun gespeichert und kann mit `Alt+Left`/`Alt+Right` angesteuert werden.
* Eine Elementeliste hinzugefügt! Derzeit zeigt sie nur einen Baum aller Überschriften in deinem Dokument oder eine Liste von Links an, aber es sind Pläne vorhanden, dies in Zukunft zu erweitern.
* Eine Option hinzugefügt, um Paperback standardmäßig im maximierten Modus zu starten.
* Links in einigen EPUB-Dokumenten funktionieren nun richtig.
* Parsing von EPUB-Inhaltsverzeichnissen mit relativen Pfaden wurde korrigiert.
* Einige EPUB-Dokumente zeigen nun einen Titel oder Autor an.
* Die Titel einiger EPUB-Kapitel werden nun richtig im Inhaltsverzeichnis-Dialog angezeigt.
* Du kannst nun die Leertaste verwenden, um die Schaltflächen „OK" und „Abbrechen" im Inhaltsverzeichnis-Dialog zu aktivieren.
* Die Behandlung von Überschriften in Word-Dokumenten wurde verbessert.
* Du erhältst jetzt Sprachfeedback, wenn die Liste der letzten Dokumente leer ist und du versuchst, den Dialog zu öffnen.

### Version 0.6.0
* Eine neue Option zum Anzeigen des Menüs „Gehe zu" in einer viel kompakteren Form wurde zum Optionsdialog hinzugefügt und ist standardmäßig aktiviert.
* Eine Option zum Umschließen der Navigation durch strukturelle Elemente hinzugefügt.
* Eine Option zum Menü „Extras" hinzugefügt, um den Ordner mit dem derzeit fokussierten Dokument zu öffnen.
* Ein ziemlich einfaches, aber sehr effektives Aktualisierungssystem hinzugefügt.
* Eine grundlegende Schlafzeitgeberfunktion hinzugefügt, erreichbar mit `Ctrl+Shift+S`.
* Unterstützung für das Parsen von FB2-eBooks hinzugefügt!
* Unterstützung für das Parsen von OpenDocument-Präsentationen hinzugefügt!
* Unterstützung für das Parsen von OpenDocument Text-Dateien hinzugefügt!
* Lesezeichen können jetzt ein ganzes Zeichen oder nur etwas angegebenen Text als Lesezeichen setzen. Wenn beim Platzieren eines Lesezeichens keine Auswahl aktiv ist, verhält sich dies wie vor 0.6 und markiert das gesamte Zeichen. Wenn du jedoch einen Text auswählst, wird nur dieser Text in das Lesezeichen einbezogen.
* Lesezeichen können jetzt optionale Textnotizen enthalten! Navigiere zwischen Lesezeichen mit Notizen mit `N` und `Shift+N`, oder öffne den Lesezeichendialog mit allen Lesezeichen, nur Notizen oder nur Lesezeichen ohne Notizen mit spezifischen Hotkeys.
* Lesezeichen im Lesezeichendialog haben nicht mehr das lästige Präfix „Lesezeichen x".
* EPUB-Bücher mit HTML-Inhalten, die sich als XML ausgeben, werden nun korrekt verarbeitet.
* Laden großer Markdown-Dokumente behoben.
* Das Drücken von Leerzeichen in der Baumansicht des Inhaltsverzeichnisses, das die OK-Schaltfläche aktiviert, behoben.
* Whitespace-Behandlung am Anfang von Pre-Tags in HTML- und XHTML-Dokumenten behoben.
* Das Textfeld erhält manchmal nicht den Fokus zurück, wenn man zu Paperbacks Fenster zurückkehrt, behoben.
* Das Textfeld im Dialog „Gehe zu Prozent" aktualisiert den Wert des Schiebereglers nicht, behoben.
* Das Rendern von benutzerdefinierten HTML-IDs in Markdown-Dokumenten behoben.
* HTML innerhalb von Markdown-Codeblöcken wird nun korrekt gerendert.
* Wenn du ein Buch mit einem Befehlszeilenparameter lädst, während eine vorhandene Paperback-Instanz ausgeführt wird, erhältst du keine Fehlermeldung mehr, wenn das Laden deines Dokuments länger als 5 Sekunden dauert.
* Wenn Paperback als Administrator ausgeführt wird, wird die Konfiguration nun korrekt geladen und gespeichert.
* Es ist nun möglich, ein Lesezeichen direkt aus dem Lesezeichendialog zu löschen.
* Es ist nun möglich, deine Lesezeichen und Leseposition für ein bestimmtes Dokument zu importieren und zu exportieren. Die generierte Datei wird nach der Datei mit einer .paperback-Erweiterung benannt. Wenn eine solche Datei beim Laden in demselben Verzeichnis wie eine Datei gefunden wird, wird sie automatisch geladen. Andernfalls kannst du sie manuell mit einem Element im Menü „Extras" importieren.
* Links innerhalb von Dokumenten werden nun vollständig unterstützt! Verwende `K` und `Shift+K`, um vorwärts und rückwärts durch sie zu navigieren, und drücke `Enter`, um einen zu öffnen/aktivieren.
* Viele interne Umgestaltungen, die die App schneller und das Binärformat kleiner machen.
* Markdown-Inhalte werden nun vorverarbeitet, um vor dem Rendern CommonMark-konform zu sein.
* Navigation durch Listen und ihre Elemente wird nun vollständig unterstützt! Verwende `L` und `Shift+L`, um zu Listen selbst zu navigieren, und `I` und `Shift+I`, um durch Listenelemente zu gehen.
* Numpad Delete funktioniert jetzt zusätzlich zum normalen Delete, um Dokumente aus der Registerleiste zu entfernen.
* Paperback kann sich nun optional in dein Systemtablett minimieren! Diese Option ist standardmäßig deaktiviert, aber wenn du sie aktivierst, wird die Minimieroption im Systemmenü Paperback in dein Tablett legen, das durch Klicken auf das erstellte Symbol wiederhergestellt werden kann.
* Paperback ist nun vollständig übersetzbar! Die Liste der unterstützten Sprachen ist derzeit noch ziemlich klein, wächst aber ständig!
* Paperback hat jetzt eine offizielle Website unter [paperback.dev](https://paperback.dev)!
* PPTX-Dokumente zeigen jetzt ein grundlegendes Inhaltsverzeichnis mit allen Folien.
* Der vollständige Pfad zum geöffneten Dokument wird nun im Dokumentinfo-Dialog angezeigt.
* Das Installationsprogramm enthält jetzt eine Option, um die Readme nach der Installation in deinem Browser anzuzeigen.
* Die Liste der letzten Dokumente wurde dramatisch erweitert! Anstatt dir einfach die letzten 10 Dokumente zu zeigen, die du geöffnet hast, zeigt es dir jetzt eine anpassbare Anzahl an, wobei die übrigen Dokumente, die du jemals geöffnet hast, über einen kleinen Dialog erreichbar sind.
* Verschiedene kleine Verbesserungen an den Parsern insgesamt, einschließlich einer Leerzeile zwischen Folien in PPTX-Präsentationen, Behebung der Zeilenumbruchbehandlung innerhalb von Absätzen in Word-Dokumenten und Hinzufügen von Aufzählungszeichen zu Listenelementen.

### Version 0.5.0
* Unterstützung für Microsoft-Word-Dokumente hinzugefügt!
* Unterstützung für PowerPoint-Präsentationen hinzugefügt!
* Bestimmte Menüelemente werden nun korrekt deaktiviert, wenn keine Dokumente geöffnet sind.
* Die Ausrichtung des Schiebers für „Gehe zu Prozent" wurde korrigiert.
* Das Inhaltsverzeichnis in EPUB-Büchern mit URL-codierten Dateipfaden und/oder Fragment-IDs wurde korrigiert.
* Leerzeichen werden nicht mehr auf seltsame Weise aus XHTML-Überschriften entfernt.
* Die Behandlung von Leerzeichen innerhalb verschachtelter pre-Tags in HTML-Dokumenten wurde korrigiert.
* HTML- und Markdown-Dokumente unterstützen nun die Inhaltsverzeichnis-Funktion! Wenn du ein HTML-/Markdown-Dokument lädst, erstellt Paperback automatisch ein Inhaltsverzeichnis aus der Struktur der Überschriften in deinem Dokument und zeigt es dir im Dialog `Ctrl+T` an.
* HTML-Dokumente erhalten nun den Titel, der im title-Tag gesetzt ist, sofern dieser vorhanden ist. Andernfalls wird weiterhin der Dateiname ohne Erweiterung verwendet.
* Von UniversalSpeech gewechselt zu einer Live-Region für die Sprachausgabe. Das bedeutet, dass keine Bildschirmleser-DLLs mehr mit dem Programm ausgeliefert werden, und weitere Bildschirmleser werden nun unterstützt, wie beispielsweise Microsoft Narrator.
* Zip-Bibliotheken gewechselt, um das Öffnen einer größeren Vielfalt von EPUB-Büchern zu ermöglichen.
* Der Dialog, der dich fragt, ob du dein Dokument als Nur-Text öffnen möchtest, wurde vollständig überarbeitet und erlaubt dir nun, dein Dokument als Nur-Text, HTML oder Markdown zu öffnen.
* Der Dialog „Gehe zu Prozent" enthält nun ein Textfeld, mit dem du manuell einen Prozentsatz eingeben kannst, zu dem du springen möchtest.
* Der HTML-Parser erkennt nun dd, dt und dl als Listenelemente.
* Das Inhaltsverzeichnis in EPUB-Büchern wird nun wieder exakt beibehalten.
* Das Unicode-Leerzeichen ohne Umbruch wird nun beim Entfernen von Leerzeilen berücksichtigt.
* Du wirst nicht mehr gefragt, wie du eine unbekannte Datei öffnen möchtest, jedes Mal wenn du sie lädst, sondern nur beim ersten Mal.

### Version 0.4.1
* Ein optionales Startmenü-Symbol zum Installer hinzugefügt.
* Das Inhaltsverzeichnis sollte jetzt in einigen Fällen sauberer sein, zum Beispiel wenn du ein untergeordnetes und übergeordnetes Element mit demselben Text an derselben Position hast, wird jetzt nur das übergeordnete Element angezeigt.
* Das Inhaltsverzeichnis in bestimmten CHM-Dokumenten behoben.
* Das Inhaltsverzeichnis in EPUB 3-Büchern mit absoluten Pfaden behoben.
* CHM-Dokumente sollten jetzt ihren im Metadaten-File eingestellten Titel anzeigen.

### Version 0.4.0
* CHM-Dateiunterstützung hinzugefügt!
* Lesezeichen-Unterstützung hinzugefügt! Du kannst beliebig viele Lesezeichen in mehreren Dokumenten haben. Du kannst mit `b` und `Shift+b` vorwärts und rückwärts durch sie springen, eins mit `Ctrl+Shift+b` setzen und mit `Ctrl+b` einen Dialog öffnen, um zu einem bestimmten Lesezeichen zu springen.
* Ein Installationsprogramm neben der portablen ZIP-Datei hinzugefügt! Das Installationsprogramm installiert Paperback in dein Verzeichnis „Programme" und richtet automatisch Dateizuordnungen für dich ein.
* Textdateien mit BOMs werden jetzt korrekt dekodiert, und die BOM wird nicht mehr am Anfang des Texts angezeigt.
* Viel mehr Informationen zur Statusleiste hinzugefügt. Sie zeigt dir jetzt deine aktuelle Zeile, das Zeichen und den Lesefortschritt in Prozent an.
* HTML-Kommentare sowie der Inhalt von Script- und Style-Tags werden nicht mehr in der Textausgabe angezeigt.
* Wenn du einen relativen Pfad an Paperback in der Befehlszeile übergibst, wird er jetzt korrekt aufgelöst.
* Prozentuale Bewegung wird jetzt von einem eigenen Schieberegler-Dialog verwaltet, der mit `Ctrl+Shift+g` erreichbar ist.
* Dokumente ohne bekannte Titel oder Autoren haben jetzt immer einen Standard.
* Die Logik zum Speichern der Leseposition ist jetzt viel intelligenter und sollte nur noch schreiben, wenn es absolut notwendig ist.
* Das Dokument, das du hattest, als du Paperback geschlossen hast, wird jetzt über Neustarts der Anwendung hinweg gespeichert.
* Die Eingabe in die Dialoge „Gehe zu Zeile" und „Gehe zu Seite" wird jetzt strikter bereinigt.
* Navigation im Inhaltsverzeichnis in EPUB-3-Büchern mit relativen Pfaden in ihren Manifesten behoben.

### Version 0.3.0
* Inhaltsverzeichnis in EPUB-Büchern mit URL-codierten Manifesten wurde korrigiert.
* Überschrift-Navigation in HTML-Dokumenten mit mehrbyte-Unicode-Zeichen wurde korrigiert.
* Hoher CPU-Verbrauch in Dokumenten mit langen Titeln aufgrund einer Regression in wxWidgets wurde korrigiert.
* Laden von UTF-8-Textdateien wurde korrigiert.
* Verschachtelte Inhaltsverzeichnis-Elemente in EPUB-Büchern, die den Cursor an die falsche Position setzten, wurden korrigiert.
* Ein Absturz beim Beenden der Anwendung in bestimmten Fällen wurde korrigiert.
* Ein Kontrollkästchen im Optionsdialog zum Aktivieren oder Deaktivieren des Zeilenumbruchs wurde hinzugefügt!
* Es ist jetzt möglich, Paperbracks Entwicklung zu unterstützen, entweder über den neuen Spenden-Eintrag im Hilfemenü oder über den Link „Sponsor this project" am unteren Ende der Hauptseite des GitHub-Repositorys.
* Markdown-Dokumente haben jetzt immer einen Titel, und Paperback sollte jetzt in der Lage sein, praktisch jede Markdown-Datei zu laden.
* PDF-Dokumente haben jetzt immer einen Titel, auch wenn die Metadaten fehlen.
* PDF-Bibliotheken wurden auf die in Chromium verwendete umgestellt, was zu einer deutlich zuverlässigeren PDF-Analyse insgesamt führt.
* Es können jetzt nur noch eine Instanz von Paperback gleichzeitig ausgeführt werden. Das Ausführen von paperback.exe mit einem Dateinamen, während es bereits ausgeführt wird, öffnet das Dokument in der bereits ausgeführten Instanz.
* Du kannst jetzt die Löschtaste auf einem Dokument in der Registerkarte drücken, um es zu schließen.

### Version 0.2.1
* Die Gesamtzahl der Seiten wurde zum Seitenlabel im Dialog „Seite ansteuern" hinzugefügt.
* Tabbing vom Dokumentinhalt zur Liste der geöffneten Dokumente ermöglicht.
* Der Fehler wurde behoben, dass die Tastenkombinationen für Überschriften manchmal die zuletzt verwendeten Dokumente öffneten, wenn genug davon vorhanden waren.
* Paperback entfernt jetzt unnötige bedingte Trennstriche aus der Textausgabe.
* Der Fehler wurde behoben, dass die Navigation nach Überschriften dich manchmal auf das falsche Zeichen setzte.

### Version 0.2.0
* Unterstützung für Markdown-Dokumente hinzugefügt!
* Unterstützung für PDF-Dokumente hinzugefügt, einschließlich der Möglichkeit, zwischen Seiten zu navigieren!
* Tastenkombinationen zum Navigieren nach Überschriften in HTML-Inhalten hinzugefügt, einschließlich EPUB-Bücher und Markdown-Dokumente. Diese Tastenkombinationen wurden so konzipiert, dass sie ähnlich wie ein Bildschirmleser funktionieren.
* Laden von EPUBs mit URL-codierten Dateinamen in ihren Manifesten behoben.
* Laden von EPUB-3-Büchern mit darin eingebettetem XHTML behoben.
* Eine Meldung wird nun angesagt, wenn das Dokument kein Inhaltsverzeichnis oder Abschnitte unterstützt, anstatt dass die Menüelemente deaktiviert sind.
* Menü für zuletzt verwendete Dokumente hinzugefügt! Es speichert derzeit die letzten 10 geöffneten Dokumente, und die Eingabetaste auf einem Dokument öffnet es zum Lesen.
* Dialog „Suchen" komplett neu geschrieben, wodurch er viel einfacher zu verwenden ist, außerdem wurde ein Verlauf der letzten 25 Suchen und Unterstützung für reguläre Ausdrücke hinzugefügt!
* Zuvor geöffnete Dokumente werden nun über Neustarts der Anwendung hinweg gespeichert. Dies ist konfigurierbar über den neuen Eintrag „Optionen" im Menü „Extras".
* `Shift+F1` zum direkten Öffnen der Readme in Paperback hinzugefügt.

### Version 0.1.0
* Erste Veröffentlichung.
