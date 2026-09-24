<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

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

## Schermlezer-compatibiliteit

Paperback werkt goed met alle belangrijke schermlezers. Er zijn echter twee bekende problemen voor JAWS-gebruikers.

### JAWS en brailleleesregels

Als je JAWS gebruikt met een brailleleesregel, kan het gebeuren dat lange alinea's worden afgekapt bij het vooruitscrollen met de navigatietoetsen van je display. Het commando voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een bug in JAWS's omgang met het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf. Dit probleem duurde lang voordat er een oplossing kwam, gezien Vispero's enthousiasme voor het reageren op problemen met open source-software.

De omleiding, uiteindelijk naar voren gekomen via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". Je wilt ook "Pan Text by Paragraph" inschakelen, anders blijft je display op de actieve alinea staan in plaats van vooruit te gaan. Met beide instellingen op hun plaats zou het schuiven correct moeten werken.

### JAWS en de meldingen van Paperback

Paperback zegt dingen zoals "No pages." of "This document has no audio." als toegankelijkheidsmeldingen, waardoor een schermlezer ze kan uitspreken over wat het zegt. JAWS reageert hierop alleen wanneer "Enable accessible notification events" is ingeschakeld voor de applicatie. Op sommige machines is dit niet het geval.

Als JAWS niets zegt wanneer je op een toets drukt die iets zou moeten melden, open je Settings Center met Paperback in beeld (`Insert+6`), zoek naar "notification" en vink "Enable accessible notification events" aan. Dit schrijft de instelling naar `paperback.jcf`, zodat deze alleen op Paperback van toepassing is.

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

## Wijzigingen

### Versie 1.0

1.0 is de eerste release op alle vijf platforms: Windows, macOS, Linux, iOS en Android, met de iOS- en Android-apps in de App Store en Google Play.

#### Toegevoegd

##### Algemeen
* Linux-ondersteuning, als AppImage of tar.gz, met bureaubladintegratie zodat documenten vanuit je bestandsbeheerder openen.
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles vanaf daar tot waar je bent met `Alt+F10`, en ga terug naar het merk met `Alt+Shift+F9`, voor het kopiëren van een lange reeks tekst zonder shift-arrowing. Alle drie staan onder Extra > Selecteren en kopiëren.
* De `=` sneltoets meldt nu de pagina en het percentage, bijvoorbeeld "15%, pagina 30", en blijft zoals het was voor documenten zonder paginanummers.
* Het About-dialoogvenster toont nu de licentie van Paperback en alle vertalers.
* Een Oekraïense vertaling.

##### Nieuwe indelingen
* Comic-archieven (`.cbz`).
* M4B-audioboeken, opgesplitst in hun hoofdstukken.
* Handmatige pagina's, zowel `man` als BSD `mdoc`, gecomprimeerd of niet.
* MP3-audioboeken, opgesplitst in hoofdstukken wanneer het bestand die bevat.
* reStructuredText-documenten.
* Windows Write-bestanden (`.wri`).
* WinHelp-bestanden (`.hlp`).
* Word 6- en Word 95-documenten.

##### OCR
* Gescande PDF-pagina's kunnen nu worden herkend met de OCR die in Windows en macOS is ingebouwd. Druk `Enter` op een gescande pagina om deze te herkennen, of gebruik Bulk-OCR (`Ctrl+Shift+O`) voor een reeks pagina's.

##### Navigatie
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met behulp van MathCAT. Gebruik `M` of `Shift+M` om formules te navigeren, druk vervolgens `Enter` of `Space` om de originele MathML in Formuleweergave te openen.
* Een knop Alles zoeken in het zoekdialoogvenster, met elke regel met een overeenkomst, zodat je direct naar de regel die je wilt kunt springen.
* Weergaven voor tabellen, lijsten en pagina's in de elementenlijst (`F7`).
* Ga naar regel, Ga naar pagina en Ga naar percentage accepteren nu `+n` en `-n` voor relatieve verplaatsing ten opzichte van waar je bent.
* EPUB-, MOBI- en CHM-boeken zonder hun eigen koppen krijgen nu kopnavigatie van hun inhoudsopgave.
* KF8-boeken (AZW3) ondersteunen nu sectienavigatie.
* EPUB-pagina's die alleen een afbeelding zijn, tonen nu een regel ervoor, zodat je erop kunt landen in plaats van er rechtdoor heen te springen.

##### Audioboeken
* Afspeelsnelheid-instellingen, van halve snelheid tot drie keer zo snel. Gebruik `Ctrl+Shift+.` en `Ctrl+Shift+,`, of het menu Extra.
* Bladwijzers en notities in alleen-audioboeken onthouden nu het exacte moment waarop je ze instelt.
* Volgende en vorige positie (`Alt+Left` en `Alt+Right`) werken nu in audioboeken.
* Voortgang door een audioboek wordt nu gemeten aan de hand van de opname, dus Ga naar percentage en de statusbalk komen overeen met hoe ver je echt bent.

