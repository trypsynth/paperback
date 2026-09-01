<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc); please review and edit as needed -->

# Paperback - versie 0.9.2

## Introductie

Paperback is een lichte, snelle en toegankelijke ebook- en documentlezer voor iedereen, van casual lezers tot krachtige gebruikers. Het is ontworpen voor schermlezer-toegankelijkheid, snelheid en een bloatware-vrije ervaring.

## Systeemvereisten

Paperback draait momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testversies gepland kort na de 0.9.0 desktoprelease, voorafgaand aan een uniforme 1.0-release die alle vier platforms dekt.

## Functies

* Volledig zelfstandig, waarvoor geen software op uw computer hoeft te worden geïnstalleerd om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op voor elk document dat u opent.
* Kan optioneel onthouden welke documenten u had geopend toen u het programma sloot, en herstelt deze bij de volgende start.
* Bevat navigatiefunctionaliteit vergelijkbaar met die in de webbrowsingmodus van veel schermlezers voor snelle en gemakkelijke navigatie door documenten.
* Bevat een robuuste zoekdialoog, inclusief functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of geïnstalleerd met bestandskoppellingen die automatisch worden ingesteld.
* Ondersteunt een enorme reeks gangbare bestandsindelingen.

## Compatibiliteit schermlezer

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en brailleweergaven

Als u JAWS met een brailleweergave gebruikt, merkt u mogelijk dat lange alinea's worden afgekapt wanneer u vooruit bladert met de navigatietoetsen van uw weergave. Het commando voor het lezen van de huidige alinea wordt ook beïnvloed. Dit is een bug in JAWS's handling van het RICHEDIT50W-tekstbesturingselement, niet iets in Paperback zelf, en iets wat behoorlijk lang duurde om een fix voor uit te brengen gezien Vispero's enthousiasme voor het reageren op problemen met open source-software.

De workaround, uiteindelijk aan het licht gebracht via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw weergave op de actieve alinea staan in plaats van vooruit te gaan. Met beide instellingen op hun plaats zou bladeren correct moeten werken.

## Momenteel ondersteunde bestandstypen

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

## Toetsenbordsnelkoppelingen

Paperback is ontworpen voor gebruik met het toetsenbord voorop. Hier zijn de huidige snelkoppelingen.

De snelkoppelingen hieronder zijn voor Windows. Waar macOS afwijkt, wordt het equivalent tussen haakjes vermeld — vooral omdat Ctrl+G, Ctrl+W en Alt+Left/Right al door andere systeem- of app-conventies op dat platform in beslag worden genomen.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (uit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS is dit in plaats daarvan in het app-menu).

### Menu Gaan

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (als ondersteund door het huidige document).
* `=`: Uw huidige leespercentage aankondigen.
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
* `Shift+B`: Vorig bladwijzer.
* `B`: Volgende bladwijzer.
* `/`: Uw tijdelijke bladwijzer instellen.
* `\`: Naar uw tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Naar bladwijzers alleen springen.
* `Ctrl+Alt+M`: Naar notities alleen springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Notitietekst op de huidige positie weergeven.
* `Shift+K`: Vorige link.
* `K`: Volgende link.
* `Shift+G`: Vorig afbeelding.
* `G`: Volgende afbeelding.
* `Shift+F`: Vorig figuur.
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

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, d.w.z. de fysieke Control-toets in plaats van Cmd): Woordaantal voor het huidige document weergeven.
* `Ctrl+I`: Documentinformatie weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Map openen die het document bevat.
* `Ctrl+Shift+V`: Huidige inhoud openen in webweergave.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document naar platte tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernotitie op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Woordomloop in-/uitschakelen.
* `Ctrl+Space`: Audio-vertelling afspelen/onderbreken.
* `'`: Audio-vertelling vooruitspoelen.
* `;`: Audio-vertelling terugspoelen.
* `Ctrl+'`: De audio-zoekbedrag verhogen.
* `Ctrl+;`: De audio-zoekbedrag verlagen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, d.w.z. Control+Command+F): Volledig scherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, onder het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.

### Menu Help

* `Ctrl+F1`: Dialoogvenster Over weergeven.
* `F1`: Help in uw standaardbrowser weergeven.
* `Shift+F1`: Help in Paperback weergeven.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: De donatiepagina in uw standaardbrowser openen.

