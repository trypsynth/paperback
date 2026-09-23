# Paperback - versio 1.0

## Johdanto

Paperback on kevyt ja saavutettava e- ja asiakirjojen luku- ja äänikirjojen kuuntelusovellus kaikille satunnaisista lukijoista vaativiin tehokäyttäjiin. Se on suunniteltu ruudunlukijaystävällisyyttä, suorituskykyä ja turhista ominaisuuksista riisuttua käyttökokemusta ajatellen.

## Järjestelmävaatimukset

Paperback toimii Windows 10:ssä ja 11:ssä, kaikissa nykyaikaisissa ARM-pohjaisissa macOS:n versioissa, Linuxissa, iOS 17:ssä ja sitä uudemmissa sekä Android 7:ssä ja sitä uudemmissa versioissa. iOS- ja Android-sovellukset ovat saatavilla App Storesta ja Google Playsta.

## Ominaisuudet

* Toimii täysin itsenäisesti ilman kolmannen osapuolen ohjelmistojen asentamista.
* Toimii erittäin nopeasti myös vanhalla laitteistolla.
* Yksinkertainen välilehtikäyttöliittymä, jonka avulla voit avata rajattoman määrän asiakirjoja.
* Tallentaa tarkan lukukohdan jokaisessa avaamassasi asiakirjassa.
* Muistaa valinnaisesti, mitkä asiakirjat olivat avoinna ohjelmaa suljettaessa, ja avaa ne seuraavalla käynnistyskerralla.
* Ruudunlukijoista tuttu verkkoselaustilaa muistuttava navigointitoiminto, jonka avulla voit liikkua asiakirjoissa nopeasti ja vaivattomasti.
* Tehokas tekstin etsimisen valintaikkuna, joka tukee hakuhistoriaa ja säännöllisiä lausekkeita.
* Voidaan käyttää massamuistiversiona tai asentaa siten, että tiedostokytkennät määritetään automaattisesti.
Tukee erittäin kattavasti yleisiä tiedostomuotoja.
* Toistaa äänikirjoja ja tukee niiden toistonopeuden muuttamista  sekä tarkan kohdan muistavia kirjanmerkkejä.
* Mahdollistaa skannattujen PDF-asiakirjojen lukemisen Windowsin ja macOS:n tekstintunnistusominaisuuden avulla.
* Kirjanmerkit ja muistiinpanot lukukohdan merkitsemistä ja siihen palaamista varten.
* Kaikkia pikanäppäimiä on mahdollista vaihtaa.
* Mukana tulee `pb`-komentorivityökalu, jolla voi muuntaa minkä tahansa tuetun asiakirjan HTML-, Markdown- tai tekstimuotoon.

## Ruudunlukijoiden yhteensopivuus

Paperback toimii sujuvasti kaikilla yleisimmillä ruudunlukijoilla. JAWS-käyttäjien on kuitenkin hyvä tietää  eräästä tunnetusta ongelmasta.

### JAWS ja pistenäytöt

Jos käytät JAWS-ruudunlukijaa ja pistenäyttöä, pitkät kappaleet saattavat katketa, kun tekstiä vieritetään eteenpäin näytön navigointinäppäimillä. Ongelma koskee myös nykyisen kappaleen lukukomentoa. Tämä johtuu JAWSin virheestä RICHEDIT50W-tekstikentän käsittelyssä, eli vika ei ole Paperbackissa. Ratkaisun löytäminen kesti kauan, koska Visperolta on tunnetusti vaikea saada vastauksia avoimen lähdekoodin sovellusten virheraportteihin.

Kiertotienä ongelmaan on paperback.jcf-tiedoston muokkaaminen siten, että asetuksen "Braille Presentation and Panning" (Pistekirjoitusesitys ja -vieritys) arvoksi määritetään "Always use DOM if available" (Käytä aina DOMia, jos se on saatavilla). Lisäksi asetus "Pan Text by Paragraph" (Vieritä kappaleittain) on otettava käyttöön, jotta pistenäyttö siirtyy eteenpäin seuraavaan kappaleeseen. Näillä asetuksilla vierityksen pitäisi toimia oikein.

## Tuettavat tiedostomuodot

Paperback tukee seuraavia tiedostomuotoja:

* Sarjakuvapaketit (`.cbz`)
* CHM-ohjetiedostot (`.chm`)
* DAISY-kirjat (`.opf`, `.zip`)
* EPUB-kirjat (`.epub`)
* FB2-e-kirjat (`.fb2`)
* HTML-asiakirjat (`.htm`, `.html`, `.xhtml`)
* `Man`- ja BSD `mdoc` -muodoissa olevat man-sivut (`.1`–`.9`, `.man`, `.roff` sekä niiden gzip-pakatut versiot)
* Markdown-asiakirjat (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word -asiakirjat (`.docx`, `.docm`, `.doc`)
* M4B-äänikirjat (`.m4b`)
* MOBI- ja Kindle-kirjat (`.mobi`, `.azw`, `.azw3`)
* MP3-äänikirjat (`.mp3`)
* OpenDocument-esitykset (`.odp`, `.fodp`)
* OpenDocument-tekstiasiakirjat (`.odt`, `.fodt`)
* PDF-asiakirjat (`.pdf`)
* PowerPoint-esitykset (`.pptx`, `.pptm`, `.ppt`)
* RTF-asiakirjat (`.rtf`)
* Windows Write -asiakirjat (`.wri`)
* WinHelp-ohjetiedostot (`.hlp`)
* Teksti- ja lokitiedostot (`.txt`, `.log`)

## Pikanäppäimet

Paperback on suunniteltu ensisijaisesti näppäimistöllä käytettäväksi. Alla on luettelo nykyisistä pikanäppäimistä.

Nämä pikanäppäimet toimivat Windowsissa. MacOS-komennot on merkitty sulkeisiin. Erot johtuvat siitä, että näppäinyhdistelmät Ctrl+G, Ctrl+W sekä Alt + vasen/oikea nuoli on varattu macOS-alustalla järjestelmän tai muiden sovellusten käyttöön.

### Tiedosto-valikko

* `Ctrl+O`: Avaa asiakirja.
* `Ctrl+F4` (macOS: `Cmd+W`): Sulje nykyinen asiakirja.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Sulje kaikki avoimet asiakirjat.
* `Ctrl+Shift+T`: Avaa viimeksi suljetun asiakirjan uudelleen.
* `Ctrl+R`: Näytä "Kaikki asiakirjat" -valintaikkuna (Viimeksi avatut -valikosta).
* `Ctrl+Q`: Lopeta (vain Windowsissa; macOS:ää käytettäessä tämä komento löytyy sovellusvalikosta).

### Siirry-valikko

* `Ctrl+F`: Näytä Etsi-valintaikkuna.
* `F3` (macOS: `Cmd+G`): Etsi seuraava.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Etsi edellinen.
* `Ctrl+G` (macOS: `Cmd+L`): Siirry riville.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Siirry prosenttiin.
* `Ctrl+P`: Siirry sivulle (jos asiakirja tukee sitä).
* `=`: Ilmoittaa asiakirjan lukukohdan prosentteina sekä sivunumeron (esim. "15 %, sivu 30"). Sivunumeroa ei ilmoiteta, jos asiakirjassa ei niitä ole.
* `Alt+Vasen nuoli` (macOS: `Cmd+[`): Siirry taaksepäin navigointihistoriassa.
* `Alt+Oikea nuoli` (macOS: `Cmd+]`): Siirry eteenpäin navigointihistoriassa.
* `[`: Edellinen luku.
* `]`: Seuraava luku.
* `Shift+H`: Edellinen otsikko.
* `H`: Seuraava otsikko.
* `Shift+1`–`Shift+6`: Edellinen otsikko tasoilla 1–6.
* `1`–`6`: Seuraava otsikko tasoilla 1–6.
* `Shift+P`: Edellinen sivu.
* `P`: Seuraava sivu.
* `Shift+B`: Edellinen kirjanmerkki.
* `B`: Seuraava kirjanmerkki.
* `/`: Lisää tilapäinen kirjanmerkki.
* `\`: Siirry tilapäiseen kirjanmerkkiin.
* `Shift+N`: Edellinen muistiinpano.
* `N`: Seuraava muistiinpano.
* `Ctrl+B`: Siirry kaikkiin kirjanmerkkeihin ja muistiinpanoihin.
* `Ctrl+Alt+B`: Siirry vain kirjanmerkkeihin.
* `Ctrl+Alt+M`: Siirry vain muistiinpanoihin.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W` eli fyysinen Ctrl-näppäin Cmd-näppäimen sijaan): Näytä muistiinpanon teksti nykyisessä sijainnissa.
* `Shift+K`: Edellinen linkki.
* `K`: Seuraava linkki.
* `Shift+G`: Edellinen kuva.
* `G`: Seuraava kuva.
* `Shift+F`: Edellinen kuvitus.
* `F`: Seuraava kuvitus.
* `Shift+T`: Edellinen taulukko.
* `T`: Seuraava taulukko.
* `Shift+M`: Edellinen matemaattinen kaava.
* `M`: Seuraava matemaattinen kaava.
* `Shift+S`: Edellinen erotin.
* `S`: Seuraava erotin.
* `Shift+L`: Edellinen luettelo.
* `L`: Seuraava luettelo.
* `Shift+I`: Edellinen luettelokohde.
* `I`: Seuraava luettelokohde.
* `Shift+,`: Siirry nykyisen säilön, eli luettelon tai taulukon, alkuun.
* `,`: Siirry nykyisen säilön, eli luettelon tai taulukon, jälkeiseen kohtaan.

### Työkalut-valikko

