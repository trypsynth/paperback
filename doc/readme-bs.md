# Paperback - verzija 1.0

## Uvod

Paperback je lagan, brz i pristupačan čitač e-knjiga, dokumenata i zvučnih knjiga za svakoga, od običnih čitatelja do naprednih korisnika. Osmišljen je za pristupačnost s čitačima zaslona, velike brzine i iskustvo bez nepotrebnih stvari.

## Sistemski zahtjevi

Paperback radi na Windowsu 10/11, svim modernim verzijama macOS-a za ARM procesore, Linuxu, iOS-u 17 i novijim te Androidu 7 i novijim. Aplikacije za iOS i Android dostupne su na App Storeu i Google Playu.

## Funkcije

* Potpuno je samostalan, ne zahtijeva instaliranje nikakvog dodatnog softvera na vaš računar za početak čitanja.
* Nevjerovatno je brz, čak i na starom hardveru.
* Ima jednostavno sučelje s karticama, koje vam omogućava otvaranje koliko god dokumenata želite jedan pored drugog.
* Pamti tačan položaj čitanja u svakom dokumentu koji otvorite.
* Po želji pamti koje ste dokumente otvorili kada ste zatvorili program i vraća ih pri sljedećem pokretanju.
* Uključuje funkcije kretanja slične onima koje se nalaze u načinu pretraživanja weba mnogih čitača zaslona za brzo i jednostavno kretanje kroz dokumente.
* Uključuje opsežan dijalog za traženje, uključujući funkcije kao što su historija i podrška za regularne izraze.
* Može se koristiti kao prenosivi program ili instalirati uz automatsko postavljanje pridruživanja datoteka.
* Podržava veliki broj uobičajenih formata datoteka.
* Reprodukuje zvučne knjige, uz podesivu brzinu i oznake koje pamte tačno vrijeme.
* Čita skenirane PDF stranice pomoću OCR-a ugrađenog u Windows i macOS.
* Podržava oznake i bilješke, pa možete obilježiti mjesto do kojeg ste stigli i kasnije mu se vratiti.
* Svaka prečica na tastaturi može se promijeniti.
* Uključuje `pb`, alat za komandnu liniju koji bilo koji podržani dokument pretvara u HTML, Markdown ili običan tekst.

## Kompatibilnost s čitačima zaslona

Paperback radi vrlo dobro s većinom poznatih čitača zaslona. Međutim, postoje dva poznata problema za korisnike JAWS-a.

### JAWS i brajevi zasloni

Ako koristite JAWS s brajevim zaslonom, mogli biste primijetiti da su dugi odlomci skraćeni prilikom kretanja naprijed tasterima za navigaciju na vašem brajevom zaslonu. Komanda za čitanje trenutnog odlomka također je zahvaćena. Ovo je greška u načinu na koji JAWS rukuje tekstualnom kontrolom `RICHEDIT50W`, a ne u samom Paperbacku. Nažalost, trebalo je prilično dugo da Vispero ispravi ovaj problem zbog njihovog nedostatka entuzijazma za odgovaranje na prijave problema u softveru otvorenog koda.

Rješenje za ovaj problem, koji se nakon višemjesečnog čekanja na kraju pojavio u JAWS grupi za diskusiju, jeste da uredite datoteku `paperback.jcf` i opciju "Braille Presentation and Panning" postavite na "Always use DOM if available". Također ćete morati uključiti opciju "Pan Text by Paragraph", jer će u suprotnom vaš brajev zaslon ostati na trenutnom odlomku umjesto da prelazi na sljedeći. Kada su obje opcije uključene, kretanje po tekstu trebalo bi ispravno raditi.

### JAWS i poruke Paperbacka

Paperback poruke poput "Nema stranica." ili "Ovaj dokument nema zvučni zapis." šalje kao obavještenja pristupačnosti, zahvaljujući čemu ih čitač zaslona može izgovoriti čak i dok čita nešto drugo. JAWS na takva obavještenja reaguje samo kada je za aplikaciju uključena opcija "Enable accessible notification events", a na nekim računarima ona nije uključena.

Ako JAWS ništa ne izgovori kada pritisnete taster koji bi trebao nešto prijaviti, otvorite Settings Center dok je Paperback u prvom planu (`Insert+6`), potražite "notification" i označite opciju "Enable accessible notification events". Time se ova postavka upisuje u datoteku `paperback.jcf`, pa se primjenjuje samo na Paperback.

## Trenutno podržane vrste datoteka

Paperback podržava sljedeće formate i ekstenzije:

