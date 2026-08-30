<!-- machine-translated from doc/readme.md (source-hash: bdf582cc25a739ea); please review and edit as needed -->

# Paperback - versie 0.9.0

## Inleiding

Paperback is een lichtgewicht, snelle en toegankelijke lezer voor ebooks en documenten, voor iedereen: van gelegenheidslezers tot intensieve powerusers. Het is ontworpen met het oog op toegankelijkheid voor schermlezers, hoge snelheid en een ervaring zonder overbodige ballast.

## Systeemvereisten

Paperback werkt momenteel op Windows 10/11 en alle moderne versies van ARM macOS. Native apps voor iOS en Android zijn in actieve ontwikkeling, met openbare testversies die kort na de desktopversie 0.9.0 gepland staan, vooruitlopend op een gezamenlijke 1.0-release voor alle vier de platformen.

## Functies

* Volledig zelfstandig; er hoeft geen software op je computer geïnstalleerd te worden om te beginnen met lezen.
* Ongelooflijk snel, zelfs op oude hardware.
* Eenvoudige interface met tabbladen, waarmee je zoveel documenten naast elkaar kunt openen als je wilt.
* Bewaart je exacte leespositie in elk document dat je opent.
* Onthoudt optioneel welke documenten je open had staan toen je het programma afsloot, en herstelt ze bij de volgende start.
* Bevat navigatiefunctionaliteit die vergelijkbaar is met die van de webbrowsemodus van veel schermlezers, om snel en eenvoudig door documenten te navigeren.
* Bevat een krachtig zoekvenster, met functies zoals geschiedenis en ondersteuning voor reguliere expressies.
* Kan volledig portable worden gebruikt, of worden geïnstalleerd met automatisch ingestelde bestandskoppelingen.
* Ondersteunt een enorm aantal veelgebruikte bestandsformaten.

## Compatibiliteit met schermlezers

Paperback werkt goed met alle grote schermlezers. Er is echter één bekend probleem voor JAWS-gebruikers.

### JAWS en brailleleesregels

Als je JAWS met een brailleleesregel gebruikt, kan het zijn dat lange alinea's worden afgekapt bij het vooruit pannen met de navigatietoetsen van je leesregel. Ook het commando om de huidige alinea te lezen wordt hierdoor beïnvloed. Dit is een fout in de manier waarop JAWS omgaat met het tekstelement RICHEDIT50W, niet iets in Paperback zelf, en een fout waarvoor het nogal lang duurde voordat er een oplossing opdook, gezien het enthousiasme van Vispero om te reageren op problemen met opensourcesoftware.

De tijdelijke oplossing, die na maanden wachten uiteindelijk via de JAWS-discussiegroep boven kwam, is om `paperback.jcf` te bewerken en "Braille Presentation and Panning" in te stellen op "Always use DOM if available". Je wilt ook "Pan Text by Paragraph" inschakelen, anders blijft je leesregel op de actieve alinea staan in plaats van verder te gaan. Met beide instellingen ingeschakeld zou het pannen correct moeten werken.

## Momenteel ondersteunde bestandstypen

Paperback ondersteunt de volgende formaten en extensies:

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

## Sneltoetsen

Paperback is ontworpen voor gebruik met het toetsenbord op de eerste plaats. Hieronder staan de huidige sneltoetsen.

De onderstaande sneltoetsen gelden voor Windows. Waar macOS afwijkt, staat het equivalent tussen haakjes vermeld — vooral omdat Ctrl+G, Ctrl+W en Alt+Links/Rechts op dat platform al door andere systeem- of app-conventies in gebruik zijn.

### Menu Bestand