##### Recente documenten
* Een item Recente documenten wissen in het submenu Recente documenten.

##### PDF-documenten
* Een instelling om elke regel van een PDF gescheiden te houden, in plaats van ze samen te voegen tot alinea's.
* Afbeeldingen en figuren in PDF's worden nu aangekondigd.
* PDF's die leesstructuur bevatten maar geen van hun afbeeldingen van tags voorzien, melden nu die afbeeldingen, in plaats van ze helemaal uit het boek weg te laten.

##### Webweergave
* Elk document kan nu in de webweergave worden geopend, niet alleen EPUB, HTML en Markdown.

##### Leesbaarheid
* Koppen worden nu getekend in een grootte die past bij hun niveau, en afbeeldingen en tabellen worden gescheiden van de omringende tekst.

##### pb
* `pb --list-formats` geeft alle indelingen weer die pb kan lezen.
* pb geeft nu aan welk bestand niet kon worden gelezen en waarom.

#### Opgelost

##### Algemeen
* Een boek dat bij het opstarten heropend wordt, speelt nu direct af in plaats van stil te blijven totdat het gesloten en opnieuw geopend wordt.
* Een document waarvan het bestand verdwenen is, kan nu uit Alle documenten verwijderd worden in plaats van in de lijst te blijven staan hoe vaak je ook bevestigt.
* Crash bij het sluiten van Paperback opgelost.
* Het sluiten van Paperback verbergt het venster nu direct in plaats van het op het scherm te laten staan terwijl het wordt opgeslagen.
* Grote boeken met weinig opmaak gaan nu ongeveer twee keer zo snel open.
* Berichten die uit een menu worden gekozen, zoals "Dit document heeft geen audio", worden niet langer door de schermlezer afgekapt voordat je ze hoort.
* Het openen van een document schakelt Laatst gesloten heropenen niet meer in als er niets te heropenen is.
* Paperback herprobeert documenten in je recente lijst die verdwenen zijn niet langer, en beperkt het aantal recente documenten dat het opslaat.
* Het oude INI-instellingenbestand wordt nu verwijderd nadat het naar het nieuwe formaat is overgezet.
* De titels van de lettertype- en kleurendialogen en het menu Exporteren als in het Vietnamees zijn nu vertaald.
* Bij het bijwerken verschijnt het heropgestarte venster nu op de voorgrond in plaats van achter alle andere vensters in Alt+Tab.
* Automatische terugloop wordt nu direct toegepast op grote documenten in plaats van het hele document opnieuw te laden.

##### Navigatie
* `Alt+Left` gaat nu terug naar waar je vandaan sprong in plaats van naar een oudere positie.
* Bladwijzerklanken spelen nu alleen af als je over een bladwijzer beweegt, niet als je op de regel waar deze op staat terechtkomt.
* Het sluiten van de inhoudsopgave, de elementenlijst en de Ga-dialogen brengt je nu direct naar de regel waar je terechtkomt in plaats van de schermlezer het venster opnieuw te laten voorlezen.
* Ga naar regel, Ga naar pagina en Ga naar percentage weigeren nu getallen buiten het document in plaats van stilletjes ergens anders heen te gaan.
* NVDA kapt de melding niet langer af als een document geen pagina's heeft.
* Op OK drukken in de inhoudsopgave zonder te verplaatsen gaat nu naar de vermelding die al geselecteerd was.
* De inhoudsopgave, de elementenlijst en de bladwijzerlijst vertonen nu geen vertraging of bevriezen meer op boeken met duizenden vermeldingen.
* Pijl-omhoog en pijl-omlaag onthouden nu hun kolom per document in plaats van deze mee te nemen bij het schakelen tussen tabbladen.

##### Audioboeken
* Audioweergave maakt nu gebruik van `Control+Space` op macOS, omdat `Command+Space` van Spotlight is.

##### PDF-documenten
* PDF's geëxporteerd uit Apple Pages lezen nu correct in plaats van als gewone tekst, zonder de koppen en lijsten waarmee ze geschreven waren.
* PDF-alinea's en koppen splitsen niet langer op elke regel en woorden splitsen niet langer op spaties.
* Genummerde PDF-koppen lopen niet langer in elkaar aan één kop.
* PDF's waarvan de structuurboom tot geen tekst leidt, gaan niet langer leeg open.
* Regels met een monospaced lettertype, zoals code, worden niet langer samengevoegd tot alinea's.
* Paginakop- en voetteksten worden niet langer op elke pagina van niet-gecodeerde PDF's voorgelezen.
* PDF's die hun paginakop- en voetteksten als gewone tekst taggen, herhalen de titel en het paginanummer niet langer tussen twee alinea's op elke pagina.
* PDF's tonen nu hun echte titel in plaats van hun bestandsnaam.