* Arhive stripova (`.cbz`)
* CHM datoteke pomoći (`.chm`)
* DAISY knjige (`.opf`, `.zip`)
* EPUB e-knjige (`.epub`)
* FB2 e-knjige (`.fb2`)
* HTML dokumente (`.htm`, `.html`, `.xhtml`)
* Stranice priručnika, u formatu `man` i BSD `mdoc` (od `.1` do `.9`, `.man`, `.roff`, kao i njihove gzip komprimovane verzije)
* Markdown dokumente (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word dokumente (`.docx`, `.docm`, `.doc`)
* M4B zvučne knjige (`.m4b`)
* MOBI/Kindle knjige (`.mobi`, `.azw`, `.azw3`)
* MP3 zvučne knjige (`.mp3`)
* OpenDocument prezentacije (`.odp`, `.fodp`)
* OpenDocument tekstualne datoteke (`.odt`, `.fodt`)
* PDF dokumente (`.pdf`)
* PowerPoint prezentacije (`.pptx`, `.pptm`, `.ppt`)
* reStructuredText dokumente (`.rst`, `.rest`)
* RTF dokumente (`.rtf`)
* Windows Write dokumente (`.wri`)
* WinHelp datoteke (`.hlp`)
* Običan tekst i datoteke dnevnika (`.txt`, `.log`)

## Prečice na tastaturi

Paperback je osmišljen prvenstveno za korištenje putem tastature. Evo trenutno dostupnih prečica.

Prečice navedene u nastavku odnose se na Windows. Tamo gdje se na macOS-u razlikuju, odgovarajuća prečica navedena je u zagradi. Razlog je uglavnom to što su `Control+G`, `Control+W` i `Alt+Strelica lijevo/desno` na toj platformi već zauzete sistemskim prečicama ili ustaljenim prečicama aplikacija.

### Izbornik `Datoteka`

* `Control+O`: Otvara dokument.
* `Control+F4` (macOS: `Cmd+W`): Zatvara trenutni dokument.
* `Control+Shift+F4` (macOS: `Cmd+Shift+W`): Zatvara sve otvorene dokumente.
* `Control+Shift+T`: Ponovo otvara posljednji zatvoreni dokument.
* `Control+R`: Otvara dijalog "Svi dokumenti" (iz nedavnih dokumenata).
* `Control+Q`: Izlazi iz programa (samo na Windowsu; na macOS-u se ova stavka nalazi u izborniku aplikacije).

### Izbornik `Idi`

* `Control+F`: Otvara dijalog "Traži".
* `F3` (macOS: `Cmd+G`): Traži sljedeće.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Traži prethodno.
* `Control+G` (macOS: `Cmd+L`): Otvara dijalog "Idi na red".
* `Control+Shift+G` (macOS: `Cmd+Shift+L`): Otvara dijalog "Idi na postotak".
* `Control+P`: Otvara dijalog "Idi na stranicu" (kad dokument to podržava).
* `=`: Izgovara trenutni postotak pročitanog dokumenta i stranicu, npr. "15%, stranica 30". Kod dokumenata koji nemaju brojeve stranica stranica se izostavlja.
* `Alt+Strelica lijevo` (macOS: `Cmd+[`): Ide natrag u historiji kretanja.
* `Alt+Strelica desno` (macOS: `Cmd+]`): Ide naprijed u historiji kretanja.
* `[`: Prethodni odjeljak.
* `]`: Sljedeći odjeljak.
* `Shift+H`: Prethodni naslov.
* `H`: Sljedeći naslov.
* Od `Shift+1` do `Shift+6`: Prethodni naslov nivoa od 1 do 6.
* Od `1` do `6`: Sljedeći naslov nivoa od 1 do 6.
* `Shift+P`: Prethodna stranica.
* `P`: Sljedeća stranica.
* `Shift+B`: Prethodna oznaka.
* `B`: Sljedeća oznaka.
* `/`: Postavlja privremenu oznaku.
* `\`: Prelazi na privremenu oznaku.
* `Shift+N`: Prethodna bilješka.
* `N`: Sljedeća bilješka.
* `Control+B`: Prelazi na sve oznake i bilješke.
* `Control+Alt+B`: Prelazi samo na oznake.
* `Control+Alt+M`: Prelazi samo na bilješke.
* `Control+Shift+W` (macOS: `RawCtrl+Shift+W`, tj. fizički taster Control umjesto tastera Cmd): Prikazuje tekst bilješke na trenutnom položaju.
* `Shift+K`: Prethodna poveznica.
* `K`: Sljedeća poveznica.
* `Shift+G`: Prethodna slika.
* `G`: Sljedeća slika.
* `Shift+F`: Prethodna figura.
* `F`: Sljedeća figura.
* `Shift+T`: Prethodna tabela.
* `T`: Sljedeća tabela.
* `Shift+M`: Prethodna formula.
* `M`: Sljedeća formula.
* `Shift+S`: Prethodni rastavljač.
* `S`: Sljedeći rastavljač.
* `Shift+L`: Prethodni popis.
* `L`: Sljedeći popis.
* `Shift+I`: Prethodna stavka popisa.
* `I`: Sljedeća stavka popisa.
* `Shift+,`: Prelazi na početak trenutnog bloka (popisa ili tabele).
* `,`: Prelazi iza kraja trenutnog bloka (popisa ili tabele).

### Izbornik `Alati`

* `Control+W` (macOS: `RawCtrl+W`, tj. fizički taster Control umjesto tastera Cmd): Prikazuje broj riječi u trenutnom dokumentu.
* `Control+I`: Prikazuje informacije o dokumentu.
* `Control+T`: Prikazuje sadržaj.
* `F7`: Prikazuje popis elemenata.
* `Control+Shift+C`: Otvara mapu u kojoj se nalazi trenutna datoteka.
* `Control+Shift+V`: Otvara trenutni sadržaj u web prikazu.
* `Control+U`: Prikazuje izvorni sadržaj dokumenta u novoj kartici.
* `Control+Shift+E`: Izvozi podatke dokumenta (`.paperback`).
* `Control+Shift+I`: Uvozi podatke dokumenta (`.paperback`).
* `Control+E`: Izvozi trenutni dokument kao običan tekst.
* `Control+Shift+B`: Dodaje i uklanja oznaku na trenutnom odabiru ili položaju.
* `Control+Shift+N`: Dodaje ili uređuje bilješku oznake na trenutnom odabiru ili položaju.
* `Control+Alt+W`: Uključuje i isključuje prelamanje riječi.
* `Control+Razmak` (macOS: `RawCtrl+Razmak`, tj. fizički taster Control, jer `Cmd+Razmak` otvara Spotlight): Pokreće ili pauzira zvučnu naraciju.
* `'`: Premotava zvučnu naraciju naprijed.
* `;`: Premotava zvučnu naraciju nazad.
* `Shift+'`: Povećava korak premotavanja.
* `Shift+;`: Smanjuje korak premotavanja.
* `Control+Shift+.`: Ubrzava zvučnu naraciju.
* `Control+Shift+,`: Usporava zvučnu naraciju.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tj. Control+Command+F): Uključuje i isključuje prikaz preko cijelog zaslona.
* `Control+,`: Otvara postavke (na macOS-u se nalaze u izborniku aplikacije).
* `Control+Shift+S`: Uključuje i isključuje odbrojavanje za spavanje.
* `Control+Shift+O`: Prepoznaje raspon skeniranih PDF stranica pomoću OCR-a.
* `Alt+F9` (macOS: `Cmd+F9`): Označava početak odabira, tako da sve od tog mjesta do mjesta do kojeg stignete možete kopirati odjednom.
* `Alt+F10` (macOS: `Cmd+F10`): Kopira sve od označenog početka odabira do trenutnog položaja.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Vraća vas na označeni početak odabira, a označeni početak ostaje sačuvan.

### Izbornik `Pomoć`

* `Control+F1`: Prikazuje dijalog "O Paperbacku".
* `F1`: Prikazuje dokumentaciju u zadanom pretraživaču.
* `Shift+F1`: Prikazuje dokumentaciju u Paperbacku.
* `Control+Shift+U`: Provjerava ima li ažuriranja.
* `Control+D`: Otvara stranicu za donacije u zadanom pretraživaču.

### Dodatne prečice

* `Delete` / `Numpad Delete` na kontroli kartica: Zatvara karticu odabranog dokumenta.
* Od `Control+1` do `Control+9` (macOS: od `Cmd+1` do `Cmd+9`) u tekstu dokumenta ili na kontroli kartica: Prelazi na jedan od prvih devet otvorenih dokumenata, redoslijedom kojim su otvoreni.
* `Enter` ili `Razmak` u dokumentu: Otvara poveznicu na položaju kursora ili otvara prikaz tabele ili formule.
* `Enter` na skeniranoj PDF stranici: Prepoznaje stranicu pomoću OCR-a.
* `Shift+F10` ili taster `Aplikacije` u dokumentu: Otvara kontekstni izbornik.

## iOS i Android

Aplikacije za iOS i Android koriste isti mehanizam za čitanje kao i verzija za računare, pa otvaraju iste formate i na isti način pamte gdje ste stali. Napravljene su za korištenje s VoiceOverom na iOS-u i TalkBackom na Androidu.

### Otvaranje dokumenata

* Koristite dugme "Otvori knjigu" ili otvorite dokument iz aplikacije Files (Datoteke) ili neke druge aplikacije i odaberite Paperback.
* Na Androidu umjesto toga u postavkama možete uključiti ugrađeni pretraživač datoteka. Za njega je potrebna dozvola za pristup svim datotekama, a velike datoteke otvara odmah, bez prethodnog kopiranja.
* Dugo pritisnite dugme "Otvori knjigu" da biste uvezli ili izvezli podatke dokumenta (`.paperback`). To su iste datoteke koje koristi i verzija za računare.

### Čitanje i slušanje

Svaka aplikacija nudi dva načina čitanja dokumenta. U tekstualnom načinu rada tekst čitate pomoću čitača zaslona. U načinu čitanja naglas Paperback vam čita tekst glasom koji odaberete u postavkama i nastavlja čitati u pozadini i na zaključanom zaslonu. Između ta dva načina rada prebacujete se iz izbornika "Više opcija".

