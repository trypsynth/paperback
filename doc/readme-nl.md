<!-- machine-translated from doc/readme.md (source-hash: df18cffffe239932); please review and edit as needed -->

# Paperback - versie 0.9.1

## Inleiding

Paperback is een lichte, snelle en toegankelijke eboek- en documentlezer voor iedereen, van occasionele lezers tot ervaren powergebruikers. Het is ontworpen voor schermlezer-toegankelijkheid, hoge snelheid en een bloatware-vrije ervaring.

## Systeemvereisten

Paperback draait momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testbuilds gepland kort na de 0.9.0 desktoprelease, voorafgaand aan een uniforme 1.0 release die alle vier platforms omvat.

## Functies

* Volledig zelfstandig, zonder dat er software op uw computer hoeft te worden geïnstalleerd om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op voor elk document dat u opent.
* Onthoud optioneel welke documenten u had geopend toen u het programma sloot, en herstel ze bij de volgende start.
* Bevat navigatiefunctionaliteit vergelijkbaar met die in de webbrowsingmodus van veel schermlezers, voor snelle en gemakkelijke navigatie door documenten.
* Bevat een robuuste zoekdialog, inclusief functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of worden geïnstalleerd met automatisch ingestelde bestandskoppelingen.
* Ondersteunt een enorme reeks van veelvoorkomende bestandsindelingen.

## Compatibiliteit met schermlezers

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en Braille-displays

Als u JAWS met een Braille-display gebruikt, kan het voorkomen dat lange alinea's worden afgekapt wanneer u vooruit bladert met de navigatietoetsen van uw display. Het commando voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een bug in JAWS's afhandeling van het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf, en iets waarbij het best lang duurde voordat een fix aan het licht kwam gezien Vispero's enthousiasme voor het reageren op problemen met open source software.

De omleiding, uiteindelijk aan het licht gebracht via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw display op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen op zijn plaats, zou bladeren correct moeten werken.

## Momenteel ondersteunde bestandstypes

Paperback ondersteunt de volgende indelingen en extensies:

* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-eboeken (`.fb2`)
* HTML-documenten (`.htm`, `.html`, `.xhtml`)
* Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
* MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
* OpenDocument-presentaties (`.odp`, `.fodp`)
* OpenDocument-tekstbestanden (`.odt`, `.fodt`)
* PDF-documenten (`.pdf`)
* PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
* RTF-documenten (`.rtf`)
* Platte tekst- en logbestanden (`.txt`, `.log`)

## Toetsenbordssneltoetsen

Paperback is ontworpen voor gebruik met het toetsenbord voorop. Hier zijn de huidige sneltoetsen.

De sneltoetsen hieronder zijn voor Windows. Waar macOS afwijkt, staat het equivalent tussen haakjes — vooral omdat `Ctrl+G`, `Ctrl+W` en `Alt+Left`/`Right` al zijn geclaimd door andere systeem- of app-conventies op dat platform.

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
* `Ctrl+P`: Naar pagina gaan (indien ondersteund door het huidige document).
* `=`: Uw huidige leespercentage aankondigen.
* `Alt+Left` (macOS: `Cmd+[`): Teruggaan in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruit gaan in navigatiegeschiedenis.
* `[`: Vorige sectie.
* `]`: Volgende sectie.
* `Shift+H`: Vorige kop.
* `H`: Volgende kop.
* `Shift+1` tot en met `Shift+6`: Vorige kop op niveau 1-6.
* `1` tot en met `6`: Volgende kop op niveau 1-6.
* `Shift+P`: Vorige pagina.
* `P`: Volgende pagina.
* `Shift+B`: Vorig bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Stel uw tijdelijke bladwijzer in.
* `\`: Naar uw tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Naar bladwijzers alleen springen.
* `Ctrl+Alt+M`: Naar notities alleen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Notitietekst op de huidige positie weergeven.
* `Shift+K`: Vorige link.
* `K`: Volgende link.
* `Shift+G`: Vorige afbeelding.
* `G`: Volgende afbeelding.
* `Shift+F`: Vorige figuur.
* `F`: Volgende figuur.
* `Shift+T`: Vorige tabel.
* `T`: Volgende tabel.
* `Shift+S`: Vorige scheidingsteken.
* `S`: Volgende scheidingsteken.
* `Shift+L`: Vorige lijst.
* `L`: Volgende lijst.
* `Shift+I`: Vorig lijstitem.
* `I`: Volgende lijstitem.
* `Shift+,`: Ga naar het begin van de huidige container (lijst of tabel).
* `,`: Ga voorbij het einde van de huidige container (lijst of tabel).

### Menu Gereedschappen

* `Ctrl+W` (macOS: `RawCtrl+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Woordaantal voor het huidige document tonen.
* `Ctrl+I`: Documentinfo tonen.
* `Ctrl+T`: Inhoudsopgave tonen.
* `F7`: Elementenlijst tonen.
* `Ctrl+Shift+C`: Map met inhoud openen.
* `Ctrl+Shift+V`: Huidige inhoud in Web View openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer bij de huidige selectie/cursor in- of uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernotitie bij de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Woordterugloop in- of uitschakelen.
* `Ctrl+Space`: Audio-vertelling afspelen/onderbreken.
* `'`: Audio-vertelling vooruitspoelen.
* `;`: Audio-vertelling terugspoelen.
* `Ctrl+'`: Hoeveelheid audio-zoeken verhogen.
* `Ctrl+;`: Hoeveelheid audio-zoeken verlagen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d.w.z. Control+Command+F): Volledig scherm in- of uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in- of uitschakelen.