##### MOBI/AZW3-boeken
* Grote MOBI-boeken raken niet langer zonder geheugen en worden niet langer afgekapt na 20 MB.
* MOBI- en AZW3-boeken gaan nu veel sneller open.
* MOBI-boeken die hun hoofdstuklijst verliezen, opgelost.
* Verminkte tekst waar MOBI-boeken van het ene record naar het volgende overgaan, opgelost.

##### Webweergave
* De webweergave laadt niet langer een heel groot boek in één keer.
* De webweergave toont documenten nu in hun geheel als de lezer ze in hun geheel toont, in plaats van slechts een deel ervan.

##### Andere formaten
* FictionBook (.fb2)-boeken geschreven in windows-1251, wat de meeste zijn, gaan nu open in plaats van helemaal niet gelezen te worden.
* FictionBook-boeken die een namespace of HTML-entiteit gebruiken die ze nooit gedeclareerd hebben, gaan nu open in plaats van als verbroken te worden geweigerd.
* Boeken in verouderde coderingen gaan nu veel sneller open.
* Sommige Chinese tekstbestanden die als verminkte tekst werden geopend, opgelost.
* Met wachtwoord beveiligde OpenDocument-bestanden vragen nu naar hun wachtwoord in plaats van als verbroken te worden gemeld.
* Met wachtwoord beveiligde verouderde PowerPoint-bestanden gaan nu open en verouderde PowerPoint-dia's verliezen hun tekst niet langer.
* Platte tekstbestanden met een `.rtf`-extensie openen nu als tekst in plaats van met een fout te mislukken.
* RTF-besturingswoorden verschijnen niet langer als tekst.

#### iOS en Android

De iOS- en Android-apps openen elk formaat dat de desktop-app kan, en bevatten:

* Voorlezen, met je keuze van stem, spreeksnelheid en toonhoogte, een spreeksnelheidsbesturing direct op de leesbalk en een optionele pauze tussen alinea's.
* Afspelen van DAISY-, M4B- en MP3-audioboeken, dat doorgaat op de achtergrond en vanaf het vergrendelscherm.
* Navigatie via koppen, pagina's, links, tabellen, lijsten en meer vanuit de leesbalk, plus de inhoudsopgave en Zoeken.
* Een slaaptimer, woordenaantal en documentexport, plus een uitspraakwoordenboek op iOS. Op iOS verloopt de export via het deelblad, zodat een boek naar een ander app of naar Bestanden kan, in een ander formaat of exact zoals het is.
* Opties voor tekengrootte, regelafstand en contrast.
* Toetsenbordsneltoetsen die overeenkomen met de desktop-app.

### Versie 0.9.2
* Audioboeken zorgen er niet langer voor dat je schermlezer een reeks spaties hardop leest wanneer je de tekstinvoer focust.
* Audioboeken benoemen nu het bestand terwijl je er doorheen navigeert per sectie.
* Audioboeken melden nu hun werkelijke tijdsduur in plaats van voor elk bestand 24 uur te claimen.
* Het sluiten van de webweergave met Escape geeft geen debug-waarschuwing meer nadat je een link erin hebt gevolgd.
* Kopiëren na Alles selecteren geeft je nu het hele document, in plaats van alleen het momenteel geladen deel.
* Zoeken gaat nu rechtstreeks naar de regel waar het iets vond, in plaats van je door te laten worstelen terwijl de schermlezer het venster opnieuw hardop leest wanneer de focus terugkeert naar het boek.
* EPUB-bestanden met een stray ZIP64-blok die weigerden te openen met "Invalid local file header" zijn gerepareerd.
* Lange documenten die terugliepen naar het begin terwijl een schermlezer continu doorheen las zijn gerepareerd.
* Links in de webweergave brengen je nu naar de sectie waar ze naar verwijzen, in plaats van te mislukken met "Bestand niet gevonden".
* De automatische "Document herladen"-mededeling onderbreekt je schermlezer niet meer midden in een zin, maar wacht tot het klaar is met wat het zei.
* Het tabblad Algemeen van het dialoogvenster Instellingen navigeert nu door zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie updates controleren.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordenaantal en Documentinformatie tonen nu hoeveel bestanden een audioboek bevat en hoe lang het totaal duurt.

