<!-- machine-translated from doc/readme.md (source-hash: d583a89d8ac391f5; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,af36028a,71df8e94,e9860ee8,93dd8dd6); please review and edit as needed -->

# Paperback - versie 0.9.2

## Introductie

Paperback is een lichte, snelle en toegankelijke ebook- en documentlezer voor iedereen, van casual lezers tot ervaren gebruikers. Het is ontworpen met schermlezeraccessibiliteit, snelheid en een minimalistisch ontwerp in gedachten.

## Systeemvereisten

Paperback werkt momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testversies gepland kort na de 0.9.0 desktoprelease, voorafgaand aan een uniforme 1.0-release voor alle vier platforms.

## Functies

* Volledig zelfstandig, waarbij u geen software op uw computer hoeft te installeren om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten naast elkaar kunt openen als u wilt.
* Slaat uw exacte leespositie op voor elk document dat u opent.
* Kan optioneel onthouden welke documenten u had geopend toen u het programma sloot, en herstelt deze bij de volgende start.
* Bevat navigatiefunctionaliteit vergelijkbaar met die in de webbrowsingsmodus van veel schermlezers om snel en gemakkelijk door documenten te navigeren.
* Bevat een robuuste zoekdialoog, inclusief functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of worden geïnstalleerd met bestandskoppelingen die automatisch worden ingesteld.
* Ondersteunt een enorm aantal gangbare bestandsindelingen.

## Compatibiliteit met schermlezers

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en brailleweergaven

Als u JAWS met een brailleweergave gebruikt, kan het voorkomen dat lange alinea's worden afgekapt wanneer u vooruit navigeert met de navigatietoetsen van uw weergave. Het commando voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een bug in JAWS's verwerking van het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf, en het duurde even voordat een oplossing werd gevonden gezien Vispero's enthousiasme voor het reageren op problemen met open source-software.

De workaround, uiteindelijk opgeleverd via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw weergave op de actieve alinea staan in plaats van deze op te schuiven. Met beide instellingen in plaats moeten de schuifbewegingen correct werken.

## Ondersteunde bestandstypes

Paperback ondersteunt de volgende formaten en extensies:

* Comic book archives (`.cbz`, `.cbr`)
* CHM help files (`.chm`)
* DAISY books (`.opf`, `.zip`)
* EPUB books (`.epub`)
* FB2 ebooks (`.fb2`)
* HTML documents (`.htm`, `.html`, `.xhtml`)
* Manual pages, both `man` and BSD `mdoc` (`.1` to `.9`, `.man`, `.roff`, and the gzipped forms of each)
* Markdown documents (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word documents (`.docx`, `.docm`, `.doc`)
* M4B audiobooks (`.m4b`)
* MOBI/Kindle books (`.mobi`, `.azw`, `.azw3`)
* OpenDocument presentations (`.odp`, `.fodp`)
* OpenDocument text files (`.odt`, `.fodt`)
* PDF documents (`.pdf`)
* PowerPoint presentations (`.pptx`, `.pptm`, `.ppt`)
* RTF documents (`.rtf`)
* WinHelp files (`.hlp`)
* Plain text and log files (`.txt`, `.log`)

## Toetsenbordssnelkoppelingen

Paperback is ontworpen voor gebruik met toetsenbordfocus. Hier zijn de huidige snelkoppelingen.

De snelkoppelingen hieronder zijn voor Windows. Waar macOS verschilt, staat het equivalent in haakjes vermeld — vooral omdat Ctrl+G, Ctrl+W en Alt+Left/Right op dat platform al door andere systeem- of app-conventies zijn geclaimd.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (van Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS bevindt dit zich in plaats daarvan in het app-menu).

### Menu Gaan naar

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (wanneer ondersteund door het huidige document).
* `=`: Geef uw huidige leespercentage en pagina aan, bijvoorbeeld "15%, pagina 30". De pagina wordt weggelaten voor documenten zonder paginanummers.
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
* `Shift+B`: Vorige bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Uw tijdelijke bladwijzer instellen.
* `\`: Naar uw tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Naar bladwijzers alleen springen.
* `Ctrl+Alt+M`: Naar notities alleen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dat wil zeggen de fysieke Control-toets in plaats van Cmd): Notitiestekst op de huidige positie weergeven.
* `Shift+K`: Vorige link.
* `K`: Volgende link.
* `Shift+G`: Vorige afbeelding.
* `G`: Volgende afbeelding.
* `Shift+F`: Vorige figuur.
* `F`: Volgende figuur.
* `Shift+T`: Vorige tabel.
* `T`: Volgende tabel.
* `Shift+S`: Vorig scheidingsteken.
* `S`: Volgende scheidingsteken.
* `Shift+L`: Vorige lijst.
* `L`: Volgende lijst.
* `Shift+I`: Vorig lijstitem.
* `I`: Volgende lijstitem.
* `Shift+,`: Naar het begin van de huidige container gaan (lijst of tabel).
* `,`: Voorbij het einde van de huidige container gaan (lijst of tabel).

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, dat wil zeggen de fysieke Control-toets in plaats van Cmd): Woordaantal voor het huidige document weergeven.
* `Ctrl+I`: Documentinformatie weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: De map met het document openen.
* `Ctrl+Shift+V`: Huidige inhoud in Web View openen.
* `Ctrl+U`: De documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer bij de huidige selectie/cursor schakelen.
* `Ctrl+Shift+N`: Bladwijzeropmerking bij de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Woordomvloeiing schakelen.
* `Ctrl+Space`: Audio-voordracht afspelen/onderbreken.
* `'`: Audio-voordracht vooruit spoelen.
* `;`: Audio-voordracht terugspoelen.
* `Ctrl+'`: Het bedrag voor audio-zoeken verhogen.
* `Ctrl+;`: Het bedrag voor audio-zoeken verlagen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dat wil zeggen Control+Command+F): Volledig scherm schakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer schakelen.