* `Ctrl+W` (macOS: `RawCtrl+W` eli fyysinen Ctrl-näppäin Cmd-näppäimen sijaan): Näytä nykyisen asiakirjan sanamäärä.
* `Ctrl+I`: Näytä asiakirjan tiedot.
* `Ctrl+T`: Näytä sisällysluettelo.
* `F7`: Näytä elementtilista.
* `Ctrl+Shift+C`: Avaa asiakirjan kansio.
* `Ctrl+Shift+V`: Avaa nykyinen sisältö selainnäkymässä.
* `Ctrl+U`: Näytä asiakirjan lähdekoodi uudessa välilehdessä.
* `Ctrl+Shift+E`: Vie asiakirjan tiedot `.paperback`-tiedostoon.
* `Ctrl+Shift+I`: Tuo asiakirjan tiedot `.paperback`-tiedostosta.
* `Ctrl+E`: Vie nykyinen asiakirja pelkkänä tekstinä.
* `Ctrl+Shift+B`: Lisää kirjanmerkki nykyisen valinnan kohdalle tai kohdistimen sijaintiin tai poista se.
* `Ctrl+Shift+N`: Lisää kirjanmerkin muistiinpano nykyisen valinnan tai kohdistimen kohdalle tai muokkaa sitä.
* `Ctrl+Alt+W`: Ota rivitys käyttöön tai poista se käytöstä.
* `Ctrl+Välilyönti` (macOS: `RawCtrl+Välilyönti` eli fyysinen Ctrl-näppäin, koska Cmd+Välilyönti avaa Spotlight-haun): Aloita tai pysäytä äänitteen toisto.
* `'`: Kelaa äänitettä eteenpäin.
* `;`: Kelaa äänitettä taaksepäin.
* `Shift+'`: Pidennä äänitteen kelauksen aikasiirtymää.
* `Shift+;`: Lyhennä äänitteen kelauksen aikasiirtymää.
* `Ctrl+Shift+.`: Nopeuta äänitteen toistoa.
* `Ctrl+Shift+,`: Hidasta äänitteen toistoa.
* `F11` (macOS: `RawCtrl+Ctrl+F` eli Ctrl+Cmd+F): Ota koko näytön tila käyttöön tai poista se käytöstä.
* `Ctrl+,`: Avaa asetukset (löytyy macOS:ää käytettäessä sovellusvalikosta).
* `Ctrl+Shift+S`: Ota uniajastin käyttöön tai poista se käytöstä.
* `Ctrl+Shift+O`: Suorita tekstintunnistus skannatun PDF-asiakirjan valituille sivuille.
* `Alt+F9` (macOS: `Cmd+F9`): Merkitse valinnan alkukohta.
* `Alt+F10` (macOS: `Cmd+F10`): Kopioi valinnan merkityn alkukohdan ja kohdistimen nykyisen sijainnin välinen teksti.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Palaa valinnan alkukohtaan.

### Ohje-valikko

* `Ctrl+F1`: Näytä Tietoa-valintaikkuna.
* `F1`: Näytä ohje oletusselaimessa.
* `Shift+F1`: Näytä ohje Paperbackissa.
* `Ctrl+Shift+U`: Tarkista päivitykset.
* `Ctrl+D`: Avaa lahjoitussivu oletusselaimessa.

### Asiakirjanäkymän lisänäppäimet

* `Delete` / `Laskinnäppäimistön Delete` välilehtien ohjausobjektissa: Sulje valittu asiakirjan välilehti.
* `Enter` tai `Välilyönti` asiakirjan tekstissä: Avaa kohdistimen kohdalla oleva linkki tai näytä taulukko tai kaava omassa näkymässään.
* `Enter` skannatun PDF-asiakirjan sivulla: Suorita sivun tekstintunnistus.
* `Shift+F10` tai sovellusnäppäin asiakirjan tekstissä: Avaa pikavalikko.

## iOS ja Android

iOS- ja Android-sovellukset käyttävät samaa lukumoottoria kuin työpöytäversio, joten ne avaavat samoja tiedostomuotoja ja muistavat lukukohdan samalla tavalla. Ne on suunniteltu käytettäviksi iOS:n VoiceOver- ja Androidin TalkBack-ruudunlukijan kanssa.

### Asiakirjojen avaaminen

* Käytä Avaa kirja -painiketta tai avaa asiakirja Tiedostot- tai muusta sovelluksesta ja valitse Paperback.
* Androidissa voit ottaa asetuksista käyttöön sovelluksen sisäisen tiedostoselaimen. Se tarvitsee kaikkien tiedostojen käyttöoikeuden, minkä jälkeen suuret tiedostot voidaan avata suoraan kopioimatta niitä ensin.
* Tuo tai vie asiakirjan tiedot (`.paperback`) painamalla pitkään Avaa kirja -painiketta. Samoja tiedostoja käytetään myös työpöytäsovelluksessa.

### Reading and listening

Asiakirjoja voi lukea sovelluksissa kahdella tavalla. Tekstitilassa teksti luetaan ruudunlukijalla. Tekstistä puheeksi -tilassa Paperback lukee tekstin asetuksista valitulla äänellä. Lukeminen jatkuu myös taustalla ja lukitusnäytössä. Tilaa voi vaihtaa Lisää vaihtoehtoja -valikosta.

DAISY-, M4B- ja MP3-äänikirjoissa toistetaan niiden omaa äänitallennetta.

### Lukupalkki

Näytön alareunassa olevassa palkissa ovat vasemmalta oikealle:

* Siirtymistapa, kuten kappale, otsikko, sivu tai linkki. Vaihda sitä pyyhkäisemällä ylös- tai alaspäin.
* Edellinen-, Toista- ja Seuraava-painikkeet. Edellinen- ja Seuraava-painikkeilla siirrytään valitun siirtymistavan mukaisesti.
* Puhenopeus. Muuta Paperbackin lukunopeutta pyyhkäisemällä ylös- tai alaspäin.

Voit siirtyä valitun siirtymistavan mukaisesti myös pyyhkäisemällä Toista-painikkeen kohdalla ylös- tai alaspäin, jolloin Edellinen- ja Seuraava-painikkeita ei tarvitse käyttää. Jos käytät vain tätä tapaa, voit piilottaa painikkeet ruudunlukijalta Piilota Edellinen- ja Seuraava-painikkeet -asetuksella. Ylöspäin pyyhkäisy siirtää eteenpäin -asetuksella valitaan, kumpaan suuntaan pyyhkäisyllä siirrytään.

### Lisää vaihtoehtoja

Muut toiminnot löytyvät Lisää vaihtoehtoja -valikosta. Osa niistä toimii sovelluksesta riippuen hieman eri tavalla.

* **Vaihda tekstistä puheeksi -tilaan tai Vaihda tekstitilaan:** vaihtaa edellä kuvatun tekstistä puheeksi -lukutilan ja tekstitilan välillä. Tekstitilassa "Lue tekstistä puheeksi -toiminnolla" -vaihtoehto aloittaa ja pysäyttää ääneenluvun poistumatta tekstitilasta.
* **Sisällysluettelo:** näyttää kirjan luvut avattuna parhaillaan luettavan luvun kohdalta. Valitse luku siirtyäksesi sen kohdalle. Alalukuja sisältävät kohdat voidaan laajentaa ja supistaa ruudunlukijan toiminnoilla.
* **Elementit:** näyttää asiakirjan otsikot ja linkit erillisissä luetteloissa. iOS:ssa luetteloa vaihdetaan Tyyppi-valitsimella ja Androidissa välilehdillä. Valitse haluamasi kohde siirtyäksesi siihen.
* **Etsi:** kirjoita etsittävä teksti tai valitse hakuhistoriasta aiempi haku ja valitse, otetaanko kirjainkoko huomioon, etsitäänkö vain kokonaisia sanoja vai käytetäänkö säännöllistä lauseketta. Etsi edellinen- ja Etsi seuraava -painikkeet siirtävät hakutuloksen kohdalle samalla kun sen sijainti ilmoitetaan. Etsi-ikkuna pysyy avoinna haun jatkamista varten. Tekstistä puheeksi -lukutilassa Etsi-toiminto näkyy myös lukupalkissa navigointiyksikkönä, joten hakutuloksia voi selata myös sitä kautta.
* **Siirry:** siirry asiakirjassa haluamallesi riville, sivulle tai tiettyyn prosenttikohtaan. Valitse haluamasi siirtymätapa Tila-valitsimella.
* **Viimeksi avatut:** näyttää kaikki avaamasi asiakirjat ja ilmoittaa kunkin kohdalla, onko se parhaillaan avoinna, suljettu tai jos sen tiedostoa ei löydy. Asiakirjoille on kaksi ruudunlukijan toimintoa: "Poista" poistaa asiakirjan luettelosta ja "Etsi"-toiminnolla voit etsiä asiakirjan tiedoston, jos se on siirretty toiseen hakemistoon. "Tyhjennä viimeksi avattujen luettelo" tyhjentää luettelon poistamatta varsinaisia asiakirjoja.
* **Sanamäärä:** näyttää asiakirjan sanojen määrän.
* **Asiakirjan tiedot:** näyttää asiakirjan otsikon, tekijän, tiedostonimen ja iOS:ssä myös rivien ja merkkien määrän.
* **Vie:** tallentaa asiakirjan teksti-, HTML- tai Markdown-muodossa.
* **Uniajastin:** pysäyttää lukemisen 5, 10, 15, 30, 45 tai 60 minuutin kuluttua tai itse valitsemanasi ajankohtana. Avaa se uudelleen ajastuksen ollessa käynnissä nähdäksesi jäljellä olevan ajan tai peruuttaaksesi sen.
* **Ohje:** avaa tämän lueminut-tiedoston.
* **Asetukset:**
    * **Tekstistä puheeksi:** ääni, nopeus ja korkeus, Toista näyte -painike arvojen vaikutuksen kuuntelemiseksi sekä kappaleiden välinen tauko. Androidissa voit valita myös puhemoottorin. iOS:ssa täältä löytyy myös käyttäjän sanasto, johon lisättävillä säännöillä on mahdollista muuttaa sanojen ääntämistä kaikilla tai vain tietyillä äänillä.
    * **Luettavuus:** tekstin koko, rivi- ja kappaleväli, tasaus sekä suurikontrastinen teksti. iOS:ssa on myös vaalea ja tumma ulkoasu.
    * **Toiminta:** avataanko asiakirjat uudelleen sovelluksen käynnistyessä, mihin suuntaan toistopainikkeen pyyhkäisy siirtää ja piilotetaanko Edellinen- ja Seuraava-painikkeet. Android-versiossa täällä on myös sovelluksen sisäinen tiedostoselain.