### Versie 0.9.1
* Geluid van bladwijzers en notities speelt nu af op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van hun tijdlijn stilzwijgend te openen en bij te werken.
* Gekrulde aanhalingstekens, em-streepjes en soortgelijke tekens die uit RTF-documenten verdwenen en de omliggende woorden samenvoeging zijn gerepareerd.
* RTF-afbeeldingen die hun ruwe gegevens als versleuterde tekst in het document lieten lekken zijn gerepareerd.
* Het submenu Recente documenten dat verouderde items behield tot iets anders het heropbouwde is gerepareerd.
* Toetsenbordversnellers zijn terug in elke vertaling, dus Russische menu's hebben opnieuw toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten worden nu bij Windows geregistreerd, zodat ze in de taakbalkjumplist en de recente lijst van het Start-menu verschijnen.
* Opties is hernoemd naar Instellingen, wat aansluit bij de mobiele apps en op macOS de platformconventie.
* Paperback onthoudt nu de raamposition, grootte en gemaximaliseerde staat tussen runs.
* Meervoudsvormen zijn nu vertaald, zodat berichten die dingen tellen correct in talen met meer dan één vorm lezen.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in het dialoogvenster Toetsenbordsneltoetsen aanpassen kunnen nu vertaald worden.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken in de taakbalk en Alt+Tab uit elkaar kunnen worden gehouden.
* Het updatedialoogvenster is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een terminalprogramma, genaamd pb, om snel documenten in een van Paperback's ondersteunde indelingen naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, handig voor bijvoorbeeld het bewerken van Markdown.
* Documenttekst is nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden nu in slechts een paar seconden kunt laden. Meld alstublieft eventuele vreemdheden hiermee.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een volledig scherm in-/uitschakelen.

##### Dialoogvenster Alle documenten
* Een knop Terugvinden om ontbrekende boeken te lokaliseren die hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat je op documentstatus kunt filteren en ziet hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten af te selecteren.

##### Opties en leesbaarheid
* Een tabblad Leesbaarheid, met de volgende opties:
    * Automatische terugloop (verplaatst uit algemeen);
    * Tabellen in de tekst weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menuitem Automatische terugloop en daaropvolgende venstersneltoets.
* Een wisselaar om te bepalen hoe je tabellen wilt weergegeven, en hoe tabellen over documenten heen uniform worden weergegeven.

##### Navigatie
* Ondersteuning voor navigatie op container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met bladermodus in schermlezers.
* De sneltoets Gelijkteken om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en ze blijven bestaan. Gebruik schuine streep om er een in te stellen en backslash om ernaar te springen.

##### Woordenaantal
* Geschatte leestijd in het dialoogvenster Woordenaantal, evenals de mogelijkheid om je leessnelheid in te stellen om deze metriek daadwerkelijk bruikbaar te maken.
* Als een selectie actief is wanneer je het dialoogvenster Woordenaantal opent, wordt het aantal woorden dat je hebt geselecteerd nu weergegeven.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordsneltoets om Paperback uit het systeemvak terug te zetten.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menuitem Exporteren uitgebreid om naar HTML en Markdown te exporteren, naast platte tekst.

##### Updater
* Een knop Annuleren in het dialoogvenster Update wordt uitgevoerd.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audio afspelen.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunend DAISY audio (inclusief DAISY audio + tekst) en ZIP-bestanden met audiobestanden.
* Toetsenbordsneltoetsen en menu-items om audio af te spelen/pauzeren, vooruit en achteruit te spoelen en de audiospoelstap aan te passen.
* Opties om de leescursor met audio afspelen te synchroniseren, de audiospoelstap in te stellen en te kiezen of spoelen voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als onleesbare tekens.
* "Laatst gesloten heropenen" probeert niet langer de meegeleverde handleiding opnieuw te openen.
* Je geselecteerde tabblad krijgt na het herstarten van Paperback nu goed focus.
* Paperbacks verwerking van bestanden op Windows-netwerkschijven: als je nu de bestandslocatie opent, krijgt het bestand op de netwerkopslag goed focus, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer forceer geladen bij documentherstel; in plaats daarvan krijg je een bevestigingsverzoek als er een wordt gevonden.
* Map openen geeft nu de gegeven bestand focus in de verkenner.
* Het openen van de handleiding respecteert nu je geselecteerde taal.
* De gebruikersinterface van Paperback schaalt nu correct op high-DPI-schermen.
* Het menu werkt nu correct bij, en focus gaat naar het tekstbesturingselement wanneer je hulp opent in Paperback.
* Overgestapt naar een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het schakelen tussen tabbladen.
* Verminderd geheugengebruik bij grote documenten door de omvang van de interne per-teken-indextabellen te halveren.

