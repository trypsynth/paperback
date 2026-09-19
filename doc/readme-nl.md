<!-- machine-translated from doc/readme.md (source-hash: 73a0b4f33ccffbf1; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,d4622ac1,2fb18876,71df8e94,e9860ee8,a7ac6234); please review and edit as needed -->

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

## Ondersteunde bestandstypen

Paperback ondersteunt de volgende formaten en extensies:

* Stripboekarchieven (`.cbz`)
* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-e-boeken (`.fb2`)
* HTML-documenten (`.htm`, `.html`, `.xhtml`)
* Handmatige pagina's, zowel `man` als BSD `mdoc` (`.1` tot `.9`, `.man`, `.roff`, en de gecomprimeerde vormen van elk)
* Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
* M4B-audioboeken (`.m4b`)
* MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
* OpenDocument-presentaties (`.odp`, `.fodp`)
* OpenDocument-tekstbestanden (`.odt`, `.fodt`)
* PDF-documenten (`.pdf`)
* PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
* RTF-documenten (`.rtf`)
* WinHelp-bestanden (`.hlp`)
* Gewone tekstbestanden en logbestanden (`.txt`, `.log`)

## Sneltoetsen

Paperback is ontworpen voor toetsenbordgebruik als primaire invoermethode. Hier zijn de huidige sneltoetsen.

De sneltoetsen hieronder zijn voor Windows. Waar macOS verschilt, wordt het equivalent in haakjes vermeld — vooral omdat `Ctrl+G`, `Ctrl+W` en `Alt+Left`/`Right` al zijn geclaimd door andere systeem- of app-conventies op dat platform.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" tonen (uit Recent Documents).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in plaats daarvan in het app-menu).

### Menu Gaan naar

* `Ctrl+F`: Het dialoogvenster Zoeken tonen.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (wanneer ondersteund door het huidige document).
* `=`: Uw huidige leespercentage en pagina aankondigen, bijv. "15%, pagina 30". De pagina wordt weggelaten voor documenten zonder paginanummers.
* `Alt+Left` (macOS: `Cmd+[`): Teruggaan in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruitgaan in navigatiegeschiedenis.
* `[`: Vorige sectie.
* `]`: Volgende sectie.
* `Shift+H`: Vorige kop.
* `H`: Volgende kop.
* `Shift+1` tot en met `Shift+6`: Vorige kop op niveau 1-6.
* `1` tot en met `6`: Volgende kop op niveau 1-6.
* `Shift+P`: Vorige pagina.
* `P`: Volgende pagina.
* `Shift+B`: Vorige bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Uw tijdelijke bladwijzer instellen.
* `\`: Naar uw tijdelijke bladwijzer springen.
* `Shift+N`: Vorige opmerking.
* `N`: Volgende opmerking.
* `Ctrl+B`: Naar alle bladwijzers en opmerkingen springen.
* `Ctrl+Alt+B`: Alleen naar bladwijzers springen.
* `Ctrl+Alt+M`: Alleen naar opmerkingen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dus de fysieke Control-toets en niet Cmd): Opmerkingstekst op de huidige positie weergeven.
* `Shift+K`: Vorige koppeling.
* `K`: Volgende koppeling.
* `Shift+G`: Vorige afbeelding.
* `G`: Volgende afbeelding.
* `Shift+F`: Vorige figuur.
* `F`: Volgende figuur.
* `Shift+T`: Vorige tabel.
* `T`: Volgende tabel.
* `Shift+M`: Vorige formule.
* `M`: Volgende formule.
* `Shift+S`: Vorig scheidingsteken.
* `S`: Volgende scheidingsteken.
* `Shift+L`: Vorige lijst.
* `L`: Volgende lijst.
* `Shift+I`: Vorig lijstitem.
* `I`: Volgende lijstitem.
* `Shift+,`: Naar het begin van de huidige container (lijst of tabel) gaan.
* `,`: Voorbij het einde van de huidige container (lijst of tabel) gaan.

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, dus de fysieke Control-toets en niet Cmd): Woordtelling voor het huidige document tonen.
* `Ctrl+I`: Documentgegevens weergeven.
* `Ctrl+T`: Inhoudsopgave tonen.
* `F7`: Elementenlijst tonen.
* `Ctrl+Shift+C`: Bevattende map openen.
* `Ctrl+Shift+V`: Huidige inhoud in webweergave openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzeropmerkingen op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Regelterugloop in-/uitschakelen.
* `Ctrl+Space`: Audionarratieve afspelen/onderbreken.
* `'`: Audionarratieve vooruitspoelen.
* `;`: Audionarratieve terugspoelen.
* `Ctrl+'`: Het bedrag voor audionavigatie vergroten.
* `Ctrl+;`: Het bedrag voor audionavigatie verkleinen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dus Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, onder het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.
* `Alt+F9` (macOS: `Cmd+F9`): Het begin van een selectie markeren, zodat alles van hier tot waar u ook gaat, in één keer kan worden gekopieerd.
* `Alt+F10` (macOS: `Cmd+F10`): Alles van het gemarkeerde begin van de selectie tot de huidige positie kopiëren.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Teruggaan naar het gemarkeerde begin van de selectie, met het gemarkeerde gedeelte intact.

