# Paperback - verzija 0.9.1

## Uvod

Paperback je lagan, brz i pristupačan čitač digitalnih knjiga i dokumenata za svakoga, od prosečnih čitalaca do naprednih korisnika. Dizajniran je kako bi bio pristupačan sa čitačima ekrana, brz i pružio iskustvo bez bespotrebnog zatrpavanja.

## Sistemski zahtevi

Paperback trenutno radi na Windowsu 10 i 11 kao i svim modernim verzijama ARM macOS-a. Izvorne iOS i Android aplikacije su u aktivnom razvoju, javne verzije za testiranje su u planu ubrzo nakon 0.9.0 desktop verzije, pre jedinstvene 1.0 verzije koja pokriva sve četiri platforme.

## Funkcije

* U potpunosti je samostalan, i ne zahteva da nijedan program bude instaliran na vašem računaru kako biste započeli čitanje.
* Neverovatno brz, čak i na starijem hardveru.
* Jednostavan interfejs sa karticama, koji vam dozvoljava da otvorite koliko god dokumenata želite na jednom mestu.
* Čuva vašu preciznu poziciju čitanja u svakom dokumentu koji otvorite.
* Opciono pamti koje ste dokumente imali otvorene kada ste zatvorili program, i vraća ih kada se sledeći put pokrene.
* Uključuje funkcije navigacije slične funkcijama koje su dostupne u režimu kretanja po Web stranicama mnogih čitača ekrana, koje vam dozvoljavaju da se lako i brzo krećete kroz dokument.
* Uključuje obiman dijalog pretrage, koji sadrži funkcije kao što su istorija i podrška za regularne izraze.
* Može se u potpunosti pokrenuti kao prenosni program, ili se instalirati uz automatsko pridruživanje podržanih vrsta datoteka.
* Podržava ogromnu listu čestih formata datoteka.

## Kompatibilnost sa čitačima ekrana

Paperback dobro radi sa svim značajnim čitačima ekrana. Postoji, međutim, jedan poznat problem za JAWS korisnike.

### JAWS i brajevi redovi

Ako koristite JAWS sa brajevim redom, možda ćete primetiti da su dugi pasusi odsečeni kada se krećete napred sa navigacionim tasterima na vašem brajevom redu. Ovo takođe utiče na komandu za čitanje trenutnog pasusa. Ovo je greška u Jaws-ovom načinu obrade RICHEDIT50W tekstualne kontrole, a ne nešto u samom Paperbacku, a na ispravku ove greške se dugo čekalo ako se u obzir uzme entuzijazam kompanije Vispero za odgovaranje na greške u programima otvorenog koda.

Način da se ova greška zaobiđ je konačno pronađen u JAWS grupi za Diskusiju nakon nekoliko meseci čekanja, a to je uređivanje `paperback.jcf` datoteke i podešavanje "Braille Presentation and Panning" na "Always use DOM if available". Takođe treba omogućiti "Pan Text by Paragraph", u suprotnom će vaš brajev red ostati na aktivnom pasusu umesto da se kreće napred. Uz oba podešavanja, navigacija bi trebala ispravno da radi.

## Trenutno podržane vrste datoteka

Paperback podržava sledeće formate i ekstenzije:

* CHM datoteke pomoći (`.chm`)
* DAISY knjige (`.opf`, `.zip`)
* EPUB knjige (`.epub`)
* FB2 digitalne knjige (`.fb2`)
* HTML dokumente (`.htm`, `.html`, `.xhtml`)
* Markdown dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word dokumente (`.docx`, `.docm`, `.doc`)
* MOBI/Kindle knjige (`.mobi`, `.azw`, `.azw3`)
* OpenDocument prezentacije (`.odp`, `.fodp`)
* OpenDocument tekstualne datoteke (`.odt`, `.fodt`)
* PDF dokumente (`.pdf`)
* PowerPoint prezentacije (`.pptx`, `.pptm`, `.ppt`)
* RTF dokumente (`.rtf`)
* Datoteke običnog teksta ili dnevnike (`.txt`, `.log`)

## Prečice na tastaturi

Paperback je dizajniran za korišćenje prvenstveno putem tastature. Ovo su trenutne prečice.

Prečice ispod su za Windows. Kada se macOS razlikuje, ekvivalentne prečice su u zagradama — obično zato što su CTRL+G, Ctrl+W i Alt+levo/desno zauzete za druge konvencije sistema ili ostalih aplikacija na ovoj platformi.

### Meni datoteke

* `Ctrl+O`: otvori dokumnt.
* `Ctrl+F4` (macOS: `Cmd+W`): zatvori trenutni dokument.
* `Ctrl+Šift+F4` (macOS: `Cmd+Šift+W`): zatvori sve otvorene dokumente.
* `Ctrl+Šift+T`: ponovo otvori poslednji zatvoren dokument.
* `Ctrl+R`: prikaži dijalog sa svim dokumentima (iz nedavnih dokumenata).
* `Ctrl+Q`: Izlaz iz aplikacije (Samo na Windowsu; na macOS-u ovo je u meniju aplikacije).

### Meni kretanja