### Help-menu

* `Ctrl+F1`: Dialoogvenster Info weergeven.
* `F1`: Help in uw standaardbrowser weergeven.
* `Shift+F1`: Help in Paperback weergeven.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: De donatatiepagina in uw standaardbrowser openen.

### Aanvullende toetsen voor documentweergave

* `Delete` / `Numpad Delete` op het tabblaadbesturingselement: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Link op cursor activeren, of tabelweergave openen als u zich op een tabelmarkering bevindt.
* `Shift+F10` of de Menu/Application-toets in de documenttekst: Het contextmenu openen.

## Ondersteunde talen

Paperback is vertaald in veel verschillende talen, en er worden steeds meer toegevoegd. Een volledige lijst volgt hieronder.

Als u wilt weten hoe u kunt bijdragen, lees dan onze [vertaalgids](translating.md).

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
* Vietnamees

## Credits
### Ontwikkeling
* Quin Gillespie: primaire ontwikkelaar en projectoprichter.
* Aryan Choudhary: belangrijkste bijdrager.

### Donaties
De volgende personen hebben donaties van enige omvang aan Paperback-ontwikkeling gedaan. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd; ik voeg alleen personen toe die hun donatie openbaar willen maken.

Opmerking: Ik beschouw een openbare GitHub-sponsor als reden voor automatische opneming in deze lijst.

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

