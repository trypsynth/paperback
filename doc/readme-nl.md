<!-- machine-translated from doc/readme.md (source-hash: 6564745fd3218b1a; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ca4819ea,a9eba369,e9860ee8,3b8321f8); please review and edit as needed -->

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

De iOS en Android apps gebruiken dezelfde leesmachine als de desktop, dus ze openen dezelfde formaten en onthouden je plek op dezelfde manier. Ze zijn gebouwd om gebruikt te worden met VoiceOver op iOS en TalkBack op Android.

### Documenten openen

* Gebruik de Open Book knop, of open een document vanuit de Files app of een ander app en kies Paperback.
* Op Android kun je in plaats daarvan de in-app bestandsbrowser aanzetten in Settings. Dit vereist de All Files Access toestemming en opent grote bestanden meteen in plaats van ze eerst te kopiëren.
* Houd de Open Book knop ingedrukt om de gegevens van een document (`.paperback`) te importeren of exporteren; dezelfde bestanden die de desktop app gebruikt.

### Lezen en luisteren

Elke app heeft twee manieren om een document te lezen. In tekstmodus lees je de tekst met je schermlezer. In voorleesmodus leest Paperback de tekst voor je voor met de stem die je in Settings hebt gekozen, en gaat door op de achtergrond en vanaf het vergrendelingsscherm. Wissel tussen beide vanuit het More Options menu.

Audioboeken, zoals DAISY, M4B en MP3 boeken, spelen hun eigen opname af.

### De leesbalk

De balk onderaan het scherm heeft, van links naar rechts:

* De navigatie-eenheid, zoals alinea, kop, pagina of link. Veeg omhoog of omlaag erop om het te veranderen.
* Vorige, afspeel en volgende knoppen. Vorige en volgende bewegen per navigatie-eenheid.
* De spreeksnelheid. Veeg omhoog of omlaag erop om te veranderen hoe snel Paperback voorleest.

Je kunt ook omhoog of omlaag vegen op de afspeelknop om per navigatie-eenheid te bewegen, zonder naar de vorige en volgende knoppen te reiken. Als je alleen dat gebruikt, neemt de instelling Hide previous and next buttons ze uit je schermlezer. De Swipe up moves forward instelling bepaalt welke kant een veegbeweging op gaat.

### More options

Het More Options menu is waar al het andere is. Sommige items werken iets anders op elke app.

* **Switch to TTS Mode of Switch to Text Mode:** schakelt tussen voorleesmodus en tekstmodus, hierboven beschreven. In tekstmodus start en pauzeert een Read Aloud item het voorlezen zonder tekstmodus te verlaten.
* **Table of Contents:** de hoofdstukken van het boek, geopend op het hoofdstuk dat je leest. Kies er een om er direct naar toe te gaan. Vermeldingen met hoofdstukken eronder kunnen worden uitgevouwen en ingevouwen met de acties van je schermlezer.
* **Elements:** een lijst met de koppen of links van het document. Wissel met de Type picker op iOS of de tabs op Android, selecteer er een om er naar toe te gaan.
* **Find:** typ wat je wilt zoeken, of kies een eerdere zoekopdracht uit Search History, en kies of je hoofdletters wilt matchen, alleen hele woorden of een reguliere expressie. Find Previous en Find Next springen naar een overeenkomst en zeggen waar deze is geland, en Find blijft open zodat je door kunt gaan. In voorleesmodus verschijnt Find ook als navigatie-eenheid op de leesbalk, zodat je van daaruit door de overeenkomsten kunt stappen.
* **Go To:** spring naar een regel, pagina of percentage door het document. Kies welke met de Mode picker.
* **Recent Documents:** elk document dat je hebt geopend, elk gemarkeerd als momenteel open, gesloten of bestand ontbreekt. Elk document heeft twee schermlezer acties: Remove haalt het uit de lijst, en Locate laat je een document vinden waarvan het bestand is verplaatst. Clear Recent Documents maakt de lijst leeg zonder documenten te verwijderen.
* **Word Count:** het aantal woorden in het document.
* **Document Info:** de titel, de auteur, de bestandsnaam, en op iOS ook het aantal regels en tekens.
* **Export:** slaat het document op als platte tekst, HTML of Markdown.
* **Sleep Timer:** stopt het voorlezen na 5, 10, 15, 30, 45 of 60 minuten, of een door jezelf gekozen tijd. Open het opnieuw terwijl het loopt om te zien hoeveel tijd er nog over is, of om het te annuleren.
* **Help:** opent deze readme.
* **Settings:**
    * **Text to speech:** de stem, spreeksnelheid en toonhoogte, een Play Sample knop om ze te horen, en de pauze tussen alinea's. Android laat je ook de spreekmotor kiezen. Op iOS is dit ook waar het spreekawoordenboek is: regels die veranderen hoe woorden worden uitgesproken, voor elke stem of alleen sommige.
    * **Readability:** tekstgrootte, regelafstand, alineaafstand, uitlijning en tekst met hoog contrast. iOS heeft ook licht en donker uiterlijk.
    * **Behavior:** of je documenten opnieuw wilt openen wanneer de app start, welke kant een veeg op de afspeelknop op gaat, en of je de vorige en volgende knoppen wilt verbergen. Android heeft ook de in-app bestandsbrowser hier.