### Aanvullende document-weergavetoetsen

* `Delete` / `Numpad Delete` op het tabbereik: Het geselecteerde document-tabblad sluiten.
* `Enter` of `Space` in de documenttekst: Link bij cursor activeren, of een tabelweergave openen wanneer u zich op een tabelmarkering bevindt.
* `Shift+F10` of de Menu/Toepassingstoets in de documenttekst: Het contextmenu openen.

## Ondersteunde talen

Paperback is vertaald in veel verschillende talen, en er worden voortdurend meer toegevoegd. Een volledige lijst volgt hieronder.

Wil je weten hoe je kunt bijdragen, lees dan onze [Vertaalgids](translating.md).

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
* Quin Gillespie: primaire ontwikkelaar en oprichter van het project.
* Aryan Choudhary: primaire bijdrager.

### Donaties
De volgende personen hebben een donatie van enige omvang aan de Paperback-ontwikkeling gedaan. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd. Ik voeg alleen personen toe die willen dat hun donatie openbaar wordt gemaakt.

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
* Audioboeken doen uw schermlezer niet langer een reeks spaties voorlezen wanneer u de tekstveld focust.
* Audioboeken noemen nu de bestandsnaam wanneer u erdoorheen stapt per sectie.
* Audioboeken rapporteren nu hun werkelijke lengte, in plaats van te beweren dat elk bestand erin 24 uur duurt.
* Het sluiten van de Web View met Escape veroorzaakt niet langer een debug-waarschuwing nadat u een link erin hebt gevolgd.
* Kopiëren na Select All geeft u nu het hele document, in plaats van alleen het gedeelte dat momenteel is geladen.
* Zoeken gaat nu rechtstreeks naar de regel die het heeft gevonden, in plaats van u door uw schermlezer te laten zitten wachten terwijl focus terugkeert naar het boek.
* Vaste EPUB's die een stray ZIP64-blok bevatten en weigerden te openen met "Invalid local file header".
* Vaste lange documenten die naar het begin terugliepen terwijl een schermlezer continu erdoorheen las.
* Links in de WebView brengen u nu naar de sectie waar ze naartoe wijzen, in plaats van te mislukken met "File not found".
* De automatische "Document opnieuw geladen"-aankondiging onderbreekt uw schermlezer niet langer midden in een zin, maar wacht tot het klaar is met spreken.
* Het tabblad Algemeen van het dialoogvenster Instellingen bladert nu door de opties in de volgorde waarin ze op het scherm verschijnen, met het updatekanaal direct na de optie voor updatecontrole.
* Windows toont nu altijd "Paperback" in het menu Openen met, in plaats van de volledige tagline van het programma.
* Woordentelling en Documentinfo tonen nu hoeveel bestanden een audioboek bevat en hoe lang het totaal duurt.

### Versie 0.9.1
* Bladwijzer- en notitiemeldingen spelen nu af op macOS.
* DAISY-boeken spelen nu hun audio af op macOS, in plaats van om hun tijdlijn te openen en bij te houden in stilte.
* Vaste krulhaakjes, emdashs en vergelijkbare tekens verdwenen uit RTF-documenten en voegden de omringende woorden samen.
* Vaste RTF-afbeeldingen lekten hun ruwe gegevens in het document als verminkte tekst.
* Vaste submenu Recente documenten die verouderde invoeren bijhield tot er iets anders gebeurde om deze opnieuw op te bouwen.
* Toetsenbordaccelerators zijn terug in elke vertaling, dus Russische menu's hebben opnieuw toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten zijn nu geregistreerd bij Windows, dus ze verschijnen in de sprong-lijstbalk en in de lijst met recente items van het startmenu.
* Opties is gewijzigd in Instellingen, wat aansluit bij mobiele apps en op macOS de platformconventie.
* Paperback onthoudt nu zijn vensterpositie, -grootte en gemaximaliseerde status tussen sessies.
* Meervoudsvormen zijn nu vertaald, dus berichten die dingen tellen, lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in het dialoogvenster Toetsenbordsnelkoppelingen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, dus geopende boeken kunnen in de taakbalk en Alt+Tab onderscheiden worden.
* Het updatedialoogvenster is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-hulpmiddel, genaamd pb, om snel elk van Paperback's ondersteunde formaten naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw te laden die op schijf door andere programma's zijn gewijzigd.
* Een optie View Source om de bron van een document in een nieuw tabblad te openen, handig bijvoorbeeld voor het bewerken van Markdown.
* Documenttekst is nu gepagineerd, wat betekent dat u boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Gelieve vreemdheid die hiermee wordt gevonden te rapporteren.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Native macOS-ondersteuning!
* Een schakelaar voor volledig scherm.

