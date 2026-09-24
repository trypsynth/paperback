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

## Schermlezercompatibiliteit

Paperback werkt goed met alle belangrijke schermlezers. Er zijn echter twee bekende problemen voor JAWS-gebruikers.

### JAWS en brailleleesregels

Als je JAWS met een brailleleesregel gebruikt, kan het zijn dat lange alinea's worden afgekapt wanneer je met de navigatietoetsen van je leesregel vooruit scrolt. Het commando om de huidige alinea te lezen heeft er ook last van. Dit is een bug in de manier waarop JAWS omgaat met het tekstbesturingselement RICHEDIT50W, niet iets in Paperback zelf, en een bug waarvoor het behoorlijk lang heeft geduurd voordat er een oplossing boven water kwam, gezien het enthousiasme waarmee Vispero reageert op problemen met opensourcesoftware.

De workaround, die na maanden wachten uiteindelijk via de JAWS-discussiegroep boven water kwam, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" op "Always use DOM if available" te zetten. Schakel ook "Pan Text by Paragraph" in, anders blijft je leesregel op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen zou scrollen correct moeten werken.

### JAWS en de meldingen van Paperback

Paperback meldt dingen als "Geen pagina's." of "Dit document heeft geen audio." via toegankelijkheidsmeldingen. Daardoor kan een schermlezer ze uitspreken, wat hij op dat moment ook aan het zeggen is. JAWS reageert daar alleen op als "Enable accessible notification events" voor de applicatie is ingeschakeld, en op sommige computers is dat niet zo.

Als JAWS niets zegt wanneer je op een toets drukt die iets zou moeten melden, open dan Settings Center met Paperback op de voorgrond (`Insert+6`), zoek naar "notification" en vink "Enable accessible notification events" aan. Daarmee komt de instelling in `paperback.jcf` te staan, zodat die alleen voor Paperback geldt.

## Momenteel ondersteunde bestandstypen

Paperback ondersteunt de volgende indelingen en extensies:

* Comicarchieven (`.cbz`)
* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-e-books (`.fb2`)
* HTML-documenten (`.htm`, `.html`, `.xhtml`)
* Manual pages, zowel `man` als BSD `mdoc` (`.1` t/m `.9`, `.man`, `.roff` en de met gzip gecomprimeerde vormen daarvan)
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
* Plattetekst- en logbestanden (`.txt`, `.log`)

## Sneltoetsen

Paperback is ontworpen voor toetsenbordgericht gebruik. Hieronder staan de huidige sneltoetsen.

De sneltoetsen hieronder gelden voor Windows. Waar macOS afwijkt, staat het equivalent tussen haakjes — vooral omdat Ctrl+G, Ctrl+W en Alt+Links/Rechts op dat platform al bezet zijn door andere systeem- of appconventies.

### Menu Bestand

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document heropenen.
* `Ctrl+R`: Het venster "Alle documenten" tonen (vanuit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in het appmenu).

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
* `Ctrl+,`: Instellingen openen (macOS: in het appmenu).
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

## Versiegeschiedenis

### Versie 1.0

1.0 is de eerste release op alle vijf de platforms: Windows, macOS, Linux, iOS en Android, met de iOS- en Android-apps in de App Store en op Google Play.

#### Toegevoegd

##### Algemeen
* Linux-ondersteuning, als AppImage of tar.gz, met desktopintegratie zodat documenten vanuit je bestandsbeheerder worden geopend.
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles van daar tot waar je bent uitgekomen met `Alt+F10` en ga terug naar de markering met `Alt+Shift+F9`, om een lang stuk tekst te kopiëren zonder er met shift+pijltjes doorheen te hoeven gaan. Alle drie staan onder Extra > Selecteren en kopiëren.
* De sneltoets `=` meldt nu naast het percentage ook de pagina, bijvoorbeeld "15%, pagina 30", en blijft zoals het was bij documenten zonder paginanummers.
* Het venster Over toont nu de licentie van Paperback en alle vertalers.
* Een Oekraïense vertaling.

##### Nieuwe indelingen
* Comicarchieven (`.cbz`).
* M4B-audioboeken, opgesplitst in hun hoofdstukken.
* Manual pages, zowel `man` als BSD `mdoc`, al dan niet met gzip gecomprimeerd.
* MP3-audioboeken, opgesplitst in hoofdstukken als het bestand die heeft.
* reStructuredText-documenten.
* Windows Write-bestanden (`.wri`).
* WinHelp-bestanden (`.hlp`).
* Word 6- en Word 95-documenten.

##### OCR
* Gescande PDF-pagina's kunnen nu worden herkend met de OCR die in Windows en macOS is ingebouwd. Druk op `Enter` op een gescande pagina om die te herkennen, of gebruik Bulk-OCR (`Ctrl+Shift+O`) voor een reeks pagina's.