##### Dialoogvenster Alle documenten
* Escape sluit niet de dialoogvensters Documentinformatie en Alle documenten.
* De titelbalk werkt niet bij na het sluiten van een document uit het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan je lijst Alle documenten toegevoegd wanneer deze via Shift+F1 wordt geopend.
* Het verwijderen van documenten uit het dialoogvenster Recent wordt nu ook hun actieve tabblad gesloten.
* Je zoekfilter blijft nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie meldt in sommige situaties onjuiste regeltekst.
* "Ga naar regel", "Ga naar pagina" en "Ga naar procent" plaatsen je cursor op de verkeerde positie in grote documenten.
* "Zoeken" en "Volgende zoeken" respecteren het geladen documentvenster in grote documenten niet.

##### Bladwijzers
* Bladwijzer-/notitiegeluiden worden nu correct uitsluitend afgespeeld wanneer je over een woord navigeert dat er een bevat.

##### Leesbaarheid
* "Automatische terugloop" inschakelen stuurt je naar het begin van je document.

##### Webweergave
* Het webweergave-dialoogvenster kan niet worden aangepast en verschijnt met een erg klein initieel formaat.
* Afbeeldingen worden nu correct weergegeven in de ingebedde webweergave.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten verwerken met niet-Latijnse tekens erin.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en voegen rommel in de boektekst in.
* Links in verouderde Mobi-boeken.
* Sterk verbeterd AZW3-verwerking.

##### Word-documenten
* Word-documenten met landinstellingsspecifieke stijlnamen geven hun koppen niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen produceren geen regelafbrekingen in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op normale tekstextractie voor onterecht gemarkeerde PDF's.
* PDF-documenten met besturingselementen in hun titels en/of bladwijzers laten Paperback niet langer crashen bij het openen.

### Versie 0.8.5
* Paginaondersteuning aan epub-boeken toegevoegd.
* Ondersteuning voor versleutelde Microsoft Office-documenten toegevoegd. Momenteel worden klassieke Word, moderne Word en moderne Powerpoint ondersteund; klassieke Powerpoint staat gepland voor de toekomst.
* Ondersteuning voor klassieke Microsoft Word-documenten toegevoegd.
* Ondersteuning voor klassieke Powerpoint-presentaties toegevoegd.
* Ondersteuning voor mobi- en AZW3-boeken toegevoegd.
* Ondersteuning voor getagde PDF-bestanden toegevoegd.
* Sneltoets `Ctrl+Q` om de app te sluiten toegevoegd.
* Ondersteuning voor gecomprimeerde boeken van Bookshare (zowel DAISY als Word) toegevoegd.
* Alt-tekst voor ingebedde afbeeldingen wordt nu correct weergegeven.
* CHM-documenten ondersteunen nu goed interne linknavigatie.
* "Ga naar pagina" corrigeerd zodat het niet meer 1 afwijkt.
* Escape-toets werkt nu om het dialoogvenster "Openen als" te sluiten.
* Het contextmenu van de lezer verschijnt nu bij rechtsklikken of de toepassingstoets.
* Het juiste document krijgt focus wanneer documenten van de opdrachtregel worden geopend.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen je van hun bestaan.
* Het is nu mogelijk door afbeeldingen en figuren te navigeren met g/Shift+G en f/Shift+F.
* Paperback respecteert nu je instellingen voor donkere modus van de applicatie.
* DAISY XML-ondersteuning verwijderd, omdat deze niet langer nodig is.
* Overgestapt naar de ingebouwde Win32-navigatie met eerste letter in de inhoudsopgaveboom.
* Het foutladings-dialoogvenster toont nu meer gedetailleerde foutmeldingen.
* De webweergave opent nu veel sneller en soepeler.

### Versie 0.8.2
* Paginaondersteuning aan RTF-documenten toegevoegd.
* Bug opgelost waarbij het openen van de webweergave in epubs met externe links deze automatisch zou activeren.
* Bug opgelost waarbij de RTF-verwerker in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's worden niet langer opgesplitst in meerdere korte regels in sommige PDF-documenten.
* PDF-documenten ondersteunen nu basisnavigatie voor links en koppen.
* RTF-tabs en regeleindes worden nu exact weergegeven zoals ze in het document voorkomen.
* Teruggegaan naar de beproefd werkende pdfium-bibliotheek voor PDF-verwerking, waardoor PDF-rendering veel betrouwbaarder wordt.

