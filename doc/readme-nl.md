<!-- machine-translated from doc/readme.md (source-hash: 4d3bd6acdc082011; sections: 84030068,db723a70,df2f4c18,14335443,d44bf4c8,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,fabb029c); please review and edit as needed -->

# Paperback - versie 1.0

## Inleiding

Paperback is een lichtgewicht, snelle en toegankelijke lezer voor e-books, documenten en audioboeken, voor iedereen, van vrijetijdslezers tot ervaren powerusers. Het is ontworpen met het oog op schermlezertoegankelijkheid, hoge snelheid en een gebruikerservaring zonder overbodige ballast.

## Systeemvereisten

Paperback draait op Windows 10/11, alle moderne versies van macOS op ARM, Linux, iOS 17 of nieuwer en Android 7 of nieuwer. De iOS- en Android-apps staan in de App Store en op Google Play.

## Functies

* Volledig zelfstandig; er hoeft geen software op je computer te worden geïnstalleerd om te kunnen lezen.
* Bijzonder snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee je zoveel documenten naast elkaar kunt openen als je wilt.
* Bewaart je exacte leespositie in elk document dat je opent.
* Onthoudt desgewenst welke documenten je open had toen je het programma afsloot en herstelt ze bij de volgende start.
* Bevat navigatiefunctionaliteit die lijkt op de bladermodus van veel schermlezers, om snel en eenvoudig door documenten te navigeren.
* Bevat een robuust zoekvenster, met functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig portable worden uitgevoerd of worden geïnstalleerd met automatisch ingestelde bestandstoewijzingen.
* Ondersteunt een enorme reeks gangbare bestandsindelingen.
* Speelt audioboeken af, met instelbare snelheid en bladwijzers die het exacte tijdstip onthouden.
* Leest gescande PDF-pagina's met de OCR die in Windows en macOS is ingebouwd.
* Bladwijzers en notities, zodat je kunt markeren waar je gebleven bent en er later naar terug kunt gaan.
* Elke toetsenbordsneltoets kan worden aangepast.
* Wordt geleverd met `pb`, een terminalprogramma dat elk ondersteund document omzet naar HTML, Markdown of platte tekst.

## Schermlezercompatibiliteit

Paperback werkt goed met alle grote schermezers. Er is echter een bekend probleem voor JAWS-gebruikers.

### JAWS en brailleleesregels

Als je JAWS gebruikt met een brailleleesregel, kan het voorkomen dat lange alinea's worden afgekapt wanneer je vooruit bladert met de navigatietoetsen van je display. Het commando voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een fout in de manier waarop JAWS de RICHEDIT50W-tekstbesturingselement verwerkt, niet iets in Paperback zelf. Het duurde lange tijd voordat een oplossing beschikbaar kwam, gegeven Vispero's enthousiasme voor het reageren op problemen met open source-software.

De tijdelijke oplossing, uiteindelijk gevonden via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". Je wilt ook "Pan Text by Paragraph" inschakelen, anders blijft je display op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen op hun plaats zou bladeren correct moeten werken.

### JAWS en berichten van Paperback

Paperback geeft meldingen zoals "No pages." of "This document has no audio." als toegankelijkheidsmeldingen, waardoor een schermlezer deze kan uitspreken boven wat het anders zegt. JAWS reageert daar alleen op wanneer "Enable accessible notification events" is ingeschakeld voor de applicatie, en op sommige machines is dit niet het geval.

Als JAWS niets zegt wanneer je een toets indrukt die iets zou moeten rapporteren, open je het instellingencentrum met Paperback op de voorgrond (`Insert+6`), zoek naar "notification" en vink "Enable accessible notification events" aan. Dit schrijft de instelling naar `paperback.jcf`, zodat deze alleen op Paperback van toepassing is.

## Ondersteunde bestandstypen

Paperback ondersteunt de volgende indelingen en extensies:

* Comic-archieven (`.cbz`)
* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-e-boeken (`.fb2`)
* HTML-bestanden (`.htm`, `.html`, `.xhtml`)
* Handmatige pagina's, zowel `man` als BSD `mdoc` (`.1` tot `.9`, `.man`, `.roff` en de gecomprimeerde vormen van elk)
* Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
* M4B-audioboeken (`.m4b`)
* MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
* MP3-audioboeken (`.mp3`)
* OpenDocument-presentaties (`.odp`, `.fodp`)
* OpenDocument-tekstbestanden (`.odt`, `.fodt`)
* PDF-documenten (`.pdf`)
* PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
* reStructuredText-documenten (`.rst`, `.rest`)
* RTF-documenten (`.rtf`)
* Windows Write-documenten (`.wri`)
* WinHelp-bestanden (`.hlp`)
* Platte tekst- en logbestanden (`.txt`, `.log`)

## Sneltoetsen

Paperback is ontworpen voor toetsenbordgericht gebruik. Hieronder staan de huidige sneltoetsen.

De sneltoetsen hieronder gelden voor Windows. Waar macOS afwijkt, staat het equivalent tussen haakjes — vooral omdat Ctrl+G, Ctrl+W en Alt+Links/Rechts op dat platform al bezet zijn door andere systeem- of app-conventies.

### Menu Bestand

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document heropenen.
* `Ctrl+R`: Het venster "Alle documenten" tonen (vanuit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in het app-menu).

### Menu Ga

