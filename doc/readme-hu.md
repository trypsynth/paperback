<!-- machine-translated from doc/readme.md (source-hash: cc466634600b58b4; sections: 84030068,db723a70,df2f4c18,e9b82d46,1387e8b7,3887c286,0ee39028,3a4c3efe,d3762a01,e9860ee8,80b9b9ca); please review and edit as needed -->

# Paperback – 1.0-s verzió

## Bevezetés

A Paperback kis erőforrásigényű, gyors és akadálymentes olvasóprogram e-könyvekhez, dokumentumokhoz és hangoskönyvekhez, mindenkinek, az alkalmi olvasóktól a legigényesebb haladó felhasználókig. Tervezésekor a képernyőolvasós akadálymentesség, a sebesség és a fölösleges sallangoktól mentes használat volt a cél.

## Rendszerkövetelmények

A Paperback a következő rendszereken fut: Windows 10/11, az ARM-alapú macOS összes újabb verziója, Linux, iOS 17 vagy újabb és Android 7 vagy újabb. Az iOS- és az Android-alkalmazás az App Store-ban, illetve a Google Playen érhető el.

## Funkciók

* Teljesen önálló: az olvasás megkezdéséhez semmilyen más szoftvert nem kell telepíteni a számítógépre.
* Rendkívül gyors, még régi hardveren is.
* Egyszerű, lapokra osztott felület, amelyben tetszőleges számú dokumentumot nyithat meg egymás mellett.
* Minden megnyitott dokumentumban pontosan megjegyzi az olvasási pozíciót.
* Ha kéri, megjegyzi, mely dokumentumok voltak megnyitva a program bezárásakor, és a következő indításkor újra megnyitja őket.
* Számos képernyőolvasó böngészési módjához hasonló navigációs funkciókat kínál, amelyekkel gyorsan és egyszerűen mozoghat a dokumentumokban.
* Sokoldalú keresési párbeszédpanel, többek között keresési előzményekkel és a reguláris kifejezések támogatásával.
* Teljesen hordozható módon is futtatható, de telepíthető is, ekkor a fájltársítások automatikusan beállnak.
* A gyakori fájlformátumok rendkívül széles körét támogatja.
* Hangoskönyveket is lejátszik, állítható sebességgel és olyan könyvjelzőkkel, amelyek a pontos időpontot jegyzik meg.
* A szkennelt PDF-oldalakat a Windows és a macOS beépített szövegfelismerésével teszi olvashatóvá.
* Könyvjelzők és jegyzetek, hogy megjelölhesse, hol tart, és később visszatérhessen oda.
* Minden billentyűparancs módosítható.
* Dokumentumokat közvetlenül http- és https-hivatkozásokról is megnyit, és újra letölti őket, amikor ismét megnyitja őket, vagy megnyomja az `F5` billentyűt; az olvasási pozíció és a könyvjelzők ilyenkor is megmaradnak. A korábban megnyitott dokumentumok visszaállításakor a hivatkozásról megnyitottak közül a legfeljebb 2 MB méretűeket tölti le újra.
* Része a `pb` parancssori eszköz, amely bármely támogatott dokumentumot HTML-, Markdown- vagy egyszerű szöveges formátumba alakít. Egyszerre tetszőleges számú dokumentumot feldolgoz, a `*.pdf` és a hasonló mintákat maga bontja ki, így ugyanaz a parancssor Windowson is működik, és dokumentumonként külön fájlt is írhat egy mappába. A `--files-from` kapcsolóval a dokumentumok listáját fájlból vagy a szabványos bemenetről (stdin) is beolvashatja, és közvetlenül http- és https-hivatkozásokról is át tud alakítani dokumentumokat.

## Képernyőolvasó-kompatibilitás

A Paperback minden elterjedt képernyőolvasóval jól működik. A JAWS felhasználóinál azonban két ismert probléma van.

### A JAWS és a braille-kijelzők

Ha a JAWS-t braille-kijelzővel használja, előfordulhat, hogy a hosszú bekezdések csonkák lesznek, amikor a kijelző navigációs gombjaival előre léptet. A hiba a Bekezdés felolvasása parancsot is érinti. Ez a JAWS hibája a RICHEDIT50W szövegvezérlő kezelésében, nem magáé a Paperbacké, és elég sokáig tartott, mire megoldódott – ami nem meglepő, ha figyelembe vesszük, milyen lelkesen szokott reagálni a Vispero a nyílt forráskódú szoftverekkel kapcsolatos hibajelentésekre.

A megkerülő megoldás, amely hónapokig tartó várakozás után végül a JAWS vitacsoportjában került elő: szerkessze a `paperback.jcf` fájlt, és a „Braille-leképezés és léptetés” beállításnál válassza a „Ha van, mindig a DOM használatával.” lehetőséget. Érdemes a „Szöveg léptetése bekezdésenként” beállítást is bekapcsolni, különben a kijelző az aktív bekezdésen marad, és nem lép tovább. Ha mindkét beállítás érvényben van, a léptetésnek megfelelően kell működnie.

### A JAWS és a Paperback üzenetei

A Paperback az olyan üzeneteket, mint a „Nincsenek oldalak.” vagy az „Ehhez a dokumentumhoz nem tartozik hang.”, akadálymentességi értesítésként küldi; ez teszi lehetővé, hogy a képernyőolvasó akkor is kimondja őket, ha éppen mást mond. A JAWS csak akkor reagál ezekre, ha az alkalmazáshoz be van kapcsolva az „Elérhető értesítési események engedélyezése” beállítás, egyes gépeken pedig nincs bekapcsolva.

Ha a JAWS semmit sem mond, amikor olyan billentyűt nyom meg, amelynek valamit be kellene jelentenie, nyissa meg a Beállítási központot, miközben a Paperback van előtérben (`Insert+6`), keressen rá az „értesítés” szóra, és jelölje be az „Elérhető értesítési események engedélyezése” beállítást. Ez a `paperback.jcf` fájlba írja a beállítást, így csak a Paperbackre vonatkozik.

## Jelenleg támogatott fájltípusok

A Paperback a következő formátumokat és kiterjesztéseket támogatja:

* Képregényarchívumok (`.cbz`)
* CHM-súgófájlok (`.chm`)
* DAISY-könyvek (`.opf`, `.zip`)
* EPUB-könyvek (`.epub`)
* FB2 formátumú e-könyvek (`.fb2`)
* HTML-dokumentumok (`.htm`, `.html`, `.xhtml`)
* Unix-kézikönyvoldalak, `man` és BSD `mdoc` formátumban is (`.1`–`.9`, `.man`, `.roff`, valamint mindegyik gzippel tömörített változata)
* Markdown-dokumentumok (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Microsoft Word-dokumentumok (`.docx`, `.docm`, `.doc`)
* M4B-hangoskönyvek (`.m4b`)
* MOBI/Kindle-könyvek (`.mobi`, `.azw`, `.azw3`)
* MP3-hangoskönyvek (`.mp3`)
* OpenDocument-bemutatók (`.odp`, `.fodp`)
* OpenDocument-szövegfájlok (`.odt`, `.fodt`)
* PDF-dokumentumok (`.pdf`)
* PowerPoint-bemutatók (`.pptx`, `.pptm`, `.ppt`)
* reStructuredText-dokumentumok (`.rst`, `.rest`)
* RTF-dokumentumok (`.rtf`)
* Windows Write-dokumentumok (`.wri`)
* WinHelp-fájlok (`.hlp`)
* Egyszerű szöveges és naplófájlok (`.txt`, `.log`)

## Billentyűparancsok

A Paperback elsősorban billentyűzetes használatra készült. Az alábbiakban a jelenlegi billentyűparancsok olvashatók.

Az alábbi billentyűparancsok a Windowsra vonatkoznak. Ahol a macOS eltér, a megfelelő billentyűparancs zárójelben szerepel – főleg azért, mert ezen a platformon a Ctrl+G, a Ctrl+W és az Alt+Balra/Jobbra nyíl már más rendszer- vagy alkalmazáskonvenciókhoz tartozik. macOS-en a Ctrl helyett a Cmd billentyűt kell használni ott is, ahol nincs külön macOS-billentyűparancs feltüntetve: a `Ctrl+R` például `Cmd+R`.

### Fájl menü

* `Ctrl+O`: Dokumentum megnyitása.
* `Ctrl+L` (macOS: `Cmd+Option+O`): Dokumentum megnyitása webcímről.
* `Ctrl+F4` (macOS: `Cmd+W`): Az aktuális dokumentum bezárása.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Az összes megnyitott dokumentum bezárása.
* `Ctrl+Shift+T`: A legutóbb bezárt dokumentum ismételt megnyitása.
* `Ctrl+R`: Az „Összes dokumentum” párbeszédpanel megjelenítése (a Legutóbbi dokumentumok almenüből).
* `Ctrl+Shift+C`: A fájl megjelenítése a mappájában.
* `Ctrl+Shift+E`: Dokumentumadatok exportálása (`.paperback`).
* `Ctrl+Shift+I`: Dokumentumadatok importálása (`.paperback`).
* `Ctrl+E`: Az aktuális dokumentum exportálása egyszerű szövegként.
* `Ctrl+Q`: Kilépés (Windowson és Linuxon; macOS-en az alkalmazásmenüben található, ott `Cmd+Q`).

### Szerkesztés menü

* `Ctrl+F`: A Keresés párbeszédpanel megjelenítése.
* `F3` (macOS: `Cmd+G`): Következő keresése.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Előző keresése.
* `Alt+F9` (macOS: `Cmd+F9`): A kijelölés elejének megjelölése, hogy innen addig, ameddig eljut, egyetlen lépésben kimásolhasson mindent.
* `Alt+F10` (macOS: `Cmd+F10`): Minden szöveg másolása a kijelölés megjelölt elejétől az aktuális pozícióig.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Visszaugrás a kijelölés megjelölt elejére; a jelölés megmarad.

### Nézet menü

* `F5`: Az aktuális dokumentum ismételt beolvasása a lemezről, illetve újbóli letöltése, ha hivatkozásról nyitotta meg. Attól függetlenül működik, hogy a Beállításokban be van-e kapcsolva a „Módosult dokumentumok automatikus újratöltése” beállítás.
* `Ctrl+Alt+W`: Sortörés be- és kikapcsolása.
* `F11` (macOS: `RawCtrl+Ctrl+F`, azaz Control+Command+F): Teljes képernyő be- és kikapcsolása.
* `Ctrl+Shift+V`: Az aktuális tartalom megnyitása webes nézetben.
* `Ctrl+U`: A dokumentum forrásának megtekintése új lapon.

### Ugrás menü

* `Ctrl+T`: A tartalomjegyzék megjelenítése.
* `F7`: Az elemlista megjelenítése.
* `Ctrl+G` (macOS: `Cmd+L`): Ugrás sorra.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Ugrás adott százalékra.
* `Ctrl+P`: Ugrás oldalra (ha az aktuális dokumentum támogatja).
* `=`: Az aktuális olvasási százalék és oldal bejelentése, például „15%, 30. oldal”. Az oldalszám nélküli dokumentumoknál az oldal kimarad.
* `Alt+Left` (macOS: `Cmd+[`): Visszalépés a navigációs előzményekben.
* `Alt+Right` (macOS: `Cmd+]`): Előrelépés a navigációs előzményekben.
* `[`: Előző szakasz.
* `]`: Következő szakasz.
* `Shift+H`: Előző címsor.
* `H`: Következő címsor.
* `Shift+1`-től `Shift+6`-ig: Előző 1–6. szintű címsor.
* `1`-től `6`-ig: Következő 1–6. szintű címsor.
* `Shift+P`: Előző oldal.
* `P`: Következő oldal.
* `Shift+K`: Előző hivatkozás.
* `K`: Következő hivatkozás.
* `Shift+G`: Előző kép.
* `G`: Következő kép.
* `Shift+F`: Előző ábra.
* `F`: Következő ábra.
* `Shift+T`: Előző táblázat.
* `T`: Következő táblázat.
* `Shift+M`: Előző képlet.
* `M`: Következő képlet.
* `Shift+S`: Előző elválasztó.
* `S`: Következő elválasztó.
* `Shift+L`: Előző lista.
* `L`: Következő lista.
* `Shift+I`: Előző listaelem.
* `I`: Következő listaelem.
* `Shift+,`: Ugrás az aktuális tároló (lista vagy táblázat) elejére.
* `,`: Ugrás az aktuális tároló (lista vagy táblázat) vége mögé.

### Könyvjelzők menü

* `Ctrl+Shift+B`: Könyvjelző hozzáadása vagy eltávolítása az aktuális kijelölésnél vagy a kurzornál.
* `Ctrl+Shift+N`: Könyvjelzőjegyzet hozzáadása vagy szerkesztése az aktuális kijelölésnél vagy a kurzornál.
* `Shift+B`: Előző könyvjelző.
* `B`: Következő könyvjelző.
* `Shift+N`: Előző jegyzet.
* `N`: Következő jegyzet.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, azaz a fizikai Control billentyű, nem a Cmd): Az aktuális pozíción lévő jegyzet szövegének megtekintése.
* `Ctrl+B`: Ugrás könyvjelzőre vagy jegyzetre (az összes közül).
* `Ctrl+Alt+B`: Ugrás könyvjelzőre (csak a könyvjelzők közül).
* `Ctrl+Alt+M`: Ugrás jegyzetre (csak a jegyzetek közül).
* `/`: Az ideiglenes könyvjelző beállítása.
* `\`: Ugrás az ideiglenes könyvjelzőre.

### Hang menü

* `Ctrl+Space` (macOS: `RawCtrl+Space`, azaz a fizikai Control billentyű, mivel a Cmd+Space a Spotlightot nyitja meg): Az audionarráció lejátszása vagy szüneteltetése.
* `'`: Az audionarráció előretekerése.
* `;`: Az audionarráció visszatekerése.
* `Shift+'`: A léptetési időegység növelése.
* `Shift+;`: A léptetési időegység csökkentése.
* `Ctrl+Shift+.`: Az audionarráció gyorsítása.
* `Ctrl+Shift+,`: Az audionarráció lassítása.
* `Ctrl+'`: Az audionarráció hangosítása.
* `Ctrl+;`: Az audionarráció halkítása.

### Eszközök menü

* `Ctrl+W` (macOS: `RawCtrl+W`, azaz a fizikai Control billentyű, nem a Cmd): Az aktuális dokumentum szavainak száma.
* `Ctrl+I`: A dokumentuminformációk megjelenítése.
* `Ctrl+Shift+O`: Szkennelt PDF-oldalak egy tartományának szövegfelismerése.
* `Ctrl+Shift+S`: Az elalvási időzítő be- és kikapcsolása.
* `Ctrl+,`: A Beállítások megnyitása (macOS-en az alkalmazásmenüben).

### Súgó menü

* `Ctrl+F1`: A Paperback névjegyének megjelenítése.
* `F1`: A súgó megtekintése az alapértelmezett böngészőben.
* `Shift+F1`: A súgó megtekintése a Paperbackben.
* `Ctrl+Shift+U`: Frissítések keresése.
* `Ctrl+D`: Az adományozási oldal megnyitása az alapértelmezett böngészőben.

### További billentyűk a dokumentumnézetben

* `Delete` / `Numpad Delete` a lapsávon: A kijelölt dokumentumlap bezárása.
* `Ctrl+1`-től `Ctrl+9`-ig (macOS: `Cmd+1`-től `Cmd+9`-ig) a dokumentum szövegében vagy a lapsávon: Ugrás az első kilenc megnyitott dokumentumra, a megnyitásuk sorrendjében.
* `Ctrl+Tab` és `Ctrl+Shift+Tab`: Ugrás a következő vagy az előző megnyitott dokumentumra; az utolsó után az első, az első előtt az utolsó következik.
* `Enter` vagy `Space` a dokumentum szövegében: A kurzornál lévő hivatkozás követése, illetve táblázat- vagy képletnézet megnyitása.
* `Enter` egy szkennelt PDF-oldalon: Az oldal szövegfelismerése.
* `Shift+F10` vagy a helyimenü-billentyű a dokumentum szövegében: A helyi menü megnyitása. EPUB, HTML, Markdown, CHM, FB2 vagy képregényfájl képe esetén lehetőség van a kép fájlba mentésére is.

## iOS és Android

Az iOS- és az Android-alkalmazás ugyanazt az olvasómotort használja, mint az asztali változat, így ugyanazokat a formátumokat nyitja meg, és ugyanúgy jegyzi meg, hol tart. iOS-en a VoiceOverrel, Androidon a TalkBackkel való használatra készültek.

### Dokumentumok megnyitása

* Használja a Könyv megnyitása gombot, vagy nyisson meg egy dokumentumot a Fájlok alkalmazásból vagy egy másik alkalmazásból, és válassza a Paperbacket.
* Androidon ehelyett a Beállításokban bekapcsolhatja a beépített fájlböngészőt. Ehhez a „Hozzáférés az összes fájlhoz” engedély szükséges, cserébe a nagy fájlokat azonnal megnyitja, és nem másolja át őket előbb.
* Ha hosszan megnyomja a Könyv megnyitása gombot, importálhatja vagy exportálhatja egy dokumentum adatait (`.paperback`); ezek ugyanazok a fájlok, amelyeket az asztali alkalmazás is használ.

### Olvasás és hallgatás

Mindkét alkalmazásban kétféleképpen olvashat egy dokumentumot. Szöveges módban a képernyőolvasóval olvassa a szöveget. Felolvasási módban a Paperback olvassa fel a szöveget a Beállításokban kiválasztott beszédhangon, és a háttérben, illetve a zárolási képernyőn is folytatja. A két mód között a További lehetőségek menüben válthat.

A hangoskönyvek, például a DAISY-, M4B- és MP3-könyvek ehelyett a saját hangfelvételüket játsszák le.

### Az olvasósáv

A képernyő alján lévő sávon balról jobbra a következők találhatók:

* A navigációs egység, például bekezdés, címsor, oldal vagy hivatkozás. A módosításához legyintsen rajta felfelé vagy lefelé.
* Az előző, a lejátszás és a következő gomb. Az előző és a következő gomb navigációs egységenként léptet.
* A beszédsebesség. Felfelé vagy lefelé legyintve módosíthatja, milyen gyorsan olvas a Paperback.