### Versie 0.8.1
* `Ctrl+Shift+T` toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Enkele bugs in de RTF-verwerker opgelost.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) worden niet langer beschadigd wanneer een bestand wordt geopend via een tweede Paperback-exemplaar.
* PDF-tekst wordt niet langer in de verkeerde volgorde gelezen, en onjuiste spatiëring rond gekapitaliseerde woorden opgelost.
* Traag laden van documenten bij het openen van grote bestanden opgelost.
* Lokalisatie van de Ja-/Nee-knoppen in bevestigingsdialoogvensters opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsweergave toegevoegd voor het bereiken van een bladwijzer of notitie, dank Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor Flat Open Document Text-bestanden toegevoegd!
* Ondersteuning voor Flat Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingen met `s` en `shift+s` toegevoegd.
* Elke beweging van meer dan 300 tekens voegt nu automatisch aan je navigatiegeschiedenis toe.
* Het herstellen van Paperback's venster vanuit het systeemvak opgelost.
* Markdown-documenten die onbewerkte tekst weergaven in plaats van weergegeven HTML in de webweergave opgelost.
* Tabellen die niet correct in Markdown-bestanden werden weergegeven opgelost.
* Alleen-afbeeldings-PDF's geven je nu een waarschuwing wanneer je probeert er een te laden.
* Versie-informatie correct in het Paperback-uitvoerbare bestand ingebed.
* De opties-dialoog in tabbladen verdeeld voor gemakkelijker gebruik en navigatie.
* Over gegaan naar Hayro voor PDF-verwerking, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app in Rust herschreven. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML- en XHTML-gebaseerde documenten toegevoegd! Navigeer tussen tabellen met `T` en `shift+t`, en druk `Enter` om er een in een webweergave te bekijken.
* Een basale webweergavefunctie toegevoegd! Druk `ctrl+shift+v` om de huidige sectie van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan de dialoog Alle documenten.
* De updatecontrole geeft nu release notes weer wanneer een nieuwe versie beschikbaar is.
* Het herstellen van het venster vanuit het systeemvak opgelost.
* Vertaling van Ja/Nee-knoppen in bevestigingsdialogen opgelost.
* Configuraties laden bij het uitvoeren als beheerder opgelost.
* Commentaarverwerking in XML- en HTML-documenten opgelost.
* TOC-verwerking in Epub 2-boeken opgelost.
* Navigeren naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* De zoekdialoog niet correct verbergen bij gebruik van de volgende/vorige knoppen opgelost.
* Epub-inhoudsopgave gooit je af en toe naar het verkeerde item opgelost.
* Verschillende problemen met witruimteverwerking in XML-, HTML- en pre-tags opgelost.
* Off-by-one fout in linknavigatie opgelost.
* Sommige boeken met overbodige witruimte aan het einde van regels opgelost.
* Verschillende verwerkingsproblemen opgelost.
* Bladwijzer-gerelateerde menu-items en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document geopend is.
* Lijstverwerking in verschillende documentformaten verbeterd.
* De vertalingsworkflow voor bijdragers verbeterd.
* Veel interne refactors, waarbij het merendeel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een heel basale functie voor navigeren naar vorige/volgende positie toegevoegd. Als je `Enter` op een interne link drukt en de cursor beweegt, zal die positie nu worden onthouden en kan ermee worden genavigeerd met `alt+left`/`alt+right`.
* Een elementenlijst toegevoegd! Deze toont momenteel alleen een structuur van alle koppen in je document of een lijst met links, maar er zijn plannen om deze in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Links in sommige Epub-documenten die niet goed werkten opgelost.
* Verwerking van Epub-inhoudsopgaven met relatieve paden opgelost.
* Sommige Epub-documenten die geen titel of auteur weergaven opgelost.
* De titels van sommige Epub-hoofdstukken die niet goed in de inhoudsopgave-dialoog werden weergegeven opgelost.
* Je kon de spatiebalk niet gebruiken om de OK/annuleer-knoppen in de inhoudsopgave-dialoog te activeren opgelost.
* De verwerking van koppen in Word-documenten verbeterd.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je probeert de dialoog te openen.

