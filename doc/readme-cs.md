# Paperback - verze 0.8.5

## Představení

Paperback je nenáročná, rychlá a přístupná čtečka e-knih a dokumentů určená komukoli, od příležitostných čtenářů až po náročné pokročilé uživatele. Je navržen pro maximální podporu odečítačů obrazovky, rychlou odezvu a uživatelský zážitek bez rušivých prvků.

## Systémové požadavky

Paperback v současnosti funguje na Windows 10 a 11. Podpora macOS a Linuxu je v plánu.

## Funkce

* Kompletně soběstačný program, který nevyžaduje mít v počítači nainstalovaný žádný další software k tomu, abyste mohli prostě začít číst.
* Neuvěřitelně rychlý, dokonce i na starém hardwaru.
* Jednoduché vícepanelové rozhraní, takže si můžete vedle sebe otevřít, kolik dokumentů chcete.
* Ukládá místo, kde jste přestali, v každém dokumentu, který otevřete.
* Volitelně si pamatuje, které dokumenty jste měli při zavření programu otevřené, a při příštím spuštění je znovu načte.
* Navržen uživatelem odečítače pro ostatní uživatele odečítačů.
* Obsahuje funkce rychlé navigace, podobné režimu procházení webu ve většině odečítačů, pro rychlý a snadný pohyb v dokumentech.
* Obsahuje robustní dialog pro vyhledávání, včetně funkcí, jako je historie nebo podpora regulárních výrazů.
* Lze spouštět v přenosné verzi nebo nainstalovat i s automatickým nastavením přidružených typů souborů.

## Aktuálně podporované typy souborů

Paperback podporuje následující formáty a přípony:

* CHM soubory nápovědy (`.chm`)
* E-knihy ve formátu EPUB (`.epub`)
* E-knihy ve formátu FB2 (`.fb2`)
* HTML dokumenty (`.htm`, `.html`, `.xhtml`)
* Dokumenty ve formátu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Dokumenty Microsoft Wordu (`.docx`, `.docm`)
* OpenDocument prezentace (`.odp`, `.fodp`)
* OpenDocument textové soubory (`.odt`, `.fodt`)
* PDF dokumenty (`.pdf`)
* PowerPointové prezentace (`.pptx`, `.pptm`)
* RTF dokumenty (`.rtf`)
* Soubory v prostém textu a log soubory (`.txt`, `.log`)
* XML dokumenty (`.xml`)

## Klávesové zkratky

Paperback je navržen především pro použití s klávesnicí a s odečítačem obrazovky. Toto jsou aktuální klávesové zkratky:

### Nabídka Soubor

* `Ctrl+O`: Otevření dokumentu.
* `Ctrl+F4`: Zavření aktuálního dokumentu.
* `Ctrl+Shift+F4`: Zavření všech otevřených dokumentů.
* `Ctrl+R`: Zobrazení dialogu "Všechny dokumenty" (z nabídky Nedávné dokumenty).

### Nabídka Přejít

* `Ctrl+F`: Zobrazení dialogu Najít.
* `F3`: Najít další.
* `Shift+F3`: Najít předchozí.
* `Ctrl+G`: Přejít na řádek.
* `Ctrl+Shift+G`: Přejít na procenta.
* `Ctrl+P`: Přejít na stránku (pokud je podporováno v aktuálním dokumentu).
* `Alt+Left`: Posun zpět v historii navigace.
* `Alt+Right`: Posun vpřed v historii navigace.
* `[`: Předchozí oddíl.
* `]`: Další oddíl.
* `Shift+H`: Předchozí nadpis.
* `H`: Další nadpis.
* `Shift+1` až `Shift+6`: Předchozí nadpis úrovně 1 až 6.
* `1` až `6`: Další nadpis úrovně 1 až 6.
* `Shift+P`: Předchozí stránka.
* `P`: Další stránka.
* `Shift+B`: Předchozí záložka.
* `B`: Další záložka.
* `Shift+N`: Předchozí poznámka.
* `N`: Další poznámka.
* `Ctrl+B`: Zobrazení všech záložek a poznámek.
* `Ctrl+Alt+B`: Zobrazení pouze záložek.
* `Ctrl+Alt+M`: Zobrazení pouze poznámek.
* `Ctrl+Shift+W`: Zobrazení textu poznámky na aktuální pozici.
* `Shift+K`: Předchozí odkaz.
* `K`: Další odkaz.
* `Shift+T`: Předchozí tabulka.
* `T`: Další tabulka.
* `Shift+S`: Předchozí oddělovač.
* `S`: Další oddělovač.
* `Shift+L`: Předchozí seznam.
* `L`: Další seznam.
* `Shift+I`: Předchozí položka seznamu.
* `I`: Další položka seznamu.

### Nabídka Nástroje