### Toetsenborden en headsets

Met een toetsenbord werken de desktop sneltoetsen voor het openen van boeken, recente documenten, Find, Go To, de inhoudsopgave, woordaantal, documentinfo, export en de slaaptimer allemaal, met `Cmd` in plaats van `Ctrl` op iOS. Ook de enkelvoudige toetsen voor het verplaatsen per kop, pagina, link en de rest, en `Space` om af te spelen en te pauzeren. Op iOS bereiken de enkelvoudige toetsen Paperback alleen als VoiceOver's enkelvoudige Quick Nav uit staat.

Op Android speelt een headset knop af en pauzeert met één druk, gaat vooruit met twee en gaat terug met drie.

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

### Versie 1.0

1.0 is de eerste release op alle vijf platforms: Windows, macOS, Linux, iOS en Android, met de iOS en Android apps in de App Store en Google Play.

#### Added

##### Algemeen
* Linux-ondersteuning, als AppImage of tar.gz, met desktopintegratie zodat documenten openen vanuit uw bestandsbeheer.
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles van daar naar waar je bent met `Alt+F10`, en ga terug naar de markering met `Alt+Shift+F9`, voor het kopiëren van een lange tekstspan zonder shift-pijltjes erdoorheen. Alle drie zitten onder Tools > Select and copy.
* De sneltoets `=` kondigt nu zowel de pagina als het percentage aan, bijvoorbeeld "15%, pagina 30", en blijft zoals het was voor documenten zonder paginanummers.
* Het About-vak toont nu de licentie van Paperback en elke vertaler.
* Een Oekraïense vertaling.

##### Nieuwe formaten
* Comic book-archieven (`.cbz`).
* M4B-audioboeken, gesplitst in hun hoofdstukken.
* Manpages, zowel `man` als BSD `mdoc`, gecomprimeerd of niet.
* MP3-audioboeken, gesplitst in hoofdstukken wanneer het bestand deze heeft.
* Windows Write-bestanden (`.wri`).
* WinHelp-bestanden (`.hlp`).
* Word 6- en Word 95-documenten.

##### OCR
* Gescande PDF-pagina's kunnen nu worden herkend met de OCR ingebouwd in Windows en macOS. Druk `Enter` op een gescande pagina om deze te herkennen, of gebruik Batch OCR (`Ctrl+Shift+O`) voor een reeks pagina's.

##### Navigatie
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met behulp van MathCAT. Gebruik `M` of `Shift+M` om formules te navigeren, druk vervolgens `Enter` of `Space` om de originele MathML in Formula View te openen.
* Een Find All-knop in de Find-dialoog, met elk regel met een overeenkomst zodat u direct naar degene kunt springen die u wilt.
* Views Tabel, Lijsten en Pagina's in de elementenlijst (`F7`).
* Go to Line, Go to Page en Go to Percent nemen nu `+n` en `-n` aan om relatief te bewegen van waar u bent.
* EPUB-, MOBI- en CHM-boeken zonder eigen koppelingen krijgen nu koppelingen navigatie van hun inhoudsopgave.
* KF8 (AZW3)-boeken ondersteunen nu sectienavigate.
* EPUB-pagina's die alleen een afbeelding zijn, tonen nu een regel ervoor, zodat u er op kunt landen in plaats van er direct voorbij te gaan.

##### Audioboeken
* Afspeelsnelheidsregelaars, van halve snelheid tot drie keer zo snel. Gebruik `Ctrl+Shift+.` en `Ctrl+Shift+,`, of het Tools-menu.
* Bladwijzers en aantekeningen in alleen-audio-boeken onthouden nu het exacte moment waarop u deze hebt ingesteld.
* Volgende en vorige positie (`Alt+Left` en `Alt+Right`) werken nu in audioboeken.
* Voortgang door een audioboek wordt nu gemeten aan de hand van de opname, dus Go to Percent en de statusbalk komen overeen met hoe ver u er echt doorheen bent.

##### Recente documenten
* Een Clear Recent Documents-item in het submenu Recent Documents.

##### PDF-documenten
* Een instelling om elke regel van een PDF gescheiden te houden, in plaats van ze samen te voegen tot alinea's.
* Afbeeldingen en figuren in PDF's worden nu aangekondigd.
* PDF's die leesvolgorde bevatten maar geen van hun afbeeldingen labelen, kondigen deze afbeeldingen nu aan, in plaats van ze helemaal uit het boek weg te laten.

##### Web View
* Elk document kan nu in de webweergave worden geopend, niet alleen EPUB, HTML en Markdown.