##### Navigatie
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met behulp van MathCAT. Gebruik `M` of `Shift+M` om door formules te navigeren, en daarna `Enter` of `Spatie` om de oorspronkelijke MathML te openen in de formuleweergave.
* Een knop Alles zoeken in het zoekvenster, die elke regel met een overeenkomst opsomt zodat je direct naar de gewenste kunt springen.
* De weergaven Tabellen, Lijsten en Pagina's in de elementenlijst (`F7`).
* Ga naar regel, Ga naar pagina en Ga naar percentage accepteren nu `+n` en `-n` om relatief ten opzichte van je huidige positie te navigeren.
* EPUB-, MOBI- en CHM-boeken zonder eigen koppen krijgen nu kopnavigatie op basis van hun inhoudsopgave.
* KF8-boeken (AZW3) ondersteunen nu sectienavigatie.
* EPUB-pagina's die alleen uit een afbeelding bestaan, tonen daar nu een regel voor, zodat je erop kunt landen in plaats van er direct voorbij te gaan.

##### Audioboeken
* Bediening van de afspeelsnelheid, van halve snelheid tot drie keer zo snel. Gebruik `Ctrl+Shift+.` en `Ctrl+Shift+,`, of het menu Extra.
* Bladwijzers en notities in boeken met alleen audio onthouden nu het exacte tijdstip waarop je ze hebt gezet.
* Volgende en vorige positie (`Alt+Links` en `Alt+Rechts`) werken nu in audioboeken.
* De voortgang in een audioboek wordt nu gemeten aan de hand van de opname, zodat Ga naar percentage en de statusbalk overeenkomen met hoe ver je werkelijk bent.

##### Recente documenten
* Een item Recente documenten wissen in het submenu Recente documenten.

##### PDF-documenten
* Een instelling om elke regel van een PDF apart te houden, in plaats van ze samen te voegen tot alinea's.
* Afbeeldingen en figuren in PDF's worden nu gemeld.
* PDF's die wel leesstructuur bevatten maar geen van hun afbeeldingen taggen, melden die afbeeldingen nu, in plaats van ze helemaal uit het boek weg te laten.

##### Webweergave
* Elk document kan nu in de webweergave worden geopend, niet alleen EPUB, HTML en Markdown.

##### Leesbaarheid
* Koppen worden nu weergegeven in een grootte die past bij hun niveau, en afbeeldingen en tabellen staan los van de tekst eromheen.

##### pb
* `pb --list-formats` toont elke indeling die pb kan lezen.
* pb meldt nu welk bestand het niet kon lezen, en waarom.

#### Opgelost

##### Algemeen
* Een boek dat bij het opstarten wordt heropend, is nu meteen te lezen, in plaats van stil te blijven tot het gesloten en opnieuw geopend werd.
* Een document waarvan het bestand ontbreekt, kan nu uit Alle documenten worden verwijderd, in plaats van in de lijst te blijven staan hoe vaak je ook bevestigt.
* Een crash bij het afsluiten van Paperback opgelost.
* Paperback verbergt het venster bij het afsluiten nu meteen, in plaats van het op het scherm te laten staan terwijl het opslaat.
* Grote boeken met weinig opmaak worden nu in ongeveer de helft van de tijd geopend.
* Meldingen na het kiezen van een menu-item, zoals "Dit document heeft geen audio", worden niet langer door de schermlezer afgebroken voordat je ze hoort.
* Na het openen van een document blijft 'Laatst gesloten heropenen' niet langer ingeschakeld wanneer er niets te heropenen valt.
* Paperback blijft niet langer documenten uit je lijst met recente documenten proberen die verdwenen zijn, en beperkt hoeveel recente documenten het opslaat.
* Het oude INI-instellingenbestand wordt nu verwijderd zodra het naar de nieuwe indeling is overgezet.
* De titels van de vensters voor lettertype en kleur, en het menu Exporteren als in het Vietnamees, zijn nu vertaald.
* Na het bijwerken komt het opnieuw gestarte venster nu naar voren, in plaats van achter alle andere vensters in Alt+Tab te blijven staan.
* Automatische terugloop wordt nu direct toegepast op grote documenten, in plaats van het hele document opnieuw te laden.

##### Navigatie
* `Alt+Links` gaat nu terug naar de plek waar je vandaan sprong, in plaats van naar een oudere positie.
* Bladwijzergeluiden worden nu alleen afgespeeld wanneer je over een bladwijzer heen gaat, niet wanneer je op de regel landt waarop die staat.
* Bij het sluiten van de inhoudsopgave, de elementenlijst en de Ga naar-vensters kom je nu direct op de gekozen regel terecht, in plaats van te moeten wachten tot de schermlezer het venster opnieuw heeft voorgelezen.
* Ga naar regel, Ga naar pagina en Ga naar percentage weigeren nu getallen buiten het document, in plaats van stilletjes ergens anders heen te gaan.
* Wanneer een document geen pagina's heeft, kapt NVDA de melding niet meer af.
* Op OK drukken in de inhoudsopgave zonder te navigeren gaat nu naar het item dat al geselecteerd was.
* De inhoudsopgave, de elementenlijst en de bladwijzerlijst haperen of bevriezen niet meer bij boeken met duizenden items.
* Pijl omhoog en pijl omlaag onthouden hun kolom nu per document, in plaats van die mee te nemen wanneer je van tabblad wisselt.