Zvučne knjige, poput DAISY, M4B i MP3 knjiga, umjesto toga reprodukuju vlastiti snimak.

### Traka za čitanje

Traka na dnu zaslona sadrži, s lijeva na desno:

* Jedinicu kretanja, npr. odlomak, naslov, stranicu ili poveznicu. Kliznite prema gore ili dolje po njoj da biste je promijenili.
* Dugmad za prethodno, reprodukciju i sljedeće. Dugmad za prethodno i sljedeće pomjeraju vas za jednu jedinicu kretanja.
* Brzinu govora. Kliznite prema gore ili dolje po njoj da biste promijenili koliko brzo Paperback čita.

Također možete kliznuti prema gore ili dolje po dugmetu za reprodukciju da biste se pomjerili za jednu jedinicu kretanja, bez posezanja za dugmadi za prethodno i sljedeće. Ako koristite samo to, postavka "Sakrij dugmad za prethodno i sljedeće" uklanja ih kako ne bi smetala čitaču zaslona. Postavka "Klizanje prema gore pomjera naprijed" određuje u kojem smjeru klizanje pomjera.

### Više opcija

U izborniku "Više opcija" nalazi se sve ostalo. Neke stavke rade malo drugačije u svakoj od aplikacija.

* **Prebaci na čitanje naglas ili Prebaci na tekstualni način:** prebacuje između načina čitanja naglas i tekstualnog načina rada, opisanih iznad. U tekstualnom načinu rada stavka "Čitaj naglas" pokreće i pauzira čitanje naglas bez napuštanja tekstualnog načina rada.
* **Sadržaj:** poglavlja knjige, otvorena na poglavlju koje trenutno čitate. Odaberite poglavlje da biste odmah prešli na njega. Stavke koje sadrže podređena poglavlja mogu se proširiti i sažeti pomoću radnji čitača zaslona.
* **Elementi:** popis naslova ili poveznica u dokumentu. Između njih se prebacujete biračem "Vrsta" na iOS-u ili karticama na Androidu, a zatim odaberete stavku da biste prešli na nju.
* **Traži:** upišite šta tražite ili odaberite raniju pretragu iz historije pretraživanja te odaberite želite li razlikovati velika i mala slova, tražiti samo cijele riječi ili koristiti regularni izraz. "Traži prethodno" i "Traži sljedeće" prelaze na podudaranje i izgovaraju gdje ste se našli, a pretraga ostaje otvorena kako biste mogli nastaviti. U načinu čitanja naglas traženje se pojavljuje i kao jedinica kretanja na traci za čitanje, pa se i odatle možete kretati kroz podudaranja.
* **Idi na:** prelazi na red, stranicu ili postotak dokumenta. Šta od toga želite, birate biračem "Način".
* **Nedavni dokumenti:** svi dokumenti koje ste otvorili, pri čemu je svaki označen kao trenutno otvoren, zatvoren ili kao dokument čija datoteka nedostaje. Svaki od njih ima dvije radnje čitača zaslona: "Ukloni" ga uklanja s popisa, a "Pronađi" vam omogućava da pronađete dokument čija je datoteka premještena. "Očisti nedavne dokumente" prazni popis bez brisanja ijednog dokumenta.
* **Broj riječi:** broj riječi u dokumentu.
* **Informacije o dokumentu:** naslov, autor, naziv datoteke, a na iOS-u i broj redova i znakova.
* **Izvoz:** sprema dokument kao običan tekst, HTML ili Markdown.
* **Odbrojavanje za spavanje:** zaustavlja čitanje nakon 5, 10, 15, 30, 45 ili 60 minuta ili nakon vremena koje sami odredite. Ako ga ponovo otvorite dok radi, možete vidjeti koliko je vremena preostalo ili ga otkazati.
* **Pomoć:** otvara ovaj dokument s uputama.
* **Postavke:**
    * **Tekst u govor:** glas, brzina govora i visina glasa, dugme "Reprodukuj uzorak" za preslušavanje te pauza između odlomaka. Na Androidu možete odabrati i govorni mehanizam. Na iOS-u se ovdje nalazi i govorni rječnik: pravila koja mijenjaju način na koji se riječi izgovaraju, za sve glasove ili samo za neke.
    * **Čitljivost:** veličina teksta, prored, razmak između odlomaka, poravnanje i tekst visokog kontrasta. Na iOS-u možete birati i između svijetlog i tamnog izgleda.
    * **Ponašanje:** da li se vaši dokumenti ponovo otvaraju pri pokretanju aplikacije, u kojem smjeru klizanje po dugmetu za reprodukciju pomjera i da li se dugmad za prethodno i sljedeće skrivaju. Na Androidu se ovdje nalazi i ugrađeni pretraživač datoteka.

### Tastature i slušalice

Uz tastaturu rade sve prečice iz verzije za računare za otvaranje knjiga, nedavne dokumente, traženje, dijalog "Idi na", sadržaj, broj riječi, informacije o dokumentu, izvoz i odbrojavanje za spavanje, s tim da se na iOS-u umjesto `Control` koristi `Cmd`. Isto vrijedi i za tastere s jednim slovom za kretanje po naslovima, stranicama, poveznicama i ostalim elementima, a `Razmak` pokreće i pauzira reprodukciju. Na iOS-u tasteri s jednim slovom stižu do Paperbacka samo kada je VoiceOverova funkcija brzog kretanja jednim slovom (Quick Nav) isključena.

Na Androidu dugme na slušalicama jednim pritiskom pokreće i pauzira reprodukciju, s dva pritiska ide naprijed, a s tri pritiska vraća nazad.

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
* Ukrajinski
* Vijetnamski

## Zasluge

### Razvoj

* Quin Gillespie: glavni programer i osnivač projekta.
* Aryan Choudhary: glavni saradnik.

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
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Dnevnik promjena

### Verzija 1.0

Verzija 1.0 je prvo izdanje dostupno na svih pet platformi: Windowsu, macOS-u, Linuxu, iOS-u i Androidu, a aplikacije za iOS i Android nalaze se na App Storeu i Google Playu.

#### Dodano

##### Općenito
* Podrška za Linux, u obliku AppImage ili tar.gz paketa, uz integraciju s radnim okruženjem, tako da se dokumenti otvaraju iz upravitelja datoteka.
* Početak odabira možete označiti pomoću `Alt+F9`, sve od tog mjesta do mjesta do kojeg ste stigli kopirati pomoću `Alt+F10`, a na označeni početak vratiti se pomoću `Alt+Shift+F9`. Tako dug dio teksta možete kopirati bez označavanja strelicama uz `Shift`. Sve tri komande nalaze se u izborniku "Alati" > "Odabir i kopiranje".
* Prečica `=` sada uz postotak izgovara i stranicu, npr. "15%, stranica 30", a kod dokumenata bez brojeva stranica radi kao i prije.
* Dijalog "O Paperbacku" sada prikazuje licencu Paperbacka i sve prevoditelje.
* Prijevod na ukrajinski jezik.

##### Novi formati
* Arhive stripova (`.cbz`).
* M4B zvučne knjige, podijeljene na poglavlja.
* Stranice priručnika, u formatu `man` i BSD `mdoc`, komprimovane gzipom ili ne.
* MP3 zvučne knjige, podijeljene na poglavlja kada ih datoteka sadrži.
* reStructuredText dokumenti.
* Windows Write (`.wri`) datoteke.
* WinHelp (`.hlp`) datoteke.
* Word 6 i Word 95 dokumenti.