### Näppäimistöt ja kuulokkeet

Näppäimistöä käytettäessä työpöytäsovelluksen pikanäppäimillä voi avata kirjoja ja viimeksi avattuja asiakirjoja, käyttää Etsi- ja Siirry-toimintoja, avata sisällysluettelon, tarkistaa sanamäärän ja tarkastella asiakirjan tietoja, viedä asiakirjan eri tiedostomuotoihin sekä käyttää uniajastinta. iOS:ssa käytetään `Ctrl`-näppäimen sijasta `Cmd`-näppäintä. Myös otsikoiden, sivujen, linkkien ja muiden kohteiden välillä siirtymiseen tarkoitetut navigointikomennot toimivat, ja `Välilyönti` aloittaa ja pysäyttää toiston. iOS:ssa navigointikomennot toimivat Paperbackissa vain, kun VoiceOverin pikanavigointinäppäimet on poistettu käytöstä.

Android-laitteissa kuulokkeiden painikkeen yksi painallus aloittaa tai pysäyttää toiston, kaksi painallusta siirtää eteenpäin ja kolme painallusta taaksepäin.	

## Tuettavat kielet

Paperback on käännetty useille eri kielille, ja uusia lisätään jatkuvasti. Täydellinen luettelo on alla.

Jos haluat osallistua kääntämiseen, katso ohjeet [käännösoppaasta](translating.md).

* bosnia
* brasilianportugali
* espanja
* hollanti
* japani
* puola
* ranska
* saksa
* serbia
* suomi
* tšekki
* ukraina
* venäjä
* vietnam
* yksinkertaistettu kiina

## Tekijät
### Kehitys
* Quin Gillespie: pääkehittäjä ja projektin perustaja.
* Aryan Choudhary: pääasiallinen avustaja.

### Lahjoitukset
Seuraavat henkilöt ovat lahjoittaneet Paperbackin kehitykseen. Jos lahjoitat, nimeäsi ei lisätä tähän luetteloon automaattisesti. Vain sellaiset henkilöt lisätään, jotka haluavat lahjoituksensa julkiseksi.

Huom: julkista GitHub-sponsorointia pidetään automaattisen lisäämisen perusteena.

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

## Versiohistoria

### Versio 1.0

Tämä on ensimmäinen julkaisu kaikille viidelle alustalle: Windowsille, macOS:lle, Linuxille, iOS:lle ja Androidille. iOS- ja Android-sovellukset ovat saatavilla App Storesta ja Google Playsta.

#### Lisätty

##### Yleistä
* Linux-tuki AppImage- ja tar.gz-paketteina sekä työpöytäintegraatiolla, jonka ansiosta asiakirjat avautuvat suoraan tiedostonhallinnasta.
* Valinnan alkukohdan merkitseminen näppäinyhdistelmällä `Alt+F9`, valinnan alkukohdan ja nykyisen sijainnin välisen tekstin kopiointi näppäinyhdistelmällä `Alt+F10` ja valinnan alkukohtaan siirtyminen näppäinyhdistelmällä `Alt+Shift+F9`. Näin voit kopioida pitkän tekstijakson tarvitsematta valita sitä Shift- ja nuolinäppäimillä. Kaikki kolme toimintoa löytyvät Työkalut-valikon kohdasta Valitse ja kopioi.
* =-näppäin ilmoittaa nyt sekä prosenttiarvon että sivunumeron (esim. ”15 %, sivu 30”). Toiminto säilyy ennallaan, jos asiakirjassa ei ole sivunumeroita.
* Tietoja-ikkunassa näkyvät nyt Paperbackin lisenssi ja kaikki kääntäjät.
* Ukrainankielinen käännös.

##### Uudet tiedostomuodot
* Sarjakuvapaketit (`.cbz`).
* Lukuihin jaetut M4B-äänikirjat.
* Gzip-pakatut tai pakkaamattomat `man`- ja BSD `mdoc` -muodossa olevat man-sivut.
* Tavalliset ja lukuihin jaetut MP3-äänikirjat.
* Windows Write -tiedostot (`.wri`).
* WinHelp-ohjetiedostot (`.hlp`).
* Word 6- ja 95-asiakirjat.

##### Tekstintunnistus
* Skannatuille PDF-asiakirjojen sivuille voidaan nyt suorittaa tekstintunnistus Windowsin ja macOS:n omalla tekstintunnistustoiminnolla. Suorita tunnistus skannatulle sivulle painamalla `Enter` sen kohdalla tai käytä tietylle sivualueelle usean sivun tunnistusta (`Ctrl+Shift+O`).

##### Navigointi
* EPUB- ja HTML-tiedostojen MathML-kaavat näytetään AsciiMath-muodossa MathCATia käyttäen. Navigoi kaavojen välillä painamalla `M` tai `Shift+M` ja avaa alkuperäinen MathML kaavanäkymässä painamalla `Enter` tai `Välilyönti`.
* Etsi-valintaikkunassa on uusi Etsi kaikki -painike, joka näyttää kaikki hakusanan sisältävät rivit, mikä mahdollistaa siirtymisen suoraan halutulle riville.
* Elementtilista (`F7`) näyttää nyt asiakirjan taulukot, luettelot ja sivut.
* Siirry riville-, Siirry sivulle- ja Siirry prosenttiin -toiminnot tukevat nyt merkintöjä `+n` ja `-n`, joilla voit siirtyä suhteessa nykyiseen sijaintiisi.
* EPUB- ja MOBI-kirjat sekä CHM-ohjetiedostot, joissa ei ole omia otsikoita, saavat nyt otsikkonavigoinnin sisällysluettelostaan.
* KF8- eli AZW3-kirjat tukevat nyt luvuittain navigointia.
* Pelkän kuvan sisältävillä EPUB-kirjan sivuilla näytetään nyt kuvasta ilmoittava rivi, jotta niille on mahdollista siirtyä, eikä niitä enää ohiteta.

##### Äänikirjat
* Toistonopeuden muuttaminen puolikkaasta kolminkertaiseen. Käytä näppäinyhdistelmiä `Ctrl+Shift+.` ja `Ctrl+Shift+,` tai Työkalut-valikkoa.
* Äänikirjojen kirjanmerkit ja muistiinpanot muistavat nyt tarkan kohdan, johon ne lisättiin.
* Seuraava ja edellinen sijainti (`Alt+Vasen nuoli` ja `Alt+Oikea nuoli`) toimivat nyt äänikirjoissa.
* Äänikirjan kuuntelun edistymistä mitataan nyt tallenteen keston perusteella, joten Siirry prosenttiin -komento ja tilarivi näyttävät kirjan todellisen kohdan.

##### Viimeksi avatut asiakirjat
* Viimeksi avatut -alivalikkoon Tyhjennä viimeksi avattujen luettelo -toiminto.

##### PDF-asiakirjat
* Asetus, joka pitää PDF-tiedoston kaikki rivit erillisinä sen sijaan, että ne yhdistettäisiin kappaleiksi.
* PDF-tiedostojen kuvat ja kuvitukset ilmoitetaan.
* PDF-tiedostojen merkitsemättömät kuvat ilmoitetaan nyt, jos tiedostossa on lukemisjärjestys, eikä niitä enää jätetä kirjasta kokonaan pois.

##### Selainnäkymä
* Selainnäkymässä voidaan nyt avata mikä tahansa asiakirja , eikä pelkästään EPUB-, HTML- ja Markdown-tiedostoja.

##### Luettavuus
* Otsikot näytetään nyt niiden tasoa vastaavassa koossa, ja kuvat sekä taulukot erotetaan selkeästi ympäröivästä tekstistä.

##### Pb-komentorivityökalu
* Komento `pb --list-formats` näyttää luettelon kaikista tiedostomuodoista, joita pb pystyy lukemaan.
* Työkalu ilmoittaa nyt, mitä tiedostoa se ei voinut lukea ja miksi.

#### Korjattu

##### Yleiset
* Kaatuminen Paperbackia suljettaessa.
* Ikkuna piilotetaan nyt heti Paperbackia suljettaessa sen sijaan, että se jäisi näkyviin tallennuksen ajaksi.
* Avaa viimeksi suljettu -toiminto ei ole enää käytettävissä asiakirjan avaamisen jälkeen, jos uudelleen avattavia tiedostoja ei ole.
* Paperback ei enää yritä toistuvasti avata viimeksi avattujen asiakirjojen luettelosta hävinneitä asiakirjoja, ja luetteloon tallennettavien asiakirjojen määrää on myös rajoitettu.
* Vanha INI-asetustiedosto poistetaan sen jälkeen, kun se on siirretty uuteen muotoon.
* Fontin- ja värinmääritysikkunoiden otsikot sekä Vie muodossa -valikko on nyt käännetty vietnamiksi.
* Päivityksen jälkeen uudelleen käynnistyvän sovelluksen ikkuna siirtyy nyt etualalle sen sijaan, että jäisi Alt+Sarkain-järjestyksessä viimeiseksi.
* Rivitys otetaan nyt käyttöön suurissa asiakirjoissa heti sen sijaan, että koko tiedosto avattaisiin uudelleen.