* `Ctrl+O`: Een document openen.
* `Ctrl+F4` (macOS: `Cmd+W`): Het huidige document sluiten.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Alle geopende documenten sluiten.
* `Ctrl+Shift+T`: Het laatst gesloten document opnieuw openen.
* `Ctrl+R`: Het dialoogvenster "Alle documenten" weergeven (vanuit Recente documenten).
* `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat dit in het app-menu).

### Menu Ga naar

* `Ctrl+F`: Het zoekvenster weergeven.
* `F3` (macOS: `Cmd+G`): Volgende zoeken.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Vorige zoeken.
* `Ctrl+G` (macOS: `Cmd+L`): Ga naar regel.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ga naar percentage.
* `Ctrl+P`: Ga naar pagina (indien ondersteund door het huidige document).
* `=`: Je huidige leespercentage aankondigen.
* `Alt+Left` (macOS: `Cmd+[`): Terug in de navigatiegeschiedenis.
* `Alt+Right` (macOS: `Cmd+]`): Vooruit in de navigatiegeschiedenis.
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
* `/`: Je tijdelijke bladwijzer instellen.
* `\`: Naar je tijdelijke bladwijzer springen.
* `Shift+N`: Vorige notitie.
* `N`: Volgende notitie.
* `Ctrl+B`: Naar alle bladwijzers en notities springen.
* `Ctrl+Alt+B`: Alleen naar bladwijzers springen.
* `Ctrl+Alt+M`: Alleen naar notities springen.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, dus de fysieke Control-toets in plaats van Cmd): De notitietekst op de huidige positie weergeven.
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
* `I`: Volgend lijstitem.
* `Shift+,`: Naar het begin van de huidige container gaan (lijst of tabel).
* `,`: Voorbij het einde van de huidige container gaan (lijst of tabel).

### Menu Extra

* `Ctrl+W` (macOS: `RawCtrl+W`, dus de fysieke Control-toets in plaats van Cmd): Het aantal woorden van het huidige document weergeven.
* `Ctrl+I`: Documentinformatie weergeven.
* `Ctrl+T`: Inhoudsopgave weergeven.
* `F7`: Elementenlijst weergeven.
* `Ctrl+Shift+C`: Bijbehorende map openen.
* `Ctrl+Shift+V`: Huidige inhoud openen in Web View.
* `Ctrl+U`: De brontekst van het document in een nieuw tabblad weergeven.
* `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
* `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
* `Ctrl+E`: Het huidige document exporteren naar platte tekst.
* `Ctrl+Shift+B`: Bladwijzer op de huidige selectie/cursorpositie in- of uitschakelen.
* `Ctrl+Shift+N`: Bladwijzernotitie toevoegen of bewerken op de huidige selectie/cursorpositie.
* `Ctrl+Alt+W`: Tekstterugloop in- of uitschakelen.
* `Ctrl+Space`: Audioverhaal starten/pauzeren.
* `'`: Vooruit spoelen in het audioverhaal.
* `;`: Achteruit spoelen in het audioverhaal.
* `Ctrl+'`: De spoelstap voor audio vergroten.
* `Ctrl+;`: De spoelstap voor audio verkleinen.
* `F11` (macOS: `RawCtrl+Ctrl+F`, dus Control+Command+F): Volledig scherm in- of uitschakelen.
* `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu).
* `Ctrl+Shift+S`: Slaaptimer in- of uitschakelen.

### Menu Help

* `Ctrl+F1`: Het dialoogvenster Over weergeven.
* `F1`: Help weergeven in je standaardbrowser.
* `Shift+F1`: Help weergeven in Paperback.
* `Ctrl+Shift+U`: Controleren op updates.
* `Ctrl+D`: De donatiepagina openen in je standaardbrowser.

### Extra toetsen in de documentweergave

* `Delete` / `Numpad Delete` op het tabbladbesturingselement: Het geselecteerde documenttabblad sluiten.
* `Enter` of `Space` in de documenttekst: De link op de cursorpositie activeren, of een tabelweergave openen wanneer je op een tabelmarkering staat.
* `Shift+F10` of de Menu-/Applicatietoets in de documenttekst: Het contextmenu openen.

## Ondersteunde talen

Paperback is vertaald in veel verschillende talen, en er komen er steeds meer bij. Hieronder volgt een volledige lijst.

Lees onze [Vertaalgids](translating.md) om te weten hoe je kunt bijdragen.

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

## Met dank aan
### Ontwikkeling
* Quin Gillespie: hoofdontwikkelaar en oprichter van het project.
* Aryan Choudhary: belangrijkste bijdrager.

### Donaties
De volgende mensen hebben een donatie van enige omvang gedaan aan de ontwikkeling van Paperback. Als je een donatie doet, wordt je naam niet automatisch hier toegevoegd; ik voeg alleen mensen toe die willen dat hun donatie openbaar wordt gemaakt.

Opmerking: ik beschouw een openbare GitHub-sponsor als reden voor automatische opname in deze lijst.

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

### Versie 0.9.0

#### Toegevoegd

##### Algemeen
* Een CLI-tool, genaamd pb, om snel elk door Paperback ondersteund formaat te converteren naar HTML, Markdown of platte tekst.
* Een optie om documenten te herladen die door andere programma's op de schijf zijn gewijzigd.
* Een optie "Bron weergeven" om de bron van een document in een nieuw tabblad te openen, bijvoorbeeld handig om Markdown te bewerken.
* Documenttekst wordt nu gepagineerd, wat betekent dat je boeken met tientallen miljoenen woorden nu in slechts een paar seconden kunt laden. Meld eventuele vreemdheden die je hierbij tegenkomt.

##### Platformondersteuning
* Ondersteuning voor ARM64 Windows!
* Native ondersteuning voor macOS!
* Een schakelaar voor volledig scherm.

##### Dialoogvenster Alle documenten
* Een knop om ontbrekende boeken te lokaliseren waarvan het pad net is gewijzigd.
* Een statusfilter en statusbalk, zodat je op documentstatus kunt filteren en kunt zien hoeveel documenten worden weergegeven en geselecteerd zijn.
* De sneltoets `Ctrl+Shift+A` om de selectie van alle documenten op te heffen.

##### Opties en leesbaarheid
* Een tabblad leesbaarheid, met de volgende opties:
    * Tekstterugloop (verplaatst van algemeen);
    * Tabellen inline weergeven (nieuw in deze release, zie hieronder);
    * Lettertype;
    * Achtergrondkleur;
    * Regelafstand;
    * Alinea-afstand;
    * Letterafstand;
    * Tekstuitlijning.
* Een menu-item voor tekstterugloop en een bijbehorende sneltoets.
* Een schakelaar om te bepalen hoe je tabellen wilt weergeven, en een uniforme weergave van tabellen in alle documenten.

##### Navigatie
* Ondersteuning voor navigeren per container.
* Een optie om de cursor automatisch naar het begin van de regel te verplaatsen bij het navigeren tussen regels, vergelijkbaar met de leesmodus in schermlezers.
* De sneltoets is-gelijk-teken om je huidige percentage in een document aan te kondigen.

##### Bladwijzers
* Tijdelijke bladwijzers: je kunt er één per document hebben, en ze blijven bewaard. Gebruik de schuine streep om er een te plaatsen en de omgekeerde schuine streep om ernaartoe te gaan.

##### Woordenteller
* Geschatte leestijd in het dialoogvenster woordenteller, evenals de mogelijkheid om je leessnelheid in te stellen zodat deze waarde daadwerkelijk nuttig wordt.
* Als er een selectie actief is wanneer je het dialoogvenster woordenteller opent, wordt nu weergegeven hoeveel woorden je hebt geselecteerd.

##### Sneltoetsen
* De mogelijkheid om elke sneltoets in de app aan te passen via een eenvoudig dialoogvenster.
* Een instelbare sneltoets om Paperback vanuit het systeemvak te herstellen.

##### Talen
* Nederlands, Fins en Pools.

##### Exporteren
* Het menu-item exporteren is uitgebreid zodat je naast platte tekst ook naar HTML en Markdown kunt exporteren.

##### Updater
* Een annuleerknop in het dialoogvenster voor een lopende update.
* De updater controleert nu of er niet met het gedownloade bestand is gemanipuleerd.

##### Webweergave
* De webweergave wordt nu geopend op je huidige leespositie.

##### DAISY-boeken
* Ondersteuning voor DAISY 2.0-boeken.
* Ondersteuning voor audioweergave van DAISY 2.02.

##### Audioboeken
* De mogelijkheid om audioboeken te spelen, momenteel met ondersteuning voor zowel DAISY-audio (inclusief DAISY-audio + tekst) als zips met audiobestanden.
* Sneltoetsen en menu-items om de narratie te starten/pauzeren, vooruit en achteruit te spoelen en de spoelafstand aan te passen.
* Opties om de leescursor te synchroniseren met de audioweergave, de spoelafstand in te stellen en te kiezen of het spoelen voorbij het einde van een hoofdstuk doorgaat naar het volgende.

##### CHM-documenten
* Ondersteuning voor lijsten, lijstitems, figuren en afbeeldingen.

##### PowerPoint
* PowerPoint-documenten ondersteunen nu tabellen.

#### Opgelost

##### Algemeen
* Documenten die gecodeerd zijn in verouderde CJK-coderingen, zoals GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van als een hoop mojibake.
* "Laatst gesloten opnieuw openen" dat probeerde de meegeleverde readme opnieuw te openen.
* Je geselecteerde tabblad kreeg niet correct de focus na het herstarten van Paperback.
* De manier waarop Paperback omgaat met bestanden op Windows-netwerkschijven: bij "bestand in map weergeven" krijgt het bestand op de netwerkopslag nu correct de focus, en de paden bevatten geen vreemde tekens meer.
* .paperback-bestanden worden niet langer gedwongen geladen bij het herstellen van documenten; in plaats daarvan wordt om bevestiging gevraagd wanneer er een wordt gevonden.
* "Bijbehorende map openen" geeft het betreffende bestand nu de focus in de verkenner.
* Bij het openen van de readme wordt nu je gekozen taal gerespecteerd.
* De gebruikersinterface van Paperback wordt nu correct geschaald op beeldschermen met hoge DPI.
* Het menu wordt nu correct bijgewerkt en de focus gaat naar het tekstveld wanneer je de help in Paperback opent.
* Overgestapt op een veel veiligere methode van IPC op Windows.
* De titel van het actieve document wordt nu voorgelezen bij het wisselen tussen tabbladen.
* Verminderd geheugengebruik bij grote documenten door de grootte van de interne indextabellen per teken te halveren.

##### Dialoogvenster Alle documenten
* Escape sloot de dialoogvensters Documentinfo en Alle documenten niet.
* De titelbalk werd niet bijgewerkt na het sluiten van een document via het dialoogvenster Alle documenten.
* Readme.html wordt niet langer toegevoegd aan je lijst met alle documenten wanneer je het opent via Shift+F1.
* Het verwijderen van documenten uit het dialoogvenster recente documenten sluit nu ook hun actieve tabblad.
* Je zoekfilter blijft nu bewaard na het verwijderen van een document.

##### Navigatie
* Paginanavigatie kondigde in sommige situaties de verkeerde regeltekst aan.
* Ga naar regel, Ga naar pagina en Ga naar percentage plaatsten je cursor op de verkeerde positie in grote documenten.
* Zoeken en Volgende zoeken respecteerden het geladen documentvenster niet in grote documenten.

##### Bladwijzers
* Bladwijzer-/notitiegeluiden zouden nu uitsluitend moeten klinken wanneer je over een woord navigeert dat er een bevat.

##### Leesbaarheid
* Het toepassen van tekstterugloop bracht je naar het begin van je document.

##### Webweergave
* Het dialoogvenster van de webweergave was niet in grootte aanpasbaar en verscheen met een zeer kleine beginafmeting.
* Afbeeldingen zouden nu correct moeten worden weergegeven in de ingebedde webweergave.

##### Updater
* De updater geeft nu de inhoud van markdown-codetags in releasenotes correct weer.

##### DAISY-boeken
* DAISY-boeken toonden onjuiste informatie in de statusbalk.
* Het laden van DAISY-boeken met onjuiste coderingsdeclaraties.

##### RTF-documenten
* Het verwerken van RTF-documenten met niet-Latijnse tekens erin.
* RTF `\pict`-groepen, zodat ingebedde afbeeldingsgegevens niet langer in de documenttekst terechtkomen.

##### Mobi/AZW3-boeken
* Filepos-ankers in Mobi-boeken die HTML-tags opsplitsten en rommel in de boektekst plaatsten.
* Links in verouderde Mobi-boeken.
* Het verwerken van AZW3 is aanzienlijk verbeterd.

##### Word-documenten
* Word-documenten met landspecifieke stijlnamen gaven hun koppen niet correct weer.

##### HTML/XHTML-documenten
* dl-, dt- en dd-elementen produceerden geen regeleinden in XHTML-documenten.

##### PDF-documenten
* Paperback valt nu terug op extractie van platte tekst voor onjuist getagde PDF's.
* PDF-documenten met stuurtekens in hun titels en/of bladwijzers laten Paperback niet langer crashen bij het openen.

### Versie 0.8.5
* Pagina-ondersteuning toegevoegd aan epub-boeken.
* Ondersteuning toegevoegd voor versleutelde Microsoft Office-documenten. Momenteel worden verouderd Word, modern Word en modern PowerPoint ondersteund, met verouderd PowerPoint gepland voor de toekomst.
* Ondersteuning toegevoegd voor verouderde Microsoft Word-documenten (*.doc)!
* Ondersteuning toegevoegd voor verouderde PowerPoint-presentaties (*.ppt)!
* Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
* Ondersteuning toegevoegd voor getagde PDF-bestanden!
* De sneltoets ctrl+q toegevoegd om de app te sluiten.
* Ondersteuning toegevoegd voor gezipte boeken van Bookshare (zowel DAISY als Word)!
* Alt-tekst voor ingebedde afbeeldingen zou nu correct moeten worden weergegeven.
* CHM-documenten ondersteunen nu correct navigatie via interne links.
* Opgelost dat bladwijzergeluiden aan het begin van de alinea afgingen in plaats van op de positie van de bladwijzer.
* Opgelost dat "ga naar pagina" er 1 naast zat.
* Opgelost dat de escapetoets niet werkte om het dialoogvenster "openen als" te sluiten.
* Opgelost dat het contextmenu van de lezer niet verscheen bij een rechtsklik of met de Toepassingen-toets.
* Opgelost dat soms het verkeerde document de focus kreeg bij het openen van documenten via de opdrachtregel.
* PDF's die alleen afbeeldingen bevatten worden weer gedetecteerd en waarschuwen je voor hun bestaan.
* Het is nu mogelijk om door afbeeldingen en figuren te navigeren met respectievelijk g/shift+g en f/shift+f.
* Paperback respecteert nu je instelling voor de donkere modus van applicaties.
* Ondersteuning voor DAISY XML verwijderd, aangezien deze niet langer nodig is.
* Teruggeschakeld naar de native Win32-navigatie op eerste letter in de inhoudsopgaveboom.
* Het dialoogvenster voor laadfouten toont nu gedetailleerdere foutmeldingen.
* De webweergave opent nu veel sneller en soepeler.

### Versie 0.8.2
* Pagina-ondersteuning toegevoegd aan RTF-documenten!
* Een bug opgelost waarbij het openen van de webweergave in epubs met externe links deze automatisch activeerde.
* Een bug opgelost waarbij de RTF-parser in zeldzame gevallen geen spatie tussen woorden plaatste.
* Opgelost dat alinea's in sommige PDF-documenten in meerdere korte regels werden opgesplitst.
* PDF-documenten hebben nu basisondersteuning voor navigatie via links en koppen!
* RTF-tabs en regeleinden worden nu exact weergegeven zoals ze in het document staan.
* Teruggeschakeld naar de vertrouwde pdfium-bibliotheek voor het verwerken van PDF's, waardoor de PDF-weergave weer veel betrouwbaarder is.

### Versie 0.8.1
* Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te openen.
* Het dialoogvenster Alle documenten ondersteunt nu het selecteren van meerdere documenten om tegelijk te openen.
* Een paar bugs in de RTF-parser opgelost.
* Opgelost dat bestandspaden met niet-ASCII-tekens (zoals de Bosnische š, č, ć, ž) beschadigd raakten bij het openen van een bestand via een tweede Paperback-instantie.
* Opgelost dat PDF-tekst in de verkeerde volgorde werd voorgelezen, en onjuiste spatiëring rond woorden met hoofdletters.
* Opgelost dat documenten langzaam laadden bij het openen van grote bestanden.
* De vertaling van de Ja/Nee-knoppen in bevestigingsdialogen opgelost.

### Versie 0.8.0
* Japanse, vereenvoudigd Chinese en Vietnamese vertalingen toegevoegd!
* Een automatische updater toegevoegd die nu je huidig geïnstalleerde versie van Paperback vervangt in plaats van alleen de nieuwe versie te downloaden!
* Optionele geluidsfeedback toegevoegd voor het bereiken van een bladwijzer of notitie, met dank aan Andre Louis voor de geluiden!
* Ondersteuning voor RTF-documenten toegevoegd!
* Ondersteuning toegevoegd voor DAISY XML-documenten.
* Ondersteuning toegevoegd voor Flat Open Document Text-bestanden!
* Ondersteuning toegevoegd voor Flat Open Document-presentaties!
* Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
* Elke beweging van meer dan 300 tekens wordt nu automatisch toegevoegd aan je navigatiegeschiedenis.
* Het herstellen van het Paperback-venster vanuit het systeemvak opgelost.
* Opgelost dat Markdown-documenten ruwe tekst toonden in plaats van weergegeven HTML in de webweergave.
* Opgelost dat tabellen niet correct werden weergegeven in Markdown-bestanden.
* PDF's die alleen afbeeldingen bevatten waarschuwen je nu voor hun bestaan wanneer je er een probeert te laden.
* Het is nu mogelijk om op nieuwe dev-builds te controleren in plaats van stabiele releases bij het zoeken naar updates.
* Versie-informatie wordt nu correct ingebed in het uitvoerbare bestand van Paperback.
* Het dialoogvenster met opties opgesplitst in tabbladen voor gebruiksgemak en navigatie.
* Overgeschakeld naar Hayro voor het verwerken van PDF's, wat leidt tot meer betrouwbaarheid, snelheid en minder DLL's.
* De hele app herschreven in Rust. De nieuwe codebase is veiliger, laadt documenten sneller en is eenvoudiger te onderhouden en uit te breiden.
* Het contextmenu van het tekstveld bevat nu lezer-specifieke acties in plaats van algemene items zoals knippen en plakken.

### Versie 0.7.0
* Tabelondersteuning toegevoegd voor HTML- en XHTML-gebaseerde documenten! Navigeer tussen tabellen met T en Shift+T, en druk op Enter om er een in een webweergave te bekijken.
* Een eenvoudige webweergavefunctie toegevoegd! Druk op Ctrl+Shift+V om het huidige gedeelte van je document in een webgebaseerde renderer te openen, handig voor inhoud zoals complexe opmaak of codevoorbeelden.
* Een Russische vertaling toegevoegd, met dank aan Ruslan Gulmagomedov!
* Een knop "Alles wissen" toegevoegd aan het dialoogvenster Alle documenten.
* De updatecontrole toont nu releasenotes wanneer er een nieuwe versie beschikbaar is.
* Het herstellen van het venster vanuit het systeemvak opgelost.
* Vertalingen van de Ja/Nee-knoppen in bevestigingsdialogen opgelost.
* Het laden van configuraties bij uitvoeren als beheerder opgelost.
* De verwerking van opmerkingen in XML- en HTML-documenten opgelost.
* Het verwerken van de inhoudsopgave in Epub 2-boeken opgelost.
* Het navigeren naar het volgende item met dezelfde letter in de inhoudsopgave opgelost.
* Opgelost dat het zoekdialoogvenster niet correct werd verborgen bij gebruik van de knoppen volgende/vorige.
* Opgelost dat epub-inhoudsopgaven je soms naar het verkeerde item brachten.
* Diverse problemen met de verwerking van witruimte in XML, HTML en pre-tags opgelost.
* Een off-by-one-fout in linknavigatie opgelost.
* Opgelost dat sommige boeken overtollige witruimte aan het einde van hun regels hadden.
* Diverse parserproblemen opgelost.
* Menu-items met betrekking tot bladwijzers en de elementenlijst worden nu correct uitgeschakeld wanneer er geen document open is.
* De verwerking van lijsten in diverse documentformaten verbeterd.
* De vertaalworkflow voor bijdragers verbeterd.
* Veel interne refactors, waarbij het merendeel van de bedrijfslogica van de applicatie van C++ naar Rust is verplaatst voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1
* Ondersteuning voor met een wachtwoord beveiligde PDF's toegevoegd!
* Een zeer eenvoudige functie voor "ga naar vorige/volgende positie" toegevoegd. Als je op enter drukt op een interne link en je cursor verplaatst wordt, wordt die positie nu onthouden en kun je er met alt+pijltjes links/rechts naartoe navigeren.
* Een elementenlijst toegevoegd! Momenteel toont deze alleen een boomstructuur van alle koppen in je document of een lijst met links, maar er zijn plannen om dit in de toekomst uit te breiden.
* Een optie toegevoegd om Paperback standaard gemaximaliseerd te starten.
* Opgelost dat links in sommige Epub-documenten niet correct werkten.
* Het verwerken van Epub-inhoudsopgaven met relatieve paden opgelost.
* Opgelost dat sommige epub-documenten geen titel of auteur toonden.
* Opgelost dat de titels van sommige epub-hoofdstukken niet correct in het inhoudsopgavedialoogvenster werden weergegeven.
* Opgelost dat je de spatiebalk niet kon gebruiken om de OK-/annuleerknoppen in het inhoudsopgavedialoogvenster te activeren.
* De verwerking van koppen in Word-documenten verbeterd.
* Je krijgt nu gesproken feedback als de lijst met recente documenten leeg is wanneer je het dialoogvenster probeert te openen.

### Versie 0.6.0
* Een nieuwe optie om het ga-menu in een veel compactere vorm weer te geven is toegevoegd aan het dialoogvenster met opties, standaard aangevinkt.
* Een optie toegevoegd om navigatie per structuurelement te laten doorlopen.
* Een optie toegevoegd aan het menu Extra om de map van het momenteel gefocuste document te openen.
* Een vrij eenvoudig, maar zeer effectief updatesysteem toegevoegd.
* Een eenvoudige slaaptimerfunctie toegevoegd, toegankelijk met Ctrl+Shift+S.
* Ondersteuning toegevoegd voor het verwerken van FB2-ebooks!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument-presentaties!
* Ondersteuning toegevoegd voor het verwerken van OpenDocument Text-bestanden!
* Bladwijzers kunnen nu een hele regel markeren, of alleen bepaalde opgegeven tekst. Als je geen selectie actief hebt bij het plaatsen van een bladwijzer, is het gedrag zoals voor 0.6 en wordt de hele regel gemarkeerd. Als je echter tekst selecteert, wordt alleen die tekst in de bladwijzer opgenomen.
* Bladwijzers kunnen nu optionele tekstnotities hebben! Navigeer tussen bladwijzers met notities met N en Shift+N, of open het bladwijzerdialoogvenster met alle bladwijzers, alleen notities of alleen niet-notities geselecteerd via specifieke sneltoetsen.
* Bladwijzers in het bladwijzerdialoogvenster hebben niet langer een irritant "bladwijzer x"-voorvoegsel.
* Epub-boeken met HTML-inhoud die zich voordoet als XML worden nu correct verwerkt.
* Het laden van grote Markdown-documenten opgelost.
* Opgelost dat het indrukken van de spatiebalk in de inhoudsopgaveboom de OK-knop activeerde.
* De verwerking van witruimte aan het begin van pre-tags in zowel HTML- als XHTML-documenten opgelost.
* Opgelost dat het tekstveld soms de focus niet terugkreeg bij het terugkeren naar het Paperback-venster.
* Opgelost dat het tekstveld in het dialoogvenster "ga naar percentage" de waarde van de schuifregelaar niet bijwerkte.
* De weergave van aangepaste HTML-ID's in Markdown-documenten opgelost.
* HTML binnen Markdown-codeblokken wordt nu correct weergegeven.
* Als je een boek laadt met een opdrachtregelparameter terwijl er al een Paperback-instantie draait, krijg je geen foutmelding meer als het laden van je document meer dan 5 seconden duurt.
* Als je Paperback als beheerder uitvoert, wordt de configuratie nu correct geladen en opgeslagen.
* Het is nu mogelijk om een bladwijzer direct vanuit het bladwijzerdialoogvenster te verwijderen.
* Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald document te importeren en exporteren. Het gegenereerde bestand is vernoemd naar het bestand met de extensie .paperback. Als zo'n bestand wordt gevonden in dezelfde map als een bestand tijdens het laden, wordt het automatisch geladen. Anders kun je ze handmatig importeren via een item in het menu Extra.
* Links binnen documenten worden nu volledig ondersteund! Gebruik k en shift+k om er vooruit en achteruit door te bewegen, en druk op enter om er een te openen/activeren.
* Veel interne refactors, waardoor de app sneller en het binaire bestand kleiner is.
* Markdown-inhoud wordt nu voorbewerkt om CommonMark-conform te zijn voordat het wordt weergegeven.
* Navigatie per lijst en lijstitem wordt nu volledig ondersteund! Gebruik L en Shift+L voor de lijsten zelf, en I en Shift+I om door lijstitems te gaan.
* De delete-toets op het numerieke toetsenblok werkt nu ook om documenten uit de tabbalk te verwijderen, naast de normale delete.
* Paperback kan nu optioneel minimaliseren naar je systeemvak! Deze optie staat standaard uit, maar als je hem aanzet plaatst de minimaliseeroptie in het systeemmenu Paperback in je systeemvak, waarna het kan worden hersteld door op het gegenereerde pictogram te klikken.
* Paperback is nu volledig vertaalbaar! De lijst met ondersteunde talen is momenteel vrij klein, maar groeit constant!
* Paperback heeft nu een officiële website, op [paperback.dev](https://paperback.dev)!
* PPTX-documenten tonen nu een eenvoudige inhoudsopgave met alle dia's.
* Het volledige pad naar het geopende document wordt nu weergegeven in het dialoogvenster documentinfo.
* Het installatieprogramma bevat nu een optie om de readme na installatie in je browser te bekijken.
* De lijst met recente documenten is drastisch uitgebreid! In plaats van alleen de laatste 10 documenten te tonen die je hebt geopend, toont het nu een aanpasbaar aantal, waarbij de rest van de documenten die je ooit hebt geopend toegankelijk zijn via een klein dialoogvenster.
* Diverse kleine verbeteringen aan de parsers in het algemeen, waaronder het plaatsen van een blanco regel tussen dia's in PPTX-presentaties, het oplossen van de verwerking van regeleinden binnen alinea's in Word-documenten en het toevoegen van opsommingstekens aan lijstitems.

### Versie 0.5.0
* Ondersteuning voor Microsoft Word-documenten toegevoegd!
* Ondersteuning voor PowerPoint-presentaties toegevoegd!
* Opgelost dat bepaalde menu-items niet werden uitgeschakeld wanneer er geen documenten open waren.
* De oriëntatie van de schuifregelaar "ga naar percentage" opgelost.
* De inhoudsopgave in Epub-boeken met URL-gecodeerde bestandspaden en/of fragment-ID's opgelost.
* Opgelost dat witruimte op vreemde manieren uit XHTML-koppen werd verwijderd.
* De verwerking van witruimte binnen geneste pre-tags in HTML-documenten opgelost.
* HTML- en Markdown-documenten ondersteunen nu de inhoudsopgavefunctie! Wanneer je een HTML-/Markdown-document laadt, bouwt Paperback zijn eigen inhoudsopgave op uit de structuur van de koppen in je document, en toont die in het ctrl+t-dialoogvenster.
* HTML-documenten krijgen nu de titel zoals ingesteld in de title-tag, als die bestaat. Anders blijven ze de bestandsnaam zonder extensie gebruiken.
* Overgestapt van UniversalSpeech naar het gebruik van een live region om spraak te melden. Dit betekent dat er geen schermlezer-DLL's meer met het programma worden meegeleverd en dat er nu meer schermlezers worden ondersteund, zoals Microsoft Verteller.
* Van zip-bibliotheek gewisseld om een breder scala aan epub-boeken te kunnen openen.
* Het dialoogvenster dat je vraagt of je je document als platte tekst wilt openen is volledig vernieuwd en laat je nu je document openen als platte tekst, HTML of Markdown.
* Het dialoogvenster "ga naar percentage" bevat nu een tekstveld waarin je handmatig een percentage kunt invoeren om naartoe te springen.
* De HTML-parser herkent nu dd, dt en dl als lijstelementen.
* De inhoudsopgave in Epub-boeken wordt weer exact behouden.
* De unicode non-breaking space wordt nu meegenomen bij het verwijderen van blanco regels.
* Je wordt niet langer elke keer gevraagd hoe je een onbekend bestand wilt openen, alleen de eerste keer.

### Versie 0.4.1
* Een optioneel startmenupictogram toegevoegd aan het installatieprogramma.
* De inhoudsopgave zou nu in een aantal gevallen schoner moeten zijn, bijvoorbeeld als je een onderliggend en bovenliggend item met dezelfde tekst op dezelfde positie hebt, zie je nu alleen het bovenliggende item.
* De inhoudsopgave in bepaalde CHM-documenten opgelost.
* De inhoudsopgave in Epub 3-boeken met absolute paden erin opgelost.
* CHM-documenten zouden nu hun titel moeten tonen zoals ingesteld in het metadatabestand.

### Versie 0.4.0
* Ondersteuning voor CHM-bestanden toegevoegd!
* Ondersteuning voor bladwijzers toegevoegd! Je kunt zoveel bladwijzers in zoveel documenten hebben als je wilt. Je kunt er vooruit en achteruit door springen met b en shift+b, er een instellen met control+shift+b, en een dialoogvenster openen om naar een specifieke bladwijzer te gaan met control+b.
* Een installatieprogramma toegevoegd naast het portable zipbestand! Het installatieprogramma installeert Paperback in je Program Files-map en stelt automatisch bestandskoppelingen voor je in.
* Tekstbestanden met BOM's zouden nu correct moeten worden gedecodeerd, en de BOM wordt ook niet langer aan het begin van de tekst weergegeven.
* Veel meer informatie toegevoegd aan de statusbalk. Deze toont nu je huidige regel, teken en leespercentage.
* HTML-opmerkingen, evenals de inhoud van script- en style-tags, worden niet langer in de tekstuitvoer weergegeven.
* Als je een relatief pad aan Paperback doorgeeft via de opdrachtregel, wordt dit nu correct opgelost.
* Beweging op percentage wordt nu verwerkt door een eigen dialoogvenster met schuifregelaar, toegankelijk met control+shift+g.
* Documenten zonder bekende titels of auteurs hebben nu altijd een standaardwaarde.
* De logica voor het opslaan van de positie is nu veel slimmer en zou alleen naar de schijf moeten schrijven wanneer dat absoluut noodzakelijk is.
* Het document dat de focus had toen je Paperback afsloot wordt nu onthouden tussen het herstarten van de applicatie.
* Invoer in de dialoogvensters "ga naar regel" en "ga naar pagina" wordt nu strenger gecontroleerd.
* Navigatie in de inhoudsopgave van epub 3-boeken met relatieve paden in hun manifesten opgelost.

### Versie 0.3.0
* De inhoudsopgave in epub-boeken met URL-gecodeerde manifesten opgelost.
* Koppennavigatie in HTML-documenten met multi-byte Unicode-tekens opgelost.
* Hoog CPU-gebruik in documenten met lange titels opgelost, veroorzaakt door een regressie in wxWidgets.
* Het laden van UTF-8-tekstbestanden opgelost.
* Opgelost dat geneste inhoudsopgave-items in Epub-boeken je cursor op de verkeerde positie plaatsten.
* Een crash bij het afsluiten van de applicatie in bepaalde gevallen opgelost.
* Een aanvinkvakje toegevoegd in het dialoogvenster met opties om tekstterugloop in of uit te schakelen!
* Het is nu mogelijk om te doneren aan de ontwikkeling van Paperback, via het nieuwe donatie-item in het menu Help of via de link "sponsor this project" onderaan de hoofdpagina van de GitHub-repository.
* Markdown-documenten hebben nu altijd een titel, en Paperback zou nu vrijwel elk Markdown-bestand moeten kunnen laden.
* PDF-documenten hebben nu altijd een titel, zelfs als de metadata ontbreekt.
* Overgestapt naar de PDF-bibliotheek die in Chromium wordt gebruikt, wat leidt tot veel betrouwbaardere PDF-verwerking over de hele lijn.
* Je kunt nu maar één instantie van Paperback tegelijk uitvoeren. Als je paperback.exe met een bestandsnaam uitvoert terwijl het al draait, wordt dat document in de al draaiende instantie geopend.
* Je kunt nu op delete drukken bij een document in het tabbladbesturingselement om het te sluiten.

### Versie 0.2.1
* Het totale aantal pagina's toegevoegd aan het pagina-label in het dialoogvenster "ga naar pagina".
* Tabben van de documentinhoud naar je lijst met geopende documenten toegestaan.
* Opgelost dat de koppen-sneltoetsen soms recente documenten openden als je er genoeg van had.
* Paperback verwijdert nu onnodige zachte afbreekstreepjes uit de tekstuitvoer.
* Opgelost dat koppennavigatie je soms op het verkeerde teken plaatste.

### Versie 0.2.0
* Ondersteuning voor markdown-documenten toegevoegd!
* Ondersteuning voor PDF-documenten toegevoegd, inclusief de mogelijkheid om tussen pagina's te navigeren!
* Sneltoetsen toegevoegd voor navigatie per kop in HTML-inhoud, waaronder epub-boeken en markdown-documenten. Deze sneltoetsen zijn ontworpen om vergelijkbaar met een schermlezer te werken.
* Het laden van epubs met URL-gecodeerde bestandsnamen in hun manifesten opgelost.
* Het laden van epub 3-boeken met daarin ingebedde XHTML opgelost.
* Er wordt nu een bericht gesproken als het document geen inhoudsopgave of secties ondersteunt, in plaats van dat de menu-items worden uitgeschakeld.
* Een menu met recente documenten toegevoegd! Het bewaart momenteel je laatste 10 geopende documenten, en op enter drukken bij een ervan opent het om te lezen.
* Het zoekdialoogvenster volledig herschreven, waardoor het veel eenvoudiger in gebruik is, met daarnaast een geschiedenis van je laatste 25 zoekopdrachten en ondersteuning voor reguliere expressies!
* Eerder geopende documenten worden nu onthouden tussen het herstarten van de applicatie. Dit is instelbaar via het nieuwe optie-item in het menu Extra.
* Shift+F1 toegevoegd om de readme direct in Paperback zelf te openen.

### Versie 0.1.0
* Eerste release.
