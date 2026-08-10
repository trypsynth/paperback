<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Pehmeäkantinen - versio 0.8.5 {#paperback---version-0.8.5}

## Johdanto {#introduction}

Paperback on kevyt, nopea ja helppokäyttöinen e-kirjojen ja asiakirjojen
lukija kaikille, satunnaisista lukijoista aktiivisimpiin käyttäjiin. Se
on suunniteltu näytönlukijoiden kanssa käytettäväksi, nopeaksi ja
turhista ominaisuuksista vapaaksi.

## Järjestelmävaatimukset {#system-requirements}

Paperback toimii tällä hetkellä Windowsissa, macOS:ssa, iOS:ssa ja
Androidissa.

## Ominaisuudet {#features}

-   Täysin itsenäinen sovellus, joka ei vaadi minkään ohjelmiston
    asentamista tietokoneellesi lukemisen aloittamiseksi.
-   Uskomattoman nopea, jopa vanhalla laitteistolla.
-   Yksinkertainen välilehtipohjainen käyttöliittymä, jonka avulla voit
    avata niin monta asiakirjaa kuin haluat vierekkäin.
-   Tallentaa tarkan lukukohdan jokaisessa avaamassasi asiakirjassa.
-   Voit valita, muistetaanko mitkä asiakirjat olivat auki, kun suljit
    ohjelman, ja palautetaanko ne seuraavalla käynnistyskerralla.
-   Sisältää navigointitoimintoja, jotka muistuttavat monien
    näytönlukijoiden verkkoselausmoodia, jotta asiakirjoissa voi liikkua
    nopeasti ja helposti. Sisältää tehokkaan hakudialogin, jossa on muun
    muassa hakuhistoria
-   Sisältää tehokkaan hakudialogin, jossa on muun muassa hakuhistoria
    ja säännöllisten lausekkeiden tuki.
-   Voidaan käyttää täysin siirrettävänä tai asentaa niin, että
    tiedostoyhdistykset määritetään automaattisesti.
-   Tukee laajaa valikoimaa yleisiä tiedostomuotoja.

## Yhteensopivuus näytönlukijoiden kanssa {#screen-reader-compatibility}

Paperback toimii hyvin kaikkien tärkeimpien näytönlukijoiden kanssa. On
kuitenkin yksi tunnettu ongelma JAWS-käyttäjille.

### JAWS ja pistekirjoitusnäytöt {#jaws-and-braille-displays}

Jos käytät JAWSia pistekirjoitusnäytön kanssa, saatat huomata, että
pitkät kappaleet katkeavat, kun siirryt eteenpäin näytön
navigointinäppäimillä. Tämä vaikuttaa myös nykyisen kappaleen
lukemiskomentoon. Kyseessä on virhe JAWS:n RICHEDIT50W-tekstikentän
käsittelyssä, ei mitään Paperbackissa itsessään, ja sen korjaaminen
kesti melko kauan, vaikka Vispero onkin innokas vastaamaan avoimen
lähdekoodin ohjelmistojen ongelmiin.

Kiertotapa, joka lopulta löytyi JAWS-keskusteluryhmästä kuukausien
odottelun jälkeen, on muokata `paperback.jcf` ja asettaa "Braille-esitys
ja panorointi" -asetukseksi "Käytä aina DOM:ia, jos saatavilla".
Kannattaa myös ottaa käyttöön "Panoroi tekstiä kappaleittain", muuten
näyttö pysyy aktiivisessa kappaleessa sen sijaan, että etenisi. Kun
molemmat asetukset ovat käytössä, panorointi pitäisi toimia oikein.

## Tällä hetkellä tuetut tiedostotyypit Paperback tukee seuraavia tiedostomuotoja ja laajennuksia: {#currently-supported-file-types}

Paperback tukee seuraavia formaatteja ja tiedostotunnisteita:

-   CHM-ohjetiedostot (`.chm`)
-   DAISY-kirjat (`.opf`, `.zip`)
-   EPUB-kirjat (`.epub`)
-   FB2-e-kirjat (`.fb2`)
-   HTML-asiakirjat (`.htm`, `.html`, `.xhtml`)
-   Markdown-asiakirjat (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Microsoft Word -asiakirjat (`.docx`, `.docm`, `.doc`)
-   MOBI/Kindle-kirjat (`.mobi`, `.azw`, `.azw3`)
-   OpenDocument-esitykset (`.odp`, `.fodp`)
-   OpenDocument-tekstitiedostot (`.odt`, `.fodt`)
-   PDF-asiakirjat (`.pdf`)
-   PowerPoint-esitykset (`.pptx`, `.pptm`, `.ppt`)
-   RTF-asiakirjat (`.rtf`)
-   Pelkkä teksti ja lokitiedostot (`.txt`, `.log`)

## Pikanäppäimet {#keyboard-shortcuts}

Paperback on suunniteltu ensisijaisesti näppäimistöllä käytettäväksi.
Tässä ovat nykyiset pikanäppäimet.

Alla olevat pikanäppäimet koskevat Windowsia. Jos macOS eroaa näistä,
vastaava pikanäppäin on merkitty sulkeisiin --- pääasiassa siksi, että
Ctrl+G, Ctrl+W ja Alt+Vasen/Oikea ovat jo varattuja muiden järjestelmä-
tai sovelluskonventioiden vuoksi kyseisellä alustalla.

### Tiedosto-valikko {#file-menu}

-   `Ctrl+O`: Avaa asiakirja.
-   `Ctrl+F4` (macOS: `Cmd+W`): Sulje nykyinen asiakirja.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Sulje kaikki avoimet
    asiakirjat.
-   `Ctrl+Shift+T`: Avaa viimeksi suljettu asiakirja uudelleen.
-   `Ctrl+R`: Näytä "Kaikki asiakirjat" -valintaikkuna (kohdasta
    Viimeisimmät asiakirjat).
-   `Ctrl+Q`: Poistu (vain Windows; macOS:ssa tämä löytyy
    sovellusvalikosta).

### Siirry-valikko {#go-menu}

-   `Ctrl+F`: Näytä Etsi-valintaikkuna.
-   `F3` (macOS: `Cmd+G`): Etsi seuraava.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Etsi edellinen.
-   `Ctrl+G` (macOS: `Cmd+L`): Siirry riville.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Siirry prosenttiin.
-   `Ctrl+P`: Siirry sivulle (jos nykyinen asiakirja tukee tätä
    toimintoa).
-   `Alt+Left` (macOS: `Cmd+[`): Palaa taaksepäin selaushistoriassa.
-   `Alt+Right` (macOS: `Cmd+]`): Siirry eteenpäin selaushistoriassa.
-   `[`: Edellinen osio.
-   `]`: Seuraava osio.
-   `Shift+H`: Edellinen otsikko.
-   `H`: Seuraava otsikko.
-   `Shift+1` koko `Shift+6`: Edellinen otsikko tasoilla 1--6.
-   `1` lähdön kautta `6`: Seuraava otsikko tasolla 1--6.
-   `Shift+P`: Edellinen sivu.
-   `P`: Seuraava sivu.
-   `Shift+B`: Edellinen kirjanmerkki.
-   `B`: Seuraava kirjanmerkki.
-   `Shift+N`: Edellinen muistiinpano.
-   `N`: Seuraava muistiinpano.
-   `Ctrl+B`: Siirry kaikkiin kirjanmerkkeihin ja muistiinpanoihin.
-   `Ctrl+Alt+B`: Siirry vain kirjanmerkkeihin.
-   `Ctrl+Alt+M`: Siirry vain muistiinpanoihin.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, eli fyysinen
    Control-näppäin Cmd:n sijaan): Näytä muistiinpanon teksti nykyisessä
    kohdassa.
-   `Shift+K`: Edellinen linkki.
-   `K`: Seuraava linkki.
-   `Shift+G`: Edellinen kuva.
-   `G`: Seuraava kuva.
-   `Shift+F`: Edellinen kuva.
-   `F`: Seuraava kuva.
-   `Shift+T`: Edellinen taulukko.
-   `T`: Seuraava taulukko.
-   `Shift+S`: Edellinen erottimerkki.
-   `S`: Seuraava erottimerkki.
-   `Shift+L`: Edellinen luettelo.
-   `L`: Seuraava luettelo.
-   `Shift+I`: Edellinen luettelokohta.
-   `I`: Seuraava luettelokohta.
-   `Shift+,`: Siirry nykyisen säilön (luettelon tai taulukon) alkuun.
-   `,`: Siirry nykyisen säilön (luettelon tai taulukon) lopun ohi.

### Työkalut-valikko {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, eli fyysinen Control-näppäin Cmd:n
    sijaan): Näytä nykyisen asiakirjan sanamäärä.