##### Dialoogvenster Alle documenten
* Een locatieknop om vermiste boeken te lokaliseren die zojuist hun pad hebben gewijzigd.
* Een statusfilter en statusbalk, zodat u kunt filteren op documentstatus en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De snelkoppeling `Ctrl+Shift+A` om alle documenten uit te schakelen.

##### Opties en leesbaarheid
* Een tabblad Leesbaarheid met de volgende opties:
    * Woordomloop (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterspaçiëring;
    * Tekstuitlijning.
* Een menu-item voor woordomloop en daaropvolgende sneltoets.
* Een schakelaar om te bepalen hoe u tabellen wilt weergegeven, en hoe tabellen worden weergegeven in documenten.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met browse-modus in schermlezersl.
* De sneltoets gelijkteken om uw huidige percentage door een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: u kunt er één per document hebben, en deze worden bewaard. Gebruik slash om er een in te stellen en backslash om naar het springe.

##### Woordentelling
* Geschatte leestijd in het dialoogvenster woordentelling, evenals de mogelijkheid om uw leessnelheid in te stellen om deze metrische waarde werkelijk nuttig te maken.
* Als een selectie actief is wanneer u het woordentellingsvenster opent, wordt nu weergegeven hoeveel woorden u hebt geselecteerd.

##### Toetsenbordsnelkoppelingen
* De mogelijkheid om elke toetsenbordsnelkoppeling in de app aan te passen via een eenvoudig dialoogvenster.
* Een configureerbare toetsenbordsnelkoppeling om Paperback uit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item exporteren is uitgebreid om exporteren naar HTML en Markdown mogelijk te maken, naast platte tekst.

##### Updater
* Een annuleringsknop voor het dialoogvenster voor updates in uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is gewijzigd.

##### Web View
* De webweergave wordt nu geopend op uw huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02 audiospeler.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als ZIP-bestanden met audiobestanden.
* Toetsenbordsnelkoppelingen en menu-items om narration af te spelen/pauzeren, vooruit en achteruit te zoeken en de zoekvolume aan te passen.
* Opties voor het synchroniseren van de leesetwcursor met audiospeler, het instellen van de zoekvolume en het kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Gerepareerd

##### Algemeen
* Documenten gecodeerd in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een bos mojibake.
* "Reopen last closed" probeert het gebundelde leesmij opnieuw te openen.
* Uw geselecteerde tabblad dat niet correct werd gefocust na het opnieuw starten van Paperback.
* Paperback's verwerking van bestanden op Windows-netwerkstations: het indrukken van bestand in map weergeven focust nu correct het bestand op de netwerkopslag, en de paden bevatten niet langer vreemde tekens.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u om bevestiging gevraagd wanneer er een wordt gevonden.
* Open containing folder focust nu het gegeven bestand in explorer.
* Het openen van het leesmij respecteert nu uw geselecteerde taal.
* Paperback's gebruikersinterface schalen nu correct op high-DPI-displays.
* Het menu wordt nu correct bijgewerkt en focus gaat naar het tekstbedienelement wanneer u hulp opent in Paperback.
* Gewijzigd naar een veel veiliger IPC-methode in Windows.
* De actieve documenttitel wordt nu uitgesproken bij het schakelen tussen tabbladen.
* Verminderd geheugengebruik op grote documenten door de grootte van de interne per-karakterindextabellen te halveren.

##### Dialoogvenster Alle documenten
* Escape sluit het dialoogvenster Documentinfo en Alle documenten niet.
* De titelbalk wordt niet bijgewerkt na het sluiten van een document in het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan uw lijst met alle documenten toegevoegd wanneer het via Shift+F1 wordt geopend.
* Het verwijderen van documenten uit het recente dialoogvenster sluit nu ook hun actieve tabblad.
* Uw zoekfilter wordt nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie die in sommige situaties onjuiste lijntekst aankondigt.
* Ga naar regel, Ga naar pagina en Ga naar procent plaats uw cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren de geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Bladwijzer-/notitieluiden moeten nu correct uitsluitend afspelen wanneer u over een woord met een woord navigeert.

##### Leesbaarheid
* Het toepassen van woordomloop brengt u naar het begin van uw document.

##### Web View
* Het webweergave-dialoogvenster kan niet worden gewijzigd en wordt weergegeven met een zeer kleine initiële grootte.
* Afbeeldingen moeten nu correct in de ingesloten webweergave worden weergegeven.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in release-aantekeningen.

##### DAISY-boeken
* DAISY-boeken die onjuiste info in de statusbalk weergeven.
* DAISY-boeken laden met valse coderingsverklaringen.

##### RTF-documenten
* RTF-documenten parseren met niet-Latijnse tekens erin.
* RTF `\pict` groepen zodat ingesloten afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en rommel in de boektekst plaatsen.
* Links in verouderde Mobi-boeken.
* Aanzienlijk verbeterde AZW3-parsering.

##### Word-documenten
* Word-documenten met landinstellingsspecifieke stijlnamen die hun koppen niet correct weergeven.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen die geen regeleinden in XHTML-documenten opleveren.

##### PDF-documenten
* Paperback valt nu terug op platte tekstextractie voor PDF-bestanden die onjuist zijn gelabeld.
* PDF-documenten met besturingselementen in hun titels en/of bladwijzers crashen Paperback niet meer bij het openen.

### Versie 0.8.5
* Paginaondersteuning aan epub-boeken toegevoegd.
* Ondersteuning voor versleutelde Microsoft Office-documenten toegevoegd. Momenteel worden verouderde Word, moderne Word en moderne PowerPoint ondersteund, met verouderde PowerPoint gepland voor de toekomst.
* Ondersteuning voor verouderde Microsoft Word-documenten toegevoegd!
* Ondersteuning voor verouderde PowerPoint-presentaties toegevoegd!
* Ondersteuning voor mobi- en AZW3-boeken toegevoegd!
* Ondersteuning voor gelabelde PDF-bestanden toegevoegd!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning voor gecomprimeerde boeken van Bookshare (zowel DAISY als Word) toegevoegd!
* Alt-tekst voor ingesloten afbeeldingen moet nu correct worden weergegeven.
* CHM-documenten ondersteunen nu correct navigatie naar interne links.
* Ga naar pagina vast dat met 1 was uitgeschakeld.
* Escape-toets werkt niet om het dialoogvenster "Open als" te sluiten.
* Het lezercontextmenu verschijnt niet op rechter muisknop of de toepassingstoets.
* Soms werd het verkeerde document gefocust wanneer documenten via de opdrachtregel werden geopend.
* Afbeeldingseigen PDF's worden opnieuw gedetecteerd en waarschuwen u voor hun bestaan.
* Het is nu mogelijk om afbeeldingen en figuren met g/shift+g en f/shift+f respectievelijk te navigeren.
* Paperback respecteert nu uw toepassingsinstelling voor donkere modus.
* DAISY XML-ondersteuning verwijderd, aangezien het niet meer nodig is.
* Teruggekeerd naar native Win32 eerste letternavigatie in de inhoudsopgavestructuur.
* Het dialoogvenster Fout laden toont nu meer gedetailleerde foutmeldingen.
* De webweergave opent nu veel sneller en soepeler.

### Versie 0.8.2
* Paginaondersteuning aan RTF-documenten toegevoegd!
* Een bug verholpen waarbij het openen van de webweergave in epub's met externe links deze automatisch zou activeren.
* Een bug verholpen waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's worden gesplitst in meerdere korte regels in sommige PDF-documenten.
* PDF-documenten hebben nu basisondersteuning voor link- en kopnavigatie!
* RTF-tabs en regeleinden worden nu exact weergegeven zoals ze in het document worden weergegeven.
* Teruggeschakeld naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering veel betrouwbaarder is.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs met de RTF-parser verholpen.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) die beschadigd raakten bij het openen van een bestand via een tweede Paperback-instantie.
* PDF-tekst wordt in de verkeerde volgorde gelezen en onjuiste afstand rond gekapitaliseerde woorden.
* Trage documentlading bij het openen van grote bestanden.
* De lokalisatie van de Ja/Nee-knoppen in bevestigingsdialogen verholpen.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu uw huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionaal geluidfeedback voor het bereiken van een bladwijzer of aantekening toegevoegd, dank aan Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning voor DAISY XML-documenten toegevoegd.
* Ondersteuning voor platte Open Document Text-bestanden toegevoegd!
* Ondersteuning voor platte Open Document-presentaties toegevoegd!
* Ondersteuning voor scheidingstekens met s en shift+s.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan uw navigatiegeschiedenis.
* Paperback-venster herstellen uit het systeemvak vast.
* Markdown-documenten die onbewerkte tekst in plaats van gerenderde HTML in de Web View weergeven.
* Tabellen worden niet correct weergegeven in Markdown-bestanden.
* Afbeeldingseigen PDF's waarschuwen u nu voor hun bestaan wanneer u probeert er een te laden.
* Versie-informatie correct insluiten in het Paperback-uitvoeringsbestand.
* Het dialoogvenster Opties in tabbladen verdeeld voor gemak en navigatie.
* Overgestapt op Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbedienelement bevat nu lezersspecifieke acties in plaats van algemene items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning voor HTML en XHTML-gebaseerde documenten toegevoegd! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om ze in een webweergave weer te geven.
* Een basisfunctie voor webweergave toegevoegd! Druk op Ctrl+Shift+V om het huidige gedeelte van uw document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank aan Ruslan Gulmagomedov!
* Een knop Clear All aan het dialoogvenster Alle documenten toegevoegd.
* De updatecontrole geeft nu release-aantekeningen weer wanneer een nieuwe versie beschikbaar is.
* Venster herstellen uit het systeemvak vast.
* Ja/Nee-knopvertalingen in bevestigingsdialogen vast.
* Configs laden wanneer het als administrator wordt uitgevoerd.
* Commentaarverwerking in XML- en HTML-documenten vast.
* TOC-parsering in Epub 2-boeken vast.
* Navigeren naar het volgende item met dezelfde letter in de inhoudsopgave vast.
* Find-dialoogvenster verbergt niet correct wanneer u de knoppen Volgende/Vorige gebruikt.
* Epub TOC's brengen u soms naar het verkeerde item.
* Verschillende problemen met spatieafhandeling in XML-, HTML- en pre-tags.
* Off-by-one fout in linknavigatie.
* Sommige boeken die spaties aan het einde van hun regels hebben.
* Verschillende parseerproblemen.
* Bladwijzer-gerelateerde menu-items evenals de elementenlijst worden nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstverwerking in verschillende documentindelingen.
* Verbeterde vertaalworkflow voor medewerkers.
* Veel interne refactors, waarbij het grootste deel van de bedrijfslogica van de toepassing van C++ naar Rust is verplaatst voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een zeer basische functie voor navigatie naar vorige/volgende positie toegevoegd. Als u Enter op een interne link drukt en deze uw cursor verplaatst, wordt die positie nu onthouden en kan er met alt+left/right-pijlen naar worden genavigeerd.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een structuur van alle kopjes in uw document of een lijst met links, maar er zijn plannen om dit in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard gemaximaliseerd te starten.
* Links in enkele Epub-documenten werken niet correct.
* Parsering van Epub TOC's met relatieve paden.
* Sommige epub-documenten geven geen titel of auteur weer.
* De titels van sommige epub-hoofdstukken verschijnen niet correct in het TOC-dialoogvenster.
* U kon de spatiebalk niet gebruiken om de knoppen OK/Annuleren in het TOC-dialoogvenster te activeren.
* Verbeterde verwerking van kopjes in Word-documenten.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u het dialoogvenster opent.