* `Ctrl+W`: Zobrazení počtu slov v aktuálním dokumentu.
* `Ctrl+I`: Zobrazení informací o dokumentu.
* `Ctrl+T`: Zobrazení obsahu (osnovy) dokumentu.
* `F7`: Zobrazení seznamu prvků.
* `Ctrl+Shift+C`: Otevření nadřazené složky (kde je aktuální dokument uložen).
* `Ctrl+Shift+V`: Otevření aktuálního dokumentu ve webovém zobrazení.
* `Ctrl+Shift+E`: Export dat dokumentu (`.paperback`).
* `Ctrl+Shift+I`: Import dat dokumentu (`.paperback`).
* `Ctrl+E`: Export aktuálního dokumentu do prostého textu.
* `Ctrl+Shift+B`: Přepnutí záložky v aktuálním výběru / na aktuální pozici.
* `Ctrl+Shift+N`: Přidání nebo úpravy poznámky k záložce v aktuálním výběru / na aktuální pozici.
* `Ctrl+,`: Otevření dialogu Možnosti.
* `Ctrl+Shift+S`: Přepnutí časovače spánku.

### Nabídka Nápověda

* `Ctrl+F1`: Zobrazení dialogu O Paperbacku.
* `F1`: Zobrazení této nápovědy ve výchozím webovém prohlížeči.
* `Shift+F1`: Zobrazení této nápovědy přímo v Paperbacku.
* `Ctrl+Shift+U`: Vyhledání aktualizací.
* `Ctrl+D`: Otevření stránky podpory ve výchozím webovém prohlížeči.

### Další zkratky v zobrazení dokumentu

* `Delete` / `Numpad Delete` na seznamu záložek: Zavření záložky (panelu) vybraného dokumentu.
* `Enter` v textu dokumentu: Aktivuje odkaz pod kurzorem nebo otevře zobrazení tabulky, pokud se kurzor nachází na značce tabulky.
* `Shift+F10` v textu dokumentu: Otevření kontextové nabídky.

## Podporované jazyky

Paperback je přeložen do mnoha různých jazyků a další neustále přibývají. Následuje kompletní výčet:

* Bosenština
* Čeština
* Francouzština
* Němčina
* Japonština
* Ruština
* Zjednodušená čínština
* Srbština
* Španělština
* Vietnamština

## Poděkování
### Vývoj
* Quin Gillespie: Primární vývojář a zakladatel projektu.
* Aryan Choudhary: primární přispěvatel.

### Podpora
Následující lidé podpořili další vývoj Paperbacku finančním příspěvkem. Pokud přispějete i vy, vaše jméno se zde automaticky neobjeví. Uvádím pouze lidi, kteří si přáli svůj příspěvek zveřejnit.

Upozornění: Veřejné sponzory na GitHubu automaticky považuji za určené ke zveřejnění.

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

## Historie změn