### Version 0.9.2
* Audioboeken laten je schermlezer niet langer een reeks spaties voorlezen wanneer je de tekstveld focust.
* Audioboeken benoemen nu het bestand wanneer je erdoorheen stapt per onderdeel.
* Audioboeken melden nu hun werkelijke lengte, in plaats van te claimen dat elk bestand 24 uur duurt.
* Het sluiten van de webweergave met Escape toont niet langer een debug-waarschuwing nadat je een link erin hebt gevolgd.
* Kopiëren na Alles selecteren geeft je nu het hele document, in plaats van alleen het deel dat momenteel is geladen.
* Zoeken gaat nu direct naar de regel die het heeft gevonden, in plaats van je door de schermlezer de hele venster te laten voorlezen wanneer de focus naar het boek terugkeert.
* Vaste EPUB-bestanden die een rogue ZIP64-blok bevatten en weigeren te openen met 'Ongeldige lokale bestandskoptekst'.
* Vaste lange documenten die terugliepen naar hun begin terwijl een schermlezer continu erdoorheen las.
* Links in de webweergave brengen je nu naar de sectie waar ze naar wijzen, in plaats van te mislukken met 'Bestand niet gevonden'.
* De `=` sneltoets kondigt nu het paginanummer en het percentage aan, bijvoorbeeld '15%, pagina 30', en blijft hetzelfde voor documenten zonder paginanummers.
* De automatische mededeling 'Document opnieuw geladen' onderbreekt je schermlezer niet langer mid-zin, maar wacht tot het klaar is.
* Het tabblad Algemeen van het dialoogvenster Instellingen doorloopt nu zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie Controleren op updates.
* Bijwerken brengt het opnieuw gestarte venster nu naar voren, in plaats van het achter elk ander venster in Alt+Tab te laten.
* Windows toont nu altijd 'Paperback' in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordentellingen en Documentinfo tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Version 0.9.1
* Geluiden van bladwijzers en notities worden nu afgespeeld op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van hun tijdlijn in stilte te openen en bij te houden.
* Vaste kromlijnige aanhalingstekens, em-streepjes en vergelijkbare tekens verdwijnen uit RTF-documenten, waardoor omringende woorden eraan vastplakken.
* Vaste RTF-afbeeldingen lekken hun onbewerkte gegevens als garbled tekst in het document.
* Vaste het submenu Recente documenten dat verouderde items behoudt tot iets anders het opnieuw opbouwt.
* Toetsenbordaccelerators zijn terug in elke vertaling, dus Russische menu's hebben weer toetsenbordfunctie.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, dus ze verschijnen in de taakbalkspronglijst en in de lijst met recente items in het Startmenu.
* Opties is gewijzigd in Instellingen, overeenkomstig met de mobiele apps en op macOS de platformconventie.
* Paperback onthoudt nu zijn vensterpositie, -grootte en gemaximaliseerde toestand tussen runs.
* Meervoudsvormen zijn nu vertaald, dus berichten die dingen tellen lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van een DAISY-boek's ncc.html opent nu het volledige audioboek in plaats van alleen de tekst ervan.
* De actienamen van het dialoogvenster Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, dus geopende boeken kunnen worden onderscheiden in de taakbalk en Alt+Tab.
* Het bijwerkingsdialoogvenster is nu vertaald.

### Version 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool, genaamd pb, om snel een van Paperback's ondersteunde formaten naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die zijn gewijzigd door andere programma's op schijf.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, nuttig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden nu in slechts enkele seconden kunt laden. Meld alles vreemdelijks dat je daarmee ontdekt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Inheemse macOS-ondersteuning!
* Een volledige scherminvulling in-/uitschakelen.

##### Dialoogvenster Alle documenten
* Een zoekknop om vermiste boeken te zoeken die net van pad zijn veranderd.
* Een statusfilter en statusbalk, zodat je kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten te deselecteren.