##### Audioboeken
* Voor het afspelen van audio wordt op macOS nu `Control+Spatie` gebruikt, omdat `Command+Spatie` van Spotlight is.

##### PDF-documenten
* Opgelost dat PDF's die uit Apple Pages zijn geëxporteerd als platte tekst werden gelezen, zonder de koppen en lijsten waarmee ze zijn geschreven.
* Opgelost dat PDF-alinea's en -koppen bij elke regel werden opgesplitst en woorden bij spaties uit elkaar vielen.
* Opgelost dat genummerde PDF-koppen tot één kop werden samengevoegd.
* Opgelost dat PDF's waarvan de structuur naar geen enkele tekst leidt, leeg werden geopend.
* Regels in een monospacelettertype, zoals code, worden niet langer samengevoegd tot alinea's.
* Kop- en voetteksten worden niet langer op elke pagina van PDF's zonder tags voorgelezen.
* PDF's die hun kop- en voetteksten als gewone tekst taggen, herhalen niet langer op elke pagina de titel en het paginanummer tussen twee alinea's.
* PDF's tonen nu hun echte titel in plaats van hun bestandsnaam.

##### MOBI/AZW3-boeken
* Grote MOBI-boeken lopen niet langer vast op een tekort aan geheugen en worden niet langer afgekapt na 20 MB.
* MOBI- en AZW3-boeken worden nu veel sneller geopend.
* Opgelost dat MOBI-boeken hun hoofdstuklijst kwijtraakten.
* Verminkte tekst opgelost op de plekken waar MOBI-boeken van het ene record naar het volgende overgaan.

##### Webweergave
* De webweergave laadt een enorm boek niet langer in één keer helemaal.
* De webweergave toont documenten nu in hun geheel wanneer de lezer ze in hun geheel toont, in plaats van slechts een deel ervan.

##### Overige indelingen
* FictionBook-boeken (.fb2) in windows-1251, en dat zijn de meeste, worden nu geopend in plaats van helemaal niet gelezen te kunnen worden.
* FictionBook-boeken die een namespace of een HTML-entiteit gebruiken die ze nooit hebben gedeclareerd, worden nu geopend in plaats van als beschadigd te worden geweigerd.
* Boeken in verouderde tekstcoderingen worden nu veel sneller geopend.
* Opgelost dat sommige Chinese tekstbestanden als verminkte tekst werden geopend.
* Met een wachtwoord beveiligde OpenDocument-bestanden vragen nu om hun wachtwoord, in plaats van als beschadigd te worden gemeld.
* Met een wachtwoord beveiligde klassieke PowerPoint-bestanden worden nu geopend, en klassieke PowerPoint-dia's raken hun tekst niet meer kwijt.
* Plattetekstbestanden die met de extensie `.rtf` zijn opgeslagen, worden nu als tekst geopend, in plaats van te mislukken met een foutmelding.
* RTF-opmaakcodes verschijnen niet langer als tekst.

#### iOS en Android

De iOS- en Android-apps openen elke indeling die de desktopversie ondersteunt, en bevatten:

* Voorlezen, met je eigen keuze van stem, snelheid en toonhoogte, een regelaar voor de spreeksnelheid direct op de leesbalk, en een optionele pauze tussen alinea's.
* Afspelen van DAISY-, M4B- en MP3-audioboeken, dat doorgaat op de achtergrond en vanaf het vergrendelscherm.
* Navigatie per kop, pagina, link, tabel, lijst en meer vanaf de leesbalk, plus de inhoudsopgave en Zoeken.
* Een slaaptimer, woordenaantal en documentexport, plus een uitspraakwoordenboek op iOS. Op iOS loopt exporteren via het deelmenu, zodat een boek naar een andere app of naar Bestanden kan, in een andere indeling of precies zoals het is.
* Opties voor tekstgrootte, afstand en tekst met hoog contrast.
* Sneltoetsen die overeenkomen met de desktopversie.

