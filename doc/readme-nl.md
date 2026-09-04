<!-- machine-translated from doc/readme.md (source-hash: efe922e94821c70e); please review and edit as needed -->

# Paperback - versie 0.9.2

## Introductie

Paperback is een lichte, snelle en toegankelijke ebook- en documentlezer voor iedereen, van casual lezers tot geavanceerde gebruikers. Het is ontworpen voor toegankelijkheid van schermlezers, snelheid en een onbesmette ervaring.

## Systeemvereisten

Paperback draait momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Systeemeigen iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testversies gepland kort na de 0.9.0 desktoprelease, vóór een uniforme 1.0-release die alle vier platforms bestrijkt.

## Functies

* Volledig zelfstandig, waarvoor geen software op uw computer hoeft te worden geïnstalleerd om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op in elk document dat u opent.
* Onthoudt optioneel welke documenten u open had toen u het programma sloot en herstelt deze bij de volgende keer dat u het start.
* Bevat navigatiefunctionaliteit die lijkt op die in de webbrowsingmodus van veel schermlezers om snel en gemakkelijk door documenten te navigeren.
* Bevat een robuust zoekdialoogvenster, inclusief functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd, of geïnstalleerd met automatisch ingestelde bestandskoppelingen.
* Ondersteunt een enorm aantal veel voorkomende bestandsindelingen.

## Compatibiliteit schermlezers

Paperback werkt goed met alle grote schermlezers. Er is echter een bekend probleem voor JAWS-gebruikers.

### JAWS en brailleweergaven

Als u JAWS met een brailleweergave gebruikt, merkt u mogelijk dat lange alinea's worden afgekapt wanneer u vooruit bladert met de navigatietoetsen van uw weergave. De opdracht voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een fout in de verwerking van JAWS van het RICHEDIT50W-besturingselement, niet iets in Paperback zelf, en een fout die nogal wat tijd nodig had om een oplossing aan het licht te brengen gezien Vispero's enthousiasme voor het reageren op problemen met open-sourcesoftware.

De oplossing, uiteindelijk aan het licht gebracht via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw weergave op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen op zijn plaats zou bladeren correct moeten werken.

## Momenteel ondersteunde bestandstypen

Paperback ondersteunt de volgende indelingen en extensies:

* CHM-helpbestanden (`.chm`)
* DAISY-boeken (`.opf`, `.zip`)
* EPUB-boeken (`.epub`)
* FB2 eboeken (`.fb2`)
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

Paperback is ontworpen voor toetsenbordgebruik. Hier zijn de huidige snelkoppelingen.

De snelkoppelingen hieronder zijn voor Windows. Waar macOS afwijkt, wordt het equivalent tussen haakjes vermeld — vooral omdat Ctrl+G, Ctrl+W en Alt+Left/Right al gereserveerd zijn door andere systeem- of app-conventies op dat platform.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle open documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document heropenen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (uit Recent Documents).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in het app-menu).

### Menu Gaan

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Ga naar regel.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ga naar procent.
* `Ctrl+P`: Ga naar pagina (indien ondersteund door het huidige document).
* `=`: Uw huidige leespercentage aankondigen.
* `Alt+Left` (macOS: `Cmd+[`): Ga terug in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Ga vooruit in navigatiegeschiedenis.
* `[`: Vorige gedeelte.
* `]`: Volgende gedeelte.
* `Shift+H`: Vorige kop.
* `H`: Volgende kop.
* `Shift+1` tot en met `Shift+6`: Vorige kop op niveau 1-6.
* `1` tot en met `6`: Volgende kop op niveau 1-6.
* `Shift+P`: Vorige pagina.
* `P`: Volgende pagina.
* `Shift+B`: Vorige bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Stel uw tijdelijke bladwijzer in.
* `\`: Spring naar uw tijdelijke bladwijzer.
* `Shift+N`: Vorige opmerking.
* `N`: Volgende opmerking.
* `Ctrl+B`: Ga naar alle bladwijzers en opmerkingen.
* `Ctrl+Alt+B`: Ga naar alleen bladwijzers.
* `Ctrl+Alt+M`: Ga naar alleen opmerkingen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dus de fysieke Control-toets in plaats van Cmd): Opmerkingtekst op de huidige positie weergeven.
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

### Menu Gereedschappen

* `Ctrl+W` (macOS: `RawCtrl+W`, dus de fysieke Control-toets in plaats van Cmd): Aantal woorden voor het huidige document weergeven.
* `Ctrl+I`: Documentinformatie weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Bevattende map openen.
* `Ctrl+Shift+V`: Huidige inhoud in webweergave openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad bekijken.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzeropmerkingen op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Regelterugloop in-/uitschakelen.
* `Ctrl+Space`: Audiovertelkunde afspelen/pauzeren.
* `'`: Audiovertelkunde vooruitspelen.
* `;`: Audiovertelkunde terugspoelen.
* `Ctrl+'`: Het bedrag voor audiospelen vergroten.
* `Ctrl+;`: Het bedrag voor audiospelen verkleinen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dus Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.

