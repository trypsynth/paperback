# Paperback - verzija 0.8.5

## Uvod

Paperback je lagan, brz i pristupačan čitač e-knjiga i dokumenata za svakoga, od običnih čitatelja do naprednih korisnika. Osmišljen je za pristupačnost s čitačima zaslona, velike brzine i iskustvo bez nepotrebnih stvari.

## Sistemski zahtjevi

Paperback trenutno radi na Windowsu, MacOS-u, Linuxu, iOS-u i Androidu.

## Funkcije

* Potpuno je samostalan, ne zahtijeva instaliranje nikakvog dodatnog softvera na vaš računar za početak čitanja.
* Nevjerovatno je brz, čak i na starom hardveru.
* Ima jednostavno sučelje s karticama, koje vam omogućuje otvaranje koliko god dokumenata želite jedan pored drugog.
* Pamti položaj kursora na svakom dokumentu koji otvorite.
* Po želji pamti koje ste dokumente otvorili kada ste zatvorili program i vraća ih pri sljedećem pokretanju.
* Osmišljen je od strane korisnika čitača zaslona za korisnike čitača zaslona.
* Uključuje funkcije kretanja slične onima koje se nalaze u načinu pretraživanja weba mnogih čitača zaslona za brzo i jednostavno kretanje kroz dokumente.
* Uključuje opsežan dijalog za traženje, uključujući funkcije kao što su historija i podrška za regularne izraze.
* Može se koristiti kao prenosivi program ili instalirati uz automatsko postavljanje pridruživanja datoteka.

## Kompatibilnost s čitačima zaslona

Paperback radi vrlo dobro s većinom poznatih čitača zaslona. Međutim, postoji jedan poznat problem za JASW korisnike.

### JAWS i brajevi zasloni

Ako koristite JAWS s brajevim zaslonom, mogli biste primijetiti da su dugi odlomci skraćeni prilikom kretanja naprijed tasterima za navigaciju na vašem brajevom zaslonu. Komanda za čitanje trenutnog odlomka također je zahvaćena. Ovo je greška u JAWS-ovom rukovanju `RICHEDIT50W` tekstualnih kontrola, a ne u samom Paperbacku. Nažalost, trebalo je prilično dugo da Vispero ispravi ovaj problem zbog njihovog nedostatka entuzijazma za odgovaranje na prijave problema u softveru otvorenog koda.

Rješenje za ovaj problem, koji se nakon višemjesečnog čekanja na kraju pojavio u JAWS grupi za diskusiju, jeste da uredite datoteku `paperback.jcf` i opciju "Braille Presentation and Panning" postavite na "Always use DOM if available". Također ćete morati uključiti opciju "Pan Text by Paragraph", jer će u suprotnom vaš brajev zaslon ostati na trenutnom odlomku umjesto da prelazi na sljedeći. Kada su obje opcije uključene, kretanje po tekstu trebalo bi ispravno raditi.

## Trenutno podržane vrste datoteka

Paperback podržava sljedeće formate i ekstenzije:

* CHM datoteke pomoći (`.chm`)
* EPUB e-knjige (`.epub`)
* FB2 e-knjige (`.fb2`)
* HTML dokumente (`.htm`, `.html`, `.xhtml`)
* Markdown dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word dokumente (`.docx`, `.docm`)
* OpenDocument prezentacije (`.odp`, `.fodp`)
* OpenDocument tekstualne datoteke (`.odt`, `.fodt`)
* PDF dokumente (`.pdf`)
* PowerPoint prezentacije (`.pptx`, `.pptm`)
* RTF dokumente (`.rtf`)
* Čisti tekst i datoteke dnevnika (`.txt`, `.log`)
* XML dokumente (`.xml`)

## Prečice na tastaturi

Paperback je osmišljen prvenstveno za korištenje putem tastature i čitača zaslona. Evo trenutno dostupnih prečica.

### Izbornik `Datoteka`

* `Control+O`: Otvara dokument.
* `Control+F4`: Zatvara trenutni dokument.
* `Control+Shift+F4`: Zatvara sve otvorene dokumente.
* `Control+R`: Otvara dijalog "Svi dokumenti" (iz nedavnih dokumenata).

### Izbornik `Idi`