### Menu Help

* `Ctrl+F1`: Het dialoogvenster Info tonen.
* `F1`: Help in uw standaardbrowser weergeven.
* `Shift+F1`: Help in Paperback weergeven.
* `Ctrl+Shift+U`: Op updates controleren.
* `Ctrl+D`: Donatiesite in uw standaardbrowser openen.

### Aanvullende documentweergavetoetsen

* `Delete` / `Numpad Delete` op het tabblaatbesturingselement: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Een koppeling volgen of een tabel- of formuleweergave op de cursor openen.
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

## Wijzigingslogboek

### Versie 0.9.2
* Audioboeken zorgen niet langer ervoor dat je schermlezer een reeks spaties uitspreekt wanneer je het tekstveld focus geeft.
* Audioboeken benoemen nu het bestand terwijl je er doorheen stapt op sectieniveau.
* Audioboeken rapporteren nu hun werkelijke lengte, in plaats van te beweren dat elk bestand erin 24 uur duurt.
* Het sluiten van de webweergave met Escape geeft geen debug-waarschuwing meer nadat je een link erin hebt gevolgd.
* Kopiëren na Alles selecteren geeft nu het hele document, in plaats van alleen het deel dat momenteel is geladen.
* Zoeken gaat nu direct naar de regel die het heeft gevonden, in plaats van je door de schermlezer te laten zitten die het venster opnieuw uitspreekt als focus terugkeert naar het boek.
* EPUB-bestanden met een zwervend ZIP64-blok weigerden te openen met "Ongeldig lokaal bestandskoptekst" - dit is nu opgelost.
* Lange documenten liepen terug naar hun begin terwijl een schermlezer er doorheen las - dit is nu opgelost.
* Links in de webweergave brengen je nu naar de sectie waar ze naar verwijzen, in plaats van te mislukken met "Bestand niet gevonden".
* Markeer het begin van een selectie met `Alt+F9`, kopieer alles van daar naar waar je bent met `Alt+F10`, en ga terug naar de markering met `Alt+Shift+F9`, voor het kopiëren van een lange tekstspanne zonder shift-pijlen. Alle drie staan onder Extra > Selecteren en kopiëren.
* De `=` sneltoets kondigt nu de pagina aan evenals het percentage, bijv. "15%, pagina 30", en blijft hetzelfde voor documenten zonder paginanummers.
* De automatische "Document opnieuw geladen"-mededeling onderbreekt je schermlezer niet langer midden in een zin, maar wacht tot het klaar is met wat het zei.
* Het tabblad Algemeen van de instellingendialoog loopt nu door zijn opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor controle op updates.
* Bijwerken brengt het opnieuw gestarte venster nu naar voren, in plaats van het achter elk ander venster in Alt+Tab te laten.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordentelling en Documentinfo tonen nu hoeveel bestanden een audioboek bevat en hoe lang het totaal duurt.