### Help-menu

* `Ctrl+F1`: Dialoogvenster Info tonen.
* `F1`: Help weergeven in uw standaardbrowser.
* `Shift+F1`: Help weergeven in Paperback.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: Donattiepagina openen in uw standaardbrowser.

### Aanvullende toetsen in documentweergave

* `Delete` / `Numpad Delete` op het taabbeheer: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Link bij cursor activeren, of een tabelweergave openen wanneer u op een tabelmarker staat.
* `Shift+F10` of de Menu/Application-toets in de documenttekst: Het contextmenu openen.

## Ondersteunde talen

Paperback is in veel verschillende talen vertaald en er worden steeds meer bijgevoegd. Hieronder volgt een volledige lijst.

Om te leren hoe u kunt bijdragen, lees dan onze [Vertaalhandleiding](translating.md).

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
* Aryan Choudhary: primaire bijdrager.

### Donaties
De volgende personen hebben donaties van enige omvang gedaan aan Paperback-ontwikkeling. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd. Ik voeg alleen personen toe die willen dat hun donatie openbaar wordt gemaakt.

Opmerking: Ik beschouw een openbare GitHub-sponsor als reden voor automatische opname in deze lijst.

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
* Audioboeken laten uw schermlezer niet langer een reeks spaties uitspreken wanneer u de tekstveld focust.
* Audioboeken vermelden nu de bestandsnaam terwijl u door secties navigeert.
* Audioboeken geven nu de werkelijke lengte aan, in plaats van te beweren dat elk bestand 24 uur duurt.
* Het sluiten van Web View met Escape geeft niet langer een debugwaarschuwing nadat u een link erin hebt gevolgd.
* Kopiëren na Selecteer alles geeft u nu het hele document, in plaats van alleen het gedeelte dat momenteel is geladen.
* Zoeken gaat nu rechtstreeks naar de regel die het heeft gevonden, in plaats van u door de schermlezer te laten luisteren terwijl de focus naar het boek terugkeert.
* Vast: EPUB-bestanden die een stray ZIP64-blok bevatten weigeren niet langer te openen met "Invalid local file header".
* Vast: lange documenten springen niet langer terug naar het begin terwijl een schermlezer er doorheen leest.
* Links in Web View brengen u nu naar de sectie waar ze naar wijzen, in plaats van te mislukken met "File not found".
* De automatische aankondiging "Document opnieuw geladen" onderbreekt uw schermlezer niet langer halverwege een zin, maar wacht tot het klaar is met spreken.
* Het tabblad Algemeen van het dialoogvenster Instellingen gaat nu door de opties in de volgorde waarin ze op het scherm verschijnen, met het update-kanaal direct na de optie Controleren op updates.
* Windows geeft nu altijd "Paperback" weer in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordentelling en Documentinfo geven nu aan hoeveel bestanden een audioboek bevat en hoe lang het totaal duurt.