-   `Ctrl+I`: Näytä asiakirjan tiedot.
-   `Ctrl+T`: Näytä sisällysluettelo.
-   `F7`: Näytä elementtiluettelo.
-   `Ctrl+Shift+C`: Avaa sisältökansio.
-   `Ctrl+Shift+V`: Avaa nykyinen sisältö Web View -näkymässä.
-   `Ctrl+U`: Näytä asiakirjan lähdekoodi uudessa välilehdessä.
-   `Ctrl+Shift+E`: Vie asiakirjan tiedot (`.paperback`).
-   `Ctrl+Shift+I`: Tuo asiakirjan tiedot (`.paperback`).
-   `Ctrl+E`: Vie nykyinen asiakirja tekstimuotoon.
-   `Ctrl+Shift+B`: Lisää tai poista kirjanmerkki nykyisen
    valinnan/kohdistimen kohdalle.
-   `Ctrl+Shift+N`: Lisää tai muokkaa kirjanmerkkimuistiinpanoa nykyisen
    valinnan/kohdistimen kohdalla.
-   `Ctrl+Alt+W`: Kytke sananvaihto päälle tai pois päältä.
-   `Ctrl+,`: Avaa asetukset (macOS: Asetukset, sovelluksen valikossa).
-   `Ctrl+Shift+S`: Kytke lepotila päälle tai pois päältä.

### Ohje-valikko {#help-menu}

-   `Ctrl+F1`: Näytä Tietoja-valintaikkuna.
-   `F1`: Näytä ohje oletusselaimessasi.
-   `Shift+F1`: Näytä ohje Paperback-sovelluksessa.
-   `Ctrl+Shift+U`: Tarkista päivitykset.
-   `Ctrl+D`: Avaa lahjoitussivu oletusselaimessasi .

### Lisäpainikkeet asiakirjan katseluun välilehtien hallinnassa: {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` välilehtivalikossa: Sulje valittu
    asiakirjavälilehti.
-   `Enter` tai `Space` asiakirjan tekstissä: Aktivoi linkin kohdistimen
    kohdalla tai avaa taulukkonäkymä, kun olet taulukon merkinnän
    kohdalla.
-   `Shift+F10` tai Menu/Application-näppäin asiakirjan tekstissä: Avaa
    kontekstivalikko.

## Tuetut kielet {#supported-languages}

Paperback on käännetty monille eri kielille, ja uusia kieliä lisätään
jatkuvasti. Täydellinen luettelo on alla.

Jos haluat tietää, miten voit osallistua, lue
[käännösoppaamme](translating.md).

-   Bosnia
-   Tšekki
-   hollanti
-   Suomi
-   Ranska
-   saksalainen
-   japani
-   puola
-   portugali (Brasilia)
-   Venäjä
-   Yksinkertaistettu kiina
-   serbia
-   espanja
-   Vietnam

## Tekijätiedot {#credits}

### Kehitys {#development}

-   Quin Gillespie: pääkehittäjä ja projektin perustaja.
-   Aryan Choudhary: pääasiallinen avustaja.

### Lahjoitukset {#donations}

Seuraavat henkilöt ovat tehneet jonkin verran lahjoituksia Paperbackin
kehitykseen. Jos teet lahjoituksen, nimesi ei tule automaattisesti
lisättyä tähän luetteloon; lisään luetteloon vain ne henkilöt, jotka
haluavat lahjoituksensa julkistettavan.

Huomautus: Pidän julkista GitHub-sponsorointia perusteena
automaattiselle lisäämiselle tähän luetteloon.

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

## Muutosloki {#changelog}

### Versio 0.9.0 (julkaisematon) {#version-0.9.0-unreleased}