### Versie 0.9.2
* Audioboeken laten je schermlezer niet langer een reeks spaties voorlezen wanneer je het tekstveld focust.
* Audioboeken noemen nu de bestandsnaam wanneer je er per sectie doorheen gaat.
* Audioboeken melden nu hun werkelijke lengte, in plaats van te beweren dat elk bestand erin 24 uur duurt.
* Het sluiten van de webweergave met Escape geeft geen debugmelding meer nadat je er een link in hebt gevolgd.
* Kopiëren na Alles selecteren geeft je nu het hele document, in plaats van alleen het deel dat op dat moment geladen is.
* Zoeken gaat nu direct naar de gevonden regel, in plaats van je te laten wachten tot de schermlezer het venster opnieuw heeft voorgelezen wanneer de focus naar het boek terugkeert.
* Opgelost dat EPUB's met een verdwaald ZIP64-blok weigerden te openen met "Invalid local file header".
* Opgelost dat lange documenten naar het begin terugsprongen terwijl een schermlezer ze aan het voorlezen was.
* Links in de webweergave brengen je nu naar de sectie waarnaar ze verwijzen, in plaats van te mislukken met "File not found".
* De automatische melding "Document opnieuw geladen" onderbreekt je schermlezer niet langer midden in een zin, maar wacht tot die klaar is met wat hij aan het zeggen was.
* Het tabblad Algemeen van het venster Instellingen doorloopt zijn opties met Tab nu in de volgorde waarin ze op het scherm staan, met het updatekanaal direct na de optie voor het controleren op updates.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige omschrijving van het programma.
* Woordenaantal en Documentinformatie tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Versie 0.9.1
* Bladwijzer- en notitiegeluiden worden nu afgespeeld op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van te openen en hun tijdlijn in stilte bij te houden.
* Opgelost dat gekrulde aanhalingstekens, gedachtestreepjes en vergelijkbare tekens uit RTF-documenten verdwenen en daarbij de omringende woorden aan elkaar plakten.
* Opgelost dat RTF-afbeeldingen hun ruwe gegevens als verminkte tekst in het document lekten.
* Opgelost dat het submenu Recente documenten verouderde items behield totdat iets anders het toevallig opnieuw opbouwde.
* De sneltoetsletters zijn terug in elke vertaling, zodat de Russische menu's weer met het toetsenbord te bedienen zijn.
* Grote CHM-documenten worden nu tot zeven keer sneller geopend.
* Geopende documenten worden nu bij Windows geregistreerd, zodat ze verschijnen in de jumplist van de taakbalk en in de lijst met recente items van het menu Start.
* Opties is hernoemd naar Instellingen, in lijn met de mobiele apps en, op macOS, de platformconventie.
* Paperback onthoudt nu de positie, grootte en gemaximaliseerde staat van het venster tussen sessies.
* Meervoudsvormen worden nu vertaald, zodat berichten die dingen tellen goed leesbaar zijn in talen die meer dan één vorm nodig hebben.
* Het selecteren van de ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in het venster Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu vooraan in de titelbalk, zodat geopende boeken in de taakbalk en Alt+Tab uit elkaar te houden zijn.
* Het updatevenster is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een terminalprogramma, pb genaamd, om snel elke door Paperback ondersteunde indeling om te zetten naar HTML, Markdown of platte tekst.
* Een optie om documenten opnieuw te laden die door andere programma's op de schijf zijn gewijzigd.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, bijvoorbeeld handig voor het bewerken van Markdown.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden nu in slechts een paar seconden kunt laden. Meld het als je hierbij iets vreemds tegenkomt.

##### Platformondersteuning
* Ondersteuning voor Windows op ARM64!
* Native ondersteuning voor macOS!
* Een optie voor volledig scherm.

##### Venster Alle documenten
* Een knop Terugvinden voor ontbrekende boeken waarvan alleen het pad is gewijzigd.
* Een statusfilter en een statusbalk, zodat je op documentstatus kunt filteren en kunt zien hoeveel documenten er worden getoond en geselecteerd zijn.
* De sneltoets `Ctrl+Shift+A` om de selectie van alle documenten op te heffen.

##### Opties en leesbaarheid
* Een tabblad Leesbaarheid, met de volgende opties:
    * Automatische terugloop (verplaatst vanuit Algemeen);
    * Tabellen in de tekst weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Tekenafstand;
    * Tekstuitlijning.
* Een menu-item voor automatische terugloop met bijbehorende sneltoets.
* Een schakeloptie om te bepalen hoe je tabellen wilt laten weergeven, en de weergave van tabellen is gelijkgetrokken voor alle documenten.

##### Navigatie
* Ondersteuning voor navigeren per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij het navigeren tussen regels, vergelijkbaar met de bladermodus van schermlezers.
* De sneltoets = om je huidige percentage in een document te melden.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en ze blijven bewaard. Gebruik de schuine streep om er een in te stellen en de backslash om ernaartoe te springen.

##### Woordenaantal
* Geschatte leestijd in het woordenaantalvenster, plus de mogelijkheid om je leessnelheid in te stellen zodat dit getal echt nuttig is.
* Als er een selectie actief is wanneer je het woordenaantalvenster opent, wordt nu getoond hoeveel woorden je hebt geselecteerd.

##### Sneltoetsen
* De mogelijkheid om elke sneltoets in de applicatie aan te passen via een eenvoudig venster.
* Een instelbare sneltoets om Paperback vanuit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren is uitgebreid zodat je naast platte tekst ook naar HTML en Markdown kunt exporteren.

