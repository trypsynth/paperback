<!-- machine-translated from doc/readme.md (source-hash: d49e7044d9856698); please review and edit as needed -->

# Paperback - versie 0.9.1

## Inleiding

Paperback is een lichte, snelle en toegankelijke ebook- en documentlezer voor iedereen, van casual lezers tot power users. Het is ontworpen voor schermlezertoegang, snelheid en een opruimingvrije ervaring.

## Systeemvereisten

Paperback draait momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native iOS- en Android-apps zijn in actieve ontwikkeling, met openbare testbuilds die binnenkort na de release van desktop 0.9.0 zijn gepland, voorafgaand aan een uniforme 1.0-release die alle vier platforms omvat.

## Functies

* Volledig zelfstandig, het vereist geen software om op uw computer te worden geïnstalleerd om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee u zoveel documenten als u wilt naast elkaar kunt openen.
* Slaat uw exacte leespositie op voor elk document dat u opent.
* Kan optioneel onthouden welke documenten u open had toen u het programma sloot, en herstelt deze bij de volgende start.
* Bevat navigatiefunctionaliteit die vergelijkbaar is met die in de webbrowsemodus van veel schermlezers om snel en eenvoudig door documenten te navigeren.
* Bevat een robuuste zoekopdracht, inclusief functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig draagbaar worden uitgevoerd of worden geïnstalleerd met automatisch ingestelde bestandskoppelingen.
* Ondersteunt een groot aantal veelgebruikte bestandsindelingen.

## Compatibiliteit met schermlezers

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en brailleschermen

Als u JAWS met een braillescherm gebruikt, kunt u merken dat lange alinea's worden afgekapt wanneer u vooruit schuift met de navigatietoetsen van uw scherm. De opdracht huidige alinea lezen wordt ook beïnvloed. Dit is een bug in de manier waarop JAWS de RICHEDIT50W-tekstbesturingselement afhandelt, niet iets in Paperback zelf, en iets wat behoorlijk lang heeft geduurd om een oplossing voor op te duiken gezien Vispero's enthousiasme voor het reageren op problemen met open source-software.

De tijdelijke oplossing, uiteindelijk opgedoken via de JAWS-discussiegroep na maanden wachten, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". U wilt ook "Pan Text by Paragraph" inschakelen, anders blijft uw scherm op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen op hun plaats zou panoramering correct moeten werken.

## Momenteel ondersteunde bestandstypen

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
* Platte tekst en logbestanden (`.txt`, `.log`)

## Toetsenbordsneltoetsen

Paperback is ontworpen voor gebruik met het toetsenbord. Hieronder vindt u de huidige sneltoetsen.

De sneltoetsen hieronder zijn voor Windows. Waar macOS verschilt, wordt het equivalent tussen haakjes vermeld — voornamelijk omdat `Ctrl+G`, `Ctrl+W` en `Alt+Left`/`Alt+Right` al door andere system- of app-conventies op dat platform worden gebruikt.

### Bestandsmenu

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle open documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (uit Recent Documents).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS bevindt dit zich in plaats daarvan in het app-menu).

### Menu Gaan

* `Ctrl+F`: Het dialoogvenster Zoeken weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Naar regel gaan.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Naar percentage gaan.
* `Ctrl+P`: Naar pagina gaan (wanneer ondersteund door het huidige document).
* `=`: Uw huidige leespercentage aangeven.
* `Alt+Left` (macOS: `Cmd+[`): Terug in navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruit in navigatiegeschiedenis.
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
* `Shift+N`: Vorige noot.
* `N`: Volgende noot.
* `Ctrl+B`: Naar alle bladwijzers en noten gaan.
* `Ctrl+Alt+B`: Alleen naar bladwijzers gaan.
* `Ctrl+Alt+M`: Alleen naar noten gaan.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dus de fysieke Control-toets in plaats van Cmd): Nootekst op de huidige positie weergeven.
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

### Menu Extra's