##### Navigointi
* `Alt+Vasen nuoli` -näppäinkomennolla palataan nyt vanhemman sijainnin asemesta edelliseen kohtaan.
* Kirjanmerkin merkkiääni ei enää kuulu heti samalle riville tultaessa vaan pelkästään kirjanmerkin kohdalle siirryttäessä.
* Sisällysluettelon, elementtilistan tai Siirry-ikkunan sulkeminen siirtää kohdistimen heti halutulle riville ilman, että ruudunlukija lukee koko ikkunaa uudelleen.
* Siirry riville-, Siirry sivulle- ja Siirry prosenttiin -toiminnot hylkäävät nyt asiakirjan ulkopuoliset numerot sen sijaan, että siirtäisivät muualle.
* NVDA ei enää keskeytä ilmoitusta, joka kertoo, ettei asiakirjassa ole sivuja.
* Kohdistuksen paikka ei muutu, kun sisällysluettelossa painetaan OK-painiketta siirtämättä kohdistinta senhetkisestä sijainnista.
* Sisällysluettelo, elementtilista ja kirjanmerkkiluettelo eivät enää hidastele tai jumiudu kirjoissa, joissa on tuhansia kohteita.
* Ylä- ja alanuolinäppäimillä vaihdettu sarake on nyt asiakirjakohtainen sen sijaan, että se olisi sama kaikilla välilehdillä.

##### Äänikirjat
* Äänitteen toiston pikanäppäin on nyt macOS:ssä `Ctrl+Välilyönti`, koska `Cmd+Välilyönti` on varattu Spotlight-haulle.

##### PDF-asiakirjat
* Ongelma, jonka vuoksi Apple Pages -sovelluksesta viedyt PDF-tiedostot luettiin pelkkänä tekstinä ilman niihin sisältyviä otsikoita ja luetteloita.
* Ongelma, jonka vuoksi PDF-tiedostojen kappaleet ja otsikot katkesivat jokaiselta riviltä ja sanat jakautuivat välilyöntien kohdalta.
* Ongelma, jonka vuoksi PDF-tiedostojen numeroidut otsikot yhdistyivät yhdeksi otsikoksi.
* Ongelma, jonka vuoksi PDF-tiedostot avautuivat tyhjinä, jos niiden rakennepuu ei sisältänyt tekstiä.
* Ylä- ja alatunnisteita ei enää lueta jokaisella merkitsemättömien PDF-tiedostojen sivulla.
* Otsikko ja sivunumero eivät enää toistu jokaisella sivulla kappaleiden välissä PDF-tiedostoissa, joissa ylä- ja alatunnisteet on merkitty tavalliseksi tekstiksi.
* PDF-tiedostoissa näytetään nyt niiden oikea otsikko tiedostonimen sijasta.
* Tasalevyisellä fontilla näytettäviä rivejä, kuten koodia, ei enää yhdistetä kappaleiksi.

##### MOBI- ja AZW3-kirjat
* Suuret MOBI-kirjat eivät enää aiheuta muistin loppumista, eikä niitä enää katkaista 20 megatavun kohdalla.
* MOBI- ja AZW3-kirjat avautuvat nyt huomattavasti nopeammin.
* MOBI-kirjojen sisällysluettelon katoaminen.
* Ongelma, jonka vuoksi MOBI-kirjojen teksti muuttui lukukelvottomaksi tietueiden vaihtumiskohdassa.

##### Selainnäkymä
* Suuren kirjan koko sisältöä ei käsitellä kerralla selainnäkymässä.
* Selainnäkymä näyttää nyt asiakirjan kokonaan lukutilan mukaisesti, eikä vain pientä osaa siitä.

##### Muut tiedostomuodot
* Windows-1251-merkistökoodausta käyttävät FictionBook-kirjat (.fb2) avautuvat nyt oikein. Suurin osa FB2-kirjoista on tällaisia.
* FictionBook-kirjat, joissa käytetään määrittelemätöntä nimiavaruutta tai HTML-entiteettiä, avautuvat nyt eikä niitä enää hylätä virheellisinä.
* Vanhoja merkistökoodauksia käyttävät kirjat avautuvat nyt huomattavasti nopeammin.
* Joidenkin kiinankielisten tekstitiedostojen avautuminen lukukelvottomina.
* Salasanalla suojattuja OpenDocument-tiedostoja ei enää väitetä virheellisiksi, vaan sovellus pyytää salasanaa niitä avattaessa.
* Salasanalla suojatut vanhat PowerPoint-esitykset avautuvat nyt, eikä niiden dioista enää katoa tekstiä.
* RTF-päätteellä tallennetut tekstitiedostot avautuvat nyt sen sijaan, että niistä annettaisiin virheilmoitus.
* RTF-tiedostojen muotoilukoodit eivät näy enää tekstinä.

#### iOS ja Android

iOS- ja Android-sovellukset tukevat samoja tiedostomuotoja kuin työpöytäversio, ja niissä on lisäksi seuraavat ominaisuudet:

* Tekstistä puheeksi -toiminnolla lukeminen valitsemallasi äänellä, nopeudella ja äänenkorkeudella, puhenopeuden säätö suoraan lukupalkissa sekä valinnainen tauko kappaleiden välissä.
* DAISY-, M4B- ja MP3-äänikirjojen toisto, joka jatkuu taustalla ja lukitusnäytöllä.
* Navigointi lukupalkista otsikoiden, sivujen, linkkien, taulukoiden, luetteloiden ja muiden elementtien perusteella, minkä lisäksi käytössä ovat sisällysluettelo ja Etsi-toiminto.
* Uniajastin, sanamäärä ja asiakirjan vienti sekä iOS:ssa käyttäjän sanasto.
* Tekstikoon, rivivälin ja suurikontrastisen tekstin asetukset.
* Samat pikanäppäimet kuin työpöytäversiossa.

### Versio 0.9.2
* Ruudunlukija ei enää lue äänikirjoissa peräkkäisiä välilyöntejä kohdistuksen siirtyessä kirjan tekstinäkymään.
* Äänikirjoissa ilmoitetaan nyt kulloisenkin äänitiedoston nimi lukujen välillä siirryttäessä.
* Sovellus ilmoittaa nyt äänikirjojen todellisen keston sen sijaan, että kaikkien väitettäisiin olevan 24 tunnin mittaisia
* Kun selainnäkymässä on siirrytty linkin osoittamaan kohtaan, näkymän sulkeminen Esc-näppäimellä ei aiheuta enää vianmääritysilmoitusta.
* Valitse kaikki -toiminnon jälkeinen kopiointi kattaa nyt koko asiakirjan eikä pelkkää näytettävää osaa.
* Etsi-toiminto siirtää suoraan löytämälleen riville eikä ruudunlukija enää lue koko ikkunaa uudelleen kohdistuksen palatessa kirjaan.
* Korjattu ongelma, jonka vuoksi ylimääräisen ZIP64-lohkon sisältämät EPUB-tiedostot aiheuttivat ”Invalid local file header” -virheilmoituksen sen sijaan, että olisivat avautuneet normaalisti.
* Pitkissä asiakirjoissa ei enää palata alkuun, kun niitä luetaan ruudunlukijan jatkuvalla luvulla.
* Selainnäkymän linkit siirtävät nyt osoittamaansa kohtaan eivätkä aiheuta enää ”Tiedostoa ei löydy” -ilmoitusta.
* Automaattinen ”Asiakirja päivitetty” -ilmoitus odottaa nyt ruudunlukijan puheen päättymistä eikä keskeytä sitä enää kesken lauseen.
* Asetukset-ikkunan Yleiset-välilehden sarkainjärjestys noudattaa nyt näytön järjestystä, ja päivityskanava on heti päivitystarkistuksen jälkeen.
* Avaa sovelluksessa -valikossa ei näytetä enää sovelluksen koko kuvausta vaan pelkkä "Paperback".
* Sanamäärä- ja Asiakirjan tiedot -ikkunoissa näytetään nyt äänikirjan tiedostomäärä sekä kokonaiskesto.

