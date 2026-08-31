<!-- machine-translated from doc/readme.md (source-hash: 13c58fb50049f608); please review and edit as needed -->

# Paperback - versie 0.9.1

## Inleiding

Paperback is een lichte, snelle en toegankelijke ebook- en documentlezer voor iedereen, van casual lezers tot ervaren power users. Het is ontworpen voor schermlezer-toegankelijkheid, snelheid en een ervaring zonder onnodige ballast.

## Systeemvereisten

Paperback draait momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testbuilds gepland kort na de 0.9.0 desktoprelease, voorafgaand aan een uniforme 1.0-release voor alle vier platforms.

## Functies

* Volledig zelfstandig, zonder dat u software op uw computer hoeft te installeren om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op in elk document dat u opent.
* Onthoud optioneel welke documenten u open had toen u het programma sloot en herstelt deze bij de volgende start.
* Bevat navigatiefunctionaliteit vergelijkbaar met die in de webbrowsermodus van veel schermlezers om snel en eenvoudig door documenten te navigeren.
* Bevat een robuuste zoekdialoog met functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of worden geïnstalleerd met bestandskoppelingen die automatisch worden ingesteld.
* Ondersteunt een enorm aantal veelgebruikte bestandsindelingen.

## Schermlezer-compatibiliteit

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en braille-displays

Als u JAWS met een braille-display gebruikt, kan het gebeuren dat lange alinea's worden afgekapt wanneer u vooruit bladert met de navigatietoetsen van uw display. Ook de opdracht huidige alinea lezen wordt beïnvloed. Dit is een bug in JAWS's verwerking van het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf, en iets waarvoor het behoorlijk lang duurde om een oplossing aan het licht te brengen gezien Vispero's enthousiasme voor het reageren op problemen met open source-software.

De omweg, uiteindelijk naar voren gebracht via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw display op de actieve alinea staan in plaats van vooruit te gaan. Met beide instellingen op zijn plaats zou het bladeren correct moeten werken.

## Momenteel ondersteunde bestandstypen

Paperback ondersteunt de volgende indelingen en extensies:

* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2-ebooks (`.fb2`)
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

## Toetsenbordsnelkoppelingen

Paperback is ontworpen voor gebruik met toetsenbord voorop. Hier volgen de huidige snelkoppelingen.

Snelkoppelingen hieronder zijn voor Windows. Waar macOS afwijkt, wordt het equivalent in haakjes vermeld — vooral omdat Ctrl+G, Ctrl+W en Alt+Left/Right al in beslag genomen zijn door andere systeemconventies of app-conventies op dat platform.

### Bestandsmenu

* `Ctrl+O`: Document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Huidig document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Laatst gesloten document opnieuw openen.
* `Ctrl+R`: Dialoogvenster "Alle documenten" weergeven (van Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in het app-menu).

### Menu Gaan naar

* `Ctrl+F`: Dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Ga naar regel.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ga naar percentage.
* `Ctrl+P`: Ga naar pagina (indien ondersteund door het huidige document).
* `=`: Uw huidige leespercentage mededelen.
* `Alt+Left` (macOS: `Cmd+[`): Teruggaan in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruitgaan in navigatiegeschiedenis.
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
* `Shift+N`: Vorige opmerking.
* `N`: Volgende opmerking.
* `Ctrl+B`: Naar alle bladwijzers en opmerkingen springen.
* `Ctrl+Alt+B`: Alleen naar bladwijzers springen.
* `Ctrl+Alt+M`: Alleen naar opmerkingen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Opmerkingstekst op de huidige positie weergeven.
* `Shift+K`: Vorige koppeling.
* `K`: Volgende koppeling.
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

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Woordentelling voor het huidige document weergeven.
* `Ctrl+I`: Documentinfo weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Map met inhoud openen.
* `Ctrl+Shift+V`: Huidige inhoud in Webweergave openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Huidig document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer bij huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzeropmerking bij huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Tekstomloop in-/uitschakelen.
* `Ctrl+Space`: Audionarratief afspelen/pauzeren.
* `'`: Audionarratief vooruit spoelen.
* `;`: Audionarratief terug spoelen.
* `Ctrl+'`: Audiosoekeringhoeveelheid vergroten.
* `Ctrl+;`: Audiosoekeringhoeveelheid verkleinen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d.w.z. Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, onder het app-menu).
* `Ctrl+Shift+S`: Slaaptimerschakelaar in-/uitschakelen.