A lejátszás gombon felfelé vagy lefelé legyintve is léptethet navigációs egységenként, anélkül hogy az előző vagy a következő gombhoz kellene nyúlnia. Ha csak ezt használja, az „Előző és következő gomb elrejtése” beállítással ezek a gombok nem lesznek a képernyőolvasó útjában. A „Felfelé legyintés előre léptet” beállítással azt adhatja meg, merre léptessen a legyintés.

### További lehetőségek

A További lehetőségek menüben található minden más. Egyes elemek a két alkalmazásban kissé eltérően működnek.

* **Váltás felolvasási módra vagy Váltás szöveges módra:** vált a fent leírt felolvasási és szöveges mód között. Szöveges módban a Felolvasás elem elindítja és szünetelteti a felolvasást anélkül, hogy kilépne a szöveges módból.
* **Tartalomjegyzék:** a könyv fejezetei, az éppen olvasottnál megnyitva. Válasszon ki egyet, és közvetlenül odaugrik. Az alfejezeteket tartalmazó bejegyzések a képernyőolvasó műveleteivel kibonthatók és összecsukhatók.
* **Elemek:** a dokumentum címsorainak vagy hivatkozásainak listája. iOS-en a Típus választóval, Androidon a lapokkal válthat a kettő között, majd válasszon ki egy elemet, és odaugrik.
* **Keresés:** írja be, mit keres, vagy válasszon egy korábbi keresést a Keresési előzmények közül, és adja meg, hogy megkülönböztesse-e a kis- és nagybetűket, csak teljes szavakra keressen-e, illetve reguláris kifejezést használjon-e. Az Előző keresése és a Következő keresése a találatra ugrik, és bejelenti, hová került, a Keresés pedig nyitva marad, így folytathatja. Felolvasási módban a Keresés navigációs egységként is megjelenik az olvasósávon, így onnan is végiglépkedhet a találatokon. A Beállításokban a Szövegfelolvasás alatt található **Felolvasás leállítása a keresési találatok után** beállítás alapértelmezés szerint be van kapcsolva, és a találatot tartalmazó bekezdés felolvasása után leállítja a felolvasást. Kapcsolja ki, ha a találattól folytatni szeretné a felolvasást; az olvasósáv előző és következő gombja ekkor is a keresési találatok között navigál.
* **Ugrás:** ugrás egy sorra, egy oldalra vagy a dokumentum adott százalékára. Hogy melyikre, azt a Mód választóval adhatja meg.
* **Legutóbbi dokumentumok:** az összes dokumentum, amelyet valaha megnyitott; mindegyiknél ott áll, hogy „Jelenleg megnyitva”, „Bezárva” vagy „A fájl hiányzik”. Mindegyikhez két képernyőolvasó-művelet tartozik: az Eltávolítás leveszi a listáról, a Megkeresés segítségével pedig megkeresheti azt a dokumentumot, amelynek a fájlja máshová került. A Legutóbbi dokumentumok listájának kiürítése kiüríti a listát, de egyetlen dokumentumot sem töröl.
* **Szavak száma:** a dokumentumban lévő szavak száma.
* **Dokumentuminformáció:** a cím, a szerző, a fájlnév, iOS-en pedig a sorok és a karakterek száma is.
* **Exportálás:** a dokumentum mentése egyszerű szövegként, HTML-ként vagy Markdownként.
* **Elalvási időzítő:** 5, 10, 15, 30, 45 vagy 60 perc, illetve egy saját maga által megadott idő elteltével leállítja a felolvasást. Ha futás közben újra megnyitja, láthatja, mennyi idő van hátra, és le is állíthatja.
* **Súgó:** megnyitja ezt a kézikönyvet.
* **Beállítások:**
    * **Szövegfelolvasás:** a beszédhang, a beszédsebesség és a hangmagasság, a meghallgatásukhoz egy Minta lejátszása gomb, valamint a bekezdések közötti szünet. Androidon a beszédmotor is kiválasztható. iOS-en itt található a kiejtési szótár is: szabályok, amelyek megváltoztatják bizonyos szavak kiejtését, akár minden beszédhangnál, akár csak egyeseknél.
    * **Olvashatóság:** szövegméret, sorköz, bekezdésköz, igazítás és nagy kontrasztú szöveg. iOS-en világos és sötét megjelenés is választható.
    * **Működés:** a dokumentumok újbóli megnyitása az alkalmazás indításakor, a lejátszás gombon végzett legyintés iránya, valamint az előző és a következő gomb elrejtése. Androidon itt kapcsolható be a beépített fájlböngésző is.

### Billentyűzet és headset

Billentyűzettel az asztali változat billentyűparancsai is működnek a könyvek megnyitásához, a legutóbbi dokumentumokhoz, a Kereséshez, az Ugráshoz, a tartalomjegyzékhez, a szavak számához, a dokumentuminformációkhoz, az exportáláshoz és az elalvási időzítőhöz; iOS-en a `Ctrl` helyett a `Cmd` billentyűvel. Ugyanígy működnek a címsorok, oldalak, hivatkozások és a többi elem közötti navigáció egybetűs billentyűi is, a `Space` pedig elindítja és szünetelteti a lejátszást. iOS-en az egybetűs billentyűk csak akkor jutnak el a Paperbackhez, ha a VoiceOver egybillentyűs gyorsnavigációja ki van kapcsolva.

Androidon a headset gombjának egyszeri megnyomása elindítja vagy szünetelteti a lejátszást, a kétszeri előre, a háromszori pedig visszaléptet.

## Támogatott nyelvek

A Paperback számos nyelvre le van fordítva, és a sor folyamatosan bővül. Az alábbiakban a teljes lista olvasható.

Ha szeretne részt venni a fordításban, olvassa el a [fordítási útmutatót](translating.md).

* Bosnyák
* Cseh
* Holland
* Finn
* Francia
* Német
* Magyar
* Japán
* Lengyel
* Portugál (brazíliai)
* Orosz
* Egyszerűsített kínai
* Szerb
* Spanyol
* Ukrán
* Vietnami

## Köszönetnyilvánítás
### Fejlesztés
* Quin Gillespie: fő fejlesztő és a projekt alapítója.
* Aryan Choudhary: fő közreműködő.

### Adományok
Az alábbiak kisebb-nagyobb összeggel támogatták a Paperback fejlesztését. Ha adományoz, a neve nem kerül fel automatikusan erre a listára: csak azokat veszem fel, akik szeretnék, hogy az adományuk nyilvános legyen.

Megjegyzés: a nyilvános GitHub-támogatást (GitHub Sponsors) elegendő oknak tekintem ahhoz, hogy valaki automatikusan felkerüljön erre a listára.

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

## Változásnapló

### 1.0-s verzió

Az 1.0 az első kiadás mind az öt platformon: Windowson, macOS-en, Linuxon, iOS-en és Androidon; az iOS- és az Android-alkalmazás az App Store-ban, illetve a Google Playen érhető el.

#### Újdonságok

##### Általános
* Linux-támogatás AppImage- vagy tar.gz-csomagként, asztali integrációval, így a dokumentumok a fájlkezelőből is megnyithatók.
* Az `Alt+F9` billentyűparanccsal megjelölheti a kijelölés elejét, az `Alt+F10` billentyűparanccsal mindent kimásolhat onnan addig, ahová eljutott, az `Alt+Shift+F9` billentyűparanccsal pedig visszaugorhat a jelöléshez; így hosszú szövegrészt is másolhat anélkül, hogy a Shift és a nyílbillentyűk segítségével végig kellene jelölnie. Mindhárom a Szerkesztés menüben található.
* Az `=` billentyűparancs most már az oldalt is bejelenti a százalék mellett, például „15%, 30. oldal”; az oldalszám nélküli dokumentumoknál a korábbiak szerint működik.
* A névjegy most már megjeleníti a Paperback licencét és az összes fordítót.
* Ukrán fordítás.

##### Új formátumok
* Képregényarchívumok (`.cbz`).
* M4B-hangoskönyvek, fejezetekre bontva.
* Unix-kézikönyvoldalak, `man` és BSD `mdoc` formátumban is, gzippel tömörítve vagy anélkül.
* MP3-hangoskönyvek, fejezetekre bontva, ha a fájl tartalmaz fejezeteket.
* reStructuredText-dokumentumok.
* Windows Write-fájlok (`.wri`).
* WinHelp-fájlok (`.hlp`).
* Word 6- és Word 95-dokumentumok.

##### Szövegfelismerés
* A szkennelt PDF-oldalak most már felismerhetők a Windows és a macOS beépített szövegfelismerésével. Egy szkennelt oldal felismeréséhez nyomja meg rajta az `Enter` billentyűt, egy oldaltartományhoz pedig használja a Kötegelt szövegfelismerés funkciót (`Ctrl+Shift+O`).

##### Navigáció
* Az EPUB- és HTML-dokumentumokban lévő MathML-képleteket a Paperback a MathCAT segítségével AsciiMath formában jeleníti meg. A képletek között az `M` és a `Shift+M` billentyűvel navigálhat, majd az `Enter` vagy a `Space` billentyűvel megnyithatja az eredeti MathML-t a Képletnézetben.
* Összes keresése gomb a Keresés párbeszédpanelen, amely minden találatot tartalmazó sort listáz, így közvetlenül a kívánt sorra ugorhat.
* Táblázatok, Listák és Oldalak nézet az elemlistában (`F7`).
* Az Ugrás sorra, az Ugrás oldalra és az Ugrás adott százalékra most már a `+n` és a `-n` értéket is elfogadja, amellyel az aktuális helyhez képest léphet.
* A saját címsorok nélküli EPUB-, MOBI- és CHM-könyvekben most már a tartalomjegyzékük alapján működik a címsoronkénti navigáció.
* A KF8 (AZW3) könyvek most már támogatják a szakaszonkénti navigációt.
* Azoknál az EPUB-oldalaknál, amelyek csak egy képből állnak, most már megjelenik egy sor a képhez, így rájuk léphet ahelyett, hogy átugorná őket.