### Versio 0.9.1
* Kirjanmerkkien ja muistiinpanojen merkkiäänet toistuvat nyt macOS:ssä.
* DAISY-kirjojen ääni kuuluu nyt macOS:ssä, eikä niiden aikajanaa seurata äänettömästi.
* Korjattu ongelma, jonka vuoksi kaarevat lainausmerkit, ajatusviivat ja vastaavat merkit katosivat RTF-asiakirjoista yhdistäen samalla ympäröivät sanat toisiinsa.
* Korjattu ongelma, jonka vuoksi RTF-kuvien raakadata vuoti asiakirjaan tekstisotkuna.
* Korjattu ongelma, jonka vuoksi Viimeksi avatut -alivalikossa säilyi vanhentuneita merkintöjä, kunnes valikon sisältö muodostettiin uudelleen.
* Valikkojen pikanäppäimet ovat palanneet kaikkiin käännöksiin, joten esimerkiksi venäjänkielistä käyttöliittymää käytettäessä valikkokohteita on taas mahdollista avata korostetuilla näppäimillä.
* Suuret CHM-asiakirjat avautuvat nyt jopa seitsemän kertaa nopeammin.
* Avatut asiakirjat rekisteröidään Windowsiin, joten ne näkyvät nyt tehtäväpalkin pikavalikossa ja Käynnistä-valikon viimeksi avattujen tiedostojen luettelossa.
* Asetukset-valikon englanninkielinen nimi (Options) on muutettu muotoon Settings, mikä vastaa mobiilisovelluksia sekä macOS-käyttöjärjestelmän käytäntöä.
* Paperback muistaa nyt ikkunan sijainnin, koon ja suurennetun tilan käynnistysten välillä.
* Monikkomuodot on nyt käännetty, joten lukumääriä sisältävät viestit toimivat oikein kielissä, joissa tarvitaan yhtä useampaa monikkomuotoa.
* DAISY-kirjan ncc.html-tiedoston valitseminen avaa nyt koko äänikirjan pelkän tekstin sijaan.
* Muokkaa pikanäppäimiä -ikkunan toimintojen nimet ovat nyt käännettävissä eri kielille.
* Asiakirjan nimi näkyy nyt ensimmäisenä otsikkopalkissa, joten avoinna olevat kirjat on helpompi erottaa toisistaan tehtäväpalkissa ja Alt+Sarkain-näppäimiä käytettäessä.
* Päivitysikkuna on nyt käännetty eri kielille.

### Versio 0.9.0

#### Lisätty

##### Yleiset
* Pb-niminen komentorivityökalu, jolla voi muuntaa nopeasti minkä tahansa Paperbackin tukeman tiedostomuodon HTML:ksi, Markdowniksi tai pelkäksi tekstiksi.
* Asetusz, joka päivittää muilla sovelluksilla muokattujen asiakirjojen sisällön automaattisesti tekstinäkymään.
* Näytä lähdekoodi -vaihtoehto, joka avaa asiakirjan lähdekoodin uuteen välilehteen, josta on hyötyä esimerkiksi Markdown-tiedostojen muokkauksessa.
* Asiakirjan teksti näytetään nyt sivuittain, minkä ansiosta jopa kymmeniä miljoonia sanoja sisältävät kirjat avautuvat vain parissa sekunnissa. Ilmoitathan, jos havaitset toiminnassa jotain poikkeavaa.

##### Tuettavat käyttöjärjestelmät
* Tuki ARM64-pohjaiselle Windowsille.
* Tuki macOS:lle.
* Koko näytön tilan käyttöönotto tai käytöstä poisto.

##### Kaikki asiakirjat -valintaikkuna
* Etsi-painike sellaisten kirjojen etsimiseen, joiden hakemistopolut ovat muuttuneet.
* Tilasuodatin ja tilarivi, joiden avulla voit suodattaa asiakirjoja tilan perusteella sekä nähdä näytettävien ja valittujen asiakirjojen määrän.
* `Ctrl+Shift+A`-pikanäppäin kaikkien asiakirjojen valinnan perumiseen.

##### Asetukset ja luettavuus
* Luettavuus-välilehti, jossa on seuraavat asetukset:
    * Tekstin rivitys (siirretty Yleiset-välilehdeltä)
    * Näytä taulukot tekstin osana (uusi tässä versiossa, katso jäljempää)
    * Fontti
    * Taustaväri
    * Riviväli
    * Kappaleväli
    * Kirjainväli
    * Tekstin tasaus
* Rivitys-vaihtoehto ja sille oma pikanäppäin.
* Asetus taulukoiden näyttötavan määrittämiseksi sekä yhtenäistetty taulukoiden esitystapa kaikille asiakirjoille.

##### Navigointi
* Lisätty tuki säilöittäin navigoimiselle.
* Asetus, joka siirtää kohdistimen navigoitaessa rivin alkuun, kuten ruudunlukijoiden selaustilassa.
* Kohdistimen sijainnin kirjassa prosentteina ilmoittava =-pikanäppäin.

##### Kirjanmerkit
* Tuki tilapäisille kirjanmerkeille, joita voi olla yksi kussakin asiakirjassa, ja ne säilyvät myös asiakirjan sulkemisen jälkeen. Lisää painamalla / ja siirry siihen painamalla \.

##### Sanamäärä
* Sanamäärä-valintaikkunaan arvioitu lukuaika sekä mahdollisuus oman lukunopeuden määrittämiseen, jotta tästä tiedosta on oikeasti hyötyä.
* Jos tekstiä on valittuna Sanamäärä-valintaikkunaa avattaessa, näkymässä näytetään nyt valittujen sanojen määrä.

##### Pikanäppäimet
* Helppokäyttöinen valintaikkuna, jossa voi muokata kaikkia sovelluksen pikanäppäimiä.
* Muokattava pikanäppäin Paperbackin palauttamiseen ilmoitusalueelta.

##### Kielet
* Hollannin-, puolan- ja suomenkieliset käännökset.

##### Vienti
* Vientitoimintoa on laajennettu siten, että se mahdollistaa viennin pelkän tekstin lisäksi myös HTML- ja Markdown-muotoihin.

##### Päivittäjä
* Peruuta-painike käynnissä olevan päivityksen valintaikkunaan.
* Päivittäjä varmistaa nyt, ettei ladattua tiedostoa ole peukaloitu.

##### Selainnäkymä
* Selainnäkymä avautuu nyt nykyisessä lukukohdassa.

##### DAISY-kirjat
* DAISY 2.0 -kirjojen tuki.
* DAISY 2.02 -äänikirjojen tuki.

##### Äänikirjat
* Mahdollisuus äänikirjojen kuunteluun. Sovellus tukee ZIP-tiedostoiksi pakattuja tai niistä purettuja pelkkää ääntä tai sekä tekstiä että ääntä sisältäviä DAISY-kirjoja.
* Äänitteen toiston ja pysäytyksen, eteen- ja taaksepäin kelauksen sekä kelattaessa käytettävän aikasiirtymän muuttamisen näppäinkomennot ja valikkokohteet.
* Asetukset, joilla lukukohdistin seuraa äänitteen toistokohtaa, määritetään äänitteen kelauksen aikasiirtymä ja valitaan, jatkuuko toisto seuraavasta luvusta nykyisen luvun loppua pidemmälle kelattaessa.

##### CHM-asiakirjat
* Luetteloiden, luettelokohteiden, kuvitusten ja kuvien tuki.

##### PowerPoint
* PowerPoint-esitykset tukevat nyt taulukoita.

#### Korjattu

##### Yleiset
* Vanhoja CJK-merkistöjä, kuten GBK, Big5 ja Shift_JIS, käyttäviä asiakirjoja ei näytetä enää pelkkänä merkkisotkuna.
* Avaa viimeksi suljettu -toiminto yritti avata sovelluksen mukana toimitettavan readme-tiedoston.
* Kohdistus ei siirtynyt oikein valitulle välilehdelle Paperbackin uudelleenkäynnistyksen jälkeen.
* Verkkolevyillä sijaitsevien tiedostojen käsittelyä on parannettu. Avaa asiakirjan kansio -toiminto siirtää nyt kohdistuksen asianmukaisesti verkkolevyllä olevan tiedoston kohdalle, eikä hakemistopoluissa ole enää outoja merkkejä.
* Työpöytäversiossa .paperback-tiedostoja ei enää avata automaattisesti asiakirjoja palautettaessa, vaan tiedoston löytyessä pyydetään vahvistus.
* Avaa asiakirjan kansio -toiminto siirtää nyt kohdistuksen kyseisen tiedoston kohdalle Resurssienhallinnassa.
* Käytössä oleva kieli otetaan nyt huomioon Readme-tiedostoa avattaessa.
* Paperbackin käyttöliittymä mukautuu nyt oikein tarkoilla näytöillä.
* Valikko päivittyy nyt oikein ja kohdistus siirtyy asiakirjan tekstiin, kun ohje avataan Paperbackissa.
* Windowsissa on otettu käyttöön huomattavasti turvallisempi prosessien välinen viestintämenetelmä.
* Aktiivisen asiakirjan nimi luetaan nyt välilehteä vaihdettaessa.
* Suurten asiakirjojen muistinkäyttöä on vähennetty puolittamalla sisäisten merkkikohtaisten indeksitaulukoiden koko.

##### Kaikki asiakirjat -valintaikkuna
* Asiakirjan tiedot- ja Kaikki asiakirjat -valintaikkunat eivät sulkeutuneet Esc-näppäimellä.
* Otsikkopalkki ei päivittynyt, kun asiakirja suljettiin Kaikki asiakirjat -valintaikkunasta.
* Readme.html-tiedostoa ei enää lisätä Kaikki asiakirjat -luetteloon, kun se avataan Shift+F1-pikanäppäimellä.
* Asiakirjojen poistaminen Viimeksi avatut asiakirjat -valintaikkunasta sulkee nyt myös niiden aktiiviset välilehdet.
* Hakusuodatin säilytetään nyt myös asiakirjan poistamisen jälkeen.

##### Navigointi
* Ruudunlukija puhui joissakin tilanteissa väärän rivin asiakirjassa liikuttaessa.
* Siirry riville-, Siirry sivulle- ja Siirry prosenttiin -toiminnot siirsivät kohdistimen suurissa asiakirjoissa väärään kohtaan.
* Etsi- ja Etsi seuraava -toiminnot eivät huomioineet suurten asiakirjojen näkyvissä olevaa osaa.

##### Kirjanmerkit
* Merkkiääni toistetaan nyt vain sellaisten sanojen kohdalla, joihin on lisätty kirjanmerkki tai muistiinpano.