##### OCR
* Skenirane PDF stranice sada se mogu prepoznati pomoću OCR-a ugrađenog u Windows i macOS. Pritisnite `Enter` na skeniranoj stranici da biste je prepoznali ili za raspon stranica koristite skupni OCR (`Control+Shift+O`).

##### Kretanje
* MathML formule u EPUB i HTML dokumentima prikazuju se kao AsciiMath pomoću alata MathCAT. Koristite `M` ili `Shift+M` za kretanje po formulama, a zatim `Enter` ili `Razmak` da biste izvorni MathML otvorili u prikazu formule.
* Dugme "Traži sve" u dijalogu "Traži", koje prikazuje popis svih redova s podudaranjem, tako da možete odmah preći na onaj koji želite.
* Prikazi tabela, popisa i stranica u popisu elemenata (`F7`).
* Dijalozi "Idi na red", "Idi na stranicu" i "Idi na postotak" sada prihvataju `+n` i `-n` za pomjeranje u odnosu na trenutni položaj.
* EPUB, MOBI i CHM knjige koje nemaju vlastite naslove sada dobijaju kretanje po naslovima na osnovu sadržaja.
* KF8 (AZW3) knjige sada podržavaju kretanje po odjeljcima.
* EPUB stranice koje sadrže samo sliku sada za nju prikazuju jedan red, tako da se možete zaustaviti na njima umjesto da ih odmah preskočite.

##### Zvučne knjige
* Kontrole brzine reprodukcije, od upola sporije do tri puta brže. Koristite `Control+Shift+.` i `Control+Shift+,` ili izbornik "Alati".
* Oznake i bilješke u knjigama koje sadrže samo zvuk sada pamte tačno vrijeme u kojem ste ih postavili.
* Sljedeći i prethodni položaj (`Alt+Strelica lijevo` i `Alt+Strelica desno`) sada rade i u zvučnim knjigama.
* Napredak kroz zvučnu knjigu sada se mjeri prema njenom snimku, pa "Idi na postotak" i statusna traka odgovaraju onome koliko ste zaista odmakli.

##### Nedavni dokumenti
* Stavka "Očisti nedavne dokumente" u podizborniku "Nedavni dokumenti".

##### PDF dokumenti
* Postavka koja svaki red PDF dokumenta zadržava kao zaseban red, umjesto da redove spaja u odlomke.
* Slike i figure u PDF dokumentima sada se izgovaraju.
* PDF dokumenti koji sadrže strukturu za čitanje, ali nisu označili nijednu svoju sliku, sada izgovaraju te slike umjesto da ih potpuno izostave iz knjige.

##### Web prikaz
* Svaki dokument sada se može otvoriti u web prikazu, a ne samo EPUB, HTML i Markdown dokumenti.

##### Čitljivost
* Naslovi se sada prikazuju veličinom koja odgovara njihovom nivou, a slike i tabele su odvojene od okolnog teksta.

##### pb
* `pb --list-formats` prikazuje sve formate koje pb može pročitati.
* pb sada navodi koju datoteku nije mogao pročitati i zašto.

#### Ispravljeno

##### Općenito
* Knjiga koja se ponovo otvori pri pokretanju sada se odmah čita, umjesto da ostane prazna dok je ne zatvorite i ponovo otvorite.
* Dokument čija datoteka nedostaje sada se može ukloniti iz dijaloga "Svi dokumenti", umjesto da ostaje na popisu koliko god puta potvrdili uklanjanje.
* Ispravljeno je rušenje programa prilikom zatvaranja Paperbacka.
* Zatvaranje Paperbacka sada odmah skriva prozor, umjesto da ga ostavlja na zaslonu dok sprema podatke.
* Velike knjige s malo oblikovanja sada se otvaraju otprilike dvostruko brže.
* Poruke pokrenute iz izbornika, poput "Ovaj dokument nema zvučni zapis", čitač zaslona više ne prekida prije nego što ih čujete.
* Otvaranje dokumenta više ne ostavlja stavku "Ponovo otvori posljednji zatvoreni" omogućenom kada nema šta ponovo otvoriti.
* Paperback više ne pokušava iznova otvarati nestale dokumente s popisa nedavnih dokumenata i ograničava broj nedavnih dokumenata koje sprema.
* Stara INI datoteka s postavkama sada se briše nakon što se prenese u novi format.
* Naslovi dijaloga za font i boju, kao i izbornik "Izvezi kao" na vijetnamskom jeziku, sada su prevedeni.
* Nakon ažuriranja ponovo pokrenuti prozor sada dolazi u prvi plan, umjesto da ostane iza svih ostalih prozora u `Alt+Tab` popisu.
* Prelamanje riječi sada se odmah primjenjuje na velike dokumente, bez ponovnog učitavanja cijelog dokumenta.

##### Kretanje
* `Alt+Strelica lijevo` sada vas vraća na mjesto s kojeg ste skočili, a ne na neki stariji položaj.
* Zvučni signali oznaka sada se reprodukuju samo kada pređete preko oznake, a ne kada dođete na red na kojem se ona nalazi.
* Zatvaranje sadržaja, popisa elemenata i dijaloga za prelaženje sada vas odmah vodi na red na koji ste došli, umjesto da morate slušati kako čitač zaslona ponovo čita cijeli prozor.
* "Idi na red", "Idi na stranicu" i "Idi na postotak" sada odbijaju brojeve izvan dokumenta, umjesto da vas bez upozorenja odvedu negdje drugdje.
* NVDA više ne prekida obavještenje kada dokument nema stranica.
* Pritiskanje dugmeta "U redu" u sadržaju bez pomjeranja sada vodi na stavku koja je već bila odabrana.
* Sadržaj, popis elemenata i popis oznaka više ne usporavaju niti se zamrzavaju u knjigama s hiljadama stavki.
* Strelice gore i dolje sada pamte kolonu kursora za svaki dokument posebno, umjesto da je prenose kada promijenite karticu.

##### Zvučne knjige
* Reprodukcija zvuka na macOS-u sada koristi `Control+Razmak`, jer `Command+Razmak` pripada Spotlightu.

##### PDF dokumenti
* Ispravljena je greška zbog koje su se PDF dokumenti izvezeni iz Apple Pagesa čitali kao običan tekst, bez naslova i popisa s kojima su napisani.
* Ispravljena je greška zbog koje su se odlomci i naslovi u PDF dokumentima dijelili na svakom redu, a riječi razdvajale na razmacima.
* Ispravljena je greška zbog koje su se numerisani naslovi u PDF dokumentima spajali u jedan naslov.
* Ispravljena je greška zbog koje su se PDF dokumenti čije stablo strukture ne vodi ni do kakvog teksta otvarali prazni.
* Redovi napisani fontom fiksne širine, poput koda, više se ne spajaju u odlomke.
* Zaglavlja i podnožja stranica više se ne čitaju na svakoj stranici neoznačenih PDF dokumenata.
* PDF dokumenti koji zaglavlja i podnožja svojih stranica označavaju kao običan tekst više ne ponavljaju naslov i broj stranice između dva odlomka na svakoj stranici.
* PDF dokumenti sada prikazuju svoj stvarni naslov, a ne naziv datoteke.