##### Hangoskönyvek
* A lejátszási sebesség szabályozása a fél sebességtől a háromszorosig. Használja a `Ctrl+Shift+.` és a `Ctrl+Shift+,` billentyűparancsot vagy a Hang menüt.
* A csak hangot tartalmazó könyvekben a könyvjelzők és a jegyzetek most már pontosan azt az időpontot jegyzik meg, amelynél létrehozta őket.
* A következő és az előző pozíció (`Alt+Left` és `Alt+Right`) most már a hangoskönyvekben is működik.
* A hangoskönyvekben való haladást a program most már a felvétel alapján méri, így az Ugrás adott százalékra és az állapotsor azt mutatja, valójában hol tart.

##### Legutóbbi dokumentumok
* Lista kiürítése elem a Legutóbbi dokumentumok almenüben.

##### PDF-dokumentumok
* Beállítás, amellyel a PDF-ek minden sora különálló marad, és nem egyesülnek bekezdésekké.
* A PDF-ekben lévő képeket és ábrákat a program most már bejelenti.
* Azok a PDF-ek, amelyek tartalmaznak olvasási struktúrát, de egyetlen képüket sem címkézik, most már bejelentik ezeket a képeket, ahelyett hogy teljesen kihagynák őket a könyvből.

##### Webes nézet
* Most már bármely dokumentum megnyitható a webes nézetben, nem csak az EPUB, a HTML és a Markdown.

##### Olvashatóság
* A címsorok most már a szintjüknek megfelelő méretben jelennek meg, a képek és a táblázatok pedig elkülönülnek a környező szövegtől.

##### pb
* A `pb --list-formats` kilistázza az összes formátumot, amelyet a pb be tud olvasni.
* A pb most már megmondja, melyik fájlt nem tudta beolvasni, és miért.

#### Javítások

##### Általános
* Az indításkor újra megnyitott könyv most már azonnal olvasható, és nem marad néma addig, amíg be nem zárják és újra meg nem nyitják.
* A hiányzó fájlú dokumentum most már eltávolítható az Összes dokumentum listából, és nem marad benne, akárhányszor erősíti is meg az eltávolítást.
* Megszűnt egy összeomlás, amely a Paperback bezárásakor történt.
* A Paperback bezárásakor az ablak most már azonnal eltűnik, és nem marad a képernyőn, amíg a mentés tart.
* A kevés formázást tartalmazó nagy könyvek most már nagyjából feleannyi idő alatt nyílnak meg.
* A menüből választott üzeneteket, például az „Ehhez a dokumentumhoz nem tartozik hang” üzenetet, a képernyőolvasó most már nem szakítja félbe, mielőtt meghallaná őket.
* Egy dokumentum megnyitása után a Legutóbb bezárt dokumentum megnyitása parancs most már nem marad engedélyezve, ha nincs mit újra megnyitni.
* A Paperback most már nem próbálja újra meg újra megnyitni a legutóbbi dokumentumok listájában szereplő, időközben eltűnt dokumentumokat, és korlátozza, hány legutóbbi dokumentumot tárol.
* A régi INI-beállításfájl most már törlődik, miután a beállítások átkerültek az új formátumba.
* A betűtípus- és a színválasztó párbeszédpanel címe, valamint a vietnami változatban az Exportálás másként menü most már le van fordítva.
* Frissítés után az újraindított ablak most már előtérbe kerül, és nem marad az összes többi ablak mögött az Alt+Tab listában.
* A sortörés most már nagy dokumentumoknál is azonnal érvénybe lép, és nem tölti újra az egész dokumentumot.

##### Navigáció
* Az `Alt+Left` most már oda viszi vissza, ahonnan ugrott, és nem egy régebbi pozícióra.
* A könyvjelzőhangok most már csak akkor szólalnak meg, amikor áthalad egy könyvjelzőn, és nem akkor, amikor arra a sorra lép, amelyben a könyvjelző van.
* A tartalomjegyzék, az elemlista és az Ugrás párbeszédpanelek bezárása után most már közvetlenül arra a sorra kerül, ahová érkezett, és nem kell végighallgatnia, ahogy a képernyőolvasó újra felolvassa az ablakot.
* Az Ugrás sorra, az Ugrás oldalra és az Ugrás adott százalékra most már elutasítja a dokumentumon kívül eső számokat, és nem ugrik szó nélkül máshová.
* Az NVDA most már nem vágja el a bejelentést, ha egy dokumentumnak nincsenek oldalai.
* Ha a tartalomjegyzékben mozgás nélkül nyomja meg az OK gombot, most már a már kijelölt bejegyzésre ugrik.
* A tartalomjegyzék, az elemlista és a könyvjelzők listája most már nem akad és nem fagy le a több ezer bejegyzést tartalmazó könyveknél.
* A felfelé és a lefelé nyíl most már dokumentumonként jegyzi meg az oszlopot, és nem viszi át, amikor lapot vált.

##### Hangoskönyvek
* A hanglejátszás macOS-en most már a `Control+Space` billentyűparancsot használja, mivel a `Command+Space` a Spotlighthoz tartozik.

##### PDF-dokumentumok
* Az Apple Pagesből exportált PDF-ek most már nem egyszerű szövegként jelennek meg, hanem megmaradnak bennük a címsorok és a listák, amelyekkel írták őket.
* A PDF-bekezdések és -címsorok most már nem törnek szét minden sornál, és a szavak belsejébe sem kerülnek szóközök.
* A számozott PDF-címsorok most már nem olvadnak össze egyetlen címsorrá.
* Azok a PDF-ek, amelyeknek a struktúrafája nem vezet szöveghez, most már nem nyílnak meg üresen.
* A nem proporcionális betűtípussal szedett sorokat, például a kódot, a program most már nem vonja össze bekezdésekké.
* A címkézetlen PDF-ekben az élőfejeket és élőlábakat a program most már nem olvassa fel minden oldalon.
* Azok a PDF-ek, amelyek az élőfejeiket és élőlábaikat közönséges szövegként címkézik, most már nem ismétlik meg minden oldalon a címet és az oldalszámot két bekezdés között.
* A PDF-ek most már a valódi címüket mutatják, nem a fájlnevüket.

##### MOBI/AZW3-könyvek
* A nagy MOBI-könyveknél most már nem fogy el a memória, és 20 MB után sem vágódnak le.
* A MOBI- és AZW3-könyvek most már sokkal gyorsabban nyílnak meg.
* A MOBI-könyvek most már nem veszítik el a fejezetlistájukat.
* A MOBI-könyvekben most már nem torzul el a szöveg ott, ahol az egyik rekordból a következőbe lép át.

##### Webes nézet
* A webes nézet most már nem tölti be egyszerre egy hatalmas könyv egészét.
* A webes nézet most már egészben mutatja a dokumentumokat, ha a fő nézet is egészben mutatja őket, nem csak egy szeletüket.

##### Egyéb formátumok
* A windows-1251 kódolású FictionBook (.fb2) könyvek – vagyis a legtöbb – most már megnyílnak, ahelyett hogy egyáltalán nem lehetne beolvasni őket.
* Azok a FictionBook-könyvek, amelyek nem deklarált névteret vagy HTML-entitást használnak, most már megnyílnak, és a program nem utasítja el őket sérültként.
* A régi kódolású könyvek most már sokkal gyorsabban nyílnak meg.
* Egyes kínai szövegfájlok most már nem nyílnak meg olvashatatlan karakterekkel.
* A jelszóval védett OpenDocument-fájlok most már kérik a jelszavukat, és a program nem jelzi őket sérültként.
* A jelszóval védett régi PowerPoint-fájlok most már megnyílnak, és a régi PowerPoint-diák most már nem veszítik el a szövegüket.
* Az `.rtf` kiterjesztéssel mentett egyszerű szöveges fájlok most már szövegként nyílnak meg, és nem jelenik meg hibaüzenet.
* Az RTF-vezérlőszavak most már nem jelennek meg szövegként.

#### iOS és Android

Az iOS- és az Android-alkalmazás minden formátumot megnyit, amelyet az asztali változat, és a következőket kínálja:

* Felolvasás választható beszédhanggal, sebességgel és hangmagassággal, beszédsebesség-szabályzóval közvetlenül az olvasósávon, valamint választható szünettel a bekezdések között.
* DAISY-, M4B- és MP3-hangoskönyvek lejátszása, amely a háttérben és a zárolási képernyőn is folytatódik.
* Navigáció címsoronként, oldalanként, hivatkozásonként, táblázatonként, listánként és még sok más módon az olvasósávról, valamint tartalomjegyzék és keresés.
* Elalvási időzítő, szavak száma és dokumentumexportálás, iOS-en pedig kiejtési szótár is. iOS-en az exportálás a megosztási menün keresztül történik, így egy könyv más alkalmazásba vagy a Fájlokba is küldhető, más formátumban vagy pontosan úgy, ahogy van.
* Szövegméret, térköz és nagy kontrasztú szöveg beállítása.
* Az asztali változattal egyező billentyűparancsok.