* `Control+F`: Otvara dijalog "Traži".
* `F3`: Traži sljedeće.
* `Shift+F3`: Traži prethodno.
* `Control+G`: Otvara dijalog "Idi na red".
* `Control+Shift+G`: Otvara dijalog "Idi na postotak".
* `Control+P`: Otvara dijalog "Idi na stranicu" (kad dokument to podržava).
* `Alt+Strelica lijevo`: Ide natrag u historiji kretanja.
* `Alt+Strelica desno`: Ide naprijed u historiji kretanja.
* `[`: Prethodni odjeljak.
* `]`: Sljedeći odjeljak.
* `Shift+H`: Prethodni naslov.
* `H`: Sljedeći naslov.
* Od `Shift+1` do `Shift+6`: Prethodni naslov nivoa od 1 do 6.
* Od `1` do `6`: Sljedeći naslov nivoa od 1 do 6.
* `Shift+P`: Prethodna stranica.
* `P`: Sljedeća stranica.
* `Shift+B`: Prethodna knjižna oznaka.
* `B`: Sljedeća knjižna oznaka.
* `Shift+N`: Prethodna napomena.
* `N`: Sljedeća napomena.
* `Control+B`: Prelazi na sve knjižne oznake i napomene.
* `Control+Alt+B`: Prelazi samo na knjižne oznake.
* `Control+Alt+M`: Prelazi samo na napomene.
* `Control+Shift+W`: Prikazuje tekst napomene na trenutnom položaju.
* `Shift+K`: Prethodna poveznica.
* `K`: Sljedeća poveznica.
* `Shift+T`: Prethodna tabela.
* `T`: Sljedeća tabela.
* `Shift+S`: Prethodni rastavljač.
* `S`: Sljedeći rastavljač.
* `Shift+L`: Prethodni popis.
* `L`: Sljedeći popis.
* `Shift+I`: Prethodna stavka popisa.
* `I: Sljedeća stavka popisa.
* `Shift+,`: Prelazi na početak trenutnog kontejnera (popisa ili tabele).
* `,`: Prelazi na kraj trenutnog kontejnera (popisa ili tabele).

### Izbornik `Alati`

* `Control+W`: Prikazuje broj riječi u trenutnom dokumentu.
* `Control+I`: Prikazuje informacije o dokumentu.
* `Control+T`: Prikazuje sadržaj.
* `F7`: Prikazuje popis elemenata.
* `Control+Shift+C: Otvara mapu u kojoj se nalazi trenutna datoteka.
* `Control+Shift+V`: Otvara trenutni sadržaj u web prikazu.
* `Control+U`: Prikazuje izvorni sadržaj dokumenta u novoj kartici.
* `Control+Shift+E`: Izvozi podatke dokumenta (`.paperback`).
* `Control+Shift+I`: Uvozi podatke dokumenta (`.paperback`).
* `Control+E`: Izvozi trenutni dokument u obični tekst.
* `Ctrl+Shift+B`: Dodaje i uklanja knjižnu oznaku na trenutnom odabiru ili položaju.
* `Ctrl+Shift+N`: Dodaje ili uređuje napomenu knjižne oznake na trenutnom odabiru ili položaju.
* `Ctrl+Alt+W`: Uključuje i isključuje prelamanje riječi.
* `Control+,`: Otvara postavke.
* `Control+Shift+S`: Uključuje i isključuje brojač spavanja.

### Izbornik `Pomoć`