-   Lisättiin peruutuspainike käynnissä olevaan päivitykseen liittyvään
    valintaikkunaan.
-   Lisättiin pb-niminen komentorivityökalu, jolla voi nopeasti muuntaa
    minkä tahansa Paperbackin tukeman muodon HTML:ksi, Markdowniksi tai
    pelkäksi tekstiksi.
-   Lisättiin konfiguroitava pikanäppäin Paperbackin palauttamiseksi
    ilmaisinalueelta.
-   Lisättiin Etsi-painike Kaikki asiakirjat -valintaikkunaan, jotta
    voidaan etsiä puuttuvia kirjoja, joiden polku on juuri muuttunut.
-   Lisättiin asetusvalintaikkunaan "Luettavuus"-välilehti, jossa on
    seuraavat asetukset:
    -   Sanojen rivinvaihto (siirretty yleisistä asetuksista);
    -   Taulukoiden renderointi rivien sisällä (uusi tässä versiossa,
        katso alla);
    -   Fontti;
    -   Taustaväri;
    -   Riviväli;
    -   Kappaleiden väli;
    -   Kirjainväli;
    -   Tekstin tasaus.
-   Lisättiin kytkin, jolla voi määrittää taulukoiden näyttötavan, ja
    yhdenmukaistettiin taulukoiden näyttötapa kaikissa asiakirjoissa.
-   Lisätty Näytä lähdekoodi -vaihtoehto, jolla asiakirjan lähdekoodi
    avautuu uuteen välilehteen, mikä on hyödyllistä esimerkiksi
    Markdownin muokkaamisessa.
-   Lisättiin arvioitu lukuaika sanamäärän valintaikkunaan sekä
    mahdollisuus asettaa lukunopeutesi, jotta tästä mittarista tulee
    todella hyödyllinen.
-   Lisätty ARM64-tuki Windowsille!
-   Lisätty Android-tuki!
-   Lisätty iOS-tuki!
-   Lisätty tuki macOS:lle!
-   Lisätty uusia kieliä: hollanti, suomi ja puola.
-   Lisätty tuki konttikohtaiseen navigointiin.
-   Lisätty tuki luetteloille, luettelokohteille, kuvioille ja kuville
    CHM- asiakirjoissa.
-   Lisätty sananvaihtovalikkokohta ja siihen liittyvä pikanäppäin.
-   Kirjanmerkki- ja muistiinpanoäänet toistuvat nyt oikein vain
    silloin, kun hiiri viedään sanan päälle, jossa niitä on.
-   Vanhoilla CJK-koodauksilla, kuten GBK, Big5 ja Shift_JIS, koodatut
    asiakirjat näkyvät nyt oikein sen sijaan, että ne näkyisivät
    joukkona mojibake-merkkejä.
-   Vienti-valikkokohtaa on laajennettu, jotta asiakirjoja voidaan viedä
    HTML- ja Markdown-muotoon tavallisen tekstin lisäksi.
-   Korjattu ongelma, jossa sananvaihdon käyttäminen siirsi käyttäjän
    asiakirjan alkuun.
-   Korjattu ongelma, jossa Daisy-kirjoissa näkyi virheellistä tietoa
    tilapalkissa.
-   Korjattu ongelma, jossa dl-, dt- ja dd-elementit eivät tuottaneet
    rivinvaihtoja XHTML- asiakirjoissa.
-   Korjattu virhe, jossa Escape-näppäin ei sulkenut Asiakirjan tiedot-
    ja Kaikki asiakirjat - valintaikkunoita.
-   Korjattu ongelma, jossa Mobi-kirjojen filepos-ankkurit jakivat
    HTML-tunnisteita ja lisäsivät roskatietoa kirjan tekstiin.
-   Korjattu viive, joka ilmeni suurten asiakirjojen tekstikentän
    loppupuolella.
-   Korjattu linkit vanhoissa Mobi-kirjoissa.
-   Korjattu DAISY-kirjojen lataaminen virheellisillä
    koodausmäärittelyillä.
-   Korjattu sivunavigoinnin virheellinen rivitekstin ilmoitus joissakin
    tilanteissa.
-   Korjattu RTF-asiakirjojen jäsentäminen, kun niissä on muita kuin
    latinalaisia merkkejä.
-   Korjattu ongelma, jossa "Avaa viimeksi suljettu" -toiminto yritti
    avata uudelleen mukana toimitetun readme-tiedoston.
-   Korjattu ongelma, jossa otsikkopalkki ei päivittynyt, kun asiakirja
    suljettiin "Kaikki asiakirjat" -valintaikkunasta.
-   Korjattu, että webview-valintaikkunan kokoa ei voinut muuttaa ja se
    avautui aluksi hyvin pienessä koossa.
-   Korjattu ongelma, jossa Word-asiakirjoissa, joissa oli
    kielikohtaisia tyylinimiä, otsikot eivät näkyneet oikein.
-   Korjattu ongelma, jossa valitulle välilehdelle ei siirtynyt
    kohdistus oikein Paperbackin uudelleenkäynnistyksen jälkeen.
-   Jos valinta on aktiivinen, kun avaat sanamäärävalintaikkunan,
    valitsemiesi sanojen määrä näkyy nyt.
-   Kuvat pitäisi nyt näkyä oikein upotetussa webview-näkymässä.
-   Parannettu Paperbackin tiedostojen käsittelyä
    Windows-verkkoasemilla: "Näytä tiedosto kansiossa" -painikkeen
    painaminen siirtää fokuksen nyt oikein tiedostoon verkkoasemalla,
    eikä poluissa enää esiinny outoja merkkejä.
-   AZW3-tiedostojen jäsentämistä on parannettu merkittävästi.
-   Siirryttiin chmlib-kirjastosta omaan, puhtaasti Rust-kielellä
    kirjoitettuun CHM-tiedostojen lukijaan.
-   Työpöydällä .paperback-tiedostoja ei enää ladata pakotetusti
    asiakirjan palautuksen yhteydessä. Sen sijaan sinulta kysytään
    vahvistusta, kun tiedosto löytyy.
-   Paperback käyttää nyt varajärjestelmänä pelkkää tekstiä
    virheellisesti merkittyjen PDF-tiedostojen poiminnassa.