##### Leesbaarheid
* Koppelingen worden nu getekend in een grootte die overeenkomt met hun niveau, en afbeeldingen en tabellen zijn gescheiden van de tekst eromheen.

##### pb
* `pb --list-formats` geeft een overzicht van elk formaat dat pb kan lezen.
* pb zegt nu welk bestand het niet kon lezen en waarom.

#### Opgelost

##### Algemeen
* Opgelost: crashes wanneer Paperback wordt gesloten.
* Paperback verbergt het venster nu onmiddellijk wanneer je het sluit, in plaats van het op het scherm te laten staan terwijl het opslaat.
* Een document openen schakelt "Reopen Last Closed" niet meer in wanneer er niets te heropenen is.
* Paperback probeert niet meer steeds opnieuw documenten uit je recente lijst te openen die verdwenen zijn, en beperkt hoeveel recente documenten het opslaat.
* Het oude INI-instellingenbestand wordt nu verwijderd zodra het naar het nieuwe formaat is verplaatst.
* De titels van de dialoogvensters voor lettertype en kleur, en het menu "Export As" in het Vietnamees, zijn nu vertaald.
* Bij het updaten wordt het opnieuw gestarte venster nu naar voren gebracht, in plaats van het achter alle andere vensters in Alt+Tab te laten.
* Tekstomloop wordt nu onmiddellijk toegepast op grote documenten, in plaats van het hele document opnieuw in te laden.

##### Navigatie
* `Alt+Left` gaat nu terug naar waar je vandaan sprong, in plaats van naar een oudere positie.
* Bladwijzersounds worden nu alleen afgespeeld wanneer je over een bladwijzer beweegt, niet wanneer je op de regel ervan terechtkomt.
* Als je de inhoudsopgave, de elementenlijst en de Go-dialogen sluit, ga je nu rechtstreeks naar de regel waar je terechtkomt, in plaats van de schermlezer het venster opnieuw te laten uitspreken.
* "Go to Line", "Go to Page" en "Go to Percent" weigeren nu getallen buiten het document in plaats van stilletjes ergens anders heen te gaan.
* NVDA onderbreekt de aankondiging niet meer wanneer een document geen pagina's heeft.
* Als je OK indruk in de inhoudsopgave zonder je te verplaatsen, ga je nu naar de al geselecteerde vermelding.
* De inhoudsopgave, de elementenlijst en de bladwijzerslijst vertonen niet meer vertraging of bevriezen bij boeken met duizenden vermeldingen.
* Pijltje omhoog en omlaag onthouden nu hun kolom per document, in plaats van deze over te nemen wanneer je van tabblad wisselt.

##### Audioboeken
* Audiowerkstelling gebruikt nu `Control+Space` op macOS, omdat `Command+Space` toebehoort aan Spotlight.

##### PDF-documenten
* Opgelost: PDF's die uit Apple Pages zijn geëxporteerd, lezen als platte tekst, zonder de koppen en lijsten waarmee ze zijn geschreven.
* Opgelost: PDF-alinea's en koppen splitsen op elke regel, en woorden splitsen uit elkaar op spaties.
* Opgelost: genummerde PDF-koppen die samen één kop vormen.
* Opgelost: PDF's waarvan de structuurwering tot geen tekst leidt, openen leeg.
* Paginakop- en voetregels worden niet meer op elke pagina van niet-gecodeerde PDF's uitgesproken.
* PDF's die hun paginakop- en voetregels als gewone tekst labelen, herhalen de titel en het paginanummer niet meer tussen twee alinea's op elke pagina.
* PDF's tonen nu hun werkelijke titel, niet hun bestandsnaam.
* Regels ingesteld in een monospaced-lettertype, zoals code, worden niet meer samengevoegd tot alinea's.

##### MOBI/AZW3-boeken
* Grote MOBI-boeken raken niet meer zonder geheugen en worden niet meer afgekapt na 20 MB.
* MOBI- en AZW3-boeken openen nu veel sneller.
* Opgelost: MOBI-boeken verliezen hun hoofdstuklijst.
* Opgelost: verminkte tekst waar MOBI-boeken van het ene record naar het volgende gaan.

##### Webweergave
* De webweergave laadt niet meer het geheel van een enorm boek tegelijk.
* De webweergave toont documenten nu volledig wanneer de lezer ze volledig toont, in plaats van alleen een deel ervan.