##### Updater
* Een knop Annuleren in het venster dat tijdens het bijwerken wordt getoond.
* De updater controleert nu of er niet met het gedownloade bestand is geknoeid.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor het afspelen van DAISY 2.02-audio.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als ZIP's met audiobestanden.
* Sneltoetsen en menu-items om de audio af te spelen/te pauzeren, vooruit en terug te spoelen en de spoelstap aan te passen.
* Opties om de leescursor te synchroniseren met het afspelen van de audio, de spoelstap in te stellen en te kiezen of spoelen voorbij het einde van een hoofdstuk verdergaat in het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een brij mojibake.
* Opgelost dat 'Laatst gesloten heropenen' de meegeleverde handleiding probeerde te heropenen.
* Opgelost dat je geselecteerde tabblad niet goed de focus kreeg na het opnieuw starten van Paperback.
* De omgang van Paperback met bestanden op Windows-netwerkschijven: 'Bestandslocatie openen' focust het bestand nu correct op de netwerkopslag, en de paden bevatten geen vreemde tekens meer.
* .paperback-bestanden worden bij het herstellen van documenten niet langer geforceerd geladen; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* Bovenliggende map openen focust nu het betreffende bestand in de Verkenner.
* Bij het openen van de handleiding wordt nu je gekozen taal gerespecteerd.
* De gebruikersinterface van Paperback wordt nu goed geschaald op schermen met hoge DPI.
* Het menu wordt nu correct bijgewerkt, en de focus gaat naar het tekstbesturingselement, bij het openen van de help in Paperback.
* Overgestapt op een veel veiligere IPC-methode op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het wisselen tussen tabbladen.
* Minder geheugengebruik bij grote documenten door de interne indextabellen per teken te halveren.

##### Venster Alle documenten
* Opgelost dat Escape de vensters Documentinformatie en Alle documenten niet sloot.
* Opgelost dat de titelbalk niet werd bijgewerkt na het sluiten van een document vanuit het venster Alle documenten.
* Readme.html wordt niet langer aan je lijst met alle documenten toegevoegd wanneer je die opent via Shift+F1.
* Het verwijderen van documenten uit het venster met recente documenten sluit nu ook hun actieve tabblad.
* Je zoekfilter blijft nu behouden na het verwijderen van een document.

##### Navigatie
* Opgelost dat paginanavigatie in sommige situaties de verkeerde regeltekst meldde.
* Opgelost dat Ga naar regel, Ga naar pagina en Ga naar percentage je cursor in grote documenten op de verkeerde positie plaatsten.
* Opgelost dat Zoeken en Volgende zoeken in grote documenten geen rekening hielden met het geladen documentvenster.

##### Bladwijzers
* Bladwijzer-/notitiegeluiden zouden nu alleen nog moeten worden afgespeeld wanneer je over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Opgelost dat het toepassen van automatische terugloop je naar het begin van je document schoot.

##### Webweergave
* Opgelost dat het webweergavevenster niet in grootte kon worden aangepast en heel klein opende.
* Afbeeldingen zouden nu correct moeten worden weergegeven in de ingesloten webweergave.

##### Updater
* De updater toont nu correct de inhoud van Markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* Opgelost dat DAISY-boeken onjuiste informatie in de statusbalk toonden.
* Het laden van DAISY-boeken met onjuiste coderingsdeclaraties opgelost.

##### RTF-documenten
* Het verwerken van RTF-documenten met niet-Latijnse tekens opgelost.
* RTF `\pict`-groepen opgelost, zodat ingesloten afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Opgelost dat filepos-ankers in Mobi-boeken HTML-tags opsplitsten en rommel in de boektekst zetten.
* Links in klassieke Mobi-boeken opgelost.
* Het verwerken van AZW3 sterk verbeterd.

##### Word-documenten
* Opgelost dat Word-documenten met taalspecifieke stijlnamen hun koppen niet goed weergaven.

##### HTML/XHTML-documenten
* Opgelost dat dl-, dt- en dd-elementen geen regeleinden opleverden in XHTML-documenten.