### Versie 0.9.1
* Geluiden voor bladwijzers en notities worden nu afgespeeld op macOS.
* DAISY-boeken spelen nu hun audio af op macOS, in plaats van hun tijdlijn te openen en bij te werken in stilte.
* Gekrulde aanhalingstekens, em-streepjes en vergelijkbare tekens verdwenen uit RTF-documenten - dit is nu opgelost.
* RTF-afbeeldingen lekten hun onbewerkte gegevens in het document als verminkte tekst - dit is nu opgelost.
* Het recente documenten-submenu behield stale vermeldingen totdat iets anders het opnieuw opbouwde - dit is nu opgelost.
* Sneltoetsen zijn terug in elke vertaling, dus de menu's van Russisch hebben opnieuw toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, dus ze verschijnen in de taakbalksprong- en startmenulijst.
* Opties is hernoemd naar Instellingen, in overeenstemming met de mobiele apps en op macOS met de platformconventie.
* Paperback onthoudt nu zijn vensterpositie, grootte en gemaximaliseerde status tussen runs.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van een DAISY-boek's ncc.html opent nu het volledige audioboek in plaats van alleen de tekst ervan.
* De actienamen van de dialoog Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, dus geopende boeken kunnen in de taakbalk en Alt+Tab onderscheiden worden.
* De update-dialoog is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool genaamd pb om snel elk door Paperback ondersteund formaat naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die zijn gewijzigd door andere programma's op schijf.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alles vreemds dat je hiermee vindt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Systeemeigen macOS-ondersteuning!
* Een volledig scherm schakelen.

##### Dialoog Alle documenten
* Een knop zoeken om vermiste boeken die zojuist hun pad hebben gewijzigd, te zoeken.
* Een statusfilter en statusbalk, zodat je kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten af te selecteren.