##### Andere indelingen
* FictionBook (.fb2)-boeken geschreven in windows-1251, waarvan er de meeste zijn, openen nu in plaats van helemaal niet te kunnen worden gelezen.
* FictionBook-boeken die een naamruimte of HTML-entiteit gebruiken die ze nooit hebben gedeclareerd, openen nu, in plaats van als verbroken te worden geweigerd.
* Boeken in verouderde coderingen openen nu veel sneller.
* Opgelost: sommige Chinese tekstbestanden openen als verminkte tekst.
* Met wachtwoord beveiligde OpenDocument-bestanden vragen nu om hun wachtwoord, in plaats van als verbroken te worden gerapporteerd.
* Met wachtwoord beveiligde oudere PowerPoint-bestanden openen nu, en oudere PowerPoint-dia's verliezen hun tekst niet langer.
* Platte tekstbestanden opgeslagen met een `.rtf`-extensie openen nu als tekst, in plaats van met een fout te mislukken.
* RTF-controlewoorden verschijnen niet meer als tekst.

#### iOS en Android

De iOS- en Android-apps openen elke indeling die de desktop doet, en bevatten:

* Hardop lezen, met je keuze van stem, snelheid en toon, een snelheidsregelaar direct op de leesregel, en een optionele pauze tussen alinea's.
* Afspelen van DAISY-, M4B- en MP3-audioboeken, die op de achtergrond en vanaf het vergrendelingsscherm doorluisteren.
* Navigatie per koppen, pagina's, koppelingen, tabellen, lijsten en meer van de leesregel, plus de inhoudsopgave en Zoeken.
* Een slaaptimer, woordtelling en documentexport, plus een spraakwoordenboek op iOS.
* Opties voor tekengrootte, afstand en contrast.
* Sneltoetsen die overeenkomen met de desktop.

### Versie 0.9.2
* Audioboeken laten je schermlezer niet meer een reeks spaties uitspreken wanneer je focus op het tekstveld plaatst.
* Audioboeken benoemen het bestand nu wanneer je ze per sectie doorloopt.
* Audioboeken rapporteren nu hun werkelijke lengte, in plaats van te beweren dat elk bestand 24 uur duurt.
* Het sluiten van de webweergave met Escape geeft niet meer een debug-waarschuwing nadat je een koppeling erin hebt gevolgd.
* Kopiëren na "Alles selecteren" geeft nu het hele document, in plaats van alleen het deel ervan dat momenteel is geladen.
* Zoeken gaat nu rechtstreeks naar de regel die het heeft gevonden, in plaats van je de schermlezer het venster opnieuw te laten uitspreken terwijl focus naar het boek terugkeert.
* Opgelost: EPUB's die een dwaalse ZIP64-blok dragen weigeren te openen met "Invalid local file header".
* Opgelost: lange documenten stappen terug naar hun begin terwijl een schermlezer continu erdoorheen leest.
* Koppelingen in de webweergave brengen je nu naar de sectie waarnaar ze verwijzen, in plaats van te mislukken met "File not found".
* De automatische aankondiging "Document opnieuw geladen" onderbreekt je schermlezer niet meer halverwege een zin, maar wacht tot deze klaar is met wat het aan het zeggen was.
* Het tabblad Algemeen van het dialoogvenster Instellingen typt nu door zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor controleren op updates.
* Windows toont nu altijd "Paperback" in het menu "Openen met", in plaats van de volledige tagline van het programma.
* Woordtelling en Documentinfo tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal loopt.

### Versie 0.9.1
* Geluiden voor bladwijzers en notities worden nu afgespeeld op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van hun tijdlijn in stilte te openen en bij te houden.
* Opgelost: krullehaakjes, em-dashes en vergelijkbare tekens verdwijnen uit RTF-documenten en voegen de omringende woorden samen.
* Opgelost: RTF-afbeeldingen lekken hun onbewerkte gegevens als verminkte tekst in het document.
* Opgelost: het submenu Recente documenten behoudt oude items totdat iets anders het opnieuw opbouwt.
* Toetsenbordversnellers zijn terug in elke vertaling, dus de menu's van het Russisch hebben nu opnieuw toetsenbordtoegang.
* Grote CHM-documenten worden nu tot zeven keer sneller geopend.
* Geopende documenten worden nu geregistreerd bij Windows, zodat ze in de taakbalk springlijst en in de recente lijst van het menu Start verschijnen.
* Opties is hernoemd naar Instellingen, wat overeenkomt met de mobiele apps en, op macOS, de platformconventie.
* Paperback onthoudt nu de vensterpositie, grootte en gemaximaliseerde status tussen sessies.
* Meervoudige vormen worden nu vertaald, dus berichten die dingen tellen, lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van het ncc.html-bestand van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in het dialoogvenster Toetsenbordsnelkoppelingen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken in de taakbalk en in `Alt+Tab` van elkaar kunnen worden onderscheiden.
* Het updatedialoogvenster is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool, pb genaamd, om snel een van Paperbacks ondersteunde formaten naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie View Source om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown.
* Documenttekst is nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alstublieft eventuele vreemdheden die je hiermee vindt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een knop voor volledig scherm.

##### Dialoogvenster Alle documenten
* Een zoekknop om ontbrekende boeken te vinden die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat je kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De `Ctrl+Shift+A` snelkoppeling om alle documenten uit te schakelen.

