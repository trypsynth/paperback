<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Paperback - versie 0.8.5 {#paperback---version-0.8.5}

## Inleiding {#introduction}

Paperback is een lichtgewicht, snelle en toegankelijke e-boek- en
documentlezer voor iedereen, van gelegenheidslezers tot intensieve
gebruikers. Het is ontworpen met het oog op toegankelijkheid voor
schermlezers, hoge snelheden en een overzichtelijke gebruikerservaring.

## Systeemvereisten {#system-requirements}

Paperback draait momenteel op Windows, macOS, iOS en Android.

## Functies {#features}

-   Volledig zelfstandig, er hoeft geen software op je computer te
    worden geïnstalleerd om te beginnen met lezen.
-   Ongelooflijk snel, zelfs op oude hardware.
-   Eenvoudige interface met tabbladen, waarmee je zoveel documenten
    kunt openen als je wilt, naast elkaar.
-   Slaat je exacte leespositie op in elk document dat je opent.
-   Kan desgewenst onthouden welke documenten je open had staan toen je
    het programma sloot, en deze bij de volgende keer opstarten weer
    openen.
-   Bevat navigatiefuncties die vergelijkbaar zijn met die in de web
    browsing-modus van veel schermlezers, om snel en gemakkelijk door
    documenten te navigeren.
-   Bevat een robuust zoekvenster, inclusief functies zoals een
    zoekgeschiedenis en ondersteuning voor reguliere expressies.
-   Kan volledig draagbaar worden uitgevoerd, of geïnstalleerd met
    automatisch ingestelde bestandsassociaties.
-   Ondersteunt een enorm scala aan gangbare bestandsformaten.

## Compatibiliteit met schermlezers {#screen-reader-compatibility}

Paperback werkt goed met alle gangbare schermlezers. Er is echter één
bekend probleem voor JAWS-gebruikers.

### JAWS en brailleleesregels {#jaws-and-braille-displays}

Als u JAWS met een braillescherm gebruikt, kunt u merken dat lange
alinea's worden afgekapt wanneer u met de navigatietoetsen van uw scherm
vooruit bladert. Dit geldt ook voor de opdracht 'huidige alinea
voorlezen'. Dit is een bug in de manier waarop JAWS omgaat met het
RICHEDIT50W-tekstveld, niet iets in Paperback zelf, en het heeft
behoorlijk lang geduurd voordat er een oplossing voor kwam, gezien het
enthousiasme van Vispero om te reageren op problemen met
open-source-software.

De tijdelijke oplossing, die uiteindelijk na maanden wachten via de
JAWS-discussiegroep naar voren kwam, is om `paperback.jcf` en
"Braille-weergave en pannen" in te stellen op "Altijd DOM gebruiken
indien beschikbaar". Je moet ook "Tekst per alinea pannen" inschakelen,
anders blijft je scherm op de actieve alinea staan in plaats van verder
te gaan. Met beide instellingen ingeschakeld, zou het pannen correct
moeten werken.

## Momenteel ondersteunde bestandsformaten {#currently-supported-file-types}

Paperback ondersteunt de volgende formaten en extensies:

-   CHM-helpbestanden (`.chm`)
-   DAISY-boeken (`.opf`, `.zip`)
-   EPUB-boeken (`.epub`)
-   FB2-e-boeken (`.fb2`)
-   HTML-documenten (`.htm`, `.html`, `.xhtml`)
-   Markdown-documenten (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Microsoft Word-documenten (`.docx`, `.docm`, `.doc`)
-   MOBI/Kindle-boeken (`.mobi`, `.azw`, `.azw3`)
-   OpenDocument-presentaties (`.odp`, `.fodp`)
-   OpenDocument-tekstbestanden (`.odt`, `.fodt`)
-   PDF-documenten (`.pdf`)
-   PowerPoint-presentaties (`.pptx`, `.pptm`, `.ppt`)
-   RTF-documenten (`.rtf`)
-   Platte tekst en logbestanden (`.txt`, `.log`)

## Sneltoetsen {#keyboard-shortcuts}

Paperback is ontworpen voor gebruik waarbij het toetsenbord centraal
staat. Hieronder staan de huidige sneltoetsen.

De onderstaande sneltoetsen gelden voor Windows. Waar macOS hiervan
afwijkt, wordt de equivalente toetscombinatie tussen haakjes vermeld ---
voornamelijk omdat Ctrl+G, Ctrl+W en Alt+Links/Rechts al in gebruik zijn
door andere systeem- of app-conventies op dat platform.

### Menu „Bestand" {#file-menu}

-   `Ctrl+O`: Een document openen.
-   `Ctrl+F4` (macOS: `Cmd+W`): Sluit het huidige document.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Sluit alle geopende
    documenten.
-   `Ctrl+Shift+T`: Open het laatst gesloten document opnieuw.
-   `Ctrl+R`: Open het dialoogvenster „Alle documenten" (vanuit „Recente
    documenten").
-   `Ctrl+Q`: Afsluiten (alleen Windows; op macOS staat deze optie in
    het app-menu).

### Menu 'Ga' {#go-menu}

-   `Ctrl+F`: Open het dialoogvenster Zoeken.
-   `F3` (macOS: `Cmd+G`): Volgende zoeken.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Vind vorige.
-   `Ctrl+G` (macOS: `Cmd+L`): Ga naar regel.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ga naar percentage.
-   `Ctrl+P`: Ga naar pagina (indien ondersteund door het huidige
    document).
-   `Alt+Left` (macOS: `Cmd+[`): Ga terug in de navigatiegeschiedenis.
-   `Alt+Right` (macOS: `Cmd+]`): Ga vooruit in de
    navigatiegeschiedenis.
-   `[`: Vorige paragraaf.
-   `]`: Volgende paragraaf.
-   `Shift+H`: Vorige kop.
-   `H`: Volgende kop.
-   `Shift+1` via `Shift+6`: Vorige kop op niveau 1-6.
-   `1` tot en met `6`: Volgende kop op niveau 1-6.
-   `Shift+P`: Vorige pagina.
-   `P`: Volgende pagina.
-   `Shift+B`: Vorige bladwijzer.
-   `B`: Volgende bladwijzer.
-   `Shift+N`: Vorige notitie.
-   `N`: Volgende notitie.
-   `Ctrl+B`: Ga naar alle bladwijzers en notities.
-   `Ctrl+Alt+B`: Ga alleen naar bladwijzers.
-   `Ctrl+Alt+M`: Ga alleen naar notities.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, d.w.z. de fysieke
    Control-toets in plaats van Cmd): Bekijk de tekst van de notitie op
    de huidige positie.
-   `Shift+K`: Vorige link.
-   `K`: Volgende link.
-   `Shift+G`: Vorige afbeelding.
-   `G`: Volgende afbeelding.
-   `Shift+F`: Vorige afbeelding.
-   `F`: Volgende figuur.
-   `Shift+T`: Vorige tabel.
-   `T`: Volgende tabel.
-   `Shift+S`: Vorige scheidingsteken.
-   `S`: Volgende scheidingsteken.
-   `Shift+L`: Vorige lijst.
-   `L`: Volgende lijst.
-   `Shift+I`: Vorig lijstitem.
-   `I`: Volgend lijstitem.
-   `Shift+,`: Ga naar het begin van de huidige container (lijst of
    tabel).
-   `,`: Ga voorbij het einde van de huidige container (lijst of tabel).

### Menu 'Extra' {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, d.w.z. de fysieke Control-toets in
    plaats van Cmd): Toon het aantal woorden voor het huidige document.