* `Ctrl+F`: prikaži dijalog pretrage.
* `F3`(macOS: `Cmd+G`): pronađi sledeće.
* `Šift+F3` (macOS: `Cmd+Šift+G`): pronađi prethodno.
* `Ctrl+G` (macOS: `Cmd+L`): pređi u red.
* `Ctrl+Šift+G` (macOS: `Cmd+Šift+L`): pređi na procenat.
* `Ctrl+P`: pređi na stranicu (kada trenutni dokument ovo podržava).
* `=`: izgovori trenutni procenat čitanja.
* `Alt+Levo` (macOS: `Cmd+[`): vrati se nazad u istoriji navigacije.
* `Alt+Desno` (macOS: `Cmd+]`): kreći se napred u istoriji navigacije.
* `[`: prethodni odeljak.
* `]`: naredni odeljak.
* `Šift+H`: prethodni naslov.
* `H`: naredni naslov.
* `Šift+1` do `Šift+6`: prethodni naslov na nivou 1-6.
* `1` do `6`: naredni naslov na nivou 1-6.
* `Šift+P`: prethodna stranica.
* `P`: naredna stranica.
* `Šift+B`: prethodna knjižna oznaka.
* `B`: naredna knjižna oznaka.
* `/`: postavi  privremenu knjižnu oznaku.
* `\`: skoči na privremenu knjižnu oznaku.
* `Šift+N`: prethodna napomena.
* `N`: naredna napomena.
* `Ctrl+B`: skoči na sve knjižne oznake i napomene.
* `Ctrl+Alt+B`: skoči samo na knjižne oznake.
* `Ctrl+Alt+M`: skoči samo na napomene.
* `Ctrl+Šift+W` (macOS: `FizičkiCtrl+Šift+W`, kontrol taster, a ne CMD): prikaži tekst napomene na trenutnoj poziciji.
* `Šift+K`: prethodni link.
* `K`: naredni link.
* `Šift+G`: prethodna slika.
* `G`: naredna slika.
* `Šift+F`: prethodna figura.
* `F`: naredna figura.
* `Šift+T`: prethodna tabela.
* `T`: naredna tabela.
* `Šift+S`: prethodni separator.
* `S`: naredni separator.
* `Šift+L`: prethodna lista.
* `L`: naredna lista.
* `Šift+I`: prethodna stavka liste.
* `I`: naredna stavka liste.
* `Shift+,`: idi na početak trenutnog sadrživača (liste ili tabele).
* `,`: prebaci se nakon kraja trenutnog sadrživača (liste ili tabele).

### Meni sa alatima

* `Ctrl+W` (macOS: `FizičkiCtrl+W`, CTRL taster, a ne CMD): prikaži broj reči za trenutni dokument.
* `Ctrl+I`: prikaži informacije o dokumentu.
* `Ctrl+T`: prikaži sadržaj.
* `F7`: prikaži listu elemenata.
* `Ctrl+Šift+C`: otvori izvorni folder.
* `Ctrl+Šift+V`: Otvori trenutni sadržaj u Web prikazu.
* `Ctrl+U`: prikaži izvor dokumenta u novoj kartici.
* `Ctrl+Šift+E`: izvezi podatke o dokumentu (`.paperback`).
* `Ctrl+Šift+I`: uvezi podatke o dokumentu (`.paperback`).
* `Ctrl+E`: izvezi trenutni dokument kao običan tekst.
* `Ctrl+Šift+B`: dodaj ili ukloni knjižnu oznaku na poziciji trenutno izabranog teksta ili kursora.
* `Ctrl+Šift+N`: dodaj ili izmeni napomenu knjižne oznake na poziciji trenutno izabranog teksta ili kursora.
* `Ctrl+Alt+W`: uključi ili isključi prelamanje reči.
* `Ctrl+razmak`: reprodukuj ili pauziraj zvučnu naraciju.
* `'`: premotaj zvučnu naraciju napred.
* `;`: premotaj zvučnu naraciju nazad.
* `Ctrl+'`: povećaj količinu premotavanja zvuka.
* `Ctrl+;`: smanji količinu premotavanja zvuka.
* `F11` (macOS: `FizičkiCtrl+Ctrl+F`, CTRL+Command+F): uključi ili isključi režim celog ekrana.
* `Ctrl+,`: otvori podešavanja (macOS: postavke, u meniju aplikacije).
* `Ctrl+Šift+S`: uključi ili isključi tajmer pre spavanja.

### Meni pomoći

* `Ctrl+F1`: prikaži dijalog sa informacijama o programu.
* `F1`: prikaži pomoć u podrazumevanom pretraživaču.
* `Šift+F1`: prikaži pomoć u Paperbacku.
* `Ctrl+Šift+U`: proveri da li postoje ažuriranja.
* `Ctrl+D`: otvori stranicu za doniranje u podrazumevanom pretraživaču.

### Dodatni tasteri pri prikazivanju dokumenta

* `Delete` ili `numerički Delete` na kontroli kartica: zatvara izabranu karticu dokumenta.
* `Enter` ili `razmak` u tekstu dokumenta: aktivira link na poziciji kursora, ili otvara prikaz tabele kada ste na markeru tabele.
* `Šift+F10` ili aplikacioni/taster za meni u tekstu dokumenta: otvara kontekstni meni.

## Podržani jezici

Paperback je preveden na puno različitih jezika, a novi se stalno dodaju. Potpuna lista je ispod.

Kako biste saznali kako da doprinesete, molimo pročitajte naš [vodič za prevođenje na engleskom](translating.md).

* bosanski
* vijetnamski
* japanski
* nemački
* pojednostavljeni kineski
* poljski
* portugalski (Brazil)
* ruski
* srpski
* finski
* francuski
* holandski
* češki
* španski

## Zahvalnost
### Razvoj
* Quin Gillespie: primarni programer i osnivač projekta.
* Aryan Choudhary: primarni saradnik.

### Donacije
Sledeći ljudi su značajno doprineli razvoju  Paperbacka donacijom. Ako donirate vaše ime se neće automatski dodati u ovu listu, dodajem samo ljude koji žele da se njihove donacije objave.

Napomena: smatram javno GitHub sponzorisanje osnovu za automatsko dodavanje u ovu listu.

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

## Dnevnik promena

