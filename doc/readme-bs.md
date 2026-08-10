<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Meki uvez - verzija 0.8.5 {#paperback---version-0.8.5}

## Uvod {#introduction}

Paperback je lagan, brz i pristupačan čitač e-knjiga i dokumenata za
sve, od povremenih čitalaca do naprednih korisnika. Dizajniran je za
pristupačnost ekranskim čitačima, veliku brzinu i iskustvo bez
nepotrebnih dodataka.

## Sistemski zahtjevi {#system-requirements}

Paperback trenutno radi na Windows, macOS, iOS i Android sistemima.

## Karakteristike {#features}

-   Potpuno samostalan, ne zahtijeva instalaciju bilo kakvog softvera na
    vaše računalo da biste počeli čitati.
-   Nevjerovatno brz, čak i na starom hardveru.
-   Jednostavan interfejs sa karticama, koji vam omogućava da otvorite
    onoliko dokumenata koliko želite jedan pored drugog.
-   Sačuvajte vašu tačnu poziciju čitanja u svakom dokumentu koji
    otvorite.
-   Opcionalno pamti koje ste dokumente imali otvorene kada ste
    zatvorili program i vraća ih pri sljedećem pokretanju.
-   Uključuje navigacijsku funkcionalnost sličnu onoj koja se nalazi u
    načinu pregledavanja weba mnogih čitalaca ekrana, za brzu i
    jednostavnu navigaciju kroz dokumente.
-   Uključuje robusni dijalog za pretraživanje, s funkcijama kao što su
    historija i podrška za regularne izraze.
-   Može se pokretati potpuno portabilno ili instalirati uz automatski
    postavljene asocijacije datoteka.
-   Podržava ogroman broj uobičajenih formata datoteka.

## Kompatibilnost sa čitačima ekrana {#screen-reader-compatibility}

Paperback dobro radi sa svim glavnim čitačima ekrana. Međutim, postoji
jedan poznati problem za korisnike JAWS-a.

### JAWS i Braille prikazivači {#jaws-and-braille-displays}

Ako koristite JAWS sa brajevo displejom, možda ćete primijetiti da su
dugi odjeljci skraćeni kada prelazite naprijed navigacijskim tipkama na
vašem displeju. Pogođena je i komanda za čitanje trenutnog odjeljka. Ovo
je greška u načinu na koji JAWS obrađuje kontrolu teksta RICHEDIT50W, a
ne nešto u samom Paperbacku, i trebalo je dosta vremena da se za nju
pronađe rješenje, s obzirom na entuzijazam kompanije Vispero za
odgovaranje na probleme sa softverom otvorenog koda.

Zaobilazno rješenje, koje je na kraju objavljeno putem diskusijske grupe
JAWS nakon mjeseci čekanja, jeste da uredite `paperback.jcf` i postaviti
\"Braille Presentation and Panning\" na \"Uvijek koristi DOM ako je
dostupan\". Također ćete htjeti omogućiti \"Pan Text by Paragraph\",
inače će se vaš prikaz zadržati na aktivnom odlomku umjesto da prelazi
na sljedeći. Kada su obje postavke postavljene, pomicanje bi trebalo
raditi ispravno.

## Trenutno podržani formati datoteka {#currently-supported-file-types}

Paperback podržava sljedeće formate i ekstenzije:

-   CHM pomoćne datoteke (`.chm`)
-   DAISY knjige (`.opf`, `.zip`)
-   EPUB knjige (`.epub`)
-   FB2 e-knjige (`.fb2`)
-   HTML dokumenti (`.htm`, `.html`, `.xhtml`)
-   Markdown dokumenti (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Microsoft Word dokumenti (`.docx`, `.docm`, `.doc`)
-   MOBI/Kindle knjige (`.mobi`, `.azw`, `.azw3`)
-   OpenDocument prezentacije (`.odp`, `.fodp`)
-   OpenDocument tekstualne datoteke (`.odt`, `.fodt`)
-   PDF dokumenti (`.pdf`)
-   PowerPoint prezentacije (`.pptx`, `.pptm`, `.ppt`)
-   RTF dokumenti (`.rtf`)
-   Obični tekst i log datoteke (`.txt`, `.log`)

## Prečice na tipkovnici {#keyboard-shortcuts}

Paperback je dizajniran za upotrebu prvenstveno putem tastature. Ovdje
su trenutni prečaci.

Prečice ispod su za Windows. Gdje se macOS razlikuje, ekvivalent je
naveden u zagradama --- uglavnom zato što su Ctrl+G, Ctrl+W i
Alt+Lijevo/Desno već zauzeti drugim sistemskim ili aplikacijskim
konvencijama na toj platformi.

### Izbornik Datoteka {#file-menu}

-   `Ctrl+O`: Otvori dokument.
-   `Ctrl+F4` (macOS: `Cmd+W`): Zatvori trenutni dokument.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Zatvori sve otvorene
    dokumente.
-   `Ctrl+Shift+T`: Ponovo otvori posljednji zatvoreni dokument.
-   `Ctrl+R`: Prikaži dijalog \"Svi dokumenti\" (iz nedavnih
    dokumenata).
-   `Ctrl+Q`: Izlazak (samo za Windows; na macOS-u se nalazi u meniju
    aplikacije).

### Idi izbornik {#go-menu}

-   `Ctrl+F`: Prikaži dijalog Pronađi.
-   `F3` (macOS: `Cmd+G`): Pronađi sljedeći.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Pronađi prethodni.
-   `Ctrl+G` (macOS: `Cmd+L`): Idite na redak.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Idite na postojak.
-   `Ctrl+P`: Idite na stranicu (kada je podržano u trenutnom
    dokumentu).
-   `Alt+Left` (macOS: `Cmd+[`): Vratite se unazad u historiji
    navigacije.
-   `Alt+Right` (macOS: `Cmd+]`): Idite naprijed u historiji navigacije.
-   `[`: Prethodni odjeljak.
-   `]`: Sljedeći odjeljak.
-   `Shift+H`: Prethodni naslov.
-   `H`: Sljedeći naslov.
-   `Shift+1` kroz `Shift+6`: Prethodni naslov na nivou 1-6.
-   `1` kroz `6`: Sljedeći naslov na nivou 1-6.
-   `Shift+P`: Prethodna stranica.
-   `P`: Sljedeća stranica.
-   `Shift+B`: Prethodna oznaka.
-   `B`: Sljedeći zakladač.
-   `Shift+N`: Prethodna bilješka.
-   `N`: Sljedeća bilješka.
-   `Ctrl+B`: Preskoči na sve oznake i bilješke.
-   `Ctrl+Alt+B`: Preskoči na oznake samo.
-   `Ctrl+Alt+M`: Preskoči na bilješke.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tj. fizička tipka Control
    umjesto Cmd): Prikaži tekst bilješke na trenutnoj poziciji.
