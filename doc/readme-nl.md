<!-- machine-translated from doc/readme.md (source-hash: 651d0b411879a6d8; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ce87a64f,a9eba369,e9860ee8,007c0542); please review and edit as needed -->

# Paperback - versie 1.0

## Inleiding

Paperback is een lichte, snelle en toegankelijke lezer voor ebooks, documenten en audioboeken, voor iedereen, van casual lezers tot heavy power users. Het is ontworpen voor schermlezer-toegankelijkheid, hoge snelheid en een onbelaste ervaring.

## Systeemvereisten

Paperback draait op Windows 10/11, alle moderne versies van ARM macOS, Linux, iOS 17 en hoger, en Android 7 en hoger. De iOS- en Android-apps zijn beschikbaar in de App Store en Google Play.

## Functies

* Volledig zelfstandig, zonder dat software op uw computer hoeft te worden geïnstalleerd om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op voor elk document dat u opent.
* Onthoudt optioneel welke documenten u had geopend toen u het programma sloot en herstelt deze bij de volgende start.
* Bevat navigatiefunctionaliteit vergelijkbaar met die in de webbrowsermodus van veel schermlezers voor snelle en gemakkelijke navigatie door documenten.
* Bevat een robuuste zoekdialoog, met functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of worden geïnstalleerd met bestandskoppelingen die automatisch worden ingesteld.
* Ondersteunt een groot aantal veelgebruikte bestandsindelingen.
* Speelt audioboeken af, met verstelbare snelheid en bladwijzers die het exacte moment onthouden.
* Leest gescande PDF-pagina's met de OCR ingebouwd in Windows en macOS.
* Bladwijzers en notities, zodat u uw plaats kunt markeren en er later naar teruggaan.
* Elk toetsenbordsnelkoppeling kan worden gewijzigd.
* Wordt geleverd met `pb`, een opdrachtregelhulpprogramma dat elk ondersteund document naar HTML, Markdown of platte tekst converteert.

## Schermlezerscompatibiliteit

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en brailleweergaven

Als u JAWS gebruikt met een brailleweergave, kunt u merken dat lange alinea's worden afgekapt wanneer u vooruit schuift met de navigatietoetsen van uw weergave. De opdracht voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een bug in de JAWS-verwerking van het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf, en een bug die nog een hele tijd heeft geduurd voordat een oplossing aan het licht kwam gezien Vispero's enthousiasme voor het reageren op problemen met open source software.

De workaround, uiteindelijk ontdekt door de JAWS-discussiegroep na maanden wachten, is het bewerken van `paperback.jcf` en het instellen van "Braille Presentation and Panning" op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw weergave op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen op hun plaats, zou schuiven correct moeten werken.

## Op dit moment ondersteunde bestandstypen

Paperback ondersteunt de volgende indelingen en extensies:

* Comic book-archieven (`.cbz`)
* CHM-hulpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-ebooks (`.fb2`)
* HTML-documenten (`.htm`, `.html`, `.xhtml`)
* Handleidingspagina's, zowel `man` als BSD `mdoc` (`.1` tot `.9`, `.man`, `.roff`, en de gecomprimeerde vormen van elk)
* Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
* M4B-audioboeken (`.m4b`)
* MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
* MP3-audioboeken (`.mp3`)
* OpenDocument-presentaties (`.odp`, `.fodp`)
* OpenDocument-tekstbestanden (`.odt`, `.fodt`)
* PDF-documenten (`.pdf`)
* PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
* RTF-documenten (`.rtf`)
* Windows Write-documenten (`.wri`)
* WinHelp-bestanden (`.hlp`)
* Platte tekst- en logbestanden (`.txt`, `.log`)

## Toetsenbordsnelkoppelingen

Paperback is ontworpen voor gebruik met het toetsenbord voorop. Hier zijn de huidige snelkoppelingen.

De onderstaande snelkoppelingen zijn voor Windows. Waar macOS afwijkt, staat het equivalent tussen haakjes vermeld — vooral omdat `Ctrl+G`, `Ctrl+W` en `Alt+Left`/`Right` al zijn geclaimd door andere systeem- of app-conventies op dat platform.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document heropenen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (uit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS bevindt dit zich in plaats daarvan in het app-menu).