### Versie 0.9.1
* Geluid van bladwijzers en notities speelt nu af op macOS.
* DAISY-boeken spelen hun audio nu af op macOS, in plaats van de tijdlijn stil te openen en bij te houden.
* Vast: krullige aanhalingstekens, em-dashes en soortgelijke tekens verdwijnen niet langer uit RTF-documenten, waarbij omringende woorden samenlopen.
* Vast: RTF-afbeeldingen lekken hun onbewerkte gegevens niet langer in het document als verwarde tekst.
* Vast: het submenu Recent geopend houdt verouderde items niet langer vast totdat iets anders het herbouwt.
* Toetsenbordversnellers zijn terug in elke vertaling, dus de menu's van Russisch hebben weer toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, dus ze verschijnen in de taakbalkspringlijst en de lijst met recente items van het Startmenu.
* Options is hernoemd naar Settings, wat overeenkomt met de mobiele apps en op macOS met de platformconventie.
* Paperback onthoudt nu zijn vensterositie, grootte en gemaximaliseerde status tussen uitvoeringen.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen, lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actiemenamen in het dialoogvenster Sneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken in de taakbalk en Alt+Tab kunnen worden onderscheiden.
* Het update-dialoog is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool genaamd pb om snel elk ondersteund formaat van Paperback naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie Bron weergeven om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown.
* Documenttekst is nu gepagineerd, wat betekent dat u boeken met tientallen miljoenen woorden nu in slechts een paar seconden kunt laden. Meld alles vreemds dat u hiermee vindt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een volledig scherm-wissel.

##### Dialoog Alle documenten
* Een zoekknop om ontbrekende boeken te zoeken die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat u kunt filteren op documentstatus en zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten uit te schakelen.