### Menu Help

* `Ctrl+F1`: Dialoogvenster Over weergeven.
* `F1`: Help in uw standaardbrowser weergeven.
* `Shift+F1`: Help in Paperback weergeven.
* `Ctrl+Shift+U`: Op updates controleren.
* `Ctrl+D`: Donatatiepagina in uw standaardbrowser openen.

### Aanvullende documentweergavesneltoetsen

* `Delete` / `Numpad Delete` op het tabtabbladelement: Geselecteerde documenttab sluiten.
* `Enter` of `Space` in de documenttekst: Koppeling op cursor activeren, of een tabelweergave openen wanneer u zich op een tabelmarkering bevindt.
* `Shift+F10` of de Menu/Toepassingstoets in de documenttekst: Contextmenu openen.

## Ondersteunde talen

Paperback is in veel verschillende talen vertaald en er worden voortdurend meer toegevoegd. Hieronder volgt een volledige lijst.

Wil je weten hoe je kunt bijdragen, lees dan onze [Vertaalhandleiding](translating.md).

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
* Aryan Choudhary: primaire medewerker.

### Donaties
De volgende personen hebben donaties van enige omvang gedaan aan de ontwikkeling van Paperback. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd; ik voeg alleen personen toe die hun donatie openbaar willen maken.

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

## Wijzigingslogboek

### Versie 0.9.2
* Audioboeken zorgen niet langer ervoor dat je schermlezer een reeks spaties voorleest wanneer je de tekstveld focust.
* Audioboeken benoemen nu het bestand als je er doorheen gaat per sectie.
* Audioboeken geven nu hun werkelijke lengte aan, in plaats van te claimen dat elk bestand 24 uur duurt.
* Het sluiten van de Web View met Escape geeft geen debugwaarschuwing meer nadat je een link erin hebt gevolgd.
* Kopiëren na Select All geeft je nu het hele document, in plaats van alleen het gedeelte dat momenteel is geladen.
* Find gaat nu rechtstreeks naar de regel die het heeft gevonden, in plaats van je door de schermlezer te laten horen terwijl de focus naar het boek terugkeert.
* Vaste EPUB's met een zwervend ZIP64-blok die weigeren te openen met "Invalid local file header".
* Lange documenten die terugliepen naar hun begin terwijl een schermlezer doorlopend door ze heen las, zijn nu opgelost.
* Links in de WebView brengen je nu naar de sectie waarnaar ze verwijzen, in plaats van te mislukken met "File not found".
* De automatische "Document reloaded"-aankondiging onderbreekt je schermlezer niet langer halverwege een zin, maar wacht tot deze klaar is met spreken.
* Het tabblad Algemeen van de instellingendialoog loopt nu op logische volgorde door de opties, met het updatekanaal direct na de optie voor updatecontrole.
* Windows toont nu altijd "Paperback" in het menu "Openen met", in plaats van de volledige tagline van het programma.
* Word Count en Document Info tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Versie 0.9.1
* Bladwijzer- en notitiegeluiden worden nu afgespeeld op macOS.
* DAISY-boeken spelen nu hun audio af op macOS, in plaats van hun tijdlijn in stilte te openen en te volgen.
* Krulhaakjes, emtekens en vergelijkbare tekens verdwijnen niet meer uit RTF-documenten en voegen omringende woorden niet meer samen.
* RTF-afbeeldingen lekken niet langer hun onbewerkte gegevens als verminkt tekstdocument uit.
* Het recente documenten-submenu behoudt niet langer verouderde vermeldingen totdat iets anders het opnieuw opbouwt.
* Toetsenbordaccelerators zijn terug in elke vertaling, zodat Russische menu's opnieuw toetsenbordtoegang hebben.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, dus ze verschijnen in de spraaklist van de taakbalk en de recente lijst van het Startmenu.
* Options is hernoemd naar Settings, wat overeenkomt met de mobiele apps en, op macOS, de platformconventie.
* Paperback onthoudt nu zijn vensterposition, grootte en gemaximaliseerde toestand tussen sessies.
* Meervoudsvormen zijn nu vertaald, zodat berichten die dingen tellen correct in talen worden gelezen die meer dan één vorm nodig hebben.
* Het selecteren van het ncc.html-bestand van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in de dialoog Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat open boeken in de taakbalk en Alt+Tab uit elkaar kunnen worden gehouden.
* De updatedialoog is nu vertaald.

