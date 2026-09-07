<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,55bac79e,a548b5d0,71df8e94,e9860ee8,c7735cbe); please review and edit as needed -->

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

## Momenteel ondersteunde bestandstypes

Paperback ondersteunt de volgende indelingen en extensies:

* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-ebooks (`.fb2`)
* HTML-documenten (`.htm`, `.html`, `.xhtml`)
* Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
* M4B-audioboeken (`.m4b`)
* MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
* OpenDocument-presentaties (`.odp`, `.fodp`)
* OpenDocument-tekstbestanden (`.odt`, `.fodt`)
* PDF-documenten (`.pdf`)
* PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
* RTF-documenten (`.rtf`)
* Platte tekst- en logbestanden (`.txt`, `.log`)

## Sneltoetsen

Paperback is ontworpen voor gebruik met het toetsenbord voorop. Hier zijn de huidige sneltoetsen.

De sneltoetsen hieronder zijn voor Windows. Waar macOS verschilt, staat het equivalent tussen haakjes — vooral omdat `Ctrl+G`, `Ctrl+W` en `Alt+Left`/`Right` al geclaimd worden door andere systeem- of app-conventies op dat platform.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle open documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (uit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS bevindt dit zich in plaats daarvan in het app-menu).

### Menu Gaan

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (wanneer ondersteund door het huidige document).
* `=`: Uw huidige leespercentage aankondigen.
* `Alt+Left` (macOS: `Cmd+[`): Teruggaan in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruitzetting in navigatiegeschiedenis.
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
* `\`: Naar uw tijdelijke bladwijzer gaan.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities gaan.
* `Ctrl+Alt+B`: Naar alleen bladwijzers gaan.
* `Ctrl+Alt+M`: Naar alleen notities gaan.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Notitietekst op de huidige positie weergeven.
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
* `Shift+,`: Naar het begin van de huidige container (lijst of tabel) gaan.
* `,`: Voorbij het einde van de huidige container (lijst of tabel) gaan.

### Menu Hulpmiddelen

* `Ctrl+W` (macOS: `RawCtrl+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Woordentelling voor het huidige document weergeven.
* `Ctrl+I`: Documentinfo weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Bevattende map openen.
* `Ctrl+Shift+V`: Huidige inhoud in webweergave openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Huidig document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernotitie op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Woordomslag in-/uitschakelen.
* `Ctrl+Space`: Audio-narratie afspelen/onderbreken.
* `'`: Audio-narratie vooruit zoeken.
* `;`: Audio-narratie achteruit zoeken.
* `Ctrl+'`: Zoekbereik voor audio verhogen.
* `Ctrl+;`: Zoekbereik voor audio verlagen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d.w.z. Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.

### Menu Help

* `Ctrl+F1`: Over-dialoogvenster weergeven.
* `F1`: Help weergeven in uw standaardbrowser.
* `Shift+F1`: Help in Paperback weergeven.
* `Ctrl+Shift+U`: Op updates controleren.
* `Ctrl+D`: Donatiepagina openen in uw standaardbrowser.

### Aanvullende documentweergavesleutels

* `Delete` / `Numpad Delete` op het tabregelaar: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Link op cursor activeren, of een tabelweergave openen wanneer op een tabelmarkering.
* `Shift+F10` of de Menu-/Toepassingstoets in de documenttekst: Het contextmenu openen.

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