### Versie 0.6.0
* Een nieuwe optie om het gaan-menu in een veel compactere vorm weer te geven is aan het dialoogvenster Opties toegevoegd, standaard ingeschakeld.
* Een optie toegevoegd om navigatie per structuurelement in te laten staan.
* Een optie aan het menu Extra toegevoegd om de map met het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief bijwerksysteem toegevoegd.
* Een basisfunctie voor slaaptimer toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning voor het parseren van FB2 e-boeken toegevoegd!
* Ondersteuning voor het parseren van OpenDocument-presentaties toegevoegd!
* Ondersteuning voor het parseren van OpenDocument Text-bestanden toegevoegd!
* Bladwijzers kunnen nu een hele regel als bladwijzer markeren of alleen bepaalde tekst markeren. Als u geen selectie actief hebt wanneer u een bladwijzer plaatst, is het gedrag als vóór 0.6 en wordt de hele regel gemarkeerd. Als u echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnota's krijgen! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het dialoogvenster Bladwijzers met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een vervelend "bookmark x"-voorvoegsel.
* Epub-boeken die HTML-inhoud als XML proberen voor te doen worden nu correct verwerkt.
* Het laden van grote Markdown-documenten vast.
* Het indrukken van spatie in de inhoudsopgavestructuurweergave activeert de OK-knop.
* Spatieafhandeling aan het begin van pre-tags in zowel HTML- als XHTML-documenten vast.
* Het tekstbedienelement krijgt soms niet terug focus bij terugkeer naar het Paperback-venster.
* Het tekstveld in het dialoogvenster gaan naar procent werkt niet bij met de waarde van de schuifregelaar.
* De weergave van aangepaste HTML-ID's in Markdown-documenten vast.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijgt u niet langer een fout als het laden van uw document meer dan 5 seconden duurt.
* Als Paperback als administrator wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks uit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand heet naar het bestand met een .paperback-extensie. Als een dergelijk bestand in dezelfde map wordt gevonden als een bestand terwijl het wordt geladen, wordt het automatisch geladen. Anders kunt u het handmatig importeren met een item in het menu Extra.
* Links in documenten worden nu volledig ondersteund! Gebruik k en shift+k om voor- en achteruit te bewegen en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorbewerkt om CommonMark-compatibel te zijn voordat het wordt weergegeven.
* Navigatie per lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om door lijsten zelf te gaan en I en Shift+I om door lijstitems te gaan.
* Numpad Delete verwijdert nu ook documenten uit de taakbalk naast normale Delete.
* Paperback kan nu optioneel minimaliseren naar uw systeemvak! Deze optie staat standaard uit, maar het inschakelen ervoor zorgt dat de minimaliseeroptie in het systeemmenu Paperback in uw systeemvak plaatst, herstelbaar door op het gespawnd pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met ondersteunde talen is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten geven nu een basisinhoudsopgave weer met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Documentinfo.
* Het installatieprogramma bevat nu een optie om het leesmij-bestand na installatie in uw browser te bekijken.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van u eenvoudig de laatste 10 geopende documenten weer te geven, toont het nu een aanpasbaar getal, met de rest van de documenten die u ooit hebt geopend die toegankelijk zijn via een klein dialoogvenster.
* Verschillende kleine verbeteringen van de parsers in het algemeen, inclusief het plaatsen van een blanco regel tussen dia's in PPTX-presentaties, het aanpassen van de regelehandeling in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items worden niet uitgeschakeld als er geen documenten zijn geopend.
* De oriëntatie van de schuifregelaar voor gaan naar procent vast.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragmentID's vast.
* Spaties worden op vreemde manieren uit XHTML-kopjes verwijderd.
* Spatieafhandeling in geneste pre-tags in HTML-documenten.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave uit de structuur van de kopjes in uw document en toont het die in het dialoogvenster ctrl+t.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, indien deze bestaat. Anders blijven ze de bestandsnaam zonder extensie gebruiken.
* Overgestapt van UniversalSpeech naar het gebruik van een live region om spraak te rapporteren. Dit betekent dat geen schermlezer-DLL's meer samen met het programma worden geleverd en meer schermlezerss nu worden ondersteund, zoals Microsoft Narrator.
* Gewijzigd van zip-bibliotheken om een breder scala aan epub-boeken te openen.
* Het dialoogvenster waarin u wordt gevraagd of u uw document als platte tekst wilt openen, is volledig opnieuw gemaakt en u kunt uw document nu openen als platte tekst, HTML of Markdown.
* Het dialoogvenster gaan naar procent bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar toe te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken blijft nu weer exact behouden.
* De Unicode-spatie zonder regelafbreking wordt nu overwogen bij het verwijderen van lege regels.
* U wordt niet langer elke keer om gevraagd hoe u een onherkenbaar bestand wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel pictogram voor het startmenu aan het installatieprogramma toegevoegd.
* De inhoudsopgave moet nu in enkele gevallen schoner zijn, bijvoorbeeld als u een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, ziet u nu alleen het bovenliggende item.
* Inhoudsopgave in bepaalde CHM-documenten vast.
* Inhoudsopgave in Epub 3-boeken met absolute paden erin vast.
* CHM-documenten moeten nu hun titel weergeven zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Ondersteuning voor bladwijzers toegevoegd! U kunt zoveel bladwijzers in zoveel documenten hebben als u wilt. U kunt ermee vooruit en achteruit springen met b en shift+b, u kunt er een instellen met control+shift+b, en u kunt een dialoogvenster openen om naar een specifieke bladwijzer te springen met control+b.
* Een installatieprogramma naast het draagbare ZIP-bestand toegevoegd! Het installatieprogramma installeert Paperback in uw map Program Files en stelt automatisch bestandskoppelingen in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd en de BOM zal niet langer aan het begin van de tekst worden weergegeven.
* Veel meer informatie aan de statusbalk toegevoegd. Het toont nu uw huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer weergegeven in tekstuitvoer.
* Als u een relatief pad doorgeeft aan Paperback op de opdrachtregel, zal het dit nu correct omzetten.
* Percentagebewegingen worden nu verwerkt door hun eigen schuifregelaar-gebaseerd dialoogvenster, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs krijgen nu altijd een standaard.
* De logica voor positionering is nu veel intelligenter en schrijft alleen naar de schijf wanneer dit absoluut noodzakelijk is.
* Het document dat u hebt gefocust toen u Paperback sloot, wordt nu onthouden tussen toepassingen opnieuw starten.
* Invoer in de dialoogvensters gaan naar regel en gaan naar pagina moet nu strikter worden opgeschoond.
* Inhoudsopgavenavigatie in epub 3-boeken met relatieve paden in hun manifesten vast.