##### Opties en Leesbaarheid
* Een leesbaarheidstabblad met de volgende opties:
    * Woordomvloeiing (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor woordomvloeiing en de volgende sneltoets.
* Een schakeloptie om te bepalen hoe je tabellen wilt weergeven, en geünificeerde weergave van tabellen in documenten.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de bladermodus in schermlezers.
* De sneltoets Gelijkteken om je huidige percentage door een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en ze blijven bestaan. Gebruik slash om er één in te stellen en backslash om ernaar te springen.

##### Woordentellingen
* Geschatte leestijd in het dialoogvenster Woordentellingen, evenals de mogelijkheid om je leessnelheid in te stellen zodat deze metriek werkelijk nuttig is.
* Als een selectie actief is wanneer je het dialoogvenster Woordentellingen opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordsneltoets om Paperback uit de systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren is uitgebreid om exporteren naar HTML en Markdown, naast platte tekst.

##### Updater
* Een annuleerknop voor het dialoogvenster Update-in-uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gemanipuleerd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02-audioweergave.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunend zowel DAISY-audio (inclusief DAISY-audio + tekst) als gecomprimeerde audiobestanden.
* Toetsenbordsneltoetsen en menu-items om vertellingen af te spelen/onderbreken, vooruit en achteruit te zoeken en de zoekbedrag aan te passen.
* Opties om het leesvakje met audioweergave te synchroniseren, het audiosoekhoeveelheid in te stellen en te kiezen of het zoeken voorbij het einde van een hoofdstuk in de volgende gaat.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Verholpen

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een hoop mojibake.
* 'Laatste gesloten opnieuw openen' probeert de bundled readme opnieuw te openen.
* Je geselecteerde tabblad krijgt niet goed focus nadat je Paperback opnieuw bent gestart.
* Paperback's verwerking van bestanden op Windows-netwerkstations: op 'Bestand in map weergeven' drukken focust nu correct het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* Omvattende map openen focust nu het gegeven bestand in verkenner.
* Het openen van de readme respecteert nu je geselecteerde taal.
* Paperback's gebruikersinterface schaalt nu correct op high-DPI-displays.
* Het menu wordt nu correct bijgewerkt en de focus gaat naar het tekstbewerkingselement wanneer help in Paperback wordt geopend.
* Overgeschakeld naar een veel veiliger methode van IPC op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het wisselen van tabbladen.
* Verminderd geheugengebruik bij grote documenten door de grootte van de interne indexatietabellen per teken in tweeën te delen.

##### Dialoogvenster Alle documenten
* Escape sluit niet het dialoogvenster Documentinfo en Alle documenten.
* De titelbalk wordt niet bijgewerkt na het sluiten van een document uit het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan je lijst Alle documenten toegevoegd wanneer deze via Shift+F1 wordt geopend.
* Het verwijderen van documenten uit het dialoogvenster Recents sluit nu ook hun actieve tabblad.
* Je zoekfilter is nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regeltekst aan.
* Ga naar Regel, Ga naar Pagina en Ga naar Percentage plaatsen je cursor op de verkeerde plaats in grote documenten.
* Zoeken en Zoeken volgende respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Geluiden van bladwijzers/notities moeten nu correct uitsluitend worden afgespeeld wanneer je over een woord met een ervan navigeert.

##### Leesbaarheid
* Woordomvloeiing toepassen schiet je naar het begin van je document.

##### Webweergave
* Het dialoogvenster Webweergave is niet schaalbaar en verschijnt met een zeer kleine initiële grootte.
* Afbeeldingen moeten nu correct in de ingebedde webweergave worden weergegeven.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste info in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten met niet-Latijnse tekens parseren.
* RTF `\pict`-groepen zodat ingesloten afbeeldingsgegevens niet langer in documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en rommel in de boektekst plaatsen.
* Links in verouderde Mobi-boeken.
* Aanzienlijk verbeterde AZW3-parsing.

##### Word-documenten
* Word-documenten met taalafhankelijke stijlnamen die hun koppen niet correct weergeven.

##### HTML/XHTML-documenten
* dl, dt en dd elementen die geen regelafbrekingen in XHTML-documenten produceren.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor onjuist getagde PDF's.
* PDF-documenten met besturingstekens in hun titels en/of bladwijzers crashen Paperback niet langer bij openen.

### Version 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden Legacy Word, modern Word en modern Powerpoint ondersteund, met Legacy Powerpoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor verouderde Microsoft Word-documenten!
* Ondersteuning toegevoegd voor verouderde Powerpoint-presentaties!
* Ondersteuning toegevoegd voor mobi en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De ctrl+q-sneltoets toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor gecomprimeerde boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingesloten afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct interne linknavigatie.
* Vaste ga naar pagina is 1 uit.
* Vaste de escape-toets werkt niet om het dialoogvenster Open als te sluiten.
* Vaste het leezercontextmenu verschijnt niet op rechter klikken of de Applications-toets.
* Vaste het verkeerde document wordt soms gefocust bij het openen van documenten vanaf de opdrachtregel.
* PDF-bestanden met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen je voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback zal nu je instellingen voor donkere toepassingsmodus respecteren.
* DAISY XML-ondersteuning verwijderd, aangezien deze niet langer nodig is.
* Teruggekeerd naar de inheemse Win32-navigatie met eerste letter in de inhoudsopgave boom.
* Het dialoogvenster voor foutbericht toont nu meer gedetailleerde foutmeldingen.
* De webweergave opent nu veel sneller en soepeler.

### Version 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Vaste een bug waarbij het openen van de webweergave in epub's met externe links deze automatisch zou activeren.
* Vaste een bug waarbij de RTF-parser niet altijd een spatie tussen woorden zou plaatsen.
* Vaste alinea's worden in sommige PDF-documenten in meerdere korte regels opgesplitst.
* PDF-documenten hebben nu basis link- en kopnavigatie-ondersteuning!
* RTF-tabs en regelafbrekingen worden nu exact weergegeven zoals ze in het document voorkomen.
* Teruggekeerd naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering veel betrouwbaarder is.

### Version 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Vaste enkele bugs met de RTF-parser.
* Vaste bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) worden beschadigd wanneer een bestand via een tweede Paperback-instantie wordt geopend.
* Vaste PDF-tekst wordt in de verkeerde volgorde gelezen en onjuiste afstand rond gekapitaliseerde woorden.
* Vaste langzaam laden van documenten bij het openen van grote bestanden.
* Vaste lokalisatie van de Ja/Nee-knoppen in bevestigingsdialoogvensters.