-   `Shift+K`: Prethodna poveznica.
-   `K`: Sljedeći link.
-   `Shift+G`: Prethodna slika.
-   `G`: Sljedeća slika.
-   `Shift+F`: Prethodna slika.
-   `F`: Sljedeći lik.
-   `Shift+T`: Prethodna tabela.
-   `T`: Sljedeća tabela.
-   `Shift+S`: Prethodni razdjelnik.
-   `S`: Sljedeći razdjelnik.
-   `Shift+L`: Prethodna lista.
-   `L`: Sljedeća lista.
-   `Shift+I`: Prethodni stavak liste.
-   `I`: Sljedeći stavak liste.
-   `Shift+,`: Idite na početak trenutnog kontejnera (liste ili tabele).
-   `,`: Pređi iza kraja trenutnog kontejnera (liste ili tabele).

### Izbornik alata {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, tj. fizičku tipku Control umjesto
    Cmd): Prikaži broj riječi za trenutni dokument.
-   `Ctrl+I`: Prikaži informacije o dokumentu.
-   `Ctrl+T`: Prikaži sadržaj.
-   `F7`: Prikaži listu elemenata.
-   `Ctrl+Shift+C`: Otvori sadržaj u Web pregledu.
-   `Ctrl+Shift+V`: Otvori trenutni sadržaj u pregledu na webu.
-   `Ctrl+U`: Prikaži izvor dokumenta u novoj kartici.
-   `Ctrl+Shift+E`: Izvoz podataka dokumenta (`.paperback`).
-   `Ctrl+Shift+I`: Uvezi podatke dokumenta (`.paperback`).
-   `Ctrl+E`: Izvezi trenutni dokument u običan tekst.
-   `Ctrl+Shift+B`: Postavi/ukloni oznaku na trenutnom odabiru/kursoru.
-   `Ctrl+Shift+N`: Dodaj ili uredi bilješku o oznaci na trenutnom
    odabiru/kursoru.
-   `Ctrl+Alt+W`: Uključiti/isključiti automatski prijelom riječi.
-   `Ctrl+,`: Otvori opcije (macOS: Preferanse, u meniju aplikacije).
-   `Ctrl+Shift+S`: Prekini/nastavi tajmer za spavanje.

### Izbornik pomoći {#help-menu}

-   `Ctrl+F1`: Prikaži dijalog O aplikaciji.
-   `F1`: Pogledajte pomoć u vašem zadatom pregledniku.
-   `Shift+F1`: Pogledajte pomoć u Paperbacku.
-   `Ctrl+Shift+U`: Provjeri ažuriranja.
-   `Ctrl+D`: Otvori stranicu za donacije u vašem zadatom pregledniku.

### Dodatne prečice za pregled dokumenta {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` na traci s karticama: Zatvori odabranu
    karticu dokumenta.
-   `Enter` ili `Space` u tekstu dokumenta: Aktivira poveznicu na mjestu
    kursora ili otvara prikaz tabele kada je na markeru tabele.
-   `Shift+F10` ili tipku Izbornik/Aplikacija u tekstu dokumenta:
    Otvorite kontekstualni izbornik.

## Podržani jezici {#supported-languages}

Paperback je preveden na mnogo različitih jezika, a stalno se dodaju
novi. Kompletna lista slijedi u nastavku.

Da biste saznali kako možete doprinijeti, molimo pročitajte naš [Vodič
za prevođenje](translating.md).

-   Bosanski
-   Češki
-   Holandski
-   Finski
-   Francuski
-   Njemački
-   Japanski
-   Poljski
-   Portugalski (Brazil)
-   Ruski
-   Pojednostavljeni kineski
-   Serbijski
-   Španski
-   Vietnamski

## Zasluge {#credits}

### Razvoj {#development}

-   Quin Gillespie: glavni programer i osnivač projekta.
-   Aryan Choudhary: glavni doprinosilac.

### Donacije {#donations}