* `Control+F1`: Prikazuje dijalog "O programu".
* `F1`: Prikazuje dokumentaciju u zadanom pretraživaču.
* `Shift+F1`: Prikazuje dokumentaciju u Paperbacku.
* `Control+Shift+U: Provjerava ima li ažuriranja.
* `Control+D`: Otvara stranicu za donacije u zadanom pretraživaču.

### Dodatne prečice

* `Delete` / `Numpad Delete` na kontroli kartica: Zatvara karticu odabranog dokumenta.
* `Enter` u dokumentu: Otvara poveznicu na položaju kursora ili otvara tabelu.
* `Shift+F10` u dokumentu: Otvara kontekstni izbornik.

## Podržani jezici

Paperback je preveden na mnoge različite jezike, a novi prijevodi se stalno dodaju. Potpuni popis nalazi se u nastavku.

Ako želite doprinijeti projektu prevođenjem, pročitajte naš [vodič za prevođenje](translating.md).

* Bosanski
* Češki
* Finski
* Francuski
* Japanski
* Kineski (pojednostavljeni)
* Nizozemski
* Njemački
* Poljski
* Portugalski (Brazil)
* Ruski
* Srpski
* Španski
* Vijetnamski

## Zasluge

### Razvoj

* Quin Gillespie: glavni programer i osnivač projekta.
* Aryan Choudhary: glavni suradnik.

### Donacije

Sljedeće osobe su finansijski podržale razvoj Paperbacka. Ako donirate, vaše ime neće automatski biti dodano na ovaj popis. Na njega uključujem samo osobe koje žele da njihova donacija bude javno vidljiva.

Napomena: Ako ste javni sponzor na GitHubu, smatrat ću to pristankom da vaše ime bude automatski uključeno na ovaj popis.

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
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Dnevnik promjena

### Verzija  0.9.0

* Dodano je dugme za otkazivanje preuzimanja ažuriranja.
* Dodan je CLI alat koji se zove PB, a koji za cilj ima jednostavno pretvaranje svih oblika datoteka koje Paperback podržava u HTML, Markdown ili običan tekst.
* Dodana je promjenjiva prečica za vraćanje Paperbacka iz sistemske trake.
* Dodano je dugme "Pronađi" u dijalog "Svi dokumenti", koje omogućava pronalaženje knjiga koje nedostaju zbog promjene njihove putanje.
* Dodana je kartica "Čitljivost" u postavke, a sadrži sljedeće opcije:
    * Prelamanje riječi (premješteno iz opće kartice);
    * Prikazuj tabele unutar teksta (novo u ovom izdanju, pogledajte ispod);
    * Font;
    * Boja pozadine;
    * Prored;
    * Razmak između odlomaka;
    * Razmak između slova;
    * Poravnanje teksta.
* Dodana je opcija za odabir načina prikaza tabela, a njihov prikaz je sada ujednačen u svim vrstama dokumenata.
* Dodana je opcija "Prikaži izvor" za otvaranje izvornog sadržaja dokumenta u novoj kartici, što je korisno, na primjer, prilikom uređivanja Markdown dokumenata.
* U dijalog za broj riječi dodana je procjena vremena čitanja, kao i mogućnost podešavanja vlastite brzine čitanja kako bi ovaj podatak bio što korisniji.
* Dodana je podrška za Android.
* Dodana je podrška za iOS.
* Dodana je podrška za Linux.
* Dodana je podrška za MacOS.
* Dodani su novi jezici: finski, nizozemski i poljski.
* Dodana je podrška za kretanje po bloku sadržaja (popisu ili tabeli).
* Dodana je podrška za popise, stavke popisa, figure i slike u CHM dokumentima.
* Dodana je stavka izbornika "Prelamanje riječi" i odgovarajuća prečica.
* Omogućena je promjena veličine dijaloga web prikaza, koji se sada otvara u znatno preglednijoj veličini.
* Opcija za izvoz je proširena i sada, pored običnog teksta, omogućava izvoz u HTML i Markdown oblik.
* Ispravljena je greška zbog koje vas je uključivanje prelamanja riječi vraćalo na početak dokumenta.
* Ispravljene su netačne informacije u statusnoj traci DAISY knjiga.
* Ispravljena je greška zbog koje elementi `dl`, `dt` i `dd` nisu stvarali prijelome redova u XHTML dokumentima.
* Ispravljena je greška da taster `Escape` nije zatvarao dijaloge "Informacije o dokumentu" i "Svi dokumenti".
* Ispravljena je greška zbog koje su `filepos` sidra u MOBI knjigama razdvajala HTML oznake i ubacivala neispravan sadržaj u tekst knjige.
* Ispravljeno je usporavanje pri približavanju kraju tekstualnog polja u velikim dokumentima.
* Ispravljene su poveznice u starijim MOBI knjigama.
* Ispravljeno je obrađivanje RTF dokumenata koji sadrže znakove koji nisu latinični.
* Ispravljena je greška zbog koje se naslovna traka nije ažurirala nakon zatvaranja dokumenta iz dijaloga "Svi dokumenti".
* Ispravljeno je prikazivanje naslova u Word dokumentima koji koriste nazive stilova specifične za određeni jezik.
* Ispravljena je greška zbog koje nakon ponovnog pokretanja Paperbacka fokus nije bio ispravno postavljen na odabranu karticu.
* Ako je prilikom otvaranja dijaloga za brojanje riječi aktivan odabir teksta, sada će biti prikazan i broj riječi u odabranom tekstu.
* Biblioteka `chmlib` zamijenjena je vlastitim CHM čitačem napisanim u programskom jeziku `Rust`.
* Na Windowsu datoteke `.paperback` više se neće automatski učitavati prilikom vraćanja prethodne sesije. Umjesto toga, ako se takva datoteka pronađe, od vas će biti zatražena potvrda.
* Paperback će sada za PDF dokumente koji su pogrešno označeni kao strukturisani koristiti izdvajanje običnog teksta.
* Opcija "Otvori mapu datoteke" sada će u Windows Exploreru označiti odabranu datoteku.
* Otvaranje dokumentacije će sada poštovati vaš odabrani jezik.
* PowerPoint dokumenti sada podržavaju tabele.
* Prilikom otvaranja dokumentacije iz Paperbacka, izbornik će se ispravno ažurirati, a fokus će biti postavljen na tekstualno polje.
* Datoteka `readme.html` se više neće dodavati na popis nedavnih dokumenata kad se ista otvara prečicom `Shift+F1`.
* Uklonjena je podrška za AZW3 oblik koja nije ispravno radila.
* Uklanjanje dokumenata iz nedavnih će sada također zatvoriti njihovu karticu.
* Na Windowsu je uveden znatno sigurniji način međuprocesne komunikacije (IPC).
* Program za ažuriranje sada ispravno prikazuje sadržaj Markdown oznaka za kod u odjeljku dnevnika promjena.
* Program za ažuriranje sada provjerava da preuzeta datoteka nije mijenjana.
* Web prikaz se sada otvara na položaju čitanja.
* Filter pretrage u dijalogu "Svi dokumenti" sada će ostati spremljen  nakon uklanjanja dokumenta.

### Verzija 0.8.5

* Dodana je podrška za stranice u EPUB knjigama.
* Dodana je podrška za šifrovane Microsoft Office dokumente. Trenutno su podržani stariji Word dokumenti, moderni Word dokumenti i moderne PowerPoint prezentacije, dok je podrška za starije PowerPoint prezentacije planirana u budućnosti.
* Dodana je podrška za starije Microsoft Word dokumente (*.doc). nije odabrano
* Dodana je podrška za starije Microsoft PowerPoint prezentacije (  *.ppt  ).
* Dodana je podrška za MOBI i AZW3 knjige.
* Dodana je podrška za označene PDF dokumente.
* Dodana je prečica `Control+Q` za izlaženje iz programa.
* Dodana je podrška za ZIP arhive knjiga preuzete sa servisa Bookshare (i DAISY i Word oblik).
* Alternativni tekst za ugrađene slike bi se sada trebao prikazivati.
* CHM dokumenti sada ispravno podržavaju kretanje po internim poveznicama.
* Ispravljena je greška zbog koje su se zvučni signali oznaka reprodukovali na početku odlomka umjesto na položaju same oznake.
* Ispravljena je greška zbog koje je funkcija "Idi na stranicu" bila pomjerena za jednu stranicu.
* Ispravljena je greška zbog koje taster `Escape` nije zatvarao dijalog za odabir načina otvaranja dokumenta.
* Ispravljena je greška zbog koje se kontekstni izbornik čitača nije prikazivao desnim klikom ili pritiskom na taster `Aplikacije`.
* Ispravljena je greška zbog koje je prilikom otvaranja dokumenata putem komandne linije ponekad bio fokusiran pogrešan dokument.
* PDF dokumenti koji sadrže samo slike ponovo se ispravno prepoznaju i upozorit će vas na to prilikom otvaranja.
* Sada je moguće kretati se između slika pomoću `G` i `Shift+G`, odnosno između figura pomoću `F` i `Shift+F`.
* Paperback će sada poštovati tamnu temu vašeg sistema.
* Uklonjena je podrška za DAISY XML datoteke, budući da više nije potrebna.
* Vraćeno je izvorno `Win32` kretanje prvim slovom u stablu sadržaja.
* Dijalog za greške pri učitavanju sada prikazuje detaljnije poruke o greškama.
* Web prikaz će se sada otvarati mnogo brže i glađe.

### Verzija 0.8.2

* Dodana je podrška za stranice u RTF dokumentima.
* Ispravljena je greška zbog koje su se vanjske poveznice u EPUB dokumentima automatski otvarale prilikom otvaranja web prikaza.
* Ispravljena je greška zbog koje obrađivač RTF dokumenata u rijetkim slučajevima nije umetao razmak između riječi.
* Ispravljena je greška zbog koje su u pojedinim PDF dokumentima odlomci bili podijeljeni na više kratkih redova.
* PDF dokumenti sada podržavaju osnovno kretanje po poveznicama i naslovima.
* RTF tabulatori i prijelomi redova sada se prikazuju tačno onako kako se nalaze u dokumentu.
* Vraćena je provjerena biblioteka `PDFium` za obrađivanje PDF dokumenata, čime je prikaz PDF dokumenata ponovo postao mnogo pouzdaniji.

### Verzija 0.8.1

* Dodana je prečica `Control+Shift+T` za otvaranje posljednjih zatvorenih dokumenata.
* Dijalog "Svi dokumenti" sada podržava višestruki izbor.
* Ispravljeno je nekoliko grešaka u obrađivaču RTF dokumenata.
* Ispravljena je greška zbog koje su putanje datoteka koje sadrže znakove izvan ASCII skupa (poput bosanskih slova č, ć, š, đ i ž) postajale oštećene prilikom otvaranja datoteke putem druge instance Paperbacka.
* Ispravljeno je čitanje teksta u PDF dokumentima pogrešnim redoslijedom, kao i nepravilni razmaci oko riječi napisanih velikim slovima.
* Ispravljeno je sporo učitavanje velikih dokumenata.
* Ispravljeno je prevođenje dugmadi "Da" i "Ne" u dijalozima za potvrdu.

### Verzija 0.8.0

* Dodani su prijevodi na japanski, kineski (pojednostavljeni) i vijetnamski jezik.
* Dodan je automatski sistem ažuriranja koji će sada automatski zamijeniti trenutno instaliranu verziju Paperbacka, umjesto da samo preuzme novu verziju.
* Dodana je izborni zvučni signal pri dolasku do oznake ili bilješke. Zahvaljujemo Andreu Louisu na ustupljenim zvukovima!
* Dodana je podrška za RTF dokumente.
* Dodana je podrška za DAISY XML dokumente.
* Dodana je podrška za Flat Open Document tekstualne datoteke.
* Dodana je podrška za Flat Open Document prezentacije.
* Dodana je podrška za rastavljače (`S` i `Shift+S`).
* Svako kretanje veće od 300 znakova bit će automatski dodano u vašu historiju kretanja.
* Ispravljena je greška s vraćanjem Paperbacka iz sistemske trake.
* Ispravljeno je prikazivanje Markdown dokumenata koji su u web prikazu prikazivali izvorni tekst umjesto HTML sadržaja.
* Tabele se sada ispravno prikazuju u Markdown dokumentima.
* Prilikom otvaranja PDF dokumenata koji sadrže isključivo slike, sada ćete biti upozoreni na to.
* Sada je moguće provjeravati dostupnost novih razvojnih verzija umjesto samo stabilnih izdanja prilikom provjere ažuriranja.
* Podaci o verziji sada su ispravno ugrađeni u izvršnu datoteku Paperbacka.
* Dijalog postavki podijeljen je na kartice radi lakšeg korištenja.
* Za obrađivanje PDF dokumenata uvedena je biblioteka `Hayro`, što donosi veću pouzdanost, bolje performanse i manji broj potrebnih DLL datoteka.
* Cijeli program je ponovo napisan u programskom jeziku `Rust`. Novi kod je sigurniji, brže učitava dokumente te ga je lakše održavati i proširivati.
* Kontekstni izbornik tekstualnog polja sada sadrži radnje namijenjene čitaču, umjesto generičnih opcija kao što su izrezivanje i lijepljenje.

### Verzija 0.7.0
* Added table support for HTML and XHTML-based documents! Navigate between tables using T and Shift+T, and press Enter to view one in a webview.
* Added a basic web rendering feature! Press Control+Shift+V to open the current section of your document in a web-based renderer, useful for content like complex formatting or code samples.
* Added a Russian translation, thanks Ruslan Gulmagomedov!
* Added a Clear All button to the All Documents dialog.
* The update checker now displays release notes when a new version is available.
* Fixed restoring the window from the system tray.
* Fixed Yes/No button translations in confirmation dialogs.
* Fixed loading configs when running as administrator.
* Fixed comment handling in XML and HTML documents.
* Fixed TOC parsing in Epub 2 books.
* Fixed navigating to the next item with the same letter in the table of contents.
* Fixed the find dialog not hiding properly when using the next/previous buttons.
* Fixed epub TOCs occasionally throwing you to the wrong item.
* Fixed various whitespace handling issues in XML, HTML, and pre tags.
* Fixed off-by-one error in link navigation.
* Fixed some books having trailing whitespace on their lines.
* Fixed various parser issues.
* Bookmark-related menu items as well as the elements list are now properly disabled when no document is open.
* Improved list handling in various document formats.
* Improved the translation workflow for contributors.
* Many internal refactors, moving the majority of the application's business logic from C++ to Rust for improved performance and maintainability.

### Verzija 0.6.1

* Dodana je podrška za PDF dokumente zaštićene lozinkom.
* Dodana je osnovna funkcija za prelaženje na prethodni ili sljedeći položaj. Ako pritisnete `Enter` na internoj poveznici i ona pomjeri kursor, taj položaj će biti zapamćen, a na njega se možete vratiti pomoću `Alt+Strelica lijevo` i `Alt+Strelica desno`.
* Dodan je popis elemenata! Trenutno prikazuje samo stablo svih naslova u dokumentu ili popis poveznica, ali se u budućnosti planira njegovo proširenje.
* Dodana je opcija za pokretanje Paperbacka u maksimizovanom prozoru.
* Ispravljena je greška s poveznicama koje u pojedinim EPUB dokumentima nisu ispravno radile.
* Ispravljeno je obrađivanje sadržaja u EPUB dokumentima koji sadrže relativne putanje.
* Ispravljena je greška kad se naslov ili autor u pojedinim EPUB dokumentima nije prikazivao.
* Ispravljeno je nepravilno prikazivanje naslova pojedinih poglavlja EPUB knjiga u dijalogu sadržaja.
* Ispravljena je greška zbog koje nije bilo moguće koristiti taster `Razmak` za aktiviranje dugmadi   "U redu" i "Otkaži"   u dijalogu sadržaja.
* Unaprijeđeno je rukovanje naslovima u Word dokumentima.
* Ako je popis nedavno otvorenih dokumenata prazan kada pokušate otvoriti odgovarajući dijalog, sada će se to izgovoriti.

### Verzija 0.6.0

* Dodana je nova opcija u postavki koja omogućava prikaz izbornika "Idi" u znatno sažetijem obliku. Ova opcija je podrazumijevano uključena.
* Dodana je opcija koja omogućava kružno kretanje prilikom navigacije po strukturnim elementima.
* U izbornik "Alati" dodana je opcija za otvaranje direktorija u kojem se nalazi trenutno aktivni dokument.
* Dodan je jednostavan, ali vrlo efikasan sistem za ažuriranje.
* Dodana je osnovna funkcija brojača vremena za spavanje, kojoj možete pristupiti pomoću `Control+Shift+S`.
* Dodana je podrška za FB2 elektronske knjige.
* Dodana je podrška za OpenDocument prezentacije.
* Dodana je podrška za OpenDocument tekstualne dokumente.
* Oznake sada mogu obuhvatati cijeli red ili samo odabrani tekst. Ako prilikom postavljanja oznake nemate aktivan odabir, ponašanje je isto kao prije verzije `0.6.0` i označit će se cijeli red. Ako je dio teksta označen, oznaka će obuhvatiti samo taj tekst.
* Oznakama se sada mogu dodati i napomene. Između oznaka koje sadrže napomene možete se kretati pomoću `N` i `Shift+N`, a dijalog s oznakama možete otvoriti tako da prikazuje sve oznake, samo one s napomenama ili samo one bez napomena, koristeći odgovarajuće prečice.
* Oznake u dijalogu s oznakama više neće imati dosadni prefiks "Oznaka x".
* EPUB knjige koje sadrže HTML sadržaj predstavljen kao XML sada će se ispravno obrađivati.
* Ispravljeno je učitavanje velikih Markdown dokumenata.
* Ispravljeno je pritiskanje tastera `Razmak` u prikazu stabla sadržaja koje je aktiviralo dugme "U redu".
* Ispravljeno je rukovanje razmacima na početku oznaka `pre` u HTML i XHTML dokumentima.
* Ispravljena je greška zbog koje tekstualno polje ponekad nije ponovo dobijalo fokus nakon povratka u prozor Paperbacka.
* Ispravljena je greška zbog koje tekstualno polje u dijalogu "Idi na postotak" nije ažuriralo vrijednost klizača.
* Ispravljeno je prikazivanje prilagođenih HTML identifikatora u Markdown dokumentima.
* HTML unutar Markdown blokova koda sada će se ispravno prikazivati.
* Ako otvorite knjigu pomoću parametra komandne linije dok je druga instanca Paperbacka već pokrenuta, više nećete dobiti grešku ako učitavanje dokumenta traje duže od pet sekundi.
* Ako Paperback pokrenete kao administrator, konfiguracija će se sada ispravno učitavati i spremati.
* Sada je moguće izbrisati oznaku direktno iz dijaloga s oznakama.
* Sada je moguće uvesti i izvesti oznake i položaj čitanja za određeni dokument. Stvorena datoteka nosi isti naziv kao dokument, ali s ekstenzijom `.paperback`. Ako se takva datoteka nalazi u istom direktoriju kao dokument prilikom njegovog otvaranja, automatski će biti učitana. U suprotnom, možete je ručno uvesti putem opcije u izborniku "Alati".
* Poveznice unutar dokumenata sada su u potpunosti podržane. Koristite `K` i `Shift+K` za kretanje između njih, a `Enter` za otvaranje ili otvaranje poveznice.
* Brojne interne izmjene učinile su program bržim, a izvršnu datoteku manjom.
* Markdown sadržaj sada se prije prikazivanja obrađuje kako bi bio usklađen sa CommonMark standardom.
* Kretanje po popisima i njihovim stavkama sada je u potpunosti podržana. Koristite `L` i `Shift+L` za kretanje po popisima, a `I` i `Shift+I` za kretanje po stavkama popisa.
* Tipka `Delete` na numeričkoj tastaturi sada također uklanja dokumente s trake kartica, kao i standardna tipka `Delete`.
* Paperback se sada po želji može minimizovati u sistemsku traku. Ova opcija je podrazumijevano isključena, ali kada je uključite, Paperback će se smjestiti u sistemsku traku, odakle ga možete vratiti klikom na njegovu ikonu.
* Paperback je sada u potpunosti prevodiv! Broj podržanih jezika trenutno nije velik, ali se stalno povećava.
* Paperback sada ima i svoju službenu web stranicu na [paperback.dev](https://paperback.dev).
* PPTX dokumenti sada prikazuju osnovni sadržaj koji sadrži sve slajdove.
* Puna putanja otvorenog dokumenta sada će biti prikazana u dijalogu s informacijama o dokumentu.
* Instalacijski program sada uključuje opciju za otvaranje dokumentacije u pretraživaču nakon završetka instaliranja.
* Popis nedavno otvorenih dokumenata značajno je proširen. Umjesto samo posljednjih deset otvorenih dokumenata, sada možete odrediti koliko će ih biti prikazano, dok su svi ostali dokumenti koje ste ikada otvorili dostupni putem posebnog dijaloga.
* Unesena su brojna manja poboljšanja u obrađivačima, uključujući dodavanje praznog reda između slajdova u PPTX prezentacijama, rukovanje novim redovima unutar odlomaka u Word dokumentima te dodavanje grafičkih oznaka stavkama popisa.

### Verzija 0.5.0

* Dodana je podrška za Word dokumente.
* Dodana je podrška za PowerPoint prezentacije.
* Ispravljeni su pojedini izbornici koji nisu bili onemogućeni kada nijedan dokument nije bio otvoren.
* Ispravljena je orijentacija klizača u dijalogu "Idi na postotak".
* Ispravljenn je sadržaj u EPUB knjigama s URL kodiranim putanjama datoteka i/ili identifikatorima fragmenata.
* Ispravljeno je nepravilno uklanjanje razmaka iz XHTML naslova u određenim slučajevima.
* Ispravljeno je rukovanje razmacima unutar ugniježđenih oznaka `pre` u HTML dokumentima.
* HTML i Markdown dokumenti sada podržavaju funkciju sadržaja! Prilikom otvaranja HTML ili Markdown dokumenta, Paperback će na osnovu strukture naslova u dokumentu automatski napraviti sadržaj i prikazati ga u dijalogu `Control+T`.
* HTML dokumenti sada će koristiti naslov definisan u oznaci `title`, ako postoji. U suprotnom će, kao i do sada, koristiti naziv datoteke bez ekstenzije.
* UniversalSpeech je zamijenjen live regionom za prijavljivanje govora. Zbog toga se DLL datoteke čitača zaslona više ne isporučuju uz program, a podržan je i veći broj čitača zaslona, uključujući Microsoft Narrator.
* Biblioteka za rad sa ZIP datotekama je zamijenjena kako bi bilo moguće otvoriti veći broj EPUB knjiga.
* Dijalog koji vas pita želite li otvoriti dokument kao običan tekst potpuno je redizajnovan i sada omogućava otvaranje dokumenta kao običan tekst, HTML ili Markdown dokumenta.
* Dijalog "Idi na postotak" sada sadrži i tekstualno polje koje omogućava ručni unos postotka na koji želite otići.
* HTML obrađivač sada prepoznaje oznake `dd`, `dt` i `dl` kao elemente liste.
* Sadržaj u EPUB knjigama ponovo će biti spremljen tačno onako kako je definisan.
* Unicode znak za nerazdvojivi razmak sada se uzima u obzir prilikom uklanjanja praznih redova.
* Prilikom otvaranja nepoznate datoteke više nećete svaki put biti pitani kako je želite otvoriti, već samo prvi put.

### Verzija 0.4.1

* Instalacijski program sada nudi mogućnost dodavanja prečice u Start izbornik.
* Sadržaj će sada u pojedinim slučajevima biti pregledniji. Na primjer, ako nadređena i podređena stavka imaju isti tekst na istoj poziciji, prikazat će se samo nadređena stavka.
* Ispravljen je sadržaj u određenim CHM dokumentima.
* Ispravljen je sadržaj u EPUB 3 knjigama koje sadrže apsolutne putanje.
* CHM dokumenti sada će prikazivati naslov definisan u datoteci s metapodacima.

### Verzija 0.4.0

* Dodana je podrška za CHM dokumente.
* Dodana je podrška za oznake! Možete imati neograničen broj oznaka u neograničenom broju dokumenata. Krećite se između njih naprijed i nazad pomoću `B` i `Shift+B`, postavite novu oznaku pomoću `Control+Shift+B`, a dijalog za prelazak na određenu oznaku otvorite pomoću `Control+B`.
* Dodan je instalacijski program uz prenosivu ZIP verziju! Instalacijski program će instalirati Paperback u direktorij `Program Files` i automatski postaviti pridruživanje datoteka.
* Tekstualne datoteke s BOM oznakom sada će se ispravno dekodirati, a BOM više neće biti prikazan na početku teksta.
* U statusnu traku dodano je mnogo više informacija. Sada prikazuje trenutni red, znak i postotak pročitanog dokumenta.
* HTML komentari, kao ni sadržaj oznaka `script` i `style`, više se neće prikazivati u tekstualnom izlazu.
* Ako Paperbacku putem komandne linije proslijedite relativnu putanju, sada će je ispravno razriješiti.
* Kretanje po postotku sada koristi zaseban dijalog sa klizačem, kojem možete pristupiti pomoću `Control+Shift+G`.
* Dokumenti bez poznatog naslova ili autora sada će uvijek imati podrazumijevane vrijednosti.
* Logika za spremanje položaja sada je mnogo pametnija i zapisivat će podatke na disk samo kada je to zaista potrebno.
* Dokument koji je bio u fokusu prilikom zatvaranja Paperbacka sada će biti zapamćen i ponovo otvoren nakon ponovnog pokretanja programa.
* Unos u dijalozima "Idi na red" i "Idi na stranicu" sada se strožije provjerava.
* Ispravljeno je kretanje kroz sadržaj u EPUB 3 knjigama koje u svojim manifestima koriste relativne putanje.

### Verzija 0.3.0

* Ispravljen je sadržaj u EPUB knjigama s URL kodiranim manifestima.
* Ispravljeno je kretanje po naslovima u HTML dokumentima koji sadrže višebajtne Unicode znakove.
* Ispravljena je povećana upotreba procesora u dokumentima s dugim naslovima uzrokovana regresijom u `wxWidgets` biblioteci.
* Ispravljeno je učitavanje `UTF-8` tekstualnih datoteka.
* Ispravljene su ugniježđene stavke sadržaja u EPUB knjigama koje su ponekad postavljale kursor na pogrešno mjesto.
* Ispravljeno je rušenje programa pri izlasku u određenim slučajevima.
* Dodan je potvrdni okvir u postavke za uključivanje ili isključivanje prelamanja riječi.
* Sada je moguće podržati razvoj Paperbacka donacijom, bilo putem nove stavke "Doniraj" u izborniku `Pomoć` ili putem poveznice "Sponsor this project" na dnu glavne stranice GitHub repozitorija.
* Markdown dokumenti sada će uvijek imati naslov, a Paperback bi sada trebao moći učitati gotovo svaku Markdown datoteku.
* PDF dokumenti sada će uvijek imati naslov, čak i ako nedostaju metapodaci.
* Biblioteka za rad s PDF dokumentima zamijenjena je onom koju koristi Chromium, što omogućava znatno pouzdaniju obradu PDF dokumenata.
* Sada je moguće pokrenuti samo jednu instancu Paperbacka istovremeno. Ako pokrenete `paperback.exe` s nazivom datoteke dok je program već pokrenut, taj dokument će biti otvoren u već pokrenutoj instanci.
* Sada možete pritisnuti taster `Delete` na kontroli kartica za zatvaranje otvorenog dokumenta.

### Verzija 0.2.1

* Dodan je ukupni broj stranica u dijalog za kretanje po stranicama.
* Omogućeno je kretanje između sadržaja dokumenta i popisa otvorenih dokumenata.
* Ispravljena je greška zbog koje su prečice za naslove ponekad otvarale nedavno otvorene dokumente ako ih je bilo dovoljno na popisu.
* Paperback sada automatski uklanja nepotrebne meke crtice iz tekstualnog izlaza.
* Ispravljena je greška zbog koje vas je kretanje po naslovima ponekad postavljalo na pogrešan znak.

### Verzija 0.2.0

* Dodana je podrška za Markdown dokumente.
* Dodana je podrška za PDF dokumente, uključujući mogućnost kretanja po stranicama.
* Dodane su prečice za kretanje po naslovima u HTML sadržaju, uključujući EPUB knjige i Markdown dokumente. Ove prečice osmišljene su tako da rade slično kao u čitačima zaslona.
* Ispravljeno je učitavanje EPUB datoteka čiji manifesti sadrže URL kodirana imena datoteka.
* Ispravljeno je učitavanje EPUB 3 knjiga koje sadrže ugrađene XHTML datoteke.
* Ako dokument ne podržava sadržaj ili odjeljke, sada se izgovara poruka o tome, umjesto da stavke izbornika budu onemogućene.
* Dodan je izbornik "Nedavni dokumenti". On trenutno sprema deset posljednjih otvorenih dokumenata.
* Prozor za pretragu je potpuno prerađen kako bi bio mnogo jednostavniji za korištenje, a uz to su dodani historija pretraživanja (uključuje 25 posljednjih pretraga) i podrška za regularne izraze.
* Prethodno otvoreni dokumenti sada ostaju zapamćeni i nakon ponovnog pokretanja programa. To se može podesiti putem nove opcije u izborniku "Alati".
* Dodana je prečica `Shift+F1` za otvaranje dokumentacije direktno u Paperbacku.

### Verzija 0.1.0

* Prva verzija