### Help-menu

* `Ctrl+F1`: Het dialoogvenster Over weergeven.
* `F1`: Help in uw standaardbrowser bekijken.
* `Shift+F1`: Help in Paperback bekijken.
* `Ctrl+Shift+U`: Op updates controleren.
* `Ctrl+D`: De donatiepagina in uw standaardbrowser openen.

### Aanvullende documentweergavetoetsen

* `Delete` / `Numpad Delete` op het tabbladbesturingselement: Geselecteerd documenttabblad sluiten.
* `Enter` of `Space` in documenttekst: Activeer koppeling op cursor, of open een tabelweergave wanneer u op een tabelmarker staat.
* `Shift+F10` of de toets Menu/Toepassing in documenttekst: Het contextmenu openen.

## Ondersteunde talen

Paperback is in veel verschillende talen vertaald, en er worden voortdurend meer toegevoegd. Hieronder volgt een volledige lijst.

PLEASE read our [Translation Guide](translating.md) om te leren hoe u kunt bijdragen.

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
* Aryan Choudhary: primaire contributor.

### Donaties
De volgende personen hebben donaties ter ondersteuning van Paperback-ontwikkeling gedaan. Als u een donatie doet, zal uw naam niet automatisch hier worden toegevoegd. Ik voeg alleen personen toe die willen dat hun donatie openbaar wordt gemaakt.

Opmerking: Ik beschouw een openbare GitHub-sponsor als automatische reden voor opname in deze lijst.

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
* Audioboeken zorgen niet langer voor het voorlezen van reeksen spaties door je schermleezer wanneer je het tekstveld focust.
* Audioboeken noemen nu de bestandsnaam wanneer je er doorheen gaat per sectie.
* Audioboeken geven nu hun werkelijke lengte aan, in plaats van te claimen dat elk bestand 24 uur duurt.
* Het sluiten van de Web View met Escape toont niet langer een debug-waarschuwing nadat je een link erin hebt gevolgd.
* Kopiëren na Select All geeft nu het gehele document, in plaats van alleen het deel dat momenteel is geladen.
* Find springt nu direct naar de regel die het gevonden heeft, in plaats van je door de schermleezer te laten wachten terwijl deze het venster opnieuw voordraagt wanneer de focus terugkeert naar het boek.
* Fixed EPUB's die een zwevend ZIP64-blok dragen en weigeren te openen met "Invalid local file header".
* Fixed lange documenten die terugkeren naar het begin terwijl een schermleezer ze continu voordraagt.
* Links in de WebView brengen je nu naar de sectie waar ze naar verwijzen, in plaats van te mislukken met "File not found".
* De automatische "Document herladen"-aankondiging onderbreekt je schermleezer niet langer midden in een zin, maar wacht tot deze klaar is met spreken.
* Het tabblad Algemeen van het dialoogvenster Instellingen tabuleert nu door de opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor het controleren op updates.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Word Count en Document Info tonen nu hoeveel bestanden een audioboek bevat en hoe lang het in totaal duurt.

### Version 0.9.1
* Geluiden van bladwijzers en notities worden nu afgespeeld op macOS.
* DAISY-boeken spelen nu hun audio op macOS af, in plaats van hun tijdlijn in stilte te openen en bij te houden.
* Fixed krulhaakjes, getallen en soortgelijke karakters die verdwijnen uit RTF-documenten, waarbij de omringende woorden samensmelten.
* Fixed RTF-afbeeldingen die hun onbewerkte gegevens als verminkte tekst in het document lekken.
* Fixed het menu Recente documenten dat verouderde vermeldingen bewaart tot iets anders gebeurt om het opnieuw op te bouwen.
* Toetsenbordacceleratoren zijn teruggekeerd in elke vertaling, dus Russische menu's hebben weer toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten worden nu geregistreerd bij Windows, dus ze verschijnen in de sprong lijst van de taakbalk en de recente lijst van het Startmenu.
* Options is hernoemd naar Settings, wat overeenkomt met de mobiele apps en op macOS de platformconventie.
* Paperback onthoudt nu zijn vensterpostie, grootte en gemaximaliseerde status tussen runs.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van het ncc.html-bestand van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* Actienamen in het dialoogvenster Toetsenbordsnelkoppelingen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, dus geopende boeken kunnen in de taakbalk en Alt+Tab van elkaar onderscheiden worden.
* Het updatedialoogvenster is nu vertaald.