### Versie 0.9.2
* Audioboeken maken het niet meer zo dat uw schermlezer een reeks spaties voorleest wanneer u focus op het tekstveld plaatst.
* Audioboeken geven nu de bestandsnaam aan wanneer u ze per sectie doorloopt.
* Audioboeken rapporteren nu hun werkelijke lengte, in plaats van te beweren dat elk bestand erin 24 uur duurt.
* Het Web View met Escape sluiten leidt niet langer tot een debug-waarschuwing nadat u een link erin hebt gevolgd.
* Kopiëren na Alles selecteren geeft u nu het hele document, in plaats van alleen het momenteel geladen gedeelte.
* Zoeken springt nu rechtstreeks naar de regel die het heeft gevonden, in plaats van u door het schermlezer te laten wachten terwijl het venster opnieuw wordt voorgelezen als de focus naar het boek terugkeert.
* EPUB's met een verdwaald ZIP64-blok openen niet langer met "Invalid local file header".
* Lange documenten lopen niet meer terug naar het begin terwijl een schermlezer er voortdurend doorheen leest.
* Links in WebView brengen u nu naar de sectie waar zij naar verwijzen, in plaats van te mislukken met "File not found".
* De automatische aankondiging "Document herladen" onderbreekt uw schermlezer niet langer halverwege een zin, maar wacht tot deze klaar is met wat het zegt.
* Het tabblad Algemeen van het dialoogvenster Instellingen bladert nu door de opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor controleren op updates.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordentelling en Documentinfo tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Versie 0.9.1
* Bladwijzer- en notitiesignalen spelen nu af op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van hun chronologie in stilte te openen en bij te werken.
* Gekrulde aanhalingstekens, em-dashes en vergelijkbare tekens verdwijnen niet meer uit RTF-documenten, waarbij de omringende woorden aan elkaar groeien.
* RTF-afbeeldingen lekken hun onbewerkte gegevens niet meer als vervormd tekst in het document.
* Het menu Recent documenten behoudt niet langer verouderde vermeldingen totdat iets anders gebeurt om het opnieuw op te bouwen.
* Toetsenaccelerators zijn terug in elke vertaling, dus de menu's van het Russisch hebben opnieuw toetsenbordbeveiligingsacces.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, zodat zij in de jumplist van de taakbalk en in de recente lijst van het Startmenu verschijnen.
* Opties is hernoemd naar Instellingen, wat overeenkomt met de mobiele apps en, op macOS, de platformconventie.
* Paperback onthoudt nu het vensterposition, grootte en gemaximaliseerde staat tussen runs.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen worden correct gelezen in talen die meer dan één vorm nodig hebben.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De naamgeving van acties in het dialoogvenster Toetsenbordssnelkoppelingen aanpassen kan nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken in de taakbalk en Alt+Tab van elkaar kunnen worden onderscheiden.
* Het dialoogvenster Update wordt nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool, genaamd pb, om snel een van Paperback's ondersteunde formaten naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst is nu gepagineerd, wat betekent dat u boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Gelieve elke vreemdheid met dit alstublieft te melden.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een volledig schermschakelaar.

##### Dialoogvenster Alle documenten
* Een knop Zoeken om verloren boeken te lokaliseren die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat u kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten af te selecteren.