-   Kansion avaaminen tuo nyt kyseisen tiedoston esiin
    tiedostoselaimessa.
-   Readme-tiedoston avaaminen noudattaa nyt valitsemaasi kieltä.
-   PowerPoint-asiakirjat tukevat nyt taulukoita.
-   Päivitä valikko oikein ja aseta kohdistus tekstikenttään, kun
    avataan ohje Paperbackissa.
-   Readme.html-tiedostoa ei enää lisätä Kaikki asiakirjat -luetteloon,
    kun se avataan Shift+F1-näppäinyhdistelmällä.
-   Asiakirjojen poistaminen Viimeisimmät-valintaikkunasta sulkee nyt
    myös niiden aktiivisen välilehden.
-   Siirryttiin Windowsissa huomattavasti turvallisempaan
    IPC-menetelmään.
-   Aktiivisen asiakirjan otsikko luetaan nyt, kun vaihdetaan
    välilehtien välillä.
-   Päivitysohjelma näyttää nyt oikein markdown-kooditunnisteiden
    sisällön julkaisutiedoissa.
-   Päivitysohjelma varmistaa nyt, ettei ladattua tiedostoa ole
    peukaloitu .
-   Web-näkymä avautuu nyt nykyisessä lukukohdassasi.
-   Kaikki asiakirjat -valintaikkunan hakusuodatin säilyy nyt asiakirjan
    poistamisen jälkeen.

### Versio 0.8.5 {#version-0.8.5}

-   EPUB-kirjoille on lisätty sivutuki.
-   Lisätty tuki salatuille Microsoft Office -asiakirjoille. Tällä
    hetkellä tuetaan vanhaa Wordia, uutta Wordia ja uutta PowerPointia,
    ja vanhan PowerPointin tuki on suunniteltu tulevaisuudessa.
-   Lisätty tuki vanhoille Microsoft Word -asiakirjoille (\*.doc)!
-   Lisätty tuki vanhoille PowerPoint-esityksille (\*.ppt)!
-   Lisätty tuki mobi- ja AZW3-kirjoille!
-   Lisätty tuki merkityille PDF-tiedostoille!
-   Lisätty Ctrl+Q-pikanäppäin sovelluksen sulkemiseen.
-   Lisätty tuki Booksharen pakatuille kirjoille (sekä DAISY- että
    Word-muodossa)!
-   Upotettujen kuvien vaihtoehtoinen teksti pitäisi nyt näkyä oikein.
-   CHM-asiakirjat tukevat nyt oikein sisäisten linkkien selaamista.
-   Korjattu ongelma, jossa kirjanmerkkien äänet laukeivat kappaleen
    alussa sen sijaan, että ne laukeaisivat kirjanmerkin kohdalla.
-   Korjattu, että "Siirry sivulle" -toiminto oli 1 sivua väärässä.
-   Korjattu virhe, jossa Esc-näppäin ei sulkenut "Avaa nimellä"
    -valintaikkunaa.
-   Korjattu lukijan kontekstivalikon näkymättömyys hiiren oikealla
    painikkeella tai Sovellukset-näppäimellä.
-   Korjattu virhe, jossa joskus valittiin väärä asiakirja avattaessa
    asiakirjoja komentoriviltä.
-   Pelkästään kuvia sisältävät PDF-tiedostot tunnistetaan jälleen, ja
    niiden olemassaolosta ilmoitetaan.
-   Nyt on mahdollista selata kuvia ja kaavioita näppäinyhdistelmillä
    g/shift+g ja f/shift+f.
-   Paperback noudattaa nyt sovelluksen tumman tilan asetusta.
-   DAISY XML -tuki on poistettu, koska sitä ei enää tarvita.
-   Palattiin takaisin alkuperäiseen Win32-tyyppiseen ensimmäisen
    kirjaimen selaukseen sisällysluettelopuussa.
-   Virheen latausikkunassa näkyy nyt yksityiskohtaisempia
    virheilmoituksia. Webview avautuu nyt paljon nopeammin ja
    sujuvammin.
-   Webview avautuu nyt paljon nopeammin ja sujuvammin.

### Versio 0.8.2 {#version-0.8.2}

-   Lisätty sivutuki RTF-dokumenteille!
-   Korjattu virhe, jossa webview-näkymän avaaminen ulkoisia linkkejä
    sisältävissä epub-tiedostoissa aktivoi ne automaattisesti.
-   Korjattu virhe, jossa RTF-jäsennin ei harvinaisissa tapauksissa
    lisännyt välilyöntiä sanojen väliin .
-   Korjattu virhe, jossa kappaleet jakautuivat useiksi lyhyiksi
    riveiksi joissakin PDF- tiedostoissa.
-   PDF-tiedostoissa on nyt peruslinkkien ja otsikoiden selaamisen tuki!
-   RTF-sarkaimet ja rivinvaihdot näytetään nyt täsmälleen sellaisina
    kuin ne näkyvät asiakirjassa.
-   Palattiin takaisin luotettavaan pdfium-kirjastoon PDF-tiedostojen
    jäsentämiseen, mikä tekee PDF-tiedostojen renderoinnista jälleen
    paljon luotettavampaa.

### Versio 0.8.1 {#version-0.8.1}

-   Lisätty Ctrl+Shift+T-näppäinyhdistelmä viimeisimmän suljetun
    asiakirjan avaamiseksi uudelleen.
-   Kaikki asiakirjat -valintaikkuna tukee nyt useiden asiakirjojen
    valitsemista avattavaksi kerralla.
-   Korjattu muutamia RTF-jäsennelijän virheitä.
-   Korjattu ongelma, jossa ASCII-merkistöön kuulumattomia merkkejä
    (kuten bosnialaiset š, č, ć, ž) sisältävät tiedostopolut
    vioittuivat, kun tiedosto avattiin toisen Paperback- istunnon
    kautta.
-   Korjattu PDF-tekstin lukeminen väärässä järjestyksessä sekä
    virheelliset välit isolla alkavien sanojen ympärillä.
-   Korjattu asiakirjojen hidas latautuminen suuria tiedostoja
    avattaessa.
-   Korjattu "Kyllä/Ei"-painikkeiden lokalisointi
    vahvistusvalintaikkunoissa .