##### Opties en leesbaarheid
* Een leesbaarheid-tabblad met de volgende opties:
    * Woordterugloop (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor woordterugloop en daaropvolgende sneltoets.
* Een wissel om te bepalen hoe u tabellen wilt weergeven, en hoe tabellen consistent in documenten worden weergegeven.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met bladermodus in schermlezers.
* De sneltoets gelijkteken om uw huidige percentage in een document aan te geven.

##### Bladwijzers
* Tijdelijke bladwijzers: u kunt er één per document hebben, en ze blijven bestaan. Gebruik slash om er één in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het dialoogvenster voor woordentelling, plus de mogelijkheid om uw leessnelheid in te stellen om deze metriek werkelijk nuttig te maken.
* Als een selectie actief is wanneer u het dialoogvenster Woordentelling opent, ziet u nu hoeveel woorden u hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare sneltoets om Paperback uit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het exporteermenu-item uitgebreid om ook naar HTML en Markdown te exporteren, naast platte tekst.

##### Updater
* Een annuleringsknop naar het dialoogvenster voor update in uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gemanipuleerd.

##### Web View
* De webweergave wordt nu geopend op uw huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor afspelen van DAISY 2.02-audio.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zipbestanden met audiobestanden.
* Toetsenbordsneltoetsen en menu-items om voice over af te spelen/onderbreken, vooruit en achteruit te zoeken, en het zoekbereik aan te passen.
* Opties om de leescursor te synchroniseren met audioweergave, het audio-zoekbereik in te stellen en te kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Vastgesteld

##### Algemeen
* Documenten gecodeerd in legacy CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een heleboel mojibake.
* "Laatst gesloten opnieuw openen" probeert de gebundelde readme niet opnieuw te openen.
* Uw geselecteerde tabblad wordt na het opnieuw starten van Paperback niet correct gefocust.
* Paperback's verwerking van bestanden op Windows-netwerkstations: het drukken op bestand in map weergeven focust nu correct het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u om bevestiging gevraagd wanneer er een wordt gevonden.
* Map openen bevat nu het gegeven bestand in de verkenner.
* Het openen van de readme respecteert nu uw geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op beeldschermen met hoge DPI.
* Het menu wordt nu correct bijgewerkt en de focus verplaatst naar het tekstbesturingselement wanneer Help in Paperback wordt geopend.
* Overgeschakeld naar een veel veiliger IPC-methode op Windows.
* De titel van het actieve document wordt nu uitgesproken bij schakelen tussen tabbladen.
* Verminderd geheugengebruik bij grote documenten door de grootte van de interne per-character indexeertabellen te halveren.

##### Dialoog Alle documenten
* Escape sluit de dialoogvensters Documentinfo en Alle documenten niet.
* De titelbalk wordt niet bijgewerkt na het sluiten van een document in het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan uw lijst met alle documenten toegevoegd wanneer deze via Shift+F1 wordt geopend.
* Als u documenten uit het dialoogvenster Recents verwijdert, worden ook de actieve tabbladen ervan gesloten.
* Uw zoekfilter blijft nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigt in sommige situaties onjuiste regeltekst aan.
* Ga naar regel, Ga naar pagina en Ga naar percentage plaatsen uw cursor op de verkeerde plaats in grote documenten.
* Zoeken en Volgende zoeken respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Het geluid van bladwijzers/notities zou nu correct exclusief moeten afspelen wanneer u over een woord navigeert dat er één bevat.

##### Leesbaarheid
* Woordterugloop toepassen schiet u naar het begin van uw document.

##### Web View
* Het dialoogvenster voor webweergave kan niet worden vergroot en verschijnt in een zeer kleine initiële grootte.
* Afbeeldingen worden nu correct weergegeven in de ingesloten webweergave.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in release notes.

##### DAISY-boeken
* DAISY-boeken geven onjuiste info in de statusbalk.
* DAISY-boeken laden met bogus-coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten parseren met niet-Latijnse tekens erin.
* RTF `\pict` groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken splitsen HTML-tags en plaatsen rommel in de boektekst.
* Links in legacy Mobi-boeken.
* Aanzienlijk verbeterde AZW3-parsing.

##### Worddocumenten
* Worddocumenten met taalspecifieke stijlnamen geven hun koppen niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen produceren geen regelbreaks in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor onjuist gelabelde PDF's.
* PDF-documenten met besturingskarakters in hun titels en/of bladwijzers zullen Paperback niet langer crashen bij opening.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden oudere Word, moderne Word en moderne Powerpoint ondersteund, met oudere Powerpoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor legacy Microsoft Word-documenten (*.doc)!
* Ondersteuning toegevoegd voor legacy Powerpoint-presentaties (*.ppt)!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor gelabelde PDF-bestanden!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor ingepakte boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingebedde afbeeldingen wordt nu correct weergegeven.
* CHM-documenten ondersteunen nu correct interne linknavigatie.
* Vast: geluiden van bladwijzers activeren niet langer bij alinea-start in plaats van op de positie van de bladwijzer.
* Vast: naar pagina gaan was 1 uit.
* Vast: de Escape-toets werkt niet om het dialoogvenster Openen als te sluiten.
* Vast: het contextmenu van de lezer verschijnt niet bij rechts klikken of de toets Toepassingen.
* Vast: soms wordt het verkeerde document gefocust bij het openen van documenten vanaf de opdrachtregel.
* Alleen-afbeelding-PDF's worden opnieuw gedetecteerd en waarschuwen u voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback respecteert nu uw toepassingsinstellingen voor de donkere modus.
* Verwijderd DAISY XML-ondersteuning, omdat deze niet meer nodig is.
* Teruggescakeld naar de native Win32 eerste letter navigatie in de inhoudsopgave boom.
* Het dialoogvenster voor foutmeldingen toont nu meer gedetailleerde foutmeldingen.
* De webweergave wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Vast: een bug waarbij het openen van de webweergave in epub's met externe links deze automatisch zou activeren.
* Vast: een bug waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Vast: alinea's worden in sommige PDF-documenten in meerdere korte regels gesplitst.
* PDF-documenten hebben nu basale link- en koppelingnavigatie!
* RTF-tabbladen en regelinvoer worden nu exact weergegeven zoals ze in het document verschijnen.
* Teruggescakeld naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering veel betrouwbaarder is.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatste gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Enkele bugs met de RTF-parser vastgesteld.
* Vast: bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) werden beschadigd bij het openen van een bestand via een tweede Paperback-exemplaar.
* Vast: PDF-tekst werd in de verkeerde volgorde gelezen, met onjuiste spatiëring rond gekapitaliseerde woorden.
* Vast: traag laden van documenten bij het openen van grote bestanden.
* Vast: lokalisatie van Ja/Nee-knoppen in bevestigingsdialoogvensters.