##### Luettavuus
* Rivityksen käyttöönotto siirsi kohdistuksen asiakirjan alkuun.

##### Selainnäkymä
* Selainnäkymäikkunan kokoa voi nyt muuttaa, eikä se avaudu enää liian pienenä.
* Kuvat näytetään nyt oikein selainnäkymässä.

##### Päivittäjä
* Päivittäjä näyttää nyt versiotiedoissa oikein Markdown-kooditunnisteiden sisällön.

##### DAISY-kirjat
* DAISY-kirjoista näytettiin tilarivillä virheellisiä tietoja.
* Virheellisen koodausmäärityksen sisältävien DAISY-kirjojen lataaminen.

##### RTF-asiakirjat
* Muita kuin latinalaisia kirjaimia sisältävien RTF-asiakirjojen jäsennys.
* RTF-tiedostojen `\pict`-ryhmien käsittely, jotta upotettujen kuvien tiedot eivät enää päädy asiakirjan tekstiin.

##### Mobi- ja AZW3-kirjat
* Mobi-kirjojen filepos-ankkurit rikkoivat HTML-elementtejä ja aiheuttivat merkkisotkua kirjan tekstiin.
* Vanhojen Mobi-kirjojen linkkien toiminta.
* AZW3-kirjojen jäsennystä on parannettu huomattavasti.

##### Word-asiakirjat
* Eri kielisiä tyylinimiä sisältävien Word-asiakirjojen otsikoita ei näytetty oikein.

##### HTML- ja XHTML-asiakirjat
* XHTML-asiakirjojen dl-, dt- ja dd-elementit eivät tehneet rivinvaihtoja.

##### PDF-asiakirjat
* Paperback käyttää nyt virheellisesti merkityille PDF-tiedostoille varavaihtoehtona pelkän tekstin poimimista.
* Paperback ei enää kaadu avattaessa PDF-asiakirjoja, joiden otsikoissa ja/tai kirjanmerkeissä on ohjausmerkkejä.

### Versio 0.8.5
* Lisätty sivujen tuki EPUB-kirjoille.
* Lisätty tuki salatuille Microsoft Office -asiakirjoille. Tällä hetkellä tuetaan vanhaa ja uudempaa Wordia sekä uudempaa PowerPointia, ja vanhan PowerPointin tuki on tulossa myöhemmin.
* Lisätty tuki vanhoille Microsoft Word -asiakirjoille.
* Lisätty tuki vanhoille PowerPoint-esityksille.
* Lisätty tuki mobi- ja AZW3-kirjoille.
* Lisätty tuki tunnisteita sisältäville PDF-tiedostoille.
* Lisätty Ctrl+Q-näppäinkomento sovelluksen lopettamista varten.
* Lisätty tuki sekä DAISY- että Word-muodossa oleville Booksharen pakatuille kirjoille.
* Upotettujen kuvien vaihtoehtoisen tekstin pitäisi nyt näkyä oikein.
* CHM-asiakirjoissa tuetaan nyt asianmukaisesti sisäisten linkkien navigointia.
* Korjattu "Siirry sivulle" -toiminnon virhe, jonka vuoksi tietylle sivulle siirtyminen oli aina yhden numeron verran pielessä.
* Korjattu ongelma, jonka vuoksi "Avaa muodossa" -valintaikkuna ei sulkeutunut Esc-näppäimellä.
* Korjattu lukijan pikavalikko, joka ei avautunut hiiren oikealla painikkeella eikä sovellusnäppäimellä.
* Korjattu ongelma, jonka vuoksi kohdistus siirtyi toisinaan väärään asiakirjaan, kun niitä avattiin komentoriviltä.
* Pelkkiä kuvia sisältävät PDF-tiedostot tunnistetaan taas ja sovellus ilmoittaa niistä.
* Kuvien ja kuvitusten välillä on nyt mahdollista liikkua G/Shift+G- ja F/Shift+F-näppäimillä.
* Paperback noudattaa nyt sovelluksen tumman tilan asetusta.
* DAISY XML -tuki on poistettu, koska sitä ei enää tarvita.
* Palattu käyttämään alkuperäistä Win32:n ensimmäisen kirjaimen navigointia sisällysluettelopuussa.
* Asiakirjan avaamisvirheestä ilmoittava ikkuna näyttää nyt yksityiskohtaisempia virheilmoituksia.
* Selainnäkymä avautuu nyt paljon nopeammin ja sulavammin.

### Versio 0.8.2
* Lisätty sivujen tuki RTF-asiakirjoille.
* Korjattu virhe, jonka vuoksi EPUB-kirjojen ulkoiset linkit avattiin automaattisesti selainnäkymää avattaessa.
* Korjattu virhe, jonka vuoksi RTF-jäsennin ei lisännyt joissakin harvinaisissa tapauksissa välilyöntiä sanojen väliin.
* Korjattu kappaleet, jotka jakautuivat useiksi lyhyiksi riveiksi joissakin PDF-asiakirjoissa.
* PDF-asiakirjoissa on nyt perustason linkki- ja otsikkonavigointi.
* RTF:n sarkaimet ja rivinvaihdot piirretään nyt täsmälleen niin kuin ne näkyvät asiakirjassa.
* Palattu käyttämään hyväksi havaittua pdfium-kirjastoa PDF-tiedostojen jäsentämiseen, minkä ansiosta niiden näyttäminen toimii jälleen huomattavasti luotettavammin.

### Versio 0.8.1
* Lisätty Ctrl+Shift+T viimeksi suljetun asiakirjan uudelleenavaamista varten.
* Kaikki asiakirjat -valintaikkuna tukee nyt kerralla useiden avattavien asiakirjojen valintaa.
* Korjattu muutamia RTF-jäsentimen virheitä.
* Korjattu muita kuin ASCII-merkkejä (kuten bosnian š, č, ć ja ž) sisältävät tiedostopolut, jotka vioittuivat, kun tiedosto avattiin toisen Paperback-kopion kautta.
* Korjattu PDF-tekstin lukujärjestys sekä virheellinen sanaväli isolla kirjaimella alkavien sanojen ympärillä.
* Korjattu hitaat asiakirjojen lataukset suuria tiedostoja avattaessa.
* Korjattu vahvistusvalintaikkunoiden Kyllä- ja Ei-painikkeiden lokalisointi.

### Versio 0.8.0
* Lisätty japanin, yksinkertaistetun kiinan ja vietnamin käännökset.
* Lisätty automaattinen päivittäjä, joka korvaa nyt nykyisen asennetun Paperback-version sen sijaan, että vain lataisi uuden version.
* Lisätty valinnainen äänipalaute kirjanmerkin tai muistiinpanon kohdalle siirtymisestä. Kiitos Andre Louis'lle merkkiäänistä.
* Lisätty RTF-asiakirjojen tuki.
* Lisätty tuki DAISY XML -asiakirjoille.
* Lisätty tuki Flat Open Document Text -tiedostoille.
* Lisätty tuki Flat Open Document -esityksille.
* Lisätty tuki erottimiin siirtymiselle S- ja Shift+S-näppäimillä.
* Kaikki yli 300 merkin pituiset siirtymät lisätään nyt automaattisesti navigointihistoriaan.
* Korjattu Paperbackin ikkunan palautus ilmoitusalueelta.
* Korjattu Markdown-asiakirjojen näyttäminen selainnäkymässä raakatekstinä muotoillun HTML:n sijaan.
* Korjattu Markdown-taulukoiden virheellinen muotoilu.
* Paperback varoittaa nyt yritettäessä avata pelkkiä kuvia sisältäviä PDF-tiedostoja.
* Versiotiedot upotetaan nyt oikein Paperbackin sovellustiedostoon.
* Asetusvalintaikkuna jaettu välilehtiin käytön ja navigoinnin helpottamiseksi.
* Siirrytty käyttämään Hayro-kirjastoa PDF-tiedostojen jäsentämiseen, mikä parantaa luotettavuutta, nopeutta ja vähentää DLL-tiedostojen määrää.
* Koko sovellus on uudelleenkirjoitettu Rust-ohjelmointikielellä. Uusi koodipohja on turvallisempi, lataa asiakirjat nopeammin ja sitä on helpompi ylläpitää ja laajentaa.
* Sisältöä näyttävän elementin pikavalikossa on nyt lukusovellukselle ominaisia komentoja eikä yleisiä toimintoja, kuten Leikkaa tai Liitä.