### Versie 0.9.0

#### Toevoegingen

##### Algemeen
* Een CLI-tool, genaamd pb, om snel elk ondersteund formaat van Paperback naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie View Source om de bron van een document in een nieuw tabblad te openen, handig voor het bewerken van Markdown bijvoorbeeld.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alles wat raar is.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een wissel voor volledig scherm.

##### Dialoog Alle documenten
* Een knop om ontbrekende boeken te lokaliseren die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat je kunt filteren op documentstatus en zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten af te selecteren.

##### Opties en Leesbaarheid
* Een tabblad Leesbaarheid met de volgende opties:
    * Tekstterugloop (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor tekstterugloop en een daaropvolgende sneltoets.
* Een schakelaar om te bepalen hoe je tabellen wilt weergeven en hoe tabellen in documenten geuniformeerd worden weergegeven.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met de browse mode in schermlezers.
* De sneltoets equals om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben en deze blijven bestaan. Gebruik schuine streep om er één in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in de dialoog woordentelling, plus de mogelijkheid om je leessnelheid in te stellen om deze maatstaf werkelijk nuttig te maken.
* Als er een selectie actief is wanneer je de dialoog woordentelling opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke sneltoets in de app aan te passen via een eenvoudige dialoog.
* Een configureerbare sneltoets om Paperback uit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren uitgebreid om naar HTML en Markdown te exporteren, naast platte tekst.

##### Updater
* Een knop Annuleren voor de dialoog update-in-uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Web View
* De webview wordt nu geopend op je huidige leeспositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02-audioafspeling.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zips van audiobestanden.
* Sneltoetsen en menu-items om naratie af te spelen/pauzeren, vooruit en achteruit te zoeken en de zoekduur aan te passen.
* Opties om de leescursor te synchroniseren met audioafspeling, de audiosoekhoeveelheid in te stellen en te kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Verholpen

##### Algemeen
* Documenten gecodeerd in oudere CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een reeks mojibake.
* "Laatst gesloten opnieuw openen" probeert het gebundelde readme opnieuw te openen.
* Je geselecteerde tabblad niet correct geplaatst na herstart van Paperback.
* Paperback's verwerking van bestanden op Windows-netwerkstations: het indrukken van bestand in map weergeven focust nu correct het bestand op de netwerkopslag en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer forcerend geladen bij documentherstel; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* Map openen bevat nu het gegeven bestand in verkenner.
* Het openen van het readme respecteert nu je geselecteerde taal.
* De gebruikersinterface van Paperback wordt nu correct geschaald op displays met hoge DPI.
* Het menu wordt nu correct bijgewerkt en de focus verplaatst zich naar de tekstbesturing wanneer je help in Paperback opent.
* Overgegaan op een veel veiliger methode van IPC op Windows.
* De titel van het actieve document wordt nu gelezen bij het schakelen tussen tabbladen.
* Gereduceerd geheugengebruik op grote documenten door de interne indexeertabellen per teken tot de helft te verkleinen.

##### Dialoog Alle documenten
* Escape sluit de dialogen Document Info en All Documents niet.
* De titelbalk wordt niet bijgewerkt nadat een document uit de dialoog alle documenten is gesloten.
* Readme.html wordt niet langer aan je lijst met alle documenten toegevoegd wanneer deze wordt geopend via Shift+F1.
* Het verwijderen van documenten uit de dialoog recent gebruikt opent nu ook hun actieve tabblad.
* Je zoekfilter blijft nu behouden nadat je een document hebt verwijderd.

##### Navigatie
* Paginavigatie die in sommige situaties onjuiste regeltekst aankondigt.
* Go to Line, Go to Page en Go to Percent plaatsen je cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren het geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Bladwijzer-/notitiegeluiden moeten nu uitsluitend correct afspelen wanneer je over een woord met een ervan navigeert.

##### Leesbaarheid
* Tekstterugloop toepassen schiet je naar het begin van je document.

##### Web View
* De webview-dialoog is niet wijzigbaar en verschijnt met een erg kleine initiële grootte.
* Afbeeldingen moeten nu correct in de ingebedde webview worden weergegeven.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in opmerkingen over releases.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten met niet-Latijnse tekens parseren.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en rommel in de boektekst zetten.
* Links in verouderde Mobi-boeken.
* Aanzienlijk verbeterde AZW3-parsing.

##### Word-documenten
* Word-documenten met landinstelling-specifieke stijlnamen die hun koppen niet correct weergeven.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen die geen regelafbrekingen produceren in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op extractie van platte tekst voor onjuist gelabelde PDF's.
* PDF-documenten met controletekens in hun titels en/of bladwijzers crashen Paperback niet meer wanneer deze worden geopend.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden oudere Word, moderne Word en moderne Powerpoint ondersteund, met oudere Powerpoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor verouderde Microsoft Word-documenten!
* Ondersteuning toegevoegd voor verouderde Powerpoint-presentaties!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor ingepakte boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingebedde afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct navigatie via interne links.
* Ga naar pagina dat met 1 afwezig is, opgelost.
* De Escape-sleutel werkt niet om de dialoog Openen als te sluiten, opgelost.
* Het contextmenu van de lezer verschijnt niet op rechtklik of de toets Toepassingen, opgelost.
* Het verkeerde document wordt soms geplaatst wanneer documenten via de opdrachtregel worden geopend, opgelost.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen je voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback respecteert nu je instellingen voor de modus Donker app.
* DAISY XML-ondersteuning verwijderd, omdat het niet langer nodig is.
* Teruggeschakeld naar de native Win32-navigatie met eerste letter in de inhoudsopgave-boom.
* Het dialoogvenster voor laadfouten toont nu meer gedetailleerde foutberichten.
* De webview wordt nu veel sneller en soepeler geopend.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Een bug verholpen waarbij het openen van de webview in epubs met externe links deze automatisch zou activeren.
* Een bug verholpen waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's die in sommige PDF-documenten in meerdere korte regels werden opgesplitst, opgelost.
* PDF-documenten hebben nu basisondersteuning voor link- en koppelingnavigatie!
* RTF-tabbladen en regeleindes worden nu exact weergegeven zoals ze in het document voorkomen.
* Teruggeschakeld naar de beproefde pdfium-bibliotheek voor PDF-parsing, wat PDF-weergave veel betrouwbaarder maakt.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs in de RTF-parser verholpen.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) die beschadigd raken bij het openen van een bestand via een tweede Paperback-instantie, opgelost.
* PDF-tekst die in de verkeerde volgorde wordt gelezen en onjuiste afstand rond gekapitaliseerde woorden, opgelost.
* Trage documentlading bij het openen van grote bestanden, opgelost.
* De lokalisatie van de knoppen Ja/Nee in bevestigingsdialogen, opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je huidigeInstallatie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionaal geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of een notitie, bedankt Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor Flat Open Document Text-bestanden!
* Ondersteuning toegevoegd voor Flat Open Document-presentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan je navigatiegeschiedenis.
* Paperback uit het systeemvak herstellen verholpen.
* Markdown-documenten tonen onbewerkte tekst in plaats van weergegeven HTML in de Web View, opgelost.
* Tabellen renderen niet correct in Markdown-bestanden, opgelost.
* PDF's met alleen afbeeldingen waarschuwen je nu voor hun bestaan wanneer je een laadt.
* Versioninformatie correct in het Paperback-uitvoerbare bestand ingebed.
* De instellingendialoog in tabbladen verdeeld voor gemak en navigatie.
* Overgegaan op Hayro voor PDF-parsing, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De gehele app in Rust herschreven. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van de tekstbesturing bevat nu lezerspécifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T en druk op Enter om er een in een webview weer te geven.
* Een basale webrenderfunctie toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, bedankt Ruslan Gulmagomedov!
* Een knop Clear All toegevoegd aan het dialoogvenster All Documents.
* De updatecontrole toont nu opmerkingen over releases wanneer een nieuwe versie beschikbaar is.
* Venster uit het systeemvak herstellen opgelost.
* Ja/Nee-knopvertalingen in bevestigingsdialogen opgelost.
* Configs laden bij uitvoering als beheerder opgelost.
* Opmerkingenverwerking in XML- en HTML-documenten opgelost.
* TOC-parsing in Epub 2-boeken opgelost.
* Navigatie naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* Het dialoogvenster Zoeken verbergt niet correct bij gebruik van de knoppen volgende/vorige, opgelost.
* EPUB TOC's gooien je soms naar het verkeerde item, opgelost.
* Verschillende problemen met witruimteverwerking in XML-, HTML- en pre-tags opgelost.
* Fout met één verschuiving in linknavigatie opgelost.
* Sommige boeken met navolgende witruimte op hun regels opgelost.
* Verschillende parserfouten opgelost.
* Menu-items met betrekking tot bladwijzers en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstverwerking in verschillende documentindelingen.
* Verbeterde workflowvertaling voor bijdragers.
* Veel interne refactors, het verplaatsen van het merendeel van de zakelijke logica van de toepassing van C++ naar Rust voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Wachtwoordbeveiligde PDF-ondersteuning toegevoegd!
* Een zeer basale functie voor navigatie naar vorige/volgende positie toegevoegd. Als je op een interne link drukt en het verplaatst je cursor, zal die positie nu worden onthouden en kan er naar worden genavigeerd met alt+pijl links/rechts.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle kopjes in je document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard gemaximaliseerd te starten.
* Links in sommige EPUB-documenten werken niet correct, opgelost.
* EPUB TOC's met relatieve paden parseren, opgelost.
* Sommige epub-documenten tonen geen titel of auteur, opgelost.
* De titels van sommige epub-hoofdstukken verschijnen niet correct in de TOC-dialoog, opgelost.
* Je kunt de spatiebalk niet gebruiken om de OK/annuleer-knoppen in het TOC-dialoog in te schakelen, opgelost.
* Verbeterde verwerking van kopjes in Word-documenten.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je probeert de dialoog op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het menu "Gaan naar" in een veel compactere vorm weer te geven, is toegevoegd aan het dialoogvenster Opties en is standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structuuralementen om te wikkelen.
* Een optie toegevoegd aan het menu Hulpmiddelen om de bevattingmap van het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatersysteem toegevoegd.
* Een basale functie voor slaaptimer, toegankelijk met Ctrl+Shift+S, toegevoegd.
* Ondersteuning toegevoegd voor het parseren van FB2-ebooks!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu worden gebruikt om een volledige regel aan te duiden of alleen bepaalde tekst aan te duiden. Als je geen selectie actief hebt bij het plaatsen van een bladwijzer, is het gedrag hetzelfde als voor 0.6, en het markeert de hele regel. Als je echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bijgevoegd hebben! Navigeer tussen bladwijzers met notities met N en Shift+N, of open de bladwijzerdialoog met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een vervelend "bookmark x"-voorvoegsel.
* EPUB-boeken met HTML-inhoud die zich voordoen als XML worden nu correct verwerkt.
* Grote Markdown-documenten laden, opgelost.
* Spatiebalk indrukken in de inhoudsopgave-boomweergave activeert de knop OK, opgelost.
* Witruimteverwerking aan het begin van pre-tags in zowel HTML- als XHTML-documenten, opgelost.
* Tekstbesturing krijgt focus niet terug wanneer je terugkeert naar het Paperback-venster, opgelost.
* Het tekstveld in het dialoogvenster "Naar procent gaan" werkt de schuifregelaarwaarde niet bij, opgelost.
* Weergave van aangepaste HTML-ID's in Markdown-documenten, opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek met een opdrachtregelparameter laadt terwijl een bestaande Paperback-instantie wordt uitgevoerd, krijg je geen fout meer als het laden van je document langer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks uit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om je bladwijzers en leesspositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand heet naar het bestand met een .paperback-extensie. Als zo'n bestand in dezelfde map als een bestand wordt gevonden wanneer deze wordt geladen, wordt dit automatisch geladen. Anders kun je ze handmatig importeren met een item in het menu Hulpmiddelen.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om vooruit en achteruit tussen hen te bewegen en druk op Enter om er een te openen/activeren.
* Veel interne refactors, het sneller maken van de app en het binaire bestand kleiner.
* Markdown-inhoud wordt nu voorverwerkt om CommonMark-conform te zijn voordat deze wordt weergegeven.
* Navigatie door lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om door de lijsten zelf te gaan en I en Shift+I om door lijstitems te gaan.
* Numpad Delete werkt nu ook om documenten uit de tabbalk te verwijderen naast normale Delete.
* Paperback kan nu optioneel naar je systeemvak minimaliseren! Deze optie staat standaard uit, maar door deze in te schakelen, plaatst de minimaliseeroption in het systeemmenu Paperback in je vak en kan deze worden hersteld door op het gemaakte pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt, is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentgegevens.
* Het installatieprogramma bevat nu een optie om het readme na de installatie in je browser weer te geven.
* De lijst met recente documenten is enorm uitgebreid! In plaats van alleen de laatste 10 documenten weer te geven die je hebt geopend, worden nu een aanpasbaar aantal weergegeven, met de rest van de documenten die je ooit hebt geopend toegankelijk via een klein dialoogvenster.
* Verschillende kleine verbeteringen in de parsers over het hele bord, inclusief het plaatsen van een blanco regel tussen dia's in PPTX-presentaties, het repareren van de regelafhandelingshandling binnen alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Microsoft Word-documentondersteuning toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden niet uitgeschakeld zonder geopende documenten, opgelost.
* De oriëntatie van de schuifregelaar "Ga naar procent", opgelost.
* De inhoudsopgave in EPUB-boeken met URL-gecodeerde bestandspaden en/of fragmentID's, opgelost.
* Witruimte wordt op vreemde manieren uit XHTML-koppen verwijderd, opgelost.
* Witruimteverwerking in geneste pre-tags in HTML-documenten, opgelost.
* HTML- en Markdown-documenten ondersteunen nu de functie Inhoudsopgave! Wanneer je een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave uit de structuur van de kopjes in je document en toont deze aan je in het dialoogvenster ctrl+t.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, als deze bestaat. Anders blijven zij de bestandsnaam zonder extensie gebruiken.
* Van UniversalSpeech overgeschakeld naar het gebruik van een live-regio voor spraakaankondiging. Dit betekent dat geen schermlezer-DLL's meer naast het programma worden geleverd en dat meer schermlezers nu worden ondersteund, zoals Microsoft Narrator.
* Gezoomd bibliotheeken om een breder scala van epub-boeken te openen.
* Het dialoogvenster met de vraag of je je document als platte tekst wilt openen is volledig opnieuw gemaakt en je kunt je document nu als platte tekst, HTML of Markdown openen.
* Het dialoogvenster "Ga naar procent" bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naar te springen.
* De HTML-parser zal nu dd, dt en dl als lijstelementen herkennen.
* De inhoudsopgave in EPUB-boeken blijft nu exact behouden.
* De unicode-spatie die niet breekt, wordt nu in aanmerking genomen bij het verwijderen van lege regels.
* Je wordt niet langer gevraagd hoe je een niet-herkend bestand wilt openen telkens wanneer je het laadt, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenupictogram toegevoegd aan het installatieprogramma.
* De inhoudsopgave zou in een paar gevallen schoner moeten zijn, bijvoorbeeld als je een kind- en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten, opgelost.
* De inhoudsopgave in EPUB 3-boeken met absolute paden erin, opgelost.
* CHM-documenten moeten nu hun titel weergeven zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* CHM-bestandsondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers in zoveel documenten hebben als je wilt. Je kunt er met b en shift+b doorheen springen, er een met control+shift+b instellen en met control+b een dialoogvenster openen om naar een specifieke bladwijzer te springen.
* Een installatieprogramma toegevoegd naast het draagbare zip-bestand! Het installatieprogramma zal Paperback in uw Program Files-map installeren en bestandskoppelingen automatisch instellen.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd en de BOM wordt niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Het toont nu je huidige regel, karakter en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer in tekstuitvoer weergegeven.
* Bij het doorgeven van een relatief pad aan Paperback op de opdrachtregel, zal het nu correct worden opgelost.
* Percentagebeweging wordt nu afgehandeld door zijn eigen schuifregelaardialoog, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaardinstelling.
* De logica voor positiebesparing is nu veel slimmer en mag alleen naar schijf schrijven wanneer absoluut noodzakelijk.
* Het document waarop je je richtte toen je Paperback sloot, wordt nu onthouden tussen toepassingsherstarts.
* Invoer in de dialoogvensters "Ga naar regel" en "Ga naar pagina" moet nu strikter worden gereinigd.
* Vaste navigatie in inhoudsopgave in epub 3-boeken met relatieve paden in hun manifesten.