### Versie 0.3.0
* Inhoudsopgave in epub-boeken met URL-gecodeerde manifesten vast.
* Kopnavigatie in HTML-documenten met multi-byte Unicode-tekens vast.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets.
* UTF-8 tekstbestanden laden vast.
* Geneste TOC-items in Epub-boeken plaatsen uw cursor op de verkeerde positie.
* Een crash bij afsluiten van de toepassing in bepaalde gevallen.
* Een selectievakje in het dialoogvenster Opties toegevoegd om woordomloop in of uit te schakelen!
* Het is nu mogelijk om aan Paperback's ontwikkeling bij te dragen, hetzij via het nieuwe donatieitem in het menu Help, hetzij via de koppeling Sponsor this project onderaan de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel en Paperback moet nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, ook al ontbreken de metagegevens.
* Overgestapt op de PDF-bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaarder PDF-parsering over het hele bord.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. paperback.exe met een bestandsnaam uitvoeren terwijl het al wordt uitgevoerd, opent dat document in het al actieve exemplaar.
* U kunt nu op Delete op een document in het tabtabblad drukken om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's aan het paginalabel in het dialoogvenster gaan naar pagina toegevoegd.
* Tabblad van de documentinhoud naar uw lijst met geopende documenten toestaan.
* De kopingstoetsen openen soms recente documenten als u er genoeg van had.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit tekstuitvoer.
* Kopnavigatie plaatst u soms op het verkeerde teken.

### Versie 0.2.0
* Ondersteuning voor Markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen voor het navigeren via kopjes in HTML-inhoud, inclusief epub-boeken en Markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar met een schermlezer te werken.
* Epub's laden met URL-gecodeerde bestandsnamen in hun manifesten vast.
* Epub 3-boeken laden met XHTML erin ingesloten.
* Een bericht wordt nu uitgesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items zijn uitgeschakeld.
* Een menu voor recente documenten toegevoegd! Het opslaat momenteel uw laatste 10 geopende documenten, en Enter op een ervan opent het voor lezen.
* Het dialoogvenster Zoeken volledig herschreven, waardoor het veel eenvoudiger te gebruiken is, en ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies is toegevoegd!
* Eerder geopende documenten worden nu onthouden tussen toepassingen opnieuw starten. Dit kan worden geconfigureerd via het nieuwe item opties in het menu Extra.
* shift+f1 toegevoegd om het leesmij-bestand rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Eerste release.