### Versie 0.6.0
* Een nieuwe optie om het Ga-menu in een veel compactere vorm weer te geven is aan het instellingendialoogvenster toegevoegd, standaard ingeschakeld.
* Een optie toegevoegd om navigatie op basis van structurele elementen met terugloop in te schakelen.
* Een optie aan het Extra-menu toegevoegd om de map met het momenteel actieve document te openen.
* Een vrij eenvoudig, maar zeer effectief, updatersysteem toegevoegd.
* Een basale slaaptimer-functie toegevoegd, toegankelijk via `Ctrl+Shift+S`.
* Ondersteuning voor het verwerken van FB2-ebooks toegevoegd!
* Ondersteuning voor het verwerken van OpenDocument-presentaties toegevoegd!
* Ondersteuning voor het verwerken van OpenDocument Text-bestanden toegevoegd!
* Bladwijzers kunnen nu worden gebruikt om een hele regel aan te duiden, of alleen bepaalde tekst. Als je geen selectie actief hebt wanneer je een bladwijzer plaatst, werkt het zoals vóór 0.6 en markeert het de hele regel. Als je echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities hebben! Navigeer tussen bladwijzers met notities met N en `Shift+N`, of open het bladwijzerdialoogvenster met alle bladwijzers, alleen notities of alleen niet-notities ingeschakeld met specifieke venstersneltoetsen.
* Bladwijzers in het bladwijzerdialoogvenster hebben niet langer een vervelend "bladwijzer x"-voorvoegsel.
* EPUB-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Laden van grote Markdown-documenten gerepareerd.
* Het indrukken van de spatiebalk in de inhoudsopgave-boomweergave die de OK-knop activeert, gerepareerd.
* Witruimteverwerking aan het begin van pre-tags in zowel HTML- als XHTML-documenten gerepareerd.
* De tekstbesturing die soms niet opnieuw focus kreeg bij terugkeer naar het Paperback-venster, gerepareerd.
* Het tekstveld in het dialoogvenster voor gaan naar procent dat de schuifregelaarwaarde niet bijwerkte, gerepareerd.
* De weergave van aangepaste HTML-ID's in Markdown-documenten gerepareerd.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek met een terminalparameter laadt terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijg je niet langer een fout als het laden van je document meer dan 5 seconden duurt.
* Als je Paperback als beheerder uitvoert, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer direct uit het bladwijzerdialoogvenster te verwijderen.
* Het is nu mogelijk om je bladwijzers en leeespositie voor een bepaald document in te voeren en uit te voeren. Het gegenereerde bestand krijgt de naam van het bestand met een .paperback-extensie. Als zo'n bestand in dezelfde map als een bestand wordt gevonden terwijl dit wordt geladen, wordt het automatisch geladen. Anders kun je ze handmatig importeren met behulp van een item in het Extra-menu.
* Links in documenten worden nu volledig ondersteund! Gebruik k en `Shift+K` om vooruit en achteruit door deze te gaan, en druk op Enter om een link te openen/activeren.
* Veel interne refactorings, waardoor de app sneller wordt en het binaire bestand kleiner.
* Markdown-inhoud wordt nu vooraf verwerkt om CommonMark-compatibel te zijn vóór weergave.
* Navigatie op basis van lijsten en hun items wordt nu volledig ondersteund! Gebruik L en `Shift+L` om naar lijsten zelf te gaan, en I en `Shift+I` om door lijstitems te gaan.
* Numpad-Delete werkt nu ook om documenten uit de tabbalk te verwijderen, naast normale Delete.
* Paperback kan nu optioneel naar je systeemvak minimaliseren! Deze optie is standaard uitgeschakeld, maar als je deze inschakelt, worden documenten in het systeemmenu geminimaliseerd in je vak, en kunnen ze worden hersteld door op het gegenereerde pictogram te klikken.
* Paperback kan nu volledig worden vertaald! De lijst met ondersteunde talen is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten geven nu een basisinhoudsopgave weer met daarin alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinformatie.
* Het installatieprogramma bevat nu een optie om de handleiding in je browser weer te geven na de installatie.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van alleen de laatste 10 documenten die je hebt geopend weer te geven, toont het nu een aanpasbaar aantal, met de rest van de documenten die je ooit hebt geopend, toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen voor alle parsers, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het repareren van de newline-verwerking in alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden nu correct uitgeschakeld als er geen documenten open zijn.
* Oriëntatie van de ga naar procent-schuifregelaar gerepareerd.
* Inhoudsopgave in EPUB-boeken met URL-gecodeerde bestandspaden en/of fragmentaanduidingen gerepareerd.
* Witruimte in XHTML-koppen op vreemde manieren verwijderd gerepareerd.
* Witruimteafhandeling in geneste pre-tags in HTML-documenten gerepareerd.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Als je een HTML-/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave uit de structuur van de koppen in je document, en toont die in de `Ctrl+T`-dialoog.
* HTML-documenten hebben nu de titel zoals ingesteld in de title-tag, indien aanwezig. Anders gebruiken ze de bestandsnaam zonder extensie.
* Van UniversalSpeech overgeschakeld naar het gebruik van een livegebied voor spraakrapportage. Dit betekent dat er geen schermlezer-DLL's meer bij het programma worden meegeleverd, en meer schermlezers worden nu ondersteund, zoals Microsoft Narrator.
* Zipbibliotheken overgeschakeld om een breder scala aan EPUB-boeken te kunnen openen.
* De dialoog waarin je wordt gevraagd of je je document als platte tekst wilt openen is volledig opnieuw gemaakt, en biedt nu de mogelijkheid om je document als platte tekst, HTML of Markdown te openen.
* De ga naar procent-dialoog bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-verwerker herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in EPUB-boeken wordt nu weer exact behouden.
* De Unicode-onbreekbare spatie wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* Je wordt niet langer elke keer gevraagd hoe je een niet-herkend bestand wilt openen, maar alleen de eerste keer.