### Verzija 0.9.2
* Zvučne knjige više ne izazivaju da vaš čitač ekrana čita niz razmaka kada se fokusirate na tekstualno polje.
* Zvučne knjige sada imenuju datoteke kada se pomerate kroz njih po odeljcima.
* Zvučne knjige sada prijavljuju njihovo stvarno trajanje, umesto da prijavljuju da svaka datoteka u njima traje 24 sata.
* Zatvaranje Web prikaza tasterom Escape više ne prikazuje upozorenje o ispravljanju greške nakon što ste pratili link u njemu.
* Kopiranje nakon izbora celog teksta vam sada daje ceo dokument, umesto samo dela koji je trenutno učitan.
* Pretraga sada odmah izgovara red koji je pronađen, umesto da čeka dok čitač ekrana ponovo pročita prozor kada se fokus vrati na knjigu.
* Ispravljeni EPUBovi koji sadrže suvišan ZIP64 blok i odbijaju da se odvore uz grešku "neispravno lokalno zaglavlje datoteke".
* ispravljeni duži dokumenti koji su vas vraćali na njihov početak dok ih je čitač ekrana neprekidno čitao.
* Linkovi u Web prikazu vas sada prebacuju na odeljak ka kom vode, umesto da izbacuju grešku koja govori da "datoteka nije pronađena".
* Automatski izgovor koji vas obaveštava da je "Dokument ponovo učitan" više ne prekida vaš čitač ekrana u sred rečenice, umesto toga sačekaće da završi ono što je govorio.
* Opšta kartica dijaloga sa podešavanjima se sada tabom kreće po opcijama prema redosledu prikazivanja na ekranu, tako da će kanal ažuriranja biti odmah nakon opcije za proveru ažuriranja.
* Windows će sada uvek prikazati "Paperback" u meniju za otvaranje, umesto celog opisa programa.
* Broj reči i informacije o dokumentu sada prikazuju koliko zvučna knjiga ima datoteka, i koliko dugo ukupno traje.

### Verzija 0.9.1
* Zvukovi za knjižne oznake i napomene se sada reprodukuju na macOS-u.
* DAISY knjige sada reprodukuju svoj zvuk na macOS-u, umesto da se otvore i prate vremenski tok u tišini.
* Ispravljeni navodnici, em crtice i slični znakovi koji su nestajali iz RTF dokumenata, a reči koje su ih okruživale su se prikazivale zajedno kao jedna reč.
* Ispravljene RTF slike koje su svoje podatke prikazivale u dokumentu kao besmislen tekst.
* Ispravljen podmeni nedavnih dokumenata koji je zadržavao zastarele stavke dok se nešto drugo ne dogodi što bi ga ponovo osvežilo.
* Tasterske prečice su se vratile u svim prevodima, tako da će ruski meniji ponovo imati tasterske prečice.
* Veliki CHM dokumenti se sada otvaraju do sedam puta brže.
* Otvoreni dokumenti su sada registrovani u Windowsu, tako da će se prikazati u programskoj traci kao i u listi nedavnih dokumenata start menija.
* Opcije su preimenovane u podešavanja, što se podudara sa mobilnim aplikacijama, a na macOS-u, sa standardom platforme.
* Paperback sada pamti poziciju svog prozora, veličinu i da li je uvećan tokom različitih pokretanja.
* Oblici množina su sada prevedeni, tako da će se poruke koje broje stvari ispravno čitati na jezicima kojima je potrebno više od jednog oblika.
* Izbor ncc.html-a DAISY knjige  sada otvara potpunu zvučnu knjigu umesto samo njen tekst.
* Radnje dijaloga za prilagođavanje tasterskih prečica se sada mogu prevesti.
* Naslov dokumenta je sada prvi u traci naslova, kako bi otvorene knjige mogle da se razlikuju u programskoj traci i kada se koristi Alt+Tab.
* Dijalog za ažuriranje je sada preveden.

### Verzija 0.9.0

#### Dodato

##### Opšte
* Alat komandne linije, pb, koji brzo pretvara bilo koji format koji Paperback podržava u HTML, Markdown ili običan tekst.
* Opcija za ponovno učitavanje dokumenata koje su drugi programi izmenili na disku.
* Opcija za prikazivanje izvora koja otvara izvor dokumenta u novoj kartici, što može biti korisno na primer za uređivanje Markdown dokumenata.
* Tekst dokumenta se sada odvaja po stranicama, što znači da sada možete da učitate knjige sa desetinama miliona reči za nekoliko sekundi. Molimo prijavite bilo koje čudno ponašanje sa ovom funkcijom.

##### Podrška za platforme
* ARM64 Windows podrška!
* Izvorna macOS podrška!
* Opcija prikazivanja na celom ekranu.

##### Dijalog svih dokumenata
* Dugme za lociranje u dijalogu sa svim dokumentima koje vam dozvoljava da pronađete knjige koje nedostaju zato što je njihova putanja promenjena.
* Filtriranje stanja i statusna traka, kako biste mogli da izdvojite dokumente prema stanju dokumenta i vidite koliko dokumenata je prikazano, kao i izabrano.
* Prečica `Ctrl+Šift+A` da poništite izbor svih dokumenata.

##### Opcije i čitljivost
* Kartica čitljivosti, sa sledećim opcijama:
* Prelamanje reči (premešteno iz opšte kartice);
    * Obradi table u redu (novo u ovoj verziji, pogledajte ispod);
    * Font;
    * Boja pozadine;
    * Odvajanje redova;
    * Odvajanje pasusa;
    * Odvajanje slova;
    * Poravnanje teksta.
* Stavka menija za prelamanje reči kao i odgovarajuća prečica.
* Podešavanje koje određuje kako će se prikazivati tabele, a prikazivanje tabela je sada jedinstveno u svim dokumentima.

##### Navigacija
* Podrška za kretanje po sadrživačima.
* Opcija za automatsko pomeranje kursora na početak reda kada se krećete po redovima, slično režimu pretraživanja u čitačima ekrana.
* Taster jednako koji izgovara vaš trenutni procenat u dokumentu.

##### Knjižne oznake
* Privremene knjižne oznake: možete imati jednu po dokumentu, i one ostaju. Koristite kosu crtu da postavite privremenu knjižnu oznaku a obrnutu kosu crtu da skočite na nju.