### Versio 0.8.0 {#version-0.8.0}

-   Lisätty japanin-, yksinkertaistetun kiinan- ja vietnamin käännökset!
-   Lisätty automaattinen päivitysohjelma, joka nyt korvaa nykyisen
    asennetun Paperback-version sen sijaan, että se vain lataisi uuden
    version!
-   Lisätty valinnainen äänipalaute kirjanmerkin tai muistiinpanon
    saavuttamisesta, kiitos äänistä Andre Louis!
-   Lisätty tuki RTF-tiedostoille!
-   Lisätty tuki DAISY XML -tiedostoille.
-   Lisätty tuki Flat Open Document -tekstitiedostoille!
-   Lisätty tuki Flat Open Document -esityksille!
-   Lisätty tuki erottimille, joita käytetään näppäimillä s ja shift+s.
-   Kaikki yli 300 merkin siirrot lisätään nyt automaattisesti
    navigointihistoriaasi.
-   Korjattu Paperback-ikkunan palauttaminen tehtäväpalkista.
-   Korjattu ongelma, jossa Markdown-asiakirjat näyttivät raakatekstiä
    renderoidun HTML:n sijaan Web View -näkymässä.
-   Korjattu taulukoiden virheellinen renderointi Markdown-tiedostoissa.
-   Pelkästään kuvia sisältävät PDF-tiedostot ilmoittavat nyt niiden
    olemassaolosta, kun yrität ladata sellaisen.
-   Päivityksiä tarkistettaessa on nyt mahdollista etsiä uusia
    kehitysversioita vakaiden julkaisujen sijaan.
-   Versiótiedot on upotettu oikein Paperback-suoritusohjelmaan.
-   Asetukset-valintaikkuna on jaettu välilehtiin käytön ja navigoinnin
    helpottamiseksi.
-   Siirryttiin Hayroon PDF-tiedostojen jäsentämisessä, mikä parantaa
    luotettavuutta ja nopeutta sekä vähentää DLL-tiedostojen määrää.
-   Koko sovellus on kirjoitettu uudelleen Rust-kielellä. Uusi
    koodipohja on turvallisempi, lataa asiakirjat nopeammin ja on
    helpompi ylläpitää ja laajentaa.
-   Tekstikentän kontekstivalikko sisältää nyt lukijakohtaisia
    toimintoja yleisten toimintojen, kuten leikkaa ja liitä, sijaan.

### Versio 0.7.0 {#version-0.7.0}

-   Lisätty taulukoiden tuki HTML- ja XHTML-pohjaisille asiakirjoille!
    Siirry taulukoiden välillä näppäimillä T ja Shift+T, ja paina
    Enter-näppäintä tarkastellaksesi taulukkoa web-näkymässä.
-   Lisätty perusominaisuus verkkonäyttöön! Paina Ctrl+Shift+V
    avataksesi asiakirjan nykyisen osan verkkopohjaisessa
    näyttöohjelmassa, mikä on hyödyllistä esimerkiksi monimutkaisen
    muotoilun tai koodiesimerkkien kaltaiselle sisällölle.
-   Lisätty venäjänkielinen käännös, kiitos Ruslan Gulmagomedov!
-   Lisätty Tyhjennä kaikki -painike Kaikki asiakirjat -valintaikkunaan.
-   Päivitystarkistaja näyttää nyt julkaisutiedot, kun uusi versio on
    saatavilla.
-   Korjattu ikkunan palauttaminen tehtäväpalkista.
-   Korjattu Kyllä/Ei-painikkeiden käännökset
    vahvistusvalintaikkunoissa.
-   Korjattu asetusten lataaminen, kun ohjelmaa ajetaan
    järjestelmänvalvojana.
-   Korjattu kommenttien käsittely XML- ja HTML-asiakirjoissa.
-   Korjattu sisällysluettelon jäsentäminen Epub 2 -kirjoissa.
-   Korjattu siirtyminen seuraavaan samaa kirjainta sisältävään kohtaan
    sisällysluettelossa .
-   Korjattu ongelma, jossa hakudialogi ei piiloutunut oikein, kun
    käytettiin Seuraava/Edellinen-painikkeita.
-   Korjattu ongelma, jossa epub-sisällysluettelot ohjasivat toisinaan
    väärään kohtaan.
-   Korjattu erilaisia välilyöntien käsittelyongelmia XML-, HTML- ja
    pre- tunnisteissa.
-   Korjattu off-by-one-virhe linkkien selauksessa.
-   Korjattu ongelma, jossa joidenkin kirjojen riveissä oli ylimääräisiä
    välilyöntejä rivin lopussa.
-   Korjattiin erilaisia jäsennelyongelmia.
-   Kirjanmerkkeihin liittyvät valikkokohdat sekä elementtiluettelo ovat
    nyt oikein pois käytöstä, kun yhtään asiakirjaa ei ole auki.
-   Parannettu luetteloiden käsittelyä eri asiakirjamuodoissa.
-   Parannettu käännöstyönkulkua avustajille.
-   Tehtiin useita sisäisiä uudelleenkirjoituksia, joissa suurin osa
    sovelluksen liiketoimintalogiikasta siirrettiin C++:sta
    Rust-kielelle suorituskyvyn ja ylläpidettävyyden parantamiseksi.

### Versio 0.6.1 {#version-0.6.1}

-   Lisätty salasanasuojattujen PDF-tiedostojen tuki!
-   Lisätty hyvin yksinkertainen siirry edelliseen/seuraavaan kohtaan
    -toiminto. Jos painat Enter-näppäintä sisäisen linkin kohdalla ja se
    siirtää kohdistinta, kyseinen sijainti tallennetaan nyt muistiin, ja
    sinne voi siirtyä Alt+vasen/oikea nuolinäppäimillä.
-   Lisätty elementtiluettelo! Tällä hetkellä se näyttää vain
    puurakenteen kaikista asiakirjasi otsikoista tai linkkiluettelon,
    mutta suunnitelmissa on laajentaa sitä tulevaisuudessa.
-   Lisätty vaihtoehto, jolla Paperback käynnistyy oletuksena
    maksimoidussa tilassa.
-   Korjattu joidenkin Epub-asiakirjojen linkkien toimimattomuus.
-   Korjattu suhteisia polkuja sisältävien Epub-sisällysluetteloiden
    jäsentäminen.