### Versie 0.4.1
* Optioneel pictogram in het startmenu toegevoegd aan het installatieprogramma.
* De inhoudsopgave zou nu in enkele gevallen schoner moeten zijn, bijvoorbeeld als je een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* Inhoudsopgave in bepaalde CHM-documenten gerepareerd.
* Inhoudsopgave in EPUB 3-boeken met absolute paden gerepareerd.
* CHM-documenten moeten nu hun titel weergeven zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* CHM-bestandsondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers hebben als je wilt in zoveel documenten als je wilt. Je kunt er doorheen gaan met `B` en `Shift+B`, er een instellen met `Ctrl+Shift+B`, en een dialoog openen om naar een specifieke bladwijzer te springen met `Ctrl+B`.
* Een installatieprogramma toegevoegd naast het draagbare zipbestand! Het installatieprogramma installeert Paperback in je Program Files-map en stelt automatisch bestandskoppelingen voor je in.
* Tekstbestanden met BOM's worden nu correct gedecodeerd, en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Deze toont nu je huidige regel, teken en leesvorderingspercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer weergegeven in tekstuitvoer.
* Als je een relatief pad aan Paperback op de opdrachtregel doorgeeft, wordt dit nu correct omgezet.
* Percentagebewegingen worden nu afgehandeld door hun eigen schuifregelaar-gebaseerde dialoog, toegankelijk met `Ctrl+Shift+G`.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor het opslaan van positie is nu veel intelligenter en schrijft alleen naar schijf wanneer absoluut nodig.
* Het document waar je op gefocust was toen je Paperback sloot, wordt nu onthouden voor volgende toepassingsstarts.
* Invoer in de ga naar regel- en ga naar pagina-dialogen moet nu strenger worden opgeschoond.
* Inhoudsopgavenavigatie in EPUB 3-boeken met relatieve paden in hun manifesten gerepareerd.

### Versie 0.3.0
* Inhoudsopgave in EPUB-boeken met URL-gecodeerde manifesten gerepareerd.
* Kopnavigatie in HTML-documenten met multibyte Unicode-tekens gerepareerd.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets gerepareerd.
* Laden van UTF-8-tekstbestanden gerepareerd.
* Geneste inhoudsopgaveitems in EPUB-boeken die je cursor op de verkeerde positie plaatsen, gerepareerd.
* Crash bij toepassingsafsluiting in bepaalde gevallen gerepareerd.
* Selectievakje in de optiesdialoog toegevoegd om automatische terugloop in- of uit te schakelen!
* Het is nu mogelijk om te doneren aan de ontwikkeling van Paperback, via het nieuwe donatie-item in het Help-menu of via de sponsor this project-link onderin de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* PDF-bibliotheken overgeschakeld naar de bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaardere PDF-verwerking in het algemeen.
* Je kunt nu maar één exemplaar van Paperback tegelijk uitvoeren. `paperback.exe` uitvoeren met een bestandsnaam terwijl het al wordt uitgevoerd, opent dat document in het reeds actieve exemplaar.
* Je kunt nu op Verwijderen drukken op een document in het tabbladbesturingselement om het te sluiten.

### Versie 0.2.1
* Totaal aantal pagina's toegevoegd aan het paginalabel in de ga naar pagina-dialoog.
* Tabben van documentinhoud naar je lijst geopende documenten toegestaan.
* Kopsneltoetsen die soms recente documenten openen als je er genoeg hebt, gerepareerd.
* Paperback verwijdert nu onnodige zware koppeltekens uit tekstuitvoer.
* Kopnavigatie die je soms op het verkeerde teken plaatst, gerepareerd.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsenbordsneltoetsen toegevoegd voor navigatie via koppen in HTML-inhoud, inclusief EPUB-boeken en markdown-documenten. Deze toetsenbordsneltoetsen zijn ontworpen om op dezelfde manier te werken als een schermlezer.
* Laden van EPUB's met URL-gecodeerde bestandsnamen in hun manifesten is opgelost.
* Laden van EPUB 3-boeken met ingesloten XHTML is opgelost.
* Er wordt nu een bericht uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items zijn uitgeschakeld.
* Menu met recente documenten toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op, en je kunt op een ervan drukken om het te openen voor lezen.
* De zoekdialoog volledig herschreven, waardoor het veel eenvoudiger is om te gebruiken, en ondersteuning voor een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies toegevoegd!
* Eerder geopende documenten worden nu onthouden na het herstarten van de applicatie. Dit kan worden ingesteld via het nieuwe opties-item in het menu Extra.
* `Shift+F1` toegevoegd om de handleiding rechtstreeks in Paperback te openen.

### Versie 0.1.0
* Initiële release.