##### Opties en Leesbaarheid
* Een tabblad Leesbaarheid, met de volgende opties:
    * Woordombreking (verplaatst van algemeen);
    * Tabellen inline renderen (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor woordombreking en daaropvolgende sneltoets.
* Een schakelaar om te bepalen hoe u tabellen wilt weergeven, en hoe u de weergave van tabellen in documenten unificeert.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de bladermodus in schermlezers.
* De sneltoets Gelijkteken om uw huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: u kunt er één per document hebben, en zij blijven bestaan. Gebruik schuine streep om er één in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het dialoogvenster Woordentelling, evenals de mogelijkheid om uw leesstempo in te stellen om deze meting werkelijk nuttig te maken.
* Als een selectie actief is wanneer u het dialoogvenster Woordentelling opent, wordt nu weergegeven hoeveel woorden u hebt geselecteerd.

##### Toetsenbordssnelkoppelingen
* De mogelijkheid om elke toetsenbordssnelkoppeling in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordssnelkoppeling om Paperback vanuit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren uitgebreid om te exporteren naar HTML en Markdown, naast platte tekst.

##### Updater
* Een knop Annuleren in het dialoogvenster Update in voortgang.
* De updater controleert nu of het gedownloade bestand niet is gewijzigd.

##### Webweergave
* De webweergave wordt nu geopend op uw huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02-audioweergave.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zip-bestanden met audiobestanden.
* Toetsenbordssnelkoppelingen en menu-items om afspelen/pauzeren, vooruit en achteruit zoeken en de zoekbedrag aan te passen.
* Opties om de leesstekst met audioweergave te synchroniseren, de audiozoekhoeveelheid in te stellen en te kiezen of het zoeken voorbij het einde van een hoofdstuk verdergaat in het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, afbeeldingen en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten die zijn gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een reeks mojibake.
* "Laatst gesloten opnieuw openen" probeert de gebundelde readme opnieuw te openen.
* Uw geselecteerde tabblad niet correct gefocust na herstart van Paperback.
* Paperback's behandeling van bestanden op Windows-netwerkstations: het huidige bestand in map weergeven focust nu correct het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u om bevestiging gevraagd wanneer er een wordt gevonden.
* Map openen bevat nu het gegeven bestand in verkenner.
* Het openen van de readme respecteert nu uw geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op high-DPI-beeldschermen.
* Het menu wordt nu correct bijgewerkt en de focus gaat naar het tekstbesturingselement wanneer u help in Paperback opent.
* Overgeschakeld naar een veel veiliger IPC-methode in Windows.
* De actieve documenttitel wordt nu voorgelezen bij overschakeling tussen tabbladen.
* Verminderd geheugengebruik op grote documenten door de grootte van de interne per-tekenindextabellen te halveren.

##### Dialoogvenster Alle documenten
* Escape sluit de dialoogvensters Documentinfo en Alle documenten niet.
* De titelbalk wordt niet bijgewerkt nadat een document in het dialoogvenster Alle documenten is gesloten.
* Readme.html wordt niet langer aan uw lijst met alle documenten toegevoegd wanneer geopend via Shift+F1.
* Het verwijderen van documenten uit het dialoogvenster Recent zal ook hun actieve tabblad sluiten.
* Uw zoekfilter wordt nu bewaard na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regelttekst aan.
* Ga naar regel, Ga naar pagina en Ga naar procent plaatsen uw cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Bladwijzer-/notitiesignalen moeten nu correct uitsluitend worden afgespeeld wanneer u over een woord met een ervan navigeert.

##### Leesbaarheid
* Het toepassen van woordombreking schiet u naar het begin van uw document.

##### Webweergave
* Het webweergave-dialoogvenster kan niet worden vergroot en verschijnt met een zeer kleine initiële grootte.
* Afbeeldingen moeten nu correct in de ingesloten webweergave worden weergegeven.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in opmerkingen bij release.

##### DAISY-boeken
* DAISY-boeken met onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten met niet-Latijnse tekens parseren.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet meer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en garbage in de boektekst plaatsen.
* Links in verouderde Mobi-boeken.
* Sterk verbeterde AZW3-parsing.

##### Word-documenten
* Word-documenten met landinstellings-specifieke stijlnamen die hun koppen niet correct weergeven.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen die niet tot regelafbrekingen in XHTML-documenten leiden.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor onjuist gelabelde PDF's.
* PDF-documenten die controletekens in hun titels en/of bladwijzers bevatten, crashen Paperback niet langer bij het openen.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning voor gecodeerde Microsoft Office-documenten toegevoegd. Momenteel worden verouderde Word, moderne Word en moderne PowerPoint ondersteund, met verouderde PowerPoint gepland voor de toekomst.
* Ondersteuning voor verouderde Microsoft Word-documenten toegevoegd!
* Ondersteuning voor verouderde PowerPoint-presentaties toegevoegd!
* Ondersteuning voor mobi- en AZW3-boeken toegevoegd!
* Ondersteuning voor gelabelde PDF-bestanden toegevoegd!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning voor zip-boeken van Bookshare (zowel DAISY als Word) toegevoegd!
* Alt-tekst voor ingebedde afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct navigatie van interne links.
* Go to page opgelost, was 1 af.
* De Escape-toets werkt nu niet meer om het dialoogvenster Openen als te sluiten.
* Het contextmenu van de lezer verschijnt nu niet meer bij rechtklikken of de toets Toepassingen.
* Het onjuiste document werd soms gefocust wanneer documenten vanuit de opdrachtregel werden geopend.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen u voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback respecteert nu uw instelling voor donkere modus van toepassing.
* DAISY XML-ondersteuning verwijderd, omdat deze niet meer nodig is.
* Teruggewisseld naar de native Win32-navigatie met eerste letter in de tabel met inhoud.
* Het dialoogvenster Fout bij laden toont nu meer gedetailleerde foutmeldingen.
* De webweergave opent nu veel sneller en vloeiender.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Een bug opgelost waarbij het openen van de webweergave in epubs met externe links deze automatisch zou activeren.
* Een bug opgelost waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's werden in sommige PDF-documenten in meerdere korte regels opgesplitst.
* PDF-documenten hebben nu basisondersteuning voor link- en kopnavigatie!
* RTF-tabbladen en regelinvoer worden nu exact weergegeven zoals ze in het document verschijnen.
* Teruggezwisseld naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering veel betrouwbaarder is.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs met de RTF-parser opgelost.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) die beschadigd raakten bij het openen van een bestand via een tweede Paperback-exemplaar.
* PDF-tekst in de verkeerde volgorde gelezen en onjuiste afstand rond gekapitaliseerde woorden.
* Traag documenten laden bij het openen van grote bestanden.
* De lokalisatie van de knoppen Ja/Nee in bevestigingsdialoogvensters opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu uw huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionaal geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, dank aan Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor platte Open Document Text-bestanden toegevoegd!
* Ondersteuning voor Flat Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingstekens met s en shift+s toegevoegd.
* Elke beweging groter dan 300 tekens zal nu automatisch aan uw navigatiegeschiedenis worden toegevoegd.
* Het herstellen van Paperback's venster vanuit het systeemvak opgelost.
* Markdown-documenten vertonen niet langer onbewerkte tekst maar gerenderde HTML in de webweergave.
* Tabellen renderen niet langer correct in Markdown-bestanden.
* PDF's met alleen afbeeldingen waarschuwen u nu voor hun bestaan wanneer u probeert er een in te laden.
* Versie-informatie correct ingebed in het Paperback-uitvoerbare bestand.
* Het dialoogvenster Opties opgesplitst in tabbladen voor gebruiksgemak en navigatie.
* Overgeschakeld naar Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app in Rust herschreven. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezer-specifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om er een in een webweergave te bekijken.
* Een basiswebrenderfunctie toegevoegd! Druk op Ctrl+Shift+V om het huidige gedeelte van uw document in een op internet gebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het dialoogvenster Alle documenten.
* De updatecontrole toont nu opmerkingen bij release wanneer een nieuwe versie beschikbaar is.
* Het herstellen van het venster vanuit het systeemvak opgelost.
* Ja/Nee-knopvertalingen in bevestigingsdialoogvensters opgelost.
* Configuraties laden bij het uitvoeren als beheerder opgelost.
* Opmerkingenafhandeling in XML- en HTML-documenten opgelost.
* TOC-parsing in Epub 2-boeken opgelost.
* Navigeren naar het volgende item met dezelfde letter in de tabel met inhoud opgelost.
* Het dialoogvenster Zoeken verbergt niet correct wanneer u de volgende/vorige knoppen gebruikt.
* Epub TOC's werpen u af en toe naar het verkeerde item.
* Verschillende witruimte-afhandelingsproblemen in XML-, HTML- en pre-tags opgelost.
* Off-by-one-fout in linknavigatie opgelost.
* Sommige boeken hebben achterliggende witruimte op hun regels.
* Verschillende parserproblemen opgelost.
* Bladwijzer-gerelateerde menu-items en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstafhandeling in verschillende documentindelingen.
* Verbeterde vertaalsamenwerking voor bijdragers.
* Veel interne refactors, het verplaatsen van het merendeel van de bedrijfslogica van de toepassing van C++ naar Rust voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een zeer basische functie voor vorige/volgende positie toegevoegd. Als u op Enter drukt op een interne link en deze verplaatst uw cursor, zal die positie nu worden onthouden en kan ernaar worden genavigeerd met alt+linkerpijl/rechterpijl.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle koppen in uw document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Links in sommige Epub-documenten werken niet meer correct.
* Epub TOC's parseren met relatieve paden opgelost.
* Sommige epub-documenten tonen geen titel of auteur.
* De titels van bepaalde epub-hoofdstukken verschijnen niet correct in het TOC-dialoogvenster.
* U kunt de spatiebalk niet gebruiken om de knoppen OK/annuleren in het TOC-dialoogvenster te activeren.
* Verbeterde afhandeling van koppen in Word-documenten.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u het dialoogvenster wilt openen.