Sljedeće osobe su donirale određeni iznos za razvoj Paperbacka. Ako
donirate, vaše ime neće automatski biti dodano ovdje; dodajem samo osobe
koje žele da njihova donacija bude javna.

Napomena: Smatram da javni GitHub sponzorstvo predstavlja osnovu za
automatsko uključivanje na ovu listu.

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

## Zapisnik o promjenama {#changelog}

### Verzija 0.9.0 (nije objavljena) {#version-0.9.0-unreleased}

-   Dodan je dugme za otkazivanje u dijalogu za ažuriranje u toku.
-   Dodan CLI alat, nazvan pb, za brzo pretvaranje bilo kojeg od
    podržanih formata Paperbacka u HTML, Markdown ili običan tekst.
-   Dodan je podesiv prečac na tipkovnici za vraćanje Paperbacka iz
    sistemskog traka.
-   Dodan je dugme za lociranje u dijalogu za sve dokumente kako bi se
    pronašle nedostajuće knjige koje su upravo promijenile putanju.
-   Dodana je kartica čitljivosti u dijalog opcija, sa sljedećim
    opcijama:
    -   Automatski prijelom riječi (premješteno iz općih postavki);
    -   Ugrađivanje tabela u tekst (novo u ovoj verziji, pogledajte
        ispod);
    -   Font;
    -   Boja pozadine;
    -   Razmak između redova;
    -   razmak između odlomaka;
    -   Razmak između slova;
    -   Poravnanje teksta.
-   Dodan je prekidač za odabir načina prikaza tabela i ujednačen je
    njihov prikaz u svim dokumentima.
-   dodana je opcija Prikaži izvor za otvaranje izvornog koda dokumenta
    u novoj kartici, što je korisno, na primjer, za uređivanje
    Markdowna.
-   Dodano je procijenjeno vrijeme čitanja u dijalog za brojanje riječi,
    kao i mogućnost podešavanja vaše brzine čitanja kako bi ova metrika
    zaista bila korisna.
-   Dodana podrška za ARM64 Windows!
-   Dodana podrška za Android!
-   Dodana podrška za iOS!
-   Dodana podrška za macOS!
-   Dodani novi jezici: nizozemski, finski i poljski.
-   Dodana podrška za navigaciju po kontejneru.
-   Dodana je podrška za liste, stavke liste, figure i slike u CHM
    dokumentima.
-   Dodana je stavka izbornika za prelom riječi i odgovarajući prečac na
    tipkovnici.
-   Zvukovi za oznake/bilješke sada se ispravno reproduciraju isključivo
    kada se krećete preko riječi koja ih sadrži.
-   Dokumenti kodirani u naslijeđenim CJK kodiranjima, kao što su GBK,
    Big5 i Shift_JIS, sada će se pravilno prikazivati umjesto kao gomila
    mojibake.
-   Proširen je meni za izvoz kako bi se omogućio izvoz u HTML i
    Markdown, pored običnog teksta.
-   Popravljeno je automatsko prelijevanje teksta koje vas premješta na
    početak dokumenta.
-   Popravljeno prikazivanje netačnih informacija u statusnoj traci za
    Daisy knjige.
-   Popravljeno je da elementi dl, dt i dd ne prave prijelome redova u
    XHTML dokumentima.
-   Popravljeno je da tipka Escape ne zatvara dijaloge \"Informacije o
    dokumentu\" i \"Svi dokumenti\".
-   Ispravljene su sidrišne oznake filepos u Mobi knjigama koje su
    dijelile HTML oznake i ubacivale neispravne podatke u tekst knjige.
-   Popravljen je zastoj pri približavanju kraju polja za tekst u
    velikim dokumentima.
-   Popravljene su veze u starijim mobi knjigama.
-   Popravljeno učitavanje DAISY knjiga sa neispravnim deklaracijama
    kodiranja.
-   Popravljena navigacija stranicama koja je u nekim situacijama
    prikazivala pogrešan tekst reda.
-   Popravljeno parsiranje RTF dokumenata s nelatiničnim znakovima.
-   Ispravljeno ponašanje opcije \"Ponovo otvori posljednje zatvoreno\"
    koja je pokušavala ponovo otvoriti priloženi readme.
-   Popravljen je problem ažuriranja trake naslova nakon zatvaranja
    dokumenta iz dijaloga \"Svi dokumenti\".
-   Popravljeno: dijalog webviewa se nije mogao mijenjati veličine i
    pojavljivao se u vrlo maloj početnoj veličini.
-   Ispravljeno prikazivanje naslova u Word dokumentima sa nazivima
    stilova specifičnim za određeni jezički okoliš.
-   Popravljeno je da odabrana kartica ne bude pravilno fokusirana nakon
    ponovnog pokretanja Paperbacka.
-   Ako je selekcija aktivna kada otvorite dijalog za brojanje riječi,
    sada će biti prikazano koliko riječi ste odabrali.
-   Slike bi sada trebale ispravno prikazivati se u ugrađenom webviewu.
-   Poboljšana je obrada datoteka na mrežnim diskovima u Paperbacku:
    pritiskanjem opcije \'Prikaži datoteku u mapi\' sada se datoteka na
    mrežnom skladištu ispravno fokusira, a putanje više ne sadrže čudne
    znakove.
-   Značajno poboljšana obrada AZW3 formata.
-   Prešlo se sa chmlib na naš vlastiti čitač CHM datoteka napisan u
    čistom Rustu.