### 0.9.2-es verzió
* A hangoskönyveknél a képernyőolvasó most már nem olvas fel egy sor szóközt, amikor a szövegmezőre kerül a fókusz.
* A hangoskönyvek most már megnevezik a fájlt, amikor szakaszonként lépked bennük.
* A hangoskönyvek most már a valódi hosszukat jelentik, és nem állítják, hogy minden bennük lévő fájl 24 órás.
* A webes nézet Escape billentyűvel való bezárása most már nem dob fel hibakeresési üzenetet, ha előtte követett benne egy hivatkozást.
* Ha Az összes kijelölése után másol, most már a teljes dokumentum a vágólapra kerül, nem csak az éppen betöltött része.
* A keresés most már közvetlenül a megtalált sorra ugrik, és nem kell végighallgatnia, ahogy a képernyőolvasó újra felolvassa az ablakot, miközben a fókusz visszatér a könyvre.
* Azok az EPUB-ok, amelyekben egy kóbor ZIP64-blokk található, most már megnyílnak, és nem jelenik meg az „Invalid local file header” hiba.
* A hosszú dokumentumok most már nem ugranak vissza az elejükre, miközben a képernyőolvasó folyamatosan olvassa őket.
* A webes nézetben a hivatkozások most már arra a szakaszra visznek, amelyre mutatnak, és nem jelenik meg a „File not found” hiba.
* Az automatikus „Dokumentum újratöltve.” bejelentés most már nem szakítja félbe a képernyőolvasót mondat közben, hanem megvárja, amíg befejezi, amit mondott.
* A Beállítások párbeszédpanel Általános lapján a Tab billentyű most már a képernyőn látható sorrendben halad végig a beállításokon, és a frissítési csatorna közvetlenül a frissítések keresésére vonatkozó beállítás után következik.
* A Windows most már mindig „Paperback” néven jeleníti meg a programot a Társítás menüben, nem a program teljes szlogenjével.
* A Szavak száma és a Dokumentuminformáció most már megmutatja, hány fájlból áll egy hangoskönyv, és összesen milyen hosszú.

### 0.9.1-es verzió
* A könyvjelző- és jegyzethangok most már macOS-en is megszólalnak.
* A DAISY-könyvek most már macOS-en is lejátsszák a hangjukat, és nem csak csendben megnyílnak és követik az idővonalukat.
* Az RTF-dokumentumokból most már nem tűnnek el a tipográfiai idézőjelek, a gondolatjelek és a hasonló karakterek, amelyek eltűnésükkel összefuttatták a környező szavakat.
* Az RTF-képek nyers adatai most már nem szivárognak át olvashatatlan szövegként a dokumentumba.
* A Legutóbbi dokumentumok almenü most már nem őrzi az elavult bejegyzéseket addig, amíg valami más újra fel nem építi.
* A gyorsbillentyűk visszakerültek minden fordításba, így az orosz menük is újra elérhetők a billentyűzetről.
* A nagy CHM-dokumentumok most már akár hétszer gyorsabban nyílnak meg.
* A megnyitott dokumentumokat a program most már regisztrálja a Windowsban, így megjelennek a tálca gyorslistájában és a Start menü legutóbbi elemei között.
* Az Options új neve Settings (magyarul mindkettő Beállítások), így egyezik a mobilalkalmazásokkal, macOS-en pedig a platform szokásaival.
* A Paperback most már megjegyzi az ablak helyét, méretét és teljes méretű állapotát két futtatás között.
* A többes számú alakok most már lefordíthatók, így a dolgokat megszámláló üzenetek helyesen hangzanak azokon a nyelveken is, amelyekben egynél több alak szükséges.
* Ha egy DAISY-könyv ncc.html fájlját választja ki, most már a teljes hangoskönyv nyílik meg, nem csak a szövege.
* A Billentyűparancsok testreszabása párbeszédpanel műveletnevei most már lefordíthatók.
* Az ablak címsorában most már a dokumentum címe áll elöl, így a megnyitott könyvek a tálcán és az Alt+Tab listában is megkülönböztethetők.
* A frissítési párbeszédpanel most már le van fordítva.

### 0.9.0-s verzió

#### Újdonságok

##### Általános
* Egy pb nevű parancssori eszköz, amellyel a Paperback által támogatott bármely formátum gyorsan HTML-, Markdown- vagy egyszerű szöveges formátumba alakítható.
* Beállítás a más programok által a lemezen módosított dokumentumok újratöltéséhez.
* Forrás megtekintése parancs, amellyel a dokumentum forrása új lapon nyitható meg; ez például Markdown szerkesztésekor hasznos.
* A dokumentum szövege most már részletekben töltődik be, így a több tízmillió szavas könyvek is mindössze néhány másodperc alatt betölthetők. Ha ezzel kapcsolatban bármilyen furcsaságot tapasztal, kérjük, jelezze.

##### Platformtámogatás
* ARM64-es Windows-támogatás!
* Natív macOS-támogatás!
* Teljes képernyős mód.

##### Összes dokumentum párbeszédpanel
* Fájl keresése gomb, amellyel megkeresheti azokat a hiányzó könyveket, amelyeknek csak az elérési útja változott meg.
* Állapotszűrő és állapotsor, így a dokumentumokat állapot szerint szűrheti, és láthatja, hány dokumentum látható és hány van kijelölve.
* A `Ctrl+Shift+A` billentyűparancs az összes dokumentum kijelölésének megszüntetéséhez.

##### Beállítások és olvashatóság
* Olvashatóság lap a következő beállításokkal:
    * Sortörés (az Általános lapról áthelyezve);
    * Táblázatok megjelenítése a szövegben (új ebben a kiadásban, lásd lent);
    * Betűtípus;
    * Háttérszín;
    * Sorköz;
    * Bekezdésköz;
    * Betűköz;
    * Szöveg igazítása.
* Sortörés menüelem és hozzá tartozó billentyűparancs.
* Beállítás annak megadására, hogyan jelenjenek meg a táblázatok; emellett a táblázatok megjelenítése egységes lett minden dokumentumtípusban.

##### Navigáció
* Tárolónkénti navigáció.
* Beállítás, amellyel a kurzor a sorok közötti navigáláskor automatikusan a sor elejére kerül, a képernyőolvasók böngészési módjához hasonlóan.
* Az egyenlőségjel billentyűparancs, amely bejelenti, hány százaléknál tart a dokumentumban.

##### Könyvjelzők
* Ideiglenes könyvjelzők: dokumentumonként egy lehet, és megmaradnak. A perjellel állíthat be egyet, a fordított perjellel ugorhat rá.

##### Szavak száma
* Becsült olvasási idő a Szavak száma párbeszédpanelen, valamint az olvasási sebesség beállításának lehetősége, hogy ez az adat valóban hasznos legyen.
* Ha a Szavak száma párbeszédpanel megnyitásakor van kijelölés, most már az is megjelenik, hány szót jelölt ki.

##### Billentyűparancsok
* Az alkalmazás minden billentyűparancsa testreszabható egy egyszerű párbeszédpanelen.
* Beállítható billentyűparancs a Paperback visszaállításához a rendszertálcáról.

##### Nyelvek
* Holland, finn és lengyel.

##### Exportálás
* Az exportálás menüelem kibővült: az egyszerű szöveg mellett most már HTML-ként és Markdownként is exportálhat.

##### Frissítő
* Mégse gomb a frissítés folyamatát mutató párbeszédpanelen.
* A frissítő most már ellenőrzi, hogy a letöltött fájlt nem módosították-e illetéktelenül.

##### Webes nézet
* A webes nézet most már az aktuális olvasási pozíciónál nyílik meg.

##### DAISY-könyvek
* A DAISY 2.0 formátumú könyvek támogatása.
* A DAISY 2.02 formátumú hanganyagok lejátszásának támogatása.

##### Hangoskönyvek
* Hangoskönyvek lejátszása; jelenleg a DAISY-hanganyagok (a hangot és szöveget is tartalmazó DAISY-könyveket is beleértve) és a hangfájlokat tartalmazó ZIP-archívumok támogatottak.
* Billentyűparancsok és menüelemek az audionarráció lejátszásához és szüneteltetéséhez, előre- és visszatekeréséhez, valamint a léptetési időegység módosításához.
* Beállítások a kurzor hanglejátszással való szinkronizálásához, a léptetési időegység megadásához, valamint annak kiválasztásához, hogy a fejezet végén túltekerve a lejátszás a következő fejezettel folytatódjon-e.

##### CHM-dokumentumok
* A listák, a listaelemek, az ábrák és a képek támogatása.

##### PowerPoint
* A PowerPoint-dokumentumok most már támogatják a táblázatokat.

#### Javítások