##### Opties en leesbaarheid
* Een leesbaarheid-tabblad met de volgende opties:
    * Tekstomloop (verplaatst uit algemeen);
    * Tabellen inline renderen (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor tekstomloop en volgende sneltoets.
* Een schakelaar om te bepalen hoe je tabellen wilt weergegeven, en hoe tabellen in documenten op uniforme wijze worden weergegeven.

##### Navigatie
* MathML-formules in EPUB en HTML worden weergegeven als AsciiMath met MathCAT. Gebruik `M` of `Shift+M` om formules te navigeren, druk vervolgens `Enter` of `Space` om de originele MathML in formulierweergave te openen.
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de bladermodus in schermlezers.
* De equals-sneltoets om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er een per document hebben, en ze blijven behouden. Gebruik schuine streep om er een in te stellen en omgekeerde schuine streep om ernaar toe te springen.

##### Woordentelling
* Geschatte leestijd in de woordentelling-dialoog, evenals de mogelijkheid om je leessnelheid in te stellen om deze metriek werkelijk bruikbaar te maken.
* Als er een selectie actief is wanneer je de woordentelling-dialoog opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudige dialoog.
* Een configureerbare toetsenbordsneltoets om Paperback terug te stellen vanuit het systeemvak.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren uitgebreid om naar HTML en Markdown te exporteren, naast platte tekst.

##### Updater
* Een annuleringsknop in de dialoog update-in-uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audioweergeving.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunend voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zips van audiobestanden.
* Toetsenbordsneltoetsen en menu-items om narratie af te spelen/onderbreken, vooruit en achteruit te zoeken en de zoekgrootte aan te passen.
* Opties om de leesvakje te synchroniseren met audioweergeving, de audio-zoekgrootte in te stellen en te kiezen of zoeken voorbij het einde van een hoofdstuk in het volgende gaat.

##### CHM-documenten
* Ondersteuning voor lijsten, listitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een hoop mojibake.
* "Laatst gesloten opnieuw openen" probeert het ingesloten readme opnieuw te openen.
* Je geselecteerde tabblad kreeg niet correct focus nadat Paperback opnieuw werd gestart.
* Paperback's verwerking van bestanden op Windows-netwerkstations: het indrukken van bestand weergeven in map geeft nu correct focus aan het bestand op de netwerkopslag, en de paden bevatten geen vreemde tekens meer.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* Map met inhoud openen geeft nu correct focus aan het gegeven bestand in verkenner.
* Het readme openen zal nu je geselecteerde taal respecteren.
* Paperback's gebruikersinterface wordt nu correct geschaald op beeldschermen met hoge DPI.
* Het menu werkt nu correct bij, en focus verplaatst zich naar het tekstbesturingselement, wanneer help in Paperback wordt geopend.
* Overgeschakeld naar een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu uitgesproken bij het schakelen tussen tabbladen.
* Verminderd geheugengebruik op grote documenten door de grootte van de interne per-teken indextabellen te halveren.

##### Dialoog Alle documenten
* Escape sluit de dialogen Documentinfo en Alle documenten niet.
* De titelbalk werkt niet bij nadat je een document uit de dialoog voor alle documenten sluit.
* Readme.html wordt niet meer aan je lijst met alle documenten toegevoegd wanneer geopend via Shift+F1.
* Het verwijderen van documenten uit de recente dialoog sluit nu ook hun actieve tabblad.
* Je zoekfilter wordt nu behouden nadat je een document hebt verwijderd.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regeltekst aan.
* Ga naar Regel, Ga naar Pagina en Ga naar Procent plaatsen je cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Geluid voor bladwijzer/opmerking zou nu correct alleen afgespeeld moeten worden wanneer je over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Tekstomloop toepassen schiet je naar het begin van je document.

##### Webweergave
* De webweergave-dialoog kon niet worden aangepast en verscheen bij een zeer kleine initiële grootte.
* Afbeeldingen moeten nu correct in de ingebedde webweergave worden weergegeven.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in release notes.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste info in de statusbalk.
* DAISY-boeken laden met nepcoderingsverklaringen.

##### RTF-documenten
* RTF-documenten met niet-Latijnse tekens erin parseren.
* RTF `\pict` groepen zodat ingesloten afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en zetten rommel in de boektekst.
* Links in verouderde Mobi-boeken.
* Sterk verbeterd AZW3-parsen.

##### Word-documenten
* Word-documenten met taalspecifieke stijlnamen renderen hun koppen niet correct.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen produceren geen regelbreuk in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op platte-tekst-extractie voor vals getagde PDF's.
* PDF-documenten met controletekens in hun titels en/of bladwijzers zullen Paperback bij opening niet langer crash.

### Versie 0.8.5
* Paginaondersteuning aan epub-boeken toegevoegd.
* Ondersteuning voor gecodeerde Microsoft Office-documenten toegevoegd. Momenteel worden verouderde Word, modern Word en modern Powerpoint ondersteund, met legacy Powerpoint gepland voor de toekomst.
* Ondersteuning voor verouderde Microsoft Word-documenten toegevoegd!
* Ondersteuning voor verouderde Powerpoint-presentaties toegevoegd!
* Ondersteuning voor mobi- en AZW3-boeken toegevoegd!
* Ondersteuning voor getagde PDF-bestanden toegevoegd!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning voor gecomprimeerde boeken van Bookshare toegevoegd (zowel DAISY als Word)!
* Alt-tekst voor ingesloten afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct interne linknavigatie.
* Go to page corrigeren was uit met 1.
* De escape-toets werkt niet om de dialoog open te sluiten.
* Het contextmenu van lezer verschijnt niet na een rechtsklik of de toets Toepassingen.
* Het verkeerde document kreeg soms focus bij het openen van documenten via de opdrachtregel.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen u voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback zal nu je donkere modus-instelling van de toepassing respecteren.
* DAISY XML-ondersteuning verwijderd, omdat deze niet langer nodig is.
* Teruggeschakeld naar de native Win32 eerste letter navigatie in de inhoudsopgave boomweergave.
* De dialoog voor laadfouten toont nu meer gedetailleerde foutberichten.
* De webweergave opent nu veel sneller en soepeler.

### Versie 0.8.2
* Paginaondersteuning aan RTF-documenten toegevoegd!
* Een bug opgelost waarbij het openen van de webweergave in epub's met externe links deze automatisch zou activeren.
* Een bug opgelost waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden plaatsen zou.
* Alinea's worden in sommige PDF-documenten in meerdere korte regels gesplitst.
* PDF-documenten hebben nu basisondersteuning voor link- en kopnavigatie!
* RTF-tabbladen en regelinvoer worden nu exact zoals ze in het document voorkomen weergegeven.
* Teruggeschakeld naar de bewezen pdfium-bibliotheek voor het parseren van PDF's, wat PDF-rendering veel betrouwbaarder maakt.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* De dialoog Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs met de RTF-parser opgelost.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) worden beschadigd wanneer een bestand wordt geopend via een tweede Paperback-instantie.
* PDF-tekst wordt in de verkeerde volgorde gelezen en onjuiste spatiëring rond gekapitaliseerde woorden.
* Langzaam laden van documenten bij het openen van grote bestanden.
* De lokalisatie van de knoppen Ja/Nee in bevestigingsdialogen opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je huidige Paperback-installatie vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsreactie voor het bereiken van een bladwijzer of opmerking toegevoegd, dank je Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor Flat Open Document Text-bestanden toegevoegd!
* Ondersteuning voor Flat Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingstekens met s en shift+s.
* Elke beweging van meer dan 300 tekens voegt nu automatisch toe aan je navigatiegeschiedenis.
* Paperback's venster van het systeemvak terugzetten opgelost.
* Markdown-documenten geven onbewerkte tekst weer in plaats van weergegeven HTML in de webweergave.
* Tabellen renderen niet correct in Markdown-bestanden.
* PDF's met alleen afbeeldingen zullen je waarschuwen voor hun bestaan wanneer je probeert er een in te laden.
* Versie-informatie correct in het Paperback-uitvoerbare bestand ingebed.
* De opties-dialoog in tabbladen opgesplitst voor gemak en navigatie.
* Overgeschakeld naar Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app in Rust herschreven. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van de tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML- en XHTML-gebaseerde documenten toegevoegd! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om er een in een webweergave te bekijken.
* Een basale webweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank je Ruslan Gulmagomedov!
* Een knop Alles wissen aan de dialoog Alle documenten toegevoegd.
* De updatechecker geeft nu release notes weer wanneer een nieuwe versie beschikbaar is.
* Het venster van het systeemvak herstellen opgelost.
* Vertaling van knop Ja/Nee in bevestigingsdialogen opgelost.
* Configuraties laden wanneer het programma als beheerder wordt uitgevoerd opgelost.
* Commentaarverwerking in XML- en HTML-documenten opgelost.
* TOC-parsing in Epub 2-boeken opgelost.
* Navigeren naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* De dialoog Zoeken verbergt niet correct wanneer u de knoppen Volgende/Vorige gebruikt.
* Epub-TOC's gooien je soms naar het verkeerde item.
* Verschillende witruimte-verwerkingsproblemen in XML-, HTML- en pre-tags opgelost.
* Off-by-one error in linknavigatie opgelost.
* Sommige boeken hebben trailing whitespace op hun regels.
* Verschillende parser-problemen opgelost.
* Bladwijzer-gerelateerde menu-items evenals de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstverwerking in verschillende documentindelingen.
* Verbeterde vertaalworkflow voor medewerkers.
* Veel interne refactors, waarbij het merendeel van de bedrijfslogica van de toepassing van C++ naar Rust is verplaatst voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Met wachtwoord beveiligde PDF-ondersteuning toegevoegd!
* Een zeer basale functie om naar de vorige/volgende positie te gaan toegevoegd. Als je op Enter drukt op een interne link en deze je cursor verplaatst, wordt die positie nu onthouden en kan ernaar worden genavigeerd met Alt+pijl links/rechts.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boomstructuur van alle koppen in je document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard gemaximaliseerd in te starten.
* Links in sommige Epub-documenten werkten niet correct.
* Epub-TOC's met relatieve paden parseren opgelost.
* Sommige epub-documenten tonen geen titel of auteur.
* De titels van sommige epub-hoofdstukken verschijnen niet correct in de TOC-dialoog.
* Je kon de spacebar niet gebruiken om de knoppen OK/annuleren in de TOC-dialoog in te schakelen.
* Verbeterde verwerking van koppen in Word-documenten.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je probeert de dialoog op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het menu Gaan in een veel compactere vorm te tonen is aan de opties-dialoog toegevoegd, standaard aangevinkt.
* Een optie toegevoegd om navigatie door structurele elementen in te keren.
* Een optie aan het menu Extra toegevoegd om de inhoudende map van het momenteel gefocuste document te openen.
* Een heel eenvoudig, maar zeer effectief update-systeem toegevoegd.
* Een basale slaaptimer-functie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning voor het parseren van FB2 e-boeken toegevoegd!
* Ondersteuning voor het parseren van OpenDocument-presentaties toegevoegd!
* Ondersteuning voor het parseren van OpenDocument Text-bestanden toegevoegd!
* Bladwijzers kunnen nu worden gebruikt om een hele regel in een bladwijzer in te stellen, of om alleen bepaalde tekst in te stellen. Als je geen selectie actief hebt wanneer je een bladwijzer plaatst, is het gedrag zoals vóór 0.6, en wordt de hele regel gemarkeerd. Als je echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities hebben! Navigeer tussen bladwijzers met notities met N en Shift+N, of open de bladwijzerdialoog met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in de bladwijzerdialoog hebben niet langer een vervelend "bladwijzer x" voorvoegsel.
* Epub-boeken met HTML-inhoud die doen alsof het XML is, worden nu correct verwerkt.
* Grote Markdown-documenten laden opgelost.
* Space drukken in de inhoudsopgave boomweergave activeert de knop OK.
* Witruimte-verwerking aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Het tekstbesturingselement krijgt soms geen focus terug wanneer het terugkeert naar Paperback's venster.
* Het tekstveld in de dialoog Ga naar percentage werkt niet correct met de schuifregelaar.
* De rendering van aangepaste HTML-ID's in Markdown-documenten opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek laadt met een opdrachtregelparameter terwijl een bestaande Paperback-instantie wordt uitgevoerd, krijg je niet langer een fout als het laden van je document meer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks vanuit de bladwijzerdialoog te verwijderen.
* Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald document in te voeren en uit te voeren. Het gegenereerde bestand krijgt dezelfde naam als het bestand met een .paperback-extensie. Als zo'n bestand in dezelfde map als een bestand wordt gevonden bij het laden ervan, wordt het automatisch geladen. Anders kun je het handmatig importeren met behulp van een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om vooruit en achteruit door links te gaan, en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorbewerkt om CommonMark-compliant te zijn voordat deze wordt weergegeven.
* Navigatie per lijst en items ervan wordt nu volledig ondersteund! Gebruik L en Shift+L om jezelf door lijsten te verplaatsen, en I en Shift+I om door listitems te gaan.
* Numpad Delete werkt nu ook om documenten uit de tabbalk te verwijderen naast normale delete.
* Paperback kan nu optioneel naar je systeemvak minimaliseren! Deze optie is standaard uitgeschakeld, maar als je deze inschakelt, zal de optie minimaliseren in het systeemmenu Paperback in je vak plaatsen, zodat je het door op het gespawner pictogram te klikken kunt terugzetten.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in de dialoog Documentinfo.
* Het installatieprogramma bevat nu een optie om het readme na installatie in je browser te bekijken.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van je alleen de laatste 10 geopende documenten te tonen, toont het je nu een aanpasbaar getal, met de rest van de documenten die je ooit hebt geopend, toegankelijk via een kleine dialoog.
* Verschillende kleine verbeteringen aan de parsers in het algemeen, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het repareren van regelinvoerverwerking in Word-documenten en het toevoegen van opsommingstekens aan listitems.

### Versie 0.5.0
* Microsoft Word-documentondersteuning toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden niet correct uitgeschakeld zonder geopende documenten.
* De oriëntatie van de schuifregelaar Ga naar procent opgelost.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragmentID's opgelost.
* Witruimte wordt op vreemde manieren uit XHTML-koppen verwijderd.
* Witruimte-verwerking in geneste pre-tags in HTML-documenten opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgave-functie! Wanneer je een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave uit de structuur van de koppen in je document en toont deze je in de Ctrl+T-dialoog.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, als deze bestaat. Anders zullen ze blijven de bestandsnaam zonder de extensie gebruiken.
* Overgeschakeld van UniversalSpeech naar het gebruik van een livegebied voor het rapporteren van spraak. Dit betekent dat er niet langer schermlezerDLL's naast het programma worden verzonden, en dat meer schermlezers nu worden ondersteund, zoals Microsoft Narrator.
* Van ZIP-bibliotheek gewisseld om een breder scala aan epub-boeken te openen.
* De dialoog waarin je wordt gevraagd of je je document als platte tekst wilt openen, is volledig opnieuw gemaakt en je kunt je document nu als platte tekst, HTML of Markdown openen.
* De dialoog Ga naar procent bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als listelementen.
* De inhoudsopgave in Epub-boeken blijft nu exact behouden.
* De unicode niet-onderbrekende spatie wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* Je wordt niet langer gevraagd hoe je een onbekend bestand wilt openen elke keer dat je het laadt, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenu-pictogram aan het installatieprogramma toegevoegd.
* De inhoudsopgave moet in een paar gevallen schoner zijn, bijvoorbeeld als je een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten opgelost.
* De inhoudsopgave in Epub 3-boeken met absolute paden erin opgelost.
* CHM-documenten moeten nu hun titel tonen zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* CHM-bestandondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers hebben in zoveel documenten als je wilt. Je kunt erdoorheen springen met b en shift+b, er een instellen met control+shift+b, en een dialoog openen om naar een specifieke bladwijzer te springen met control+b.
* Een installatieprogramma toegevoegd naast het draagbare zipbestand! Het installatieprogramma installeert Paperback in je map Program Files en stelt automatisch bestandskoppelingen in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd, en het BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie aan de statusbalk toegevoegd. Het toont nu je huidige regel, karakter en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en stijltags, worden niet langer weergegeven in tekstuitvoer.
* Als je een relatief pad aan Paperback geeft op de opdrachtregel, wordt het nu correct opgelost.
* Percentageverplaatsing wordt nu verwerkt door zijn eigen schuifregelaar-gebaseerde dialoog, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor het opslaan van posities is nu veel intelligenter en moet alleen naar schijf schrijven wanneer absoluut nodig.
* Het document waar je op had gericht toen je Paperback sloot, wordt nu onthouden bij toepassingsherstarts.
* Invoer in dialogen Ga naar regel en Ga naar pagina moet nu strikter worden ontsmend.
* Navigatie naar inhoudsopgave in epub 3-boeken met relatieve paden in hun manifesten opgelost.

### Versie 0.3.0
* De inhoudsopgave in epub-boeken met URL-gecodeerde manifesten opgelost.
* Kopnavigatie in HTML-documenten met multi-byte Unicode-tekens opgelost.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets opgelost.
* UTF-8 tekstbestanden laden opgelost.
* Geneste TOC-items in Epub-boeken zettend je cursor op de verkeerde positie.
* Een crash bij toepassingsbeëindiging in bepaalde gevallen opgelost.
* Een selectievakje in de opties-dialoog toegevoegd om tekstomloop in en uit te schakelen!
* Het is nu mogelijk om aan Paperback's ontwikkeling bij te dragen, hetzij via het nieuwe doneeritem in het menu Help of via de link Sponsor dit project onderaan de GitHub-opslagplaats's hoofdpagina.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Van PDF-bibliotheek gewisseld naar de bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaardere PDF-parsing in het algemeen.
* Je kunt nu slechts één instantie van Paperback tegelijk hebben draaien. Paperback.exe uitvoeren met een bestandsnaam terwijl het al draait, opent dat document in de al draaiende instantie.
* Je kunt nu op Delete drukken op een document in het tabbladbesturingselement om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's in het paginarlabel in de dialoog Ga naar pagina toegevoegd.
* Tabbladindeling van documentinhoud naar je lijst met geopende documenten toegestaan.
* Bepaalde bugs waar sneltoetsen voor koppen recente documenten zouden openen als je er genoeg van had opgelost.
* Paperback verwijdert nu onnodig zacht koppelteken uit tekstuitvoer.
* Kopnavigatie zet je soms op het verkeerde teken.

### Versie 0.2.0
* Markdown-documentondersteuning toegevoegd!
* PDF-documentondersteuning toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen voor navigatie per kop in HTML-inhoud toegevoegd, inclusief epub-boeken en Markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar te werken als een schermlezer.
* Epub's laden met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Epub 3-boeken laden met XHTML erin ingebed opgelost.
* Een bericht wordt nu uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Een menu voor recente documenten toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op, en op Enter drukken op een ervan zal het voor lezen openen.
* De dialoog Zoeken volledig herschreven, waardoor het veel eenvoudiger in gebruik is, terwijl ook een geschiedenis van je laatste 25 zoekacties en ondersteuning voor reguliere expressies wordt toegevoegd!
* Eerder geopende documenten worden nu bij toepassingsherstarts onthouden. Dit is configureerbaar via het nieuwe item Opties in het menu Extra.
* Shift+F1 toegevoegd om het readme rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Eerste release.