##### MOBI/AZW3 knjige
* Velikim MOBI knjigama više ne ponestaje memorije i više se ne prekidaju nakon 20 MB.
* MOBI i AZW3 knjige sada se otvaraju mnogo brže.
* Ispravljena je greška zbog koje su MOBI knjige gubile popis poglavlja.
* Ispravljen je izobličen tekst na mjestima gdje MOBI knjige prelaze iz jednog zapisa u sljedeći.

##### Web prikaz
* Web prikaz više ne učitava odjednom cijelu ogromnu knjigu.
* Web prikaz sada prikazuje dokumente u cijelosti kada ih i čitač prikazuje u cijelosti, a ne samo njihov dio.

##### Ostali formati
* FictionBook (.fb2) knjige napisane u kodiranju windows-1251, a takva je većina njih, sada se otvaraju, umjesto da ih uopšte nije moguće pročitati.
* FictionBook knjige koje koriste imenski prostor ili HTML entitet koji nikada nisu deklarisale sada se otvaraju, umjesto da budu odbijene kao oštećene.
* Knjige u starijim kodiranjima znakova sada se otvaraju mnogo brže.
* Ispravljena je greška zbog koje su se pojedine kineske tekstualne datoteke otvarale kao izobličen tekst.
* OpenDocument datoteke zaštićene lozinkom sada traže lozinku, umjesto da budu prijavljene kao oštećene.
* Starije PowerPoint datoteke zaštićene lozinkom sada se otvaraju, a slajdovi starijih PowerPoint prezentacija više ne gube tekst.
* Tekstualne datoteke spremljene s ekstenzijom `.rtf` sada se otvaraju kao tekst, umjesto da otvaranje ne uspije uz grešku.
* RTF kontrolne riječi više se ne prikazuju kao tekst.

#### iOS i Android

Aplikacije za iOS i Android otvaraju sve formate koje otvara i verzija za računare, a uključuju:

* Čitanje naglas, s odabirom glasa, brzine i visine glasa, kontrolom brzine govora direktno na traci za čitanje i opcionalnom pauzom između odlomaka.
* Reprodukciju DAISY, M4B i MP3 zvučnih knjiga, koja se nastavlja u pozadini i na zaključanom zaslonu.
* Kretanje po naslovima, stranicama, poveznicama, tabelama, popisima i drugim elementima s trake za čitanje, uz sadržaj i traženje.
* Odbrojavanje za spavanje, broj riječi i izvoz dokumenata, a na iOS-u i govorni rječnik. Na iOS-u izvoz ide kroz izbornik za dijeljenje, pa knjigu možete poslati u drugu aplikaciju ili u aplikaciju Files, u drugom formatu ili u izvornom obliku.
* Opcije za veličinu teksta, razmake i tekst visokog kontrasta.
* Prečice na tastaturi koje odgovaraju onima u verziji za računare.

### Verzija 0.9.2
* Zvučne knjige više ne tjeraju čitač zaslona da izgovara niz razmaka kada fokusirate tekstualno polje.
* U zvučnim knjigama se sada izgovara naziv datoteke dok se krećete kroz njih po odjeljcima.
* Zvučne knjige sada prijavljuju svoju stvarnu dužinu, umjesto da za svaku datoteku tvrde da traje 24 sata.
* Zatvaranje web prikaza tasterom `Escape` više ne prikazuje upozorenje za otklanjanje grešaka nakon što ste u njemu otvorili poveznicu.
* Kopiranje nakon odabira cijelog teksta sada kopira cijeli dokument, a ne samo njegov trenutno učitani dio.
* Traženje sada odmah prelazi na pronađeni red, umjesto da morate slušati kako čitač zaslona ponovo čita prozor dok se fokus vraća na knjigu.
* Ispravljena je greška zbog koje se EPUB knjige sa zalutalim ZIP64 blokom nisu otvarale, uz poruku "Invalid local file header".
* Ispravljena je greška zbog koje su se dugi dokumenti vraćali prema početku dok ih je čitač zaslona neprekidno čitao.
* Poveznice u web prikazu sada vas vode na odjeljak na koji upućuju, umjesto da ne uspiju uz poruku "File not found".
* Automatsko obavještenje "Dokument je ponovo učitan" više ne prekida čitač zaslona usred rečenice, već čeka da završi ono što govori.
* Na kartici "Opće" u dijalogu postavki tasterom `Tab` sada se kroz opcije prolazi redoslijedom kojim su prikazane na zaslonu, a kanal ažuriranja nalazi se odmah iza opcije za provjeru ažuriranja.
* Windows sada u izborniku "Otvori pomoću" uvijek prikazuje "Paperback", a ne puni opis programa.
* "Broj riječi" i "Informacije o dokumentu" sada prikazuju koliko zvučna knjiga sadrži datoteka i koliko ukupno traje.

### Verzija 0.9.1
* Zvučni signali oznaka i bilješki sada se reprodukuju na macOS-u.
* DAISY knjige sada reprodukuju zvuk na macOS-u, umjesto da se otvore i nijemo prate svoju vremensku liniju.
* Ispravljena je greška zbog koje su zaobljeni navodnici, duge crtice i slični znakovi nestajali iz RTF dokumenata, a okolne riječi se spajale.
* Ispravljena je greška zbog koje su sirovi podaci slika iz RTF dokumenata ulazili u dokument kao izobličen tekst.
* Ispravljena je greška zbog koje je podizbornik "Nedavni dokumenti" zadržavao zastarjele stavke sve dok ga nešto drugo ne bi ponovo izgradilo.
* Tasteri za brzi pristup izbornicima vraćeni su u sve prijevode, pa se ruskim izbornicima ponovo može pristupiti s tastature.
* Veliki CHM dokumenti sada se otvaraju i do sedam puta brže.
* Otvoreni dokumenti sada se registruju u Windowsu, pa se pojavljuju na popisu za brzi pristup na traci zadataka i na popisu nedavnih stavki u izborniku Start.
* "Opcije" su preimenovane u "Postavke", u skladu s mobilnim aplikacijama i, na macOS-u, s konvencijom te platforme.
* Paperback sada između pokretanja pamti položaj i veličinu prozora te da li je prozor bio maksimizovan.
* Oblici množine sada se prevode, pa se poruke koje nešto broje ispravno prikazuju u jezicima kojima je potrebno više od jednog oblika.
* Odabir datoteke ncc.html DAISY knjige sada otvara cijelu zvučnu knjigu, a ne samo njen tekst.
* Nazivi radnji u dijalogu "Prilagođavanje prečica na tastaturi" sada se mogu prevesti.
* Naslov dokumenta sada se nalazi na početku naslovne trake, pa se otvorene knjige mogu razlikovati na traci zadataka i u `Alt+Tab` popisu.
* Dijalog za ažuriranje sada je preveden.

### Verzija 0.9.0

#### Dodano

##### Općenito
* CLI alat pod nazivom pb, čiji je cilj jednostavno pretvaranje svih formata datoteka koje Paperback podržava u HTML, Markdown ili običan tekst.
* Opcija za ponovno učitavanje dokumenata koje su na disku izmijenili drugi programi.
* Opcija "Prikaži izvor" za otvaranje izvornog sadržaja dokumenta u novoj kartici, što je korisno, na primjer, prilikom uređivanja Markdown dokumenata.
* Tekst dokumenta sada se učitava po stranicama, što znači da knjige s desetinama miliona riječi sada možete učitati za svega nekoliko sekundi. Molimo prijavite svako neobično ponašanje koje primijetite.

