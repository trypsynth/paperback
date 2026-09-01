# Paperback - versio 0.9.2

## Johdanto

Paperback on kevyt ja saavutettava e- ja asiakirjojen lukusovellus kaikille satunnaisista lukijoista vaativiin tehokäyttäjiin. Se on suunniteltu ruudunlukijaystävällisyyttä, suorituskykyä ja turhista ominaisuuksista riisuttua käyttökokemusta ajatellen.

## Järjestelmävaatimukset

Paperback toimii Windows 10:ssä ja 11:ssä sekä kaikissa uudemmissa ARM-pohjaisissa macOS:n versioissa. Myös iOS- ja Android-sovellukset ovat kehitteillä. Niistä on määrä julkaista testiversiot pian työpöytäversio 0.9.0:n jälkeen ennen kaikkia neljää käyttöjärjestelmää kattavaa 1.0-versiota.

## Ominaisuudet

* Toimii täysin itsenäisesti ilman kolmannen osapuolen ohjelmistojen asentamista.
* Toimii erittäin nopeasti myös vanhalla laitteistolla.
* Yksinkertainen välilehtikäyttöliittymä, jonka avulla voit avata rajattoman määrän asiakirjoja.
* Tallentaa tarkan lukukohdan jokaisessa avaamassasi asiakirjassa.
* Muistaa valinnaisesti, mitkä asiakirjat olivat avoinna ohjelmaa suljettaessa, ja avaa ne seuraavalla käynnistyskerralla.
* Sisältää ruudunlukijoista tuttua verkkoselaustilaa muistuttavan navigointitoiminnon, jonka avulla voit liikkua asiakirjoissa nopeasti ja vaivattomasti.
* Sisältää tehokkaan etsintäikkunan, jossa on muun muassa historia ja sääntölausekkeiden tuki.
* Voidaan käyttää massamuistiversiona tai asentaa siten, että tiedostokytkennät määritetään automaattisesti.
Tukee erittäin kattavasti yleisiä tiedostomuotoja.

## Ruudunlukijoiden yhteensopivuus

Paperback toimii sujuvasti kaikilla yleisimmillä ruudunlukijoilla. JAWS-käyttäjien on kuitenkin hyvä tietää  eräästä tunnetusta ongelmasta.

### JAWS ja pistenäytöt

Jos käytät JAWS-ruudunlukijaa ja pistenäyttöä, pitkät kappaleet saattavat katketa, kun tekstiä vieritetään eteenpäin näytön navigointinäppäimillä. Ongelma koskee myös nykyisen kappaleen lukukomentoa. Tämä johtuu JAWSin virheestä RICHEDIT50W-tekstikentän käsittelyssä, eli vika ei ole Paperbackissa. Ratkaisun löytäminen kesti kauan, koska Visperolta on tunnetusti vaikea saada vastauksia avoimen lähdekoodin sovellusten virheraportteihin.

Kiertotienä ongelmaan on paperback.jcf-tiedoston muokkaaminen siten, että asetuksen "Braille Presentation and Panning" (Pistekirjoitusesitys ja -vieritys) arvoksi määritetään "Always use DOM if available" (Käytä aina DOMia, jos se on saatavilla). Lisäksi asetus "Pan Text by Paragraph" (Vieritä kappaleittain) on otettava käyttöön, jotta pistenäyttö siirtyy eteenpäin seuraavaan kappaleeseen. Näillä asetuksilla vierityksen pitäisi toimia oikein.

## Tuettavat tiedostomuodot

Paperback tukee seuraavia tiedostomuotoja:

* CHM-ohjetiedostot (`.chm`)
* DAISY-kirjat (`.opf`, `.zip`)
* EPUB-kirjat (`.epub`)
* FB2-e-kirjat (`.fb2`)
* HTML-asiakirjat (`.htm`, `.html`, `.xhtml`)
* Markdown-asiakirjat (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word -asiakirjat (`.docx`, `.docm`, `.doc`)
* M4B-äänikirjat (`.m4b`)
* MOBI- ja Kindle-kirjat (`.mobi`, `.azw`, `.azw3`)
* OpenDocument-esitykset (`.odp`, `.fodp`)
* OpenDocument-tekstiasiakirjat (`.odt`, `.fodt`)
* PDF-asiakirjat (`.pdf`)
* PowerPoint-esitykset (`.pptx`, `.pptm`, `.ppt`)
* RTF-asiakirjat (`.rtf`)
* Teksti- ja lokitiedostot (`.txt`, `.log`)

## Pikanäppäimet

Paperback on suunniteltu ensisijaisesti näppäimistöllä käytettäväksi. Alla on luettelo nykyisistä pikanäppäimistä.

Nämä pikanäppäimet toimivat Windowsissa. MacOS-komennot on merkitty sulkeisiin. Erot johtuvat siitä, että näppäinyhdistelmät Ctrl+G, Ctrl+W sekä Alt + vasen/oikea nuoli on varattu macOS-alustalla järjestelmän tai muiden sovellusten käyttöön.

### Tiedosto-valikko

* `Ctrl+O`: Avaa asiakirja.
* `Ctrl+F4` (macOS: `Cmd+W`): Sulje nykyinen asiakirja.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Sulje kaikki avoimet asiakirjat.
* `Ctrl+Shift+T`: Avaa viimeksi suljetun asiakirjan uudelleen.
* `Ctrl+R`: Näytä "Kaikki asiakirjat" -valintaikkuna (Viimeisimmät asiakirjat -valikosta).
* `Ctrl+Q`: Lopeta (vain Windowsissa; macOS:ää käytettäessä tämä komento löytyy sovellusvalikosta).

### Siirry-valikko

* `Ctrl+F`: Näytä Etsi-valintaikkuna.
* `F3` (macOS: `Cmd+G`): Etsi seuraava.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Etsi edellinen.
* `Ctrl+G` (macOS: `Cmd+L`): Siirry riville.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Siirry prosenttiin.
* `Ctrl+P`: Siirry sivulle (jos asiakirja tukee sitä).
* `=`: Ilmoittaa asiakirjan lukukohdan prosentteina.
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
* `Ctrl+Väli`: Toista äänite tai pysäytä sen toisto.
* `'`: Kelaa äänitettä eteenpäin.
* `;`: Kelaa äänitettä taaksepäin.
* `Ctrl+'`: Pidennä äänitteen kelauksen aikasiirtymää.
* `Ctrl+;`: Lyhennä äänitteen kelauksen aikasiirtymää.
* `F11` (macOS: `RawCtrl+Ctrl+F` eli Ctrl+Cmd+F): Ota koko näytön tila käyttöön tai poista se käytöstä.
* `Ctrl+,`: Avaa asetukset (löytyy macOS:ää käytettäessä sovellusvalikosta).
* `Ctrl+Shift+S`: Ota uniajastin käyttöön tai poista se käytöstä.

### Ohje-valikko

* `Ctrl+F1`: Näytä Tietoa-valintaikkuna.
* `F1`: Näytä ohje oletusselaimessa.
* `Shift+F1`: Näytä ohje Paperbackissa.
* `Ctrl+Shift+U`: Tarkista päivitykset.
* `Ctrl+D`: Avaa lahjoitussivu oletusselaimessa.

### Asiakirjanäkymän lisänäppäimet

* `Delete` / `Laskinnäppäimistön Delete` välilehtisäätimessä: Sulje valittu asiakirjan välilehti.
* `Enter` tai `Väli` asiakirjan tekstissä: Aktivoi kohdistimen kohdalla oleva linkki tai avaa taulukkonäkymä, kun kohdistin on taulukossa.
* `Shift+F10` tai sovellusnäppäin asiakirjan tekstissä: Avaa pikavalikko.

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

## Muutosloki

### Versio 0.9.2
* Ruudunlukija ei enää lue äänikirjoissa peräkkäisiä välilyöntejä kohdistuksen siirtyessä kirjan tekstinäkymään.
* Äänikirjojen tiedostonimet  ilmoitetaan nyt luvuittain navigoitaessa.
* Sovellus ilmoittaa nyt äänikirjojen todellisen keston sen sijaan, että kaikkien väitettäisiin olevan 24 tunnin mittaisia
* Kun selainnäkymässä on siirrytty linkin osoittamaan kohtaan, näkymän sulkeminen Esc-näppäimellä ei aiheuta enää vianmääritysilmoitusta.
* Valitse kaikki -toiminnon jälkeinen kopiointi kattaa nyt koko asiakirjan eikä pelkkää näytettävää osaa.
* Etsi-toiminto siirtää suoraan löytämälleen riville eikä ruudunlukija enää lue koko ikkunaa uudelleen kohdistuksen palatessa kirjaan.
* Korjattu ongelma, jonka vuoksi ylimääräisen ZIP64-lohkon sisältämät EPUB-tiedostot eivät avautuneet, vaan antoivat virheilmoituksen ”Invalid local file header”.
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
* Korjattu ongelma, jonka vuoksi Viimeisimmät asiakirjat -alivalikossa säilyi vanhentuneita merkintöjä, kunnes valikon sisältö muodostettiin uudelleen.
* Valikkojen pikanäppäimet ovat palanneet kaikkiin käännöksiin, joten esimerkiksi venäjänkielistä käyttöliittymää käytettäessä valikkokohteita on taas mahdollista avata korostetuilla näppäimillä.
* Suuret CHM-asiakirjat avautuvat nyt jopa seitsemän kertaa nopeammin.
* Avatut asiakirjat rekisteröidään Windowsiin, joten ne näkyvät nyt tehtäväpalkin pikavalikossa ja Käynnistä-valikon viimeisimpien tiedostojen luettelossa.
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
* Asiakirjojen poistaminen Viimeisimmät asiakirjat -valintaikkunasta sulkee nyt myös niiden aktiiviset välilehdet.
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
* Kääntäjien työnkulkua parannettu. [#270](https://github.com/trypsynth/paperback/issues/270).
* Useita sisäisiä uudelleenjärjestelyjä, joissa suurin osa sovelluslogiikasta siirrettiin C++:sta Rustiin suorituskyvyn ja ylläpidettävyyden vuoksi.

### Versio 0.6.1
* Lisätty tuki salasanalla suojatuille PDF-tiedostoille! [#169](https://github.com/trypsynth/paperback/issues/169).
* Lisätty hyvin yksinkertainen toiminto edelliseen ja seuraavaan sijaintiin siirtymistä varten. Kun painat Enteriä sisäisen linkin kohdalla ja kohdistin siirtyy, kyseinen sijainti tallennetaan, ja siihen voi palata komennolla Alt+vasen/oikea nuolinäppäin. [#115](https://github.com/trypsynth/paperback/issues/115),
* Elementtilista lisätty. Tällä hetkellä se näyttää vain kaikkien asiakirjan otsikoiden puunäkymän tai linkkiluettelon, mutta sitä on tarkoitus laajentaa tulevaisuudessa. [#173](https://github.com/trypsynth/paperback/issues/173),
* Lisätty asetus, jolla Paperback käynnistyy oletusarvoisesti suurennettuna. [#164](https://github.com/trypsynth/paperback/issues/164),
* Korjattu joidenkin EPUB-asiakirjojen virheellisesti toimineet linkit. [#167](https://github.com/trypsynth/paperback/issues/167), [#171](https://github.com/trypsynth/paperback/pull/171), [#178](https://github.com/trypsynth/paperback/issues/178),
* Korjattu suhteellisia polkuja sisältävien EPUB-kirjojen sisällysluetteloiden jäsennys. [#187](https://github.com/trypsynth/paperback/issues/187).
* Korjattu virhe, jossa nimeä tai tekijää ei näytetty joissakin EPUB-asiakirjoissa.
* Korjattu virhe, jossa joidenkin EPUB-asiakirjojen lukujen nimet eivät näkyneet oikein sisällysluettelovalintaikkunassa.
* Korjattu virhe, jossa sisällysluettelovalintaikkunan OK- tai Peruuta-painikkeita ei voinut painaa Väli-näppäimellä. [#170](https://github.com/trypsynth/paperback/issues/170).
* Otsikoiden käsittelyä parannettu Word-asiakirjoissa.
* Paperback antaa nyt äänipalautteen yritettäessä avata "Viimeisimmät asiakirjat" -valintaikkunaa, kun luettelo on tyhjä. [#185](https://github.com/trypsynth/paperback/issues/185).

### Versio 0.6.0
* Asetukset-valintaikkunaan lisätty uusi asetus, jolla Siirry-valikko voidaan näyttää huomattavasti tiiviimmässä muodossa. Se on oletusarvoisesti käytössä.
* Lisätty asetus, jolla rakenteisten elementtien perusteella tapahtuva navigointi palaa asiakirjan lopussa takaisin alkuun.
* Työkalut-valikkoon lisätty vaihtoehto, jolla voidaan avata nykyisen asiakirjan sisältävä kansio.
* Lisätty melko yksinkertainen mutta erittäin tehokas päivitysjärjestelmä. [#28](https://github.com/trypsynth/paperback/issues/28).
* Lisätty perustason uniajastin, jonka voi avata Ctrl+Shift+S-näppäinkomennolla. [#117](https://github.com/trypsynth/paperback/issues/117),
* Lisätty FB2-e-kirjojen jäsennystuki! [#30](https://github.com/trypsynth/paperback/issues/30),
* Lisätty OpenDocument-esitysten jäsennystuki! [#105](https://github.com/trypsynth/paperback/issues/105),
* Lisätty OpenDocument-tekstitiedostojen jäsennystuki! [#29](https://github.com/trypsynth/paperback/issues/29),
* Kirjanmerkit voivat nyt kohdistua koko riville tai pelkästään valittuun tekstiin. Jos tekstiä ei ole valittuna kirjanmerkkiä luotaessa, toiminnallisuus on sama kuin ennen versiota 0.6, ja koko rivi merkitään. Mikäli tekstiä on valittuna, kirjanmerkki kohdistuu vain kyseiseen tekstiin. [#99](https://github.com/trypsynth/paperback/issues/99).
* Kirjanmerkeissä voi nyt olla valinnaisia muistiinpanoja. Siirry muistiinpanoja sisältävien kirjanmerkkien välillä N:llä ja Shift+N:llä, tai avaa kirjanmerkkien valintaikkuna, jossa kaikki kirjanmerkit, vain muistiinpanot tai vain ilman muistiinpanoja olevat kirjanmerkit voidaan valita tietyillä pikanäppäimillä. [#68](https://github.com/trypsynth/paperback/issues/68), [#128](https://github.com/trypsynth/paperback/issues/128), [#156](https://github.com/trypsynth/paperback/issues/156), [#157](https://github.com/trypsynth/paperback/issues/157), [#158](https://github.com/trypsynth/paperback/pull/158), [#159](https://github.com/trypsynth/paperback/issues/159),
* Kirjanmerkeissä ei enää ole ärsyttävää "bookmark x" -etuliitettä kirjanmerkkien valintaikkunassa. [#86](https://github.com/trypsynth/paperback/issues/86).
* XML:ltä näyttävää HTML-koodia sisältävät EPUB-kirjat käsitellään nyt oikein. [#96](https://github.com/trypsynth/paperback/issues/96).
* Suurten Markdown-asiakirjojen lataaminen korjattu. [#97](https://github.com/trypsynth/paperback/issues/97).
* Väli-näppäimen painaminen sisällysluettelon puurakenteessa ei enää paina OK-painiketta. [#121](https://github.com/trypsynth/paperback/issues/121),
* Välilyöntien käsittely korjattu pre-tagien alussa sekä HTML- että XHTML-asiakirjoissa.
* Korjattu virhe, jossa kohdistus ei siirtynyt takaisin tekstikenttään Paperbackin ikkunaan palattaessa.
* "Siirry prosenttiin" -valintaikkunan tekstikenttä päivittää nyt oikein liukusäätimen arvon.
* Mukautettujen HTML ID -tunnisteiden renderöinti korjattu Markdown-asiakirjoissa. [#113](https://github.com/trypsynth/paperback/issues/113).
* Markdown-koodilohkojen sisällä oleva HTML renderöidään nyt oikein. [#79](https://github.com/trypsynth/paperback/issues/79).
* Kun kirja ladataan komentoriviparametrilla jo käynnissä olevaan Paperback-kopioon, virheilmoitusta ei enää näytetä, vaikka dokumentin lataus kestäisi yli 5 sekuntia.
* Asetukset ladataan ja tallennetaan nyt asianmukaisesti, kun Paperback on käynnissä järjestelmänvalvojana. [#148](https://github.com/trypsynth/paperback/issues/148),
* Kirjanmerkki voidaan nyt poistaa suoraan kirjanmerkkien valintaikkunasta. [#100](https://github.com/trypsynth/paperback/issues/100),
* Asiakirjan kirjanmerkkien ja lukukohdan tuonti ja vienti on nyt mahdollista. Luotu tiedosto nimetään asiakirjan tiedostonimen perusteella ja sen tunniste on .paperback. Mikäli tällainen tiedosto löytyy ladattaessa asiakirjan kansiosta, se ladataan automaattisesti. Muussa tapauksessa voit tuoda sen manuaalisesti Työkalut-valikon toiminnolla. [#146](https://github.com/trypsynth/paperback/issues/146),
* Asiakirjojen sisäiset linkit ovat nyt täysin tuettuja. Siirry niiden välillä eteen- ja taaksepäin K- ja Shift+K-näppäimillä ja avaa tai aktivoi linkki Enterillä. [#74](https://github.com/trypsynth/paperback/issues/74), [#87](https://github.com/trypsynth/paperback/pull/87), [#126](https://github.com/trypsynth/paperback/issues/126), [#129](https://github.com/trypsynth/paperback/issues/129), [#130](https://github.com/trypsynth/paperback/issues/130).
* Tehty useita sisäisiä uudelleenjärjestelyjä, jotka nopeuttavat ohjelmaa ja pienentävät binääriä.
* Markdown-sisältö esikäsitellään nyt CommonMark-yhteensopivaksi ennen renderöintiä.
* Luetteloiden ja niiden kohteiden välillä navigointia tuetaan nyt täysin. Voit siirtyä luetteloiden välillä L- ja Shift+L-näppäimillä ja luettelokohteiden välillä I- ja Shift+I-näppäimillä. [#119](https://github.com/trypsynth/paperback/issues/119),
* Tavallisen Delete-näppäimen lisäksi myös numeronäppäimistön Deleteä voi  nyt käyttää asiakirjojen poistamiseen välilehtipalkista.
* Paperback voidaan nyt haluttaessa pienentää ilmaisinalueelle. Tämä asetus on oletusarvoisesti poissa käytöstä, mutta kun se otetaan käyttöön, Paperbackin järjestelmävalikon pienennystoiminto siirtää sovelluksen ilmaisinalueelle, josta se voidaan palauttaa napsauttamalla Paperbackin kuvaketta. [#49](https://github.com/trypsynth/paperback/issues/49),
* Paperback on nyt käännettävissä eri kielille. Sen tukemien kielten luettelo on toistaiseksi melko pieni, mutta se kasvaa jatkuvasti. [#75](https://github.com/trypsynth/paperback/issues/75), [#92](https://github.com/trypsynth/paperback/pull/92), [#95](https://github.com/trypsynth/paperback/pull/95), [#134](https://github.com/trypsynth/paperback/pull/134), [#137](https://github.com/trypsynth/paperback/pull/137), [#141](https://github.com/trypsynth/paperback/pull/141),
* Paperbackilla on nyt virallinen verkkosivusto osoitteessa [paperback.dev](https://paperback.dev).
* PPTX-asiakirjoissa olevat diat näytetään nyt yksinkertaisessa sisällysluettelossa. [#122](https://github.com/trypsynth/paperback/issues/122).
* Asiakirjan tiedot -valintaikkunassa näytetään nyt avoimen asiakirjan koko polku. [#139](https://github.com/trypsynth/paperback/issues/139),
* Asennusohjelma sisältää nyt vaihtoehdon, jolla readme-tiedosto voidaan avata selaimessa asennuksen jälkeen.
* Viimeisimpien asiakirjojen luetteloa on laajennettu huomattavasti. Sen sijaan, että se näyttäisi vain 10 viimeksi avattua asiakirjaa, näytettävä määrä on nyt mahdollista määrittää itse, ja muut aiemmin avatut asiakirjat ovat käytettävissä erillisen valintaikkunan kautta. [#78](https://github.com/trypsynth/paperback/issues/78), [#80](https://github.com/trypsynth/paperback/pull/80), [#84](https://github.com/trypsynth/paperback/pull/84),
* Useita pieniä parannuksia jäsentimiin kautta linjan, kuten tyhjän rivin lisääminen diojen väliin PPTX-esityksissä, rivinvaihtojen käsittelyn korjaaminen Word-asiakirjojen kappaleissa ja luettelokohtamerkkien lisääminen.

### Versio 0.5.0
* Lisätty Microsoft Word -asiakirjojen tuki. [#27](https://github.com/trypsynth/paperback/issues/27).
* Lisätty PowerPoint-esityksien tuki. [#25](https://github.com/trypsynth/paperback/issues/25).
* Korjattu virhe, jossa tietyt valikkokohteet eivät poistuneet käytöstä, kun yhtään asiakirjaa ei ollut avoinna.
* Korjattu "siirry prosenttiin" -liukusäätimen suunta. [#70](https://github.com/trypsynth/paperback/issues/70).
* Korjattu EPUB-kirjojen sisällysluettelot, joissa oli URL-koodattuja tiedostopolkuja ja/tai fragmenttitunnuksia.
* Korjattu ongelma, jossa XHTML-otsikoissa olevat välilyönnit poistettiin oudosti.
* Korjattu HTML-asiakirjojen sisäkkäisten pre-tagien sisällä olevien välilyöntien käsittely.
* HTML- ja Markdown-asiakirjat tukevat nyt sisällysluetteloa. Kun HTML- tai Markdown-asiakirja ladataan, Paperback muodostaa sisällysluettelon asiakirjan otsikkorakenteesta ja näyttää sen Ctrl+T-näppäinkomennolla avattavassa valintaikkunassa.
* HTML-asiakirjoissa käytetään nyt title-tagin mukaista otsikkoa, mikäli sellainen on määritetty. Muutoin käytetään edelleen tiedoston nimeä ilman tunnistetta.
* Puhumiseen käytetään UniversalSpeech-kirjaston sijaan aktiivista aluetta. Tämä tarkoittaa, ettei ohjelman mukana enää toimiteta ruudunlukijoiden DLL-tiedostoja, ja nyt tuetaan useampia ruudunlukijoita, kuten Microsoft Narratoria.
* ZIP-kirjastoja on vaihdettu, jotta voidaan avata laajempi valikoima EPUB-kirjoja. [#73](https://github.com/trypsynth/paperback/issues/73).
* Valintaikkuna, joka kysyy asiakirjan avaamista pelkkänä tekstinä, on uudistettu kokonaan ja se mahdollistaa nyt asiakirjan avaamisen pelkkänä tekstinä, HTML:nä tai Markdownina.
* "Siirry prosenttiin" -valintaikkuna sisältää nyt tekstikentän, johon voit syöttää prosenttiluvun manuaalisesti. [#66](https://github.com/trypsynth/paperback/issues/66).
* HTML-jäsennin tunnistaa nyt dd-, dt- ja dl-elementit luetteloelementeiksi.
* EPUB-kirjojen sisällysluettelot säilytetään jälleen täsmälleen sellaisina kuin ne ovat.
* Unicode-merkistöön sisältyvä ei-sitova välilyönti käsitellään nyt tyhjiä rivejä poistettaessa. [#71](https://github.com/trypsynth/paperback/issues/71).
* Sovellus kysyy tuntemattoman tiedoston avaustapaa vain ensimmäisellä kerralla, ei enää joka avauksella.

### Versio 0.4.1
* Lisätty asennusohjelmaan valinnainen Käynnistä-valikon kuvakkeen luonti.
* Sisällysluettelon pitäisi nyt olla joissakin tapauksissa siistimpi. Esimerkiksi jos ala- ja ylätason kohde sisältävät saman tekstin samassa kohdassa, nyt näytetään vain ylätason kohde.
* Korjattu tiettyjen CHM-asiakirjojen sisällysluettelot.
* Korjattu absoluuttisia tiedostopolkuja sisältävien EPUB 3 -kirjojen sisällysluettelot. [#67](https://github.com/trypsynth/paperback/issues/67).
* CHM-asiakirjojen nimien pitäisi nyt näkyä sellaisina, kuin ne on metatiedoissa määritetty.

### Versio 0.4.0
* Lisätty CHM-tiedostojen tuki! [#23](https://github.com/trypsynth/paperback/issues/23).
* Lisätty kirjanmerkkien tuki. Voit lisätä niitä asiakirjoihin rajattomasti. Siirry niiden välillä eteen- ja taaksepäin B- ja Shift+B-näppäimillä, lisää kirjanmerkki näppäinkomennolla Ctrl+Shift+B ja avaa tiettyyn kirjanmerkkiin siirtävä valintaikkuna näppäinkomennolla Ctrl+B. [#13](https://github.com/trypsynth/paperback/issues/13).
* Massamuistiversion ZIP-paketin lisäksi on nyt saatavilla asennusohjelma. Se asentaa Paperbackin Program Files -hakemistoon ja määrittää tiedostoliitokset automaattisesti. [#33](https://github.com/trypsynth/paperback/issues/33).
* BOM-merkkejä sisältävät tekstitiedostot dekoodataan nyt oikein, eikä BOM enää näy tekstin alussa.
* Tilariville lisätty paljon uutta tietoa. Se näyttää nyt nykyisen rivin, merkin ja luetun osuuden prosentteina. [#51](https://github.com/trypsynth/paperback/issues/51).
* HTML-kommentteja tai script- ja style-tagien sisältöä ei enää näytetä tekstitulosteessa.
* Jos komentorivillä annetaan suhteellinen polku, Paperback tulkitsee sen oikein.
* Prosenttisiirtymää käsitellään nyt omassa liukusäätimeen perustuvassa valintaikkunassaan, joka voidaan avata näppäinkomennolla Ctrl+Shift+G. [#57](https://github.com/trypsynth/paperback/issues/57).
* Asiakirjoille, joilla ei ole nimeä tai tekijää, asetetaan nyt aina niiden oletusarvot.
* Sijainnin tallennuslogiikka on nyt paljon älykkäämpi ja kirjoittaa levylle vain silloin, kun se on ehdottoman välttämätöntä.
* Asiakirja, joka oli aktiivisena Paperbackin sulkemishetkellä, avataan nyt uudelleen sovelluksen käynnistyessä.
* Siirry riville- ja Siirry sivulle -valintaikkunoihin syötetty tieto puhdistetaan nyt tarkemmin.
* Korjattu EPUB 3 -kirjojen sisällysluettelonavigointi, kun manifestissa on suhteellisia polkuja.

### Versio 0.3.0
* URL-koodattuja manifesteja sisältävien EPUB-kirjojen sisällysluettelot korjattu. [#34](https://github.com/trypsynth/paperback/issues/34).
* Otsikkonavigointi korjattu monitavuisia Unicode-merkkejä sisältävissä HTML-asiakirjoissa. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Korjattu wxWidgetsin regressiosta johtuva korkea suorittimen käyttöaste asiakirjoissa, joilla on pitkät nimet. [#60](https://github.com/trypsynth/paperback/issues/60).
* UTF-8-koodattujen tekstitiedostojen lataus korjattu.
* Korjattu EPUB-kirjojen sisäkkäiset sisällysluettelokohdat, jotka siirtävät kohdistimen väärään kohtaan.
* Korjattu joissakin tilanteissa ilmenevä kaatuminen sovellusta lopetettaessa. [#45](https://github.com/trypsynth/paperback/issues/45).
* Lisätty asetusvalintaikkunaan asetus, jolla otetaan rivitys käyttöön tai poistetaan se käytöstä.
* Paperbackin kehitystä varten lahjoittaminen on nyt mahdollista joko ohje-valikon uudella Lahjoita-vaihtoehdolla tai GitHub-koodivaraston pääsivun alalaidassa olevan "Sponsor this project" -linkin kautta.
* Markdown-asiakirjoilla on nyt aina nimi, ja Paperbackin pitäisi nyt pystyä lataamaan käytännössä mikä tahansa Markdown-tiedosto. [#52](https://github.com/trypsynth/paperback/issues/52).
* PDF-asiakirjoilla on nyt aina nimi, vaikka metatiedot puuttuisivat. [#56](https://github.com/trypsynth/paperback/issues/56).
* Otettu käyttöön Chromiumin käyttämä PDF-kirjasto, joka parantaa merkittävästi PDF-tiedostojen jäsennyksen luotettavuutta koko sovelluksessa. [#41](https://github.com/trypsynth/paperback/issues/41).
* Samanaikaisesti voi nyt olla käynnissä vain yksi Paperback-kopio. Jos käynnistät paperback.exe:n tiedostonimellä ohjelman jo ollessa käynnissä, kyseinen asiakirja avataan jo käynnissä olevaan kopioon.
* Voit nyt sulkea välilehtisäätimessä näkyvän asiakirjan painamalla sen kohdalla Delete-näppäintä.

### Versio 0.2.1
* "Siirry sivulle" -valintaikkunan sivunumerokentän selitteessä näytetään nyt sivujen kokonaismäärä. [#46](https://github.com/trypsynth/paperback/issues/46).
* Asiakirjan sisällöstä voidaan nyt siirtyä Sarkain-näppäimellä avoimien asiakirjojen luetteloon. [#19](https://github.com/trypsynth/paperback/issues/19).
* Korjattu virhe, jossa otsikkonavigointinäppäimet saattoivat toisinaan avata viimeisimpiä asiakirjoja, jos niitä oli tarpeeksi. [#47](https://github.com/trypsynth/paperback/issues/47).
* Paperback poistaa nyt tarpeettomat pehmeät tavuviivat tekstitulosteesta.
* Korjattu otsikkonavigointi, joka siirsi toisinaan väärän merkin kohdalle.

### Versio 0.2.0
* Lisätty tuki markdown-asiakirjoille. [#22](https://github.com/trypsynth/paperback/issues/22).
* Lisätty tuki PDF-asiakirjoille. Siihen sisältyy myös mahdollisuus sivujen välillä siirtymiseen. [#12](https://github.com/trypsynth/paperback/issues/12), [#37](https://github.com/trypsynth/paperback/issues/37).
* Lisätty otsikkonavigoinnin pikanäppäimet HTML-sisällölle, kuten EPUB-kirjoille ja Markdown-asiakirjoille. Nämä pikanäppäimet on suunniteltu toimimaan samalla tavalla kuin ruudunlukijoissa. [#3](https://github.com/trypsynth/paperback/issues/3).
* Korjattu EPUB-tiedostojen lataus, kun manifesteissa on URL-koodattuja tiedostonimiä. [#20](https://github.com/trypsynth/paperback/issues/20).
* Upotettua XHTML:ää sisältävien EPUB 3 -kirjojen avaaminen on korjattu. [#35](https://github.com/trypsynth/paperback/issues/35).
* Jos asiakirjassa ei ole sisällysluetteloa tai lukuja, vastaavia valikkokohteita ei enää vain poisteta käytöstä, vaan sen sijaan puhutaan asianmukainen ilmoitus. [#39](https://github.com/trypsynth/paperback/issues/39).
* Lisätty viimeisimpien asiakirjojen valikko. Se tallentaa tällä hetkellä 10 viimeksi avattua asiakirjaa, ja Enter-näppäimen painaminen jonkin kohteen kohdalla avaa kyseisen asiakirjan luettavaksi. [#32](https://github.com/trypsynth/paperback/issues/32).
* Etsi-valintaikkuna on kirjoitettu kokonaan uudelleen, joten sitä on nyt paljon helpompi käyttää. Siihen on lisätty myös viimeisimpien 25 haun historia sekä sääntölausekkeiden tuki. [#21](https://github.com/trypsynth/paperback/issues/21).
* Aiemmin avatut asiakirjat muistetaan nyt myös sovelluksen uudelleenkäynnistyksen jälkeen. Tämä toiminto voidaan määrittää Työkalut-valikon uudesta Asetukset-kohdasta. [#18](https://github.com/trypsynth/paperback/issues/18).
* Lisätty näppäinkomento Shift+F1, joka avaa readme-tiedoston suoraan Paperbackissa.

### Versio 0.1.0
* Ensimmäinen julkaisu.