* `Ctrl+W` (macOS: `RawCtrl+W`, dus de fysieke Control-toets in plaats van Cmd): Woordental voor het huidige document weergeven.
* `Ctrl+I`: Documentinformatie weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Map met bestand openen.
* `Ctrl+Shift+V`: Huidige inhoud in Web View openen.
* `Ctrl+U`: Documentbron in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Huidige document naar gewone tekst exporteren.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursor in-/uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernoot op de huidige selectie/cursor toevoegen of bewerken.
* `Ctrl+Alt+W`: Regelomloop in-/uitschakelen.
* `Ctrl+Space`: Audio-navertelling afspelen/pauzeren.
* `'`: Audio-navertelling vooruit zoeken.
* `;`: Audio-navertelling achteruit zoeken.
* `Ctrl+'`: Zoekbereik voor audio verhogen.
* `Ctrl+;`: Zoekbereik voor audio verlagen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dus Control+Command+F): Volledigscherm in-/uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in-/uitschakelen.

### Menu Help

* `Ctrl+F1`: Dialoogvenster Info weergeven.
* `F1`: Help weergeven in uw standaardbrowser.
* `Shift+F1`: Help weergeven in Paperback.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: Donatiepagina openen in uw standaardbrowser.

### Aanvullende documentweergavetoetsen

* `Delete` / `Numpad Delete` op het tabbereik: Geselecteerd documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: Link op cursor activeren, of tabelweergave openen bij een tabelmarker.
* `Shift+F10` of de Menu/Toepassingstoets in de documenttekst: Contextmenu openen.

## Ondersteunde talen

Paperback is in veel verschillende talen vertaald en er worden voortdurend meer toegevoegd. Hieronder vindt u een volledige lijst.

Wil je graag bijdragen? Lees dan onze [Vertaalgids](translating.md).

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

## Dankbetuigingen
### Ontwikkeling
* Quin Gillespie: primaire ontwikkelaar en oprichter van het project.
* Aryan Choudhary: primaire bijdrager.

### Donaties
De volgende personen hebben donaties van enige omvang gedaan voor de ontwikkeling van Paperback. Als u een donatie doet, wordt uw naam niet automatisch hier toegevoegd. Ik voeg alleen personen toe die willen dat hun donatie openbaar wordt gemaakt.

Opmerking: Ik beschouw een openbare GitHub-sponsor als grond voor automatische opneming in deze lijst.

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

### Versie 0.9.1
* Geluiden voor bladwijzers en notities worden nu afgespeeld op macOS.
* DAISY-boeken spelen nu hun audio af op macOS, in plaats van te openen en hun tijdlijn zonder geluid bij te houden.
* Krullingen, gedachtestrepen en vergelijkbare tekens die verdwenen uit RTF-documenten, waardoor de omringende woorden aan elkaar groeiden, zijn gerepareerd.
* RTF-afbeeldingen lekken hun onverwerkte gegevens niet langer als garbled text in het document.
* Het submenu Recente documenten behoudt nu geen stale entries meer totdat iets anders het herbouwt.
* Toetsenbordversnellers zijn terug in elke vertaling, dus de menu's in het Russisch hebben weer toetsenbordtoegang.
* Grote CHM-documenten openen nu tot zeven keer sneller.
* Geopende documenten worden nu geregistreerd bij Windows, zodat zij verschijnen in de taakbalk-jumplist en de recente lijst van het Startmenu.
* Opties is hernoemd naar Instellingen, wat aansluit bij de mobiele apps en, op macOS, de platformconventie.
* Paperback onthoudt nu zijn vensterpositie, grootte en gemaximaliseerde status tussen runs.
* Meervoudsvormen worden nu vertaald, dus berichten die dingen tellen, lezen correct in talen die meer dan één vorm nodig hebben.
* Het selecteren van ncc.html van een DAISY-boek opent nu het volledige audioboek in plaats van alleen de tekst.
* De actienamen in het dialoogvenster Toetsenbordsneltoetsen aanpassen kunnen nu worden vertaald.
* De documenttitel staat nu eerst in de titelbalk, zodat geopende boeken kunnen worden onderscheiden in de taakbalk en Alt+Tab.
* Het dialoogvenster Update is nu vertaald.

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-hulpprogramma, genaamd pb, om snel elk ondersteund formaat van Paperback naar HTML, Markdown of platte tekst te converteren.
* Een optie om documenten opnieuw te laden die door andere programma's op schijf zijn gewijzigd.
* Een optie View Source om de bron van een document in een nieuw tabblad te openen, handig bijvoorbeeld voor het bewerken van Markdown.
* Documenttekst wordt nu gepagineerd, wat betekent dat u boeken met tientallen miljoenen woorden in slechts een paar seconden kunt laden. Meld alstublieft alle vreemdigheden die u hiermee vindt.