##### Podrška za platforme
* Podrška za Windows na ARM64 procesorima!
* Izvorna podrška za macOS!
* Uključivanje i isključivanje prikaza preko cijelog zaslona.

##### Dijalog "Svi dokumenti"
* Dugme "Pronađi", koje omogućava pronalaženje knjiga koje nedostaju zbog promjene njihove putanje.
* Filter statusa i statusna traka, pomoću kojih možete filtrirati dokumente po statusu i vidjeti koliko je dokumenata prikazano i odabrano.
* Prečica `Control+Shift+A` za poništavanje odabira svih dokumenata.

##### Postavke i čitljivost
* Kartica "Čitljivost", koja sadrži sljedeće opcije:
    * Prelamanje riječi (premješteno iz opće kartice);
    * Prikazuj tabele unutar teksta (novo u ovom izdanju, pogledajte ispod);
    * Font;
    * Boja pozadine;
    * Prored;
    * Razmak između odlomaka;
    * Razmak između slova;
    * Poravnanje teksta.
* Stavka izbornika "Prelamanje riječi" i odgovarajuća prečica.
* Opcija za odabir načina prikaza tabela, a njihov prikaz je sada ujednačen u svim vrstama dokumenata.

##### Kretanje
* Podrška za kretanje po bloku sadržaja (popisu ili tabeli).
* Opcija za automatsko pomjeranje kursora na početak reda prilikom kretanja između redova, slično načinu pretraživanja u čitačima zaslona.
* Prečica `=` za izgovaranje trenutnog postotka pročitanog dokumenta.