##### Broj  reči
* Očekivano vreme čitanja, kao i mogućnost da podesite vašu brzinu čitanja kako bi ovo vreme zapravo bilo korisno.
* Ako je tekst izabran kada otvorite dijalog sa brojem reči, biće prikazano koliko reči ste izabrali.

##### Tasterske prečice
* Mogućnost da prilagodite svaku tastersku prečicu u aplikaciji kroz jednostavan dijalog.
* Mogućnost da podesite prečicu za vraćanje Paperbacka iz sistemske trake.

##### Jezici
* Holandski, finski i poljski.

##### Izvoz
* Meni izvoza je proširen i sada dozvoljava izvoz u HTML i Markdown uz običan tekst.

##### Ažuriranje
* Dugme za otkazivanje u dijalogu ažuriranja koje je u toku.
* Ažuriranje će sada proveriti da li  preuzeta datoteka nije neočekivano izmenjena.

##### Web prikaz
* Web prikaz se sada otvara na vašoj trenutnoj poziciji čitanja.

##### DAISY knjige
* Podrška za DAISY 2.0 knjige.
* Podrška Za reprodukciju DAISY 2.02 zvuka.

##### Zvučne knjige
* Mogućnost reprodukcije zvučnih knjiga, trenutno podržava i DAISY zvuk (uključjući DAISY zvuk + tekst) i zipove zvučnih datoteka.
* Tasterske prečice i stavke menija za reprodukciju ili pauziranje naracije, premotavanje napred i nazad i menjanje količine premotavanja.
* Opcija sinhronizacije kursora sa reprodukcijom zvuka, podešavanje količine premotavanja zvuka kao i izbor da li će premotavanje nakon kraja poglavlja nastaviti na sledeće poglavlje.

##### CHM dokumenti
* Podrška za liste, stavke liste, figure i slike.

##### PowerPoint
* PowerPoint dokumenti sada podržavaju tabele.

#### Ispravljeno

##### Opšte
* Dokumenti kodirani u zastarelim CJK kodiranjima, kao što su GBK, Big5 i Shift_JIS će se sada ispravno obraditi umesto kao gomila  modžibakea.
* Opcija za ponovno otvaranje poslednjeg zatvorenog dokumenta je pokušavala da otvori ugrađenu pomoć.
* Vaša izabrana kartica se nije ispravno fokusirala nakon što se Paperback ponovo pokrene.
* Paperbackovo obrađivanje datoteka na Windows mrežnim diskovima: aktiviranje opcije za prikazivanje datoteke u folderu će sada ispravno fokusirati datoteku na mrežnoj memoriji, a putanje više neće sadržati čudne znakove.
* .paperback datoteke se više neće prisilno učitati nakon što se dokument vrati; umesto toga, bićete upitani za potvrdu kada se datoteka pronađe.
* Otvaranje izvornog foldera sada fokusira datu datoteku u istraživaču datoteka.
* Otvaranje dokumentacije će sada uzeti u obzir vaš izabran jezik.
* Paperbackov korisnički interfejs će se sada ispravno pozicionirati na ekranima sa visokim DPI-om.
* Meni se ispravno ažurira i fokus se prebacuje na kontrolu teksta kada se pomoć otvori u Paperbacku.
* Prelazak na mnogo bezbedniji način međuprocesne komunikacije na Windowsu.
* Naslov aktivnog dokumenta će se sada čitati kada se prebacujete između kartica.
* Smanjena potrošnja memorije u velikim dokumentima tako što  je veličina internih indeksnih tabela po znaku prepolovljena.

##### Dijalog svih dokumenata
* Taster Escape  nije zatvarao dijalog informacija o dokumentu i dijalog sa svim dokumentima.
* Traka naslova se nije ažurirala nakon što se dokument zatvori iz dijaloga svih dokumenata.
* Readme.html se više neće dodavati u vašu listu svih dokumenata kada se otvori prečicom Šift+F1.
* Uklanjanje dokumenata iz dijaloga nedavnih dokumenata sada takođe zatvara njihovu aktivnu karticu.
* Vaš filter pretrage se sada čuva nakon što se dokument ukloni.

##### Navigacija
* Navigacija po stranicama je izgovarala neispravan tekst reda u nekim situacijama.
* Funkcije prelaska u red, prelaska na stranicu i prelaska na procenat su vaš kursor stavljale na pogrešnu poziciju u većim dokumentima.
* Funkcije pretrage i pretrage sledećeg nisu uzimale u obzir prozor učitanog dokumenta u većim dokumentima.

##### Knjižne oznake
* Zvukovi knjižnih oznaka i napomena bi sada ispravno trebali da se reprodukuju samo kada se krećete po rečima koje ih sadrže.

##### Čitljivost
* Primena prelamanja reči vas je vraćala na početak dokumenta.

##### Web prikaz
* Veličina dijaloga Web prikaza nije bila promenljiva, a on se takođe otvarao u veoma maloj veličini.
* Slike bi sada trebale ispravno da se prikažu u umetnutom Web prikazu.

##### Ažuriranje
* Prikaz ažuriranja sada ispravno prikazuje sadržaj markdown tagova koda u informacijama o verziji.

##### DAISY knjige
*  daisy knjige su prikazivale pogrešne informacije na statusnoj traci.
* Učitavanje DAISY knjiga sa lažnim deklaracijama o kodiranju.

##### RTF dokumenti
* Obrađivanje RTF dokumenata koji imaju znakove koji nisu latinični.
* RTF `\pict` grupe kao umetnuti podaci slika se više ne prikazuju kao tekst dokumenta.

##### Mobi/AZW3 knjige
* filepos veznici u Mobi knjigama su razdvajali HTML tagove i ostavljali smeće u tekstu knjige.
* Linkovi u zastarelim Mobi knjigama.
* Značajno poboljšano AZW3 obrađivanje.