##### PDF-documenten
* Paperback valt bij PDF's met onjuiste tags nu terug op het uitlezen als platte tekst.
* PDF-documenten met besturingstekens in hun titel en/of bladwijzers laten Paperback niet langer crashen bij het openen.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan EPUB-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden klassiek Word, modern Word en modern PowerPoint ondersteund; klassiek PowerPoint is gepland voor de toekomst.
* Ondersteuning toegevoegd voor klassieke Microsoft Word-documenten!
* Ondersteuning toegevoegd voor klassieke PowerPoint-presentaties!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor PDF-bestanden met tags!
* De sneltoets Ctrl+Q toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor gecomprimeerde boeken van Bookshare (zowel DAISY als Word)!
* Alternatieve tekst voor ingesloten afbeeldingen wordt nu correct getoond.
* CHM-documenten ondersteunen nu correct de navigatie via interne links.
* Opgelost dat 'ga naar pagina' er 1 naast zat.
* Opgelost dat de Escape-toets niet werkte om het venster 'Openen als' te sluiten.
* Opgelost dat het contextmenu van de lezer niet verscheen bij rechtsklikken of met de toets Toepassingen.
* Opgelost dat soms het verkeerde document de focus kreeg bij het openen van documenten vanaf de terminal.
* PDF's die alleen uit afbeeldingen bestaan, worden weer gedetecteerd en je wordt op hun bestaan gewezen.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met respectievelijk g/shift+g en f/shift+f.
* Paperback respecteert nu je instelling voor de donkere modus van de applicatie.
* Ondersteuning voor DAISY XML verwijderd, omdat dit niet langer nodig is.
* Teruggeschakeld naar de native Win32-navigatie op eerste letter in de boomstructuur van de inhoudsopgave.
* Het foutvenster bij laden toont nu meer gedetailleerde foutmeldingen.
* De webweergave wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Een fout opgelost waarbij het openen van de webweergave in EPUB's met externe links deze automatisch activeerde.
* Een fout opgelost waarbij de RTF-verwerker in zeldzame gevallen geen spatie tussen woorden plaatste.
* Opgelost dat alinea's in sommige PDF-documenten werden opgesplitst in meerdere korte regels.
* PDF-documenten hebben nu basisondersteuning voor link- en kopnavigatie!
* RTF-tabs en regeleinden worden nu exact weergegeven zoals ze in het document voorkomen.
* Teruggeschakeld naar de beproefde pdfium-bibliotheek voor het verwerken van PDF's, waardoor PDF-weergave weer veel betrouwbaarder is.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document te heropenen.
* Het venster Alle documenten ondersteunt nu het selecteren van meerdere documenten om in één keer te openen.
* Enkele fouten in de RTF-verwerker opgelost.
* Opgelost dat bestandspaden met niet-ASCII-tekens (zoals het Bosnische š, č, ć, ž) beschadigd raakten bij het openen van een bestand via een tweede Paperback-instantie.
* Opgelost dat PDF-tekst in de verkeerde volgorde werd voorgelezen en dat er onjuiste spaties rond woorden met hoofdletters stonden.
* Opgelost dat het laden van documenten traag was bij het openen van grote bestanden.
* De lokalisatie van de knoppen Ja/Nee in bevestigingsvensters opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigd Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die je huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd bij het bereiken van een bladwijzer of notitie, met dank aan Andre Louis voor de geluiden!
* Ondersteuning voor RTF-documenten toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor Flat OpenDocument-tekstbestanden!
* Ondersteuning toegevoegd voor Flat OpenDocument-presentaties!
* Ondersteuning toegevoegd voor scheidingen met s en shift+s.
* Elke navigatie van meer dan 300 tekens wordt nu automatisch toegevoegd aan je navigatiegeschiedenis.
* Het herstellen van het venster van Paperback vanuit het systeemvak gerepareerd.
* Opgelost dat Markdown-documenten ruwe tekst toonden in plaats van weergegeven HTML in de webweergave.
* Opgelost dat tabellen niet goed werden weergegeven in Markdown-bestanden.
* PDF's die alleen uit afbeeldingen bestaan, waarschuwen je nu bij het laden over hun bestaan.
* Versie-informatie wordt nu correct ingesloten in het uitvoerbare bestand van Paperback.
* Het optievenster opgesplitst in tabbladen voor eenvoudig gebruik en navigatie.
* Overgestapt op Hayro voor het verwerken van PDF's, wat zorgt voor meer betrouwbaarheid, snelheid en minder DLL's.
* De volledige app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is eenvoudiger te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van algemene items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor op HTML en XHTML gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om er een in een webweergave te bekijken.
* Een eenvoudige webweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van je document te openen in een webgebaseerde renderer, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, met dank aan Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het venster Alle documenten.
* De updatecontrole toont nu releaseopmerkingen wanneer er een nieuwe versie beschikbaar is.
* Opgelost dat het venster werd hersteld vanuit het systeemvak.
* De vertalingen van de knoppen Ja/Nee in bevestigingsvensters opgelost.
* Het laden van configuraties bij uitvoeren als administrator opgelost.
* De verwerking van opmerkingen in XML- en HTML-documenten opgelost.
* Het verwerken van de inhoudsopgave in EPUB 2-boeken opgelost.
* Het navigeren naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* Opgelost dat het zoekvenster niet goed verborgen werd bij gebruik van de knoppen volgende/vorige.
* Opgelost dat EPUB-inhoudsopgaven je af en toe naar het verkeerde item brachten.
* Diverse problemen met de verwerking van witruimte in XML, HTML en pre-tags opgelost.
* Een off-by-onefout in de linknavigatie opgelost.
* Opgelost dat sommige boeken volgwitruimte op hun regels hadden.
* Diverse verwerkingsproblemen opgelost.
* Bladwijzergerelateerde menu-items en de elementenlijst worden nu correct uitgeschakeld wanneer er geen document open is.
* De verwerking van lijsten in diverse documentindelingen verbeterd.
* De vertaalworkflow voor bijdragers verbeterd.
* Veel interne refactors, waarbij het grootste deel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met een wachtwoord beveiligde PDF's toegevoegd!
* Een zeer eenvoudige functie 'ga naar vorige/volgende positie' toegevoegd. Als je op Enter drukt op een interne link en je cursor verplaatst wordt, wordt die positie nu onthouden en kun je er met alt+pijl links/rechts naartoe navigeren.
* Een elementenlijst toegevoegd! Momenteel toont deze alleen een boomstructuur van alle koppen in je document of een lijst met links, maar er zijn plannen om dit in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Opgelost dat links in sommige EPUB-documenten niet goed werkten.
* Het verwerken van EPUB-inhoudsopgaven met relatieve paden opgelost.
* Opgelost dat sommige EPUB-documenten geen titel of auteur toonden.
* Opgelost dat de titels van sommige EPUB-hoofdstukken niet goed verschenen in het inhoudsopgavevenster.
* Opgelost dat je de spatiebalk niet kon gebruiken om de knoppen OK/Annuleren in het inhoudsopgavevenster te activeren.
* De verwerking van koppen in Word-documenten verbeterd.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je het venster probeert te openen.