### Menu Gaan naar

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (wanneer ondersteund door het huidige document).
* `=`: Uw huidige leespercentage en pagina aankondigen, bijv. "15%, pagina 30". De pagina wordt weggelaten voor documenten zonder paginanummers.
* `Alt+Left` (macOS: `Cmd+[`): Teruggaan in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruit gaan in navigatiegeschiedenis.
* `[`: Vorige sectie.
* `]`: Volgende sectie.
* `Shift+H`: Vorige kop.
* `H`: Volgende kop.
* `Shift+1` tot `Shift+6`: Vorige kop op niveau 1-6.
* `1` tot `6`: Volgende kop op niveau 1-6.
* `Shift+P`: Vorige pagina.
* `P`: Volgende pagina.
* `Shift+B`: Vorig bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Uw tijdelijke bladwijzer instellen.
* `\`: Naar uw tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Naar alleen bladwijzers springen.
* `Ctrl+Alt+M`: Naar alleen notities springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dus de fysieke Control-toets in plaats van Cmd): Notitietekst op de huidige positie weergeven.
* `Shift+K`: Vorige link.
* `K`: Volgende link.
* `Shift+G`: Vorige afbeelding.
* `G`: Volgende afbeelding.
* `Shift+F`: Vorige figuur.
* `F`: Volgende figuur.
* `Shift+T`: Vorige tabel.
* `T`: Volgende tabel.
* `Shift+M`: Vorige formule.
* `M`: Volgende formule.
* `Shift+S`: Vorige scheidingslijn.
* `S`: Volgende scheidingslijn.
* `Shift+L`: Vorige lijst.
* `L`: Volgende lijst.
* `Shift+I`: Vorig lijstitem.
* `I`: Volgende lijstitem.
* `Shift+,`: Naar het begin van de huidige container (lijst of tabel) gaan.
* `,`: Voorbij het einde van de huidige container (lijst of tabel) gaan.

### Menu Hulpmiddelen

* `Ctrl+W` (macOS: `RawCtrl+W`, dus de fysieke Control-toets in plaats van Cmd): Woordentelling voor het huidige document weergeven.
* `Ctrl+I`: Documentinfo weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Map met bestand openen.
* `Ctrl+Shift+V`: Huidige inhoud in Webweergave openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernotitie op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Tekstomloop in-/uitschakelen.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, dus de fysieke Control-toets, omdat Cmd+Space Spotlight opent): Audio-naratie afspelen/onderbreken.
* `'`: Audio-naratie vooruit spoelen.
* `;`: Audio-naratie achteruit spoelen.
* `Shift+'`: De audio-spoelingsvooruitgang vergroten.
* `Shift+;`: De audio-spoelingsvooruitgang verkleinen.
* `Ctrl+Shift+.`: Audio-naratie versnellen.
* `Ctrl+Shift+,`: Audio-naratie vertragen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dus Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Instellingen openen (macOS: in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.
* `Ctrl+Shift+O`: Een reeks gescande PDF-pagina's herkennen met OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Het begin van een selectie markeren, zodat alles van hier naar waar u ook gaat in één keer kan worden gekopieerd.
* `Alt+F10` (macOS: `Cmd+F10`): Alles van het gemarkeerde begin van de selectie naar de huidige positie kopiëren.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Teruggaan naar het gemarkeerde begin van de selectie, met het merk op zijn plaats.

### Menu Help

* `Ctrl+F1`: Het dialoogvenster Info weergeven.
* `F1`: Help weergeven in uw standaardbrowser.
* `Shift+F1`: Help weergeven in Paperback.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: De donatieportal openen in uw standaardbrowser.

### Aanvullende documentweergavetoetsen

* `Delete` / `Numpad Delete` op het tabtabblad: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Een link volgen of een tabel- of formuleweergave op de cursor openen.
* `Enter` op een gescande PDF-pagina: De pagina herkennen met OCR.
* `Shift+F10` of de Menu/Toepassingstoets in de documenttekst: Het contextmenu openen.

## iOS en Android

De iOS- en Android-apps gebruiken dezelfde lees-engine als de desktop, dus ze openen dezelfde indelingen en onthouden je plaats op dezelfde manier. Ze zijn gebouwd om te worden gebruikt met VoiceOver op iOS en TalkBack op Android.

### Documenten openen

* Gebruik de Open Book-knop, of open een document vanuit de Files-app of een andere app en kies Paperback.
* Op Android kunt u in plaats daarvan de ingebouwde bestandsbrowser inschakelen in Instellingen. Hiervoor is de machtiging Alle bestanden openen nodig, en grote bestanden worden onmiddellijk geopend in plaats van eerst te worden gekopieerd.
* Houd de Open Book-knop ingedrukt om de gegevens van een document (`.paperback`) te importeren of exporteren, dezelfde bestanden die de desktop-app gebruikt.

### Lezen en luisteren

Elke app heeft twee manieren om een document te lezen. In de tekstmodus leest u de tekst met uw schermlezer. In de voorleesmodus leest Paperback de tekst voor u met de stem die u kiest in Instellingen, en gaat door op de achtergrond en vanaf het vergrendelingsscherm. Schakel tussen beide vanuit het menu Meer opties.

Audioboeken, zoals DAISY, M4B en MP3-boeken, spelen in plaats daarvan hun eigen opname af.

### De lezesbalk

De balk aan de onderkant van het scherm heeft van links naar rechts:

* De navigatie-eenheid, zoals alinea, kop, pagina of link. Veeg omhoog of omlaag om deze te wijzigen.
* Vorige, afspelen en volgende knoppen. Vorige en volgende verplaatsen zich per navigatie-eenheid.
* De spreeksnelheid. Veeg omhoog of omlaag om te wijzigen hoe snel Paperback voorleest.

U kunt ook omhoog of omlaag vegen op de afspeelknop om per navigatie-eenheid te verplaatsen, zonder de vorige en volgende knoppen te bereiken. Wanneer u alleen deze optie gebruikt, verwijdert de instelling Vorige en volgende knoppen verbergen deze uit het bereik van uw schermlezer. Met de instelling Omhoog vegen beweegt vooruit kunt u bepalen welke richting een veeg opgaat.

### Meer opties

Het menu Meer opties bevat al het andere. Sommige items werken op elke app iets anders.

* **Overschakelen naar TTS-modus of Overschakelen naar tekstmodus:** schakelt tussen voorleesmodus en tekstmodus, hierboven beschreven. Android heeft ook een item Hardop lezen waarmee u het hardop lezen kunt starten en onderbreken.
* **Inhoudsopgave:** de hoofdstukken van het boek. Kies er een om direct ernaar toe te gaan. Op Android kunnen vermeldingen met hoofdstukken eronder worden uitgevouwen en samengevouwen met behulp van de acties van de schermlezer. Op iOS wordt de hele lijst tegelijk weergegeven.
* **Elementen:** een lijst met de koppen of links van het document. Wissel tussen beide met de Type-kiezer op iOS of de tabbladen op Android, en kies er een om ernaar toe te gaan.
* **Zoeken:** typ wat u wilt zoeken en kies of u hoofdletters wilt afstemmen, alleen hele woorden wilt afstemmen of een reguliere expressie wilt gebruiken. Op Android blijft een balk met Vorige zoeken en Volgende zoeken onderaan het scherm totdat u deze sluit, en eerdere zoekopdrachten staan onder Zoekgeschiedenis. Op iOS bevinden de knoppen Vorige zoeken en Volgende zoeken zich op het zoekscherm, en Zoeken verschijnt ook als navigatie-eenheid op de lezesbalk, zodat u de resultaten van daaruit kunt doorlopen.
* **Ga naar:** spring naar een regel, pagina of percentage door het document. Kies welke met de Mode-kiezer.
* **Recente documenten:** elk document dat u hebt geopend, elk gemarkeerd als momenteel geopend, gesloten of bestand ontbreekt. Elk document heeft twee acties van de schermlezer: Verwijderen haalt het uit de lijst, en Zoeken helpt u een document te vinden waarvan het bestand is verplaatst. Recente documenten wissen leegt de lijst zonder documenten te verwijderen.
* **Woordaantal:** het aantal woorden in het document.
* **Documentinfo:** de titel, de auteur, de bestandsnaam en op iOS ook het aantal regels en tekens.
* **Exporteren:** slaat het document op als gewone tekst, HTML of Markdown.
* **Slaaptimer:** stopt met voorlezen na 5, 10, 15, 30, 45 of 60 minuten, of een moment naar keuze. Open het opnieuw terwijl het actief is om te zien hoeveel tijd er nog over is, of om het te annuleren.
* **Help:** opent dit leesmij.
* **Instellingen:**
    * **Tekst naar spraak:** de stem, spreeksnelheid en toonhoogte, een knop Voorbeeld afspelen om deze te horen en de pauze tussen alinea's. Android stelt u ook in staat de spraakengine te kiezen. Op iOS is dit ook waar het spraakwoordenboek staat: regels die bepalen hoe woorden worden uitgesproken, voor elke stem of slechts voor sommige.
    * **Leesbaarheid:** tekstgrootte, regelafstand, alineaafstand en uitlijning. iOS heeft ook licht en donker uiterlijk en contrastrijke tekst.
    * **Gedrag:** of u uw documenten opnieuw moet openen wanneer de app wordt gestart, welke richting een veeg op de afspeelknop inslaat en of u de vorige en volgende knoppen moet verbergen. Android heeft ook hier de ingebouwde bestandsbrowser.

### Toetsenborden en headsets

Met een toetsenbord werken de desktopsnelkoppelingen voor het openen van boeken, recente documenten, Zoeken, Ga naar, de inhoudsopgave, woordaantal, documentinfo, exporteren en de slaaptimer allemaal, met `Cmd` in plaats van `Ctrl` op iOS. Ook de letters met één teken voor verplaatsen per kop, pagina, link enzovoort, en `Space` om af te spelen en te pauzeren. Op iOS bereikt de toetsen met één teken alleen Paperback terwijl VoiceOver's Quick Nav met één teken uit staat.

Op Android speelt een headset-knop af en pauzeren met één druk, beweegt vooruit met twee en gaat terug met drie.

## Ondersteunde talen

Paperback is in veel verschillende talen vertaald, en er worden er voortdurend meer toegevoegd. Hieronder volgt een volledige lijst.

Om te leren hoe u kunt bijdragen, lees dan onze [Vertaalgids](translating.md).

* Bosnisch
* Tsjechisch
* Nederlands
* Fins
* Frans
* Duits
* Japans
* Pools
* Portugees (Brazilië)
* Russisch
* Vereenvoudigd Chinees
* Servisch
* Spaans
* Oekraïens
* Vietnamees

## Credits
### Development
* Quin Gillespie: primary developer and project founder.
* Aryan Choudhary: primary contributor.

### Donations
De volgende personen hebben donaties gedaan aan Paperback development. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd. Ik voeg alleen personen toe die willen dat hun donatie openbaar wordt gemaakt.

Opmerking: Ik beschouw een openbare GitHub sponsor als voldoende reden voor automatische opname in deze lijst.

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

1.0 is de eerste release op alle vijf platforms: Windows, macOS, Linux, iOS en Android, met de iOS en Android apps in de App Store en Google Play.

#### Added

##### General
* Linux support, als AppImage of tar.gz, met desktop integratie zodat documenten openen vanuit uw bestandsbeheerder.
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles van daar naar waar u bent met `Alt+F10`, en ga terug naar de markering met `Alt+Shift+F9`, voor het kopiëren van een lange span tekst zonder shift-arrows. Alle drie staan onder Tools > Select and copy.
* De `=` sneltoets kondigt nu zowel de pagina als het percentage aan, bijvoorbeeld "15%, pagina 30", en blijft zoals het was voor documenten zonder paginanummers.
* De About box toont nu de licentie van Paperback en elke vertaler.
* Een Oekraïense vertaling.

##### New Formats
* Comic book archives (`.cbz`).
* M4B audioboeken, verdeeld in hun hoofdstukken.
* Manual pages, zowel `man` als BSD `mdoc`, gecomprimeerd of niet.
* MP3 audioboeken, verdeeld in hoofdstukken wanneer het bestand deze heeft.
* Windows Write (`.wri`) bestanden.
* WinHelp (`.hlp`) bestanden.
* Word 6 en Word 95 documenten.

##### OCR
* Gescande PDF-pagina's kunnen nu worden herkend met de OCR die is ingebouwd in Windows en macOS. Druk op `Enter` op een gescande pagina om deze te herkennen, of gebruik Batch OCR (`Ctrl+Shift+O`) voor een bereik van pagina's.

##### Navigation
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met behulp van MathCAT. Gebruik `M` of `Shift+M` om formules te navigeren, druk vervolgens op `Enter` of `Space` om de originele MathML in Formula View te openen.
* Een Find All knop in het Find dialoogvenster, waarin elke regel met een match wordt vermeld, zodat u direct naar degene kunt springen die u wilt.
* Tables, Lists en Pages views in de elementenlijst (`F7`).
* Go to Line, Go to Page en Go to Percent accepteren nu `+n` en `-n` om relatief te verplaatsen van waar u bent.
* EPUB, MOBI en CHM boeken zonder eigen koppelingen krijgen nu koppelingennavigatie uit hun inhoudsopgave.
* KF8 (AZW3) boeken ondersteunen nu sectienavigatie.
* EPUB-pagina's die alleen een afbeelding zijn, tonen nu een regel ervoor, zodat u erop kunt landen in plaats van er overheen te gaan.

##### Audio Books
* Besturingselementen voor afspeelsnelheid, van halve snelheid tot drie keer zo snel. Gebruik `Ctrl+Shift+.` en `Ctrl+Shift+,`, of het Tools menu.
* Bladwijzers en notities in alleen-audio boeken onthouden nu het exacte moment waarop u ze hebt ingesteld.
* Next en previous position (`Alt+Left` en `Alt+Right`) werken nu ook in audioboeken.
* Voortgang door een audioboek wordt nu gemeten aan de hand van de opname, dus Go to Percent en de statusbalk stemmen overeen met hoe ver u werkelijk bent.

##### Recent Documents
* Een Clear Recent Documents item in het Recent Documents submenu.

##### PDF Documents
* Een instelling om elke regel van een PDF gescheiden te houden, in plaats van deze samen te voegen tot alinea's.
* Afbeeldingen en figuren in PDF's worden nu aangekondigd.
* PDF's die leesstructuur bevatten maar geen van hun afbeeldingen labelen, kondigen die afbeeldingen nu aan, in plaats van ze volledig uit het boek weg te laten.

##### Web View
* Elk document kan nu worden geopend in de webweergave, niet alleen EPUB, HTML en Markdown.

##### Readability
* Koppelingen worden nu getekend met een grootte die overeenkomt met hun niveau, en afbeeldingen en tabellen zijn van de omringende tekst gescheiden.

##### pb
* `pb --list-formats` geeft een lijst van elk formaat dat pb kan lezen.
* pb zegt nu welk bestand het niet kon lezen en waarom.

#### Opgelost

##### Algemeen
* Een crash bij het sluiten van Paperback is opgelost.
* Paperback verbergt het venster nu direct wanneer u het sluit, in plaats van het op het scherm te laten staan terwijl het opslaat.
* Het openen van een document schakelt Opnieuw openen laatst gesloten niet meer in wanneer er niets opnieuw moet worden geopend.
* Paperback probeert documenten in uw recente lijst die verdwenen zijn niet langer opnieuw te laden, en beperkt hoeveel recente documenten het opslaat.
* Het oude INI-instellingenbestand wordt nu verwijderd nadat het naar het nieuwe formaat is verplaatst.
* De titels van de lettertype- en kleurendialogen en het menu Exporteren als in het Vietnamees zijn nu vertaald.
* Bij het bijwerken wordt het heraangezette venster nu naar voren gebracht, in plaats van het achter alle andere vensters in Alt+Tab te laten.
* Woordomloop wordt nu direct op grote documenten toegepast, in plaats van het hele document opnieuw te laden.

##### Navigatie
* `Alt+Left` gaat nu terug naar de plaats van waaruit u bent gesprongen, in plaats van naar een oudere positie.
* Geluid van bladwijzers wordt nu alleen afgespeeld wanneer u over een bladwijzer beweegt, niet wanneer u op de regel ervan terechtkomt.
* Het sluiten van de inhoudsopgave, de elementenlijst en de dialogen Ga naar brengt u nu direct naar de regel waarop u terechtkomt, in plaats van u door de schermlezer de inhoud van het venster opnieuw te laten aanhoren.
* Ga naar regel, Ga naar pagina en Ga naar procent weigeren nu getallen buiten het document, in plaats van stilzwijgend ergens anders heen te gaan.
* NVDA verbreekt de aankondiging niet langer wanneer een document geen pagina's heeft.
* Het indrukken van OK in de inhoudsopgave zonder te verplaatsen gaat nu naar de vermelding die al was geselecteerd.
* De inhoudsopgave, de elementenlijst en de bladwijzerslijst vertonen niet langer vertragingen of bevriezen op boeken met duizenden vermeldingen.
* De pijlen omhoog en omlaag onthouden nu hun kolom per document, in plaats van deze mee te voeren wanneer u van tabblad wisselt.

##### Audioboeken
* Afspelen van audio gebruikt nu `Control+Space` op macOS, omdat `Command+Space` toebehoort aan Spotlight.

##### PDF-documenten
* PDF's die uit Apple Pages zijn geëxporteerd en als platte tekst werden gelezen, zonder de koppen en lijsten waarmee ze waren geschreven, zijn opgelost.
* PDF-alinea's en koppen die op elke regel werden gesplitst, en woorden die op spaties werden gesplitst, zijn opgelost.
* Genummerde PDF-koppen die in elkaar opliepen, zijn opgelost.
* PDF's waarvan de structuurboom tot geen tekst leidt en leeg werden geopend, zijn opgelost.
* Paginakopteksten en voetteksten worden niet langer op elke pagina van niet-getagde PDF's gelezen.
* PDF's die hun paginakopteksten en voetteksten als gewone tekst taggen, herhalen de titel en het paginanummer niet langer tussen twee alinea's op elke pagina.
* PDF's geven nu hun echte titel weer, niet hun bestandsnaam.
* Regels ingesteld in een monospaced lettertype, zoals code, worden niet langer samengevoegd tot alinea's.

##### MOBI/AZW3-boeken
* Grote MOBI-boeken raken niet langer zonder geheugen en worden niet langer na 20 MB afgekapt.
* MOBI- en AZW3-boeken worden nu veel sneller geopend.
* MOBI-boeken die hun hoofdstuklijst verloren, zijn opgelost.
* Verminkte tekst waarbij MOBI-boeken van het ene record naar het volgende overgaan, is opgelost.

##### Webweergave
* De webweergave laadt niet langer het geheel van een enorm boek in één keer.
* De webweergave toont documenten nu volledig wanneer de lezer ze volledig toont, in plaats van slechts een gedeelte ervan.

##### Overige formaten
* FictionBook-boeken (.fb2) geschreven in windows-1251, wat de meeste zijn, worden nu geopend in plaats van volledig niet te kunnen worden gelezen.
* FictionBook-boeken die een naamruimte of HTML-entiteit gebruiken die ze nooit hebben gedeclareerd, worden nu geopend, in plaats van als verbroken te worden geweigerd.
* Boeken in verouderde coderingen worden nu veel sneller geopend.
* Sommige Chinese tekstbestanden die als verminkte tekst werden geopend, zijn opgelost.
* Wachtwoord-beveiligde OpenDocument-bestanden vragen nu om hun wachtwoord, in plaats van als verbroken te worden gerapporteerd.
* Wachtwoord-beveiligde verouderde PowerPoint-bestanden worden nu geopend, en verouderde PowerPoint-dia's verliezen hun tekst niet langer.
* Platte tekstbestanden opgeslagen met een `.rtf`-extensie worden nu als tekst geopend, in plaats van met een fout te mislukken.
* RTF-besturingselementen verschijnen niet langer als tekst.

#### iOS en Android

De iOS- en Android-apps openen elk formaat dat de desktop doet, en bevatten:

* Luid voorlezen, met uw keuze van stem, tempo en toonhoogte, een spraaksnelheidsregelaar direct op de leesregel, en een optionele pauze tussen alinea's.
* Afspelen van DAISY-, M4B- en MP3-audioboeken, die op de achtergrond en vanaf het vergrendelingsscherm doorlopen.
* Navigatie per kop, pagina, koppeling, tabel, lijst en meer vanaf de leesregel, plus de inhoudsopgave en Zoeken.
* Een slaaptimer, woordentelling en documentexport, plus een spraakwoordenboek op iOS.
* Opties voor tekstgrootte en afstand, plus tekst met hoog contrast op iOS.
* Sneltoetsen die met het bureaublad overeenkomen.

### Versie 0.9.2
* Audioboeken zorgen niet langer ervoor dat uw schermlezer een reeks spaties voorleest wanneer u de tekstinvoer focust.
* Audioboeken benoemen nu het bestand terwijl u er doorheen stapt per sectie.
* Audioboeken geven nu hun werkelijke lengte aan, in plaats van te claimen dat elk bestand erin 24 uur duurt.
* Het sluiten van de Web View met Escape werpt niet langer een debug-waarschuwing op nadat u een link erin bent gevolgd.
* Kopiëren na Selecteer alles geeft u nu het hele document, in plaats van alleen het gedeelte ervan dat momenteel is geladen.
* Zoeken gaat nu rechtstreeks naar de regel die het gevonden heeft, in plaats van u door de schermlezer het venster opnieuw te laten voorlezen terwijl de focus terugkeert naar het boek.
* Vaste EPUB's die een verdwaald ZIP64-blok bevatten en weigerden te openen met "Invalid local file header".
* Lange documenten hersteld die naar het begin teruglopen terwijl een schermlezer er continu doorheen las.
* Links in de WebView brengen u nu naar de sectie waar ze naar wijzen, in plaats van te mislukken met "File not found".
* De automatische aankondiging "Document opnieuw geladen" onderbreekt uw schermlezer niet langer halverwege een zin, maar wacht tot deze klaar is.
* Het tabblad Algemeen van het dialoogvenster Instellingen gaat nu door zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor het controleren op updates.
* Windows geeft nu altijd "Paperback" weer in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordaantal en Documentinfo geven nu aan hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Versie 0.9.1
* Bladwijzer- en aantekeninggeluiden worden nu op macOS afgespeeld.
* DAISY-boeken spelen hun audio nu op macOS af, in plaats van hun tijdlijn in stilte te openen en bij te houden.
* Krullanhalingstekens, em-dashes en vergelijkbare tekens vastgesteld die uit RTF-documenten verdwenen, waardoor de omringende woorden werden samengebracht.
* RTF-afbeeldingen vastgesteld die hun ruwe gegevens als onleesbare tekst in het document lekten.
* Het submenu Recente documenten vastgesteld dat stale entries bewaarde totdat iets anders dit opnieuw opbouwde.
* Toetsenbordversnellers zijn terug in elke vertaling, dus de menu's van Russisch hebben weer toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten worden nu bij Windows geregistreerd, zodat ze in de taakbalk springen en in de recente lijst van het menu Start verschijnen.
* Opties is hernoemd naar Instellingen, wat overeenkomt met de mobiele apps en op macOS met de platformconventie.
* Paperback onthoudt nu zijn vensterposition, grootte en gemaximaliseerde status tussen runs.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen, lezen correct in talen waarvoor meer dan één vorm nodig is.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de bijbehorende tekst.
* De actienamen in het dialoogvenster Toetsenbordsnelkoppelingen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken in de taakbalk en Alt+Tab kunnen worden onderscheiden.
* De updatedialog wordt nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool genaamd pb, om snel elk van Paperbacks ondersteunde formaten naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een View Source-optie om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst is nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden nu in slechts een paar seconden kunt laden. Rapporteer alstublieft eventuele vreemdigheden die je hiermee tegenkomt.

##### Platform-ondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een volledig scherm in-/uitschakelen.

##### Dialoogvenster Alle documenten
* Een locatieknop om ontbrekende boeken te vinden die net van locatie zijn veranderd.
* Een statusfilter en statusbalk, zodat je op documentstatus kunt filteren en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De `Ctrl+Shift+A`-sneltoets om alle documenten af te selecteren.

##### Opties en leesbaarheid
* Een leesbaarheid tabblad met de volgende opties:
    * Regelomslag (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een regelomslag-menu-item en volgende sneltoets.
* Een schakelaar om te bepalen hoe je tabellen wilt weergeven, en eenheid in de manier waarop tabellen in alle documenten worden weergegeven.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de bladermodus in schermlezeers.
* De gelijkteken-sneltoets om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en deze worden opgeslagen. Gebruik schuine streep om een bladwijzer in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het dialoogvenster woordentelling, evenals de mogelijkheid om je leessnelheid in te stellen om deze meting daadwerkelijk nuttig te maken.
* Als een selectie actief is wanneer je het dialoogvenster woordentelling opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een aanpasbare toetsenbordsneltoets om Paperback uit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het exportmenu-item uitgebreid om naar HTML en Markdown te exporteren, naast platte tekst.

##### Updater
* Een annuleringsknop in het dialoogvenster met lopende update.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audio-afspeling.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunt zowel DAISY-audio (inclusief DAISY-audio + tekst) als zipbestanden met audiobestanden.
* Toetsenbordsneltoetsen en menu-items om naratie af te spelen/onderbreken, vooruit en achteruit te zoeken, en de zoekbedrag aan te passen.
* Opties om de leescursor te synchroniseren met audioafspeling, het audiosoekhoeveelheid in te stellen en te kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als mojibake.
* "Reopen last closed" probeerde het gebundelde readme-bestand opnieuw te openen.
* Uw geselecteerde tabblad wordt niet goed gefocust na het herstarten van Paperback.
* De verwerking door Paperback van bestanden op Windows-netwerkstations: "show file in folder" focust nu correct op het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u om bevestiging gevraagd wanneer er een wordt gevonden.
* "Open containing folder" focust nu het gegeven bestand in explorer.
* Het openen van het readme-bestand respecteert nu uw geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op displays met hoge DPI.
* Het menu wordt nu correct bijgewerkt en de focus verschuift naar het tekstbesturingselement wanneer u hulp in Paperback opent.
* Overgeschakeld naar een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu gelezen bij het schakelen tussen tabbladen.
* Verminderd geheugengebruik op grote documenten door de grootte van de interne per-teken-indextabellen te halveren.

##### Dialoogvenster Alle documenten
* Escape sloot het dialoogvenster "Document Info" en "All Documents" niet.
* De titelbalk wordt niet bijgewerkt na het sluiten van een document in het dialoogvenster "all documents".
* Readme.html wordt niet langer aan uw lijst met alle documenten toegevoegd wanneer deze wordt geopend via Shift+F1.
* Het verwijderen van documenten uit het dialoogvenster "recents" sluit nu ook hun actieve tabblad.
* Uw zoekfilter blijft nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigde in sommige situaties onjuiste regelstekst aan.
* "Go to Line", "Go to Page" en "Go to Percent" plaatsten uw cursor op de verkeerde positie in grote documenten.
* "Find" en "Find Next" respecteerden het geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Geluiden van bladwijzers/notities zouden nu alleen goed moeten afspelen wanneer u over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Het toepassen van tekstterugloop bracht u naar het begin van uw document.

##### Webweergave
* Het webview-dialoogvenster kon niet worden vergroot en verscheen met een zeer kleine initiële grootte.
* Afbeeldingen zouden nu correct moeten worden weergegeven in de ingebedde webweergave.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten parseren met niet-Latijnse tekens.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en voegen garbage in de boektekst in.
* Koppelingen in verouderde Mobi-boeken.
* Aanzienlijk verbeterd AZW3-parsering.

##### Word-documenten
* Word-documenten met localiteit-specifieke stijlnamen gaven hun koppen niet correct weer.

##### HTML/XHTML-documenten
* Elementen `dl`, `dt` en `dd` produceerden geen regelafbrekingen in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op verwijdering van ongecodeerde tekst voor onjuist gelabelde PDF's.
* PDF-documenten met controletekens in hun titels en/of bladwijzers crashen Paperback niet langer bij het openen.

### Versie 0.8.5
* Pagina-ondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden verouderde Word, moderne Word en moderne Powerpoint ondersteund, met verouderde Powerpoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor verouderde Microsoft Word-documenten!
* Ondersteuning toegevoegd voor verouderde Powerpoint-presentaties!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* Sneltoets `ctrl+q` toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor gezipt boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingebedde afbeeldingen zou nu correct moeten worden weergegeven.
* CHM-documenten ondersteunen nu correct navigatie met interne koppelingen.
* Opgelost dat "go to page" 1 af was.
* Opgelost dat de escape-toets niet werkte om het dialoogvenster "open as" te sluiten.
* Opgelost dat het contextmenu van de lezer niet verscheen bij rechtsklikken of de toets "Applications".
* Opgelost dat soms het verkeerde document werd gefocust bij het openen van documenten vanaf de opdrachtregel.
* PDF-bestanden met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen u van hun bestaan.
* Het is nu mogelijk door afbeeldingen en figuren te navigeren met `g`/`shift+g` en `f`/`shift+f`.
* Paperback respecteert nu uw instelling voor de donkere modus van de toepassing.
* DAISY XML-ondersteuning verwijderd, omdat deze niet langer nodig is.
* Teruggezet naar de native Win32-navigatie met eerste letter in de inhoudsopgaverboom.
* Het dialoogvenster "error loading" toont nu gedetailleerdere foutmeldingen.
* De webweergave wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Pagina-ondersteuning toegevoegd aan RTF-documenten!
* Opgelost dat het openen van de webweergave in epub's met externe koppelingen deze automatisch zou activeren.
* Opgelost dat de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Opgelost dat alinea's in sommige PDF-documenten in meerdere korte regels werden gesplitst.
* PDF-documenten hebben nu basisondersteuning voor koppelings- en kopalinea-navigatie!
* RTF-tabs en regelfeeds worden nu exact weergegeven zoals ze in het document verschijnen.
* Teruggezet naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering opnieuw veel betrouwbaarder is.

### Versie 0.8.1
* `Ctrl+Shift+T` toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster "All Documents" ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Opgelost enkele bugs in de RTF-parser.
* Opgelost dat bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) beschadigd raken bij het openen van een bestand via een tweede Paperback-instantie.
* Opgelost dat PDF-tekst in de verkeerde volgorde werd gelezen en onjuiste spatiëring rond gekapitaliseerde woorden.
* Opgelost dat het laden van documenten langzaam was bij het openen van grote bestanden.
* Opgelost dat de lokalisatie van de "Ja"/"Nee"-knoppen in bevestigingsdialogen niet werkte.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updatefunctie toegevoegd die nu uw huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of een notitie, dank aan Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor Flat Open Document Text-bestanden toegevoegd!
* Ondersteuning voor Flat Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingstekens met `s` en `shift+s` toegevoegd.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan uw navigatiegeschiedenis.
* Het herstellen van Paperback's venster vanuit het systeemvak is opgelost.
* Markdown-documenten die ruwe tekst in plaats van gerenderde HTML in de webweergave tonen, zijn opgelost.
* Tabellen die niet correct in Markdown-bestanden worden weergegeven, zijn opgelost.
* PDF's met alleen afbeeldingen waarschuwen u nu voor hun bestaan wanneer u deze probeert te laden.
* Versie-informatie correct ingebed in het Paperback-uitvoerbare bestand.
* De optiedialogvenster is in tabbladen gesplitst voor gebruiksgemak en navigatie.
* Overgestapt op Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De gehele app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML en XHTML-gebaseerde documenten toegevoegd! Navigeer tussen tabellen met `T` en `Shift+T`, en druk op `Enter` om er een in een webweergave te bekijken.
* Een basiswebweergavefunctie toegevoegd! Druk op `Ctrl+Shift+V` om de huidige sectie van uw document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank aan Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het dialoogvenster Alle documenten.
* De updatecontrole toont nu opmerkingen bij de release wanneer een nieuwe versie beschikbaar is.
* Het herstellen van het venster vanuit het systeemvak is opgelost.
* Vertalingen van Ja/Nee-knoppen in bevestigingsdialogen zijn opgelost.
* Het laden van configuraties bij uitvoering als beheerder is opgelost.
* Commentaarbehandeling in XML en HTML-documenten is opgelost.
* TOC-parsing in Epub 2-boeken is opgelost.
* Navigeren naar het volgende item met dezelfde letter in de inhoudsopgave is opgelost.
* Het dialoogvenster Zoeken dat niet goed verborgen wordt bij gebruik van de volgende/vorige knoppen, is opgelost.
* Epub TOC die u af en toe naar het verkeerde item stuurt, is opgelost.
* Verschillende problemen met witruimtebehandeling in XML, HTML en vooraf geformateerde tags zijn opgelost.
* Off-by-one-fout in linknavigatie is opgelost.
* Sommige boeken met navolgende witruimte op hun regels zijn opgelost.
* Verschillende parserproblemen zijn opgelost.
* Aan bladwijzer gerelateerde menu-items en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Lijstbehandeling in verschillende documentindelingen is verbeterd.
* De vertalingsworkflow voor medewerkers is verbeterd.
* Veel interne refactors, waarbij het grootste deel van de bedrijfslogica van de toepassing van C++ naar Rust is verplaatst voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een zeer basisvoorziening voor het gaan naar de vorige/volgende positie toegevoegd. Als u `Enter` op een interne koppeling drukt en dit uw cursor verplaatst, wordt die positie nu onthouden en kan ernaar worden genavigeerd met `alt+left`/`right`-pijlen.
* Een elementenlijst toegevoegd! Deze toont momenteel alleen een boom van alle koppen in uw document of een lijst met koppelingen, maar er zijn plannen om deze in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Koppelingen in sommige Epub-documenten werken niet goed, opgelost.
* Parsing van Epub TOCs met relatieve paden is opgelost.
* Sommige Epub-documenten zonder titel of auteur zijn opgelost.
* De titels van sommige Epub-hoofdstukken die niet goed in het TOC-dialoogvenster worden weergegeven, zijn opgelost.
* U kunt nu de spatiebalk gebruiken om de OK/Annuleren-knoppen in het TOC-dialoogvenster te activeren, opgelost.
* De behandeling van koppen in Word-documenten is verbeterd.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u het dialoogvenster probeert op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het ga-menu in een veel compactere vorm weer te geven is toegevoegd aan de optiesdialoog, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen te laten omwikkelen.
* Een optie toegevoegd aan het menu Extra om de map met het huidige gefocuste document te openen.
* Een tamelijk eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basisslaapmerkerfunctie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2-e-books!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu een volledige regel als bladwijzer gebruiken, of alleen bepaalde tekst markeren. Als u geen selectie actief hebt wanneer u een bladwijzer plaatst, is het gedrag zoals vóór 0.6, en markeert het de volledige regel. Als u echter wat tekst selecteert, wordt alleen die tekst opgenomen in de bladwijzer.
* Bladwijzers kunnen nu optionele tekstnotities krijgen! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het bladwijzerdialoogvenster met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het bladwijzerdialoog zullen niet langer het vervelende voorvoegsel "bladwijzer x" hebben.
* Epub-boeken met HTML-inhoud die zich voordoen als XML worden nu correct afgehandeld.
* Het laden van grote Markdown-documenten is opgelost.
* Het indrukken van spatie in de inhoudsopgavestructuurweergave activeert niet langer de OK-knop.
* Witruimteafhandeling aan het begin van pre-tags in HTML- en XHTML-documenten is opgelost.
* Het tekstbesturingselement krijgt soms geen focus terug wanneer u terugkeert naar het venster van Paperback. Dit is opgelost.
* Het tekstveld in het dialoogvenster "Ga naar procent" werkt nu bij met de schuifveldwaarde.
* De weergave van aangepaste HTML-id's in Markdown-documenten is opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijgt u geen fout meer als het laden van uw document langer dan 5 seconden duurt.
* Als u Paperback als beheerder uitvoert, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks uit het bladwijzerdialoog te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand krijgt de naam van het bestand met een `.paperback`-extensie. Als zo'n bestand in dezelfde map wordt gevonden als een bestand terwijl dit wordt geladen, wordt het automatisch geladen. U kunt ze anders handmatig importeren met behulp van een item in het menu Extra.
* Koppelingen in documenten worden nu volledig ondersteund! Gebruik k en shift+k om vooruit en achteruit door hen te bewegen, en druk op Enter om er een te openen/activeren.
* Veel interne herstructureringen, waardoor de app sneller is en het binaire bestand kleiner.
* Markdown-inhoud wordt nu voorverwerkt om CommonMark-compatibel te zijn voordat deze wordt weergegeven.
* Navigatie per lijst en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om via de lijsten zelf te gaan, en I en Shift+I om door listitems te gaan.
* Numpad Delete werkt nu ook om documenten uit de tabbalk te verwijderen, naast normale Delete.
* Paperback kan nu optioneel minimaliseren naar uw systeemvak! Deze optie is standaard uitgeschakeld, maar als u deze inschakelt, wordt de minimaliseringsoptie in het systeemmenu Paperback in uw lade geplaatst, zodat het kan worden hersteld door op het gemaakte pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten geven nu een basisinhoudsopgave weer met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het documentinfovenster.
* Het installatieprogramma bevat nu een optie om de readme in uw browser weer te geven na installatie.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van alleen de laatste 10 documenten die u hebt geopend weer te geven, worden nu een aanpasbaar aantal weergegeven, met de rest van de documenten die u ooit hebt geopend toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen aan de parsers in het algemeen, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het repareren van de regelafhandeling in alinea's in Word-documenten en het toevoegen van opsommingstekens aan listitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden nu correct uitgeschakeld wanneer geen documenten geopend zijn.
* De oriëntatie van de schuifbalk voor percentage naar voren gaan is opgelost.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragmentidentificatoren is opgelost.
* Witruimte wordt niet langer op vreemde manieren uit XHTML-koppen verwijderd.
* Witruimte in geneste pre-tags in HTML-documenten wordt nu correct verwerkt.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgave-functie! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave op basis van de structuur van de koppen in uw document en toont deze in de `ctrl+t` dialoog.
* HTML-documenten hebben nu de titel zoals ingesteld in de title-tag, indien aanwezig. Verder gebruiken ze de bestandsnaam zonder uitbreiding.
* Van UniversalSpeech overgeschakeld naar het gebruik van een live region voor spraakrapportage. Dit betekent dat er geen schermlezer-DLL's meer samen met het programma worden meegeleverd, en dat meer schermlegers nu worden ondersteund, zoals Microsoft Narrator.
* Gewijzigd in zip-bibliotheken om een groter aantal epub-boeken te kunnen openen.
* De dialoog waarin u wordt gevraagd of u uw document als platte tekst wilt openen, is volledig opnieuw ontworpen en stelt u nu in staat uw document als platte tekst, HTML of Markdown te openen.
* De dialoog voor percentage naar voren gaan bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar toe te gaan.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken wordt opnieuw exact behouden.
* De Unicode-spatie zonder afbreking wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* U wordt niet langer telkens gevraagd hoe u een onherkenbaar bestand wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optionaal pictogram in het startmenu is aan het installatieprogramma toegevoegd.
* De inhoudsopgave zou nu schoner moeten zijn in een paar gevallen. Als u bijvoorbeeld een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, ziet u nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten is opgelost.
* De inhoudsopgave in Epub 3-boeken met absolute paden is opgelost.
* CHM-documenten geven nu hun titel weer zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Bladwijzers ondersteuning toegevoegd! U kunt zoveel bladwijzers hebben in zoveel documenten als u wilt. U kunt er via `b` en `shift+b` heen en weer springen, er een instellen met `control+shift+b`, en een dialoog openen om naar een specifieke bladwijzer te gaan met `control+b`.
* Een installatieprogramma is naast het draagbare zipbestand toegevoegd! Het installatieprogramma installeert Paperback in uw map Program Files en stelt automatisch bestandskoppelingen voor u in.
* Tekstbestanden met BOM's worden nu correct gedecodeerd, en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie is aan de statusbalk toegevoegd. Het toont nu uw huidige regel, teken en lees-percentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer in tekstuitvoer weergegeven.
* Als u een relatief pad aan Paperback doorgeeft op de opdrachtregel, wordt dit nu correct opgelost.
* Percentagebeweging wordt nu verwerkt door zijn eigen dialoog op basis van een schuifbalk, toegankelijk via `control+shift+g`.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor het opslaan van de positie is veel slimmer en zou alleen naar schijf moeten schrijven wanneer dit absoluut noodzakelijk is.
* Het document waarop u zich bevond toen u Paperback sloot, wordt nu onthouden bij herstart van de toepassing.
* Invoer in de dialogen "ga naar regel" en "ga naar pagina" wordt nu veel strenger gesaniteerd.
* Navigatie in de inhoudsopgave in epub 3-boeken met relatieve paden in hun manifesten is opgelost.

### Versie 0.3.0
* De inhoudsopgave in epub-boeken met URL-gecodeerde manifesten is opgelost.
* Kopnavigatie in HTML-documenten met multi-byte Unicode-tekens is opgelost.
* Hoog CPU-gebruik in documenten met lange titels als gevolg van een regressie in wxWidgets is opgelost.
* Het laden van UTF-8 tekstbestanden is opgelost.
* Geneste TOC-items in Epub-boeken die uw cursor op de verkeerde positie zetten, zijn opgelost.
* Een crash bij het afsluiten van de toepassing in bepaalde gevallen is opgelost.
* Een selectievakje in de opties-dialoog is toegevoegd om het omhullen van woorden in te schakelen of uit te schakelen!
* Het is nu mogelijk om Paperback's ontwikkeling te steunen, hetzij via het nieuwe donatie-item in het Help-menu hetzij via de link "sponsor this project" onderaan de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Overgeschakeld naar een PDF-bibliotheek die ook in Chromium wordt gebruikt, wat leidt tot veel betrouwbaardere PDF-verwerking in het algemeen.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Het uitvoeren van `paperback.exe` met een bestandsnaam terwijl deze al actief is, opent dat document in het reeds actieve exemplaar.
* U kunt nu op de knop Delete drukken op een document in het tabbladbesturingselement om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's is aan het paginalabel in de dialoog "ga naar pagina" toegevoegd.
* Het is nu mogelijk om vanuit de documentinhoud naar uw lijst met geopende documenten te wisselen via Tab.
* De koppelingstoetsen openen soms recente documenten als u er veel hebt; dit is opgelost.
* Paperback verwijdert nu onnodige zachte koppeltekens uit tekstuitvoer.
* Kopnavigatie zet u soms op het verkeerde teken; dit is opgelost.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsaanslagen toegevoegd voor navigatie op basis van koppen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze toetsaanslagen zijn ontworpen om op dezelfde manier te werken als een schermlezer.
* Probleem opgelost met het laden van epubs met URL-gecodeerde bestandsnamen in hun manifests.
* Probleem opgelost met het laden van epub 3-boeken met XHTML erin ingebed.
* Er wordt nu een bericht uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items zijn uitgeschakeld.
* Menu voor recente documenten toegevoegd! Dit slaat momenteel uw laatste 10 geopende documenten op, en als u op één daarvan drukt, wordt het geopend voor lezen.
* De zoekdialog volledig herschreven, waardoor deze veel eenvoudiger te gebruiken is, terwijl ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies is toegevoegd!
* Eerder geopende documenten worden nu onthouden bij het opnieuw starten van de toepassing. Dit kan worden geconfigureerd via het nieuwe opties-item in het menu Extra.
* `Shift+F1` toegevoegd om het lesmij-bestand rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Initiële versie.