### Version 0.8.0
* Japanse, Chinees (vereenvoudigd) en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je momenteel geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of een notitie, bedankt Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor platte open documenttekstbestanden!
* Ondersteuning toegevoegd voor platte open documentpresentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan je navigatiegeschiedenis.
* Vaste het herstellen van Paperback's venster uit de systeemvak.
* Vaste Markdown-documenten tonen onbewerkte tekst in plaats van weergegeven HTML in de webweergave.
* Vaste tabellen die niet correct worden weergegeven in Markdown-bestanden.
* PDF's met alleen afbeeldingen waarschuwen je nu voor hun bestaan wanneer je probeert er een in te laden.
* Versie-informatie correct ingebed in het Paperback-uitvoerbare bestand.
* Het dialoogvenster Opties in tabbladen opgesplitst voor gemak van gebruik en navigatie.
* Overgeschakeld naar Hayro voor PDF-parsing, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De gehele app opnieuw geschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbewerkingselement bevat nu lezerspesifieke acties in plaats van generieke items zoals knippen en plakken.

### Version 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk Enter om er één in een webweergave te bekijken.
* Een basiswebweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codesamples.
* Een Russische vertaling toegevoegd, bedankt Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het dialoogvenster Alle documenten.
* De updatechecker toont nu releaseopmerkingen wanneer een nieuwe versie beschikbaar is.
* Vaste het herstellen van het venster uit de systeemvak.
* Vaste Ja/Nee-knoptvertalingen in bevestigingsdialoogvensters.
* Vaste het laden van configs wanneer het als beheerder wordt uitgevoerd.
* Vaste opmerkingenverwerking in XML- en HTML-documenten.
* Vaste TOC-parsing in Epub 2-boeken.
* Vaste navigatie naar het volgende item met dezelfde letter in de inhoudsopgave.
* Vaste het dialoogvenster Zoeken verbergen niet correct wanneer u de volgende/vorige knoppen gebruikt.
* Vaste epub TOC's gooiden je soms naar het verkeerde item.
* Vaste verschillende witruimte-handlingproblemen in XML, HTML en pre-tags.
* Vaste off-by-one fout in linknavigatie.
* Vaste sommige boeken hebben witruimte aan het einde van hun regels.
* Vaste verschillende parsingproblemen.
* Menu-items met betrekking tot bladwijzers en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstverwerking in verschillende documentformaten.
* Verbeterde vertaalgew werkstroom voor contribuanten.
* Veel interne refactors, het verplaatsen van het merendeel van de bedrijfslogica van de applicatie van C++ naar Rust voor verbeterde prestaties en onderhoudbaarheid.

### Version 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een zeer basisvoorafgaande/volgende positiefunctie toegevoegd. Als je op een interne link drukt en het verplaatst je cursor, wordt die positie nu onthouden en kan ermee worden genavigeerd met alt+pijl-links/rechts.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle koppen in je document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Vaste links in sommige Epub-documenten werken niet correct.
* Vaste parsing Epub-TOC's met relatieve paden.
* Vaste sommige epub-documenten tonen geen titel of auteur.
* Vaste de titels van sommige epub-hoofdstukken worden niet correct in het TOC-dialoogvenster weergegeven.
* Vaste je kunt niet de spatiebalk gebruiken om de OK/annuleer-knoppen in het TOC-dialoogvenster te activeren.
* Verbeterde verwerking van koppen in Word-documenten.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je het dialoogvenster opent.