-   `Ctrl+I`: Toon documentinformatie.
-   `Ctrl+T`: Inhoudsopgave weergeven.
-   `F7`: Lijst met elementen weergeven.
-   `Ctrl+Shift+C`: De bijbehorende map openen.
-   `Ctrl+Shift+V`: Open de huidige inhoud in Web View.
-   `Ctrl+U`: Bekijk de broncode van het document in een nieuw tabblad.
-   `Ctrl+Shift+E`: Documentgegevens exporteren (`.paperback`).
-   `Ctrl+Shift+I`: Documentgegevens importeren (`.paperback`).
-   `Ctrl+E`: Het huidige document exporteren naar platte tekst.
-   `Ctrl+Shift+B`: Bladwijzer in- of uitschakelen bij de huidige
    selectie/cursor.
-   `Ctrl+Shift+N`: Voeg een bladwijzernotitie toe of bewerk deze bij de
    huidige selectie/cursor.
-   `Ctrl+Alt+W`: Woordafbreking in- of uitschakelen.
-   `Ctrl+,`: Opties openen (macOS: Voorkeuren, in het app-menu ).
-   `Ctrl+Shift+S`: Slaaptimer in- of uitschakelen.

### Help-menu

-   `Ctrl+F1`: Toon het dialoogvenster 'Over'.
-   `F1`: Help weergeven in je standaardbrowser.
-   `Shift+F1`: Bekijk de help-pagina in Paperback.
-   `Ctrl+Shift+U`: Controleer op updates.
-   `Ctrl+D`: Open de donatiepagina in je standaardbrowser. :

### Extra toetsen voor het bekijken van documenten op {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` op het tabblad: Sluit het geselecteerde
    documenttabblad.
-   `Enter` of `Space` in de documenttekst: Activeer de link bij de
    cursor, of open een tabelweergave wanneer je op een tabelmarkering
    staat.
-   `Shift+F10` of de Menu-/Toepassings-toets in de documenttekst : Open
    het contextmenu.

## Ondersteunde talen {#supported-languages}

Paperback is vertaald in veel verschillende talen, en er worden
voortdurend nieuwe toegevoegd. Hieronder volgt een volledige lijst.

Lees onze [Vertaalgids](translating.md) om te zien hoe je kunt
bijdragen.

-   Bosnisch
-   Tsjechisch
-   Nederlands
-   Fins
-   Frans
-   Duits
-   Japans
-   Pools
-   Portugees (Brazilië)
-   Russisch
-   Vereenvoudigd Chinees
-   Servisch
-   Spaans
-   Vietnamees

## Colofon {#credits}

### Ontwikkeling {#development}

-   Quin Gillespie: hoofdontwikkelaar en oprichter van het project.
-   Aryan Choudhary: belangrijkste medewerker.

### Donaties {#donations}

De volgende mensen hebben een donatie van enige omvang gedaan aan de
ontwikkeling van Paperback. Als je een donatie doet, wordt je naam niet
automatisch hier toegevoegd; ik voeg alleen mensen toe die willen dat
hun donatie openbaar wordt gemaakt.