##### Platformondersteuning
* ARM64 Windows-ondersteuning!
* Systeemeigen macOS-ondersteuning!
* Een fullscreentoggle.

##### Dialoogvenster Alle documenten
* Een locatieknop om ontbrekende boeken te vinden die zojuist hun pad hebben veranderd.
* Een statusfilter en statusbalk, zodat u op documentstatus kunt filteren en kunt zien hoeveel documenten worden weergegeven en geselecteerd.
* De sneltoets `Ctrl+Shift+A` om alle documenten af te selecteren.

##### Opties en leesbaarheid
* Een tabblad voor leesbaarheid, met de volgende opties:
    * Tekstterugloop (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor tekstterugloop en een daaropvolgende sneltoets.
* Een toggle om bepalen hoe u tabellen wilt weergeven, en geïntegreerde weergave van tabellen in documenten.

##### Navigatie
* Ondersteuning voor navigatie per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij navigatie tussen regels, vergelijkbaar met browse-modus in schermlezers.
* De equals-sneltoets om uw huidige percentage door een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: u kunt er een per document hebben, en ze blijven bestaan. Gebruik slash om er een in te stellen en backslash om ernaar te springen.

##### Woordentelling
* Geschatte leestijd in het woordenteldialoog, evenals de mogelijkheid om uw leessnelheid in te stellen om deze metriek echt nuttig te maken.
* Als een selectie actief is wanneer u het woordenteldialoog opent, wordt nu weergegeven hoeveel woorden u hebt geselecteerd.

##### Toetsenbordsneltoetsen
* De mogelijkheid om elke toetsenbordsneltoets in de app aan te passen via een eenvoudig dialoog.
* Een configureerbare toetsenbordsneltoets om Paperback uit het systeemvak herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Export
* Het menu-item Export is uitgebreid om export naar HTML en Markdown mogelijk te maken, naast platte tekst.

##### Updater
* Een knop Annuleren in het dialoogvenster Update in uitvoering.
* De updater valideert nu dat het gedownloade bestand niet is manipuleerd.

##### Webweergave
* De webweergave wordt nu geopend op uw huidige leesfositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor DAISY 2.02-audioweergeving.

##### Audioboeken
* De mogelijkheid om audioboeken af te spelen, momenteel ondersteunend zowel DAISY-audio (inclusief DAISY-audio + tekst) als zip-bestanden met audiobestanden.
* Toetsenbordsneltoetsen en menu-items om naratie af te spelen/pauze in te stellen, vooruit en achteruit te zoeken, en het zoekbereik aan te passen.
* Opties om de leescursor te synchroniseren met audioweergeving, het audioszoekbereik in te stellen, en te kiezen of zoeken voorbij het einde van een hoofdstuk doorgaat naar de volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Gerepareerd

##### Algemeen
* Documenten gecodeerd in oudere CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als mojibake.
* "Laatst gesloten opnieuw openen" probeert het gebundelde readme opnieuw te openen.
* Uw geselecteerde tabblad krijgt na herstart van Paperback niet goed de focus.
* Paperback's verwerking van bestanden op Windows-netwerkschijven: het toont het bestand in de map drukken richt nu correct het bestand op de netwerkopslag en de paden bevatten geen vreemde tekens meer.
* .paperback-bestanden worden niet langer geforceerd geladen bij documentherstel; in plaats daarvan wordt u gevraagd om bevestiging wanneer er één wordt gevonden.
* Open containing folder richt nu het gegeven bestand in verkenner in.
* Het openen van het readme-bestand respecteert nu uw geselecteerde taal.
* De gebruikersinterface van Paperback schaalt nu correct op high-DPI-displays.
* Het menu wordt nu correct bijgewerkt, en de focus verplaatst zich naar het tekstbesturingselement, wanneer u hulp opent in Paperback.
* Overgeschakeld naar een veel veiliger methode van IPC op Windows.
* De titel van het actieve document wordt nu gelezen wanneer u tussen tabbladen schakelt.
* Gereduceerd geheugengebruik op grote documenten door de grootte van de interne per-tekens-indexarrays te halveren.

##### Dialoogvenster Alle documenten
* Escape sluit de dialoogvensters Document Info en Alle documenten niet.
* De titelbalk werkt niet bij na het sluiten van een document uit het dialoogvenster Alle documenten.
* Readme.html wordt niet langer aan uw lijst Alle documenten toegevoegd wanneer geopend via Shift+F1.
* Het verwijderen van documenten uit het dialoogvenster Recenten sluit nu ook hun actieve tabblad.
* Uw zoekfilter wordt nu behouden na het verwijderen van een document.

##### Navigatie
* Paginanavigatie die in sommige situaties onjuiste lijnentekst aankondigt.
* Ga naar Regel, Ga naar Pagina, en Ga naar Percent plaats uw cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteren het geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Geluiden voor bladwijzers/notities zouden nu correct exclusief moeten afspelen wanneer u over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Het toepassen van tekstterugloop schiet u naar het begin van uw document.

##### Webweergave
* Het webweergave-dialoogvenster kan niet worden vergroot en verschijnt bij een zeer kleine initiële grootte.
* Afbeeldingen worden nu correct weergegeven in de ingebedde webweergave.

##### Updater
* De updater toont nu correct de inhoud van markdown-codetags in releaseopmerkingen.

##### DAISY-boeken
* DAISY-boeken tonen onjuiste informatie in de statusbalk.
* DAISY-boeken laden met valse coderingsdeclaraties.

##### RTF-documenten
* RTF-documenten parseren met niet-Latijnse tekens erin.
* RTF `\pict`-groepen zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst lekken.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags splitsen en rotzooi in de boektekst plaatsen.
* Links in oudere Mobi-boeken.
* Veel verbeterd AZW3-parsering.

##### Word-documenten
* Word-documenten met landinstellinsgspecifieke stilnamen geven hun koppen niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen die geen regelbreukingen produceren in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op extractie van platte tekst voor onjuist getagde PDF's.
* PDF-documenten met controletekens in hun titels en/of bladwijzers crashen Paperback niet langer bij het openen.

### Versie 0.8.5
* Paginaondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden oudere Word, moderne Word en moderne PowerPoint ondersteund, met oudere PowerPoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor oudere Microsoft Word-documenten (*.doc)!
* Ondersteuning toegevoegd voor oudere PowerPoint-presentaties (*.ppt)!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De sneltoets ctrl+q toegevoegd om de app af te sluiten.
* Ondersteuning toegevoegd voor gecomprimeerde boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingebedde afbeeldingen wordt nu correct weergegeven.
* CHM-documenten ondersteunen nu correct interne koppelingnavigatie.
* Bladwijzergeluiden worden nu geactiveerd op alinea-begin in plaats van op de positie van de bladwijzer.
* Het repareren van "ga naar pagina" dat 1 af staat.
* De sneltoets Escape werkt nu niet om het dialoogvenster "openen als" te sluiten.
* Het contextmenu van de lezer verschijnt nu niet meer op rechts klikken of de Applications-toets.
* Het verkeerde document dat soms de focus krijgt bij het openen van documenten vanaf de opdrachtregel.
* PDF's met alleen afbeeldingen worden opnieuw gedetecteerd en waarschuwen u van hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met g/shift+g respectievelijk f/shift+f.
* Paperback respecteert nu uw donkere modusinstelling van de applicatie.
* DAISY XML-ondersteuning verwijderd, omdat het niet langer nodig is.
* Teruggekeerd naar de systeemeigen Win32-navigatie met eerste letter in de tabel met inhoud.
* Het dialoogvenster voor foutladen toont nu meer gedetailleerde foutmeldingen.
* De webweergave opent nu veel sneller en soepeler.

### Versie 0.8.2
* Paginaondersteuning toegevoegd aan RTF-documenten!
* Een bug opgelost waarbij het openen van de webweergave in epub's met externe koppelingen deze automatisch zou activeren.
* Een bug opgelost waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden zou plaatsen.
* Alinea's worden in sommige PDF-documenten in meerdere korte regels gesplitst.
* PDF-documenten hebben nu basisondersteuning voor link- en koppelingnavigatie!
* RTF-tabs en regelfeeds worden nu exact weergegeven zoals ze in het document voorkomen.
* Teruggekeerd naar de beproefde pdfium-bibliotheek voor het parseren van PDF's, waardoor PDF-rendering veel betrouwbaarder wordt.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs met de RTF-parser opgelost.
* Bestandspaden met niet-ASCII-tekens (zoals Bosnisch š, č, ć, ž) worden niet langer beschadigd bij het openen van een bestand via een tweede Paperback-exemplaar.
* PDF-tekst wordt nu in de juiste volgorde gelezen, en juiste afstand rond gekapitaliseerde woorden.
* Trage documentlading bij het openen van grote bestanden opgelost.
* Lokalisatie van de Ja/Nee-knoppen in bevestigingsdialogen opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigde Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu uw huidige geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionaal geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, dank je Andre Louis voor de geluiden!
* RTF-documentondersteuning toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor platte Open Document Text-bestanden!
* Ondersteuning toegevoegd voor platte Open Document-presentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging groter dan 300 tekens voegt nu automatisch toe aan uw navigatiegeschiedenis.
* Paperback's venster herstellen vanuit het systeemvak opgelost.
* Markdown-documenten tonen nu gerenderde HTML in plaats van onbewerkte tekst in de webweergave.
* Tabellen worden niet correct weergegeven in Markdown-bestanden.
* PDF's met alleen afbeeldingen waarschuwen u nu wanneer u probeert er één te laden.
* Het is nu mogelijk om in plaats van stabiele releases naar nieuwe dev-builds te zoeken bij het zoeken naar updates.
* Versiegegevens correct ingebed in het Paperback-uitvoerbare bestand.
* Het dialoogvenster met opties in tabbladen gesplitst voor eenvoudig gebruik en navigatie.
* Overgeschakeld naar Hayro voor het parseren van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app in Rust herschreven. De nieuwe codebasis is veiliger, laadt documenten sneller, en is gemakkelijker te onderhouden en uit te breiden.
* Het contextmenu van het tekstbesturingselement bevat nu lezerspecifieke acties in plaats van generieke items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om er een in een webweergave te bekijken.
* Een basische webweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om de huidige sectie van uw document in een webbrowser weer te geven, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, dank je Ruslan Gulmagomedov!
* Een knop Alles wissen toegevoegd aan het dialoogvenster Alle documenten.
* De updatecontrole toont nu releaseopmerkingen wanneer een nieuwe versie beschikbaar is.
* Het herstellen van het venster uit het systeemvak opgelost.
* Ja/Nee-knoptranslaties in bevestigingsdialogen opgelost.
* Het laden van configs bij het uitvoeren als beheerder opgelost.
* Opmerkingenverwerking in XML- en HTML-documenten opgelost.
* TOC-parsering in Epub 2-boeken opgelost.
* Navigatie naar het volgende item met dezelfde letter in de tabel met inhoud opgelost.
* Het dialoogvenster Zoeken verbergt nu niet correct bij gebruik van de volgende/vorige knoppen.
* Epub TOC's gooien je af en toe naar het verkeerde item.
* Diverse problemen met witruimte-afhandeling in XML-, HTML- en pre-tags opgelost.
* Off-by-one-fout in koppelingnavigatie opgelost.
* Sommige boeken hebben afsluitende witruimte op hun regels opgelost.
* Diverse parseerproblemen opgelost.
* Menu-items met betrekking tot bladwijzers en de elementenlijst worden nu correct uitgeschakeld wanneer geen document is geopend.
* Verbeterde lijstverwerking in verschillende documentformaten.
* Verbeterde vertaalworkflow voor medewerkers.
* Veel interne refactors, waarbij het merendeel van de bedrijfslogica van de applicatie van C++ naar Rust wordt verplaatst voor verbeterde prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met wachtwoord beveiligde PDF's toegevoegd!
* Een zeer basisfeature toegevoegd voor het gaan naar vorige/volgende positie. Als u op Enter drukt op een interne koppeling en het verplaatst uw cursor, wordt die positie nu onthouden en kan er mee worden genavigeerd met alt+linker/rechter pijlen.
* Een elementenlijst toegevoegd! Momenteel toont het alleen een boom van alle koppelingen in uw document of een lijst met koppelingen, maar er zijn plannen om het in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard in gemaximaliseerde modus te starten.
* Koppelingen in sommige Epub-documenten werken niet correct.
* Epub TOC's parseren met relatieve paden opgelost.
* Sommige epub-documenten tonen geen titel of auteur.
* De titels van enkele epub-hoofdstukken verschijnen niet correct in het TOC-dialoog.
* U kunt nu niet de spatiebalk gebruiken om de OK/annuleer-knoppen in het TOC-dialoog te activeren.
* Verbeterde verwerking van koppelingen in Word-documenten.
* U krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer u het dialoog probeert op te roepen.

### Versie 0.6.0
* Een nieuwe optie om het go-menu in een veel compactere vorm weer te geven is toegevoegd aan het dialoogvenster met opties, standaard ingeschakeld.
* Een optie toegevoegd om navigatie door structurele elementen te laten omwikkelen.
* Een optie toegevoegd aan het menu Hulpprogramma's om de map met de huidige document te openen.
* Een vrij eenvoudig, maar zeer effectief, updatesysteem toegevoegd.
* Een basisslaperafunktie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het parseren van FB2 e-boeken!
* Ondersteuning toegevoegd voor het parseren van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het parseren van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu een hele regel bladwijzeren of alleen bepaalde tekst markeren. Als u geen selectie actief hebt bij het plaatsen van een bladwijzer, is het gedrag zoals pre-0.6, en markeert het de hele regel. Als u echter wat tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities bij zich dragen! Navigeer tussen bladwijzers met notities met N en Shift+N, of pop het dialoogvenster Bladwijzers op met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd met specifieke sneltoetsen.
* Bladwijzers in het dialoogvenster Bladwijzers hebben niet langer een vervelend "bookmark x" voorvoegsel.
* Epub-boeken met HTML-inhoud die zich voordoen als XML, worden nu correct afgehandeld.
* Het laden van grote Markdown-documenten opgelost.
* Het indrukken van spatie in het tabel van inhoud boomweergave activeert nu de OK-knop.
* Witruimteafhandeling aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Het tekstbesturingselement krijgt soms niet terug de focus wanneer u terugkeert naar het venster van Paperback.
* Het tekstveld in het dialoogvenster "ga naar procent" werkt nu niet meer bij met de waarde van de schuifregelaar.
* Rendering van aangepaste HTML-id's in Markdown-documenten opgelost.
* HTML in Markdown-codeblokken wordt nu correct weergegeven.
* Als u een boek laadt met een opdrachtregelparameter terwijl een bestaand Paperback-exemplaar wordt uitgevoerd, krijgt u niet langer een fout als het laden van uw document langer dan 5 seconden duurt.
* Als Paperback als beheerder wordt uitgevoerd, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer rechtstreeks vanuit het dialoogvenster Bladwijzers te verwijderen.
* Het is nu mogelijk om uw bladwijzers en leesfositie voor een bepaald document in en uit te voeren. Het gegenereerde bestand is vernoemd naar het bestand met een .paperback-extensie. Als zo'n bestand wordt gevonden in dezelfde map als een bestand terwijl het wordt geladen, wordt het automatisch geladen. Anders kunt u ze handmatig importeren met een item in het menu Hulpprogramma's.
* Koppelingen in documenten worden nu volledig ondersteund! Gebruik k en shift+k om vooruit en achteruit door hen heen te gaan, en druk op Enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner wordt.
* Markdown-inhoud wordt nu voorverwerkt om CommonMark-conform te zijn voordat deze wordt weergegeven.
* Navigatie per lijsten en hun items wordt nu volledig ondersteund! Gebruik L en Shift+L om door lijsten zelf te gaan, en I en Shift+I om door lijstitems te gaan.
* Numpad delete werkt nu ook om documenten uit de tabbalk te verwijderen naast normale delete.
* Paperback kan nu optioneel in uw systeemvak minimaliseren! Deze optie is standaard uit, maar als u deze inschakelt, wordt de minimaliseeroptie in het systeemmenu in uw systeemvak geplaatst, zodat deze kan worden hersteld door op het gespawde pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met talen die het ondersteunt, is momenteel vrij klein, maar groeit voortdurend!
* Paperback heeft nu een officiële website op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een basistabel met inhoud, met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster Document Info.
* Het installatieprogramma bevat nu een optie om het leesmij na installatie in uw browser weer te geven.
* De lijst met recente documenten is enorm uitgebreid! In plaats van eenvoudig de laatste 10 geopende documenten weer te geven, worden nu een aanpasbaar getal weergegeven, met de rest van de documenten die u ooit hebt geopend, toegankelijk via een klein dialoog.
* Verschillende kleine verbeteringen in de parsers over de hele linie, inclusief het plaatsen van een lege regel tussen dia's in PPTX-presentaties, het repareren van de regelafhandelingen in alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Bepaalde menu-items niet worden uitgeschakeld zonder documenten opgelost.
* De oriëntatie van de schuifregelaar "ga naar procent" opgelost.
* De tabel met inhoud in Epub-boeken met URL-gecodeerde bestandspaden en/of fragment-id's opgelost.
* Witruimte wordt op vreemde manieren uit XHTML-koppelingen verwijderd opgelost.
* Witruimte-afhandeling in geneste pre-tags in HTML-documenten opgelost.
* HTML- en Markdown-documenten ondersteunen nu de tabel met inhoud! Wanneer u een HTML/Markdown-document laadt, bouwt Paperback een eigen tabel met inhoud uit de structuur van de koppelingen in uw document, en zal dit aan u tonen in het ctrl+t-dialoog.
* HTML-documenten hebben nu de titel zoals ingesteld in de titeltag, als deze bestaat. Anders zullen zij doorgaan met het gebruik van de bestandsnaam zonder de extensie.
* Overgeschakeld van UniversalSpeech naar het gebruik van een live-region om spraak te rapporteren. Dit betekent dat er niet langer schermlezers-DLL's bij het programma worden geleverd, en meer schermlezers zullen nu worden ondersteund, zoals Microsoft Narrator.
* Zip-bibliotheken overgeschakeld naar het openen van een breder scala aan epub-boeken.
* Het dialoogvenster met de vraag of u uw document als platte tekst wilt openen, is volledig opnieuw gemaakt en stelt u nu in staat om uw document als platte tekst, HTML of Markdown te openen.
* Het dialoogvenster "ga naar procent" bevat nu een tekstveld waarmee u handmatig een percentage kunt invoeren om naar te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De tabel met inhoud in Epub-boeken wordt nu exact behouden.
* De Unicode-spatie zonder einde wordt nu beschouwd bij het verwijderen van lege regels.
* U wordt niet langer gevraagd hoe u een niet-herkend bestand wilt openen, elke keer dat u het laadt, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenu-pictogram aan het installatieprogramma toegevoegd.
* De tabel met inhoud zou nu in enkele gevallen schoner moeten zijn, bijvoorbeeld als u een onderliggend item en een bovenliggend item met dezelfde tekst op dezelfde positie hebt, ziet u nu alleen het bovenliggende item.
* De tabel met inhoud in bepaalde CHM-documenten opgelost.
* De tabel met inhoud in Epub 3-boeken met absolute paden erin opgelost.
* CHM-documenten zouden nu hun titel moeten tonen zoals ingesteld in het metagegevensbestand.

### Versie 0.4.0
* CHM-bestandondersteuning toegevoegd!
* Bladwijzerondersteuning toegevoegd! U kunt zoveel bladwijzers in zoveel documenten hebben als u wilt. U kunt er via vooruit en achteruit navigeren met b en shift+b, er een met control+shift+b instellen, en een dialoog opbrengen om naar een specifieke bladwijzer te springen met control+b.
* Een installatieprogramma naast het draagbare zip-bestand toegevoegd! Het installatieprogramma installeert Paperback in uw Program Files-map en stelt automatisch bestandskoppelingen in.
* Tekstbestanden met BOM's moeten nu correct worden gedecodeerd, en de BOM zal ook niet langer aan het begin van de tekst worden weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Het toont nu uw huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer in tekstuitvoer weergegeven.
* Als u een relatief pad aan Paperback op de opdrachtregel doorgeeft, wordt dit nu correct opgelost.
* Percentage-beweging wordt nu afgehandeld door zijn eigen schuifregelaar-dialoog, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaard.
* De logica voor positieopslagruimte is nu veel slimmer en schrijft alleen naar schijf wanneer dit absoluut noodzakelijk is.
* Het document waarop u de focus had toen u Paperback sloot, wordt nu onthouden in toepassing herstart.
* Invoer in de dialogen "ga naar regel" en "ga naar pagina" moet nu strenger worden gereinigd.
* De TOC-navigatie in epub 3-boeken met relatieve paden in hun manifesten opgelost.

### Versie 0.3.0
* De tabel met inhoud in epub-boeken met URL-gecodeerde manifesten opgelost.
* Koppelingnavigatie in HTML-documenten met multi-byte Unicode-tekens opgelost.
* Hoog CPU-gebruik in documenten met lange titels vanwege een regressie in wxWidgets.
* Het laden van UTF-8 tekstbestanden opgelost.
* Geneste TOC-items in Epub-boeken die uw cursor op de verkeerde positie plaatsen opgelost.
* Een crash bij het afsluiten van de applicatie in bepaalde gevallen opgelost.
* Een selectievakje in het dialoogvenster Opties toegevoegd om tekstterugloop in of uit te schakelen!
* Het is nu mogelijk om aan de ontwikkeling van Paperback te doneren, via het nieuwe doneeritem in het menu Hulp of via de link "sponsor dit project" onderaan de hoofdpagina van de GitHub-opslagplaats.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens ontbreken.
* PDF-bibliotheken overgeschakeld naar de in Chromium gebruikte, wat leidt tot veel betrouwbaarder PDF-parsering over de hele linie.
* U kunt nu slechts één exemplaar van Paperback tegelijk uitvoeren. Het uitvoeren van paperback.exe met een bestandsnaam terwijl het al wordt uitgevoerd, opent dat document in het reeds uitgevoerde exemplaar.
* U kunt nu op verwijderen drukken op een document in het tabblaadbesturingselement om het te sluiten.

### Versie 0.2.1
* Het totaal aantal pagina's toegevoegd aan het paginalabel in het dialoogvenster "ga naar pagina".
* Tabbladen van de documentinhoud naar uw lijst met geopende documenten toestaan.
* Enkele bugs met de toetsenbordsneltoetsen voor koppelingen opgelost die soms recente documenten openen als u er genoeg van had.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit tekstuitvoer.
* Koppelingnavigatie plaatst u soms op het verkeerde teken opgelost.

### Versie 0.2.0
* Ondersteuning voor Markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen toegevoegd voor navigatie door koppelingen in HTML-inhoud, inclusief epub-boeken en Markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar met een schermlezer te werken.
* Het laden van epub's met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Het laden van epub 3-boeken met XHTML erin opgelost.
* Een bericht wordt nu gesproken als het document geen tabel met inhoud of secties ondersteunt, in tegenstelling tot het uitschakelen van menu-items.
* Een menu met recente documenten toegevoegd! Het slaat momenteel uw laatste 10 geopende documenten op, en op Enter drukken op een ervan opent het ter lezing.
* Het dialoog Zoeken volledig herschreven, waardoor het veel eenvoudiger te gebruiken is, terwijl ook een geschiedenis van uw laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies wordt toegevoegd!
* Eerder geopende documenten worden nu onthouden in toepassing herstart. Dit kan worden geconfigureerd via het nieuwe item Opties in het menu Hulpprogramma's.
* Shift+F1 toegevoegd om het readme rechtstreeks in Paperback zelf te openen.

### Versie 0.1.0
* Initiële release.
