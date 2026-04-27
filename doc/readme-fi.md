# Paperback - versio 0.8.5

## Johdanto

Paperback on kevyt, nopea ja saavutettava e-kirjojen ja asiakirjojen lukuohjelma kaikille satunnaisista lukijoista vaativiin tehokäyttäjiin. Se on suunniteltu ruudunlukijaystävällisyyttä, suorituskykyä ja turhista ominaisuuksista riisuttua käyttökokemusta ajatellen.

## Järjestelmävaatimukset

Paperback toimii tällä hetkellä Windows 10:ssä ja 11:ssä. MacOS- ja Linux-tuki on työn alla.

## Ominaisuudet

* Täysin itsenäinen, eikä vaadi mitään ohjelmia asennettavaksi tietokoneelle ennen lukemisen aloittamista.
* Erittäin nopea jopa vanhalla laitteistolla.
* Yksinkertainen välilehtikäyttöliittymä, jonka avulla voit avata rajattoman määrän asiakirjoja.
* Tallentaa kohdistimen sijainnin jokaisessa avaamassasi asiakirjassa.
* Muistaa valinnaisesti, mitkä asiakirjat olivat avoinna ohjelmaa suljettaessa, ja avaa ne uudelleen seuraavalla käynnistyskerralla.
* Ruudunlukijakäyttäjän suunnittelema muille ruudunlukijan käyttäjille.
* Sisältää verkkoselauksen tilaa muistuttavan navigointitoiminnon, jonka avulla voit liikkua asiakirjoissa nopeasti ja helposti.
* Sisältää tehokkaan etsintävalintaikkunan, jossa on muun muassa historia ja sääntölausekkeiden tuki.
* Voidaan käyttää massamuistiversiona tai asentaa niin, että tiedostokytkennät määritetään automaattisesti.

## Tällä hetkellä tuetut tiedostotyypit

Paperback tukee seuraavia tiedostomuotoja ja tunnisteita:

* CHM-ohjetiedostot (`.chm`)
* ePub-kirjat (`.epub`)
* FB2-e-kirjat (`.fb2`)
* HTML-asiakirjat (`.htm`, `.html`, `.xhtml`)
* Markdown-asiakirjat (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word -asiakirjat (`.docx`, `.docm`)
* OpenDocument-esitykset (`.odp`, `.fodp`)
* OpenDocument-tekstiasiakirjat (`.odt`, `.fodt`)
* PDF-asiakirjat (`.pdf`)
* PowerPoint-esitykset (`.pptx`, `.pptm`)
* RTF-asiakirjat (`.rtf`)
* Teksti- ja lokitiedostot (`.txt`, `.log`)
* XML-asiakirjat (`.xml`)

## Näppäinkomennot

Paperback on suunniteltu ensisijaisesti näppäimistöllä ja ruudunlukijalla käytettäväksi. Alla on luettelo nykyisistä Näppäinkomennoista.

### Tiedosto-valikko

* `Ctrl+O`: Avaa asiakirja.
* `Ctrl+F4`: Sulje nykyinen asiakirja.
* `Ctrl+Shift+F4`: Sulje kaikki avoimet asiakirjat.
* `Ctrl+R`: Näytä "Kaikki asiakirjat" -valintaikkuna (Viimeisimmät asiakirjat -valikosta).

### Siirry-valikko

* `Ctrl+F`: Näytä Etsi-valintaikkuna.
* `F3`: Etsi seuraava.
* `Shift+F3`: Etsi edellinen.
* `Ctrl+G`: Siirry riville.
* `Ctrl+Shift+G`: Siirry prosenttiin.
* `Ctrl+P`: Siirry sivulle (kun nykyinen asiakirja tukee sitä).
* `Alt+Vasen nuoli`: Siirry taaksepäin navigointihistoriassa.
* `Alt+Oikea nuoli`: Siirry eteenpäin navigointihistoriassa.
* `[`: Edellinen osio.
* `]`: Seuraava osio.
* `Shift+H`: Edellinen otsikko.
* `H`: Seuraava otsikko.
* `Shift+1`–`Shift+6`: Edellinen otsikko tasoilla 1–6.
* `1`–`6`: Seuraava otsikko tasoilla 1–6.
* `Shift+P`: Edellinen sivu.
* `P`: Seuraava sivu.
* `Shift+B`: Edellinen kirjanmerkki.
* `B`: Seuraava kirjanmerkki.
* `Shift+N`: Edellinen muistiinpano.
* `N`: Seuraava muistiinpano.
* `Ctrl+B`: Siirry kaikkiin kirjanmerkkeihin ja muistiinpanoihin.
* `Ctrl+Alt+B`: Siirry vain kirjanmerkkeihin.
* `Ctrl+Alt+M`: Siirry vain muistiinpanoihin.
* `Ctrl+Shift+W`: Näytä muistiinpanon teksti nykyisessä kohdassa.
* `Shift+K`: Edellinen linkki.
* `K`: Seuraava linkki.
* `Shift+T`: Edellinen taulukko.
* `T`: Seuraava taulukko.
* `Shift+S`: Edellinen erotin.
* `S`: Seuraava erotin.
* `Shift+L`: Edellinen luettelo.
* `L`: Seuraava luettelo.
* `Shift+I`: Edellinen luettelokohde.
* `I`: Seuraava luettelokohde.

### Työkalut-valikko

* `Ctrl+W`: Näytä nykyisen asiakirjan sanamäärä.
* `Ctrl+I`: Näytä asiakirjan tiedot.
* `Ctrl+T`: Näytä sisällysluettelo.
* `F7`: Näytä elementtilista.
* `Ctrl+Shift+C`: Avaa asiakirjan kansio.
* `Ctrl+Shift+V`: Avaa nykyinen sisältö verkkonäkymässä.
* `Ctrl+Shift+E`: Vie asiakirjan tiedot (`.paperback`).
* `Ctrl+Shift+I`: Tuo asiakirjan tiedot (`.paperback`).
* `Ctrl+E`: Vie nykyinen asiakirja pelkkänä tekstinä.
* `Ctrl+Shift+B`: Kirjanmerkki päälle tai pois nykyisessä valinnassa tai kohdistimen sijainnissa.
* `Ctrl+Shift+N`: Lisää kirjanmerkin muistiinpano nykyisen valinnan/kohdistimen kohdalle tai muokkaa sitä.
* `Ctrl+,`: Avaa asetukset.
* `Ctrl+Shift+S`: Ota uniajastin käyttöön tai poista se käytöstä.

### Ohje-valikko

* `Ctrl+F1`: Näytä Tietoa-valintaikkuna.
* `F1`: Näytä ohje oletusselaimessa.
* `Shift+F1`: Näytä ohje Paperbackissa.
* `Ctrl+Shift+U`: Tarkista päivitykset.
* `Ctrl+D`: Avaa lahjoitussivu oletusselaimessa.

### Asiakirjanäkymän lisänäppäimet

* `Delete` / `Laskinnäppäimistön Delete` välilehtisäätimessä: Sulje valittu asiakirjan välilehti.
* `Enter` asiakirjan tekstissä: Aktivoi kohdistimen kohdalla oleva linkki tai avaa taulukkonäkymä, kun kohdistin on taulukossa.
* `Shift+F10` asiakirjan tekstissä: Avaa pikavalikko.

## Tuetut kielet

Paperback on käännetty useille eri kielille, ja uusia lisätään jatkuvasti. Täydellinen luettelo on alla.

Jos haluat osallistua kääntämiseen, katso ohjeet [käännösoppaasta](translating.md).

* bosnia
* tšekki
* ranska
* saksa
* japani
* venäjä
* yksinkertaistettu kiina
* serbia
* espanja
* vietnam

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
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Muutosloki

### Versio 0.8.5
* Lisätty alkeellinen tuki tunnisteita sisältäville PDF-tiedostoille. [#364](https://github.com/trypsynth/paperback/pull/364), [#365](https://github.com/trypsynth/paperback/pull/365).
* Lisätty sivujen tuki ePub-kirjoille. [#379](https://github.com/trypsynth/paperback/issues/379), [#380](https://github.com/trypsynth/paperback/pull/380).
* Lisätty tuki vanhoille PowerPoint-esityksille (*.ppt).
* Lisätty tuki vanhoille Microsoft Word -asiakirjoille (*.doc).
* Lisätty tuki mobi- ja AZW3-kirjoille. [#369](https://github.com/trypsynth/pull/369), [#378](https://github.com/trypsynth/pull/378).
* Lisätty tuki salatuille Microsoft Office -asiakirjoille. Tällä hetkellä tuetaan vanhaa ja uudempaa Wordia sekä uudempaa PowerPointia, ja vanhan PowerPointin tuki on tulossa myöhemmin.
* Lisätty Ctrl+Q-näppäinkomento sovelluksen lopettamista varten. [#368](https://github.com/trypsynth/paperback/issues/368).
* Lisätty tuki sekä DAISY- että Word-muodossa oleville Booksharen pakatuille kirjoille. [#36](https://github.com/trypsynth/paperback/issues/36), [#358](https://github.com/trypsynth/paperback/pull/358), [#360](https://github.com/trypsynth/paperback/pull/360).
* Upotettujen kuvien vaihtoehtoisen tekstin pitäisi nyt näkyä oikein.
* CHM-asiakirjoissa tuetaan nyt asianmukaisesti sisäisten linkkien navigointia.
* Korjattu kirjanmerkkien merkkiääni, joka ei kuulunut kirjanmerkin kohdalla vaan kappaleen alussa. [#363](https://github.com/trypsynth/paperback/issues/363).
* Korjattu "Siirry sivulle" -toiminnon virhe, jossa tietylle sivulle siirtyminen oli aina yhden numeron verran pielessä. [#389](https://github.com/trypsynth/paperback/pull/389).
* Korjattu ongelma, jossa "Avaa muodossa" -valintaikkunaa ei voinut sulkea Esc-näppäimellä.
* Korjattu lukijan pikavalikko, joka ei avautunut hiiren oikealla painikkeella eikä sovellusnäppäimellä. [#362](https://github.com/trypsynth/paperback/issues/362).
* Korjattu virhe, jossa kohdistus siirtyi toisinaan väärään asiakirjaan, kun niitä avattiin komentoriviltä.
* Pelkkiä kuvia sisältävät PDF-tiedostot tunnistetaan taas ja niiden olemassaolosta ilmoitetaan.
* Kuvien ja kuvituskuvien välillä on nyt mahdollista liikkua G/Shift+G- ja F/Shift+F-näppäimillä.
* Paperback noudattaa nyt sovelluksen "Tumma tila" -asetusta.
* Poistettu DAISY XML -tuki, koska sitä ei enää tarvita.
* Palattu käyttämään alkuperäistä Win32:n ensimmäisen kirjaimen navigointia sisällysluettelopuussa.
* Virheenkäsittelyikkuna näyttää nyt yksityiskohtaisempia virheilmoituksia.
* Verkkonäkymä avautuu nyt paljon nopeammin ja sulavammin. [#359](https://github.com/trypsynth/paperback/pull/359).

### Versio 0.8.2
* Lisätty sivujen tuki RTF-asiakirjoille.
* Korjattu virhe, jossa verkkonäkymän avaaminen aktivoi automaattisesti ePubien sisältämät ulkoiset linkit.
* Korjattu virhe, jossa RTF-jäsennin ei lisännyt joissakin harvinaisissa tapauksissa sanaväliä sanojen väliin.
* Korjattu kappaleet, jotka jakautuivat useiksi lyhyiksi riveiksi joissakin PDF-asiakirjoissa. [#101](https://github.com/trypsynth/paperback/issues/101), [#355](https://github.com/trypsynth/paperback/pull/355).
* PDF-asiakirjoissa on nyt perustason linkki- ja otsikkonavigointi. [#291](https://github.com/trypsynth/paperback/issues/291), [#353](https://github.com/trypsynth/paperback/pull/353), [#354](https://github.com/trypsynth/paperback/pull/354).
* RTF:n sarkaimet ja rivinvaihdot piirretään nyt täsmälleen niin kuin ne näkyvät asiakirjassa.
* Palattu käyttämään luotettavaa pdfium-kirjastoa PDF-tiedostojen jäsentämiseen, mikä tekee PDF-renderöinnistä jälleen paljon luotettavampaa.

### Versio 0.8.1
* Lisätty Ctrl+Shift+T viimeksi suljetun asiakirjan uudelleenavaamista varten. [#343](https://github.com/trypsynth/paperback/issues/343).
* Kaikki asiakirjat -valintaikkuna tukee nyt kerralla useiden avattavien asiakirjojen valintaa. [#344](https://github.com/trypsynth/paperback/issues/344).
* Korjattu muutamia RTF-jäsentimen virheitä. [#345](https://github.com/trypsynth/paperback/issues/345).
* Korjattu muita kuin ASCII-merkkejä (kuten bosnian š, č, ć ja ž) sisältävät tiedostopolut, jotka vioittuivat, kun tiedosto avattiin toisen Paperback-kopion kautta. [#346](https://github.com/trypsynth/paperback/issues/346).
* Korjattu PDF-tekstin lukujärjestys sekä virheellinen sanaväli isolla kirjaimella alkavien sanojen ympärillä. [#44](https://github.com/trypsynth/paperback/issues/44).
* Korjattu hitaat asiakirjojen lataukset suuria tiedostoja avattaessa.
* Korjattu vahvistusvalintaikkunoiden Kyllä- ja Ei-painikkeiden lokalisointi. [#285](https://github.com/trypsynth/paperback/issues/285).

### Versio 0.8.0
* Lisätty japanin, yksinkertaistetun kiinan ja vietnamin käännökset! [#300](https://github.com/trypsynth/paperback/pull/300), [#326](https://github.com/trypsynth/paperback/pull/326), [#335](https://github.com/trypsynth/paperback/pull/335).
* Lisätty automaattinen päivittäjä, joka korvaa nyt nykyisen asennetun Paperback-version sen sijaan, että vain lataisi uuden version. [#323](https://github.com/trypsynth/paperback/pull/323).
* Lisätty valinnainen äänipalaute kirjanmerkin tai muistiinpanon kohdalle siirtymisestä. Kiitos Andre Louis'lle äänistä! [#110](https://github.com/trypsynth/paperback/issues/110).
* Lisätty RTF-asiakirjojen tuki. [#26](https://github.com/trypsynth/paperback/issues/26).
* Lisätty tuki DAISY XML -asiakirjoille. [#136](https://github.com/trypsynth/paperback/issues/136).
* Lisätty tuki Flat Open Document Text -tiedostoille.
* Lisätty tuki Flat Open Document -esityksille.
* Lisätty tuki erottimiin siirtymiselle S- ja Shift+S-näppäimillä. [#294](https://github.com/trypsynth/paperback/issues/294).
* Kaikki yli 300 merkin pituiset siirtymät lisätään nyt automaattisesti navigointihistoriaan. [#179](https://github.com/trypsynth/paperback/issues/179).
* Korjattu Paperbackin ikkunan palautus järjestelmäpalkista. [#284](https://github.com/trypsynth/paperback/issues/284).
* Korjattu Markdown-asiakirjojen näyttäminen verkkonäkymässä raakatekstinä renderöidyn HTML:n sijaan.
* Korjattu Markdown-tiedostojen sisältämien taulukoiden virheellinen renderöinti. [#303](https://github.com/trypsynth/paperback/issues/303).
* Paperback varoittaa nyt yritettäessä avata pelkkiä kuvia sisältäviä PDF-tiedostoja. [#89](https://github.com/trypsynth/paperback/issues/89).
* Päivityksiä haettaessa on nyt mahdollista tarkistaa uudet kehitysversiot vakaiden julkaisujen sijaan. [#333](https://github.com/trypsynth/paperback/pull/333).
* Versiotiedot upotetaan nyt oikein Paperbackin sovellustiedostoon.
* Asetusvalintaikkuna jaettu välilehtiin käytön ja navigoinnin helpottamiseksi.
* Siirrytty käyttämään Hayro-kirjastoa PDF-tiedostojen jäsentämiseen, mikä parantaa luotettavuutta, nopeutta ja vähentää DLL-tiedostojen määrää.
* Koko sovellus on uudelleenkirjoitettu Rust-ohjelmointikielellä. Uusi koodipohja on turvallisempi, lataa asiakirjat nopeammin ja sitä on helpompi ylläpitää ja laajentaa.
* Sisältöä näyttävän elementin pikavalikossa on nyt lukusovellukselle ominaisia komentoja eikä yleisiä toimintoja, kuten Leikkaa tai Liitä. [#114](https://github.com/trypsynth/paperback/issues/114).

### Versio 0.7.0
* Lisätty taulukoiden tuki HTML- ja XHTML-pohjaisille asiakirjoille. Liiku taulukoiden välillä T:llä ja Shift+T:llä ja avaa taulukko verkkonäkymässä painamalla Enter. [#81](https://github.com/trypsynth/paperback/issues/81), [#98](https://github.com/trypsynth/paperback/pull/98), [#226](https://github.com/trypsynth/paperback/pull/226), [#228](https://github.com/trypsynth/paperback/pull/228).
* Lisätty perustason verkkorenderöinti. Avaa asiakirjan nykyinen osio verkkopohjaisessa renderöijässä painamalla Ctrl+Shift+V. Tästä on hyötyä esimerkiksi monimutkaisessa muotoilussa tai koodiesimerkeissä. [#188](https://github.com/trypsynth/paperback/issues/188), [#239](https://github.com/trypsynth/paperback/pull/239).
* Lisätty venäjänkielinen käännös. Kiitos Ruslan Gulmagomedoville. [#211](https://github.com/trypsynth/paperback/pull/211), [#212](https://github.com/trypsynth/paperback/pull/212).
* Lisätty "Tyhjennä kaikki" -painike Kaikki asiakirjat -valintaikkunaan. [#217](https://github.com/trypsynth/paperback/issues/217).
* Päivitysten tarkistaja näyttää nyt julkaisutiedot, kun uusi versio on saatavilla. [#210](https://github.com/trypsynth/paperback/pull/210).
* Korjattu ikkunan palautus ilmaisinalueelta. [#284](https://github.com/trypsynth/paperback/issues/284).
* Korjattu Kyllä/Ei-painikkeiden käännökset vahvistusvalintaikkunoissa. [#285](https://github.com/trypsynth/paperback/issues/285).
* Korjattu asetusten lataus, kun ohjelmaa ajetaan järjestelmänvalvojana.
* Korjattu kommenttien käsittely XML- ja HTML-asiakirjoissa. [#198](https://github.com/trypsynth/paperback/issues/198).
* Korjattu sisällysluettelon jäsennys ePub 2 -kirjoissa. [#192](https://github.com/trypsynth/paperback/pull/192).
* Korjattu siirtyminen sisällysluettelon seuraavaan samalla kirjaimella alkavaan kohteeseen.
* Korjattu Etsi-valintaikkuna, jota ei aina piilotettu oikein Seuraava/Edellinen-painikkeita käytettäessä.
* Korjattu virhe, jossa ePub-kirjojen sisällysluettelot siirsivät toisinaan väärään kohtaan.
* Korjattu useita välilyöntien käsittelyyn liittyviä ongelmia XML-, HTML- ja pre-tageissa.
* Korjattu virhe, jossa linkkien välillä siirtyminen oli yhden kohdan verran pielessä.
* Korjattu joissakin kirjoissa esiintynyt rivien loppuun jäävä ylimääräinen välilyönti.
* Korjattu useita jäsentimen ongelmia. [#208](https://github.com/trypsynth/paperback/pull/208).
* Kirjanmerkkeihin liittyvät valikkokohdat sekä elementtilista poistetaan nyt käytöstä asianmukaisesti, kun yhtään asiakirjaa ei ole avoinna.
* Parannettu luetteloiden käsittelyä useissa asiakirjamuodoissa. [#213](https://github.com/trypsynth/paperback/pull/213).
* Kääntäjien työnkulkua parannettu. [#270](https://github.com/trypsynth/paperback/issues/270).
* Useita sisäisiä uudelleenjärjestelyjä, joissa suurin osa sovelluslogiikasta siirrettiin C++:sta Rustiin suorituskyvyn ja ylläpidettävyyden vuoksi.

### Versio 0.6.1
* Lisätty tuki salasanalla suojatuille PDF-tiedostoille! [#169](https://github.com/trypsynth/paperback/issues/169).
* Lisätty hyvin yksinkertainen toiminto edelliseen ja seuraavaan sijaintiin siirtymistä varten. Kun painat Enteriä sisäisen linkin kohdalla ja kohdistin siirtyy, kyseinen sijainti tallennetaan, ja siihen voi palata komennolla Alt+vasen/oikea nuolinäppäin. [#115](https://github.com/trypsynth/paperback/issues/115), [#174](https://github.com/trypsynth/paperback/pull/174).
* Elementtilista lisätty. Tällä hetkellä se näyttää vain kaikkien asiakirjan otsikoiden puunäkymän tai linkkiluettelon, mutta sitä on tarkoitus laajentaa tulevaisuudessa. [#173](https://github.com/trypsynth/paperback/issues/173), [#177](https://github.com/trypsynth/paperback/pull/177).
* Lisätty asetus, jolla Paperback käynnistyy oletusarvoisesti suurennettuna. [#164](https://github.com/trypsynth/paperback/issues/164), [#172](https://github.com/trypsynth/paperback/pull/172).
* Korjattu joidenkin ePub-asiakirjojen virheellisesti toimineet linkit. [#167](https://github.com/trypsynth/paperback/issues/167), [#171](https://github.com/trypsynth/paperback/pull/171), [#178](https://github.com/trypsynth/paperback/issues/178), [#180](https://github.com/trypsynth/paperback/pull/180).
* Korjattu suhteellisia polkuja sisältävien ePub-sisällysluetteloiden jäsennys. [#187](https://github.com/trypsynth/paperback/issues/187).
* Korjattu virhe, jossa nimeä tai tekijää ei näytetty joissakin ePub-asiakirjoissa.
* Korjattu virhe, jossa joidenkin ePub-asiakirjojen lukujen nimet eivät näkyneet oikein sisällysluettelovalintaikkunassa. [#176](https://github.com/trypsynth/paperback/pull/176).
* Korjattu virhe, jossa sisällysluettelovalintaikkunan OK- tai Peruuta-painikkeita ei voinut painaa Väli-näppäimellä. [#170](https://github.com/trypsynth/paperback/issues/170).
* Otsikoiden käsittelyä parannettu Word-asiakirjoissa. [#183](https://github.com/trypsynth/paperback/pull/183).
* Paperback antaa nyt äänipalautteen yritettäessä avata "Viimeisimmät asiakirjat" -valintaikkunaa, kun luettelo on tyhjä. [#185](https://github.com/trypsynth/paperback/issues/185).

### Versio 0.6.0
* Asetukset-valintaikkunaan lisätty uusi asetus, jolla Siirry-valikko voidaan näyttää huomattavasti tiiviimmässä muodossa. Se on oletusarvoisesti käytössä.
* Lisätty asetus, jolla rakenteisten elementtien perusteella tapahtuva navigointi palaa asiakirjan lopussa takaisin alkuun. [#116](https://github.com/trypsynth/paperback/pull/116).
* Työkalut-valikkoon lisätty vaihtoehto, jolla voidaan avata nykyisen asiakirjan sisältävä kansio. [#142](https://github.com/trypsynth/paperback/pull/142).
* Lisätty melko yksinkertainen mutta erittäin tehokas päivitysjärjestelmä. [#28](https://github.com/trypsynth/paperback/issues/28).
* Lisätty perustason uniajastin, jonka voi avata Ctrl+Shift+S-näppäinkomennolla. [#117](https://github.com/trypsynth/paperback/issues/117), [#118](https://github.com/trypsynth/paperback/pull/118).
* Lisätty FB2-e-kirjojen jäsennystuki! [#30](https://github.com/trypsynth/paperback/issues/30), [#107](https://github.com/trypsynth/paperback/pull/107).
* Lisätty OpenDocument-esitysten jäsennystuki! [#105](https://github.com/trypsynth/paperback/issues/105), [#106](https://github.com/trypsynth/paperback/pull/106).
* Lisätty OpenDocument-tekstitiedostojen jäsennystuki! [#29](https://github.com/trypsynth/paperback/issues/29), [#90](https://github.com/trypsynth/paperback/pull/90).
* Kirjanmerkit voivat nyt kohdistua koko riville tai pelkästään valittuun tekstiin. Jos tekstiä ei ole valittuna kirjanmerkkiä luotaessa, toiminnallisuus on sama kuin ennen versiota 0.6, ja koko rivi merkitään. Mikäli tekstiä on valittuna, kirjanmerkki kohdistuu vain kyseiseen tekstiin. [#99](https://github.com/trypsynth/paperback/issues/99).
* Kirjanmerkeissä voi nyt olla valinnaisia tekstimuistiinpanoja. Siirry muistiinpanoja sisältävien kirjanmerkkien välillä N:llä ja Shift+N:llä, tai avaa kirjanmerkkien valintaikkuna, jossa kaikki kirjanmerkit, vain muistiinpanot tai vain ilman muistiinpanoja olevat kirjanmerkit voidaan valita tietyillä pikanäppäimillä. [#68](https://github.com/trypsynth/paperback/issues/68), [#128](https://github.com/trypsynth/paperback/issues/128), [#156](https://github.com/trypsynth/paperback/issues/156), [#157](https://github.com/trypsynth/paperback/issues/157), [#158](https://github.com/trypsynth/paperback/pull/158), [#159](https://github.com/trypsynth/paperback/issues/159), [#161](https://github.com/trypsynth/paperback/pull/161).
* Kirjanmerkeissä ei enää ole ärsyttävää "bookmark x" -etuliitettä kirjanmerkkien valintaikkunassa. [#86](https://github.com/trypsynth/paperback/issues/86).
* XML:ltä näyttävää HTML-koodia sisältävät ePub-kirjat käsitellään nyt oikein. [#96](https://github.com/trypsynth/paperback/issues/96).
* Suurten Markdown-asiakirjojen lataaminen korjattu. [#97](https://github.com/trypsynth/paperback/issues/97).
* Väli-näppäimen painaminen sisällysluettelon puurakenteessa ei enää paina OK-painiketta. [#121](https://github.com/trypsynth/paperback/issues/121), [#123](https://github.com/trypsynth/paperback/pull/123).
* Välilyöntien käsittely korjattu pre-tagien alussa sekä HTML- että XHTML-asiakirjoissa.
* Korjattu virhe, jossa kohdistus ei siirtynyt takaisin tekstikenttään Paperbackin ikkunaan palattaessa.
* "Siirry prosenttiin" -valintaikkunan tekstikenttä päivittää nyt oikein liukusäätimen arvon.
* Mukautettujen HTML ID -tunnisteiden renderöinti korjattu Markdown-asiakirjoissa. [#113](https://github.com/trypsynth/paperback/issues/113).
* Markdown-koodilohkojen sisällä oleva HTML renderöidään nyt oikein. [#79](https://github.com/trypsynth/paperback/issues/79).
* Kun kirja ladataan komentoriviparametrilla jo käynnissä olevaan Paperback-kopioon, virheilmoitusta ei enää näytetä, vaikka dokumentin lataus kestäisi yli 5 sekuntia.
* Asetukset ladataan ja tallennetaan nyt asianmukaisesti, kun Paperback on käynnissä järjestelmänvalvojana. [#148](https://github.com/trypsynth/paperback/issues/148), [#149](https://github.com/trypsynth/paperback/pull/149).
* Kirjanmerkki voidaan nyt poistaa suoraan kirjanmerkkien valintaikkunasta. [#100](https://github.com/trypsynth/paperback/issues/100), [#103](https://github.com/trypsynth/paperback/pull/103).
* Asiakirjan kirjanmerkkien ja lukukohdan tuonti ja vienti on nyt mahdollista. Luotu tiedosto nimetään asiakirjan tiedostonimen mukaan ja sen pääte on .paperback. Mikäli tällainen tiedosto löytyy ladattaessa asiakirjan kansiosta, se ladataan automaattisesti. Muussa tapauksessa voit tuoda sen manuaalisesti Työkalut-valikon toiminnolla. [#146](https://github.com/trypsynth/paperback/issues/146), [#147](https://github.com/trypsynth/paperback/pull/147).
* Asiakirjojen sisäiset linkit ovat nyt täysin tuettuja. Siirry niiden välillä eteen- ja taaksepäin K- ja Shift+K-näppäimillä ja avaa tai aktivoi linkki Enterillä. [#74](https://github.com/trypsynth/paperback/issues/74), [#87](https://github.com/trypsynth/paperback/pull/87), [#126](https://github.com/trypsynth/paperback/issues/126), [#129](https://github.com/trypsynth/paperback/issues/129), [#130](https://github.com/trypsynth/paperback/issues/130).
* Tehty useita sisäisiä uudelleenjärjestelyjä, jotka nopeuttavat ohjelmaa ja pienentävät binääriä.
* Markdown-sisältö esikäsitellään nyt CommonMark-yhteensopivaksi ennen renderöintiä.
* Luetteloiden ja niiden kohteiden välillä navigointia tuetaan nyt täysin. Voit siirtyä luetteloiden välillä L- ja Shift+L-näppäimillä ja luettelokohteiden välillä I- ja Shift+I-näppäimillä. [#119](https://github.com/trypsynth/paperback/issues/119), [#124](https://github.com/trypsynth/paperback/pull/124).
* Tavallisen Delete-näppäimen lisäksi myös numeronäppäimistön Deleteä voi  nyt käyttää asiakirjojen poistamiseen välilehtipalkista.
* Paperback voidaan nyt haluttaessa pienentää ilmaisinalueelle. Tämä asetus on oletusarvoisesti poissa käytöstä, mutta kun se otetaan käyttöön, Paperbackin järjestelmävalikon pienennystoiminto siirtää sovelluksen ilmaisinalueelle, josta se voidaan palauttaa napsauttamalla Paperbackin kuvaketta. [#49](https://github.com/trypsynth/paperback/issues/49), [#85](https://github.com/trypsynth/paperback/pull/85).
* Paperback on nyt käännettävissä eri kielille. Sen tukemien kielten luettelo on toistaiseksi melko pieni, mutta se kasvaa jatkuvasti. [#75](https://github.com/trypsynth/paperback/issues/75), [#92](https://github.com/trypsynth/paperback/pull/92), [#95](https://github.com/trypsynth/paperback/pull/95), [#134](https://github.com/trypsynth/paperback/pull/134), [#137](https://github.com/trypsynth/paperback/pull/137), [#141](https://github.com/trypsynth/paperback/pull/141), [#152](https://github.com/trypsynth/paperback/pull/152).
* Paperbackilla on nyt virallinen verkkosivusto osoitteessa [paperback.dev](https://paperback.dev).
* PPTX-asiakirjoissa olevat diat näytetään nyt yksinkertaisessa sisällysluettelossa. [#122](https://github.com/trypsynth/paperback/issues/122).
* Asiakirjan tiedot -valintaikkunassa näytetään nyt avoimen asiakirjan koko polku. [#139](https://github.com/trypsynth/paperback/issues/139), [#140](https://github.com/trypsynth/paperback/pull/140).
* Asennusohjelma sisältää nyt vaihtoehdon, jolla readme-tiedosto voidaan avata selaimessa asennuksen jälkeen.
* Viimeisimpien asiakirjojen luetteloa on laajennettu huomattavasti. Sen sijaan, että se näyttäisi vain 10 viimeksi avattua asiakirjaa, näytettävä määrä on nyt mahdollista määrittää itse, ja muut aiemmin avatut asiakirjat ovat käytettävissä erillisen valintaikkunan kautta. [#78](https://github.com/trypsynth/paperback/issues/78), [#80](https://github.com/trypsynth/paperback/pull/80), [#84](https://github.com/trypsynth/paperback/pull/84), [#135](https://github.com/trypsynth/paperback/pull/135).
* Useita pieniä parannuksia jäsentimiin kautta linjan, kuten tyhjän rivin lisääminen diojen väliin PPTX-esityksissä, rivinvaihtojen käsittelyn korjaaminen Word-asiakirjojen kappaleissa ja luettelokohtamerkkien lisääminen.

### Versio 0.5.0
* Lisätty Microsoft Word -asiakirjojen tuki. [#27](https://github.com/trypsynth/paperback/issues/27).
* Lisätty PowerPoint-esityksien tuki. [#25](https://github.com/trypsynth/paperback/issues/25).
* Korjattu virhe, jossa tietyt valikkokohteet eivät poistuneet käytöstä, kun yhtään asiakirjaa ei ollut avoinna.
* Korjattu "siirry prosenttiin" -liukusäätimen suunta. [#70](https://github.com/trypsynth/paperback/issues/70).
* Korjattu ePub-kirjojen sisällysluettelot, joissa oli URL-koodattuja tiedostopolkuja ja/tai fragmenttitunnuksia.
* Korjattu ongelma, jossa XHTML-otsikoissa olevat välilyönnit poistettiin oudosti.
* Korjattu HTML-asiakirjojen sisäkkäisten pre-tagien sisällä olevien välilyöntien käsittely.
* HTML- ja Markdown-asiakirjat tukevat nyt sisällysluetteloa. Kun HTML- tai Markdown-asiakirja ladataan, Paperback muodostaa sisällysluettelon asiakirjan otsikkorakenteesta ja näyttää sen Ctrl+T-näppäinkomennolla avattavassa valintaikkunassa.
* HTML-asiakirjoissa käytetään nyt title-tagin mukaista otsikkoa, mikäli sellainen on määritetty. Muutoin käytetään edelleen tiedoston nimeä ilman päätettä.
* Puhumiseen käytetään UniversalSpeech-kirjaston sijaan aktiivista aluetta. Tämä tarkoittaa, ettei ohjelman mukana enää toimiteta ruudunlukijoiden DLL-tiedostoja, ja nyt tuetaan useampia ruudunlukijoita, kuten Microsoft Narratoria.
* ZIP-kirjastoja on vaihdettu, jotta voidaan avata laajempi valikoima ePub-kirjoja. [#73](https://github.com/trypsynth/paperback/issues/73).
* Valintaikkuna, joka kysyy asiakirjan avaamista pelkkänä tekstinä, on uudistettu kokonaan ja se mahdollistaa nyt asiakirjan avaamisen pelkkänä tekstinä, HTML:nä tai Markdownina.
* "Siirry prosenttiin" -valintaikkuna sisältää nyt tekstikentän, johon voit syöttää prosenttiluvun manuaalisesti. [#66](https://github.com/trypsynth/paperback/issues/66).
* HTML-jäsennin tunnistaa nyt dd-, dt- ja dl-elementit luetteloelementeiksi.
* ePub-kirjojen sisällysluettelot säilytetään jälleen täsmälleen sellaisina kuin ne ovat.
* Unicode-merkistöön sisältyvä ei-sitova välilyönti käsitellään nyt tyhjiä rivejä poistettaessa. [#71](https://github.com/trypsynth/paperback/issues/71).
* Sovellus kysyy tuntemattoman tiedoston avaustapaa vain ensimmäisellä kerralla, ei enää joka avauksella.

### Versio 0.4.1
* Lisätty asennusohjelmaan valinnainen Käynnistä-valikon kuvakkeen luonti.
* Sisällysluettelon pitäisi nyt olla joissakin tapauksissa siistimpi. Esimerkiksi jos ala- ja ylätason kohde sisältävät saman tekstin samassa kohdassa, nyt näytetään vain ylätason kohde.
* Korjattu tiettyjen CHM-asiakirjojen sisällysluettelot.
* Korjattu absoluuttisia tiedostopolkuja sisältävien ePub 3 -kirjojen sisällysluettelot. [#67](https://github.com/trypsynth/paperback/issues/67).
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
* Korjattu ePub 3 -kirjojen sisällysluettelonavigointi, kun manifestissa on suhteellisia polkuja.

### Versio 0.3.0
* URL-koodattuja manifesteja sisältävien ePub-kirjojen sisällysluettelot korjattu. [#34](https://github.com/trypsynth/paperback/issues/34).
* Otsikkonavigointi korjattu monitavuisia Unicode-merkkejä sisältävissä HTML-asiakirjoissa. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Korjattu wxWidgetsin regressiosta johtuva korkea suorittimen käyttöaste asiakirjoissa, joilla on pitkät nimet. [#60](https://github.com/trypsynth/paperback/issues/60).
* UTF-8-koodattujen tekstitiedostojen lataus korjattu.
* Korjattu ePub-kirjojen sisäkkäiset sisällysluettelokohdat, jotka siirtävät kohdistimen väärään kohtaan.
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
* Lisätty otsikkonavigoinnin pikanäppäimet HTML-sisällölle, kuten ePub-kirjoille ja Markdown-asiakirjoille. Nämä pikanäppäimet on suunniteltu toimimaan samalla tavalla kuin ruudunlukijoissa. [#3](https://github.com/trypsynth/paperback/issues/3).
* Korjattu ePub-tiedostojen lataus, kun manifesteissa on URL-koodattuja tiedostonimiä. [#20](https://github.com/trypsynth/paperback/issues/20).
* Upotettua XHTML:ää sisältävien ePub 3 -kirjojen lataaminen korjattu. [#35](https://github.com/trypsynth/paperback/issues/35).
* Jos asiakirjassa ei ole sisällysluetteloa tai osioita, vastaavia valikkokohteita ei enää vain poisteta käytöstä, vaan sen sijaan puhutaan asianmukainen ilmoitus. [#39](https://github.com/trypsynth/paperback/issues/39).
* Lisätty viimeisimpien asiakirjojen valikko. Se tallentaa tällä hetkellä 10 viimeksi avattua asiakirjaa, ja Enter-näppäimen painaminen jonkin kohteen kohdalla avaa kyseisen asiakirjan luettavaksi. [#32](https://github.com/trypsynth/paperback/issues/32).
* Etsi-valintaikkuna on kirjoitettu kokonaan uudelleen, joten sitä on nyt paljon helpompi käyttää. Siihen on lisätty myös viimeisimpien 25 haun historia sekä sääntölausekkeiden tuki. [#21](https://github.com/trypsynth/paperback/issues/21).
* Aiemmin avatut asiakirjat muistetaan nyt myös sovelluksen uudelleenkäynnistyksen jälkeen. Tämä toiminto voidaan määrittää Työkalut-valikon uudesta Asetukset-kohdasta. [#18](https://github.com/trypsynth/paperback/issues/18).
* Lisätty näppäinkomento Shift+F1, joka avaa readme-tiedoston suoraan Paperbackissa.

### Versio 0.1.0
* Ensimmäinen julkaisu.