##### Általános
* A régi CJK-kódolású (például GBK, Big5 és Shift_JIS kódolású) dokumentumok most már helyesen jelennek meg, nem pedig értelmetlen karakterhalmazként (mojibake).
* A „Legutóbb bezárt dokumentum megnyitása” most már nem próbálja újra megnyitni a mellékelt kézikönyvet.
* A Paperback újraindítása után most már helyesen kap fókuszt a kijelölt lap.
* Javult a Windows hálózati meghajtóin lévő fájlok kezelése: a fájl mappában való megjelenítése most már helyesen kijelöli a fájlt a hálózati tárhelyen, és az elérési utakban sincsenek többé furcsa karakterek.
* A program most már nem tölti be kényszerítve a .paperback fájlokat a dokumentumok visszaállításakor; ehelyett megerősítést kér, ha talál egyet.
* A tartalmazó mappa megnyitása most már kijelöli az adott fájlt a Fájlkezelőben.
* A kézikönyv megnyitása most már figyelembe veszi a kiválasztott nyelvet.
* A Paperback felülete most már helyesen méreteződik a nagy DPI-értékű kijelzőkön.
* A menü most már helyesen frissül, és a fókusz a szövegmezőre kerül, amikor a súgót a Paperbackben nyitja meg.
* Windowson a folyamatok közötti kommunikáció (IPC) sokkal biztonságosabb módszerre váltott.
* Lapváltáskor a program most már felolvassa az aktív dokumentum címét.
* Kisebb a memóriahasználat nagy dokumentumoknál, mert a belső, karakterenkénti indextáblák mérete a felére csökkent.

##### Összes dokumentum párbeszédpanel
* Az Escape billentyű most már bezárja a Dokumentuminformáció és az Összes dokumentum párbeszédpanelt.
* Az ablak címsora most már frissül, ha az Összes dokumentum párbeszédpanelről zár be egy dokumentumot.
* A Readme.html most már nem kerül fel az összes dokumentum listájára, ha a Shift+F1 billentyűparanccsal nyitja meg.
* Ha a legutóbbi dokumentumok párbeszédpanelén eltávolít egy dokumentumot, most már a hozzá tartozó lap is bezárul.
* A keresési szűrő most már megmarad egy dokumentum eltávolítása után.

##### Navigáció
* Az oldalankénti navigáció most már bizonyos helyzetekben sem jelent be hibás sorszöveget.
* Az Ugrás sorra, az Ugrás oldalra és az Ugrás adott százalékra most már nagy dokumentumokban sem teszi rossz helyre a kurzort.
* A Keresés és a Következő keresése most már nagy dokumentumokban is figyelembe veszi a betöltött dokumentumablakot.

##### Könyvjelzők
* A könyvjelző- és jegyzethangok most már kizárólag akkor szólalnak meg, ha olyan szóra navigál, amelyhez könyvjelző vagy jegyzet tartozik.

##### Olvashatóság
* A sortörés bekapcsolása most már nem dobja vissza a dokumentum elejére.

##### Webes nézet
* A webes nézet párbeszédpanele most már átméretezhető, és nem nagyon kicsi méretben jelenik meg.
* A képek most már helyesen jelennek meg a beágyazott webes nézetben.

##### Frissítő
* A frissítő most már helyesen jeleníti meg a kiadási megjegyzésekben a Markdown kódcímkéinek tartalmát.

##### DAISY-könyvek
* A DAISY-könyvek most már nem jelenítenek meg hibás adatokat az állapotsorban.
* A hamis kódolási deklarációt tartalmazó DAISY-könyvek most már betölthetők.

##### RTF-dokumentumok
* A nem latin betűs karaktereket tartalmazó RTF-dokumentumok most már helyesen dolgozhatók fel.
* Az RTF `\pict` csoportjai: a beágyazott képadatok most már nem szivárognak át a dokumentum szövegébe.

##### Mobi/AZW3-könyvek
* A Mobi-könyvekben a filepos horgonyok most már nem vágják ketté a HTML-címkéket, és nem kerül szemét a könyv szövegébe.
* A régi Mobi-könyvekben lévő hivatkozások most már működnek.
* Jelentősen javult az AZW3-könyvek feldolgozása.

##### Word-dokumentumok
* A területi beállításoktól függő stílusnevekkel készült Word-dokumentumok címsorai most már helyesen jelennek meg.

##### HTML/XHTML-dokumentumok
* A dl, dt és dd elemek most már sortörést eredményeznek az XHTML-dokumentumokban.

##### PDF-dokumentumok
* A hibásan címkézett PDF-eknél a Paperback most már egyszerű szöveg kinyerésére vált.
* A címükben vagy a könyvjelzőikben vezérlőkaraktereket tartalmazó PDF-dokumentumok most már nem okozzák a Paperback összeomlását megnyitáskor.

### 0.8.5-ös verzió
* Az EPUB-könyvek oldalainak támogatása.
* A titkosított Microsoft Office-dokumentumok támogatása. Jelenleg a régi Word, az új Word és az új PowerPoint formátum támogatott, a régi PowerPoint támogatása a jövőben várható.
* A régi Microsoft Word-dokumentumok támogatása!
* A régi PowerPoint-bemutatók támogatása!
* A MOBI- és AZW3-könyvek támogatása!
* A címkézett PDF-fájlok támogatása!
* Új billentyűparancs: ctrl+q az alkalmazásból való kilépéshez.
* A Bookshare-ről letöltött, ZIP-be tömörített könyvek támogatása (DAISY és Word formátumban is)!
* A beágyazott képek helyettesítő szövege most már helyesen jelenik meg.
* A CHM-dokumentumok most már helyesen támogatják a belső hivatkozásokkal való navigációt.
* Az ugrás oldalra most már nem téved egy oldalt.
* Az Escape billentyű most már bezárja a Megnyitás másként párbeszédpanelt (ez akkor jelenik meg, ha a Paperback nem ismeri fel a fájl formátumát).
* Az olvasó helyi menüje most már megjelenik jobb kattintásra és a helyimenü-billentyűre is.
* Ha a parancssorból nyit meg dokumentumokat, most már nem a rossz dokumentum kap fókuszt.
* A csak képeket tartalmazó PDF-eket a program ismét felismeri, és figyelmeztet rájuk.
* A képek és az ábrák között most már a g/shift+g, illetve az f/shift+f billentyűvel navigálhat.
* A Paperback most már követi az alkalmazások sötét módjára vonatkozó beállítást.
* A DAISY XML támogatása megszűnt, mivel már nincs rá szükség.
* A tartalomjegyzék fanézetében ismét a Win32 natív, első betű szerinti navigációja működik.
* A betöltési hibát jelző párbeszédpanel most már részletesebb hibaüzeneteket jelenít meg.
* A webes nézet most már sokkal gyorsabban és gördülékenyebben nyílik meg.

### 0.8.2-es verzió
* Az RTF-dokumentumok oldalainak támogatása!
* Megszűnt az a hiba, amely miatt a külső hivatkozásokat tartalmazó EPUB-ok webes nézetének megnyitásakor ezek a hivatkozások automatikusan aktiválódtak.
* Megszűnt az a hiba, amely miatt az RTF-elemző ritkán nem tett szóközt a szavak közé.
* Egyes PDF-dokumentumokban a bekezdések most már nem esnek szét több rövid sorra.
* A PDF-dokumentumok most már alapszintű hivatkozás- és címsornavigációt támogatnak!
* Az RTF-tabulátorok és -soremelések most már pontosan úgy jelennek meg, ahogy a dokumentumban.
* A PDF-ek feldolgozása visszakerült a jól bevált pdfium könyvtárra, így a PDF-ek megjelenítése ismét sokkal megbízhatóbb.

### 0.8.1-es verzió
* Új billentyűparancs: Ctrl+Shift+T a legutóbb bezárt dokumentum ismételt megnyitásához.
* Az Összes dokumentum párbeszédpanelen most már több dokumentum is kijelölhető és egyszerre megnyitható.
* Néhány hiba javítása az RTF-elemzőben.
* A nem ASCII karaktereket (például a bosnyák š, č, ć, ž betűket) tartalmazó elérési utak most már nem sérülnek meg, ha a fájlt egy második Paperback-példányon keresztül nyitja meg.
* A PDF-szöveg most már nem rossz sorrendben olvasódik fel, és a nagybetűs szavak körüli térközök is helyesek.
* Megszűnt a nagy fájlok lassú betöltése.
* A megerősítést kérő párbeszédpanelekben az Igen és a Nem gomb most már le van fordítva.

### 0.8.0-s verzió
* Japán, egyszerűsített kínai és vietnami fordítás!
* Automatikus frissítő, amely most már lecseréli a telepített Paperback-verziót, és nem csak letölti az újat!
* Választható hangjelzés könyvjelzőhöz vagy jegyzethez érve – a hangok Andre Louis munkái, köszönjük!
* Az RTF-dokumentumok támogatása!
* A DAISY XML dokumentumok támogatása.
* A Flat OpenDocument-szövegfájlok támogatása!
* A Flat OpenDocument-bemutatók támogatása!
* Az elválasztók támogatása az s és a shift+s billentyűvel.
* Minden 300 karakternél nagyobb elmozdulás most már automatikusan bekerül a navigációs előzményekbe.
* Megszűnt a Paperback ablakának rendszertálcáról való visszaállításával kapcsolatos hiba.
* A Markdown-dokumentumok a webes nézetben most már nem nyers szövegként, hanem HTML-ként jelennek meg.
* A Markdown-fájlokban a táblázatok most már helyesen jelennek meg.
* A csak képeket tartalmazó PDF-ek most már figyelmeztetnek, ha megpróbál betölteni egyet.
* A verzióinformációk most már megfelelően be vannak ágyazva a Paperback programfájljába.
* A Beállítások párbeszédpanel a könnyebb használat és navigáció érdekében lapokra tagolódik.
* A PDF-ek feldolgozása a Hayro könyvtárra váltott, ami nagyobb megbízhatóságot, gyorsaságot és kevesebb DLL-t jelent.
* Az egész alkalmazás újra lett írva Rustban. Az új kódbázis biztonságosabb, gyorsabban tölti be a dokumentumokat, és könnyebb karbantartani és bővíteni.
* A szövegmező helyi menüje most már az olvasóra jellemző műveleteket tartalmazza az olyan általános elemek helyett, mint a kivágás és a beillesztés.