### Versio 0.7.0
* Lisätty taulukoiden tuki HTML- ja XHTML-pohjaisille asiakirjoille. Liiku taulukoiden välillä T:llä ja Shift+T:llä ja avaa taulukko selainnäkymässä painamalla Enter.
* Lisätty alkeellinen verkkorenderöinti. Avaa asiakirjan nykyinen luku verkkopohjaisessa renderöijässä painamalla Ctrl+Shift+V. Tästä on hyötyä esimerkiksi monimutkaisessa muotoilussa tai koodiesimerkeissä.
* Lisätty venäjänkielinen käännös. Kiitos Ruslan Gulmagomedoville.
* Lisätty "Tyhjennä kaikki" -painike Kaikki asiakirjat -valintaikkunaan.
* Päivitysten tarkistaja näyttää nyt julkaisutiedot, kun uusi versio on saatavilla.
* Korjattu ikkunan palautus ilmaisinalueelta.
* Kyllä/Ei-painikkeiden käännökset korjattu vahvistusvalintaikkunoissa.
* Korjattu asetusten lataus, kun ohjelmaa ajetaan järjestelmänvalvojana.
* Kommenttien käsittely korjattu XML- ja HTML-asiakirjoissa.
* Korjattu sisällysluettelon jäsennys EPUB 2 -kirjoissa.
* Korjattu siirtyminen sisällysluettelon seuraavaan samalla kirjaimella alkavaan kohteeseen.
* Korjattu Etsi-valintaikkuna, jota ei aina piilotettu oikein Seuraava/Edellinen-painikkeita käytettäessä.
* Korjattu virhe, jossa EPUB-kirjojen sisällysluettelot siirsivät toisinaan väärään kohtaan.
* Korjattu useita välilyöntien käsittelyyn liittyviä ongelmia XML-, HTML- ja pre-tageissa.
* Korjattu virhe, jossa linkkien välillä siirtyminen oli yhden kohdan verran pielessä.
* Korjattu joissakin kirjoissa esiintynyt rivien loppuun jäävä ylimääräinen välilyönti.
* Korjattu useita jäsentimen ongelmia.
* Kirjanmerkkeihin liittyvät valikkokohdat sekä elementtilista poistetaan nyt käytöstä asianmukaisesti, kun yhtään asiakirjaa ei ole avoinna.
* Parannettu luetteloiden käsittelyä useissa asiakirjamuodoissa.
* Kääntäjien työnkulkua parannettu.
* Useita sisäisiä uudelleenjärjestelyjä, joissa suurin osa sovelluslogiikasta siirrettiin C++:sta Rustiin suorituskyvyn ja ylläpidettävyyden vuoksi.

### Versio 0.6.1
* Lisätty tuki salasanalla suojatuille PDF-tiedostoille.
* Lisätty hyvin yksinkertainen toiminto edelliseen ja seuraavaan sijaintiin siirtymistä varten. Kun painat Enteriä sisäisen linkin kohdalla ja kohdistin siirtyy, kyseinen sijainti tallennetaan, ja siihen voi palata komennolla Alt+vasen/oikea nuolinäppäin.
* Elementtilista lisätty. Tällä hetkellä se näyttää vain kaikkien asiakirjan otsikoiden puunäkymän tai linkkiluettelon, mutta sitä on tarkoitus laajentaa tulevaisuudessa.
* Lisätty asetus, jolla Paperback käynnistyy oletusarvoisesti suurennettuna.
* Korjattu joidenkin EPUB-asiakirjojen virheellisesti toimineet linkit.
* Korjattu suhteellisia polkuja sisältävien EPUB-kirjojen sisällysluetteloiden jäsennys.
* Korjattu virhe, jossa nimeä tai tekijää ei näytetty joissakin EPUB-asiakirjoissa.
* Korjattu virhe, jossa joidenkin EPUB-asiakirjojen lukujen nimet eivät näkyneet oikein sisällysluettelovalintaikkunassa.
* Korjattu virhe, jossa sisällysluettelovalintaikkunan OK- tai Peruuta-painikkeita ei voinut painaa välilyöntinäppäimellä.
* Otsikoiden käsittelyä parannettu Word-asiakirjoissa.
* Paperback antaa nyt äänipalautteen yritettäessä avata "Viimeksi avatut asiakirjat" -valintaikkunaa, kun luettelo on tyhjä.