### Version 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-hulpprogramma, genaamd pb, om snel elk ondersteund formaat van Paperback naar HTML, Markdown of gewone tekst te converteren.
* Een optie om documenten opnieuw in te laden die door andere programma's op schijf zijn gewijzigd.
* Een View Source-optie om de bron van een document in een nieuw tabblad te openen, handig voor bijvoorbeeld het bewerken van Markdown.
* Documenttekst is nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alles wat vreemd is.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een fullscreen-schakelaar.

##### Dialoog Alle documenten
* Een locate-knop om ontbrekende boeken te vinden die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat je op documentstatus kunt filteren en zien hoeveel documenten worden weergegeven en geselecteerd.
* De `Ctrl+Shift+A`-snelkoppeling om alle documenten uit te selecteren.

##### Opties en leesbaarheid
* Een leesbaarheidstabblad met de volgende opties:
    * Woordombreking (verplaatst van algemeen);
    * Tabellen inline renderen (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alineaafstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor woordombreking en daaropvolgende hotkey.
* Een schakelaar om te bepalen hoe je tabellen weergegeven wilt hebben en geïntegreerde manier waarop tabellen in documenten worden weergegeven.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met bladeringsmodus in schermlezers.
* De toetsencombinatie = om je huidige percentage door een document aan te geven.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en deze blijven bestaan. Gebruik slash om er één in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leesstijd in het dialoogvenster woordentelling, evenals de mogelijkheid om je leessnelheid in te stellen om deze metriek werkelijk nuttig te maken.
* Als een selectie actief is wanneer je het dialoogvenster woordentelling opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Toetsenbordsnelkoppelingen
* De mogelijkheid om elke toetsenbordsnelkoppeling in de app aan te passen via een eenvoudig dialoogvenster.
* Een instelbare toetsenbordsnelkoppeling om Paperback vanuit het systeemvak terug te zetten.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item Exporteren is uitgebreid om exporteren naar HTML en Markdown mogelijk te maken, naast gewone tekst.

##### Updater
* Een cancelknop naar het dialoogvenster voor updates die in uitvoering zijn.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Web View
* De webview wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02-audioweergave.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zip-bestanden met audiobestanden.
* Toetsenbordsnelkoppelingen en menu-items voor het afspelen/pauzeren van vertellingen, vooruit en achteruit zoeken, en het aanpassen van de zoekbereik.
* Opties voor het synchroniseren van de lees-cursor met audioweergave, het instellen van de zoekbereik voor audio en het kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lsteditems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten gecodeerd in legacy CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een stel mojibake.
* "Reopen last closed" probeert de gebundelde readme opnieuw te openen.
* Je geselecteerde tabblad niet correct focussen na het herstarten van Paperback.
* Paperbacks afhandeling van bestanden op Windows-netwerkstations: het drukken op show file in folder focust nu correct het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde karakters.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt je om bevestiging gevraagd wanneer er een wordt gevonden.
* Open containing folder focust nu het gegeven bestand in explorer.
* Het openen van de readme respecteert nu je geselecteerde taal.
* De gebruikersinterface van Paperback schaalt nu correct op schermen met hoge DPI.
* Het menu werkt nu correct bij en de focus gaat naar het tekstbesturingselement wanneer Help in Paperback wordt geopend.
* Gewijzigd naar een veel veiligere IPC-methode op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het schakelen tussen tabbladen.
* Verminderd geheugengebruik op grote documenten door de grootte van de interne per-character-indextabellen te halveren.

##### Dialoog Alle documenten
* Escape sluit niet het dialoog Document Info en All Documents.
* De titelbalk werkt niet bij na het sluiten van een document in het dialoog all documents.
* Readme.html zal niet langer aan je list All Documents worden toegevoegd wanneer geopend via Shift+F1.
* Het verwijderen van documenten uit het recents-dialoog sluit nu ook hun actieve tabblad.
* Je zoekfilter blijft nu behouden na het verwijderen van een document.

##### Navigatie
* Paginavigatie kondigt in bepaalde situaties onjuiste lijntekst aan.
* Go to Line, Go to Page en Go to Percent plaatsen je cursor op de verkeerde positie in grote documenten.
* Find en Find Next respecteren niet het geladen documentvenster in grote documenten.

##### Bladwijzers
* Geluiden van bladwijzers/notities worden nu correct en exclusief afgespeeld wanneer je over een woord navigeert dat een bevat.

##### Leesbaarheid
* Het toepassen van woordombreking schiet je naar het begin van je document.

##### Web View
* Het webview-dialoog kan niet worden aangepast en verschijnt met een zeer kleine initiële grootte.
* Afbeeldingen worden nu correct weergegeven in de ingebedde webview.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste info in de statusbalk.
* DAISY-boeken laden met onechte coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten parseren met niet-Latijnse karakters erin.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en rotzooi in de boektekst plaatsen.
* Links in legacy Mobi-boeken.
* Aanzienlijk verbeterde AZW3-parsing.

##### Word-documenten
* Word-documenten met taalspecifieke stijlnamen die hun koppen niet correct weergaven.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen die geen regelafbrekingen produceren in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor onjuist getagde PDF's.
* PDF-documenten met besturingstekens in hun titels en/of bladwijzers veroorzaken niet langer een crash van Paperback bij het openen.

### Version 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden legacy Word, moderne Word en moderne Powerpoint ondersteund, waarbij legacy Powerpoint voor de toekomst is gepland.
* Ondersteuning toegevoegd voor legacy Microsoft Word-documenten!
* Ondersteuning toegevoegd voor legacy Powerpoint-presentaties!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De snelkoppeling ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor gezipt boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingesloten afbeeldingen wordt nu correct weergegeven.
* CHM-documenten ondersteunen nu correct de navigatie van interne links.
* Fixed go to page dat 1 afweek.
* Fixed de escape-toets die niet werkte om het dialoog "open als" te sluiten.
* Fixed het contextmenu van de lezer dat niet verscheen bij het rechtsklikken of de Applications-toets.
* Fixed het verkeerde document dat soms gefocust werd bij het openen van documenten vanaf de opdrachtregel.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen je voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g en f/shift+f.
* Paperback respecteert nu je toepassingsdarkmode-instelling.
* DAISY XML-ondersteuning verwijderd, omdat het niet langer nodig is.
* Teruggekeerd naar de native Win32-navigatie met de eerste letter in de table of contents-tree.
* Het dialoog foutladen toont nu meer gedetailleerde foutberichten.
* De webview opent nu veel sneller en soepeler.

### Version 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Fixed een bug waarbij het openen van de webview in epubs met externe links deze automatisch zou activeren.
* Fixed een bug waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Fixed alinea's die in sommige PDF-documenten in meerdere korte regels werden gesplitst.
* PDF-documenten hebben nu basis- link- en koppelingnavigatie!
* RTF-tabs en regelinvoer worden nu exact weergegeven zoals ze in het document verschijnen.
* Teruggekeerd naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, wat de PDF-rendering veel betrouwbaarder maakt.

### Version 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoog All Documents ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Fixed een paar bugs met de RTF-parser.
* Fixed bestandspaden met niet-ASCII-karakters (zoals Bosnisch š, č, ć, ž) die beschadigd raken wanneer een bestand via een tweede Paperback-instantie wordt geopend.
* Fixed PDF-tekst die in de verkeerde volgorde wordt gelezen en onjuiste spatiëring rond gekapitaliseerde woorden.
* Fixed traag documentladen bij het openen van grote bestanden.
* Fixed de lokalisatie van de Ja/Nee-knoppen in bevestigingsdialogen.

### Version 0.8.0
* Japanse, vereenvoudigde Chinees en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je huidige Paperback-versie vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, dank aan Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor Flat Open Document Text-bestanden!
* Ondersteuning toegevoegd voor Flat Open Document-presentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan je navigatiegeschiedenis.
* Fixed het herstellen van Paperback's venster vanuit het systeemvak.
* Fixed Markdown-documenten die onbewerkte tekst in plaats van weergegeven HTML in de Web View weergaven.
* Fixed tabellen die niet correct in Markdown-bestanden werden weergegeven.
* PDF's met alleen afbeeldingen waarschuwen je nu voor hun bestaan wanneer je er een probeert te laden.
* Versie-informatie correct in het Paperback-bestand ingebed.
* Het dialoog Opties in tabbladen opgesplitst voor gebruiksgemak en navigatie.
* Overgeschakeld naar Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app herschreven in Rust. De nieuwe codebasis is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Version 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T en druk op Enter om er een in een webview te bekijken.
* Een basisfunctie voor webweergave toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Russische vertaling toegevoegd, dank aan Ruslan Gulmagomedov!
* Een Clear All-knop toegevoegd aan het dialoog All Documents.
* De updatecontroller geeft nu releaseopmerkingen weer wanneer een nieuwe versie beschikbaar is.
* Fixed het herstellen van het venster vanuit het systeemvak.
* Fixed ja/nee-knopvertalingen in bevestigingsdialogen.
* Fixed het laden van configs bij uitvoering als beheerder.
* Fixed commentaarverwerking in XML- en HTML-documenten.
* Fixed TOC-parsing in Epub 2-boeken.
* Fixed navigatie naar het volgende item met dezelfde letter in de table of contents.
* Fixed het Find-dialoog niet correct verbergen wanneer de volgende/vorige knoppen worden gebruikt.
* Fixed epub TOC's die je soms naar het verkeerde item brengen.
* Fixed verschillende witruimteafhandelingsproblemen in XML, HTML en pre-tags.
* Fixed off-by-one fout in linknavigatie.
* Fixed enkele boeken met spaties aan het einde van hun regels.
* Fixed verschillende parseerproblemen.
* Menu-items met betrekking tot bladwijzers en de elementenlijst zijn nu correct uitgeschakeld wanneer geen document open is.
* Verbeterde lijstverwerking in verschillende documentformaten.
* Verbeterde workflow voor vertalingen voor medewerkers.
* Veel interne refactors, waarbij het grootste deel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor verbeterde prestaties en onderhoudbaarheid.

### Version 0.6.1
* Wachtwoordbeveiligd PDF-ondersteuning toegevoegd!
* Een zeer basisfunctie voor het navigeren naar vorige/volgende positie toegevoegd. Als je op Enter drukt op een interne link en het verplaatst je cursor, zal die positie nu worden onthouden en kan ermee naar worden genavigeerd met alt+left/right pijlen.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle koppelingen in je document of een lijst met links, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard gemaximaliseerd te starten.
* Fixed links in bepaalde Epub-documenten die niet correct werkten.
* Fixed het parseren van Epub TOC's met relatieve paden erin.
* Fixed enkele epub-documenten die geen titel of auteur weergaven.
* Fixed de titels van bepaalde epub-hoofdstukken die niet correct in het TOC-dialoog verschenen.
* Fixed dat je de spacebar niet kon gebruiken om de OK/cancel-knoppen in het TOC-dialoog te activeren.
* Verbeterde afhandeling van koppelingen in Word-documenten.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je het dialoog probeert weer te geven.

### Version 0.6.0
* Een nieuwe optie om het go-menu in een veel compactere vorm weer te geven is toegevoegd aan het dialoog Opties, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen te omspannen.
* Een optie aan het menu Tools toegevoegd om de map die het momenteel gefocuste document bevat te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een basisfunctie voor de slaaptimer toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2 ebooks!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu een hele regel als bladwijzer markeren, of alleen bepaalde tekst markeren. Als je geen selectie actief hebt wanneer je een bladwijzer plaatst, is het gedrag zoals vóór 0.6 en markeert het de hele regel. Als je echter bepaalde tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities hebben! Navigeer tussen bladwijzers met notities met N en Shift+N, of pop het bladwijzerdialoog op met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke hotkeys.
* Bladwijzers in het bladwijzerdialoog hebben niet langer een vervelend "bladwijzer x"-voorvoegsel.
* Epub-boeken die HTML-inhoud voorwenden te zijn XML worden nu correct afgehandeld.
* Fixed het laden van grote Markdown-documenten.
* Fixed het drukken van spatie in de table of contents-boom die de OK-knop activeert.
* Fixed witruimteafhandeling aan het begin van pre-tags in zowel HTML als XHTML-documenten.
* Fixed het tekstveld dat soms niet opnieuw gefocust krijgt wanneer terugkeert naar het venster van Paperback.
* Fixed het tekstveld in het dialoog go to percent dat de waarde van de schuifregelaar niet bijwerkt.
* Fixed de weergave van aangepaste HTML-id's in Markdown-documenten.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek met een opdrachtregelparameter laadt terwijl een bestaande Paperback-instantie wordt uitgevoerd, krijg je niet langer een fout als het laden van je document meer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks in het bladwijzerdialoog te verwijderen.
* Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald document in en uit te voeren. Het gegenereerde bestand wordt genoemd naar het bestand met een .paperback-extensie. Als zo'n bestand in dezelfde map als een bestand wordt gevonden terwijl het wordt geladen, zal het automatisch worden geladen. Anders kun je ze handmatig importeren met behulp van een item in het menu Tools.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om vooruit en achteruit door ze heen te gaan, en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner is gemaakt.
* Markdown-inhoud wordt nu vooraf verwerkt om CommonMark-compatibel te zijn voordat deze wordt weergegeven.
* Navigatie per lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om door lijsten zelf te gaan, en I en Shift+I om door lstitems te gaan.
* Numpad delete werkt nu ook om documenten uit de tabbalk te verwijderen naast normale verwijdering.
* Paperback kan nu optioneel minimaliseren naar je systeemvak! Deze optie staat standaard uit, maar als je deze inschakelt, wordt de minimaliseeroptie in het systeemmenu Paperback in je vak geplaatst, kunnen het hersteld worden door op het spawn-pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basisinhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoog Document Info.
* Het installatieprogramma bevat nu een optie om de readme na de installatie in je browser weer te geven.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van eenvoudig de laatste 10 geopende documenten weer te geven, worden nu een aanpasbaar aantal weergegeven, met de rest van de documenten die je ooit hebt geopend via een klein dialoog.
* Verschillende kleine verbeteringen in de parsers over het algemeen, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het repareren van de regelafbrekingsafhandeling in alinea's in Word-documenten en het toevoegen van opsommingstekens aan lstitems.

### Version 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Fixed bepaalde menu-items niet worden uitgeschakeld zonder geopende documenten.
* Fixed de oriëntatie van de go to percent-schuifregelaar.
* Fixed de table of contents in Epub-boeken met URL-gecodeerde bestandspaden en/of fragment-ID's.
* Fixed witruimte die op vreemde manieren uit XHTML-koppelingen wordt verwijderd.
* Fixed witruimteafhandeling in geneste pre-tags in HTML-documenten.
* HTML- en Markdown-documenten ondersteunen nu de table of contents-functie! Wanneer je een HTML/Markdown-document laadt, zal Paperback zijn eigen table of contents uit de structuur van de koppelingen in je document bouwen en deze in het ctrl+t-dialoog weergeven.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, als deze bestaat. Anders zullen ze de bestandsnaam zonder extensie blijven gebruiken.
* Overgeschakeld van UniversalSpeech naar het gebruik van een livegebied voor het rapporteren van spraak. Dit betekent dat er geen schermleezer-DLL's meer naast het programma worden verzonden, en meer schermlezers worden nu ondersteund, zoals Microsoft Narrator.
* Gewijzigd naar zip-bibliotheken om een breder scala aan epub-boeken te openen.
* Het dialoog dat je vraagt of je je document als gewone tekst wilt openen, is volledig opnieuw gedaan en staat je nu toe om je document als gewone tekst, HTML of Markdown te openen.
* Het dialoog go to percent bevat nu een tekstveld waarmee je handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als lstelementen.
* De table of contents in Epub-boeken blijft nu exact behouden.
* De Unicode-not-breaking space wordt nu in overweging genomen wanneer lege regels worden verwijderd.
* Je wordt niet meer gevraagd hoe je een onbekend bestand wilt openen elke keer dat je het laadt, alleen de eerste keer.

### Version 0.4.1
* Een optioneel pictogram in het Startmenu toegevoegd aan het installatieprogramma.
* De table of contents zou nu in enkele gevallen schoner moeten zijn, bijvoorbeeld als je een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* Fixed de table of contents in bepaalde CHM-documenten.
* Fixed de table of contents in Epub 3-boeken met absolute paden erin.
* CHM-documenten zouden nu hun titel moeten weergeven zoals ingesteld in het metagegevensbestand.

### Version 0.4.0
* CHM-bestandsondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! Je kunt zoveel bladwijzers hebben als je wilt in zoveel documenten als je wilt. Je kunt met b en shift+b vooruit en achteruit erdoorheen springen, er een instellen met control+shift+b en een dialoog om naar een specifieke bladwijzer te springen met control+b.
* Een installatieprogramma toegevoegd naast het draagbare zip-bestand! Het installatieprogramma installeert Paperback in je Program Files-map en stelt automatisch bestandskoppelingen voor je in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd en het BOM zal niet langer aan het begin van de tekst worden weergegeven.
* Veel meer informatie aan de statusbalk toegevoegd. Het toont nu je huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet meer weergegeven in tekstuitvoer.
* Als je een relatief pad op de opdrachtregel naar Paperback doorgeeft, zal het dit nu correct oplossen.
* Percentagebewegingen worden nu afgehandeld door hun eigen schuifregelaar-dialoog, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs zullen nu altijd een standaard hebben.
* De logica voor positioneringbesparing is nu veel slimmer en schrijft alleen naar de schijf wanneer dit absoluut nodig is.
* Het document waarop je je concentreerde toen je Paperback sloot, wordt nu onthouden tussen het herstarten van toepassingen.
* Invoer in de dialogen go to line en go to page moet nu strenger zijn geverifieerd.
* Fixed table of contents-navigatie in epub 3-boeken met relatieve paden in hun manifesten.

### Version 0.3.0
* Fixed de table of contents in epub-boeken met URL-gecodeerde manifesten.
* Fixed koppelingnavigatie in HTML-documenten met multi-byte Unicode-karakters.
* Fixed hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets.
* Fixed het laden van UTF-8-tekstbestanden.
* Fixed geneste TOC-items in Epub-boeken die je cursor op de verkeerde positie plaatsen.
* Fixed een crash bij het afsluiten van toepassingen in bepaalde gevallen.
* Een selectievakje in het dialoog Opties toegevoegd om woordombreking in of uit te schakelen!
* Het is nu mogelijk om naar de ontwikkeling van Paperback te doneren, via het nieuwe donatieitem in het menu Help of via de sponsor this project-link onderaan de hoofdpagina van de GitHub-opslagplaats.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* Gewijzigd naar de PDF-bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaarder PDF-parsing over het hele bord.
* Je kunt nu alleen één instantie van Paperback tegelijk laten uitvoeren. Het uitvoeren van paperback.exe met een bestandsnaam terwijl dit al wordt uitgevoerd, opent dat document in de al uitvoerende instantie.
* Je kunt nu op verwijderen drukken op een document op het tabbediening om het te sluiten.

### Version 0.2.1
* Het totale aantal pagina's toegevoegd aan het paginalisering in het dialoog go to page.
* Tabblad van documentinhoud naar je lijst geopende documenten toestaan.
* Fixed de koppelingtoetsencombinaties soms het openen van recente documenten als je genoeg van hen had.
* Paperback zal nu onnodige zachte afbreekstreepjes uit de tekstuitvoer verwijderen.
* Fixed koppelingnavigatie die je soms op het verkeerde teken plaatst.

### Version 0.2.0
* Markdown-documentondersteuning toegevoegd!
* PDF-documentondersteuning toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Toetsencombinaties voor navigatie per koppelingen in HTML-inhoud, inclusief epub-boeken en markdown-documenten. Deze toetsencombinaties zijn ontworpen om vergelijkbaar te werken met een schermleezer.
* Fixed het laden van epubs met URL-gecodeerde bestandsnamen in hun manifesten.
* Fixed het laden van epub 3-boeken met XHTML erin.
* Er wordt nu een bericht gesproken als het document geen table of contents of secties ondersteunt, in tegenstelling tot de menu-items die worden uitgeschakeld.
* Een menu met recente documenten toegevoegd! Het slaat momenteel je laatste 10 geopende documenten op, en het drukken op een daarvan opent het voor lezen.
* Het Find-dialoog volledig herschreven, waardoor het veel eenvoudiger te gebruiken is, terwijl ook een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere uitdrukkingen wordt toegevoegd!
* Eerder geopende documenten worden nu onthouden tussen het herstarten van toepassingen. Dit is configureerbaar via het nieuwe item Opties in het menu Tools.
* Shift+F1 toegevoegd om de readme rechtstreeks in Paperback zelf te openen.

### Version 0.1.0
* Initiële release.