### 0.7.0-s verzió
* A HTML- és XHTML-alapú dokumentumok táblázatainak támogatása! A táblázatok között a T és a Shift+T billentyűvel navigálhat, az Enter billentyűvel pedig webes nézetben tekinthet meg egyet.
* Alapszintű webes megjelenítés! A Ctrl+Shift+V billentyűparanccsal a dokumentum aktuális szakaszát webalapú megjelenítőben nyithatja meg; ez az összetett formázású tartalmakhoz vagy a kódmintákhoz hasznos.
* Orosz fordítás – köszönjük, Ruslan Gulmagomedov!
* Összes eltávolítása gomb az Összes dokumentum párbeszédpanelen.
* Ha új verzió érhető el, a frissítéskereső most már megjeleníti a kiadási megjegyzéseket.
* Megszűnt az ablak rendszertálcáról való visszaállításával kapcsolatos hiba.
* A megerősítést kérő párbeszédpanelek Igen és Nem gombjának fordítása most már helyes.
* A beállítások most már rendszergazdaként futtatva is betöltődnek.
* Javult a megjegyzések kezelése az XML- és HTML-dokumentumokban.
* Javult a tartalomjegyzék feldolgozása az EPUB 2-könyvekben.
* A tartalomjegyzékben most már működik a következő, azonos betűvel kezdődő elemre ugrás.
* A keresési párbeszédpanel most már megfelelően eltűnik, ha az előző/következő gombot használja.
* Az EPUB-tartalomjegyzékek most már nem visznek alkalmanként rossz elemre.
* Megszűnt több, a szóközök kezelésével kapcsolatos hiba az XML- és HTML-dokumentumokban, valamint a pre címkékben.
* Megszűnt egy eggyel való elcsúszási (off-by-one) hiba a hivatkozásonkénti navigációban.
* Egyes könyvek sorainak végéről most már eltűnnek a fölösleges szóközök.
* Különféle elemzőhibák javítása.
* A könyvjelzőkkel kapcsolatos menüelemek és az elemlista most már megfelelően le vannak tiltva, ha nincs megnyitott dokumentum.
* Javult a listák kezelése több dokumentumformátumban.
* Javult a fordítási munkafolyamat a közreműködők számára.
* Számos belső átalakítás: az alkalmazás üzleti logikájának nagy része C++-ból Rustba került a jobb teljesítmény és karbantarthatóság érdekében.

### 0.6.1-es verzió
* A jelszóval védett PDF-ek támogatása!
* Nagyon egyszerű ugrás az előző/következő pozícióra. Ha egy belső hivatkozáson megnyomja az Enter billentyűt, és ez elmozdítja a kurzort, a program megjegyzi a pozíciót, és az alt+balra/jobbra nyíllal odaléphet.
* Elemlista! Jelenleg csak a dokumentum összes címsorának fáját vagy a hivatkozások listáját mutatja, de a jövőben bővül.
* Beállítás, amellyel a Paperback alapértelmezés szerint teljes méretű ablakban indul.
* Egyes EPUB-dokumentumokban most már helyesen működnek a hivatkozások.
* A relatív elérési utakat tartalmazó EPUB-tartalomjegyzékek most már helyesen dolgozhatók fel.
* Egyes EPUB-dokumentumok most már megjelenítik a címüket és a szerzőjüket.
* Egyes EPUB-fejezetek címe most már helyesen jelenik meg a tartalomjegyzékben.
* A tartalomjegyzék párbeszédpanel OK és Mégse gombja most már a szóköz billentyűvel is aktiválható.
* Javult a címsorok kezelése a Word-dokumentumokban.
* Ha a legutóbbi dokumentumok listája üres, a párbeszédpanel megnyitásakor most már hangos visszajelzést kap.