##### Instellingen en leesbaarheid
* Een leesbaardheidstabblad, met de volgende opties:
    * Woordombreking (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze versie, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor woordombreking en een daaropvolgende sneltoets.
* Een schakeloptie om te bepalen hoe je tabellen wilt weergeven, en een geünificeerde manier om tabellen in documenten weer te geven.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de bladeringsmodus in schermlezers.
* De sneltoets equals om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en deze blijven bestaan. Gebruik slash om er één in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het woordenteldialoogvenster, evenals de mogelijkheid om je leessnelheid in te stellen om deze metriek werkelijk nuttig te maken.
* Als een selectie actief is wanneer je het woordenteldialoogvenster opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsnelkoppelingen
* De mogelijkheid om elke toetsenbordsnelkoppeling in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordsnelkoppeling om Paperback uit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren is uitgebreid om exporteren naar HTML en Markdown mogelijk te maken, naast platte tekst.

##### Update
* Een cancelknop in het dialoogvenster met de update in uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audio afspelen.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunend zowel DAISY audio (inclusief DAISY audio + tekst) als zip-bestanden met audiobestanden.
* Toetsenbordsnelkoppelingen en menu-items voor afspelen/pauzeren van commentaar, vooruit en achteruit zoeken, en het zoekaantal aanpassen.
* Opties om de leescursor te synchroniseren met audioweergave, het audiozoekaantal in te stellen en te kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als mojibake.
* "Laatst gesloten opnieuw openen" probeert niet langer de meegeleverde readme opnieuw te openen.
* Uw geselecteerde tabblad krijgt nu correct focus na het herstarten van Paperback.
* Paperback's afhandeling van bestanden op Windows-netwerkschijven: "bestand in map weergeven" geeft nu correct focus aan het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u om bevestiging gevraagd wanneer er een wordt gevonden.
* Map met inhoud openen geeft nu correct focus aan het gegeven bestand in de verkenner.
* Het openen van de readme respects nu uw geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op displays met hoge DPI.
* Het menu wordt nu correct bijgewerkt en focus verschuift naar het tekstbesturingselement wanneer u help in Paperback opent.
* Overgestapt op een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu gelezen bij het schakelen tussen tabbladen.
* Geheugengebruik op grote documenten verminderd door de grootte van de interne per-teken-indextabellen te halveren.

##### All Documents Dialog
* Escape sluit de dialoogvensters Document Info en All Documents niet.
* De titelbalk wordt niet bijgewerkt na het sluiten van een document in het dialoogvenster all documents.
* Readme.html wordt niet langer aan uw all documents-lijst toegevoegd wanneer geopend via `Shift+F1`.
* Het verwijderen van documenten uit het dialoogvenster recent gebruikt zal nu ook hun actieve tabblad sluiten.
* Uw zoekfilter wordt nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regeltekst aan.
* Go to Line, Go to Page en Go to Percent plaatsen uw cursor op de verkeerde plaats in grote documenten.
* Find en Find Next respecteren het geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Geluiden voor bladwijzer/notitie worden nu correct uitsluitend afgespeeld wanneer u over een woord dat een bevat navigeert.

##### Leesbaarheid
* Woordomloop toepassen brengt u naar het begin van uw document.

##### Web View
* Het webview-dialoogvenster kan niet worden gebruikt en verschijnt met een zeer kleine initiële grootte.
* Afbeeldingen worden nu correct weergegeven in de ingebedde webview.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten met niet-Latijnse tekens parseren.
* RTF `\pict`-groepen zodat ingesloten afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en plaatsen afval in de boektekst.
* Links in verouderde Mobi-boeken.
* Aanzienlijk verbeterd AZW3-parseren.

##### Word-documenten
* Word-documenten met landspecifieke stijlnamen geven hun koppelingen niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen veroorzaken geen regelafbrekingen in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op gewone tekstextractie voor onjuist getagde PDF's.
* PDF-documenten die controletekens in hun titels en/of bladwijzers bevatten, kunnen Paperback niet langer laten vastlopen bij het openen.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden verouderde Word, moderne Word en moderne Powerpoint ondersteund, waarbij verouderde Powerpoint voor de toekomst is gepland.
* Ondersteuning toegevoegd voor verouderde Microsoft Word-documenten!
* Ondersteuning toegevoegd voor verouderde Powerpoint-presentaties!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De `ctrl+q`-sneltoets toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor ingepakte boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingesloten afbeeldingen wordt nu correct weergegeven.
* CHM-documenten ondersteunen nu correct interne linknavigatie.
* Go to page vastgesteld dat het 1 uit is.
* Escape-toets werkt nu niet langer om het dialoogvenster openen als te sluiten.
* Het lezercontextmenu wordt niet langer weergegeven bij rechtsklikken of de toepassingstoets.
* Het verkeerde document wordt soms geactiveerd wanneer documenten vanaf de opdrachtregel worden geopend.
* Alleen afbeeldingen PDF's worden opnieuw gedetecteerd en waarschuwen u voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/`shift+g` en f/`shift+f`.
* Paperback respecteert nu uw instelling voor donkere modus voor toepassingen.
* DAISY XML-ondersteuning verwijderd, omdat deze niet langer nodig is.
* Overgestapt op de native Win32-navigatie voor de eerste letter in de inhoudsopgavestructuur.
* Het dialoogvenster met foutmelding toont nu meer gedetailleerde foutmeldingen.
* De webview wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Een fout opgelost waarbij het openen van de webview in epubs met externe koppelingen deze automatisch zou activeren.
* Een fout opgelost waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's worden in sommige PDF-documenten opgesplitst in meerdere korte regels.
* PDF-documenten hebben nu basisondersteuning voor link- en koppelingnavigatie!
* RTF-tabs en regelinvoegingen worden nu precies weergegeven zoals ze in het document voorkomen.
* Teruggegaan naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-weergave veel betrouwbaarder wordt.

### Versie 0.8.1
* `Ctrl+Shift+T` toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster All Documents ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar fouten met de RTF-parser opgelost.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) worden niet langer beschadigd wanneer een bestand via een tweede Paperback-exemplaar wordt geopend.
* PDF-tekst wordt in de verkeerde volgorde gelezen en onjuiste spatiëring rond gekapitaliseerde woorden.
* Traag laden van documenten opgelost bij het openen van grote bestanden.
* De lokalisatie van de knoppen Ja/Nee in bevestigingsdialoogvensters opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigd Chinees en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu uw huidige versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, dank Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor Flat Open Document Text-bestanden toegevoegd!
* Ondersteuning voor Flat Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingstekens met s en shift+s toegevoegd.
* Elke beweging van meer dan 300 tekens voegt nu automatisch toe aan uw navigatiegeschiedenis.
* Het herstellen van het Paperback-venster vanuit het systeemvak opgelost.
* Markdown-documenten die ruwe tekst weergaven in plaats van weergegeven HTML in de webweergave opgelost.
* Tabellen die niet correct in Markdown-bestanden werden weergegeven opgelost.
* PDF-bestanden met alleen afbeeldingen geven nu een waarschuwing wanneer u probeert er een te laden.
* Versie-informatie correct ingebed in het Paperback-uitvoerbare bestand.
* Het optiesdialoogvenster in tabbladen verdeeld voor gemakkelijk gebruik en navigatie.
* Overgegaan op Hayro voor het parseren van PDF's, wat meer betrouwbaarheid, snelheid en minder DLL's oplevert.
* De hele app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbedieningselement bevat nu lezerspecifieke acties in plaats van algemene items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML- en XHTML-gebaseerde documenten toegevoegd! Navigeer tussen tabellen met T en Shift+T en druk op Enter om er een in een webweergave te bekijken.
* Een basiswebweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om het huidige gedeelte van uw document in een webgebaseerde renderer te openen, nuttig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank Ruslan Gulmagomedov!
* Een knop Alles wissen aan het dialoogvenster Alle documenten toegevoegd.
* De updatecontrole geeft nu opmerkingen over de release weer wanneer een nieuwe versie beschikbaar is.
* Het herstellen van het venster vanuit het systeemvak opgelost.
* Ja/Nee-knopvertalingen in bevestigingsdialoogvensters opgelost.
* Het laden van configuraties als administrator opgelost.
* Commentaarverwerking in XML- en HTML-documenten opgelost.
* TOC-parsering in Epub 2-boeken opgelost.
* Navigatie naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* Het dialoogvenster Zoeken dat niet goed verborgen was met de knoppen volgende/vorige opgelost.
* Epub TOC die u af en toe naar het verkeerde item bracht opgelost.
* Verschillende problemen met witruimteverwerking in XML-, HTML- en pre-tags opgelost.
* Off-by-one-fout in linknavigatie opgelost.
* Sommige boeken met trailing witruimte op hun regels opgelost.
* Diverse parsingproblemen opgelost.
* Bladwijzer-gerelateerde menu-items en de elementenlijst zijn nu op de juiste manier uitgeschakeld als geen document geopend is.
* Lijstverwerking in verschillende documentindelingen verbeterd.
* De vertaalworkflow voor bijdragers verbeterd.
* Veel interne refactorings, waarbij het grootste deel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Wachtwoord-beveiligde PDF-ondersteuning toegevoegd!
* Een zeer basisnavigatie naar vorige/volgende positie-functie toegevoegd. Als u op Enter druk op een interne koppeling en deze verplaatst uw cursor, wordt die positie nu onthouden en kan ermee worden genavigeerd met alt+left/right pijltjestoetsen.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle koppelingen in uw document of een lijst met koppelingen, maar er zijn plannen om dit in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Koppelingen in sommige Epub-documenten die niet goed werkten opgelost.
* Epub TOC's parseren die relatieve paden bevatten opgelost.
* Sommige epub-documenten die geen titel of auteur weergaven opgelost.
* De titels van sommige epub-hoofdstukken die niet goed in het TOC-dialoogvenster werden weergegeven opgelost.
* U kon de spatiebalk niet gebruiken om de OK/annuleren-knoppen in het TOC-dialoogvenster te activeren opgelost.
* Verwerking van koppelingen in Word-documenten verbeterd.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u probeert het dialoogvenster op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het menu "gaan naar" in een veel compactere vorm weer te geven is toegevoegd aan het dialoogvenster Opties, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen te laten omwikkelen.
* Een optie aan het menu Gereedschappen toegevoegd om de map met het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basale slaaptimerfunctie toegevoegd, toegankelijk met `Ctrl+Shift+S`.
* Ondersteuning toegevoegd voor het parseren van FB2 e-boeken!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu worden gemaakt om een hele regel aan te geven, of alleen bepaalde geselecteerde tekst. Als u geen selectie actief hebt wanneer u een bladwijzer plaatst, is het gedrag hetzelfde als vóór 0.6, en wordt de hele regel gemarkeerd. Als u echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bij zich hebben! Navigeer tussen bladwijzers met notities met N en `Shift+N`, of open het dialoogvenster Bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een irritant voorvoegsel "bladwijzer x".
* EPUB-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Laden van grote Markdown-documenten opgelost.
* Spatiebalk indrukken in de boomweergave inhoudsopgave die de knop OK activeert, opgelost.
* Behandeling van witruimte aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Het textbesturingselement dat soms focus niet terugkrijgt wanneer het venster van Paperback wordt geopend, opgelost.
* Het tekstveld in het dialoogvenster "Ga naar procent" werkt niet bij het bijwerken van de schuifregelaarwaarde, opgelost.
* De weergave van aangepaste HTML-ID's in Markdown-documenten opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar actief is, krijgt u niet langer een fout als het laden van uw document langer dan 5 seconden duurt.
* Als u Paperback als beheerder uitvoert, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks uit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand krijgt dezelfde naam als het bestand met een extensie .paperback. Als zo'n bestand in dezelfde map als een bestand wordt gevonden tijdens het laden ervan, wordt het automatisch geladen. Anders kunt u ze handmatig importeren met behulp van een item in het menu Gereedschappen.
* Koppelingen in documenten worden nu volledig ondersteund! Gebruik k en `shift+k` om vooruit en achteruit door koppelingen te gaan, en druk op `Enter` om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner is geworden.
* Markdown-inhoud wordt nu voorverwerkt om compliant te zijn met CommonMark voordat deze wordt weergegeven.
* Navigatie op lijsten en hun items wordt nu volledig ondersteund! Gebruik L en `Shift+L` om door lijsten zelf te gaan, en I en `Shift+I` om door listitems te gaan.
* Numpad Delete werkt nu ook om documenten uit de tabbalk te verwijderen, naast normale Delete.
* Paperback kan nu optioneel worden geminimaliseerd naar uw systeemvak! Deze optie is standaard uit, maar als u deze inschakelt, wordt de optie minimaliseren in het systeemmenu Paperback in uw systeemvak geplaatst, zodat deze kan worden hersteld door op het gegenereerde pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinformatie.
* Het installatieprogramma bevat nu een optie om het leesmij-bestand in uw browser weer te geven na de installatie.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van alleen de laatste 10 documenten weer te geven die u hebt geopend, wordt nu een aanpasbaar aantal weergegeven, met de overige documenten die u ooit hebt geopend toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen in de parsers overal, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het corrigeren van de afhandelingen van nieuwe regels in alinea's in Word-documenten en het toevoegen van opsommingstekens aan listitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden nu correct uitgeschakeld wanneer er geen documenten geopend zijn.
* De oriëntatie van de schuif voor "ga naar percentage" is opgelost.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragment-ID's is opgelost.
* Witruimte wordt nu op de juiste manier uit XHTML-kopjes verwijderd.
* Witruimte-verwerking in geneste pre-tags in HTML-documenten is opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgave-functie! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave op basis van de structuur van de kopjes in uw document en toont deze in de `ctrl+t`-dialoog.
* HTML-documenten hebben nu de titel zoals ingesteld in de title-tag, indien aanwezig. Anders gebruiken ze de bestandsnaam zonder extensie.
* Overgestapt van UniversalSpeech naar het gebruik van een live region voor spraakuitvoer. Dit betekent dat geen schermlezer-DLL's meer bij het programma worden geleverd, en meer schermlezers worden nu ondersteund, zoals Microsoft Narrator.
* Zip-bibliotheken zijn overgeschakeld om meer EPUB-boeken te kunnen openen.
* De dialoog waarin u wordt gevraagd of u uw document als platte tekst wilt openen is volledig opnieuw gemaakt en biedt nu de mogelijkheid om uw document als platte tekst, HTML of Markdown te openen.
* De dialoog "ga naar percentage" bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken wordt nu opnieuw precies bewaard.
* De Unicode-non-breaking space wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* U wordt niet langer gevraagd hoe u een onbekend bestand wilt openen telkens wanneer u het laadt, maar alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenupictogram is aan het installatieprogramma toegevoegd.
* De inhoudsopgave zou in enkele gevallen schoner moeten zijn. Als u bijvoorbeeld een onderliggend en overliggend item hebt met dezelfde tekst op dezelfde positie, ziet u nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten is opgelost.
* De inhoudsopgave in Epub 3-boeken met absolute paden is opgelost.
* CHM-documenten moeten nu hun titel weergeven zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Bladwijzerfunctionaliteit toegevoegd! U kunt zoveel bladwijzers hebben in zoveel documenten als u wilt. U kunt met `b` en `shift+b` vooruit en achteruit springen, één instellen met `control+shift+b` en een dialoog weergeven om naar een specifieke bladwijzer te springen met `control+b`.
* Een installatieprogramma is toegevoegd naast het draagbare zipbestand! Het installatieprogramma installeert Paperback in uw map Program Files en stelt automatisch bestandskoppelingen in.
* Tekstbestanden met BOM's worden nu correct gedecodeerd en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie is aan de statusbalk toegevoegd. Deze toont nu uw huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer weergegeven in tekstuitvoer.
* Bij het doorgeven van een relatief pad aan Paperback via de opdrachtregel wordt dit nu correct opgelost.
* Percentagebewegingen worden nu door hun eigen schuifgebaseerde dialoog verwerkt, toegankelijk met `control+shift+g`.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaardwaarde.
* De logica voor positiebesparing is nu veel slimmer en schrijft alleen naar schijf wanneer dit absoluut noodzakelijk is.
* Het document waarop u zich concentreerde toen u Paperback sloot, wordt nu onthouden bij toepassingsherstarts.
* Invoer in de dialogen "ga naar regel" en "ga naar pagina" wordt nu strenger opgeschoond.
* Inhoudsopgave-navigatie in Epub 3-boeken met relatieve paden in hun manifesten is opgelost.