-   Korjattu ongelma, jossa joissakin ePub-asiakirjoissa ei näkynyt
    otsikkoa tai tekijää.
-   Korjattu ongelma, jossa joidenkin ePub-lukujen otsikot eivät
    näkyneet oikein sisällysluettelo-valintaikkunassa.
-   Korjattu ongelma, jossa välilyöntinäppäintä ei voinut käyttää
    OK/Peruuta-painikkeiden aktivoimiseen
    sisällysluettelo-valintaikkunassa.
-   Parannettu otsikoiden käsittelyä Word-tiedostoissa.
-   Saat nyt äänipalautteen, jos viimeisimpien asiakirjojen luettelo on
    tyhjä, kun yrität avata valintaikkunan.

### Versio 0.6.0 {#version-0.6.0}

-   Uusi vaihtoehto, joka näyttää Siirry-valikon huomattavasti
    kompaktimmassa muodossa, on lisätty asetusvalintaikkunaan, ja se on
    valittuna oletuksena.
-   Lisätty asetus, jolla rakenteellisten elementtien avulla tapahtuva
    navigointi kiertää sivun reunoja.
-   Työkalut-valikkoon on lisätty vaihtoehto, jolla avataan parhaillaan
    valittuna olevan asiakirjan sisältävä kansio.
-   Lisätty melko yksinkertainen, mutta erittäin tehokas
    päivitysjärjestelmä.
-   Lisätty peruslepotimeri, jota voi käyttää näppäinyhdistelmällä
    Ctrl+Shift+S.
-   Lisätty tuki FB2-e-kirjojen jäsentämiselle!
-   Lisätty tuki OpenDocument-esitysten jäsentämiselle!
-   Lisätty tuki OpenDocument-tekstitiedostojen jäsentämiselle!
-   Kirjanmerkkejä voidaan nyt luoda koko rivin merkitsemiseksi tai vain
    tietyn tekstin merkitsemiseksi. Jos valintaa ei ole aktiivisena
    kirjanmerkkiä luotaessa, toiminta on sama kuin versioissa ennen 0.6,
    ja se merkitsee koko rivin. Jos kuitenkin valitset tekstiä, vain
    kyseinen teksti sisällytetään kirjanmerkkiin.
-   Kirjanmerkkeihin voi nyt liittää valinnaisia tekstimuistiinpanoja!
    Voit siirtyä muistiinpanoja sisältävien kirjanmerkkien välillä
    näppäimillä N ja Shift+N tai avata kirjanmerkkivalintaikkunan, jossa
    näkyvät kaikki kirjanmerkit, vain muistiinpanot tai vain
    muistiinpanoja sisältämättömät kirjanmerkit, tiettyjen
    pikanäppäinten avulla.
-   Kirjanmerkkivalintaikkunan kirjanmerkeissä ei enää ole ärsyttävää
    "kirjanmerkki x" -etuliitettä.
-   HTML-sisältöä sisältävät Epub-kirjat, jotka esittävät olevansa
    XML-muotoisia, käsitellään nyt oikein.
-   Korjattu suurten Markdown-dokumenttien lataaminen.
-   Korjattu ongelma, jossa sisällysluettelon puunäkymässä välilyönnin
    painaminen aktivoi OK-painikkeen.
-   Korjattu välilyöntien käsittely pre-tagien alussa sekä HTML- että
    XHTML-dokumenteissa.
-   Korjattu ongelma, jossa tekstikenttä ei aina saanut fokusta takaisin
    palattaessa Paperbackin ikkunaan.
-   Korjattu ongelma, jossa "Siirry prosenttiin" -valintaikkunan
    tekstikenttä ei päivittänyt liukusäätimen arvoa.
-   Korjattu mukautettujen HTML-tunnisteiden renderointi
    Markdown-dokumenteissa.
-   Markdown-koodilohkojen sisällä oleva HTML-koodi renderöidään nyt
    oikein.
-   Jos lataat kirjan komentoriviparametrilla, kun olemassa oleva
    Paperback-istunto on käynnissä, et enää saa virheilmoitusta, jos
    asiakirjan lataaminen kestää yli 5 sekuntia.
-   Jos Paperbackia ajetaan järjestelmänvalvojana, asetukset ladataan ja
    tallennetaan nyt oikein.
-   Kirjanmerkkiä on nyt mahdollista poistaa suoraan
    kirjanmerkkivalintaikkunasta.
-   Nyt on mahdollista tuoda ja viedä kirjanmerkkejä sekä lukukohtaa
    tietyn asiakirjan osalta. Luotu tiedosto nimetään tiedoston mukaan
    ja siihen lisätään .paperback-tiedostotunniste. Jos tällainen
    tiedosto löytyy samasta hakemistosta kuin ladattava tiedosto, se
    ladataan automaattisesti . Muussa tapauksessa voit tuoda ne
    manuaalisesti Työkalut-valikon kohdan avulla.
-   Asiakirjojen sisäiset linkit tuetaan nyt täysin! Käytä k- ja
    shift+k-näppäimiä liikkumiseen eteen- ja taaksepäin linkkien välillä
    ja paina Enter-näppäintä avataksesi/aktivoidaksesi yhden.
-   Monet sisäiset uudistukset, jotka nopeuttavat sovellusta ja
    pienentävät binaaritiedostoa.
-   Markdown-sisältö esikäsitellään nyt, jotta se on CommonMark-
    yhteensopiva ennen renderointia.
-   Luetteloiden ja niiden kohteiden avulla tapahtuva navigointi on nyt
    täysin tuettu! Käytä L-näppäintä ja Shift+L-näppäinyhdistelmää
    siirtyäksesi luetteloiden välillä sekä I-näppäintä ja
    Shift+I-näppäinyhdistelmää siirtyäksesi luettelokohteiden välillä.
-   Numeronäppäimistön Delete-näppäin poistaa nyt asiakirjoja
    välilehtipalkista normaalin poistotoiminnon lisäksi.