### Versie 0.8.0
* Japanese, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die uw huidge geïnstalleerde versie van Paperback nu zal vervangen in plaats van de nieuwe versie alleen te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, dankzij Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor platte Open Document Text-bestanden!
* Ondersteuning toegevoegd voor platte Open Document-presentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging van meer dan 300 tekens voegt automatisch aan uw navigatiegeschiedenis toe.
* Vast: Paperback's venster herstellen vanuit het systeemvak.
* Vast: Markdown-documenten geven onbewerkte tekst weer in plaats van weergegeven HTML in Web View.
* Vast: tabellen renderen niet correct in Markdown-bestanden.
* Alleen-afbeelding-PDF's waarschuwen u nu voor hun bestaan wanneer u er een probeert te laden.
* Het is nu mogelijk om op nieuwe dev-builds te controleren in plaats van stabiele releases bij het controleren op updates.
* Versie-informatie correct ingebouwd in het Paperback-uitvoerbare bestand.
* Het optie-dialoogvenster in tabbladen opgesplitst voor gemak van gebruik en navigatie.
* Overgeschakeld naar Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app in Rust herschreven. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu specifieke acties voor lezers in plaats van algemene items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk Enter om er een in een webweergave te bekijken.
* Een basale webrenderfunctie toegevoegd! Druk op Ctrl+Shift+V om het huidige gedeelte van uw document in een webrenderer te openen, handig voor inhoud zoals complexe opmaak of codesamples.
* Een Russische vertaling toegevoegd, dank u Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het dialoogvenster Alle documenten.
* De updatecontrole geeft nu release notes weer wanneer een nieuwe versie beschikbaar is.
* Vast: het venster herstellen vanuit het systeemvak.
* Vast: Ja/Nee-knoppen vertalingen in bevestigingsdialoogvensters.
* Vast: configuraties laden bij het uitvoeren als beheerder.
* Vast: commentaarverwerking in XML- en HTML-documenten.
* Vast: TOC-parsing in Epub 2-boeken.
* Vast: naar het volgende item navigeren met dezelfde letter in de inhoudsopgave.
* Vast: het dialoogvenster Zoeken verbergt niet correct met de knoppen volgende/vorige.
* Vast: epub TOC's gooien u occasioneel naar het verkeerde item.
* Vast: verschillende problemen met witruimteafhandeling in XML, HTML en voortaggen.
* Vast: off-by-one-fout in linknavigatie.
* Vast: sommige boeken hebben achtervolgende witruimte op hun regels.
* Vast: verschillende parserfouten.
* Menu-items met betrekking tot bladwijzers en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstafhandeling in verschillende documentformaten.
* Verbeterde vertaalworkflow voor bijdragers.
* Veel interne refactors, waarbij het grootste deel van de bedrijfslogica van de toepassing van C++ naar Rust wordt verplaatst voor betere prestaties en onderhoud.