### Version 0.6.0
* Een nieuwe optie om het menu Gaan in een veel compactere vorm weer te geven is toegevoegd aan het dialoogvenster Opties, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen in te passen.
* Een optie toegevoegd aan het menu Extra om de omvattende map van het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basisslaapmerfunctie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2-e-boeken!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-tekstbestanden!
* Bladwijzers kunnen nu een hele regel boeien of alleen bepaalde tekst markeren. Als je geen actieve selectie hebt wanneer je een bladwijzer plaatst, is het gedrag zoals pre-0.6, en markeert het de hele regel. Als je echter tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities aan zich hebben gehecht! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het dialoogvenster Bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers zullen niet langer een vervelend 'bladwijzer x'-voorvoegsel hebben.
* Epub-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Vaste het laden van grote Markdown-documenten.
* Vaste spatie indrukken in de inhoudsopgaveboomweergave activeert de OK-knop.
* Vaste witruimtebehandeling aan het begin van pre-tags in zowel HTML- als XHTML-documenten.
* Vaste het tekstbewerkingselement kreeg niet altijd focus terug wanneer het terugkeerde naar Paperback's venster.
* Vaste het tekstveld in het dialoogvenster Ga naar percentage werkt de schuifregelaar van de schuifregelaar niet bij.
* Vaste het weergeven van aangepaste HTML-id's in Markdown-documenten.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek opent met een opdrachtregelparameter terwijl een bestaande Paperback-instantie wordt uitgevoerd, krijg je geen fout meer als het laden van je document meer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks vanuit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald document in te voeren en uit te voeren. Het gegenereerde bestand heeft een .paperback-extensie. Als een dergelijk bestand in dezelfde map als een bestand wordt gevonden terwijl het wordt geladen, wordt het automatisch geladen. Anders kunt u ze handmatig importeren met behulp van een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om voor en achteruit door hen heen te gaan, en druk Enter om er één te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorverwerkt om CommonMark-compatibel te zijn voordat het wordt weergegeven.
* Navigatie op lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om zelf door lijsten heen te gaan, en I en Shift+I om door lijstitems heen te gaan.
* Numpad-verwijdering werkt nu ook om documenten van de taakbalk te verwijderen, naast normale verwijdering.
* Paperback kan nu optioneel naar je systeemvak minimaliseren! Deze optie is standaard uitgeschakeld, maar het inschakelen ervan maakt de minimaliseeroptie in het systeemmenu Paperback in je lade zet, kan worden hersteld door op het geboortepictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt, is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinfo.
* Het installatieprogramma bevat nu een optie om het Leesmij na installatie in uw browser weer te geven.
* De lijst met recente documenten is dramatisch uitgebreid! In plaats van simpelweg de laatste 10 documenten weer te geven die je hebt geopend, toont het nu een aanpasbaar aantal, met de rest van de documenten die je ooit hebt geopend, toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen aan de parsers op het bord, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het oplossen van regelafbreekbeleid in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Version 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Vaste bepaalde menu-items zijn niet uitgeschakeld zonder geopende documenten.
* Vaste de oriëntatie van de schuifregelaar Ga naar percentage.
* Vaste inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragmentids.
* Vaste witruimte wordt op vreemde manieren uit XHTML-koppen verwijderd.
* Vaste witruimtebehandeling binnen geneste pre-tags in HTML-documenten.
* HTML- en Markdown-documenten ondersteunen nu de functie Inhoudsopgave! Wanneer je een HTML/Markdown-document laadt, bouwt Paperback een eigen inhoudsopgave uit de structuur van de koppen in je document, en toont die aan je in het dialoogvenster ctrl+t.
* HTML-documenten hebben nu de titel ingesteld in de titeltag, indien aanwezig. Anders zullen zij doorgaan met het gebruik van de bestandsnaam zonder de extensie.
* Overgeschakeld van UniversalSpeech naar het gebruik van een live regio om spraak te rapporteren. Dit betekent dat er niet langer DLL's voor schermlezers bij het programma zijn geleverd, en meer schermlezers worden nu ondersteund, zoals Microsoft Narrator.
* Schakelde zip-bibliotheken over om het openen van een breder scala aan epub-boeken toe te staan.
* Het dialoogvenster waarin u wordt gevraagd of u uw document als platte tekst wilt openen, is volledig opnieuw uitgevoerd en kunt u uw document nu als platte tekst, HTML of Markdown openen.
* Het dialoogvenster Ga naar percentage bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken blijft nu precies behouden.
* De unicode non-breaking space wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* U wordt niet langer gevraagd hoe u een onbekend bestand elke keer wilt openen, alleen de eerste keer.

### Version 0.4.1
* Een optioneel pictogram in het startmenu toegevoegd aan het installatieprogramma.
* De inhoudsopgave moet nu in enkele gevallen schoner zijn, bijvoorbeeld als je een onderliggende en bovenliggende item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* Vaste inhoudsopgave in bepaalde CHM-documenten.
* Vaste inhoudsopgave in Epub 3-boeken met absolute paden.
* CHM-documenten moeten nu hun titel tonen zoals ingesteld in het metagegevensbestand.