### Versie 0.3.0
* De inhoudsopgave in Epub-boeken met URL-gecodeerde manifesten is opgelost.
* Koppelingnavigatie in HTML-documenten met multi-byte Unicode-tekens is opgelost.
* Hoog CPU-gebruik in documenten met lange titels door een regressie in wxWidgets is opgelost.
* Het laden van UTF-8-tekstbestanden is opgelost.
* Geneste inhoudsopgave-items in Epub-boeken die uw cursor op de verkeerde positie plaatsen is opgelost.
* Een crash bij toepassingsafsluiting in bepaalde gevallen is opgelost.
* Een selectievakje in de dialoog Opties is toegevoegd om woordomloop in en uit te schakelen!
* Het is nu mogelijk om Paperback's ontwikkeling te ondersteunen, ofwel via het nieuwe doneer-item in het Help-menu ofwel via de link "sponsor this project" onderaan de GitHub-opslagplaatshoofdpagina.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* PDF-bibliotheken zijn overgeschakeld naar degene die in Chromium worden gebruikt, wat leidt tot veel betrouwbaardere PDF-parsing overall.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Als u `paperback.exe` met een bestandsnaam uitvoert terwijl dit al wordt uitgevoerd, wordt dat document geopend in het al lopende exemplaar.
* U kunt nu `delete` op een document in het tabtabblad indrukken om dit te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's is toegevoegd aan het paginaLabel in de dialoog "ga naar pagina".
* Tabbladen van document-inhoud naar uw lijst met geopende documenten is toegestaan.
* Koppelingtoetsen openen soms recente documenten als u er genoeg van hebt is opgelost.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit tekstuitvoer.
* Koppelingnavigatie brengt u soms op het verkeerde teken is opgelost.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsaanslagen toegevoegd voor navigatie op basis van koppelingen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze toetsaanslagen zijn ontworpen om te werken zoals een schermlezer.
* Laden van epub's met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Laden van epub 3-boeken met XHTML erin opgelost.
* Een bericht wordt nu uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Menu voor recente documenten toegevoegd! Het slaat momenteel uw laatste 10 geopende documenten op, en door op Enter te drukken op een ervan, wordt het geopend om te lezen.
* Het dialoogvenster Zoeken volledig herschreven, waardoor het veel eenvoudiger in gebruik is, en tegelijkertijd een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies is toegevoegd!
* Eerder geopende documenten worden nu onthouden na herstart van de toepassing. Dit kan worden geconfigureerd via het nieuwe item opties in het menu Extra.
* `Shift+F1` toegevoegd om het leesmij rechtstreeks in Paperback te openen.

### Versie 0.1.0
* Initiële release.