##### Word dokumenti
* Word dokumenti sa imenima stilova u zavisnosti od jezika nisu ispravno obrađivali naslove.

##### HTML/XHTML dokumenti
* dl, dt i dd elementi nisu dodavali nove redove u XHTML dokumentima.

##### PDF dokumenti
* Paperback se vraća na izvlačenje običnog teksta za PDF datoteke sa pogrešnim tagovima.
* PDF dokumenti koji sadrže kontrolne znakove u naslovima i/ili knjižnim oznakama više neće rušiti Paperback pri otvaranju.
 
### Verzija 0.8.5
* Dodata podrška za stranice u epub knjige.
* Dodata podrška za šifrovane  Microsoft Office dokumente. Trenutno su podržani zastareli word, moderni Word i moderni Powerpoint, a zastareli Powerpoint je planiran u budućnosti.
* Dodata podrška za zastarele Microsoft Word dokumente (*.doc)!
* Dodata podrška za zastarele Powerpoint prezentacije (*.ppt)!
* Dodata podrška za mobi i AZW3 knjige!
* Dodata podrška za tagovane PDF datoteke!
* Dodata prečica ctrl+q za izlaz iz aplikacije.
* Dodata podrška za zipovane Bookshare knjige (i DAISY i Word)!
* Alt tekst za umetnute slike bi sada trebao da se ispravno prikaže.
* CHM dokumenti sada ispravno podržavaju internu navigaciju po linkovima.
* Ispravljeno reprodukovanje zvukova knjižnih oznaka na početku pasusa umesto na poziciji knjižne oznake.
* Ispravljen prelazak na stranicu koji je bio neprecizan za 1.
* Ispravljen taster Escape koji nije radio u dijalogu otvori kao.
* Ispravljen kontekstni meni čitača koji se nije prikazivao desnim klikom ili aplikacionim tasterom.
* Ispravljeno povremeno fokusiranje pogrešnog dokumenta kada se dokumenti otvaraju iz komandne linije.
* PDF datoteke koje su samo u slikama se ponovo prepoznaju i upozoravaju vas o tome.
* Sada možete da se krećete kroz slike i figure prečicama g/Šift+g i f/Šift+F.
* Paperback će sada poštovati vaše podešavanje tamnog režima.
* Uklonjena DAISY XML podrška, budući da više nije potrebna.
* Vraćena izvorna Win32 navigacija prvim slovom u stablu sadržaja.
* Dijalog greške učitavanja sada prikazuje detaljnije poruke sa greškom.
* Web prikaz će se sada otvarati puno brže i udobnije.

### Verzija 0.8.2
* Dodata podrška za stranice u RTF dokumentima!
* Ispravljena greška zbog koje otvaranje Web prikaza u  epub dokumentima koji sadrže eksterne linkove  automatski  aktivira linkove.
* Ispravljena greška zbog koje  RTF obrađivač ne stavlja razmak između reči u retkim slučajevima.
* Ispravljeno odvajanje pasusa u manje kraće redove u nekim PDF dokumentima.
* PDF dokumenti sada imaju osnovnu podršku za kretanje po naslovima i linkovima!
* RTF tabulatori i prekidi redova se sada prikazuju onako kako se pojavljuju u dokumentu.
* Vraćena isprobana i testirana biblioteka pdfium za obrađivanje PDF datoteka, što ponovo čini obrađivanje PDF datoteka puno pouzdanijim.

### Verzija 0.8.1
* Dodata prečica Ctrl+Šift+T za ponovno otvaranje poslednjeg zatvorenog dokumenta.
* Dijalog svih dokumenata sada podržava izbor više dokumenata za otvaranje odjednom.
* Ispravljeno nekoliko grešaka sa RTF obrađivačem.
* Ispravljena greška zbog koje putanje datoteka koje sadrže znakove koji nisu ASCII (kao što su naša slova  š, č, ć, ž) postaju oštećene kada se datoteka otvori drugom kopijom Paperbacka.
* Ispravljeno čitanje PDF teksta u pogrešnom redosledu, i neispravni razmaci oko reči sa velikim slovima.
* Ispravljeno sporo učitavanje dokumenta kada se otvaraju velike datoteke.
* Ispravljeni prevodi za tastere da i ne u dijalozima za potvrdu.

### Verzija 0.8.0
* Dodati japanski, pojednostavljeni kineski i vijetnamski prevodi!
* Dodato automatsko ažuriranje koje će sada zameniti vašu trenutnu instaliranu verziju Paperbacka umesto da samo preuzme novu verziju!
* Dodate opcione zvučne povratne informacije kada dođete do napomene ili knjižne oznake, hvala Andre Louis za zvukove!
* Dodata podrška za RTF dokumente!
* Dodata podrška za DAISY XML dokumente.
* Dodata podrška za Flat Open Document Text datoteke!
* Dodata podrška za Flat Open Document prezentacije!
* Dodata podrška za separatore uz s i Šift+s.
* Bilo koje kretanje koje je obimnije od 300 znakova će se sada automatski dodati u vašu istoriju navigacije.
* Ispravljeno vraćanje prozora Paperbacka iz sistemske trake.
* Ispravljeno prikazivanje Markdown dokumenata koji su prikazivali običan tekst umesto obrađenog HTML-a u Web prikazu.
* Ispravljeno neispravno obrađivanje tabela u Markdown datotekama.
* PDF datoteke koje sadrže samo slike će vas sada upozoriti o tome kada pokušate da ih učitate.
* Sada je moguće proveriti da li postoje nove razvojne (dev) verzije umesto stabilnih verzija pri proveri ažuriranja.
* Informacije o verziji su sada ispravno umetnute u Paperback izvršnoj datoteci.
* Dijalog sa podešavanjima je podeljen na kartice radi lakšeg korišćenja i kretanja.
* Prelazak na Hayro za obradu PDF datoteka, što donosi bolju pouzdanost, brzinu i manje DLL-ova.
* Cela aplikacija je prepisana u Rust. Novi kod je brži, brže učitava dokumente i lakši je za održavanje i proširivanje.
* Kontekstni meni kontrole teksta će sada uključiti radnje vezane za čitač umesto generičkih stavki kao što su iseci i nalepi.