### Versie 0.6.0
* Een nieuwe optie om het Ga-menu in een veel compactere vorm te tonen, is toegevoegd aan het optievenster en is standaard ingeschakeld.
* Een optie toegevoegd om navigatie via structuurelementen te laten doorlopen.
* Een optie toegevoegd aan het menu Extra om de map te openen die het momenteel gefocuste document bevat.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een eenvoudige slaaptimerfunctie toegevoegd, bereikbaar met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het verwerken van FB2-e-books!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument-tekstbestanden!
* Bladwijzers kunnen nu een hele regel markeren of slechts een opgegeven stuk tekst. Als er geen selectie actief is wanneer je een bladwijzer plaatst, is het gedrag zoals vóór 0.6 en wordt de hele regel gemarkeerd. Selecteer je echter tekst, dan wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bevatten! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het bladwijzervenster met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd via specifieke sneltoetsen.
* Bladwijzers in het bladwijzervenster hebben niet langer een vervelend voorvoegsel "bladwijzer x".
* EPUB-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Het laden van grote Markdown-documenten opgelost.
* Opgelost dat het indrukken van de spatiebalk in de boomstructuur van de inhoudsopgave de knop OK activeerde.
* De verwerking van witruimte aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Opgelost dat het tekstbesturingselement soms de focus niet terugkreeg bij terugkeer naar het venster van Paperback.
* Opgelost dat het tekstveld in het venster 'ga naar percentage' de waarde van de schuifregelaar niet bijwerkte.
* De weergave van aangepaste HTML-ID's in Markdown-documenten opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Bij het laden van een boek via de terminal terwijl er al een Paperback-instantie draait, krijg je geen foutmelding meer als het laden van je document langer dan 5 seconden duurt.
* Bij het uitvoeren van Paperback als administrator wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks vanuit het bladwijzervenster te verwijderen.
* Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand krijgt de naam van het bestand met de extensie .paperback. Als zo'n bestand bij het laden in dezelfde map als een bestand wordt gevonden, wordt het automatisch geladen. Anders kun je ze handmatig importeren via een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om er voor- en achteruit doorheen te gaan, en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorbewerkt om CommonMark-compatibel te zijn vóór weergave.
* Navigatie per lijst en per lijstitem wordt nu volledig ondersteund! Gebruik L en Shift+L om per lijst te gaan, en I en Shift+I om door lijstitems te gaan.
* Numpad Delete werkt nu om documenten uit de tabbalk te verwijderen, naast de gewone Delete.
* Paperback kan nu optioneel minimaliseren naar je systeemvak! Deze optie is standaard uitgeschakeld, maar door deze in te schakelen plaatst de minimaliseeroptie in het systeemmenu Paperback in je systeemvak, waarna je het kunt herstellen door op het verschenen pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met ondersteunde talen is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website, op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een eenvoudige inhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu getoond in het documentinformatievenster.
* Het installatieprogramma bevat nu een optie om de handleiding na de installatie in je browser te bekijken.
* De lijst met recente documenten is sterk uitgebreid! In plaats van alleen de laatste 10 geopende documenten te tonen, toont deze nu een instelbaar aantal, terwijl de overige ooit geopende documenten toegankelijk zijn via een klein venster.
* Diverse kleine verbeteringen aan de verwerkers in het algemeen, waaronder het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het oplossen van de verwerking van regeleinden binnen alinea's in Word-documenten, en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning toegevoegd voor PowerPoint-presentaties!
* Opgelost dat bepaalde menu-items niet werden uitgeschakeld wanneer er geen documenten open waren.
* De oriëntatie van de schuifregelaar voor 'ga naar percentage' opgelost.
* De inhoudsopgave in EPUB-boeken met URL-gecodeerde bestandspaden en/of fragment-ID's opgelost.
* Opgelost dat witruimte op vreemde wijze uit XHTML-koppen werd verwijderd.
* De verwerking van witruimte in geneste pre-tags in HTML-documenten opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Wanneer je een HTML-/Markdown-document laadt, bouwt Paperback een eigen inhoudsopgave op basis van de koppenstructuur in je document en toont deze in het venster met ctrl+t.
* HTML-documenten hebben nu de titel zoals ingesteld in de titletag, indien aanwezig. Anders blijven ze de bestandsnaam zonder extensie gebruiken.
* Overgestapt van UniversalSpeech naar een liveregio om spraak te melden. Dit betekent dat er geen schermlezer-DLL's meer met het programma worden meegeleverd en dat meer schermlezers nu worden ondersteund, zoals Microsoft Verteller.
* Van ZIP-bibliotheek gewisseld om een breder scala aan EPUB-boeken te kunnen openen.
* Het venster dat vraagt of je je document als platte tekst wilt openen, is volledig vernieuwd en biedt nu de mogelijkheid om je document te openen als platte tekst, HTML of Markdown.
* Het venster 'ga naar percentage' bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naartoe te springen.
* De HTML-verwerker herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in EPUB-boeken wordt nu weer exact behouden.
* Met de unicode niet-afbrekende spatie wordt nu rekening gehouden bij het verwijderen van lege regels.
* Je wordt niet langer elke keer dat je een onbekend bestand laadt gevraagd hoe je het wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenupictogram toegevoegd aan het installatieprogramma.
* De inhoudsopgave is nu in enkele gevallen overzichtelijker; bijvoorbeeld als je een onderliggend en een bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten opgelost.
* De inhoudsopgave in EPUB 3-boeken met absolute paden opgelost.
* CHM-documenten tonen nu hun titel zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers in zoveel documenten plaatsen als je wilt. Je kunt er voor- en achteruit doorheen springen met b en shift+b, er een instellen met ctrl+shift+b, en een venster openen om naar een specifieke bladwijzer te springen met ctrl+b.
* Een installatieprogramma toegevoegd naast het draagbare ZIP-bestand! Het installatieprogramma installeert Paperback in je map Program Files en stelt automatisch bestandskoppelingen voor je in.
* Tekstbestanden met BOM's worden nu correct gedecodeerd, en de BOM wordt niet langer aan het begin van de tekst getoond.
* Veel meer informatie toegevoegd aan de statusbalk. Deze toont nu je huidige regel, teken en leespercentage.
* HTML-opmerkingen en de inhoud van script- en styletags worden niet langer in de tekstuitvoer getoond.
* Bij het doorgeven van een relatief pad aan Paperback in de terminal wordt dit nu correct opgelost.
* Verplaatsing via percentage wordt nu afgehandeld door een eigen op een schuifregelaar gebaseerd venster, bereikbaar met ctrl+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaardwaarde.
* De logica voor het opslaan van de positie is nu veel slimmer en schrijft alleen naar de schijf wanneer dat absoluut noodzakelijk is.
* Het document dat de focus had toen je Paperback afsloot, wordt nu onthouden bij het opnieuw starten van de applicatie.
* Invoer in de vensters 'ga naar regel' en 'ga naar pagina' wordt nu strenger gecontroleerd.
* Inhoudsopgavenavigatie in EPUB 3-boeken met relatieve paden in hun manifest opgelost.