-   Na desktopu, .paperback datoteke se više neće nasilno učitavati
    prilikom vraćanja dokumenta. Umjesto toga, bit će vam zatražena
    potvrda kada se datoteka pronađe.
-   Paperback sada prelazi na izvlačenje običnog teksta za PDF-ove koji
    su pogrešno označeni.
-   Otvaranje sadržavajućeg foldera sada fokusira zadanu datoteku u
    Exploreru.
-   Otvaranje readme datoteke sada će poštovati odabrani jezik.
-   PowerPoint dokumenti sada podržavaju tabele.
-   Pravilno ažurirajte meni i postavite fokus na tekstualni kontrolni
    element prilikom otvaranja pomoći u Paperbacku.
-   Datoteka Readme.html se više neće dodavati na vašu listu svih
    dokumenata kada se otvori putem tipke Shift+F1.
-   Kada se dokumenti uklone iz dijaloga nedavnih, sada će se zatvoriti
    i njihove aktivne kartice.
-   Prešlo se na mnogo sigurniju metodu IPC-a na Windowsima.
-   Naslov aktivnog dokumenta će se sada očitati prilikom prebacivanja
    između kartica.
-   Ažuriratelj sada ispravno prikazuje sadržaj oznaka markdown koda u
    bilješkama o izdanju.
-   Ažuriratelj sada provjerava da preuzeta datoteka nije bila
    izmijenjena.
-   Web prikaz se sada otvara na vašoj trenutnoj poziciji čitanja.
-   Vaš filter za pretraživanje u dijalogu za sve dokumente sada se
    sačuvava nakon uklanjanja dokumenta.

### Verzija 0.8.5 {#version-0.8.5}

-   Dodana je podrška za stranice u ePub knjigama.
-   Dodana je podrška za šifrirane Microsoft Office dokumente. Trenutno
    su podržani stari Word, moderni Word i moderni PowerPoint, a podrška
    za stari PowerPoint je planirana za budućnost.
-   Dodana je podrška za stare Microsoft Word dokumente (\*.doc)!
-   Dodana je podrška za stare PowerPoint prezentacije (\*.ppt)!
-   Dodana podrška za mobi i AZW3 knjige!
-   Dodana je podrška za PDF datoteke sa oznakama!
-   Dodan je prečac ctrl+q za izlazak iz aplikacije.
-   Dodana podrška za zipovane knjige sa Booksharea (i DAISY i Word)!
-   Alt tekst za ugrađene slike sada bi trebao biti ispravno prikazan.
-   CHM dokumenti sada ispravno podržavaju navigaciju putem internih
    linkova.
-   Ispravljeno je da se zvukovi za oznake pokreću na početku odlomka
    umjesto na poziciji oznake.
-   Ispravljeno je da odlazak na stranicu bude pomaknut za 1.
-   Popravljen problem da tipka Esc ne zatvara dijalog \'Otvori kao\'.
-   Popravljen problem pojavljivanja kontekstualnog menija čitača pri
    desnom kliku ili pritiskom na tipku \'Aplikacije\'.
-   Ispravljeno je da se ponekad fokusirao pogrešan dokument prilikom
    otvaranja dokumenata iz komandne linije.
-   PDF dokumenti koji sadrže samo slike ponovo se detekuju i
    obavještavaju vas o njihovom postojanju.
-   Sada je moguće kretati se kroz slike i figure pomoću g/shift+g,
    odnosno f/shift+f.
-   Meki uvez sada će poštovati postavku tamnog načina rada vaše
    aplikacije.
-   Uklonjena podrška za DAISY XML, jer više nije potrebna.
-   Ponovo vraćena nativna Win32 navigacija prvim slovom u stablu
    sadržaja.
-   Dijalog za učitavanje greške sada prikazuje detaljnije poruke o
    grešci.
-   Web prikaz će se sada otvarati mnogo brže i glađe.

### Verzija 0.8.2 {#version-0.8.2}

-   Dodana je podrška za stranice u RTF dokumentima!
-   Ispravljen je bug zbog kojeg je otvaranje web-preglednika u
    e-knjigama koje sadrže vanjske linkove automatski aktiviralo te
    linkove.
-   Ispravljen je bug zbog kojeg RTF parser u rijetkim slučajevima nije
    stavljao razmak između riječi.
-   Popravljen je problem razdvajanja odlomaka na više kratkih redova u
    nekim PDF dokumentima.
-   PDF dokumenti sada imaju osnovnu podršku za navigaciju po linkovima
    i naslovima!
-   RTF tabovi i prijelomi retka sada se prikazuju tačno onako kako se
    pojavljuju u dokumentu.
-   Ponovo se prešlo na provjerenu biblioteku pdfium za analizu PDF-ova,
    čime je prikaz PDF-ova ponovo postao mnogo pouzdaniji.

### Verzija 0.8.1 {#version-0.8.1}

-   Dodan je Ctrl+Shift+T za ponovno otvaranje posljednjeg zatvorenog
    dokumenta.
-   Dijalog \"Svi dokumenti\" sada podržava odabir više dokumenata za
    istovremeno otvaranje.
-   Ispravljeno je nekoliko grešaka u RTF parseru.
-   Popravljene su putanje datoteka koje sadrže ne-ASCII znakove (kao
    što su bosanski š, č, ć, ž) i koje su se oštećivale prilikom
    otvaranja datoteke putem druge instance Paperbacka.