* `Ctrl+F`: Het zoekvenster tonen.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Ga naar regel.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ga naar percentage.
* `Ctrl+P`: Ga naar pagina (indien ondersteund door het huidige document).
* `=`: Je huidige leespercentage en pagina melden, bijvoorbeeld "15%, pagina 30". Bij documenten zonder paginanummers wordt de pagina weggelaten.
* `Alt+Links` (macOS: `Cmd+[`): Terug in navigatiegeschiedenis.
* `Alt+Rechts` (macOS: `Cmd+]`): Vooruit in navigatiegeschiedenis.
* `[`: Vorige sectie.
* `]`: Volgende sectie.
* `Shift+H`: Vorige kop.
* `H`: Volgende kop.
* `Shift+1` t/m `Shift+6`: Vorige kop op niveau 1-6.
* `1` t/m `6`: Volgende kop op niveau 1-6.
* `Shift+P`: Vorige pagina.
* `P`: Volgende pagina.
* `Shift+B`: Vorige bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Je tijdelijke bladwijzer instellen.
* `\`: Naar je tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Alleen naar bladwijzers springen.
* `Ctrl+Alt+M`: Alleen naar notities springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, oftewel de fysieke Control-toets in plaats van Cmd): De notitietekst op de huidige positie tonen.
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
* `Shift+S`: Vorige scheiding.
* `S`: Volgende scheiding.
* `Shift+L`: Vorige lijst.
* `L`: Volgende lijst.
* `Shift+I`: Vorig lijstitem.
* `I`: Volgend lijstitem.
* `Shift+,`: Ga naar het begin van de huidige container (lijst of tabel).
* `,`: Ga voorbij het einde van de huidige container (lijst of tabel).

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, oftewel de fysieke Control-toets in plaats van Cmd): Woordenaantal van het huidige document tonen.
* `Ctrl+I`: Documentinformatie tonen.
* `Ctrl+T`: Inhoudsopgave tonen.
* `F7`: Elementenlijst tonen.
* `Ctrl+Shift+C`: Bovenliggende map openen.
* `Ctrl+Shift+V`: De huidige inhoud openen in webweergave.
* `Ctrl+U`: De documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document exporteren naar platte tekst.
* `Ctrl+Shift+B`: Bladwijzer in-/uitschakelen op de huidige selectie/cursor.
* `Ctrl+Shift+N`: Bladwijzernotitie toevoegen of bewerken op de huidige selectie/cursor.
* `Ctrl+Alt+W`: Automatische terugloop in-/uitschakelen.
* `Ctrl+Spatie` (macOS: `RawCtrl+Spatie`, oftewel de fysieke Control-toets, omdat Cmd+Spatie Spotlight opent): Audio afspelen/pauzeren.
* `'`: Audio vooruitspoelen.
* `;`: Audio terugspoelen.
* `Shift+'`: De audiospoelstap vergroten.
* `Shift+;`: De audiospoelstap verkleinen.
* `Ctrl+Shift+.`: Audio sneller afspelen.
* `Ctrl+Shift+,`: Audio langzamer afspelen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, oftewel Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Instellingen openen (macOS: in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.
* `Ctrl+Shift+O`: Een reeks gescande PDF-pagina's herkennen met OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Het begin van een selectie markeren, zodat alles van hier tot waar je ook uitkomt in één keer kan worden gekopieerd.
* `Alt+F10` (macOS: `Cmd+F10`): Alles van het gemarkeerde begin van de selectie tot de huidige positie kopiëren.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Terugspringen naar het gemarkeerde begin van de selectie, waarbij de markering blijft staan.

### Menu Help

* `Ctrl+F1`: Het venster Over tonen.
* `F1`: Help bekijken in je standaardbrowser.
* `Shift+F1`: Help bekijken in Paperback.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: De donatiepagina openen in je standaardbrowser.

### Aanvullende toetsen voor de documentweergave

* `Delete` / `Numpad Delete` op het tabbladelement: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Spatie` in de documenttekst: Een link volgen of een tabel- of formuleweergave openen op de cursor.
* `Enter` op een gescande PDF-pagina: De pagina herkennen met OCR.
* `Shift+F10` of de menu- of applicatietoets in de documenttekst: Het contextmenu openen.

## iOS en Android

De iOS- en Android-apps gebruiken dezelfde leesengine als de desktopversie, dus ze openen dezelfde indelingen en onthouden op dezelfde manier waar je gebleven bent. Ze zijn gemaakt om te worden gebruikt met VoiceOver op iOS en TalkBack op Android.

### Documenten openen

* Gebruik de knop Boek openen, of open een document vanuit de app Bestanden of een andere app en kies Paperback.
* Op Android kun je in plaats daarvan in Instellingen de bestandsbrowser in de app inschakelen. Die heeft de toestemming "Toegang tot alle bestanden" nodig en opent grote bestanden meteen, in plaats van ze eerst te kopiëren.
* Houd de knop Boek openen lang ingedrukt om de documentgegevens (`.paperback`) te importeren of exporteren, dezelfde bestanden die de desktopversie gebruikt.

### Lezen en luisteren

Elke app heeft twee manieren om een document te lezen. In de tekstmodus lees je de tekst met je schermlezer. In de voorleesmodus leest Paperback de tekst aan je voor met de stem die je in Instellingen kiest, en gaat daarmee door op de achtergrond en vanaf het vergrendelscherm. Wisselen doe je via het menu Meer opties.

Audioboeken, zoals DAISY-, M4B- en MP3-boeken, spelen in plaats daarvan hun eigen opname af.

### De leesbalk

De balk onderaan het scherm bevat, van links naar rechts:

* De navigatie-eenheid, zoals alinea, kop, pagina of link. Veeg erop omhoog of omlaag om die te wijzigen.
* De knoppen Vorige, Afspelen en Volgende. Vorige en Volgende navigeren per navigatie-eenheid.
* De spreeksnelheid. Veeg erop omhoog of omlaag om te wijzigen hoe snel Paperback voorleest.

Je kunt ook omhoog of omlaag vegen op de afspeelknop om per navigatie-eenheid te navigeren, zonder naar de knoppen Vorige en Volgende te hoeven gaan. Als je alleen dat gebruikt, haalt de instelling Knoppen voor vorige en volgende verbergen ze uit de weg van je schermlezer. De instelling Omhoog vegen gaat vooruit bepaalt welke kant een veeg op gaat.

### Meer opties

In het menu Meer opties vind je al het andere. Sommige items werken in elke app een beetje anders.

* **Overschakelen naar TTS-modus of Overschakelen naar tekstmodus:** wisselt tussen de voorleesmodus en de tekstmodus, zoals hierboven beschreven. In de tekstmodus start en pauzeert een item Voorlezen het voorlezen zonder de tekstmodus te verlaten.
* **Inhoudsopgave:** de hoofdstukken van het boek, geopend bij het hoofdstuk dat je aan het lezen bent. Kies er een om er direct naartoe te gaan. Items met hoofdstukken eronder kun je uit- en samenvouwen met de acties van je schermlezer.
* **Elementen:** een lijst met de koppen of links van het document. Wissel tussen de twee met de keuzelijst Type op iOS of de tabbladen op Android, en kies er dan een om ernaartoe te gaan.
* **Zoeken:** typ waarnaar je wilt zoeken, of kies een eerdere zoekopdracht uit Zoekgeschiedenis, en kies of je hoofdlettergevoelig wilt zoeken, alleen hele woorden wilt vinden of een reguliere expressie wilt gebruiken. Vorige zoeken en Volgende zoeken springen naar een overeenkomst en zeggen waar je terecht bent gekomen, en Zoeken blijft open zodat je verder kunt. In de voorleesmodus verschijnt Zoeken ook als navigatie-eenheid op de leesbalk, zodat je ook van daaruit door de overeenkomsten kunt gaan.
* **Ga naar:** spring naar een regel, een pagina of een percentage van het document. Kies welke met de keuzelijst Modus.
* **Recente documenten:** alle documenten die je hebt geopend, elk gemarkeerd als momenteel open, gesloten of bestand ontbreekt. Elk document heeft twee schermlezeracties: Verwijderen haalt het uit de lijst en Terugvinden laat je een document terugvinden waarvan het bestand is verplaatst. Recente documenten wissen leegt de lijst zonder documenten te verwijderen.
* **Woordenaantal:** het aantal woorden in het document.
* **Documentinformatie:** de titel, de auteur, de bestandsnaam en op iOS ook het aantal regels en tekens.
* **Exporteren:** slaat het document op als platte tekst, HTML of Markdown.
* **Slaaptimer:** stopt het lezen na 5, 10, 15, 30, 45 of 60 minuten, of na een tijd die je zelf opgeeft. Open hem opnieuw terwijl hij loopt om te zien hoeveel tijd er nog over is, of om hem te annuleren.
* **Help:** opent deze handleiding.
* **Instellingen:**
    * **Tekst-naar-spraak:** de stem, spreeksnelheid en toonhoogte, een knop Voorbeeld afspelen om ze te horen, en de pauze tussen alinea's. Op Android kun je hier ook de spraakengine kiezen. Op iOS vind je hier ook het uitspraakwoordenboek: regels die veranderen hoe woorden worden uitgesproken, voor elke stem of alleen voor sommige.
    * **Leesbaarheid:** tekstgrootte, regelafstand, alinea-afstand, uitlijning en tekst met hoog contrast. iOS heeft ook een lichte en een donkere weergave.
    * **Gedrag:** of je documenten opnieuw worden geopend wanneer de app start, welke kant een veeg op de afspeelknop op gaat, en of de knoppen Vorige en Volgende worden verborgen. Op Android staat hier ook de bestandsbrowser in de app.

### Toetsenborden en headsets

Met een toetsenbord werken alle desktopsneltoetsen voor het openen van boeken, recente documenten, Zoeken, Ga naar, de inhoudsopgave, woordenaantal, documentinformatie, exporteren en de slaaptimer, met `Cmd` in plaats van `Ctrl` op iOS. Dat geldt ook voor de lettertoetsen om per kop, pagina, link enzovoort te navigeren, en `Spatie` speelt af en pauzeert. Op iOS bereiken de lettertoetsen Paperback alleen zolang de snelnavigatie met één letter van VoiceOver uit staat.

Op Android speelt een headsetknop af en pauzeert met één druk, gaat vooruit met twee en terug met drie.

## Ondersteunde talen

Paperback wordt vertaald in veel verschillende talen, en er komen voortdurend talen bij. Hieronder volgt een volledige lijst.

Lees onze [Vertaalgids](translating.md) om te leren hoe je kunt bijdragen.

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

## Met dank aan
### Ontwikkeling
* Quin Gillespie: hoofdontwikkelaar en oprichter van het project.
* Aryan Choudhary: voornaamste bijdrager.

### Donaties
De volgende mensen hebben een donatie van enige omvang gedaan aan de ontwikkeling van Paperback. Als je een donatie doet, wordt je naam hier niet automatisch toegevoegd; ik voeg alleen mensen toe die hun donatie openbaar willen maken.

Let op: een openbare GitHub-sponsor beschouw ik als grond voor automatische opname in deze lijst.

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

1.0 is de eerste release op alle vijf platforms: Windows, macOS, Linux, iOS en Android, met de iOS- en Android-apps in de App Store en Google Play.

#### Toegevoegd

##### Algemeen
* Linux-ondersteuning, als AppImage of tar.gz, met desktopintegratie zodat documenten openen vanuit je bestandsbeheerder.
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles van daar tot waar je bent met `Alt+F10`, en ga terug naar de markering met `Alt+Shift+F9`, voor het kopiëren van een lange reeks tekst zonder shift-pijl-toetsen. Alle drie staan onder Extra > Selecteren en kopiëren.
* De sneltoets `=` meldt nu de pagina evenals het percentage, bijv. "15%, pagina 30", en blijft hetzelfde voor documenten zonder paginanummers.
* Het Informatie-venster toont nu de licentie van Paperback en elke vertaler.
* Een Oekraïense vertaling.

##### Nieuwe indelingen
* Comic-archieven (`.cbz`).
* M4B-audioboeken, gesplitst in hun hoofdstukken.
* Handleidingen, zowel `man` als BSD `mdoc`, gzip-comprimeerd of niet.
* MP3-audioboeken, gesplitst in hoofdstukken wanneer het bestand die bevat.
* reStructuredText-documenten.
* Windows Write (`.wri`)-bestanden.
* WinHelp (`.hlp`)-bestanden.
* Word 6 en Word 95-documenten.

##### OCR
* Gescande PDF-pagina's kunnen nu worden herkend met de OCR ingebouwd in Windows en macOS. Druk `Enter` op een gescande pagina om deze te herkennen, of gebruik Bulk-OCR (`Ctrl+Shift+O`) voor een reeks pagina's.

##### Navigatie
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met behulp van MathCAT. Gebruik `M` of `Shift+M` om formules te navigeren, vervolgens `Enter` of `Spatie` om de originele MathML in Formuleweergave te openen.
* Een knop Alle zoeken in het Zoeken-dialoogvenster, met een lijst van elke regel met een overeenkomst zodat je direct naar de gewenste regel kunt springen.
* Tabellen-, Lijsten- en Pagina's-weergaven in de elementenlijst (`F7`).
* Ga naar regel, Ga naar pagina en Ga naar procent accepteren nu `+n` en `-n` voor verplaatsing relatief tot waar je bent.
* EPUB-, MOBI- en CHM-boeken zonder eigen koppen krijgen nu kopnavigatie uit hun inhoudsopgave.
* KF8 (AZW3)-boeken ondersteunen nu sectienavigatie.
* EPUB-pagina's die alleen een afbeelding zijn, tonen nu een regel ervoor, zodat je erop kunt landen in plaats van erlangs te springen.

##### Audioboeken
* Afspeelsnelheid regelaar, van halve snelheid tot driemaal zo snel. Gebruik `Ctrl+Shift+.` en `Ctrl+Shift+,`, of het Extra-menu.
* Bladwijzers en notities in audioboeken alleen onthouden nu de exacte tijd waarop je ze hebt ingesteld.
* Volgende en vorige positie (`Alt+Left` en `Alt+Right`) werken nu in audioboeken.
* Voortgang door een audioboek wordt nu gemeten aan de hand van de opname, zodat Ga naar procent en de statusbalk overeenkomen met hoe ver je er echt in bent.

##### Recente documenten
* Een item Recente documenten wissen in het submenu Recente documenten.

##### PDF-documenten
* Een instelling om elke regel van een PDF apart te houden, in plaats van deze samen te voegen in alinea's.
* Afbeeldingen en figuren in PDF's worden nu medegedeeld.
* PDF's die leesstructuur bevatten maar geen van hun afbeeldingen labelen, melden nu die afbeeldingen, in plaats van ze volledig uit het boek weg te laten.

##### Webweergave
* Elk document kan nu in de webweergave worden geopend, niet alleen EPUB, HTML en Markdown.

##### Leesbaarheid
* Koppen worden nu getekend in een grootte die aansluit bij hun niveau, en afbeeldingen en tabellen worden gescheiden van de omliggende tekst.

##### pb
* `pb --list-formats` geeft een lijst van elke indeling die pb kan lezen.
* pb geeft nu aan welk bestand het niet kon lezen en waarom.

#### Opgelost

##### Algemeen
* Een crash bij het sluiten van Paperback opgelost.
* Paperback sluit nu het venster direct, in plaats van het op het scherm te laten terwijl het opslaat.
* Een document openen schakelt nu niet meer Laatst gesloten heropenen in als er niets te heropenen is.
* Paperback probeert documenten in je recente lijst die verdwenen zijn niet meer opnieuw te laden, en beperkt het aantal recente documenten dat het opslaat.
* Het oude INI-instellingenbestand wordt nu verwijderd zodra het naar het nieuwe formaat is verplaatst.
* De titels van de dialoogvensters voor lettertype en kleur, en het menu Exporteren als in het Vietnamees, zijn nu vertaald.
* Bijwerken brengt het opnieuw gestarte venster nu naar voren, in plaats van het achter alle andere vensters in Alt+Tab te laten.
* Automatische terugloop wordt nu onmiddellijk toegepast op grote documenten, in plaats van het hele ding opnieuw te laden.

##### Navigatie
* `Alt+Left` gaat nu terug naar waar je vandaan bent gesprongen, in plaats van naar een oudere positie.
* Bladwijzersounds worden nu alleen afgespeeld wanneer je over een bladwijzer beweegt, niet wanneer je op de regel ervan landt.
* Het sluiten van de inhoudsopgave, de elementenlijst en de Ga-dialoogvensters brengt je nu rechtstreeks naar de regel waar je op landt, in plaats van je de schermlezer het venster opnieuw te laten uitspreken.
* Ga naar regel, Ga naar pagina en Ga naar procent weigeren nu nummers buiten het document in plaats van stil ergens anders heen te gaan.
* NVDA beperkt de aankondiging niet meer wanneer een document geen pagina's heeft.
* OK indrukken in de inhoudsopgave zonder te bewegen gaat nu naar het item dat al was geselecteerd.
* De inhoudsopgave, de elementenlijst en de bladwijzerlijst vertragen of bevriezen niet meer bij boeken met duizenden items.
* Pijltjes omhoog en omlaag onthouden nu hun kolom per document, in plaats van het over te dragen wanneer je van tabblad wisselt.

##### Audioboeken
* Afspelen van audio gebruikt nu `Control+Space` op macOS, omdat `Command+Space` bij Spotlight hoort.

##### PDF-documenten
* PDF's geëxporteerd uit Apple Pages die als platte tekst werden gelezen, zonder de koppen en lijsten waarmee ze waren geschreven, opgelost.
* PDF-alinea's en -koppen die op elke regel worden gesplitst, en woorden die bij spaties worden gesplitst, opgelost.
* Genummerde PDF-koppen die in één kop samenvloeien, opgelost.
* PDF's waarvan de structuurwagen tot geen tekst leidt en leeg openen, opgelost.
* Paginakop- en voetregels worden niet meer op elke pagina van niet-getagde PDF's uitgesproken.
* PDF's die hun paginakop- en voetregels als gewone tekst taggen, herhalen de titel en het paginanummer niet meer tussen twee alinea's op elke pagina.
* PDF's tonen nu hun echte titel, in plaats van hun bestandsnaam.
* Regels ingesteld in een vaste-breedtelettertype, zoals code, worden niet meer samengevoegd tot alinea's.

##### MOBI/AZW3-boeken
* Grote MOBI-boeken raken niet meer zonder geheugen en worden niet meer afgekapt na 20 MB.
* MOBI- en AZW3-boeken openen nu veel sneller.
* MOBI-boeken die hun hoofdstuklijst verliezen, opgelost.
* Verminkte tekst waar MOBI-boeken van de ene record naar de volgende gaan, opgelost.

##### Webweergave
* De webweergave laadt niet meer het geheel van een enorm boek tegelijk.
* De webweergave toont documenten nu geheel wanneer de lezer ze geheel toont, in plaats van slechts een stukje ervan.

##### Andere formaten
* FictionBook-boeken (.fb2) geschreven in windows-1251, wat de meeste zijn, openen nu in plaats van helemaal niet te kunnen worden gelezen.
* FictionBook-boeken die een naamruimte of een HTML-entiteit gebruiken die ze nooit hebben gedeclareerd, openen nu, in plaats van als verbroken te worden geweigerd.
* Boeken in verouderde coderingen openen nu veel sneller.
* Sommige Chinese tekstbestanden die als verminkte tekst openen, opgelost.
* Met wachtwoord beveiligde OpenDocument-bestanden vragen nu om hun wachtwoord, in plaats van als verbroken te worden gerapporteerd.
* Met wachtwoord beveiligde verouderde PowerPoint-bestanden openen nu, en verouderde PowerPoint-dia's verliezen hun tekst niet meer.
* Platte tekstbestanden opgeslagen met een `.rtf`-extensie openen nu als tekst, in plaats van te mislukken met een fout.
* RTF-controlwoorden verschijnen niet meer als tekst.

#### iOS en Android

De iOS- en Android-apps openen elk formaat dat het bureaublad doet, en bevatten:

* Voorlezen, met jouw keuze van stem, snelheid en toonhoogte, een spreeksnelheidsbediening direct op de leesbalk, en een optionele pauze tussen alinea's.
* Afspelen van DAISY-, M4B- en MP3-audioboeken, die op de achtergrond en vanaf het vergrendelingsscherm doorloopt.
* Navigatie op basis van koppen, pagina's, links, tabellen, lijsten en meer van de leesbalk, plus de inhoudsopgave en Zoeken.
* Een slaaptimer, woordenaantal en documentexport, plus een uitspraakwoordenboek op iOS.
* Opties voor tekstgrootte, afstand en hoog contrast.
* Toetsenbordsneltoetsen die aansluiten op het bureaublad.

### Versie 0.9.2
* Audioboeken laten je schermlezer niet meer een reeks spaties uitspreken wanneer je de focus op het tekstveld richt.
* Audioboeken noemen het bestand nu wanneer je er door secties doorheen stapt.
* Audioboeken melden nu hun werkelijke lengte, in plaats van te beweren dat elk bestand erin 24 uur duurt.
* Het sluiten van de Webweergave met Escape geeft niet meer een debug-waarschuwing nadat je een link erin hebt gevolgd.
* Kopiëren na Alles selecteren geeft je nu het hele document, in plaats van alleen het gedeelte dat nu is geladen.
* Zoeken gaat nu rechtstreeks naar de regel die het heeft gevonden, in plaats van je de schermlezer het venster opnieuw te laten uitspreken terwijl de focus terugkeert naar het boek.
* EPUB's die een losse ZIP64-blok dragen en weigeren te openen met "Ongeldige lokale bestandskop", opgelost.
* Lange documenten die naar hun begin terugvallen terwijl een schermlezer er continu doorheen leest, opgelost.
* Links in de WebView brengen je nu naar de sectie waar ze naar wijzen, in plaats van met "Bestand niet gevonden" te mislukken.
* De automatische "Document opnieuw geladen"-aankondiging onderbreekt je schermlezer niet meer halverwege een zin, maar wacht in plaats daarvan tot het klaar is met wat het zegt.
* Het tabblad Algemeen van het dialoogvenster Instellingen maakt nu tabs via zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie Controleren op updates.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordenaantal en Documentinformatie tonen nu hoeveel bestanden een audioboek bevat, en hoe lang het in totaal loopt.

### Versie 0.9.1
* Bladwijzer- en notitiegeluiden spelen nu af op macOS.
* DAISY-boeken spelen nu hun audio af op macOS, in plaats van te openen en hun tijdlijn stil af te spelen.
* Gekrulde aanhalingstekens, gedachtestreepjes en soortgelijke tekens die verdwenen uit RTF-documenten, zijn opgelost; de omringende woorden werden niet meer aan elkaar geplakt.
* RTF-afbeeldingen die hun onbewerkte gegevens als verminkte tekst in het document uitlekten, zijn opgelost.
* Het submenu Recente documenten dat verouderde items behield tot iets anders het herbouwde, is opgelost.
* Toetsenbordsneltoetsen zijn terug in elke vertaling, dus de menu's van het Russisch hebben weer toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, zodat ze in de taakbalkspronglijst en in de recente lijst van het Startmenu worden weergegeven.
* Instellingen hebben nu dezelfde naam als in de mobile apps en op macOS de platformconventie.
* Paperback onthoudt nu zijn vensterpositie, grootte en gemaximaliseerde staat tussen sessies.
* Meervoudsvormen zijn nu vertaald, zodat berichten die dingen tellen correct worden weergegeven in talen die meer dan één vorm nodig hebben.
* Een DAISY-boek's ncc.html selecteren opent nu het volledige audioboek in plaats van alleen de tekstversie.
* De actienamen in het dialoogvenster Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu vooraan in de titelbalk, zodat geopende boeken in de taakbalk en bij Alt+Tab van elkaar kunnen worden onderscheiden.
* Het bijwerkingsdialoogvenster is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een terminalprogramma, pb genaamd, om snel alle ondersteunde indelingen van Paperback naar HTML, Markdown of platte tekst om te zetten.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alles wat je vreemd vindt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Volledige macOS-ondersteuning!
* Een volledigschermmodus in-/uitschakelen.

##### Dialoogvenster Alle documenten
* Een button Terugvinden om vermiste boeken terug te vinden die hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat je kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten te deselecteren.

##### Instellingen en leesbaarheid
* Een leesbaarheidstabblad met de volgende opties:
    * Automatische terugloop (verplaatst van Algemeen);
    * Tabellen in de tekst weergeven (nieuw in deze versie, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor automatische terugloop en de bijbehorende sneltoets.
* Een schakeloptie om te bepalen hoe je tabellen weergegeven wilt zien, en de manier waarop tabellen in documenten worden weergegeven is nu uniform.

##### Navigatie
* Ondersteuning voor navigatie op container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met bladermodus in schermlezerss.
* De gelijkheidssneltoets om je huidige percentage door een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en ze blijven behouden. Gebruik schuine streep om er een in te stellen en omgekeerde schuine streep om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het dialoogvenster Woordentelling, en de mogelijkheid om je leessnelheid in te stellen om deze maatstaf echt bruikbaar te maken.
* Als er een selectie actief is wanneer je het dialoogvenster Woordentelling opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordsneltoets om Paperback uit het systeemvak terug te halen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het exportmenu-item is uitgebreid om naar HTML en Markdown te kunnen exporteren, naast platte tekst.

##### Updater
* Een button Annuleren voor het dialoogvenster Bijwerking wordt uitgevoerd.
* De updater valideert nu dat het gedownloade bestand niet is gemanipuleerd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audiowerking.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, ondersteunt momenteel zowel DAISY-audio (inclusief DAISY-audio + tekst) als zipbestanden met audiobestanden.
* Toetsenbordsneltoetsen en menu-items om audio af te spelen/te pauzeren, vooruit en achteruit te spoelen, en de audiospoelstap aan te passen.
* Opties om de leesCursor met audiowerking te synchroniseren, de audiospoelstap in te stellen en te kiezen of vooruitspoelen voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Verholpen

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als onleesbare tekst.
* "Laatst gesloten heropenen" probeerde de gebundelde handleiding herop te openen.
* Je geselecteerde tabblad kreeg geen goede focus na het herstarten van Paperback.
* Paperback's afhandeling van bestanden op Windows-netwerkstations: het drukken op bestandslocatie openen focust nu correct het bestand op de netwerkopslag, en de paden bevatten geen vreemde tekens meer.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* De map openen focust nu het gegeven bestand in de verkenner.
* Het openen van de handleiding respecteert nu je geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op displays met hoge DPI.
* Het menu wordt nu correct bijgewerkt en focus verplaatst naar de tekstbesturing wanneer help in Paperback wordt geopend.
* Overgegaan naar een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het schakelen tussen tabbladen.
* Geheugengebruik op grote documenten verminderd door de grootte van de interne per-teken indexeringstabellen te halveren.

##### Dialoogvenster Alle documenten
* Escape sloot het dialoogvenster Documentinformatie en Alle documenten niet.
* De titelbalk werd niet bijgewerkt na het sluiten van een document in het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan je lijst met alle documenten toegevoegd wanneer deze wordt geopend via `Shift+F1`.
* Het verwijderen van documenten uit het dialoogvenster recente documenten sluit nu ook hun actieve tabblad.
* Je zoekfilter wordt nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regeltekst aan.
* Ga naar regel, Ga naar pagina en Ga naar percentage plaatsen je cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Bladwijzer-/notitiegeluiden moeten nu correct exclusief afspelen wanneer je over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Automatische terugloop toepassen schoot je naar het begin van je document.

##### Webweergave
* Het webweergavedialoogvenster kon niet in grootte worden gewijzigd en verscheen met een erg kleine initiële grootte.
* Afbeeldingen moeten nu correct worden weergegeven in de ingesloten webweergave.

##### Updater
* De updater geeft nu correct de inhoud van markdown-codetags in releaseopmerkingen weer.

##### DAISY-boeken
* DAISY-boeken geven onjuiste info in de statusbalk.
* DAISY-boeken laden met valse coderingsverklaringen.

##### RTF-documenten
* RTF-documenten verwerken met niet-Latijnse tekens erin.
* RTF `\pict` groepen zodat ingesloten afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en plaatsen rommel in de boektekst.
* Links in klassieke Mobi-boeken.
* Aanzienlijk verbeterde AZW3-verwerking.

##### Word-documenten
* Word-documenten met lokale-specifieke stijlnamen geven hun kopjes niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen produceren geen regelafbrekingen in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor onterecht gelabelde PDF's.
* PDF-documenten met kontroletekens in hun titels en/of bladwijzers crashen Paperback niet langer bij het openen.

### Versie 0.8.5
* Paginaondersteuning aan epub-boeken toegevoegd.
* Ondersteuning voor versleutelde Microsoft Office-documenten toegevoegd. Op dit moment worden klassieke Word, moderne Word en moderne Powerpoint ondersteund, met klassieke Powerpoint gepland voor de toekomst.
* Ondersteuning voor klassieke Microsoft Word-documenten toegevoegd!
* Ondersteuning voor klassieke Powerpoint-presentaties toegevoegd!
* Ondersteuning voor mobi- en AZW3-boeken toegevoegd!
* Ondersteuning voor getagde PDF-bestanden toegevoegd!
* De sneltoets `Ctrl+Q` voor het sluiten van de app toegevoegd.
* Ondersteuning voor gecomprimeerde boeken van Bookshare toegevoegd (zowel DAISY als Word)!
* Alt-tekst voor ingesloten afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct interne linksnavigatie.
* Ga naar pagina vastgesteld was niet meer af met 1.
* Escape-toets werkt nu niet goed om het dialoogvenster Openen als te sluiten.
* Het contextmenu van de lezer verschijnt nu niet op `Shift+muis rechts` of de Applications-toets.
* Het verkeerde document werd soms gefocust wanneer documenten vanaf de terminal werden geopend.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen je van hun bestaan.
* Het is nu mogelijk om afbeeldingen en figuren te navigeren met `G`/`Shift+G` en `F`/`Shift+F`.
* Paperback respecteert nu je instellingen voor donkere modus van de applicatie.
* DAISY XML-ondersteuning verwijderd, omdat dit niet langer nodig is.
* Teruggegaan naar de native Win32-navigatie met eerste letter in de inhoudsopgaveboom.
* Het dialoogvenster fout bij laden geeft nu gedetailleerdere foutberichten.
* De webweergave wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Paginaondersteuning aan RTF-documenten toegevoegd!
* Een fout verholpen waarbij het openen van de webweergave in epubs met externe links deze automatisch zou activeren.
* Een fout verholpen waarbij de RTF-verwerker in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's gesplitst in meerdere korte regels in sommige PDF-documenten.
* PDF-documenten hebben nu ondersteuning voor basis link- en kopnavigatie!
* RTF-tabs en regeleinden worden nu precies weergegeven zoals ze in het document verschijnen.
* Teruggeschakeld naar de beproefde pdfium-bibliotheek voor het verwerken van PDF's, waardoor PDF-rendering opnieuw veel betrouwbaarder is.

### Versie 0.8.1
* `Ctrl+Shift+T` toegevoegd om het laatst gesloten document herop te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar fouten in de RTF-verwerker verholpen.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) die beschadigd raakten wanneer een bestand werd geopend via een tweede Paperback-exemplaar.
* PDF-tekst die in de verkeerde volgorde werd gelezen en onjuiste spatiëring rond gekapitaliseerde woorden.
* Trage documentlading bij het openen van grote bestanden.
* De lokalisatie van de knopppen Ja/Nee in bevestigingsdialoogvensters.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Automatische update-functie toegevoegd die je huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of een notitie, dank aan Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor bestanden met platte Open Document Text toegevoegd!
* Ondersteuning voor presentaties met platte Open Document toegevoegd!
* Ondersteuning voor scheidingen met s en `shift+s` toegevoegd.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan je navigatiegeschiedenis.
* Paperback-venster terugzetten vanuit het systeemvak hersteld.
* Markdown-documenten die onbewerkte tekst weergeven in plaats van weergegeven HTML in de webweergave hersteld.
* Tabellen die niet goed weergegeven worden in Markdown-bestanden hersteld.
* PDF's die alleen afbeeldingen bevatten geven je nu een waarschuwing als je deze probeert te laden.
* Versieinformatie correct ingebed in het Paperback-uitvoeringsbestand.
* Instellingendialoog opgesplitst in tabbladen voor eenvoudig gebruik en navigatie.
* Overschakeld naar Hayro voor het verwerken van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* Hele app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML en XHTML-documenten toegevoegd! Navigeer tussen tabellen met T en `shift+t`, en druk `Enter` om er een in een webweergave te bekijken.
* Basale webweergavefunctie toegevoegd! Druk `Ctrl+Shift+V` om het huidige onderdeel van je document in een webgebaseerde renderer te openen, nuttig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Russische vertaling toegevoegd, dank aan Ruslan Gulmagomedov!
* Knop Alles wissen toegevoegd aan de dialoog Alle documenten.
* De updatecontrole geeft nu releaseopmerkingen weer als een nieuwe versie beschikbaar is.
* Venster terugzetten vanuit het systeemvak hersteld.
* Ja/Nee-knopvertalingen in bevestigingsdialogen hersteld.
* Configuraties laden bij uitvoering als beheerder hersteld.
* Commentaarverwerking in XML- en HTML-documenten hersteld.
* Inhoudsopgaveverwerking in Epub 2-boeken hersteld.
* Navigeren naar het volgende item met dezelfde letter in de inhoudsopgave hersteld.
* Zoekopdracht-dialoog die niet goed verborgen wordt bij gebruik van de vorige/volgende knoppen hersteld.
* Epub-inhoudsopgave werpt je niet meer naar het verkeerde item.
* Verschillende witruimte-verwerkingsproblemen in XML, HTML en pre-tags hersteld.
* Fout in linknavigatie off-by-one hersteld.
* Sommige boeken met resterende witruimte aan het einde van hun regels hersteld.
* Verschillende verwerkingsproblemen hersteld.
* Bladwijzer-gerelateerde menu-items en de elementenlijst zijn nu correct uitgeschakeld als er geen document is geopend.
* Lijstverwerking in verschillende documentindelingen verbeterd.
* Vertaalworkflow voor bijdragers verbeterd.
* Veel interne refactors, waarbij het merendeel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF-bestanden toegevoegd!
* Zeer basale functie voor navigeren naar vorige/volgende positie toegevoegd. Als je op een interne link drukt en de cursor beweegt, wordt die positie onthouden en kan met `alt+links`/`alt+rechts` pijlen naar genavigeerd.
* Elementenlijst toegevoegd! Op dit moment toont deze alleen een boom van alle koppen in je document of een lijst met links, maar er zijn plannen om deze in de toekomst uit te breiden.
* Optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Links in sommige Epub-documenten werken niet correct hersteld.
* Epub-inhoudsopgaven met relatieve paden hersteld.
* Sommige epub-documenten geven geen titel of auteur weer hersteld.
* Titels van sommige epub-hoofdstukken worden niet correct in de inhoudsopgave-dialoog weergegeven hersteld.
* Je kon de spatiebalk niet gebruiken om de knoppen OK/annuleren in de inhoudsopgave-dialoog te activeren hersteld.
* Verwerking van koppen in Word-documenten verbeterd.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is als je de dialoog probeert weer te geven.

### Versie 0.6.0
* Een nieuwe optie is toegevoegd aan het optiesdialoogvenster om het Ga-menu in een veel compactere vorm weer te geven, standaard aangevinkt.
* Een optie toegevoegd om navigatie op basis van structuurelementen rond te laten lopen.
* Een optie toegevoegd aan het menu Extra om de map met het huidige document te openen.
* Een tamelijk eenvoudig, maar erg effectief updatesysteem toegevoegd.
* Een basis slaaptimer-functie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het verwerken van FB2-ebooks!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu gebruikt worden om een hele regel te markeren, of alleen bepaalde tekst te markeren. Als je geen selectie actief hebt wanneer je een bladwijzer plaatst, is het gedrag hetzelfde als in versies voor 0.6, en wordt de hele regel gemarkeerd. Als je echter tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bevatten! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het dialoogvenster voor bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster voor bladwijzers hebben niet langer een vervelend "bladwijzer x"-voorvoegsel.
* EPUB-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Laden van grote Markdown-documenten gecorrigeerd.
* Het indrukken van spatie in de boomweergave van de inhoudsopgave die de OK-knop activeert, gecorrigeerd.
* Whitespace-verwerking aan het begin van pre-tags in zowel HTML- als XHTML-documenten gecorrigeerd.
* Het tekstbesturingselement dat soms niet opnieuw focus krijgt wanneer je terugkeert naar het Paperback-venster, gecorrigeerd.
* Het tekstveld in het dialoogvenster "Ga naar procent" dat de waarde van de schuifbalk niet bijwerkt, gecorrigeerd.
* De weergave van aangepaste HTML-id's in Markdown-documenten gecorrigeerd.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek laden met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijg je niet langer een fout als het laden van je document langer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk een bladwijzer rechtstreeks vanuit het dialoogvenster voor bladwijzers te verwijderen.
* Het is nu mogelijk je bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand heet hetzelfde als het bestand, maar met een .paperback-extensie. Als zo'n bestand in dezelfde map als een bestand wordt gevonden wanneer het wordt geladen, wordt het automatisch geladen. Anders kunt je ze handmatig importeren met behulp van een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik K en Shift+K om vooruit en achteruit door links te navigeren, en druk op Enter om er een te openen/activeren.
* Veel interne refactorings, waardoor de app sneller is en het binaire bestand kleiner.
* Markdown-inhoud wordt nu voorverwerkt om conform CommonMark te zijn voordat deze wordt weergegeven.
* Navigatie op basis van lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om via lijsten zelf te gaan, en I en Shift+I om door lijstitems te gaan.
* Numpad Delete werkt nu ook om documenten uit de tabbalk te verwijderen, naast normale Delete.
* Paperback kan nu optioneel naar je systeemvak minimaliseren! Deze optie is standaard uit, maar als je hem inschakelt, plaatst de minimaliseeroptie in het systeemmenu Paperback in je vak, zodat je het kunt herstellen door op het gemaakte pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met ondersteunde talen is momenteel tamelijk klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website, op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basis-inhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster voor documentinformatie.
* Het installatieprogramma bevat nu een optie om de handleiding in je browser weer te geven na de installatie.
* De lijst met recente documenten is enorm uitgebreid! In plaats van alleen de laatste 10 geopende documenten weer te geven, toont het nu een aanpasbaar aantal, waarbij de rest van de documenten die je ooit hebt geopend, via een klein dialoogvenster toegankelijk is.
* Verschillende kleine verbeteringen aan de verwerkers over de hele linie, waaronder het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het corrigeren van de newline-verwerking in alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items die niet waren uitgeschakeld zonder geopende documenten, zijn nu opgelost.
* De oriëntatie van de schuifregelaar voor ga naar procent is opgelost.
* De inhoudsopgave in EPUB-boeken met URL-gecodeerde bestandspaden en/of fragmentnummers is opgelost.
* Witruimte die op vreemde manier uit XHTML-koppen werd verwijderd, is opgelost.
* Witruimteverwerking in geneste pre-tags in HTML-documenten is opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Als je een HTML-/Markdown-document laadt, zal Paperback zijn eigen inhoudsopgave uit de structuur van de koppen in je document bouwen en deze in het `ctrl+t`-dialoogvenster tonen.
* HTML-documenten hebben nu de titel zoals deze in de title-tag is ingesteld, als deze bestaat. Anders gebruiken ze de bestandsnaam zonder extensie.
* Overgeschakeld van UniversalSpeech naar een live-regio om spraak te melden. Dit betekent dat er geen schermlezer-DLL's meer bij het programma worden geleverd en dat meer schermlezers nu worden ondersteund, zoals Microsoft Narrator.
* Zip-bibliotheken overgeschakeld om een breder scala aan EPUB-boeken te kunnen openen.
* Het dialoogvenster waarin je wordt gevraagd of je je document als platte tekst wilt openen, is volledig opnieuw gemaakt en je kunt je document nu als platte tekst, HTML of Markdown openen.
* Het dialoogvenster ga naar procent bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naar te springen.
* De HTML-verwerker herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in EPUB-boeken wordt nu opnieuw precies behouden.
* De Unicode-spatieëring zonder regelovergang wordt nu meegenomen bij het verwijderen van lege regels.
* Je wordt niet meer gevraagd hoe je een onherkenbaar bestand wilt openen elke keer dat je het laadt, alleen de eerste keer.

### Versie 0.4.1
* Een optionaal pictogram in het startmenu aan het installatieprogramma toegevoegd.
* De inhoudsopgave zou nu in enkele gevallen schoner moeten zijn, bijvoorbeeld als je een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten is opgelost.
* De inhoudsopgave in EPUB 3-boeken met absolute paden is opgelost.
* CHM-documenten moeten nu hun titel weergeven zoals deze in het metagegevensbestand is ingesteld.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers in zoveel documenten hebben als je wilt. Je kunt er met b en `shift+b` doorheen springen, er een instellen met `ctrl+shift+b` en een dialoogvenster openen om naar een specifieke bladwijzer te springen met `ctrl+b`.
* Een installatieprogramma naast het draagbare zipbestand toegevoegd! Het installatieprogramma zal Paperback in je Program Files-map installeren en zal automatisch bestandskoppelingen voor je instellen.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd en de BOM wordt ook niet meer aan het begin van de tekst weergegeven.
* Veel meer informatie aan de statusbalk toegevoegd. Het toont nu je huidige regel, teken en leespercentage.
* HTML-opmerkingen en de inhoud van script- en style-tags worden niet meer in tekstuitvoer weergegeven.
* Bij het doorgeven van een relatief pad aan Paperback op de opdrachtregel, zal het dit nu correct omzetten.
* Percentagebewegingen worden nu afgehandeld door hun eigen dialoogvenster op basis van een schuifregelaar, toegankelijk met `ctrl+shift+g`.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaardwaarde.
* De logica voor positieopslag is nu veel slimmer en zou alleen naar de schijf moeten schrijven wanneer dit absoluut nodig is.
* Het document waarop je je concentreerde toen je Paperback sloot, wordt nu onthouden na het herstarten van de applicatie.
* Invoer in de dialoogvensters ga naar regel en ga naar pagina moet nu stricter worden opgeschoond.
* Inhoudsopgavenavigatie in EPUB 3-boeken met relatieve paden in hun manifesten is opgelost.

### Versie 0.3.0
* De inhoudsopgave in EPUB-boeken met URL-gecodeerde manifesten is opgelost.
* Koppen navigeren in HTML-documenten met multi-byte Unicode-tekens is opgelost.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets is opgelost.
* Het laden van UTF-8-tekstbestanden is opgelost.
* Geneste inhoudsopgave-items in EPUB-boeken die je cursor op de verkeerde positie zetten, zijn opgelost.
* Een crash bij afsluiten van de applicatie in bepaalde gevallen is opgelost.
* Een selectievakje in het dialoogvenster Opties toegevoegd om automatische terugloop in of uit te schakelen!
* Het is nu mogelijk om aan de ontwikkeling van Paperback bij te dragen, via het nieuwe doneeritem in het Help-menu of via de link Dit project sponsoren onderaan de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel en Paperback zou nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Overschakeld naar PDF-bibliotheken die in Chromium worden gebruikt, wat leidt tot veel betrouwbaardere PDF-verwerking in het algemeen.
* Je kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Paperback.exe uitvoeren met een bestandsnaam terwijl het al actief is, zal dat document in het al actieve exemplaar openen.
* Je kunt nu op verwijderen drukken op een document in de tabbladbesturing om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's aan het paginalabel in het dialoogvenster ga naar pagina toegevoegd.
* Tabben vanuit de documentinhoud naar je lijst met geopende documenten ingesteld.
* Koppen-sneltoetsen die soms recente documenten openden als je er genoeg van had, zijn opgelost.
* Paperback verwijdert nu onnodige zachte streepjes uit tekstuitvoer.
* Kopnavigatie die je soms op het verkeerde teken plaatste, is opgelost.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsenbordsneltoetsen toegevoegd voor navigatie per kop in HTML-inhoud, inclusief EPUB-boeken en markdown-documenten. Deze sneltoetsen zijn ontworpen om op dezelfde manier te werken als een schermlezer.
* Laden van EPUB's met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Laden van EPUB 3-boeken met daarin ingebedde XHTML opgelost.
* Er wordt nu een bericht gesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items uitgeschakeld worden.
* Recente documenten-menu toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op, en op enter drukken op een ervan opent het voor lezen.
* Zoekopdracht-dialoogvenster volledig herschreven, veel eenvoudiger te gebruiken, en er is nu een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies!
* Eerder geopende documenten worden nu onthouden na het herstarten van de applicatie. Dit is configureerbaar via het nieuwe optie-item in het Extra-menu.
* `Shift+F1` toegevoegd om de handleiding rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Initiële release.