### Versio 0.6.0
* Asetukset-valintaikkunaan lisätty uusi asetus, jolla Siirry-valikko voidaan näyttää huomattavasti tiiviimmässä muodossa. Se on oletusarvoisesti käytössä.
* Lisätty asetus, jolla rakenteisten elementtien perusteella tapahtuva navigointi palaa asiakirjan lopussa takaisin alkuun.
* Työkalut-valikkoon lisätty vaihtoehto, jolla voidaan avata nykyisen asiakirjan sisältävä kansio.
* Lisätty melko yksinkertainen mutta erittäin tehokas päivitysjärjestelmä.
* Lisätty perustason uniajastin, jonka voi avata Ctrl+Shift+S-näppäinkomennolla.
* Lisätty FB2-e-kirjojen jäsennystuki.
* Lisätty OpenDocument-esitysten jäsennystuki.
* Lisätty OpenDocument-tekstitiedostojen jäsennystuki.
* Kirjanmerkit voivat nyt kohdistua koko riville tai pelkästään valittuun tekstiin. Jos tekstiä ei ole valittuna kirjanmerkkiä luotaessa, toiminnallisuus on sama kuin ennen versiota 0.6, ja koko rivi merkitään. Mikäli tekstiä on valittuna, kirjanmerkki kohdistuu vain kyseiseen tekstiin.
* Kirjanmerkeissä voi nyt olla valinnaisia muistiinpanoja. Siirry muistiinpanoja sisältävien kirjanmerkkien välillä N:llä ja Shift+N:llä, tai avaa kirjanmerkkien valintaikkuna, jossa kaikki kirjanmerkit, vain muistiinpanot tai vain ilman muistiinpanoja olevat kirjanmerkit voidaan valita tietyillä pikanäppäimillä.
* Kirjanmerkeissä ei enää ole ärsyttävää "bookmark x" -etuliitettä kirjanmerkkien valintaikkunassa.
* XML:ltä näyttävää HTML-koodia sisältävät EPUB-kirjat käsitellään nyt oikein.
* Suurten Markdown-asiakirjojen lataaminen korjattu.
* Välilyöntinäppäimen painaminen sisällysluettelon puurakenteessa ei enää paina OK-painiketta.
* Välilyöntien käsittely korjattu pre-tagien alussa sekä HTML- että XHTML-asiakirjoissa.
* Korjattu virhe, jossa kohdistus ei siirtynyt takaisin tekstikenttään Paperbackin ikkunaan palattaessa.
* "Siirry prosenttiin" -valintaikkunan tekstikenttä päivittää nyt oikein liukusäätimen arvon.
* Mukautettujen HTML ID -tunnisteiden renderöinti korjattu Markdown-asiakirjoissa.
* Markdown-koodilohkojen sisällä oleva HTML renderöidään nyt oikein.
* Kun kirja ladataan komentoriviparametrilla jo käynnissä olevaan Paperback-kopioon, virheilmoitusta ei enää näytetä, vaikka dokumentin lataus kestäisi yli 5 sekuntia.
* Asetukset ladataan ja tallennetaan nyt asianmukaisesti, kun Paperback on käynnissä järjestelmänvalvojana.
* Kirjanmerkki voidaan nyt poistaa suoraan kirjanmerkkien valintaikkunasta.
* Asiakirjan kirjanmerkkien ja lukukohdan tuonti ja vienti on nyt mahdollista. Luotu tiedosto nimetään asiakirjan tiedostonimen perusteella ja sen tunniste on .paperback. Mikäli tällainen tiedosto löytyy ladattaessa asiakirjan kansiosta, se ladataan automaattisesti. Muussa tapauksessa voit tuoda sen manuaalisesti Työkalut-valikon toiminnolla.
* Asiakirjojen sisäiset linkit ovat nyt täysin tuettuja. Siirry niiden välillä eteen- ja taaksepäin K- ja Shift+K-näppäimillä ja avaa linkki tai siirry sen kohdalle Enterillä.
* Tehty useita sisäisiä uudelleenjärjestelyjä, jotka nopeuttavat ohjelmaa ja pienentävät binääriä.
* Markdown-sisältö esikäsitellään nyt CommonMark-yhteensopivaksi ennen renderöintiä.
* Luetteloiden ja niiden kohteiden välillä navigointia tuetaan nyt täysin. Voit siirtyä luetteloiden välillä L- ja Shift+L-näppäimillä ja luettelokohteiden välillä I- ja Shift+I-näppäimillä.
* Tavallisen Delete-näppäimen lisäksi myös numeronäppäimistön Deleteä voi  nyt käyttää asiakirjojen poistamiseen välilehtipalkista.
* Paperback voidaan nyt haluttaessa pienentää ilmaisinalueelle. Tämä asetus on oletusarvoisesti poissa käytöstä, mutta kun se otetaan käyttöön, Paperbackin järjestelmävalikon pienennystoiminto siirtää sovelluksen ilmaisinalueelle, josta se voidaan palauttaa napsauttamalla Paperbackin kuvaketta.
* Paperback on nyt käännettävissä eri kielille. Sen tukemien kielten luettelo on toistaiseksi melko pieni, mutta se kasvaa jatkuvasti.
* Paperbackilla on nyt virallinen verkkosivusto osoitteessa [paperback.dev](https://paperback.dev).
* PPTX-asiakirjoissa olevat diat näytetään nyt yksinkertaisessa sisällysluettelossa.
* Asiakirjan tiedot -valintaikkunassa näytetään nyt avoimen asiakirjan koko polku.
* Asennusohjelma sisältää nyt vaihtoehdon, jolla readme-tiedosto voidaan avata selaimessa asennuksen jälkeen.
* Viimeksi avattujen asiakirjojen luetteloa on laajennettu huomattavasti. Sen sijaan, että se näyttäisi vain 10 viimeksi avattua asiakirjaa, näytettävä määrä on nyt mahdollista määrittää itse, ja muut aiemmin avatut asiakirjat ovat käytettävissä erillisen valintaikkunan kautta.
* Useita pieniä parannuksia jäsentimiin kautta linjan, kuten tyhjän rivin lisääminen diojen väliin PPTX-esityksissä, rivinvaihtojen käsittelyn korjaaminen Word-asiakirjojen kappaleissa ja luettelokohtamerkkien lisääminen.

### Versio 0.5.0
* Lisätty Microsoft Word -asiakirjojen tuki.
* Lisätty PowerPoint-esityksien tuki.
* Korjattu virhe, jossa tietyt valikkokohteet eivät poistuneet käytöstä, kun yhtään asiakirjaa ei ollut avoinna.
* Korjattu "siirry prosenttiin" -liukusäätimen suunta.
* Korjattu EPUB-kirjojen sisällysluettelot, joissa oli URL-koodattuja tiedostopolkuja ja/tai fragmenttitunnuksia.
* Korjattu ongelma, jossa XHTML-otsikoissa olevat välilyönnit poistettiin oudosti.
* Korjattu HTML-asiakirjojen sisäkkäisten pre-tagien sisällä olevien välilyöntien käsittely.
* HTML- ja Markdown-asiakirjat tukevat nyt sisällysluetteloa. Kun HTML- tai Markdown-asiakirja ladataan, Paperback muodostaa sisällysluettelon asiakirjan otsikkorakenteesta ja näyttää sen Ctrl+T-näppäinkomennolla avattavassa valintaikkunassa.
* HTML-asiakirjoissa käytetään nyt title-tagin mukaista otsikkoa, mikäli sellainen on määritetty. Muutoin käytetään edelleen tiedoston nimeä ilman tunnistetta.
* Puhumiseen käytetään UniversalSpeech-kirjaston sijaan aktiivista aluetta. Tämä tarkoittaa, ettei ohjelman mukana enää toimiteta ruudunlukijoiden DLL-tiedostoja, ja nyt tuetaan useampia ruudunlukijoita, kuten Microsoft Narratoria.
* ZIP-kirjastoja on vaihdettu, jotta voidaan avata laajempi valikoima EPUB-kirjoja.
* Valintaikkuna, joka kysyy asiakirjan avaamista pelkkänä tekstinä, on uudistettu kokonaan ja se mahdollistaa nyt asiakirjan avaamisen pelkkänä tekstinä, HTML:nä tai Markdownina.
* "Siirry prosenttiin" -valintaikkuna sisältää nyt tekstikentän, johon voit syöttää prosenttiluvun manuaalisesti.
* HTML-jäsennin tunnistaa nyt dd-, dt- ja dl-elementit luetteloelementeiksi.
* EPUB-kirjojen sisällysluettelot säilytetään jälleen täsmälleen sellaisina kuin ne ovat.
* Unicode-merkistöön sisältyvä ei-sitova välilyönti käsitellään nyt tyhjiä rivejä poistettaessa.
* Sovellus kysyy tuntemattoman tiedoston avaustapaa vain ensimmäisellä kerralla, ei enää joka avauksella.

### Versio 0.4.1
* Lisätty asennusohjelmaan valinnainen Käynnistä-valikon kuvakkeen luonti.
* Sisällysluettelon pitäisi nyt olla joissakin tapauksissa siistimpi. Esimerkiksi jos ala- ja ylätason kohde sisältävät saman tekstin samassa kohdassa, nyt näytetään vain ylätason kohde.
* Korjattu tiettyjen CHM-asiakirjojen sisällysluettelot.
* Korjattu absoluuttisia tiedostopolkuja sisältävien EPUB 3 -kirjojen sisällysluettelot.
* CHM-asiakirjojen nimien pitäisi nyt näkyä sellaisina, kuin ne on metatiedoissa määritetty.

### Versio 0.4.0
* Lisätty CHM-tiedostojen tuki.
* Lisätty kirjanmerkkien tuki. Voit lisätä niitä asiakirjoihin rajattomasti. Siirry niiden välillä eteen- ja taaksepäin B- ja Shift+B-näppäimillä, lisää kirjanmerkki näppäinkomennolla Ctrl+Shift+B ja avaa tiettyyn kirjanmerkkiin siirtävä valintaikkuna näppäinkomennolla Ctrl+B.
* Massamuistiversion ZIP-paketin lisäksi on nyt saatavilla asennusohjelma. Se asentaa Paperbackin Program Files -hakemistoon ja määrittää tiedostoliitokset automaattisesti.
* BOM-merkkejä sisältävät tekstitiedostot dekoodataan nyt oikein, eikä BOM enää näy tekstin alussa.
* Tilariville lisätty paljon uutta tietoa. Se näyttää nyt nykyisen rivin, merkin ja luetun osuuden prosentteina.
* HTML-kommentteja tai script- ja style-tagien sisältöä ei enää näytetä tekstitulosteessa.
* Jos komentorivillä annetaan suhteellinen polku, Paperback tulkitsee sen oikein.
* Prosenttisiirtymää käsitellään nyt omassa liukusäätimeen perustuvassa valintaikkunassaan, joka voidaan avata näppäinkomennolla Ctrl+Shift+G.
* Asiakirjoille, joilla ei ole nimeä tai tekijää, asetetaan nyt aina niiden oletusarvot.
* Sijainnin tallennuslogiikka on nyt paljon älykkäämpi ja kirjoittaa levylle vain silloin, kun se on ehdottoman välttämätöntä.
* Asiakirja, joka oli aktiivisena Paperbackin sulkemishetkellä, avataan nyt uudelleen sovelluksen käynnistyessä.
* Siirry riville- ja Siirry sivulle -valintaikkunoihin syötetty tieto puhdistetaan nyt tarkemmin.
* Korjattu EPUB 3 -kirjojen sisällysluettelonavigointi, kun manifestissa on suhteellisia polkuja.

### Versio 0.3.0
* URL-koodattuja manifesteja sisältävien EPUB-kirjojen sisällysluettelot on korjattu.
* Otsikkonavigointi korjattu monitavuisia Unicode-merkkejä sisältävissä HTML-asiakirjoissa.
* Korjattu wxWidgetsin regressiosta johtuva korkea suorittimen käyttöaste asiakirjoissa, joilla on pitkät nimet.
* UTF-8-koodattujen tekstitiedostojen lataus korjattu.
* Korjattu EPUB-kirjojen sisäkkäiset sisällysluettelokohdat, jotka siirtävät kohdistimen väärään kohtaan.
* Korjattu joissakin tilanteissa ilmenevä kaatuminen sovellusta lopetettaessa.
* Lisätty asetusvalintaikkunaan asetus, jolla otetaan rivitys käyttöön tai poistetaan se käytöstä.
* Paperbackin kehitystä varten lahjoittaminen on nyt mahdollista joko ohje-valikon uudella Lahjoita-vaihtoehdolla tai GitHub-koodivaraston pääsivun alalaidassa olevan "Sponsor this project" -linkin kautta.
* Markdown-asiakirjoilla on nyt aina nimi, ja Paperbackin pitäisi nyt pystyä lataamaan käytännössä mikä tahansa Markdown-tiedosto.
* PDF-asiakirjoilla on nyt aina nimi, vaikka metatiedot puuttuisivat.
* Otettu käyttöön Chromiumin käyttämä PDF-kirjasto, joka parantaa merkittävästi PDF-tiedostojen jäsennyksen luotettavuutta koko sovelluksessa.
* Samanaikaisesti voi nyt olla käynnissä vain yksi Paperback-kopio. Jos käynnistät paperback.exe:n tiedostonimellä ohjelman jo ollessa käynnissä, kyseinen asiakirja avataan jo käynnissä olevaan kopioon.
* Voit nyt sulkea välilehtisäätimessä näkyvän asiakirjan painamalla sen kohdalla Delete-näppäintä.

### Versio 0.2.1
* "Siirry sivulle" -valintaikkunan sivunumerokentän selitteessä näytetään nyt sivujen kokonaismäärä.
* Asiakirjan sisällöstä voidaan nyt siirtyä Sarkain-näppäimellä avoimien asiakirjojen luetteloon.
* Korjattu virhe, jonka vuoksi otsikkonavigointinäppäimet saattoivat toisinaan avata viimeksi avattuja asiakirjoja, jos niitä oli tarpeeksi.
* Paperback poistaa nyt tarpeettomat pehmeät tavuviivat tekstitulosteesta.
* Korjattu otsikkonavigointi, joka siirsi toisinaan väärän merkin kohdalle.

### Versio 0.2.0
* Lisätty tuki markdown-asiakirjoille.
* Lisätty tuki PDF-asiakirjoille. Siihen sisältyy myös mahdollisuus sivujen välillä siirtymiseen.
* Lisätty otsikkonavigoinnin pikanäppäimet HTML-sisällölle, kuten EPUB-kirjoille ja Markdown-asiakirjoille. Nämä pikanäppäimet on suunniteltu toimimaan samalla tavalla kuin ruudunlukijoissa.
* Korjattu EPUB-tiedostojen lataus, kun manifesteissa on URL-koodattuja tiedostonimiä.
* Upotettua XHTML:ää sisältävien EPUB 3 -kirjojen avaaminen on korjattu.
* Jos asiakirjassa ei ole sisällysluetteloa tai lukuja, vastaavia valikkokohteita ei enää vain poisteta käytöstä, vaan sen sijaan puhutaan asianmukainen ilmoitus.
* Lisätty viimeksi avattujen asiakirjojen valikko. Siihen tallennetaan tällä hetkellä 10 viimeksi avattua asiakirjaa, ja Enter-näppäimen painaminen jonkin kohteen kohdalla avaa kyseisen asiakirjan luettavaksi.
* Etsi-valintaikkuna on kirjoitettu kokonaan uudelleen, joten sitä on nyt paljon helpompi käyttää. Siihen on lisätty myös viimeisimmän 25 haun historia sekä sääntölausekkeiden tuki.
* Aiemmin avatut asiakirjat muistetaan nyt myös sovelluksen uudelleenkäynnistyksen jälkeen. Tämä toiminto voidaan määrittää Työkalut-valikon uudesta Asetukset-kohdasta.
* Lisätty näppäinkomento Shift+F1, joka avaa readme-tiedoston suoraan Paperbackissa.

### Versio 0.1.0
* Ensimmäinen julkaisu.