### Versie 0.3.0
* De inhoudsopgave in epub-boeken met URL-gecodeerde manifesten, opgelost.
* Koppelingnavigatie in HTML-documenten met multibyte Unicode-tekens, opgelost.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets, opgelost.
* UTF-8-tekstbestanden laden, opgelost.
* Geneste TOC-items in EPUB-boeken die je cursor op de verkeerde positie zetten, opgelost.
* Een crash bij afsluiting van de toepassing in bepaalde gevallen, opgelost.
* Een selectievakje in het dialoogvenster Opties toegevoegd om tekstterugloop in of uit te schakelen!
* Het is nu mogelijk om aan de ontwikkeling van Paperback bij te dragen via het nieuwe donate-item in het Help-menu of via de link "Sponsor this project" aan de onderkant van de GitHub-opslagplaats.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Overgegaan op de PDF-bibliotheek die wordt gebruikt in Chromium, wat leidt tot veel betrouwbaardere PDF-parsing over het hele bord.
* Je kunt nu slechts één instantie van Paperback tegelijk uitvoeren. Als je paperback.exe met een bestandsnaam uitvoert terwijl deze al wordt uitgevoerd, opent u dat document in de reeds actieve instantie.
* Je kunt nu Delete op een document in het tabbladebesturingselement drukken om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's aan het paginalabel in het dialoogvenster "Ga naar pagina" toegevoegd.
* Tabblad van documentinhoud naar je lijst met geopende documenten toestaan.
* De sneltoetsen voor titels openen soms recente documenten als je er genoeg van had, opgelost.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit tekstuitvoer.
* Koppelingnavigatie plaats je soms op het verkeerde teken, opgelost.

### Versie 0.2.0
* Markdown-documentondersteuning toegevoegd!
* PDF-documentondersteuning toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen toegevoegd voor navigatie per koppelingen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar met een schermlezer te werken.
* EPUB's laden met URL-gecodeerde bestandsnamen in hun manifesten, opgelost.
* EPUB 3-boeken laden met XHTML erin ingebed, opgelost.
* Er wordt nu een bericht gesproken als het document geen inhoudsopgave of secties ondersteunt, in tegenstelling tot het uitschakelen van menu-items.
* Een menu met recente documenten toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op en als je op een drukt, opent u deze ter lezen.
* Het dialoogvenster Zoeken volledig herschreven, waardoor het veel eenvoudiger in gebruik is, terwijl ook een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies wordt toegevoegd!
* Eerder geopende documenten worden nu onthouden tussen toepassingsherstarts. Dit is configureerbaar via het nieuwe optie-item in het menu Hulpmiddelen.
* Shift+F1 toegevoegd om het readme rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Initiale release.