-   Paperback voidaan nyt haluttaessa minimoida tehtäväpalkkiin! Tämä
    vaihtoehto on oletusarvoisesti pois päältä, mutta sen kytkeminen
    päälle saa järjestelmävalikon minimointitoiminnon siirtämään
    Paperbackin tehtäväpalkkiin, josta se voidaan palauttaa
    napsauttamalla avautuvaa kuvaketta.
-   Paperback on nyt täysin käännettävissä! Sen tukemien kielten
    luettelo on tällä hetkellä melko pieni, mutta se kasvaa jatkuvasti!
-   Paperbackilla on nyt virallinen verkkosivusto osoitteessa
    [paperback](https://paperback.dev).[dev](https://paperback.dev)!
-   PPTX-asiakirjoissa näkyy nyt yksinkertainen sisällysluettelo, joka
    sisältää kaikki diat.
-   Avatun asiakirjan täydellinen polku näkyy nyt asiakirjan
    tietodialogissa.
-   Asennusohjelma sisältää nyt vaihtoehdon, jolla voit tarkastella
    readme-tiedostoa selaimessasi asennuksen jälkeen.
-   Viimeisimpien asiakirjojen luetteloa on laajennettu huomattavasti!
    Sen sijaan, että se näyttäisi vain 10 viimeksi avattua asiakirjaa,
    se näyttää nyt määritettävän määrän asiakirjoja, ja kaikki muut
    koskaan avaamasi asiakirjat ovat saatavilla pienen valintaikkunan
    kautta.
-   Erilaisia pieniä parannuksia jäsennimiin kautta linjan, mukaan
    lukien tyhjän rivin lisääminen PPTX-esitysten diojen väliin,
    rivinvaihtojen käsittelyn korjaaminen Word-asiakirjojen kappaleiden
    sisällä sekä luettelokohtien lisääminen luettelokohtiin.

### Versio 0.5.0 {#version-0.5.0}

-   Lisätty tuki Microsoft Word -asiakirjoille!
-   Lisätty tuki PowerPoint-esityksille!
-   Korjattu ongelma, jossa tiettyjä valikkokohtia ei voitu poistaa
    käytöstä, kun asiakirjoja ei ollut auki.
-   Korjattu prosenttiliukusäätimen suunta.
-   Korjattu sisällysluettelo Epub-kirjoissa, joissa on URL-koodattuja
    tiedostopolkuja ja/tai fragmenttitunnisteita.
-   Korjattu ongelma, jossa välilyöntejä poistettiin XHTML-otsikoista
    epätavallisella tavalla.
-   Korjattu välilyöntien käsittely HTML-asiakirjojen sisäkkäisissä
    pre-tunnisteissa .
-   HTML- ja Markdown-asiakirjat tukevat nyt sisällysluettelo-
    ominaisuutta! Kun lataat HTML-/Markdown-asiakirjan, Paperback luo
    oman sisällysluettelonsa asiakirjasi otsikoiden rakenteen
    perusteella ja näyttää sen sinulle Ctrl+T-valintaikkunassa.
-   HTML-dokumenteissa käytetään nyt title-tagissa määritettyä otsikkoa,
    jos sellainen on olemassa. Muussa tapauksessa käytetään edelleen
    tiedostonimeä ilman tiedostopääte.
-   Siirryttiin UniversalSpeechistä käyttämään live-aluetta puheen
    toistamiseen. Tämä tarkoittaa, että ohjelman mukana ei enää
    toimiteta näytönlukijoiden DLL-tiedostoja, ja nyt tuetaan useampia
    näytönlukijoita, kuten Microsoft Narratoria.
-   Zip-kirjastoja on vaihdettu, jotta voidaan avata laajempi valikoima
    epub- kirjoja.
-   Valintaikkuna, jossa kysytään, haluatko avata asiakirjan pelkkänä
    tekstinä, on uusittu kokonaan, ja sen avulla voit nyt avata
    asiakirjan pelkkänä tekstinä, HTML-muodossa tai Markdown-muodossa.
-   "Siirry prosenttiin" -valintaikkunassa on nyt tekstikenttä, johon
    voit syöttää manuaalisesti prosenttiluvun, johon haluat siirtyä.
-   HTML-jäsennin tunnistaa nyt dd-, dt- ja dl-elementit luettelon
    elementeiksi.
-   EPUB-kirjojen sisällysluettelo säilyy jälleen täsmälleen ennallaan.
-   Unicode-kiinteä välilyönti otetaan nyt huomioon, kun tyhjiä rivejä
    poistetaan.
-   Sinulta ei enää kysytä, miten haluat avata tunnistamatonta tiedostoa
    joka kerta, kun lataat sen, vaan vain ensimmäisellä kerralla.

### Versio 0.4.1 {#version-0.4.1}

-   Asennusohjelmaan on lisätty valinnainen aloitusvalikkokuvake.
-   Sisällysluettelo on nyt joissakin tapauksissa selkeämpi; esimerkiksi
    jos samassa kohdassa on ala- ja ylätason kohde, joiden teksti on
    sama, näkyy nyt vain ylätason kohde.
-   Sisällysluetteloa on korjattu tietyissä CHM-dokumenteissa.
-   Korjattu sisällysluettelo Epub 3 -kirjoissa, joissa on absoluuttisia
    polkuja .
-   CHM-tiedostojen otsikot pitäisi nyt näkyä metatietotiedostossa
    määritetyn mukaisesti .

### Versio 0.4.0 {#version-0.4.0}

-   Lisätty CHM-tiedostojen tuki!
-   Lisätty kirjanmerkkien tuki! Voit luoda niin monta kirjanmerkkiä
    niin moniin asiakirjoihin kuin haluat. Voit siirtyä niissä eteen- ja
    taaksepäin näppäimillä b ja shift+b, luoda kirjanmerkin näppäimillä
    control+shift+b ja avata valintaikkunan, josta voit siirtyä tiettyyn
    kirjanmerkkiin näppäimillä control+b.
-   Lisätty asennusohjelma kannettavan zip-tiedoston rinnalle!
    Asennusohjelma asentaa Paperbackin Program Files -hakemistoon ja
    määrittää tiedostoyhdistelmät automaattisesti puolestasi.
-   BOM-merkinnällä varustetut tekstitiedostot pitäisi nyt purkautua
    oikein, eikä BOM-merkintää enää näytetä tekstin alussa.
-   Tilariville on lisätty huomattavasti enemmän tietoa. Se näyttää nyt
    nykyisen rivin, merkin ja lukemisen prosenttiosuuden.
-   HTML-kommentteja sekä skripti- ja tyylitunnisteiden sisältöä ei enää
    näytetä tekstilähdössä.
-   Jos Paperbackille annetaan suhteellinen polku komentoriviltä, se
    tulkkaa sen nyt oikein.
-   Prosentuaalista siirtymää hallitaan nyt omalla liukusäätimellä
    varustetulla valintaikkunalla, joka avautuu näppäinyhdistelmällä
    Ctrl+Shift+G.
-   Asiakirjoille, joiden otsikkoa tai tekijää ei tunneta, määritetään
    nyt aina oletusarvo.
-   Sijainnin tallennuslogiikka on nyt paljon älykkäämpi ja tallentaa
    tiedot levylle vain silloin, kun se on ehdottoman välttämätöntä.
-   Asiakirja, joka oli valittuna Paperbackin sulkemisen hetkellä,
    muistetaan nyt sovelluksen uudelleenkäynnistyksissä.
-   Syötteet "Siirry riville" ja "Siirry sivulle" -valintaikkunoihin
    puhdistetaan nyt tiukemmin.
-   Korjattu sisällysluettelon navigointi epub 3 -kirjoissa, joiden
    manifesteissa on suhteellisia polkuja.

### Versio 0.3.0 {#version-0.3.0}

-   Korjattu sisällysluettelo epub-kirjoissa, joiden manifestit
    sisältävät URL-koodattuja polkuja.
-   Korjattu otsikoiden selaus HTML-asiakirjoissa, jotka sisältävät
    monibittisiä Unicode-merkkejä.
-   Korjattu pitkien otsikoiden sisältävien asiakirjojen korkea
    CPU-kuormitus, joka johtui wxWidgets-kirjaston regressiosta.
-   Korjattu UTF-8-tekstitiedostojen lataus.
-   Korjattu ePub-kirjojen sisäkkäisten sisällysluettelokohteiden
    aiheuttama kohdistimen siirtyminen väärään kohtaan.
-   Korjattu sovelluksen kaatuminen suljettaessa tietyissä tapauksissa.
-   Lisätty valintaruutu asetusvalintaikkunaan sanan rivinvaihdon
    ottamiseksi käyttöön tai poistamiseksi käytöstä!
-   Nyt on mahdollista tehdä lahjoitus Paperbackin kehitystyöhön joko
    ohjevalikossa olevan uuden lahjoitusvaihtoehdon kautta tai
    GitHub-arkiston pääsivun alaosassa olevan "Sponsoroi tätä projektia"
    -linkin kautta.
-   Markdown-dokumenteissa on nyt aina otsikko, ja Paperbackin pitäisi
    nyt pystyä lataamaan käytännössä mikä tahansa Markdown-tiedosto.
-   PDF-tiedostoissa on nyt aina otsikko, vaikka metatiedot
    puuttuisivatkin.
-   PDF-kirjastot on vaihdettu Chromiumissa käytettyyn versioon, mikä
    parantaa PDF-tiedostojen jäsentämisen luotettavuutta kautta linjan.
-   Paperbackia voi nyt käyttää vain yksi instanssi kerrallaan. Jos
    paperback.exe-tiedostoa ajetaan tiedostonimellä, kun se on jo
    käynnissä, kyseinen asiakirja avautuu jo käynnissä olevaan
    instanssiin.
-   Voit nyt sulkea asiakirjan painamalla Delete-näppäintä
    välilehtipalkissa .

### Versio 0.2.1 {#version-0.2.1}

-   Lisättiin sivujen kokonaismäärä sivunimikkeeseen Siirry sivulle -
    valintaikkunaan.
-   Voit siirtyä tabulaattorilla asiakirjan sisällöstä avattujen
    asiakirjojen luetteloon.
-   Korjattu ongelma, jossa otsikon näppäinpainallukset avasivat
    toisinaan viimeisimmät asiakirjat, jos niitä oli tarpeeksi.
-   Paperback poistaa nyt tarpeettomat pehmeät tavuviivat tekstin
    tulostuksesta.
-   Korjattu ongelma, jossa otsikonavigoinnissa päädyttiin toisinaan
    väärälle merkille.

### Versio 0.2.0 {#version-0.2.0}

-   Lisätty tuki Markdown-asiakirjoille!
-   Lisätty tuki PDF-asiakirjoille, mukaan lukien mahdollisuus navigoida
    sivujen välillä!
-   Lisätty pikanäppäimiä otsikoiden mukaan navigoimiseen
    HTML-sisällössä, mukaan lukien epub-kirjat ja Markdown-asiakirjat.
    Nämä pikanäppäimet on suunniteltu toimimaan samalla tavalla kuin
    näytönlukija.
-   Korjattu epub-tiedostojen lataaminen, joiden manifesteissa on
    URL-koodattuja tiedostonimiä.
-   Korjattu epub 3 -kirjojen lataaminen, joissa on upotettua
    XHTML-koodia.
-   Nyt kuuluu ääniviesti, jos asiakirja ei tue sisällysluetteloa tai
    osioita, sen sijaan että valikkokohdat poistettaisiin käytöstä.
-   Lisätty viimeisimpien asiakirjojen valikko! Se tallentaa tällä
    hetkellä 10 viimeksi avattua asiakirjaa, ja painamalla
    Enter-näppäintä jonkin asiakirjan kohdalla se avautuu lukemista
    varten.
-   Hakudialogi on kirjoitettu kokonaan uudelleen, mikä tekee sen
    käytöstä paljon yksinkertaisempaa, ja samalla on lisätty viimeisten
    25 haun historia sekä tuki säännöllisille lausekkeille!
-   Aiemmin avatut asiakirjat muistetaan nyt sovelluksen
    uudelleenkäynnistyksen jälkeen. Tätä voidaan määrittää
    Työkalut-valikon uuden asetusvalikon kautta.
-   Lisätty Shift+F1-näppäinyhdistelmä, jolla Readme-tiedosto avautuu
    suoraan Paperback-sovelluksessa.

### Versio 0.1.0 {#version-0.1.0}

-   Ensimmäinen julkaisu.