-   Ispravljen je problem sa pogrešnim redoslijedom čitanja PDF teksta i
    neispravnim razmakom oko velikih slova.
-   Popravljeno sporo učitavanje dokumenata pri otvaranju velikih
    datoteka.
-   Popravljena je lokalizacija dugmadi Da/Ne u potvrdnim dijaloškim
    okvirima.

### Verzija 0.8.0 {#version-0.8.0}

-   Dodani su japanski, pojednostavljeni kineski i vijetnamski
    prijevodi!
-   Dodan je automatski ažurirač koji će sada zamijeniti vašu trenutno
    instaliranu verziju Paperbacka umjesto da samo preuzima novu
    verziju!
-   Dodana je opcionalna zvučna povratna informacija za dolazak do
    oznake ili bilješke, zahvaljujemo Andreu Louisu na zvukovima!
-   Dodana je podrška za RTF dokumente!
-   Dodana podrška za DAISY XML dokumente.
-   Dodana podrška za Flat Open Document tekstualne datoteke!
-   Dodana je podrška za Flat Open Document prezentacije!
-   Dodana podrška za razdjelnike pomoću tipki s i Shift+s.
-   Svaki pomak veći od 300 znakova sada će se automatski dodati u vašu
    historiju navigacije.
-   Popravljeno vraćanje prozora Paperbacka iz sistemskog traka.
-   Popravljeno prikazivanje Markdown dokumenata koji su u Web pregledu
    prikazivali sirovi tekst umjesto renderiranog HTML-a.
-   Popravljeno prikazivanje tabela u Markdown datotekama.
-   PDF dokumenti koji sadrže samo slike sada će vas upozoriti na svoje
    postojanje kada pokušate da učitati jedan.
-   Sada je moguće provjeravati nove razvojne verzije umjesto stabilnih
    izdanja prilikom provjere ažuriranja.
-   Pravilno ugrađivanje informacija o verziji u izvršnu datoteku
    Paperbacka.
-   Podijeljen je dijalog opcija na kartice radi lakšeg korištenja i
    navigacije.
-   Prešano na Hayro za parsiranje PDF-ova, što dovodi do veće
    pouzdanosti, bržeg učitavanja i manjeg broja DLL-ova.
-   Cijela aplikacija je prepisana u Rustu. Nova baza koda je sigurnija,
    brže učitava dokumente i lakša je za održavanje i proširenje.
-   Kontekstualni meni kontrole teksta sada će uključivati akcije
    specifične za čitač umjesto općih stavki kao što su kopiraj i
    zalijepi.

### Verzija 0.7.0 {#version-0.7.0}

-   Dodana je podrška za tabele u dokumentima zasnovanim na HTML-u i
    XHTML-u! Kretanje između tabela pomoću tipki T i Shift+T, a
    pritisnite Enter da biste vidjeli jednu u web-prikazu.
-   Dodana je osnovna funkcija web prikaza! Pritisnite Ctrl+Shift+V da
    otvorite trenutni odjeljak vašeg dokumenta u web rendereru, što je
    korisno za sadržaj kao što su složeno formatiranje ili primjeri
    koda.
-   Dodan je ruski prijevod, hvala Ruslanu Gulmagomedovu!
-   Dodan je dugme \"Očisti sve\" u dijalog \"Svi dokumenti\".
-   Provjera ažuriranja sada prikazuje bilješke o izdanju kada je
    dostupna nova verzija.
-   Popravljeno je vraćanje prozora iz sistemskog traka.
-   Ispravljeni su prijevodi dugmadi Da/Ne u dijaloškim okvirima za
    potvrdu.
-   Popravljeno učitavanje konfiguracija pri pokretanju kao
    administrator.
-   Popravljeno rukovanje komentarima u XML i HTML dokumentima.
-   Popravljeno parsiranje sadržaja (TOC) u Epub 2 knjigama.
-   Popravljena navigacija na sljedeći stavak sa istim slovom u
    sadržaju.
-   Popravljeno je da se dijalog za pretraživanje ne skriva ispravno
    prilikom korištenja dugmadi sljedeći/prethodni.
-   Ispravljeno je da epub sadržaji povremeno ne prebacuju na pogrešan
    stavak.
-   Ispravljeni su razni problemi s obradom praznih znakova u XML, HTML
    i pre oznakama.
-   Ispravljena greška \'off-by-one\' u navigaciji linkovima.
-   Ispravljeno je da neki naslovi imaju višak praznog prostora na
    svojim redovima.
-   Popravljeni su razni problemi parsera.
-   Stavke menija vezane za oznake, kao i lista elemenata, sada su
    ispravno onemogućene kada nije otvoren nijedan dokument.
-   Poboljšana obrada lista u različitim formatima dokumenata.
-   Poboljšan je tok rada prevođenja za saradnike.
-   Mnoge interne refaktorizacije, premještanje većine poslovne logike
    aplikacije iz C++-a u Rust radi poboljšanih performansi i
    održavanja.

### Verzija 0.6.1 {#version-0.6.1}

-   Dodana je podrška za PDF zaštićen lozinkom!
-   Dodana je vrlo osnovna funkcija za prelazak na prethodnu/sljedeću
    poziciju. Ako pritisnete Enter na internom linku i on pomjeri vaš
    kursor, ta pozicija će sada biti zapamćena i može se do nje
    navigirati pomoću Alt+strelice lijevo/desno.