### Verze 0.8.5
* Přidána podpora PowerPointových prezentací ve starém formátu (*.ppt)!
* Přidána podpora starého formátu dokumentů Microsoft Wordu (*.doc)!
* Přidána podpora zazipovaných knih z Bookshare (DAISY i Word)! [#36](https://github.com/trypsynth/paperback/issues/36), [#358](https://github.com/trypsynth/paperback/pull/358), [#360](https://github.com/trypsynth/paperback/pull/360).
* Paperback bude nyní respektovat vaše nastavení tmavého režimu pro aplikace.
* Odstraněna podpora DAISY XML, protože už není potřeba.
* Dialog o chybě při spuštění nyní zobrazuje podrobnější chybové hlášky.
* Webové zobrazení se nyní bude načítat mnohem rychleji a hladčeji. [#359](https://github.com/trypsynth/paperback/pull/359).

### Verze 0.8.2
* Přidána podpora stránek pro RTF dokumenty!
* Opravena chyba, kdy se při otevření EPUB dokumentu ve webovém zobrazení automaticky aktivovaly odkazy, pokud je dokument obsahoval.
* Opravena chyba, kdy RTF parser ve vzácných případech nevkládal mezery mezi slova.
* Opraveno rozdělování odstavců v některých PDF dokumentech na několik krátkých řádků. [#101](https://github.com/trypsynth/paperback/issues/101), [#355](https://github.com/trypsynth/paperback/pull/355).
* PDF dokumenty teď mají základní podporu pro pohyb po odkazech a po nadpisech! [#291](https://github.com/trypsynth/paperback/issues/291), [#353](https://github.com/trypsynth/paperback/pull/353), [#354](https://github.com/trypsynth/paperback/pull/354).
* RTF tabulátory a konce řádků se teď vykreslují přesně tak, jak se v dokumentu vyskytují.
* Přešel jsem zpátky na vyzkoušenou a osvědčenou knihovnu pdfium pro parsování PDF souborů, takže je jejich renderování opět výrazně spolehlivější.

### Verze 0.8.1
* Přidána klávesová zkratka Ctrl+Shift+T pro znovuotevření naposledy zavřeného dokumentu. [#343](https://github.com/trypsynth/paperback/issues/343).
* Dialog Všechny dokumenty teď podporuje výběr více dokumentů, které se mají otevřít najednou. [#344](https://github.com/trypsynth/paperback/issues/344).
* Opraveno několik chyb v RTF parseru. [#345](https://github.com/trypsynth/paperback/issues/345).
* Opravena chyba, kdy cesty k souborům obsahující jiné než ascii znaky (například bosenské š, č, ć, ž) přestaly fungovat při otevření stejného souboru ve druhé instanci Paperbacku. [#346](https://github.com/trypsynth/paperback/issues/346).
* Opraveno čtení textu z PDF v nesprávném pořadí a chybové vkládání mezer okolo slov s velkými písmeny. [#44](https://github.com/trypsynth/paperback/issues/44).
* Opraveno pomalé načítání dokumentů při otevírání velkých souborů.
* Opravena lokalizace tlačítek Ano a Ne v potvrzovacích dialozích. [#285](https://github.com/trypsynth/paperback/issues/285).

### Verze 0.8.0
* Přidány lokalizace pro japonštinu, vietnamštinu a zjednodušenou čínštinu! [#300](https://github.com/trypsynth/paperback/pull/300), [#326](https://github.com/trypsynth/paperback/pull/326), [#335](https://github.com/trypsynth/paperback/pull/335).
* Přidán automatický updater, který nyní nahradí aktuálně nainstalovanou verzi Paperbacku, místo aby jenom stáhnul novou verzi! [#323](https://github.com/trypsynth/paperback/pull/323).
* Přidána volitelná zvuková odezva pro přesun na záložku nebo poznámku (za zvuky děkuji Andre Louisovi)! [#110](https://github.com/trypsynth/paperback/issues/110).
* Přidána podpora RTF dokumentů! [#26](https://github.com/trypsynth/paperback/issues/26).
* Přidána podpora dokumentů ve formátu DAISY XML. [#136](https://github.com/trypsynth/paperback/issues/136).
* Přidána podpora textových souborů Flat Open Document Text!
* Přidána podpora Flat Open Document prezentací!
* Přidána podpora oddělovačů a zkratky S a shift+S. [#294](https://github.com/trypsynth/paperback/issues/294).
* Každý posun o víc než 300 znaků se nyní automaticky přidá do historie navigace. [#179](https://github.com/trypsynth/paperback/issues/179).
* Opraveno obnovování okna Paperbacku ze systémové lišty. [#284](https://github.com/trypsynth/paperback/issues/284).
* Opravena chyba, kdy se ve webovém zobrazení pro dokumenty v Markdownu zobrazoval původní text místo renderovaného HTML.
* Opraveno nesprávné renderování tabulek v Markdown souborech. [#303](https://github.com/trypsynth/paperback/issues/303).
* PDF, která jsou pouze obrázková, vás na tuto skutečnost nyní upozorní, když se takový soubor pokusíte načíst. [#89](https://github.com/trypsynth/paperback/issues/89).
* Při kontrole aktualizací je nyní možné vyhledávat kromě stabilních verzí i vývojové verze. [#333](https://github.com/trypsynth/paperback/pull/333).
* Ve spustitelném souboru Paperbacku jsou správně uvedeny informace o verzi. [#204](https://github.com/trypsynth/paperback/issues/204).
* Dialog Možnosti byl rozdělen do záložek pro snadnější používání a pohodlnější navigaci.
* Přešel jsem na knihovnu Hayro pro parsování souborů PDF, což vede k větší spolehlivosti, rychlejší odezvě a menšímu počtu DLL knihoven.
* Přepsal jsem celou aplikaci do Rustu. Nový kód je bezpečnější, načítá dokumenty rychleji a je ho jednodušší udržovat a rozšiřovat.
* Kontextová nabídka na textovém prvku s obsahem dokumentu teď obsahuje příkazy specifické pro čtečku a ne obecné příkazy jako Vyjmout nebo Vložit. [#114](https://github.com/trypsynth/paperback/issues/114).

### Verze 0.7.0
* Přidána podpora tabulek pro dokumenty založené na HTML a XHTML! Mezi tabulkami se můžete pohybovat pomocí T a Shift+T a stisknutím Enteru si některou zobrazit ve webovém zobrazení. [#81](https://github.com/trypsynth/paperback/issues/81), [#98](https://github.com/trypsynth/paperback/pull/98), [#226](https://github.com/trypsynth/paperback/pull/226), [#228](https://github.com/trypsynth/paperback/pull/228).
* Přidány základy funkce webového vykreslování! Stisknutím Ctrl+Shift+V otevřete aktuální část dokumentu ve webovém zobrazení. Může se to hodit zejména u obsahu, jako je složité formátování nebo ukázky kódu. [#188](https://github.com/trypsynth/paperback/issues/188), [#239](https://github.com/trypsynth/paperback/pull/239).
* Přidán ruský překlad (díky Ruslanu Gulmagomedovovi)! [#211](https://github.com/trypsynth/paperback/pull/211), [#212](https://github.com/trypsynth/paperback/pull/212).
* Do dialogu Všechny dokumenty přidáno tlačítko Vymazat vše. [#217](https://github.com/trypsynth/paperback/issues/217).
* Kontrola aktualizací nyní při dostupnosti nové verze zobrazuje poznámky k vydání. [#210](https://github.com/trypsynth/paperback/pull/210).
* Aktualizován srbský překlad. [#219](https://github.com/trypsynth/paperback/pull/219), [#229](https://github.com/trypsynth/paperback/pull/229).
* Aktualizován bosenský překlad. [#218](https://github.com/trypsynth/paperback/pull/218), [#225](https://github.com/trypsynth/paperback/pull/225).
* Opraveno obnovování okna ze systémové lišty. [#284](https://github.com/trypsynth/paperback/issues/284).
* Opraveny překlady tlačítek Ano/Ne v potvrzovacích dialozích. [#285](https://github.com/trypsynth/paperback/issues/285).
* Opraveno načítání konfigurace při spuštění jako správce. [#201](https://github.com/trypsynth/paperback/issues/201).
* Opraveno zpracování komentářů v dokumentech XML a HTML. [#198](https://github.com/trypsynth/paperback/issues/198).
* Opraveno parsování obsahu v knihách Epub 2. [#192](https://github.com/trypsynth/paperback/pull/192).
* Opraven přechod na další položku se stejným písmenem v obsahu. [#191](https://github.com/trypsynth/paperback/pull/191).
* Opravena chyba, kdy se dialog Hledat při použití tlačítek další/předchozí neskrýval správně.
* Opravena chyba, kdy vás obsah v epub dokumentech občas přesunul na nesprávnou položku.
* Opraveny různé problémy se zpracováním bílých znaků v XML, HTML a značkách pre.
* Opraven posun o jeden odkaz mimo při navigaci po odkazech.
* Opravena chyba, kdy některé knihy měly na konci řádků nadbytečné bílé znaky.
* Opraveny různé problémy parseru. [#208](https://github.com/trypsynth/paperback/pull/208).
* Položky nabídky související se záložkami jsou nyní správně zakázány, když není otevřen žádný dokument. [#196](https://github.com/trypsynth/paperback/pull/196).
* Seznam prvků je nyní správně zakázán, když není otevřen žádný dokument. [#194](https://github.com/trypsynth/paperback/issues/194).
* Vylepšeno zpracování seznamů v různých formátech dokumentů. [#213](https://github.com/trypsynth/paperback/pull/213).
* Vylepšen pracovní postup pro překladatele. [#270](https://github.com/trypsynth/paperback/issues/270).
* Mnoho interních refaktorizací, při nichž byla většina aplikační logiky přesunuta z C++ do Rustu kvůli vyššímu výkonu a lepší udržitelnosti.

### Verze 0.6.1

* Přidána podpora PDF chráněných heslem! [#169](https://github.com/trypsynth/paperback/issues/169).
* Přidána velmi základní funkce pro přesun na předchozí/další pozici. Pokud stisknete Enter na vnitřním odkazu a kurzor se přesune, tato pozice se nyní zapamatuje a lze se na ni vracet pomocí Alt+šipka vlevo/vpravo. [#115](https://github.com/trypsynth/paperback/issues/115), [#174](https://github.com/trypsynth/paperback/pull/174).
* Přidán seznam prvků! Momentálně zobrazuje jen strom všech nadpisů v dokumentu nebo seznam odkazů, ale do budoucna počítám s jeho rozšířením. [#173](https://github.com/trypsynth/paperback/issues/173), [#177](https://github.com/trypsynth/paperback/pull/177).
* Přidána možnost spouštět Paperback ve výchozím nastavení maximalizovaný. [#164](https://github.com/trypsynth/paperback/issues/164), [#172](https://github.com/trypsynth/paperback/pull/172).
* Opravena chyba, kdy odkazy v některých dokumentech Epub nefungovaly správně. [#167](https://github.com/trypsynth/paperback/issues/167), [#171](https://github.com/trypsynth/paperback/pull/171), [#178](https://github.com/trypsynth/paperback/issues/178), [#180](https://github.com/trypsynth/paperback/pull/180).
* Opraveno parsování obsahu v Epub souborech obsahujících relativní cesty. [#187](https://github.com/trypsynth/paperback/issues/187).
* Opravena chyba, kdy se u některých epub dokumentů nezobrazoval název nebo autor. [#175](https://github.com/trypsynth/paperback/issues/175).
* Opravena chyba, kdy se názvy některých kapitol v epub dokumentech v dialogu obsahu nezobrazovaly správně. [#176](https://github.com/trypsynth/paperback/pull/176).
* Opravena chyba, kdy v dialogu obsahu nebylo možné aktivovat tlačítka OK/Zrušit mezerníkem. [#170](https://github.com/trypsynth/paperback/issues/170).
* Vylepšeno zpracování nadpisů v dokumentech Microsoft Wordu. [#183](https://github.com/trypsynth/paperback/pull/183).
* Pokud se pokusíte vyvolat dialog Nedávné dokumenty a seznam bude prázdný, dostanete nyní hlasovou odezvu. [#185](https://github.com/trypsynth/paperback/issues/185).

### Verze 0.6.0

* Do dialogu Možnosti přidána nová volba, která zobrazuje nabídku Přejít v mnohem kompaktnější podobě; ve výchozím stavu je zapnutá.
* Přidána možnost cyklické rychlé navigace po strukturních prvcích. [#116](https://github.com/trypsynth/paperback/pull/116).
* Do nabídky Nástroje byla přidána možnost otevřít složku obsahující právě fokusovaný dokument. [#142](https://github.com/trypsynth/paperback/pull/142).
* Přidán poměrně jednoduchý, ale velmi účinný systém aktualizací. [#28](https://github.com/trypsynth/paperback/issues/28).
* Přidána základní funkce časovače spánku, dostupná pomocí Ctrl+Shift+S. [#117](https://github.com/trypsynth/paperback/issues/117), [#118](https://github.com/trypsynth/paperback/pull/118).
* Přidána podpora parsování e-knih ve formátu FB2! [#30](https://github.com/trypsynth/paperback/issues/30), [#107](https://github.com/trypsynth/paperback/pull/107).
* Přidána podpora parsování OpenDocument prezentací! [#105](https://github.com/trypsynth/paperback/issues/105), [#106](https://github.com/trypsynth/paperback/pull/106).
* Přidána podpora parsování OpenDocument textových souborů! [#29](https://github.com/trypsynth/paperback/issues/29), [#90](https://github.com/trypsynth/paperback/pull/90).
* Záložky nyní mohou označovat celý řádek nebo jen vybranou část textu. Pokud při vytvoření záložky nemáte nic označeno, chová se to stejně jako před verzí 0.6 a označí se celý řádek. Pokud ale nějaký text vyberete, bude do záložky zahrnut pouze tento text. [#99](https://github.com/trypsynth/paperback/issues/99).
* K záložkám lze nyní připojit volitelné textové poznámky! Mezi záložkami obsahujícími poznámky se můžete pohybovat pomocí N a Shift+N, případně můžete vyvolat dialog záložek se zobrazením všech záložek, jen poznámek nebo jen záložek bez poznámek pomocí konkrétních klávesových zkratek. [#68](https://github.com/trypsynth/paperback/issues/68), [#128](https://github.com/trypsynth/paperback/issues/128), [#156](https://github.com/trypsynth/paperback/issues/156), [#157](https://github.com/trypsynth/paperback/issues/157), [#158](https://github.com/trypsynth/paperback/pull/158), [#159](https://github.com/trypsynth/paperback/issues/159), [#161](https://github.com/trypsynth/paperback/pull/161).
* Záložky v dialogu Záložky už nebudou mít obtěžující předponu „záložka x“. [#86](https://github.com/trypsynth/paperback/issues/86).
* Knihy Epub obsahující HTML kód, který se tváří jako XML, budou nyní zpracovávány správně. [#96](https://github.com/trypsynth/paperback/issues/96).
* Opraveno načítání velkých dokumentů Markdown. [#97](https://github.com/trypsynth/paperback/issues/97).
* Opravena chyba, kdy stisknutí mezerníku ve stromovém zobrazení obsahu aktivovalo tlačítko OK. [#121](https://github.com/trypsynth/paperback/issues/121), [#123](https://github.com/trypsynth/paperback/pull/123).
* Opraveno zpracování bílých znaků na začátku značek pre v dokumentech HTML i XHTML.
* Opravena chyba, kdy textové pole někdy po návratu do okna Paperbacku znovu nezískalo fokus. [#138](https://github.com/trypsynth/paperback/issues/138).
* Opravena chyba, kdy textové pole v dialogu Přejít na procenta neaktualizovalo hodnotu posuvníku.
* Opraveno vykreslování vlastních HTML ID v dokumentech Markdown. [#113](https://github.com/trypsynth/paperback/issues/113).
* HTML uvnitř bloků kódu v Markdownu se nyní bude vykreslovat správně. [#79](https://github.com/trypsynth/paperback/issues/79).
* Při načítání knihy pomocí parametru příkazového řádku za běhu již existující instance Paperbacku už nedostanete chybu, pokud načtení dokumentu trvá déle než 5 sekund.
* Pokud Paperback spouštíte jako správce, konfigurace se nyní bude správně načítat i ukládat. [#148](https://github.com/trypsynth/paperback/issues/148), [#149](https://github.com/trypsynth/paperback/pull/149).
* Nyní je možné smazat záložku přímo z dialogu Záložky. [#100](https://github.com/trypsynth/paperback/issues/100), [#103](https://github.com/trypsynth/paperback/pull/103).
* Nyní je možné importovat a exportovat vaše záložky a pozici čtení pro konkrétní dokument. Vygenerovaný soubor se pojmenuje podle daného souboru s příponou .paperback. Pokud se takový soubor při načítání najde ve stejné složce jako dokument, načte se automaticky. Jinak ho můžete importovat ručně pomocí položky v nabídce Nástroje. [#146](https://github.com/trypsynth/paperback/issues/146), [#147](https://github.com/trypsynth/paperback/pull/147).
* Odkazy uvnitř dokumentů jsou nyní plně podporovány! Pomocí K a Shift+K se mezi nimi můžete pohybovat vpřed a vzad a stisknutím Enteru některý otevřít nebo aktivovat. [#74](https://github.com/trypsynth/paperback/issues/74), [#87](https://github.com/trypsynth/paperback/pull/87), [#126](https://github.com/trypsynth/paperback/issues/126), [#129](https://github.com/trypsynth/paperback/issues/129), [#130](https://github.com/trypsynth/paperback/issues/130).
* Mnoho interních refaktorizací, díky nimž je aplikace rychlejší a binární soubor menší.
* Obsah v Markdownu se nyní před vykreslením předzpracovává tak, aby odpovídal CommonMarku.
* Navigace po seznamech a jejich položkách je nyní plně podporována! Pomocí L a Shift+L přecházíte po samotných seznamech a pomocí I a Shift+I po položkách seznamu. [#119](https://github.com/trypsynth/paperback/issues/119), [#124](https://github.com/trypsynth/paperback/pull/124).
* Kromě běžné klávesy Delete nyní také funguje klávesa Delete na numerické klávesnici pro odstraňování dokumentů ze seznamu záložek.
* Paperback se nyní může volitelně minimalizovat do systémové lišty! Tato volba je ve výchozím stavu vypnutá, ale po jejím zapnutí přesune možnost minimalizace v systémové nabídce Paperback do lišty, odkud jej bude možné obnovit kliknutím na vytvořenou ikonu. [#49](https://github.com/trypsynth/paperback/issues/49), [#85](https://github.com/trypsynth/paperback/pull/85).
* Paperback je nyní plně přeložitelný! Seznam podporovaných jazyků je zatím poměrně malý, ale neustále roste! [#75](https://github.com/trypsynth/paperback/issues/75), [#92](https://github.com/trypsynth/paperback/pull/92), [#95](https://github.com/trypsynth/paperback/pull/95), [#134](https://github.com/trypsynth/paperback/pull/134), [#137](https://github.com/trypsynth/paperback/pull/137), [#141](https://github.com/trypsynth/paperback/pull/141), [#152](https://github.com/trypsynth/paperback/pull/152).
* Paperback má nyní oficiální web na adrese [paperback.dev](https://paperback.dev)!
* Dokumenty PPTX nyní budou zobrazovat základní obsah všech slidů. [#122](https://github.com/trypsynth/paperback/issues/122).
* V dialogu s informacemi o dokumentu se nyní bude zobrazovat úplná cesta k otevřenému dokumentu. [#139](https://github.com/trypsynth/paperback/issues/139), [#140](https://github.com/trypsynth/paperback/pull/140).
* Instalační program nyní obsahuje možnost po instalaci zobrazit readme v prohlížeči.
* Seznam nedávných dokumentů byl výrazně rozšířen! Místo pouhého zobrazení posledních 10 otevřených dokumentů nyní zobrazí nastavitelný počet a ostatní dokumenty, které jste kdy otevřeli, budou dostupné přes malý dialog. [#78](https://github.com/trypsynth/paperback/issues/78), [#80](https://github.com/trypsynth/paperback/pull/80), [#84](https://github.com/trypsynth/paperback/pull/84), [#135](https://github.com/trypsynth/paperback/pull/135).
* Různá drobná vylepšení parserů napříč aplikací, včetně vložení prázdného řádku mezi slidy v prezentacích PPTX, opravy zpracování nových řádků uvnitř odstavců v dokumentech Wordu a přidání odrážek k položkám seznamů.

### Verze 0.5.0

* Přidána podpora dokumentů Microsoft Wordu! [#27](https://github.com/trypsynth/paperback/issues/27).
* Přidána podpora PowerPointových prezentací! [#25](https://github.com/trypsynth/paperback/issues/25).
* Opravena chyba, kdy některé položky nabídky nebyly zakázány, když nebyl otevřen žádný dokument.
* Opravena orientace posuvníku v dialogu Přejít na procenta. [#70](https://github.com/trypsynth/paperback/issues/70).
* Opraven obsah v knihách Epub s URL-kódovanými cestami k souborům a/nebo ID fragmentů.
* Opraveno podivné odstraňování bílých znaků z nadpisů XHTML.
* Opraveno zpracování bílých znaků uvnitř vnořených značek pre v dokumentech HTML.
* Dokumenty HTML a Markdown nyní podporují funkci obsahu! Když načtete dokument HTML/Markdown, Paperback sestaví vlastní obsah ze struktury nadpisů v dokumentu a zobrazí vám ho v dialogu Ctrl+T.
* Dokumenty HTML nyní budou mít název nastavený podle značky title, pokud existuje. Jinak se nadále použije název souboru bez přípony.
* Místo UniversalSpeech se nově používá live region pro oznamování řeči. To znamená, že se spolu s programem už nedodávají žádné DLL soubory pro odečítače obrazovky a bude nyní podporováno více odečítačů, například Microsoft Narrator.
* Změněny ZIP knihovny, aby bylo možné otevírat širší škálu knih Epub. [#73](https://github.com/trypsynth/paperback/issues/73).
* Dialog, který se vás ptá, zda chcete dokument otevřít jako prostý text, byl úplně přepracován a nyní umožňuje otevřít dokument jako prostý text, HTML nebo Markdown.
* Dialog Přejít na procenta nyní obsahuje textové pole, do něhož lze ručně zadat procento, na které se má přeskočit. [#66](https://github.com/trypsynth/paperback/issues/66).
* HTML parser nyní rozpozná dd, dt a dl jako prvky seznamu.
* Obsah v knihách Epub bude opět zachován přesně.
* Unicode nezalomitelná mezera se nyní bere v úvahu při odstraňování prázdných řádků. [#71](https://github.com/trypsynth/paperback/issues/71).
* Program se vás už nebude ptát, jak chcete otevřít nerozpoznaný soubor, při každém otevření, ale jen při prvním.

### Verze 0.4.1

* Do instalátoru byla přidána volitelná ikona v nabídce Start.
* Zobrazování obsahu by nyní mělo být v některých případech čistší; například pokud máte podřízenou i nadřazenou položku se stejným textem na stejné pozici, uvidíte nyní jen nadřazenou položku.
* Opraven obsah u některých dokumentů CHM.
* Opraven obsah v knihách Epub 3, které obsahovaly absolutní cesty. [#67](https://github.com/trypsynth/paperback/issues/67).
* Dokumenty CHM by nyní měly zobrazovat název nastavený v souboru metadat.

### Verze 0.4.0

* Přidána podpora souborů CHM! [#23](https://github.com/trypsynth/paperback/issues/23).
* Přidána podpora záložek! Můžete jich mít libovolné množství v libovolném počtu dokumentů. Můžete mezi nimi skákat vpřed a vzad pomocí B a Shift+B, nastavovat je pomocí Ctrl+Shift+B a vyvolat dialog pro skok na konkrétní záložku pomocí Ctrl+B. [#13](https://github.com/trypsynth/paperback/issues/13).
* Kromě portable ZIP souboru byl přidán i instalátor! Ten nainstaluje Paperback do adresáře Program Files a automaticky za vás nastaví asociace souborů. [#33](https://github.com/trypsynth/paperback/issues/33).
* Textové soubory se značkou BOM by se nyní měly dekódovat správně a BOM se už nebude zobrazovat ani na začátku textu.
* Do stavového řádku bylo přidáno mnohem více informací. Nyní se v něm zobrazí aktuální řádek, znak a procento přečtení. [#51](https://github.com/trypsynth/paperback/issues/51).
* HTML komentáře ani obsah značek script a style se už v textovém výstupu nebudou zobrazovat.
* Pokud Paperbacku na příkazové řádce předáte relativní cestu, nyní ji správně vyhodnotí.
* Pohyb po procentech je nyní řešen vlastním dialogem s posuvníkem, dostupným pomocí Ctrl+Shift+G. [#57](https://github.com/trypsynth/paperback/issues/57).
* Dokumenty bez známého názvu nebo autora budou nyní vždy mít výchozí hodnotu.
* Logika ukládání pozice je nyní mnohem chytřejší a na disk by měla zapisovat jen tehdy, když je to opravdu nutné.
* Dokument, který jste měli aktivní při zavření Paperbacku, se nyní znovu otevře po restartu aplikace.
* Vstup v dialozích Přejít na řádek a Přejít na stránku by nyní měl být přísněji čištěn.
* Opravena navigace v obsahu v knihách Epub 3 s relativními cestami v manifestech.

### Verze 0.3.0

* Opraven obsah v knihách Epub s URL-kódovanými manifesty. [#34](https://github.com/trypsynth/paperback/issues/34).
* Opravena navigace po nadpisech v dokumentech HTML obsahujících vícebajtové znaky Unicode. [#42](https://github.com/trypsynth/paperback/issues/42), [#59](https://github.com/trypsynth/paperback/issues/59), [#61](https://github.com/trypsynth/paperback/issues/61).
* Opraveno vysoké využití CPU u dokumentů s dlouhými názvy kvůli regresi ve wxWidgets. [#60](https://github.com/trypsynth/paperback/issues/60).
* Opraveno načítání textových souborů UTF-8.
* Opravena chyba, kdy vnořené položky obsahu v knihách Epub umisťovaly kurzor na nesprávnou pozici.
* V některých případech opraven pád při ukončení aplikace. [#45](https://github.com/trypsynth/paperback/issues/45).
* Do dialogu Možnosti bylo přidáno zaškrtávací políčko pro zapnutí nebo vypnutí zalamování řádků!
* Nyní je možné přispět na vývoj Paperbacku, a to buď přes novou položku Přispět v nabídce Nápověda, nebo přes odkaz sponsor this project ve spodní části hlavní stránky repozitáře na GitHubu.
* Dokumenty Markdown nyní budou mít vždy název a Paperback by nyní měl zvládnout načíst prakticky jakýkoli soubor Markdown. [#52](https://github.com/trypsynth/paperback/issues/52).
* Dokumenty PDF nyní budou mít vždy název, i když metadata chybějí. [#56](https://github.com/trypsynth/paperback/issues/56).
* Pro PDF byla nasazena knihovna používaná v Chromiu, což vede k výrazně spolehlivějšímu parsování PDF napříč aplikací. [#41](https://github.com/trypsynth/paperback/issues/41).
* Nyní může být spuštěna vždy jen jedna instance Paperbacku. Spuštění paperback.exe s názvem souboru ve chvíli, kdy už Paperback běží, otevře tento dokument v již běžící instanci.
* Na libovolném dokumentu v seznamu záložek nyní můžete stisknout Delete a zavřít jej.

### Verze 0.2.1

* Do popisku stránky v dialogu Přejít na stránku byl přidán celkový počet stránek. [#46](https://github.com/trypsynth/paperback/issues/46).
* Umožněn přesun tabulátorem z obsahu dokumentu na seznam otevřených dokumentů. [#19](https://github.com/trypsynth/paperback/issues/19).
* Opravena chyba, kdy klávesy pro pohyb po nadpisech někdy otevíraly poslední dokumenty, pokud jich bylo dostatek. [#47](https://github.com/trypsynth/paperback/issues/47).
* Paperback nyní z textového výstupu odstraňuje zbytečné měkké pomlčky.
* Opravena chyba, kdy navigace po nadpisech někdy umístila kurzor na nesprávný znak.

### Verze 0.2.0

* Přidána podpora dokumentů Markdown! [#22](https://github.com/trypsynth/paperback/issues/22).
* Přidána podpora dokumentů PDF, včetně možnosti pohybovat se po stránkách! [#12](https://github.com/trypsynth/paperback/issues/12), [#37](https://github.com/trypsynth/paperback/issues/37).
* Přidány klávesové zkratky pro navigaci po nadpisech v HTML obsahu, včetně knih Epub a dokumentů Markdown. Tyto klávesové zkratky byly navrženy tak, aby fungovaly podobně jako u odečítačů obrazovky. [#3](https://github.com/trypsynth/paperback/issues/3).
* Opraveno načítání Epubů s URL-kódovanými názvy souborů v manifestech. [#20](https://github.com/trypsynth/paperback/issues/20).
* Opraveno načítání knih Epub 3 s vloženým obsahem XHTML. [#35](https://github.com/trypsynth/paperback/issues/35).
* Pokud dokument nepodporuje obsah nebo oddíly, je nyní místo zakázání příslušných položek nabídky přečtena odpovídající zpráva. [#39](https://github.com/trypsynth/paperback/issues/39).
* Přidána nabídka nedávných dokumentů! Momentálně ukládá posledních 10 otevřených dokumentů a stisknutí Enteru na některém z nich jej otevře pro čtení. [#32](https://github.com/trypsynth/paperback/issues/32).
* Dialog Hledat byl kompletně přepsán, takže je teď mnohem jednodušší na používání, a zároveň byla přidána historie posledních 25 hledání a podpora regulárních výrazů! [#21](https://github.com/trypsynth/paperback/issues/21).
* Dříve otevřené dokumenty nyní zůstanou zapamatovány i po restartu aplikace. Toto chování lze nastavit pomocí nové položky Možnosti v nabídce Nástroje. [#18](https://github.com/trypsynth/paperback/issues/18).
* Přidána klávesová zkratka Shift+F1 pro otevření readme přímo v Paperbacku.

### Verze 0.1.0

* První vydaná verze.