### Versie 0.6.0
* Een nieuwe optie om het menu Gaan in een veel meer compacte vorm weer te geven is toegevoegd aan het dialoogvenster Opties, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen in te laten verpakken.
* Een optie toegevoegd aan het menu Extra om de containermap van het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basisslaapimersfunctie toegevoegd, toegankelijk via Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2-e-boeken!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu worden gebruikt om een volledige regel te bladwijzeren, of om alleen bepaalde tekst te markeren. Als u geen selectie actief hebt bij het plaatsen van een bladwijzer, is het gedrag zoals vóór 0.6 en wordt de hele regel gemarkeerd. Als u echter tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bij zich dragen! Navigeer tussen bladwijzers met notities via N en Shift+N, of open het dialoogvenster Bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd via specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een lastig "bladwijzer x"-voorvoegsel.
* Epub-boeken met HTML-inhoud die zich voordoen als XML worden nu correct afgehandeld.
* Het laden van grote Markdown-documenten opgelost.
* Het drukken van spatie in de tabel met inhoud aktiveerde de knop OK.
* Witruimte-afhandeling aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Het tekstbesturingselement verliest soms niet correct de focus terug wanneer u naar Paperback's venster terugkeert.
* Het tekstveld in het dialoogvenster Ga naar procent werkt de waarde van de schuifregelaar niet bij.
* De weergave van aangepaste HTML-ID's in Markdown-documenten opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijgt u niet langer een fout als het laden van uw document meer dan 5 seconden duurt.
* Als u Paperback als beheerder uitvoert, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer direct in het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leespositie voor een bepaald document in en uit te voeren. Het gegenereerde bestand heet naar het bestand met een .paperback-extensie. Als zo'n bestand zich in dezelfde map als een bestand bevindt terwijl u het laadt, wordt het automatisch geladen. Anders kunt u ze handmatig importeren met behulp van een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om voor- en achteruit door te gaan, en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu vooraf verwerkt om CommonMark-conform te zijn voordat deze wordt weergegeven.
* Navigatie per lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om via lijsten zelf te gaan, en I en Shift+I om door lijstitems te gaan.
* Numpad Delete werkt nu om documenten van de taakbalk te verwijderen naast normale Delete.
* Paperback kan nu optioneel naar uw systeemvak minimaliseren! Deze optie staat standaard uit, maar als u deze aanzet, wordt de optie Minimaliseren in het systeemmenu Paperback in uw vak geplaatst, om te worden hersteld door op het gespawande pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website, op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinfo.
* Het installatieprogramma bevat nu een optie om de readme in uw browser na installatie weer te geven.
* De lijst met recente documenten is enorm uitgebreid! In plaats van simpelweg de laatste 10 documenten weer te geven die u hebt geopend, geeft het nu een aanpasbaar aantal weer, met de rest van de documenten die u ooit hebt geopend, toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen aan de parsers in het hele bord, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het verhelpen van de newline-afhandeling in alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Microsoft Word-documentondersteuning toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden niet langer uitgeschakeld als geen documenten geopend zijn.
* De oriëntatie van de schuifregelaar Ga naar procent opgelost.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragment-ID's opgelost.
* Witruimte wordt op vreemde manieren verwijderd uit XHTML-kopjes.
* Witruimte-afhandeling in geneste pre-tags in HTML-documenten opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave op uit de structuur van de koppes in uw document en toont dit u in het ctrl+t-dialoogvenster.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, als deze bestaat. Anders blijven ze de bestandsnaam zonder de extensie gebruiken.
* Overgeschakeld van UniversalSpeech naar het gebruik van een live-regio om spraakinvoer te melden. Dit betekent dat er geen schermlezerDLL's meer bij het programma zijn verpakt, en meer schermlezers worden nu ondersteund, zoals Microsoft Narrator.
* Gewisseld van zipbibliotheken om een breder scala aan epub-boeken te openen.
* Het dialoogvenster waarin wordt gevraagd of u uw document als platte tekst wilt openen, is volledig opnieuw gemaakt en kunt u uw document nu als platte tekst, HTML of Markdown openen.
* Het dialoogvenster Ga naar procent bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als listelementen.
* De inhoudsopgave in Epub-boeken blijft nu exact behouden.
* De Unicode-spatie zonder onderbreking wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* U wordt niet langer om de vraag gesteld hoe u een onbekend bestand elke keer wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel pictogram in het Startmenu toegevoegd aan het installatieprogramma.
* De inhoudsopgave moet nu in enkele gevallen schoner zijn, bijvoorbeeld als u een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, ziet u nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten opgelost.
* De inhoudsopgave in Epub 3-boeken met absolute paden erin opgelost.
* CHM-documenten moeten nu hun titel weergeven zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* CHM-bestandsondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! U kunt zoveel bladwijzers in zoveel documenten hebben als u wilt. U kunt er met b en shift+b doorheen springen, er een met control+shift+b instellen en een dialoogvenster openen om met control+b naar een specifieke bladwijzer te springen.
* Een installatieprogramma naast het draagbare zipbestand toegevoegd! Het installatieprogramma installeert Paperback in uw map Programmabestanden en stelt automatisch bestandskoppelingen voor u in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Nu wordt uw huidige regel, teken en leespercentage weergegeven.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer in tekstuitvoer weergegeven.
* Bij het doorgeven van een relatief pad aan Paperback op de opdrachtregel, wordt het nu correct opgelost.
* Percentagebeweging wordt nu afgehandeld door zijn eigen schuifregelaar-gebaseerd dialoogvenster, toegankelijk via control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor positiebesparing is nu veel slimmer en mag alleen naar schijf schrijven als absoluut nodig.
* Het document dat u in focus hebt wanneer u Paperback sluit, wordt nu onthouden wanneer de toepassing opnieuw wordt opgestart.
* Invoer in de dialoogvensters Ga naar regel en Ga naar pagina moet nu strenger worden ontsmet.
* Navigatie naar de inhoudsopgave in epub 3-boeken met relatieve paden in hun manifesten opgelost.