### Verzija 0.7.0
* Dodata podrška za tabele u dokumentima zasnovanim na HTML-u i XHTML-u! Krećite se kroz tabele korišćenjem prečica T i Šift+T, a pritisnite Enter da biste ih pogledali u Web prikazu.
* Dodata osnovna funkcija Web obrade! Pritisnite Ctrl+Šift+V da biste otvorili trenutni odeljak vašeg dokumenta u obrađivaču zasnovanom na Webu, što može biti korisno za sadržaj sa kompleksnim formatiranjem ili delove koda.
* Dodat ruski prevod, hvala Ruslan Gulmagomedov!
* Dodato dugme za čišćenje svih dokumenata u dijalogu sa dokumentima.
* Provera ažuriranja sada prikazuje informacije o novoj verziji kada je nova verzija dostupna.
* Ispravljeno vraćanje prozora iz sistemske trake.
* Ispravljeni prevodi za tastere da/ne u dijalozima za potvrdu.
* Ispravljeno učitavanje podešavanja prilikom pokretanja kao administrator.
* Ispravljeno obrađivanje dokumenata u XML i HTML dokumentima.
* Ispravljeno obrađivanje sadržaja u Epub 2 knjigama.
* Ispravljeno kretanje do stavke koja počinje sa istim slovom u sadržaju.
* Ispravljen dijalog za pretragu koji se nije ispravno sakrio kada se koriste tasteri naredno/prethodno.
* Ispravljeni epub sadržaji koji su vas ponekad vraćali na pogrešnu stavku.
* Ispravljeni razni problemi sa obradom praznih razmaka u XML, HTML i pre tagovima.
* Ispravljena greška u  navigaciji po linkovima koja je bila neprecizna za 1.
* Ispravljena greška zbog koje su neke knjige imale početne prazne razmake u redovima.
* Ispravljeni razni problemi obrade.
* Stavke menija vezane za knjižne oznake kao i lista elemenata su sada ispravno onemogućeni kada nijedan dokument nije otvoren.
* Poboljšano obrađivanje lista u raznim formatima dokumenata.
* Poboljšan proces prevođenja za saradnike.
* Puno internih refaktorisanja, prelazak značajnog dela logike rada aplikacije iz C++-a u Rust radi poboljšane brzine i održivosti.

### Verzija 0.6.1
* Dodata podrška za PDF datoteke zaštićene lozinkom!
* Dodata veoma osnovna funkcija prelaska na prethodnu ili sledeću poziciju. Ako pritisnete Enter na interni link i ovo pomeri vaš kursor, ova pozicija će sada biti zapamćena, i možete se vratiti na nju prečicama alt+strelice levo/desno.
* Dodata lista elemenata! Trenutno prikazuje samo stablo svih naslova u vašem dokumentu ili listu linkova, ali je njeno proširivanje planirano u budućnosti.
* Dodata opcija za podrazumevano ppokretanje Paperbacka u maksimizovanom režimu.
* Ispravljeni linkovi koji nisu ispravno radili u nekim Epub dokumentima.
* Ispravljeno obrađivanje Epub sadržaja koji sadrže relativne putanje.
* Ispravljeno prikazivanje naslova ili autora u nekim epub dokumentima.
* Ispravljeni naslovi nekih  epub poglavlja koji se nisu ispravno prikazivali u dijalogu sa sadržajem.
* Ispravljena nemogućnost korišćenja razmaka za aktiviranje tastera u redu ili otkaži u dijalogu sadržaja.
* Poboljšano obrađivanje naslova u Word dokumentima.
* Sada ćete dobiti govorne povratne informacije ako je lista nedavnih dokumenata prazna kada pokušate da otvorite dijalog.