Opmerking: ik beschouw een openbare GitHub-sponsor als reden voor
automatische opname in deze lijst.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Felix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodriguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Perez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Wijzigingslogboek {#changelog}

### Versie 0.9.0 (nog niet uitgebracht) {#version-0.9.0-unreleased}

-   Er is een knop \'Annuleren\' toegevoegd aan het dialoogvenster
    \'Update bezig\'.
-   Er is een CLI-tool toegevoegd, genaamd pb, om elk van de door
    Paperback ondersteunde formaten snel te converteren naar HTML,
    Markdown of platte tekst.
-   Er is een configureerbare sneltoets toegevoegd om Paperback te
    herstellen vanuit het systeemvak.
-   Er is een zoekknop toegevoegd aan het dialoogvenster 'Alle
    documenten' om ontbrekende boeken te vinden waarvan het pad net is
    gewijzigd.
-   Er is een tabblad \'Leesbaarheid\' toegevoegd aan het dialoogvenster
    \'Opties\', met de volgende opties:
    -   Tekstafbreking (verplaatst vanuit 'Algemeen');
    -   Tabellen inline weergeven (nieuw in deze release, zie
        hieronder);
    -   Lettertype;
    -   Achtergrondkleur;
    -   Regelafstand;
    -   Alinea-afstand;
    -   Letterafstand;
    -   Tekstuitlijning.
-   Er is een schakelaar toegevoegd om te bepalen hoe tabellen moeten
    worden weergegeven, en de weergave van tabellen is in alle
    documenten gestandaardiseerd.
-   Er is een optie 'Bron weergeven' toegevoegd om de broncode van een
    document in een nieuw tabblad te openen, wat bijvoorbeeld handig is
    voor het bewerken van Markdown.
-   De geschatte leestijd is toegevoegd aan het dialoogvenster voor het
    tellen van woorden, evenals de mogelijkheid om je leessnelheid in te
    stellen, zodat deze maatstaf daadwerkelijk bruikbaar is.
-   Ondersteuning voor ARM64 op Windows toegevoegd!
-   Android-ondersteuning toegevoegd!
-   Er is ondersteuning voor iOS toegevoegd!
-   Ondersteuning voor macOS toegevoegd!
-   Nieuwe talen toegevoegd: Nederlands, Fins en Pools.
-   Ondersteuning toegevoegd voor navigeren per container.
-   Ondersteuning toegevoegd voor lijsten, lijstitems, figuren en
    afbeeldingen in CHM-documenten.
-   Er is een menu-item voor tekstomloop en een bijbehorende sneltoets
    toegevoegd.
-   Geluiden bij bladwijzers/notities zouden nu correct moeten worden
    afgespeeld, uitsluitend wanneer je met de muis over een woord
    beweegt dat er een bevat.
-   Documenten die zijn gecodeerd in verouderde CJK-coderingen, zoals
    GBK, Big5 en Shift_JIS, worden nu correct weergegeven in plaats van
    als een hoop mojibake.
-   Het menu-item 'Exporteren' is uitgebreid, zodat je nu naast platte
    tekst ook naar HTML en Markdown kunt exporteren.
-   Het probleem is opgelost waarbij het toepassen van tekstomloop je
    terugbracht naar het begin van je document.
-   Probleem opgelost waarbij Daisy-boeken onjuiste informatie in de
    statusbalk weergeven.
-   Het probleem is verholpen dat dl-, dt- en dd-elementen geen
    regeleinden produceerden in XHTML- documenten.
-   Opgelost: Escape sloot de dialoogvensters 'Documentinfo' en 'Alle
    documenten' niet.
-   Opgelost: filepos-ankers in Mobi-boeken splitsten HTML-tags en
    voegden rommel toe aan de tekst van het boek.
-   Probleem opgelost waarbij er vertraging optrad bij het naderen van
    het einde van het tekstveld in grote documenten.
-   Probleem opgelost waarbij links in oudere Mobi-boeken niet werkten.
-   Opgelost: het laden van DAISY-boeken met onjuiste
    coderingsverklaringen.
-   Opgelost: paginanavigatie gaf in sommige situaties onjuiste
    regeltekst weer.
-   Het parseren van RTF-documenten met niet-Latijnse tekens is
    verholpen.
-   Probleem opgelost waarbij \'Laatst gesloten document opnieuw
    openen\' probeerde het meegeleverde readme-bestand opnieuw te
    openen.
-   De titelbalk werd niet bijgewerkt na het sluiten van een document
    vanuit het dialoogvenster 'Alle documenten'.
-   Het dialoogvenster \'Webview\' kon niet worden aangepast en
    verscheen in een zeer kleine startgrootte. Dit is verholpen.
-   Probleem opgelost waarbij Word-documenten met locatiespecifieke
    stijlnamen hun koppen niet correct weergeven.
-   Probleem opgelost waarbij het geselecteerde tabblad niet correct
    werd geselecteerd na het opnieuw opstarten van Paperback.
-   Als er een selectie actief is wanneer u het dialoogvenster voor het
    tellen van woorden opent, wordt nu het aantal geselecteerde woorden
    weergegeven.
-   Afbeeldingen zouden nu correct moeten worden weergegeven in de
    ingebouwde webview.
-   De verwerking van bestanden op Windows-netwerkschijven door
    Paperback is verbeterd: als je op 'Bestand in map weergeven' drukt,
    wordt nu de juiste focus op het bestand op de netwerkopslag
    geplaatst en bevatten de paden geen vreemde tekens meer.
-   Het parseren van AZW3-bestanden is aanzienlijk verbeterd.
-   We zijn overgestapt van chmlib naar onze eigen, volledig in Rust
    geschreven CHM-bestandslezer.
-   Op de desktop worden .paperback-bestanden niet langer gedwongen
    geladen bij het herstellen van documenten. In plaats daarvan wordt u
    om bevestiging gevraagd wanneer het bestand wordt gevonden.
-   Paperback valt nu terug op het extraheren van platte tekst voor
    verkeerd getagde PDF\'s.
-   Als je de map opent waarin het bestand zich bevindt, wordt het
    betreffende bestand nu in Verkenner geselecteerd.
-   Bij het openen van de readme wordt nu rekening gehouden met de door
    u geselecteerde taal.
-   PowerPoint-documenten ondersteunen nu tabellen.
-   Het menu wordt correct bijgewerkt en de focus wordt op het tekstveld
    geplaatst wanneer de Help in Paperback wordt geopend.
-   Readme.html wordt niet langer toegevoegd aan de lijst met alle
    documenten wanneer het wordt geopend via Shift+F1.
-   Het verwijderen van documenten uit het dialoogvenster \'Recente
    bestanden\' sluit nu ook het actieve tabblad.
-   Er is overgeschakeld naar een veel veiligere methode voor IPC op
    Windows.
-   De titel van het actieve document wordt nu voorgelezen bij het
    schakelen tussen tabbladen.
-   De updater geeft nu de inhoud van Markdown-codetags correct weer in
    de release-opmerkingen.
-   De updater controleert nu of het gedownloade bestand niet is
    gemanipuleerd .
-   De webview wordt nu geopend op je huidige leespositie.
-   Je zoekfilter in het dialoogvenster 'Alle documenten' blijft nu
    behouden nadat je een document hebt verwijderd.

### Versie 0.8.5 {#version-0.8.5}

-   Pagina-ondersteuning toegevoegd voor EPUB-boeken.
-   Ondersteuning toegevoegd voor versleutelde Microsoft
    Office-documenten. Momenteel worden de oudere versie van Word, de
    moderne versie van Word en de moderne versie van PowerPoint
    ondersteund; ondersteuning voor de oudere versie van PowerPoint is
    gepland voor de toekomst.
-   Ondersteuning toegevoegd voor oudere Microsoft Word-documenten
    (\*.doc)!
-   Er is ondersteuning toegevoegd voor oudere PowerPoint-presentaties
    (\*.ppt)!
-   Ondersteuning toegevoegd voor mobi- en AZW3-boeken!
-   Ondersteuning toegevoegd voor getagde PDF-bestanden!
-   De sneltoets Ctrl+Q is toegevoegd om de app af te sluiten.
-   Ondersteuning toegevoegd voor gezipte boeken van Bookshare (zowel
    DAISY als Word)!
-   Alternatieve tekst voor ingesloten afbeeldingen wordt nu correct
    weergegeven.
-   CHM-documenten ondersteunen nu correct de navigatie via interne
    links.
-   Probleem opgelost waarbij bladwijzergeluiden werden geactiveerd aan
    het begin van een alinea in plaats van op de positie van de
    bladwijzer.
-   Het probleem dat de functie 'Ga naar pagina' één pagina te ver
    sprong, is verholpen.
-   De Escape-toets werkte niet om het dialoogvenster 'Openen als' te
    sluiten; dit is verholpen.
-   Het contextmenu van de reader werd niet weergegeven bij een klik met
    de rechtermuisknop of de toets \'Toepassingen\'. Dit is verholpen.
-   Er is verholpen dat soms het verkeerde document werd geselecteerd
    bij het openen van documenten via de opdrachtregel.
-   PDF\'s die alleen uit afbeeldingen bestaan, worden weer gedetecteerd
    en u krijgt een melding over het bestaan ervan.
-   Het is nu mogelijk om door afbeeldingen en figuren te navigeren met
    respectievelijk g/Shift+g en f/Shift+f.
-   Paperback houdt nu rekening met de instelling voor de donkere modus
    van je applicatie.
-   DAISY XML-ondersteuning is verwijderd, aangezien deze niet langer
    nodig is.
-   Er is weer overgeschakeld naar de native Win32-navigatie op basis
    van de eerste letter in de inhoudsopgaveboom.
-   Het dialoogvenster voor laalfouten toont nu gedetailleerdere
    foutmeldingen. De webview opent nu veel sneller en soepeler.
-   De webview opent nu veel sneller en soepeler.

### Versie 0.8.2 {#version-0.8.2}

-   Pagina-ondersteuning toegevoegd aan RTF-documenten!
-   Een bug verholpen waarbij het openen van de webview in
    epub-bestanden met externe links deze automatisch activeerde.
-   Een bug verholpen waarbij de RTF-parser in zeldzame gevallen geen
    spatie tussen woorden plaatste .
-   Er is een fout verholpen waarbij alinea's in sommige PDF- documenten
    in meerdere korte regels werden opgesplitst.
-   PDF-documenten ondersteunen nu basisnavigatie via links en
    kopteksten !
-   RTF-tabbladen en regeleinden worden nu precies weergegeven zoals ze
    in het document staan.
-   Er is weer overgeschakeld naar de beproefde pdfium-bibliotheek voor
    het parseren van PDF's, waardoor de weergave van PDF's weer veel
    betrouwbaarder is geworden.

### Versie 0.8.1 {#version-0.8.1}

-   Ctrl+Shift+T toegevoegd om het laatst gesloten document opnieuw te
    openen.
-   In het dialoogvenster 'Alle documenten' is het nu mogelijk om
    meerdere documenten te selecteren om ze tegelijk te openen.
-   Enkele bugs met de RTF-parser zijn verholpen.
-   Er is een oplossing gevonden voor het probleem dat bestandspaden met
    niet-ASCII-tekens (zoals de Bosnische š, č, ć, ž) beschadigd raakten
    bij het openen van een bestand via een tweede exemplaar van
    Paperback .
-   Er is een probleem verholpen waarbij PDF-tekst in de verkeerde
    volgorde werd gelezen en waarbij de spatiëring rond woorden met
    hoofdletters onjuist was.
-   Het traag laden van documenten bij het openen van grote bestanden is
    verholpen.
-   De lokalisatie van de knoppen Ja/Nee in bevestigingsdialoogvensters
    is verholpen.

### Versie 0.8.0 {#version-0.8.0}

-   Er zijn vertalingen in het Japans, Vereenvoudigd Chinees en
    Vietnamees toegevoegd!
-   Er is een automatische updater toegevoegd die nu je momenteel
    geïnstalleerde versie van Paperback vervangt in plaats van alleen de
    nieuwe versie te downloaden!
-   Er is optionele geluidsfeedback toegevoegd bij het bereiken van een
    bladwijzer of een notitie; met dank aan Andre Louis voor de
    geluiden!
-   Ondersteuning voor RTF-documenten toegevoegd!
-   Ondersteuning voor DAISY XML-documenten toegevoegd.
-   Ondersteuning toegevoegd voor Flat Open Document-tekstbestanden!
-   Er is ondersteuning toegevoegd voor Flat Open Document-presentaties!
-   Ondersteuning toegevoegd voor scheidingstekens met s en shift+s.
-   Elke verplaatsing van meer dan 300 tekens wordt nu automatisch
    toegevoegd aan je navigatiegeschiedenis.
-   Het herstellen van het Paperback-venster vanuit het systeemvak is
    verholpen.
-   Probleem opgelost waarbij Markdown-documenten ruwe tekst in plaats
    van gerenderde HTML weergeven in de webweergave.
-   Probleem opgelost waarbij tabellen niet correct werden weergegeven
    in Markdown-bestanden.
-   Bij PDF\'s die uitsluitend uit afbeeldingen bestaan, krijg je nu een
    waarschuwing wanneer je er een probeert te laden.
-   Het is nu mogelijk om bij het controleren op updates te zoeken naar
    nieuwe dev-builds in plaats van stabiele releases.
-   Versie-informatie correct ingebed in het uitvoerbare bestand van
    Paperback.
-   Het optievenster is opgedeeld in tabbladen voor gebruiksgemak en
    navigatie.
-   Er is overgestapt op Hayro voor het parseren van PDF\'s, wat leidt
    tot meer betrouwbaarheid, snelheid en minder DLL\'s.
-   De hele app is herschreven in Rust. De nieuwe codebase is veiliger,
    laadt documenten sneller en is gemakkelijker te onderhouden en uit
    te breiden.
-   Het contextmenu van het tekstveld bevat nu lezersspecifieke acties
    in plaats van algemene opties zoals knippen en plakken.

### Versie 0.7.0 {#version-0.7.0}

-   Tabelondersteuning toegevoegd voor op HTML en XHTML gebaseerde
    documenten! Navigeer tussen tabellen met T en Shift+T, en druk op
    Enter om een tabel in een webview te bekijken.
-   Een eenvoudige webrenderingfunctie toegevoegd! Druk op Ctrl+Shift+V
    om het huidige gedeelte van je document in een webgebaseerde
    renderer te openen, handig voor inhoud zoals complexe opmaak of
    codevoorbeelden.
-   Er is een Russische vertaling toegevoegd, met dank aan Ruslan
    Gulmagomedov!
-   Er is een knop 'Alles wissen' toegevoegd aan het dialoogvenster
    'Alle documenten'.
-   De updatechecker geeft nu release-opmerkingen weer wanneer er een
    nieuwe versie beschikbaar is.
-   Het herstellen van het venster vanuit het systeemvak is gerepareerd.
-   De vertalingen van de knoppen \'Ja\' en \'Nee\' in
    bevestigingsdialoogvensters zijn gecorrigeerd.
-   Het laden van configuraties bij uitvoering als beheerder is
    verholpen.
-   De verwerking van opmerkingen in XML- en HTML-documenten is
    verholpen.
-   Het parseren van de inhoudsopgave in Epub 2-boeken is verholpen.
-   Probleem opgelost met het navigeren naar het volgende item met
    dezelfde letter in de inhoudsopgave.
-   Het zoekvenster werd niet correct verborgen bij het gebruik van de
    knoppen \'volgende\' en \'vorige\'.
-   Probleem opgelost waarbij ePub-inhoudsopgaven je soms naar het
    verkeerde item stuurden.
-   Problemen met de verwerking van witruimte in XML-, HTML- en pre-
    tags verholpen.
-   Een \'off-by-one\'-fout bij het navigeren via links is verholpen.
-   Probleem opgelost waarbij sommige boeken witruimte aan het einde van
    hun regels hadden.
-   Diverse parserproblemen zijn verholpen.
-   Menu-items met betrekking tot bladwijzers en de elementenlijst
    worden nu correct uitgeschakeld wanneer er geen document is geopend.
-   De verwerking van lijsten in verschillende documentformaten is
    verbeterd.
-   De vertaalworkflow voor bijdragers is verbeterd.
-   Er zijn veel interne refactoren doorgevoerd, waarbij het grootste
    deel van de bedrijfslogica van de applicatie is verplaatst van C++
    naar Rust voor betere prestaties en onderhoudbaarheid.

### Versie 0.6.1 {#version-0.6.1}

-   Ondersteuning voor met een wachtwoord beveiligde PDF-bestanden
    toegevoegd!
-   Een zeer eenvoudige functie toegevoegd om naar de vorige/volgende
    positie te gaan. Als je op Enter drukt bij een interne link en je
    cursor verplaatst, wordt die positie nu onthouden en kun je
    ernaartoe navigeren met Alt+pijl-links/pijl-rechts .
-   Een elementenlijst toegevoegd! Momenteel toont deze alleen een
    boomstructuur van alle koppen in je document of een lijst met links,
    maar er zijn plannen om deze in de toekomst uit te breiden.
-   Er is een optie toegevoegd om Paperback standaard in
    gemaximaliseerde modus te starten.
-   Links in sommige EPUB-documenten werkten niet correct; dit is
    verholpen.
-   Het parseren van EPUB-inhoudsopgaven met relatieve paden is
    verholpen.
-   Er is een probleem verholpen waarbij sommige ePub-documenten geen
    titel of auteur toonden.
-   De titels van sommige EPUB-hoofdstukken werden niet correct
    weergegeven in het inhoudsopgavevenster.
-   Opgelost: het was niet mogelijk om de spatiebalk te gebruiken om de
    knoppen OK/Annuleren in het inhoudsopgavevenster te activeren.
-   De verwerking van koppen in Word-documenten is verbeterd.
-   Je krijgt nu gesproken feedback als de lijst met recente documenten
    leeg is wanneer je het dialoogvenster probeert te openen.

### Versie 0.6.0 {#version-0.6.0}

-   Er is een nieuwe optie toegevoegd aan het optievenster om het menu
    'Ga naar' in een veel compactere vorm weer te geven; deze optie is
    standaard aangevinkt.
-   Er is een optie toegevoegd om navigatie op basis van
    structuurelementen te laten doorlopen.
-   Er is een optie toegevoegd aan het menu 'Extra' om de bovenliggende
    map van het document waarop de focus ligt te openen.
-   Er is een vrij eenvoudig, maar zeer effectief updatesysteem
    toegevoegd.
-   Er is een eenvoudige slaaptimerfunctie toegevoegd, toegankelijk via
    Ctrl+Shift+S.
-   Er is ondersteuning toegevoegd voor het parseren van FB2-e-books!
-   Er is ondersteuning toegevoegd voor het parseren van
    OpenDocument-presentaties!
-   Er is ondersteuning toegevoegd voor het parseren van
    OpenDocument-tekstbestanden!
-   Bladwijzers kunnen nu worden gebruikt om een hele regel te markeren,
    of om alleen bepaalde tekst te markeren. Als er geen selectie actief
    is bij het plaatsen van een bladwijzer, werkt het net als vóór
    versie 0.6 en wordt de hele regel gemarkeerd. Als je echter tekst
    selecteert, wordt alleen die tekst opgenomen in de bladwijzer.
-   Aan bladwijzers kunnen nu optionele tekstnotities worden toegevoegd!
    Navigeer tussen bladwijzers met notities met N en Shift+N, of open
    het bladwijzervenster met alle bladwijzers, alleen notities of
    alleen niet-notities geselecteerd met specifieke sneltoetsen.
-   Bladwijzers in het bladwijzervenster hebben niet langer het
    vervelende voorvoegsel „bladwijzer x".
-   Epub-boeken met HTML-inhoud die zich voordoet als XML, worden nu
    correct verwerkt.
-   Het laden van grote Markdown-documenten is verholpen.
-   Het probleem waarbij het indrukken van de spatiebalk in de
    boomstructuur van de inhoudsopgave de OK-knop activeerde, is
    verholpen.
-   De verwerking van witruimte aan het begin van pre-tags in zowel
    HTML- als XHTML-documenten is verholpen.
-   Er is een probleem verholpen waarbij het tekstveld soms niet opnieuw
    de focus kreeg bij het terugkeren naar het venster van Paperback.
-   Het tekstveld in het dialoogvenster 'Ga naar percentage' werkte niet
    goed, waardoor de waarde van de schuifbalk niet werd bijgewerkt.
-   De weergave van aangepaste HTML-ID\'s in Markdown-documenten is
    verholpen.
-   HTML binnen Markdown-codeblokken wordt nu correct weergegeven.
-   Als je een boek laadt met een opdrachtregelparameter terwijl er al
    een Paperback-instantie actief is, krijg je geen foutmelding meer
    als het laden van je document langer dan 5 seconden duurt.
-   Als je Paperback als beheerder uitvoert, wordt de configuratie nu
    correct geladen en opgeslagen.
-   Het is nu mogelijk om een bladwijzer rechtstreeks vanuit het
    bladwijzervenster te verwijderen.
-   Het is nu mogelijk om je bladwijzers en leespositie voor een bepaald
    document te importeren en exporteren. Het gegenereerde bestand
    krijgt de naam van het document, met de extensie .paperback. Als een
    dergelijk bestand wordt aangetroffen in dezelfde map als het
    document tijdens het laden, wordt het automatisch geladen. Anders
    kun je ze handmatig importeren via een optie in het menu 'Extra'.
-   Links binnen documenten worden nu volledig ondersteund! Gebruik k en
    shift+k om vooruit en achteruit te bladeren, en druk op enter om er
    één te openen/activeren.
-   Er zijn veel interne refactoren doorgevoerd, waardoor de app sneller
    is geworden en het binaire bestand kleiner is geworden.
-   Markdown-inhoud wordt nu voorbewerkt om te voldoen aan de
    CommonMark-standaard voordat deze wordt weergegeven.
-   Navigatie via lijsten en hun items wordt nu volledig ondersteund!
    Gebruik L en Shift+L om door de lijsten zelf te bladeren, en I en
    Shift+I om door lijstitems te bladeren.
-   De Delete-toets op het numerieke toetsenbord werkt nu om documenten
    uit de tabbalk te verwijderen, naast de normale Delete-toets.
-   Paperback kan nu optioneel worden geminimaliseerd naar je
    systeemvak! Deze optie is standaard uitgeschakeld, maar als je deze
    inschakelt, zorgt de minimaliseeroptie in het systeemmenu ervoor dat
    Paperback in je systeemvak wordt geplaatst, waarna het kan worden
    hersteld door op het weergegeven pictogram te klikken.
-   Paperback is nu volledig vertaalbaar! De lijst met talen die het
    ondersteunt is momenteel nog vrij klein, maar groeit voortdurend!
-   Paperback heeft nu een officiële website, op
    [paperback.dev](https://paperback.dev)!
-   PPTX-documenten tonen nu een eenvoudige inhoudsopgave met alle
    dia's.
-   Het volledige pad naar het geopende document wordt nu weergegeven in
    het dialoogvenster met documentinformatie.
-   Het installatieprogramma bevat nu een optie om de readme na de
    installatie in je browser te bekijken.
-   De lijst met recente documenten is aanzienlijk uitgebreid! In plaats
    van alleen de laatste 10 documenten te tonen die je hebt geopend,
    toont deze nu een aanpasbaar aantal, waarbij de rest van de
    documenten die je ooit hebt geopend toegankelijk zijn via een klein
    dialoogvenster.
-   Diverse kleine verbeteringen aan de parsers over de hele linie,
    waaronder het invoegen van een lege regel tussen dia's in
    PPTX-presentaties, het corrigeren van de regeleinde-afhandeling
    binnen alinea's in Word-documenten, en het toevoegen van
    opsommingstekens aan lijstitems.

### Versie 0.5.0 {#version-0.5.0}

-   Ondersteuning voor Microsoft Word-documenten toegevoegd!
-   Ondersteuning voor PowerPoint-presentaties toegevoegd!
-   Bepaalde menu-items werden niet uitgeschakeld als er geen documenten
    geopend waren; dit is nu verholpen.
-   De uitlijning van de schuifbalk voor \'Ga naar percentage\' is
    gecorrigeerd.
-   De inhoudsopgave in ePub-boeken met URL-gecodeerde bestandspaden
    en/of fragment-ID\'s is gecorrigeerd.
-   Opmerking: witruimte werd op vreemde manieren uit XHTML-koppen
    verwijderd.
-   De verwerking van witruimte binnen geneste \`pre\`-tags in HTML-
    documenten is verholpen.
-   HTML- en Markdown-documenten ondersteunen nu de
    inhoudsopgave-functie ! Wanneer je een HTML-/Markdown-document
    laadt, stelt Paperback zijn eigen inhoudsopgave samen op basis van
    de structuur van de koppen in je document, en deze wordt weergegeven
    in het dialoogvenster dat je opent met Ctrl+T.
-   HTML-documenten krijgen nu de titel zoals deze is ingesteld in de
    title-tag, indien deze aanwezig is. Anders blijft de bestandsnaam
    zonder de extensie worden gebruikt.
-   Er is overgeschakeld van UniversalSpeech naar het gebruik van een
    live-regio voor het voorlezen. Dit betekent dat er geen DLL's voor
    schermlezers meer bij het programma worden meegeleverd, en dat er nu
    meer schermlezers worden ondersteund, zoals Microsoft Narrator.
-   De zip-bibliotheken zijn aangepast, zodat een breder scala aan epub-
    boeken kan worden geopend.
-   Het dialoogvenster waarin wordt gevraagd of je je document als
    platte tekst wilt openen, is volledig vernieuwd en biedt nu de
    mogelijkheid om je document te openen als platte tekst, HTML of
    Markdown.
-   Het dialoogvenster 'Ga naar percentage' bevat nu een tekstveld
    waarin je handmatig een percentage kunt invoeren waarnaar je wilt
    springen.
-   De HTML-parser herkent nu dd, dt en dl als lijstelementen. De
    inhoudsopgave in EPUB-boeken wordt weer
-   De inhoudsopgave in EPUB-boeken wordt weer exact behouden.
-   Er wordt nu rekening gehouden met de Unicode-niet-afbreekbare spatie
    bij het verwijderen van lege regels.
-   Je wordt niet langer elke keer dat je een onherkend bestand laadt
    gevraagd hoe je het wilt openen, maar alleen de eerste keer.

### Versie 0.4.1 {#version-0.4.1}

-   Er is een optioneel startmenupictogram toegevoegd aan het
    installatieprogramma.
-   De inhoudsopgave zou nu in enkele gevallen overzichtelijker moeten
    zijn; als je bijvoorbeeld een onderliggend en een bovenliggend item
    hebt met dezelfde tekst op dezelfde positie, zie je nu alleen het
    bovenliggende item.
-   De inhoudsopgave in bepaalde CHM-documenten is gerepareerd.
-   De inhoudsopgave in Epub 3-boeken met absolute paden is gerepareerd.
-   CHM-documenten zouden nu de titel moeten weergeven zoals deze is
    ingesteld in het metadatabestand .

### Versie 0.4.0 {#version-0.4.0}

-   Ondersteuning voor CHM-bestanden toegevoegd!
-   Ondersteuning voor bladwijzers toegevoegd! Je kunt zoveel
    bladwijzers aanmaken in zoveel documenten als je wilt. Je kunt er
    met b en shift+b doorheen springen, er een instellen met
    control+shift+b, en een dialoogvenster openen om naar een specifieke
    bladwijzer te springen met control+b.
-   Er is naast het draagbare zip-bestand een installatieprogramma
    toegevoegd! Het installatieprogramma installeert Paperback in je map
    Program Files en stelt automatisch de bestandskoppelingen voor je
    in.
-   Tekstbestanden met BOM's zouden nu correct moeten worden
    gedecodeerd, en de BOM wordt ook niet langer aan het begin van de
    tekst weergegeven.
-   Er is veel meer informatie toegevoegd aan de statusbalk. Deze toont
    je nu je huidige regel, teken en het percentage dat is gelezen.
-   HTML-opmerkingen, evenals de inhoud van script- en style-tags,
    worden niet langer weergegeven in de tekstuitvoer.
-   Als je een relatief pad doorgeeft aan Paperback via de
    opdrachtregel, wordt dit nu correct verwerkt.
-   Het verplaatsen van percentages wordt nu afgehandeld via een eigen
    dialoogvenster met schuifregelaars, dat toegankelijk is via
    Control+Shift+G.
-   Documenten zonder bekende titels of auteurs krijgen nu altijd een
    standaardwaarde.
-   De logica voor het opslaan van de positie is nu veel slimmer en zou
    alleen naar de schijf moeten schrijven wanneer dat absoluut
    noodzakelijk is.
-   Het document waarop je de focus had toen je Paperback sloot, wordt
    nu onthouden, ook na het opnieuw opstarten van de applicatie.
-   Invoer in de dialoogvensters 'Ga naar regel' en 'Ga naar pagina'
    wordt nu strenger gefilterd.
-   De navigatie door de inhoudsopgave in EPUB 3-boeken met relatieve
    paden in hun manifesten is verholpen.

### Versie 0.3.0 {#version-0.3.0}

-   De inhoudsopgave in EPUB-boeken met URL-gecodeerde manifesten is
    gerepareerd.
-   De navigatie door koppen in HTML-documenten met multibyte
    Unicode-tekens is verholpen.
-   Het hoge CPU-gebruik in documenten met lange titels is verholpen,
    dat werd veroorzaakt door een regressie in wxWidgets.
-   Het laden van UTF-8-tekstbestanden is verholpen.
-   Er is een probleem verholpen waarbij geneste inhoudsopgave-items in
    ePub-boeken de cursor op de verkeerde positie plaatsten.
-   Een crash bij het afsluiten van de applicatie in bepaalde gevallen
    is verholpen.
-   Er is een selectievakje toegevoegd in het optievenster om
    woordafbreking in of uit te schakelen!
-   Het is nu mogelijk om een donatie te doen voor de ontwikkeling van
    Paperback, hetzij via het nieuwe donatie-item in het helpmenu,
    hetzij via de link 'Sponsor dit project' onderaan de hoofdpagina van
    de GitHub-repository.
-   Markdown-documenten hebben nu altijd een titel en Paperback zou nu
    vrijwel elk Markdown-bestand moeten kunnen laden.
-   PDF-documenten hebben nu altijd een titel, zelfs als de metagegevens
    ontbreken.
-   Er is overgeschakeld naar de PDF-bibliotheek die in Chromium wordt
    gebruikt, wat leidt tot een veel betrouwbaardere PDF-parsing over de
    hele linie.
-   Er kan nu slechts één exemplaar van Paperback tegelijk worden
    uitgevoerd. Als je paperback.exe met een bestandsnaam uitvoert
    terwijl het programma al actief is, wordt dat document geopend in
    het reeds actieve exemplaar.
-   Je kunt nu op de Delete-toets drukken bij een document in het
    tabbladvenster om het te sluiten.

### Versie 0.2.1 {#version-0.2.1}

-   Het totale aantal pagina's is toegevoegd aan het paginalabel in het
    dialoogvenster 'Ga naar pagina' .
-   Het is nu mogelijk om met de tab-toets van de documentinhoud naar je
    lijst met geopende documenten te gaan.
-   De toetscombinaties voor kopteksten zijn aangepast, zodat recente
    documenten niet meer worden geopend als er te veel zijn.
-   Paperback verwijdert nu onnodige zachte koppeltekens uit de
    tekstuitvoer.
-   Er is een fout verholpen waarbij de navigatie via kopteksten je soms
    op het verkeerde teken plaatste.

### Versie 0.2.0 {#version-0.2.0}

-   Ondersteuning voor Markdown-documenten toegevoegd!
-   Ondersteuning voor PDF-documenten toegevoegd, inclusief de
    mogelijkheid om tussen pagina\'s te navigeren!
-   Toetscombinaties toegevoegd voor navigatie via koppen in
    HTML-inhoud, inclusief ePub-boeken en Markdown-documenten. Deze
    toetscombinaties zijn ontworpen om op dezelfde manier te werken als
    een schermlezer.
-   Het laden van epub-bestanden met URL-gecodeerde bestandsnamen in hun
    manifesten is verholpen.
-   Het laden van EPUB 3-boeken met daarin ingebedde XHTML is verholpen.
-   Er wordt nu een bericht voorgelezen als het document geen
    inhoudsopgave of secties ondersteunt, in plaats van dat de
    menu-items worden uitgeschakeld.
-   Er is een menu met recente documenten toegevoegd! Hierin worden
    momenteel je laatste 10 geopende documenten opgeslagen, en als je op
    Enter drukt bij een document, wordt het geopend om te lezen.
-   Het dialoogvenster 'Zoeken' is volledig herschreven, waardoor het
    veel eenvoudiger te gebruiken is, terwijl er ook een geschiedenis
    van je laatste 25 zoekopdrachten en ondersteuning voor reguliere
    expressies is toegevoegd!
-   Eerder geopende documenten worden nu onthouden, ook na het opnieuw
    opstarten van de applicatie. Dit is instelbaar via het nieuwe
    optie-item in het menu 'Extra' .
-   Shift+F1 toegevoegd om de readme direct in Paperback zelf te openen.

### Versie 0.1.0 {#version-0.1.0}

-   Eerste uitgave.