-   Dodana je lista elemenata! Trenutno prikazuje samo drvo svih naslova
    u vašem dokumentu ili listu linkova, ali postoje planovi za
    proširenje u budućnosti.
-   Dodana opcija za pokretanje Paperbacka u maksimiziranom načinu rada
    po zadanom.
-   Popravljene su poveznice u nekim Epub dokumentima koje nisu ispravno
    funkcionisale.
-   Popravljeno parsiranje sadržaja (TOC) Epub datoteka koje sadrže
    relativne putanje.
-   Popravljeno je prikazivanje naslova ili autora kod nekih ePub
    dokumenata.
-   Popravljeno prikazivanje naslova nekih poglavlja u ePub formatu u
    dijalogu sadržaja (TOC).
-   Popravljeno je da ne možete koristiti tipku za razmak za aktiviranje
    dugmadi U redu/Otkaži u dijalogu za sadržaj.
-   Poboljšana obrada naslova u Word dokumentima.
-   Sada ćete dobiti govorni povratni informaciju ako je lista nedavnih
    dokumenata prazna kada pokušate otvoriti dijalog.

### Verzija 0.6.0 {#version-0.6.0}

-   Dodana je nova opcija za prikaz menija za prelazak u znatno
    kompaktnijem obliku, koja je podrazumijevano označena.
-   Dodana je opcija za omogućavanje omotavanja navigacije kroz
    strukturne elemente.
-   Dodana je opcija u izbornik alata za otvaranje sadržavajućeg mapa
    trenutno fokusiranog dokumenta.
-   Dodan je prilično jednostavan, ali vrlo efikasan sistem ažuriranja.
-   Dodana je osnovna funkcija tajmera za spavanje, dostupna putem
    Ctrl+Shift+S.
-   Dodana je podrška za parsiranje FB2 e-knjiga!
-   Dodana je podrška za parsiranje OpenDocument prezentacija!
-   Dodana je podrška za parsiranje OpenDocument tekstualnih datoteka!
-   Sada je moguće označiti čitav redak ili samo određeni tekst. Ako
    nemate aktivan odabir prilikom postavljanja oznake, ponašanje je
    isto kao u verziji prije 0.6 i označit će čitav redak. Međutim, ako
    odaberete tekst, samo će taj tekst biti uključen u oznaku.
-   Zabilješkama se sada mogu pridodati i opcionalne tekstualne
    bilješke! Prelazite između zabilješki koje sadrže bilješke pomoću N
    i Shift+N, ili otvorite dijalog sa svim zabilješkama, samo
    bilješkama ili samo odabranim zabilješkama koje nemaju bilješke
    pomoću specifičnih prečica na tipkovnici.
-   Zabilješke u dijalogu za zabilješke više neće imati dosadni prefiks
    \"zabilješka x\".
-   Epub knjige koje sadrže HTML sadržaj koji se predstavlja kao XML
    sada će se pravilno obrađivati.
-   Popravljeno učitavanje velikih Markdown dokumenata.
-   Popravljeno je aktiviranje dugmeta OK pritiskanjem razmaka u
    hijerarhijskom prikazu sadržaja.
-   Popravljeno je rukovanje praznim znakovima na početku pre oznaka u
    HTML i XHTML dokumentima.
-   Popravljen je problem da tekstualni kontrolnik ponekad ne vraća
    fokus prilikom povratka na prozor Paperbacka.
-   Popravljen je problem s poljem za tekst u dijalogu za prelazak na
    postotak koje se nije ažuriralo.
-   Popravljeno prikazivanje prilagođenih HTML ID-ova u Markdown
    dokumentima.
-   HTML unutar blokova koda Markdowna sada će se ispravno prikazati.
-   Ako učitavate knjigu pomoću parametra komandne linije dok je
    postojeći primjerak Paperbacka pokrenut, više nećete dobiti grešku
    ako učitavanje vašeg dokumenta traje duže od 5 sekundi.
-   Ako pokrenete Paperback kao administrator, konfiguracija će se sada
    ispravno učitati i spremiti.
-   Sada je moguće izbrisati oznaku direktno iz dijaloga za oznake.
-   Sada je moguće uvoziti i izvoziti vaše oznake i poziciju čitanja za
    određeni dokument. Generisana datoteka se naziva po datoteci sa
    ekstenzijom .paperback. Ako se takva datoteka pronađe u istom
    direktoriju kao i datoteka prilikom učitavanja, ona će se automatski
    učitati. U suprotnom, možete ih ručno uvesti koristeći stavku u
    meniju alata.
-   Linkovi unutar dokumenata sada su u potpunosti podržani! Koristite k
    i shift+k za kretanje naprijed i nazad kroz njih, te pritisnite
    enter da otvorite/aktivirate jedan.
-   Mnoge interne izmjene koda, zbog čega je aplikacija brža, a binarna
    datoteka manja.
-   Sadržaj u Markdown formatu sada se predobradi kako bi bio usklađen
    sa CommonMark standardom prije iscrtavanja.
-   Navigacija kroz liste i njihove stavke sada je u potpunosti
    podržana! Koristite L i Shift+L za kretanje kroz same liste, i I i
    Shift+I za kretanje kroz stavke liste.
-   Brisanje tipkom za brojanje (Numpad) sada uklanja dokumente sa trake
    sa karticama, pored uobičajenog brisanja.