### Versie 0.6.1
* Ondersteuning voor PDF met wachtwoordbeveiliging toegevoegd!
* Een zeer basale functie voor navigatie naar vorige/volgende positie toegevoegd. Als u op een interne link drukt en deze uw cursor verplaatst, wordt die positie nu onthouden en kan er met Alt+Pijl naar links/rechts mee worden genavigeerd.
* Een elementenlijst toegevoegd! Op dit moment toont het alleen een boom van alle koppelingen in uw document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Vast: links in sommige Epub-documenten werken niet correct.
* Vast: Epub TOC's parseren met relatieve paden erin.
* Vast: sommige epub-documenten geven geen titel of auteur weer.
* Vast: de titels van sommige epub-hoofdstukken verschijnen niet correct in het TOC-dialoogvenster.
* Vast: u kunt de spatiebalk niet gebruiken om de OK/annuleer-knoppen in het TOC-dialoogvenster te activeren.
* Verbeterde verwerking van koppelingen in Worddocumenten.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u het dialoogvenster probeert op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het menu Ga naar in een veel compactere vorm weer te geven is toegevoegd aan het optiesdialoogvenster, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen te laten omwikkelen.
* Een optie toegevoegd aan het menu Extra om de beperkte map van het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basale slaaptimerfunctie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2-eboeken!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu een hele regel bladwijzeren of alleen bepaalde tekst markeren. Als u geen selectie actief hebt wanneer u een bladwijzer plaatst, gedraagt het zich als pre-0.6 en markeert het de hele regel. Als u echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities hebben! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het dialoogvenster Bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een vervelend "bladwijzer x" voorvoegsel.
* Epub-boeken die HTML-inhoud als XML voordoen, worden nu correct verwerkt.
* Vast: grote Markdown-documenten laden.
* Vast: spatiebalk indrukken in de inhoudsopgave boomweergave activeert de OK-knop.
* Vast: witruimteafhandeling aan het begin van voortaggen in HTML- en XHTML-documenten.
* Vast: het tekstbesturingselement krijgt soms niet de focus terug bij terugkeer naar Paperback's venster.
* Vast: het tekstveld in het dialoogvenster Ga naar percentage werkt niet correct met de schuifregeling.
* Vast: weergave van aangepaste HTML ID's in Markdown-documenten.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar draait, krijgt u niet langer een fout als het laden van uw document langer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks vanuit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand heeft de naam van het bestand met een .paperback-extensie. Als zo'n bestand in dezelfde directory als een bestand wordt gevonden bij het laden, wordt het automatisch geladen. Anders kunt u deze handmatig importeren met behulp van een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om voor- en achteruit door hen heen te bewegen, en druk Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller wordt en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorbewerkt om CommonMark-compatibel te zijn voordat deze wordt weergegeven.
* Navigatie op basis van lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om op basis van lijsten zelf te gaan, en I en Shift+I om door lijstitems te gaan.
* Numpad Delete werkt nu om documenten van de taakbalk te verwijderen, naast normale Delete.
* Paperback kan nu optioneel naar uw systeemvak minimaliseren! Deze optie is standaard uitgeschakeld, maar door deze in te schakelen, plaatst u Paperback in uw vak wanneer u op minimaliseren klikt, en kunt u deze herstellen door op het pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt, is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten geven nu een basale inhoudsopgave weer, met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinfo.
* Het installatieprogramma bevat nu een optie om de readme in uw browser te bekijken na installatie.
* De lijst met recente documenten is aanzienlijk uitgebreid! In plaats van alleen de laatste 10 geopende documenten weer te geven, toont het nu een aanpasbaar aantal, met de rest van de documenten die u ooit hebt geopend die toegankelijk zijn via een klein dialoogvenster.
* Verschillende kleine verbeteringen in de parsers over de hele linie, waaronder het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het oplossen van regeleinde afhandeling in alinea's in worddocumenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning toegevoegd voor PowerPoint-presentaties!
* Vast: bepaalde menu-items worden niet uitgeschakeld zonder geopende documenten.
* Vast: oriëntatie van de schuifregeling Ga naar percentage.
* Vast: inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragmentID's.
* Vast: witruimte wordt op vreemde manieren uit XHTML-koppelingen verwijderd.
* Vast: witruimteafhandeling in geneste voortaggen in HTML-documenten.
* HTML- en Markdown-documenten ondersteunen nu de functie Inhoudsopgave! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback een eigen inhoudsopgave op uit de structuur van de koppelingen in uw document en toont deze aan u in het dialoogvenster ctrl+t.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, indien aanwezig. Anders zullen ze de bestandsnaam zonder extensie blijven gebruiken.
* Overgeschakeld van UniversalSpeech naar het gebruik van een livegebied voor rapportage. Dit betekent dat er niet langer schermlezers-DLL's bij het programma zijn geïnstalleerd en meer schermlezers worden ondersteund, zoals Microsoft Narrator.
* Zipbibliotheken verwisseld om een breder scala aan epub-boeken te openen.
* Het dialoogvenster waarin u wordt gevraagd of u uw document als platte tekst wilt openen, is volledig opnieuw gemaakt en kan uw document nu als platte tekst, HTML of Markdown openen.
* Het dialoogvenster Ga naar percentage bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naartoe te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken blijft nu exact bewaard.
* De Unicode non-breaking space wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* U wordt niet langer elke keer dat u een onherkenbaar bestand laadt gevraagd hoe u het wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optionaal Startmenu-pictogram toegevoegd aan het installatieprogramma.
* De inhoudsopgave zou in enkele gevallen schoner moeten zijn, bijvoorbeeld als u een kind en ouderitem met dezelfde tekst op dezelfde positie hebt, ziet u nu alleen het ouderitem.
* Vast: inhoudsopgave in bepaalde CHM-documenten.
* Vast: inhoudsopgave in Epub 3-boeken met absolute paden erin.
* CHM-documenten moeten nu hun titel zoals ingesteld in het metagegevensbestand weergeven.