### Version 0.4.0
* CHM-bestandondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers hebben in zoveel documenten als je wilt. Je kunt er doorheen naar voren en naar achteren met b en shift+b gaan, er één instellen met control+shift+b, en een dialoogvenster brengen om naar een specifieke bladwijzer met control+b te springen.
* Een installatieprogramma toegevoegd naast het draagbare zip-bestand! Het installatieprogramma installeert Paperback in uw map Programmabestanden en stelt automatisch bestandskoppelingen voor u in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd, en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Het toont je nu je huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en stijltags, worden niet langer weergegeven in tekstuitvoer.
* Als u een relatief pad naar Paperback opgeeft op de opdrachtlijn, wordt het nu correct omgezet.
* Procentuele beweging wordt nu verwerkt door zijn eigen schuifregelaar-gebaseerd dialoogvenster, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor positiebesparing is nu veel intelligenter en hoeft alleen naar schijf te schrijven wanneer dit absoluut noodzakelijk is.
* Het document waarop je focus had toen je Paperback sloot, wordt nu onthouden over herstarts van de toepassing.
* Invoer in de dialoogvensters Ga naar regel en Ga naar pagina moet nu meer rigoureus worden ontsmet.
* Vaste inhoudsopgavenavigatie in epub 3-boeken met relatieve paden in hun manifesten.

### Version 0.3.0
* Vaste inhoudsopgave in epub-boeken met URL-gecodeerde manifesten.
* Vaste koppen navigeren in HTML-documenten met multibyte Unicode-tekens.
* Vaste hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets.
* Vaste UTF-8-tekstbestanden laden.
* Vaste geneste TOC-items in Epub-boeken plaats je cursor op de verkeerde plaats.
* Vaste een crash bij afsluiten van de toepassing in bepaalde gevallen.
* Een selectievakje in het dialoogvenster Opties toegevoegd om woordomvloeiing in of uit te schakelen!
* Het is nu mogelijk om naar Paperback's ontwikkeling te doneren, hetzij via het nieuwe doneerupperste item in het menu Help, hetzij via de link Dit project sponsoren onder aan de GitHub-hoofdpagina van de repository.
* Markdown-documenten hebben nu altijd een titel, en Paperback moet nu in staat zijn om praktisch elk Markdown-bestand in te laden.
* PDF-documenten zullen nu altijd een titel hebben, zelfs als de metagegevens ontbreken.
* Omgeschakeld naar PDF-bibliotheken die in Chromium worden gebruikt, wat veel betrouwbaarder PDF-parsing oplevert.
* Je kunt nu slechts één instantie van Paperback tegelijk hebben draaien. Paperback.exe uit met een bestandsnaam terwijl het al wordt uitgevoerd, opent dat document in de al draaiende instantie.
* Je kunt nu op verwijderen op een document in de tabcontrol drukken om het te sluiten.

### Version 0.2.1
* Het totale aantal pagina's toegevoegd aan de paginalabel in het dialoogvenster Ga naar pagina.
* Tabbladen van documentinhoud naar je lijst geopende documenten toestaan.
* Vaste de koptoetsenbordsinvoer opent soms recente documenten als je genoeg ervan had.
* Paperback verwijdert nu onnodige zachte verbindingsstreepjes uit tekstuitvoer.
* Vaste koppen navigatie plaatst je soms op het verkeerde teken.

### Version 0.2.0
* Markdown-documentondersteuning toegevoegd!
* PDF-documentondersteuning toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsenbordcombinaties voor het navigeren door koppen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze toetsenbordcombinaties zijn ontworpen om op dezelfde manier als een schermlezer te werken.
* Vaste epubs laden met URL-gecodeerde bestandsnamen in hun manifesten.
* Vaste EPUB 3-boeken laden met XHTML erin ingebed.
* Een bericht wordt nu gesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items zijn uitgeschakeld.
* Een menu met recente documenten toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op, en als je op een ervan drukt, wordt het geopend voor lezen.
* Het dialoogvenster Zoeken volledig herschreven, waardoor het veel eenvoudiger in gebruik is, terwijl ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies is toegevoegd!
* Eerder geopende documenten worden nu onthouden over herstarts van de toepassing. Dit kan via het nieuwe item Opties in het menu Extra.
* Shift+F1 toegevoegd om het leesmij rechtstreeks in Paperback zelf te openen.

### Version 0.1.0
* Initiale versie.