-   Paperback sada može opcionalno biti minimiziran u sistemsku traku!
    Ova opcija je podrazumijevano isključena, ali uključivanje iste će
    učiniti da opcija za minimiziranje u sistemskom meniju smjesti
    Paperback u vašu traku, odakle se može vratiti klikom na stvorenu
    ikonu.
-   Paperback je sada u potpunosti prevodiv! Lista jezika koje podržava
    je trenutno prilično mala, ali stalno raste!
-   Paperback sada ima zvaničnu web stranicu na adresi
    [paperback.dev](https://paperback.dev)!
-   PPTX dokumenti će sada prikazivati osnovni sadržaj, koji sadrži sve
    slajdove.
-   Kompletna putanja do otvorenog dokumenta sada će se prikazivati u
    dijalogu s informacijama o dokumentu.
-   Instalacijski program sada uključuje opciju za pregled datoteke
    readme u vašem pregledniku nakon instalacije.
-   Spisak nedavnih dokumenata je dramatično proširen! Umjesto da vam
    jednostavno prikazuje posljednjih 10 otvorenih dokumenata, sada će
    vam prikazivati prilagodljiv broj, dok su ostali dokumenti koje ste
    ikada otvorili dostupni putem malog dijaloga.
-   Razna mala poboljšanja parsera u cjelini, uključujući dodavanje
    praznog retka između slajdova u PPTX prezentacijama, ispravljanje
    obrade novog retka unutar paragrafa u Word dokumentima i dodavanje
    znakova za nabrajanje stavkama liste.

### Verzija 0.5.0 {#version-0.5.0}

-   Dodana je podrška za Microsoft Word dokumente!
-   Dodana je podrška za PowerPoint prezentacije!
-   Popravljena je greška zbog koje se određene stavke izbornika nisu
    onemogućavale kada nisu bila otvorena nikakva dokumenta.
-   Popravljena je orijentacija klizača za prelazak na procenat.
-   Popravljena je sadržajna tabela u Epub knjigama sa URL-kodiranim
    putanjama do datoteka i/ili fragment ID-ovima.
-   Ispravljeno je nepravilno uklanjanje praznih znakova iz XHTML
    naslova.
-   Popravljeno je rukovanje praznim znakovima unutar ugniježđenih pre
    oznaka u HTML dokumentima.
-   HTML i Markdown dokumenti sada podržavaju funkciju sadržaja! Kada
    učitavate HTML/Markdown dokument, Paperback će izgraditi vlastiti
    sadržaj na osnovu strukture naslova u vašem dokumentu i prikazat će
    vam ga u dijalogu ctrl+t.
-   HTML dokumenti će sada imati naslov postavljen u title tagu, ako on
    postoji. U suprotnom, nastavit će koristiti naziv datoteke bez
    ekstenzije.
-   Prešlo se sa UniversalSpeech na korištenje aktivne regije za
    izvještavanje o govoru. To znači da se uz program više ne isporučuju
    DLL-ovi za čitače ekrana i da će sada biti podržani i čitači ekrana
    kao što je Microsoft Narrator.
-   Zamijenjene su zip biblioteke kako bi se omogućilo otvaranje šireg
    spektra ePub knjiga.
-   Dijalog koji vas pita da li želite otvoriti svoj dokument kao običan
    tekst je potpuno redizajniran i sada vam omogućava da otvorite svoj
    dokument kao običan tekst, HTML ili Markdown.
-   Dijalog za prelazak na procenat sada uključuje tekstualno polje koje
    vam omogućava da ručno unesete procenat na koji želite da
    preskočite.
-   HTML parser će sada prepoznavati dd, dt i dl kao elemente liste.
-   Sadržaj u Epub knjigama će ponovo biti tačno sačuvan.
-   Unikodni razmak koji ne lomi je sada uzet u obzir prilikom
    uklanjanja praznih redova.
-   Više vas neće svaki put pitati kako želite otvoriti neprepoznatu
    datoteku, već samo prvi put.

### Verzija 0.4.1 {#version-0.4.1}

-   Dodana je opciona ikona za start meni u instalacijski program.
-   Sadržaj bi sada trebao biti pregledniji u nekim slučajevima, na
    primjer, ako imate podređenu i nadređenu stavku sa istim tekstom na
    istoj poziciji, sada ćete vidjeti samo nadređenu stavku.
-   Popravljena je sadržajna tabela u određenim CHM dokumentima.
-   Popravljena je sadržajna tabela u Epub 3 knjigama koje sadrže
    apsolutne putanje.
-   CHM dokumenti sada bi trebali prikazivati svoj naslov onako kako je
    postavljeno u datoteci s metapodacima.

### Verzija 0.4.0 {#version-0.4.0}

-   Dodana je podrška za CHM datoteke!
-   Dodana je podrška za oznake! Možete imati onoliko oznaka u onoliko
    dokumenata koliko želite. Možete prelaziti naprijed i nazad kroz
    njih pomoću tipki b i shift+b, postaviti novu pomoću
    control+shift+b, te otvoriti dijalog za prelazak na određenu oznaku
    pomoću control+b.
-   Dodan je instalacijski program pored portabilne zip datoteke!
    Instalacijski program će instalirati Paperback u vaš direktorij
    Program Files i automatski postaviti asocijacije datoteka za vas.
-   Tekstualne datoteke sa BOM-ovima sada bi trebale biti ispravno
    dešifrirane, a BOM se više neće prikazivati ni na početku teksta.
-   Dodano je mnogo više informacija u statusnu traku. Sada će vam
    prikazivati trenutni red, znak i procenat čitanja.
-   HTML komentari, kao i sadržaj skript i stil oznaka, više se neće
    prikazivati u tekstualnom izlazu.
-   Ako se na komandnoj liniji proslijedi relativna putanja do
    Paperbacka, ona će je sada ispravno razriješiti.
-   Postotak pomaka sada se obrađuje u zasebnom dijalogu sa klizačem,
    koji je dostupan pritiskom na Control+Shift+g.
-   Dokumenti bez poznatih naslova ili autora sada će uvijek imati
    zadani naslov.
-   Logika spremanja pozicije je sada mnogo pametnija i trebala bi
    zapisivati na disk samo kada je to apsolutno neophodno.
-   Dokument na kojem ste radili kada ste zatvorili Paperback sada se
    pamti nakon ponovnog pokretanja aplikacije.
-   Unos u dijaloge za \"id na red\" i \"id na stranicu\" sada bi trebao
    biti strože pročišćen.
-   Popravljena je navigacija kroz sadržaj u epub 3 knjigama sa
    relativnim putanjama u njihovim manifestima.

### Verzija 0.3.0 {#version-0.3.0}

-   Popravljena je sadržajna tabela u epub knjigama sa URL-kodiranim
    manifestima.
-   Popravljena navigacija naslovima u HTML dokumentima koji sadrže
    višebajtne Unicode znakove.
-   Popravljena je visoka potrošnja CPU-a u dokumentima s dugim
    naslovima zbog regresije u wxWidgetsu.
-   Popravljeno učitavanje UTF-8 tekstualnih datoteka.
-   Ispravljena je greška zbog koje su ugniježđene stavke sadržaja (TOC)
    u Epub knjigama pomjerale kursor na pogrešnu poziciju.
-   Popravljen je pad aplikacije pri izlasku u određenim slučajevima.
-   Dodan je potvrdni okvir u dijaloški okvir s opcijama za omogućavanje
    ili onemogućavanje automatskog preloma riječi!
-   Sada je moguće donirati za razvoj Paperbacka, bilo putem nove opcije
    za doniranje u meniju pomoći ili putem linka \"Sponzorirajte ovaj
    projekat\" na dnu glavne stranice GitHub repozitorija.
-   Markdown dokumenti će sada uvijek imati naslov, a Paperback bi sada
    trebao moći učitati gotovo svaku Markdown datoteku.
-   PDF dokumenti će sada uvijek imati naslov, čak i ako nedostaju
    metapodaci.
-   Zamijenili smo PDF biblioteke onom koja se koristi u Chromiumu, što
    je dovelo do znatno pouzdanijeg parsiranja PDF-a u svim situacijama.
-   Sada možete pokrenuti samo jednu instancu Paperbacka u isto vrijeme.
    Pokretanje paperback.exe sa nazivom datoteke dok je program već
    pokrenut otvorit će taj dokument u već pokrenutoj instanci.
-   Sada možete pritisnuti tipku Delete na dokumentu u upravljačkoj
    ploči kartica da biste ga zatvorili.

### Verzija 0.2.1 {#version-0.2.1}

-   Dodan je ukupan broj stranica u oznaku stranice u dijalogu za
    odlazak na stranicu.
-   Omogućeno je prebacivanje na karticu sa sadržajem dokumenta iz liste
    otvorenih dokumenata.
-   Ispravljen je problem zbog kojeg su pritisci na tipke za naslove
    ponekad otvarali nedavne dokumente ako ste ih imali dovoljno.
-   Paperback će sada uklanjati nepotrebne mekane crtice iz tekstualnog
    izlaza.
-   Popravljena navigacija zaglavljima koja vas je ponekad vodila na
    pogrešan znak.

### Verzija 0.2.0 {#version-0.2.0}

-   Dodana je podrška za markdown dokumente!
-   Dodana je podrška za PDF dokumente, uključujući mogućnost navigacije
    između stranica!
-   Dodan je tipkovnički prečac za navigaciju po naslovima u HTML
    sadržaju, uključujući ePub knjige i markdown dokumente. Ovi prečaci
    su dizajnirani da rade slično kao čitač ekrana.
-   Popravljeno učitavanje ePub knjiga sa URL-kodiranim nazivima
    datoteka u njihovim manifestima.
-   Popravljeno učitavanje ePub3 knjiga sa ugrađenim XHTML-om.
-   Poruka se sada čita ako dokument ne podržava sadržaj ili odjeljke,
    umjesto da se stavke menija onemogućavaju.
-   Dodan je meni nedavnih dokumenata! Trenutno pohranjuje 10
    posljednjih otvorenih dokumenata, a pritiskom na Enter na jednom od
    njih otvara se za čitanje.
-   Potpuno je prepravljen dijalog za pretraživanje, čime je postao
    mnogo jednostavniji za korištenje, a također je dodana historija
    vaših posljednjih 25 pretraga i podrška za regularne izraze!
-   Prethodno otvoreni dokumenti se sada pamte nakon ponovnog pokretanja
    aplikacije. Ovo se može podesiti putem nove stavke opcija u meniju
    alata.
-   Dodan je shift+f1 za otvaranje readme datoteke direktno u samom
    Paperbacku.

### Verzija 0.1.0 {#version-0.1.0}

-   Početno izdanje.