### Verzija 0.6.0
* Nova opcija za prikazivanje menija navigacije u mnogo kompaktnijoj formi je dodata u dijalog sa podešavanjima, podrazumevano je omogućena.
* Dodata opcija za kružnu navigaciju kroz strukturalne elemente.
* Dodata opcija u meni sa alatima za otvaranje izvornog foldera trenutno fokusiranog dokumenta.
* Dodat izuzetno jednostavan, ali vrlo efikasan sistem ažuriranja.
* Dodata osnovna funkcija tajmera pre spavanja, kojoj se može pristupiti prečicom Ctrl+Šift+S.
* Dodata podrška za obradu FB2 digitalnih knjiga!
* Dodata podrška za obradu OpenDocument prezentacija!
* Dodata podrška za obradu OpenDocument tekstualnih datoteka!
* Knjižne oznake sada mogu da označe ceo red, ili samo neki određen tekst. Ako nemate izabran tekst kada dodajete knjižnu oznaku, ponašanje je isto kao pre verzije 0.6, i označiće ceo red. Međutim, ako izaberete neki tekst, samo taj tekst će biti dodat kao knjižna oznaka.
* Knjižne oznake sada mogu da imaju priložene tekstualne napomene! Krećite se kroz knjižne oznake koje sadrže napomene prečicama N i Šift+N, ili otvorite dijalog sa svim knjižnim oznakama, samo sa napomenama, ili samo bez napomena sa određenim prečicama.
* Knjižne oznake u dijalogu sa knjižnim oznakama više neće imati dosadan "Knjižna oznaka x" prefiks.
* Epub knjige koje sadrže HTML sadržaj za obrađivanje kao XML će sada ispravno biti obrađene.
* Ispravljeno učitavanje velikih Markdown dokumenata.
* Ispravljeno pritiskanje razmaka u stablu sadržaja koje je aktiviralo dugme OK.
* Ispravljena obrada praznih razmaka na početku pre tagova u HTML i XHTML dokumentima.
* Ispravljeno fokusiranje tekstualne kontrole koje se nekada nije događalo nakon što se vratite u prozor Paperbacka.
* Ispravljeno tekstualno polje u dijalogu za prelazak na procenat koje nije ažuriralo vrednost klizača.
* Ispravljena obrada prilagođenih HTML ID-ova u Markdown dokumentima.
* HTML u Markdown blokovima koda će se sada ispravno obraditi.
* Ako učitavate knjigu parametrom iz komandne linije dok je postojeća kopija Paperbacka pokrenuta, više nećete dobiti grešku ako učitavanje vašeg dokumenta traje duže od 5 sekundi.
* Ako je Paperback pokrenut kao administrator, podešavanja će se sada ispravno učitati i sačuvati.
* Sada je moguće obrisati knjižnu oznaku direktno iz dijaloga sa knjižnim oznakama.
* Sada je moguć uvoz i izvoz vaših knjižnih oznaka i pozicije čitanja za određeni dokument. Napravljena datoteka će biti imenovana po imenu datoteke sa .paperback ekstenzijom. Ako ovakva datoteka postoji u istom folderu kao i datoteka koju čitate, automatski će biti učitana. U suprotnom, možete ručno da ih uvezete korišćenjem stavke u meniju sa alatima.
* Linkovi u dokumentima su sada u potpunosti podržani! Koristite k i Šift+k da se krećete napred i nazad kroz njih, i pritisnite Enter da ga otvorite ili aktivirate.
* Puno internog refaktorisanja, što aplikaciju čini bržom i veličinu datoteke manjom.
* Markdown sadržaj se sada unapred obrađuje kako bi bio u skladu sa standardom CommonMark pre obrade.
* Kretanje kroz liste i njihove stavke je sada u potpunosti podržano! Koristite L i Šift+L da se krećete po listama, a I i Šift+I da se krećete po stavkama liste.
* Numerički delete sada radi za uklanjanje dokumenata sa trake sa karticama uz standardni Delete.
* Paperback se sada može opciono minimizovati u vašu sistemsku traku! Ova opcija je podrazumevano isključena, ali će njeno uključivanje učiniti da opcija za minimizovanje u sistemskom meniju stavi Paperback u sistemsku traku, nakon čega se može vratiti klikom na ikonicu koja se pojavila.
* Paperback se sada u potpunosti može prevesti! Lista podržanih jezika je trenutno prilično mala, ali stalno raste!
* Paperback sada ima zvaničan websajt, na [paperback.dev](https://paperback.dev)!
* PPTX dokumenti će sada prikazati osnovni sadržaj, koji sadrži sve slajdove.
* Cela putanja do otvorenog dokumenta će se sada prikazati u dijalogu sa informacijama o dokumentu.
* Instalacija sada uključuje opciju za prikazivanje dokumentacije u vašem pretraživaču nakon instalacije.
* Lista nedavnih dokumenata je značajno proširena! Umesto da vam prikaže poslednjih 10 dokumenata koje ste otvorili, prikazaće vam prilagođeni broj, a ostatku dokumenata koji ste ikada otvorili se može pristupiti kroz manji dijalog.
* Razna manja poboljšanja u celokupnoj obradi, koja uključuju stavljanje praznog reda između slajdova u PPTX prezentacijama, ispravljanje obrade novih redova u pasusima word dokumenata, i dodavanje znakova nabrajanja za stavke liste.

### Verzija 0.5.0
* Dodata podrška za Microsoft Word dokumente!
* Dodata podrška za PowerPoint prezentacije!
* Ispravljene određene stavke menija koje nisu bile onemogućene kada nema otvorenih dokumenata.
* Ispravljena orijentacija klizača za prelazak na procenat.
* Ispravljen sadržaj u Epub knjigama sa URL-kodiranim putanjama datoteka ili ID-ovima fragmenata.
* Ispravljeno uklanjanje praznih razmaka iz XHTML naslova na čudne načine.
* Ispravljena obrada praznih razmaka u nizanim pre tagovima u HTML dokumentima.
* HTML i Markdown dokumenti sada podržavaju funkciju sadržaja! Kada učitate HTML/Markdown dokument, Paperback će izgraditi svoj sadržaj na osnovu strukture naslova vašeg dokumenta, i prikazaće vam ovaj sadržaj u dijalogu koji se otvara prečicom ctrl+t.
* HTML dokumenti će sada imati naslov koji je podešen u title tagu, ako postoji. U suprotnom, nastaviće da koriste ime datoteke bez ekstenzije.
* Prelazak sa biblioteke UniversalSpeech na korišćenje žive regije za prijavljivanje govora. Ovo znači da DLL datoteke čitača ekrana više ne dolaze uz program, a dodatni čitači ekrana će biti podržani, kao što je Microsoft Narrator.
* Promenjene Zip biblioteke kako bi se dozvolilo otvaranje većeg obima epub knjiga.
* Dijalog koji vas pita da li želite da otvorite vaš dokument kao običan tekst je u potpunosti redizajniran, i sada vam dozvoljava da otvorite vaš dokument kao običan tekst, HTML, ili Markdown.
* Dijalog za prelazak na procenat sada sadrži tekstualno polje koje vam dozvoljava da ručno upišete procenat na koji želite da skočite.
* HTML obrađivač će sada prepoznati dd, dt i dl kao elemente liste.
* Sadržaj u Epub knjigama će ponovo biti precizno očuvan.
* Unikodni neprekidan razmak se sada uzima u obzir kada se uklanjaju prazni redovi.
* Više nećete biti upitani kako želite da otvorite datoteku koja nije prepoznata svaki put kada je učitate, već samo prvi put.

### Verzija 0.4.1
* Dodata opciona ikonica u start meniju tokom instalacije.
* Sadržaj bi sada trebao da bude jasniji u nekoliko slučajeva, na primer ako imate podnaslov i naslov sa istim tekstom na istoj poziciji videćete samo naslov.
* Ispravljen sadržaj u nekim CHM dokumentima.
* Ispravljen sadržaj u Epub 3 knjigama koje u sebi sadrže apsolutne putanje.
* CHM dokumenti bi sada trebali da prikažu svoj naslov onako kako je postavljen u meta podacima datoteke.

### Verzija 0.4.0
* Dodata podrška za CHM datoteke!
* Dodata podrška za knjižne oznake! Možete imati koliko god knjižnih oznaka želite kroz koliko god dokumenata želite. Možete se kretati napred i nazad kroz njih prečicama b i Šift+b, dodati novi prečicom control+Šift+b, ili otvoriti dijalog za skakanje na određenu knjižnu oznaku prečicom control+b.
* Dodata instalacija uz prenosnu zip datoteku! Instalacija će instalirati Paperback u vaš Program Files direktorijum, i automatski pridružiti podržane datoteke.
* Tekstualne datoteke uz BOM bi trebale da budu ispravno dekodirane, a BOM se takođe neće prikazivati na početku teksta.
* Dodato mnogo više informacija u statusnu traku. Sada će vam prikazati vaš trenutni red, znak i procenat čitanja.
* HTML komentari, kao i sadržaj script i style tagova se više neće prikazivati u tekstu.
* Ako prosledite relativnu putanju Paperbacku putem komandne linije, biće ispravno obrađena.
* Kretanje kroz procenat se sada obrađuje sopstvenim dijalogom sa klizačem, kojem se može pristupiti prečicom control+Šift+g.
* Dokumenti bez poznatog naslova ili autora će sada uvek imati podrazumevane informacije.
* Logika čuvanja pozicije je sada mnogo pametnija i trebala bi da piše na disk samo kada je apsolutno neophodno.
* Dokument na koji ste se fokusirali kada ste zatvorili Paperback se sada pamti kada se aplikacija restartuje.
* Unosi u dijalozima za prelazak u red ili prelazak na stranicu bi sada trebali da se striktnije proveravaju.
* Ispravljena navigacija kroz sadržaj u epub 3 knjigama koje imaju relativne putanje u njihovim manifestima.

### Verzija 0.3.0
* Ispravljen sadržaj u epub knjigama sa URL-kodiranim manifestima.
* Ispravljena navigacija po naslovima u HTML dokumentima koji sadrže unikodne znakove sa više bajtova.
* Ispravljeno visoko opterećenje procesora u dokumentima sa dugim naslovima zbog pogoršanja u wxWidgets-u.
* Ispravljeno učitavanje UTF-8 tekstualnih datoteka.
* Ispravljene nizane stavke sadržaja u Epub knjigama koje su vaš kursor stavljale na pogrešnu poziciju.
* Ispravljeno rušenje nakon zatvaranja aplikacije u određenim slučajevima.
* Dodato izborno polje u dijalogu sa podešavanjima za omogućavanje ili onemogućavanje prelamanja reči!
* Sada je moguće donirati u svrhu razvoja Paperbacka, putem nove stavke menija za doniranje u meniju pomoći ili putem linka sponsor this project na dnu glavne stranice GitHub repozitorijuma.
* Markdown dokumenti će sada uvek imati naslov, a Paperback bi sada trebao da može da učita praktično bilo koju Markdown datoteku.
* PDF dokumenti će sada uvek imati naslov, čak i kada meta podaci nedostaju.
* Promenjene PDF biblioteke na one koje koristi Chromium, što donosi znatno pouzdaniju obradu PDF datoteka.
* Sada možete imati samo jednu pokrenutu kopiju Paperbacka u datom trenutku. Pokretanje paperback.exe uz ime datoteke dok je već pokrenut će otvoriti taj dokument u već pokrenutoj kopiji.
* Sada možete da pritisnete delete na dokumentu u kontroli kartica da biste ga zatvorili.

### Verzija 0.2.1
* Dodat ukupan broj stranica u oznaci stranica u dijalogu prelazka na stranicu.
* Dozvoljeno pritiskanje tastera Tab iz sadržaja dokumenta do liste vaših otvorenih dokumenata.
* Ispravljene prečice za kretanje po naslovima koje su ponekad otvarale nedavne dokumente ako ste ih imali dovoljno.
* Paperback će sada ukloniti bespotrebne meke crtice iz teksta.
* Ispravljena navigacija po naslovima koja vas je ponekad prebacivala na pogrešan znak.

### Verzija 0.2.0
* Dodata podrška za markdown dokumente!
* Dodata podrška za PDF dokumente, uključujući mogućnost kretanja po stranicama!
* Dodate prečice za kretanje po naslovima u HTML sadržaju, uključujući epub knjige i markdown dokumente. Ove prečice su dizajnirane tako da rade slično prečicama čitača ekrana.
* Ispravljeno učitavanje epub-ova sa URL-kodiranim imenima datoteka u njihovim manifestima.
* Ispravljeno učitavanje epub 3 knjiga sa umetnutim XHTML-om u njima.
* Poruka se sada izgovara ako dokument ne podržava sadržaj ili odeljke, umesto da stavke menija budu onemogućene.
* Dodat meni nedavnih dokumenata! Trenutno čuva vaših poslednjih 10 dokumenata, a pritiskanje Enter na nekom od njih će ga otvoriti za čitanje.
* Potpuno redizajniran dijalog pretrage, sada je puno jednostavniji za korišćenje, a takođe sadrži i istoriju vaših poslednjih 25 pretraga i podršku za regularne izraze!
* Prethodno otvoreni dokumenti se sada pamte nakon restartovanja aplikacije. Ovo se može podesiti kroz novu stavku za podešavanja u meniju sa alatima.
* Dodata prečica Šift+f1 za otvaranje dokumentacije direktno u samom Paperbacku.

### Verzija 0.1.0
* Prva verzija.