### 0.6.0-s verzió
* Új beállítás a Beállítások párbeszédpanelen, amely sokkal tömörebb formában jeleníti meg az Ugrás menüt; alapértelmezés szerint be van jelölve.
* Beállítás, amellyel a szerkezeti elemek közötti navigáció körkörössé tehető.
* Új elem az Eszközök menüben, amellyel megnyitható az éppen fókuszban lévő dokumentumot tartalmazó mappa.
* Egészen egyszerű, de nagyon hatékony frissítési rendszer.
* Alapszintű elalvási időzítő, amely a Ctrl+Shift+S billentyűparanccsal érhető el.
* Az FB2 formátumú e-könyvek feldolgozásának támogatása!
* Az OpenDocument-bemutatók feldolgozásának támogatása!
* Az OpenDocument-szövegfájlok feldolgozásának támogatása!
* A könyvjelzők most már egy teljes sort vagy csak egy megadott szövegrészt is megjelölhetnek. Ha a könyvjelző elhelyezésekor nincs kijelölés, a működés a 0.6 előttivel egyezik, és a teljes sort jelöli meg. Ha viszont kijelöl egy szövegrészt, csak az kerül a könyvjelzőbe.
* A könyvjelzőkhöz most már szöveges jegyzetek is csatolhatók! A jegyzetet tartalmazó könyvjelzők között az N és a Shift+N billentyűvel navigálhat, vagy külön billentyűparancsokkal megnyithatja a könyvjelzők párbeszédpanelt úgy, hogy az összes könyvjelző, csak a jegyzetek vagy csak a jegyzet nélküli könyvjelzők jelenjenek meg.
* A könyvjelzők párbeszédpanelen a könyvjelzők előtt most már nem áll a zavaró „könyvjelző x” előtag.
* Az XML-nek álcázott HTML-tartalmat tartalmazó EPUB-könyveket a program most már helyesen kezeli.
* A nagy Markdown-dokumentumok most már betölthetők.
* A tartalomjegyzék fanézetében a szóköz most már nem aktiválja az OK gombot.
* Javult a pre címkék elején lévő szóközök kezelése a HTML- és XHTML-dokumentumokban.
* A szövegmező most már mindig visszakapja a fókuszt, amikor visszatér a Paperback ablakába.
* Az ugrás adott százalékra párbeszédpanel szövegmezője most már frissíti a csúszka értékét.
* A Markdown-dokumentumokban az egyéni HTML-azonosítók most már helyesen jelennek meg.
* A Markdown kódblokkjaiban lévő HTML most már helyesen jelenik meg.
* Ha egy már futó Paperback-példány mellett parancssori paraméterrel tölt be egy könyvet, most már akkor sem kap hibaüzenetet, ha a dokumentum betöltése 5 másodpercnél tovább tart.
* Ha a Paperbacket rendszergazdaként futtatja, a beállítások most már helyesen töltődnek be és mentődnek.
* Most már közvetlenül a könyvjelzők párbeszédpanelről is törölhet könyvjelzőt.
* Most már exportálhatja és importálhatja egy adott dokumentum könyvjelzőit és olvasási pozícióját. A létrehozott fájl neve a dokumentum fájlnevével egyezik, .paperback kiterjesztéssel. Ha betöltéskor ilyen fájl található a dokumentummal azonos mappában, a program automatikusan betölti. Ellenkező esetben az Eszközök menü egyik elemével kézzel importálhatja.
* A dokumentumokon belüli hivatkozások most már teljes körűen támogatottak! A k és a shift+k billentyűvel előre és hátra léphet közöttük, az Enter billentyűvel pedig megnyithat vagy aktiválhat egyet.
* Számos belső átalakítás, amely gyorsabbá tette az alkalmazást, és kisebbé a programfájlt.
* A Markdown-tartalmat a program most már a megjelenítés előtt CommonMark-kompatibilissé alakítja.
* A listák és a listaelemek közötti navigáció most már teljes körűen támogatott! Az L és a Shift+L billentyűvel a listák, az I és a Shift+I billentyűvel a listaelemek között léphet.
* A dokumentumokat most már a numerikus billentyűzet Delete billentyűjével is eltávolíthatja a lapsávról, nem csak a normál Delete billentyűvel.
* A Paperback most már a rendszertálcára is kicsinyíthető! Ez a beállítás alapértelmezés szerint ki van kapcsolva; ha bekapcsolja, a rendszermenü Kis méret parancsa a rendszertálcára helyezi a Paperbacket, ahonnan a megjelenő ikonra kattintva állíthatja vissza.
* A Paperback most már teljesen lefordítható! A támogatott nyelvek listája egyelőre elég rövid, de folyamatosan bővül!
* A Paperbacknek most már hivatalos webhelye van: [paperback.dev](https://paperback.dev)!
* A PPTX-dokumentumok most már egyszerű tartalomjegyzéket jelenítenek meg, amely az összes diát tartalmazza.
* A dokumentuminformációs párbeszédpanel most már a megnyitott dokumentum teljes elérési útját is megmutatja.
* A telepítő most már felajánlja, hogy a telepítés után megnyitja a kézikönyvet a böngészőben.
* A legutóbbi dokumentumok listája jelentősen bővült! Ahelyett, hogy csak az utolsó 10 megnyitott dokumentumot mutatná, most már testreszabható számú dokumentumot jelenít meg, a valaha megnyitott többi dokumentum pedig egy kis párbeszédpanelen érhető el.
* Különféle kisebb fejlesztések az összes elemzőben, többek között üres sor a PPTX-bemutatók diái között, a Word-dokumentumok bekezdésein belüli sortörések helyes kezelése, valamint felsorolásjelek a listaelemek előtt.

### 0.5.0-s verzió
* A Microsoft Word-dokumentumok támogatása!
* A PowerPoint-bemutatók támogatása!
* Bizonyos menüelemek most már le vannak tiltva, ha nincs megnyitott dokumentum.
* Az ugrás adott százalékra csúszka iránya most már helyes.
* Az URL-kódolású elérési utakat vagy töredékazonosítókat tartalmazó EPUB-könyvekben most már helyes a tartalomjegyzék.
* Az XHTML-címsorokból most már nem tűnnek el furcsa módon a szóközök.
* Javult a beágyazott pre címkéken belüli szóközök kezelése a HTML-dokumentumokban.
* A HTML- és Markdown-dokumentumok most már támogatják a tartalomjegyzéket! HTML- vagy Markdown-dokumentum betöltésekor a Paperback a dokumentum címsorainak szerkezetéből saját tartalomjegyzéket állít össze, és ezt mutatja meg a ctrl+t párbeszédpanelen.
* A HTML-dokumentumok címe most már a title címkében megadott cím, ha van ilyen. Ellenkező esetben továbbra is a kiterjesztés nélküli fájlnév.
* A beszéd továbbítása a UniversalSpeech helyett most már élő régión keresztül történik. Így a program mellett már nem kell képernyőolvasó-DLL-eket szállítani, és több képernyőolvasó is támogatott, például a Microsoft Narrátor.
* A program másik ZIP-könyvtárra váltott, így az EPUB-könyvek szélesebb köre nyitható meg.
* Teljesen megújult az a párbeszédpanel, amely megkérdezi, hogy egyszerű szövegként szeretné-e megnyitni a dokumentumot: most már egyszerű szövegként, HTML-ként vagy Markdownként is megnyithatja.
* Az ugrás adott százalékra párbeszédpanelen most már szövegmező is van, amelyben kézzel adhatja meg, melyik százalékra szeretne ugrani.
* A HTML-elemző most már listaelemként ismeri fel a dd, dt és dl elemeket.
* Az EPUB-könyvek tartalomjegyzéke ismét pontosan megmarad.
* Az üres sorok eltávolításakor a program most már a Unicode nem törő szóközt is figyelembe veszi.
* Most már nem kell minden betöltéskor megadnia, hogyan szeretne megnyitni egy fel nem ismert fájlt, csak az első alkalommal.

### 0.4.1-es verzió
* A telepítő most már felajánlja, hogy ikont helyezzen el a Start menüben.
* A tartalomjegyzék néhány esetben átláthatóbb lett: ha például egy szülő- és egy gyermekelem ugyanazon a helyen ugyanazzal a szöveggel szerepel, most már csak a szülőelem jelenik meg.
* Egyes CHM-dokumentumokban most már helyes a tartalomjegyzék.
* Az abszolút elérési utakat tartalmazó EPUB 3-könyvekben most már helyes a tartalomjegyzék.
* A CHM-dokumentumok most már a metaadatfájlban megadott címüket jelenítik meg.

### 0.4.0-s verzió
* A CHM-fájlok támogatása!
* Könyvjelzők! Tetszőleges számú dokumentumban tetszőleges számú könyvjelzőt helyezhet el. A b és a shift+b billentyűvel előre és hátra ugorhat közöttük, a control+shift+b billentyűparanccsal beállíthat egyet, a control+b billentyűparanccsal pedig megnyithat egy párbeszédpanelt, amelyben egy adott könyvjelzőre ugorhat.
* A hordozható ZIP-fájl mellett most már telepítő is van! A telepítő a Program Files mappába telepíti a Paperbacket, és automatikusan beállítja a fájltársításokat.
* A BOM-ot tartalmazó szövegfájlok most már helyesen dekódolódnak, és a BOM sem jelenik meg a szöveg elején.
* Sokkal több információ az állapotsorban: most már az aktuális sort, karaktert és olvasási százalékot is mutatja.
* A HTML-megjegyzések, valamint a script és style címkék tartalma most már nem jelenik meg a szövegben.
* Ha a parancssorban relatív elérési utat ad meg a Paperbacknek, most már helyesen oldja fel.
* A százalékos mozgást most már egy saját, csúszkás párbeszédpanel kezeli, amely a control+shift+g billentyűparanccsal érhető el.
* Az ismeretlen című vagy szerzőjű dokumentumoknak most már mindig van alapértelmezett címük és szerzőjük.
* A pozíciómentés logikája most már sokkal okosabb, és csak akkor ír a lemezre, ha feltétlenül szükséges.
* A Paperback bezárásakor fókuszban lévő dokumentumot a program most már az újraindítás után is megjegyzi.
* Az ugrás sorra és az ugrás oldalra párbeszédpanel most már szigorúbban ellenőrzi a beírt értékeket.
* A jegyzékükben relatív elérési utakat tartalmazó EPUB 3-könyvekben most már helyesen működik a tartalomjegyzékkel való navigáció.

### 0.3.0-s verzió
* Az URL-kódolású jegyzéket tartalmazó EPUB-könyvekben most már helyes a tartalomjegyzék.
* A többbájtos Unicode-karaktereket tartalmazó HTML-dokumentumokban most már helyesen működik a címsoronkénti navigáció.
* Megszűnt a hosszú című dokumentumoknál a wxWidgets egy regressziója miatt jelentkező magas processzorhasználat.
* Az UTF-8 kódolású szövegfájlok most már helyesen töltődnek be.
* Az EPUB-könyvekben a beágyazott tartalomjegyzék-elemek most már nem rossz helyre teszik a kurzort.
* Megszűnt egy összeomlás, amely bizonyos esetekben az alkalmazásból való kilépéskor történt.
* Jelölőnégyzet a Beállítások párbeszédpanelen a sortörés be- és kikapcsolásához!
* Most már adományozhat a Paperback fejlesztésére: a Súgó menü új Adományozás elemével vagy a GitHub-tárház főoldalának alján található „Sponsor this project” hivatkozással.
* A Markdown-dokumentumoknak most már mindig van címük, és a Paperback gyakorlatilag bármilyen Markdown-fájlt be tud tölteni.
* A PDF-dokumentumoknak most már akkor is mindig van címük, ha hiányoznak a metaadatok.
* A Paperback a Chromiumban használt PDF-könyvtárra váltott, így a PDF-ek feldolgozása mindenhol sokkal megbízhatóbb.
* A Paperbackből most már egyszerre csak egy példány futhat. Ha a paperback.exe fájlt egy fájlnévvel futtatja, miközben a program már fut, a dokumentum a már futó példányban nyílik meg.
* A lapsávon most már a Delete billentyűvel bezárhat egy dokumentumot.

### 0.2.1-es verzió
* Az ugrás oldalra párbeszédpanel oldalcímkéje most már az oldalak teljes számát is mutatja.
* A Tab billentyűvel most már a dokumentum tartalmából a megnyitott dokumentumok listájára léphet.
* A címsoronkénti navigáció billentyűi most már nem nyitnak meg legutóbbi dokumentumokat, ha elég sok van belőlük.
* A Paperback most már eltávolítja a fölösleges feltételes elválasztójeleket a szövegből.
* A címsoronkénti navigáció most már nem teszi néha rossz karakterre a kurzort.

### 0.2.0-s verzió
* A Markdown-dokumentumok támogatása!
* A PDF-dokumentumok támogatása, az oldalak közötti navigációval együtt!
* Billentyűparancsok a HTML-tartalmak címsorai közötti navigáláshoz, az EPUB-könyveket és a Markdown-dokumentumokat is beleértve. Ezek a billentyűparancsok a képernyőolvasókéhoz hasonlóan működnek.
* Az URL-kódolású fájlneveket tartalmazó jegyzékű EPUB-ok most már betölthetők.
* A beágyazott XHTML-t tartalmazó EPUB 3-könyvek most már betölthetők.
* Ha a dokumentum nem támogatja a tartalomjegyzéket vagy a szakaszokat, a program most már egy kimondott üzenettel jelzi ezt, ahelyett hogy letiltaná a menüelemeket.
* Legutóbbi dokumentumok menü! Jelenleg az utolsó 10 megnyitott dokumentumot tárolja, és bármelyiket megnyithatja olvasásra az Enter billentyűvel.
* Teljesen újraírt Keresés párbeszédpanel: sokkal egyszerűbben használható, megjegyzi az utolsó 25 keresést, és támogatja a reguláris kifejezéseket!
* A korábban megnyitott dokumentumokat a program most már az újraindítások között is megjegyzi. Ez az Eszközök menü új Beállítások elemével módosítható.
* Új billentyűparancs: shift+f1 a kézikönyv közvetlenül a Paperbackben való megnyitásához.

### 0.1.0-s verzió
* Első kiadás.