### Versie 0.3.0
* De inhoudsopgave in EPUB-boeken met URL-gecodeerde manifesten opgelost.
* Kopnavigatie in HTML-documenten met multibyte-Unicode-tekens opgelost.
* Hoog CPU-gebruik in documenten met lange titels opgelost, veroorzaakt door een regressie in wxWidgets.
* Het laden van UTF-8-tekstbestanden opgelost.
* Opgelost dat geneste inhoudsopgave-items in EPUB-boeken je cursor op de verkeerde positie plaatsten.
* Een crash bij het afsluiten van de applicatie in bepaalde gevallen opgelost.
* Een selectievakje toegevoegd aan het optievenster om tekstterugloop in of uit te schakelen!
* Het is nu mogelijk om te doneren aan de ontwikkeling van Paperback, via het nieuwe donatie-item in het menu Help of via de link 'sponsor this project' onderaan de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Van PDF-bibliotheek gewisseld naar die welke in Chromium wordt gebruikt, wat zorgt voor veel betrouwbaarder PDF-verwerking in het algemeen.
* Je kunt nu slechts één instantie van Paperback tegelijk laten draaien. Als je paperback.exe met een bestandsnaam uitvoert terwijl het al draait, wordt dat document geopend in de reeds draaiende instantie.
* Je kunt nu op Delete drukken op een document in het tabbladbesturingselement om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's toegevoegd aan het paginalabel in het venster 'ga naar pagina'.
* Tabben van de documentinhoud naar je lijst met geopende documenten toegestaan.
* Opgelost dat de kopsneltoetsen soms recente documenten openden als je er genoeg had.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit de tekstuitvoer.
* Opgelost dat kopnavigatie je soms op het verkeerde teken plaatste.

### Versie 0.2.0
* Ondersteuning voor Markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen toegevoegd voor het navigeren per kop in HTML-inhoud, inclusief EPUB-boeken en Markdown-documenten. Deze sneltoetsen zijn ontworpen om te werken zoals een schermlezer.
* Het laden van EPUB's met URL-gecodeerde bestandsnamen in hun manifest opgelost.
* Het laden van EPUB 3-boeken met ingesloten XHTML opgelost.
* Er wordt nu een bericht uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Een menu met recente documenten toegevoegd! Dit bewaart momenteel je laatste 10 geopende documenten, en door op Enter te drukken op een ervan wordt het ter lezing geopend.
* Het zoekvenster volledig herschreven, waardoor het veel eenvoudiger te gebruiken is, met daarnaast een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies!
* Eerder geopende documenten worden nu onthouden bij het opnieuw starten van de applicatie. Dit is instelbaar via het nieuwe optie-item in het menu Extra.
* Shift+F1 toegevoegd om de handleiding rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Eerste release.