### Versie 0.3.0
* De inhoudsopgave in epub-boeken met URL-gecodeerde manifesten opgelost.
* Kopnavigatie in HTML-documenten met multi-byte Unicode-tekens opgelost.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets opgelost.
* UTF-8-tekstbestanden laden opgelost.
* Geneste TOC-items in Epub-boeken die uw cursor op de verkeerde positie plaatsen opgelost.
* Een crash bij toepassingsafsluiting in bepaalde gevallen opgelost.
* Een selectievakje toegevoegd in het dialoogvenster Opties om woordombreking in of uit te schakelen!
* Het is nu mogelijk om aan Paperback's ontwikkeling bij te dragen, via het nieuwe doneeritem in het menu Help of via de link Dit project sponsoren onderaan de hoofdpagina van de GitHub-opslagplaats.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Gewisseld van PDF-bibliotheeken naar degene die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaarder PDF-parseren in het hele bord.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Paperback.exe uitvoeren met een bestandsnaam terwijl dit al wordt uitgevoerd, opent dat document in het al actieve exemplaar.
* U kunt nu Delete op een document in het tabbesturingselement drukken om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's toegevoegd aan het paginalabel in het dialoogvenster Ga naar pagina.
* Tabbladen van documentinhoud naar uw lijst met geopende documenten toestaan.
* De kopnavigatie-toetsaanslagen openen soms recente documenten als u er veel had.
* Paperback verwijdert nu onnodige zachte koppeltekens uit tekstuitvoer.
* Kopnavigatie plaatste u soms op het verkeerde teken.

### Versie 0.2.0
* Markdown-documentondersteuning toegevoegd!
* PDF-documentondersteuning toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsaanslagen toegevoegd voor navigatie per kopjes in HTML-inhoud, inclusief epub-boeken en Markdown-documenten. Deze toetsaanslagen zijn ontworpen om op dezelfde manier te werken als een schermlezer.
* Epubs laden met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Epub 3-boeken met XHTML erin laden opgelost.
* Een bericht wordt nu uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Een menu Recente documenten toegevoegd! Het slaat momenteel uw laatste 10 geopende documenten op en het indrukken op Enter op een document opent het voor lezen.
* Het dialoogvenster Zoeken volledig herschreven, waardoor het veel eenvoudiger te gebruiken is, terwijl ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies is toegevoegd!
* Eerder geopende documenten worden nu onthouden wanneer de toepassing opnieuw wordt gestart. Dit kan worden ingesteld via het nieuwe item Opties in het menu Extra.
* Shift+F1 toegevoegd om de readme rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Eerste release.