### Versie 0.4.0
* CHM-bestandsondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! U kunt zoveel bladwijzers hebben als u wilt in zoveel documenten als u wilt. U kunt ermee voor- en achteruit gaan met b en shift+b, er één instellen met control+shift+b, en een dialoogvenster openen om naar een specifieke bladwijzer te gaan met control+b.
* Een installatieprogramma toegevoegd naast het draagbare zipbestand! Het installatieprogramma installeert Paperback in uw map Programmabestanden en stelt automatisch bestandskoppelingen in.
* Tekstbestanden met BOM's worden nu correct gedecodeerd en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Het toont nu uw huidge regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer weergegeven in tekstuitvoer.
* Als u een relatief pad naar Paperback op de opdrachtregel doorgeeft, wordt dit nu correct opgelost.
* Percentagebeweging wordt nu verwerkt door zijn eigen schuifregelaardialoog, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor positieopslag is nu veel intelligenter en mag alleen naar schijf schrijven wanneer absoluut nodig.
* Het document waarop u zich hebt gericht toen u Paperback sloot, wordt nu onthouden in toepassingen die opnieuw worden opgestart.
* Invoer in de dialoogvensters Ga naar regel en Ga naar pagina moet nu strenger worden gedesinfecteerd.
* Vast: inhoudsopgavenavigatie in epub 3-boeken met relatieve paden in hun manifesten.

### Versie 0.3.0
* Vast: inhoudsopgave in epub-boeken met URL-gecodeerde manifesten.
* Vast: koppelingnavigatie in HTML-documenten met multi-byte Unicode-tekens.
* Vast: hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets.
* Vast: UTF-8 tekstbestanden laden.
* Vast: geneste TOC-items in Epub-boeken plaatsen uw cursor op de verkeerde plaats.
* Vast: een crash bij toepassingsafsluiting in bepaalde gevallen.
* Een selectievakje in het dialoogvenster Opties toegevoegd om woordterugloop in of uit te schakelen!
* Het is nu mogelijk om spatie in de inhoudsopgave boomweergave in te drukken om de OK-knop te activeren.
* Vast: witruimteafhandeling aan het begin van voortaggen in HTML- en XHTML-documenten.
* Vast: het tekstbesturingselement krijgt soms niet de focus terug bij terugkeer naar Paperback's venster.
* Vast: het tekstveld in het dialoogvenster Ga naar percentage werkt niet correct met de schuifregeling.
* Vast: weergave van aangepaste HTML ID's in Markdown-documenten.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Het is nu mogelijk om te doneren aan de ontwikkeling van Paperback, via het nieuwe doneer-item in het Help-menu of via de link Sponsor dit project aan de onderkant van de GitHub-repository's hoofdpagina.
* Markdown-documenten hebben nu altijd een titel en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* PDF-bibliotheken overgeschakeld naar de bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaarder PDF-parseren.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Als u paperback.exe met een bestandsnaam uitvoert terwijl het al wordt uitgevoerd, wordt dat document in het al draaiende exemplaar geopend.
* U kunt nu op Delete drukken op een document in het tabblad om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's toegevoegd aan het paginalabel in het dialoogvenster Ga naar pagina.
* Toestaan om van de documentinhoud naar uw lijst met geopende documenten te tabben.
* Vast: koppelingtoetsaanslagen openen soms recente documenten als u er genoeg van had.
* Paperback verwijdert nu onnodige zacht koppeltekens uit tekstuitvoer.
* Vast: koppelingnavigatie plaatst u soms op het verkeerde teken.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen toegevoegd voor navigatie via koppelingen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar met een schermlezer te werken.
* Vast: epub-bestanden laden met URL-gecodeerde bestandsnamen in hun manifesten.
* Vast: epub 3-boeken laden met XHTML ingebed erin.
* Een bericht wordt nu gesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Een menu met recente documenten toegevoegd! Het slaat momenteel uw laatste 10 geopende documenten op en het indrukken van Enter op een opent het om te lezen.
* Het dialoogvenster Zoeken volledig herschreven, wat het veel eenvoudiger maakt om te gebruiken, terwijl ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies wordt toegevoegd!
* Eerder geopende documenten worden nu onthouden in toepassingen die opnieuw worden opgestart. Dit kan worden geconfigureerd via het nieuwe item Opties in het menu Extra.
* Shift+F1 toegevoegd om de readme rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Initiële release.