##### Oznake
* Privremene oznake: možete imati po jednu u svakom dokumentu i one ostaju sačuvane. Pritisnite kosu crtu (`/`) da biste je postavili, a obrnutu kosu crtu (`\`) da biste prešli na nju.

##### Broj riječi
* U dijalog za broj riječi dodana je procjena vremena čitanja, kao i mogućnost podešavanja vlastite brzine čitanja kako bi ovaj podatak bio što korisniji.
* Ako je prilikom otvaranja dijaloga za brojanje riječi aktivan odabir teksta, sada će biti prikazan i broj riječi u odabranom tekstu.

##### Prečice na tastaturi
* Mogućnost prilagođavanja svake prečice u aplikaciji putem jednostavnog dijaloga.
* Promjenjiva prečica za vraćanje Paperbacka iz sistemske trake.

##### Jezici
* Finski, nizozemski i poljski.

##### Izvoz
* Opcija za izvoz je proširena i sada, pored običnog teksta, omogućava izvoz u HTML i Markdown format.

##### Program za ažuriranje
* Dugme za otkazivanje preuzimanja ažuriranja.
* Program za ažuriranje sada provjerava da preuzeta datoteka nije mijenjana.

##### Web prikaz
* Web prikaz se sada otvara na položaju čitanja.

##### DAISY knjige
* Podrška za DAISY 2.0 knjige.
* Podrška za reprodukciju zvuka u DAISY 2.02 knjigama.

##### Zvučne knjige
* Mogućnost reprodukcije zvučnih knjiga, trenutno uz podršku za DAISY knjige sa zvukom (uključujući DAISY knjige sa zvukom i tekstom) i ZIP arhive sa zvučnim datotekama.
* Prečice na tastaturi i stavke izbornika za pokretanje i pauziranje zvučne naracije, premotavanje unaprijed i unazad te podešavanje koraka premotavanja.
* Opcije za usklađivanje kursora s reprodukcijom zvuka, postavljanje koraka premotavanja i odabir da li se premotavanje preko kraja poglavlja nastavlja u sljedeće poglavlje.

##### CHM dokumenti
* Podrška za popise, stavke popisa, figure i slike.

##### PowerPoint
* PowerPoint dokumenti sada podržavaju tabele.

#### Ispravljeno

##### Općenito
* Dokumenti kodirani u starijim CJK kodiranjima, poput GBK, Big5 i Shift_JIS, sada se ispravno prikazuju, umjesto kao niz nečitljivih znakova.
* Ispravljena je greška zbog koje je stavka "Ponovo otvori posljednji zatvoreni" pokušavala ponovo otvoriti priloženu datoteku s uputama.
* Ispravljena je greška zbog koje nakon ponovnog pokretanja Paperbacka fokus nije bio ispravno postavljen na odabranu karticu.
* Poboljšano je rukovanje datotekama na mrežnim diskovima u Windowsu: opcija za prikaz datoteke u mapi sada ispravno fokusira datoteku na mrežnoj pohrani, a putanje više ne sadrže čudne znakove.
* Datoteke `.paperback` više se neće automatski učitavati prilikom vraćanja prethodne sesije. Umjesto toga, ako se takva datoteka pronađe, od vas će biti zatražena potvrda.
* Opcija "Prikaži datoteku u mapi" sada će u Windows Exploreru označiti odabranu datoteku.
* Otvaranje dokumentacije će sada poštovati vaš odabrani jezik.
* Korisničko sučelje Paperbacka sada se ispravno prilagođava zaslonima visoke rezolucije (high-DPI).
* Prilikom otvaranja dokumentacije iz Paperbacka, izbornik će se ispravno ažurirati, a fokus će biti postavljen na tekstualno polje.
* Na Windowsu je uveden znatno sigurniji način međuprocesne komunikacije (IPC).
* Naslov aktivnog dokumenta sada se izgovara prilikom prebacivanja između kartica.
* Smanjena je potrošnja memorije kod velikih dokumenata, jer su interne tabele indeksa po znakovima prepolovljene.

##### Dijalog "Svi dokumenti"
* Ispravljena je greška da taster `Escape` nije zatvarao dijaloge "Informacije o dokumentu" i "Svi dokumenti".
* Ispravljena je greška zbog koje se naslovna traka nije ažurirala nakon zatvaranja dokumenta iz dijaloga "Svi dokumenti".
* Datoteka `readme.html` se više neće dodavati na popis nedavnih dokumenata kad se ista otvara prečicom `Shift+F1`.
* Uklanjanje dokumenata iz nedavnih će sada također zatvoriti njihovu karticu.
* Filter pretrage u dijalogu "Svi dokumenti" sada će ostati spremljen nakon uklanjanja dokumenta.

##### Kretanje
* Ispravljena je greška zbog koje je kretanje po stranicama u nekim situacijama izgovaralo pogrešan tekst reda.
* Ispravljena je greška zbog koje su "Idi na red", "Idi na stranicu" i "Idi na postotak" u velikim dokumentima postavljali kursor na pogrešan položaj.
* Ispravljena je greška zbog koje "Traži" i "Traži sljedeće" u velikim dokumentima nisu uzimali u obzir trenutno učitani dio dokumenta.

##### Oznake
* Zvučni signali oznaka i bilješki sada bi se trebali reprodukovati isključivo kada pređete preko riječi koja sadrži oznaku ili bilješku.

##### Čitljivost
* Ispravljena je greška zbog koje vas je uključivanje prelamanja riječi vraćalo na početak dokumenta.

##### Web prikaz
* Omogućena je promjena veličine dijaloga web prikaza, koji se sada otvara u znatno preglednijoj veličini.
* Slike bi se sada trebale ispravno prikazivati u ugrađenom web prikazu.

##### Program za ažuriranje
* Program za ažuriranje sada ispravno prikazuje sadržaj Markdown oznaka za kod u odjeljku dnevnika promjena.

##### DAISY knjige
* Ispravljene su netačne informacije u statusnoj traci DAISY knjiga.
* Ispravljeno je učitavanje DAISY knjiga s neispravnim deklaracijama kodiranja.

##### RTF dokumenti
* Ispravljeno je obrađivanje RTF dokumenata koji sadrže znakove koji nisu latinični.
* RTF grupe `\pict` više ne propuštaju podatke ugrađenih slika u tekst dokumenta.

##### MOBI/AZW3 knjige
* Ispravljena je greška zbog koje su `filepos` sidra u MOBI knjigama razdvajala HTML oznake i ubacivala neispravan sadržaj u tekst knjige.
* Ispravljene su poveznice u starijim MOBI knjigama.
* Znatno je poboljšano obrađivanje AZW3 knjiga.

##### Word dokumenti
* Ispravljeno je prikazivanje naslova u Word dokumentima koji koriste nazive stilova specifične za određeni jezik.

##### HTML/XHTML dokumenti
* Ispravljena je greška zbog koje elementi `dl`, `dt` i `dd` nisu stvarali prijelome redova u XHTML dokumentima.

##### PDF dokumenti
* Paperback će sada za PDF dokumente koji su pogrešno označeni kao strukturisani koristiti izdvajanje običnog teksta.
* PDF dokumenti koji u naslovu i/ili oznakama sadrže kontrolne znakove više neće rušiti Paperback prilikom otvaranja.

### Verzija 0.8.5

* Dodana je podrška za stranice u EPUB knjigama.
* Dodana je podrška za šifrovane Microsoft Office dokumente. Trenutno su podržani stariji Word dokumenti, moderni Word dokumenti i moderne PowerPoint prezentacije, dok je podrška za starije PowerPoint prezentacije planirana u budućnosti.
* Dodana je podrška za starije Microsoft Word dokumente.
* Dodana je podrška za starije Microsoft PowerPoint prezentacije.
* Dodana je podrška za MOBI i AZW3 knjige.
* Dodana je podrška za označene PDF dokumente.
* Dodana je prečica `Control+Q` za izlaženje iz programa.
* Dodana je podrška za ZIP arhive knjiga preuzete sa servisa Bookshare (i u DAISY i u Word formatu).
* Alternativni tekst za ugrađene slike bi se sada trebao prikazivati.
* CHM dokumenti sada ispravno podržavaju kretanje po internim poveznicama.
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

* Dodana je prečica `Control+Shift+T` za ponovno otvaranje posljednjeg zatvorenog dokumenta.
* Dijalog "Svi dokumenti" sada podržava višestruki odabir.
* Ispravljeno je nekoliko grešaka u obrađivaču RTF dokumenata.
* Ispravljena je greška zbog koje su putanje datoteka koje sadrže znakove izvan ASCII skupa (poput bosanskih slova č, ć, š, đ i ž) postajale oštećene prilikom otvaranja datoteke putem druge instance Paperbacka.
* Ispravljeno je čitanje teksta u PDF dokumentima pogrešnim redoslijedom, kao i nepravilni razmaci oko riječi napisanih velikim slovima.
* Ispravljeno je sporo učitavanje velikih dokumenata.
* Ispravljeno je prevođenje dugmadi "Da" i "Ne" u dijalozima za potvrdu.

### Verzija 0.8.0

* Dodani su prijevodi na japanski, kineski (pojednostavljeni) i vijetnamski jezik.
* Dodan je automatski sistem ažuriranja koji će sada automatski zamijeniti trenutno instaliranu verziju Paperbacka, umjesto da samo preuzme novu verziju.
* Dodan je izborni zvučni signal pri dolasku do oznake ili bilješke. Zahvaljujemo Andreu Louisu na ustupljenim zvukovima!
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
* Podaci o verziji sada su ispravno ugrađeni u izvršnu datoteku Paperbacka.
* Dijalog postavki podijeljen je na kartice radi lakšeg korištenja.
* Za obrađivanje PDF dokumenata uvedena je biblioteka `Hayro`, što donosi veću pouzdanost, bolje performanse i manji broj potrebnih DLL datoteka.
* Cijeli program je ponovo napisan u programskom jeziku `Rust`. Novi kod je sigurniji, brže učitava dokumente te ga je lakše održavati i proširivati.
* Kontekstni izbornik tekstualnog polja sada sadrži radnje namijenjene čitaču, umjesto generičnih opcija kao što su izrezivanje i lijepljenje.

### Verzija 0.7.0

* Dodana je podrška za tabele u dokumentima zasnovanim na HTML-u i XHTML-u! Između tabela se krećete pomoću `T` i `Shift+T`, a pritiskom na `Enter` tabelu otvarate u web prikazu.
* Dodana je osnovna funkcija web prikaza! Pritisnite `Control+Shift+V` da biste trenutni odjeljak dokumenta otvorili u web prikazu, što je korisno za sadržaj poput složenog oblikovanja ili primjera koda.
* Dodan je prijevod na ruski jezik. Zahvaljujemo Ruslanu Gulmagomedovu!
* Dodano je dugme "Očisti sve" u dijalog "Svi dokumenti".
* Provjera ažuriranja sada prikazuje bilješke o izdanju kada je dostupna nova verzija.
* Ispravljeno je vraćanje prozora iz sistemske trake.
* Ispravljeno je prevođenje dugmadi "Da" i "Ne" u dijalozima za potvrdu.
* Ispravljeno je učitavanje konfiguracije kada se program pokreće kao administrator.
* Ispravljeno je rukovanje komentarima u XML i HTML dokumentima.
* Ispravljeno je obrađivanje sadržaja u EPUB 2 knjigama.
* Ispravljeno je prelaženje na sljedeću stavku koja počinje istim slovom u sadržaju.
* Ispravljena je greška zbog koje se dijalog za traženje nije ispravno skrivao prilikom korištenja dugmadi za sljedeće i prethodno.
* Ispravljena je greška zbog koje vas je sadržaj EPUB knjiga povremeno vodio na pogrešnu stavku.
* Ispravljeni su različiti problemi s rukovanjem razmacima u XML i HTML dokumentima te u oznakama `pre`.
* Ispravljena je greška pomaka za jedno mjesto pri kretanju po poveznicama.
* Ispravljena je greška zbog koje su redovi u nekim knjigama imali suvišne razmake na kraju.
* Ispravljeni su različiti problemi u obrađivačima.
* Stavke izbornika povezane s oznakama, kao i popis elemenata, sada su ispravno onemogućene kada nijedan dokument nije otvoren.
* Poboljšano je rukovanje popisima u raznim formatima dokumenata.
* Poboljšan je postupak prevođenja za saradnike.
* Brojne interne izmjene premjestile su većinu poslovne logike aplikacije iz C++-a u Rust radi boljih performansi i lakšeg održavanja.

### Verzija 0.6.1

* Dodana je podrška za PDF dokumente zaštićene lozinkom.
* Dodana je osnovna funkcija za prelaženje na prethodni ili sljedeći položaj. Ako pritisnete `Enter` na internoj poveznici i ona pomjeri kursor, taj položaj će biti zapamćen, a na njega se možete vratiti pomoću `Alt+Strelica lijevo` i `Alt+Strelica desno`.
* Dodan je popis elemenata! Trenutno prikazuje samo stablo svih naslova u dokumentu ili popis poveznica, ali se u budućnosti planira njegovo proširenje.
* Dodana je opcija za pokretanje Paperbacka u maksimizovanom prozoru.
* Ispravljena je greška s poveznicama koje u pojedinim EPUB dokumentima nisu ispravno radile.
* Ispravljeno je obrađivanje sadržaja u EPUB dokumentima koji sadrže relativne putanje.
* Ispravljena je greška kad se naslov ili autor u pojedinim EPUB dokumentima nije prikazivao.
* Ispravljeno je nepravilno prikazivanje naslova pojedinih poglavlja EPUB knjiga u dijalogu sadržaja.
* Ispravljena je greška zbog koje nije bilo moguće koristiti taster `Razmak` za aktiviranje dugmadi "U redu" i "Otkaži" u dijalogu sadržaja.
* Unaprijeđeno je rukovanje naslovima u Word dokumentima.
* Ako je popis nedavno otvorenih dokumenata prazan kada pokušate otvoriti odgovarajući dijalog, sada će se to izgovoriti.

### Verzija 0.6.0

* Dodana je nova opcija u postavke koja omogućava prikaz izbornika "Idi" u znatno sažetijem obliku. Ova opcija je podrazumijevano uključena.
* Dodana je opcija koja omogućava kružno kretanje prilikom navigacije po strukturnim elementima.
* U izbornik "Alati" dodana je opcija za otvaranje mape u kojoj se nalazi trenutno aktivni dokument.
* Dodan je jednostavan, ali vrlo efikasan sistem za ažuriranje.
* Dodana je osnovna funkcija odbrojavanja za spavanje, kojoj možete pristupiti pomoću `Control+Shift+S`.
* Dodana je podrška za FB2 elektronske knjige.
* Dodana je podrška za OpenDocument prezentacije.
* Dodana je podrška za OpenDocument tekstualne dokumente.
* Oznake sada mogu obuhvatati cijeli red ili samo odabrani tekst. Ako prilikom postavljanja oznake nemate aktivan odabir, ponašanje je isto kao prije verzije `0.6.0` i označit će se cijeli red. Ako je dio teksta označen, oznaka će obuhvatiti samo taj tekst.
* Oznakama se sada mogu dodati i bilješke. Između oznaka koje sadrže bilješke možete se kretati pomoću `N` i `Shift+N`, a dijalog s oznakama možete otvoriti tako da prikazuje sve oznake, samo one s bilješkama ili samo one bez bilješki, koristeći odgovarajuće prečice.
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
* Sada je moguće uvesti i izvesti oznake i položaj čitanja za određeni dokument. Stvorena datoteka nosi isti naziv kao dokument, ali s ekstenzijom `.paperback`. Ako se takva datoteka nalazi u istoj mapi kao dokument prilikom njegovog otvaranja, automatski će biti učitana. U suprotnom, možete je ručno uvesti putem opcije u izborniku "Alati".
* Poveznice unutar dokumenata sada su u potpunosti podržane. Koristite `K` i `Shift+K` za kretanje između njih, a `Enter` za otvaranje ili aktiviranje poveznice.
* Brojne interne izmjene učinile su program bržim, a izvršnu datoteku manjom.
* Markdown sadržaj sada se prije prikazivanja obrađuje kako bi bio usklađen sa CommonMark standardom.
* Kretanje po popisima i njihovim stavkama sada je u potpunosti podržano. Koristite `L` i `Shift+L` za kretanje po popisima, a `I` i `Shift+I` za kretanje po stavkama popisa.
* Taster `Delete` na numeričkoj tastaturi sada također uklanja dokumente s trake kartica, kao i standardni taster `Delete`.
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
* Ispravljen je sadržaj u EPUB knjigama s URL kodiranim putanjama datoteka i/ili identifikatorima fragmenata.
* Ispravljeno je nepravilno uklanjanje razmaka iz XHTML naslova u određenim slučajevima.
* Ispravljeno je rukovanje razmacima unutar ugniježđenih oznaka `pre` u HTML dokumentima.
* HTML i Markdown dokumenti sada podržavaju funkciju sadržaja! Prilikom otvaranja HTML ili Markdown dokumenta, Paperback će na osnovu strukture naslova u dokumentu automatski napraviti sadržaj i prikazati ga u dijalogu `Control+T`.
* HTML dokumenti sada će koristiti naslov definisan u oznaci `title`, ako postoji. U suprotnom će, kao i do sada, koristiti naziv datoteke bez ekstenzije.
* UniversalSpeech je zamijenjen live regionom za prijavljivanje govora. Zbog toga se DLL datoteke čitača zaslona više ne isporučuju uz program, a podržan je i veći broj čitača zaslona, uključujući Microsoft Narrator.
* Biblioteka za rad sa ZIP datotekama je zamijenjena kako bi bilo moguće otvoriti veći broj EPUB knjiga.
* Dijalog koji vas pita želite li otvoriti dokument kao običan tekst potpuno je redizajnovan i sada omogućava otvaranje dokumenta kao običnog teksta, HTML ili Markdown dokumenta.
* Dijalog "Idi na postotak" sada sadrži i tekstualno polje koje omogućava ručni unos postotka na koji želite otići.
* HTML obrađivač sada prepoznaje oznake `dd`, `dt` i `dl` kao elemente popisa.
* Sadržaj u EPUB knjigama ponovo će biti sačuvan tačno onako kako je definisan.
* Unicode znak za nerazdvojivi razmak sada se uzima u obzir prilikom uklanjanja praznih redova.
* Prilikom otvaranja nepoznate datoteke više nećete svaki put biti pitani kako je želite otvoriti, već samo prvi put.

### Verzija 0.4.1

* Instalacijski program sada nudi mogućnost dodavanja prečice u Start izbornik.
* Sadržaj će sada u pojedinim slučajevima biti pregledniji. Na primjer, ako nadređena i podređena stavka imaju isti tekst na istom položaju, prikazat će se samo nadređena stavka.
* Ispravljen je sadržaj u određenim CHM dokumentima.
* Ispravljen je sadržaj u EPUB 3 knjigama koje sadrže apsolutne putanje.
* CHM dokumenti sada će prikazivati naslov definisan u datoteci s metapodacima.

### Verzija 0.4.0

* Dodana je podrška za CHM dokumente.
* Dodana je podrška za oznake! Možete imati neograničen broj oznaka u neograničenom broju dokumenata. Krećite se između njih naprijed i nazad pomoću `B` i `Shift+B`, postavite novu oznaku pomoću `Control+Shift+B`, a dijalog za prelazak na određenu oznaku otvorite pomoću `Control+B`.
* Dodan je instalacijski program uz prenosivu ZIP verziju! Instalacijski program će instalirati Paperback u mapu `Program Files` i automatski postaviti pridruživanje datoteka.
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
* Dijalog za traženje je potpuno prerađen kako bi bio mnogo jednostavniji za korištenje, a uz to su dodani historija pretraživanja (uključuje 25 posljednjih pretraga) i podrška za regularne izraze.
* Prethodno otvoreni dokumenti sada ostaju zapamćeni i nakon ponovnog pokretanja programa. To se može podesiti putem nove opcije u izborniku "Alati".
* Dodana je prečica `Shift+F1` za otvaranje dokumentacije direktno u Paperbacku.

### Verzija 0.1.0

* Prvo izdanje.
