# Paperback - verze 1.0

## Představení

Paperback je úsporná, rychlá a přístupná čtečka e-knih, dokumentů a audioknih určená komukoli, od příležitostných čtenářů až po náročné pokročilé uživatele. Je navržen pro maximální podporu odečítačů obrazovky, rychlou odezvu a uživatelský zážitek bez rušivých prvků.

## Systémové požadavky

Paperback funguje na Windows 10 a 11, na všech moderních verzích ARM macOS, na Linuxu, na iOS 17 a novějším a na Androidu 7 a novějším. Aplikace pro iOS a Android najdete v App Storu a na Google Play.

## Funkce

* Kompletně soběstačný program, který k tomu, abyste mohli prostě začít číst, nevyžaduje instalaci žádného dalšího softwaru.
* Neuvěřitelně rychlý, dokonce i na starém hardwaru.
* Jednoduché vícepanelové rozhraní, takže si můžete vedle sebe otevřít, kolik dokumentů chcete.
* Ukládá místo, kde jste přestali, v každém dokumentu, který otevřete.
* Volitelně si pamatuje, které dokumenty jste měli při zavření programu otevřené, a při příštím spuštění je znovu načte.
* Obsahuje funkce rychlé navigace, podobné režimu procházení webu ve většině odečítačů, pro rychlý a snadný pohyb v dokumentech.
* Obsahuje robustní dialog pro vyhledávání, včetně funkcí, jako je historie nebo podpora regulárních výrazů.
* Lze spouštět v přenosné verzi nebo nainstalovat, i s automatickým nastavením přidružených typů souborů.
* Podporuje obrovské množství běžných formátů souborů.
* Přehrává audioknihy s nastavitelnou rychlostí a se záložkami, které si pamatují přesný čas.
* Čte naskenované (obrázkové) stránky PDF pomocí OCR vestavěného ve Windows a macOS.
* Záložky a poznámky, díky kterým si můžete označit, kde jste skončili, a později se na dané místo vrátit.
* Každou klávesovou zkratku lze změnit.
* Obsahuje `pb`, nástroj příkazové řádky, který převede kterýkoli podporovaný dokument do HTML, Markdownu nebo prostého textu.

## Kompatibilita s odečítači obrazovky

Paperback dobře funguje se všemi hlavními odečítači obrazovky. Existují však dva známé problémy, které se týkají uživatelů JAWSu.

### JAWS a braillské řádky

Pokud používáte JAWS s braillským řádkem, může se stát, že se při posunu vpřed navigačními klávesami řádku oříznou dlouhé odstavce. Týká se to i příkazu pro přečtení aktuálního odstavce. Jde o chybu ve způsobu, jakým JAWS zachází s textovým prvkem RICHEDIT50W, nikoli o chybu v samotném Paperbacku, a vzhledem k nadšení, s jakým Vispero reaguje na hlášení problémů s open source softwarem, trvalo docela dlouho, než se objevilo řešení.

Řešení, které se po měsících čekání nakonec objevilo v diskusní skupině o JAWSu, spočívá v úpravě souboru `paperback.jcf`, kde nastavíte "Braille Presentation and Panning" na "Always use DOM if available". Kromě toho potřebujete zapnout i "Pan Text by Paragraph", jinak řádek zůstane na aktivním odstavci, místo aby se posouval. S oběma těmito nastaveními by posun měl fungovat správně.

### JAWS a hlášení Paperbacku

Paperback oznamuje hlášení jako "Žádné stránky." nebo "Tento dokument nemá žádnou zvukovou stopu." prostřednictvím notifikací přístupnosti, díky čemuž je odečítač obrazovky dokáže vyslovit i mezi jinými promluvami, které zrovna čte. JAWS na ně ale reaguje jen tehdy, když je pro danou aplikaci zapnutá volba "Povolit oznamování událostí přístupnosti", která na některých počítačích ve výchozím stavu zapnuta není.

Pokud JAWS po stisknutí klávesy, která by měla něco oznámit, nic neřekne, otevřete s Paperbackem v popředí Centrum nastavení (`Insert+6`), vyhledejte "oznamování" a zaškrtněte "Povolit oznamování událostí přístupnosti". Tím se nastavení zapíše do souboru `paperback.jcf`, takže platí pouze pro Paperback.

## Aktuálně podporované typy souborů

Paperback podporuje následující formáty a přípony:

* Archivy komiksů (`.cbz`)
* CHM soubory nápovědy (`.chm`)
* Knihy ve formátu DAISY (`.opf`, `.zip`)
* E-knihy ve formátu EPUB (`.epub`)
* E-knihy ve formátu FB2 (`.fb2`)
* HTML dokumenty (`.htm`, `.html`, `.xhtml`)
* Manuálové stránky, jak `man`, tak BSD `mdoc` (`.1` až `.9`, `.man`, `.roff` a také jejich varianty zkomprimované pomocí gzip)
* Dokumenty ve formátu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Dokumenty Microsoft Wordu (`.docx`, `.docm`, `.doc`)
* Audioknihy ve formátu M4B (`.m4b`)
* Knihy ve formátu MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Audioknihy ve formátu MP3 (`.mp3`)
* OpenDocument prezentace (`.odp`, `.fodp`)
* OpenDocument textové soubory (`.odt`, `.fodt`)
* PDF dokumenty (`.pdf`)
* PowerPointové prezentace (`.pptx`, `.pptm`, `.ppt`)
* Dokumenty ve formátu reStructuredText (`.rst`, `.rest`)
* RTF dokumenty (`.rtf`)
* Dokumenty Windows Write (`.wri`)
* Soubory nápovědy WinHelp (`.hlp`)
* Soubory v prostém textu a log soubory (`.txt`, `.log`)

## Klávesové zkratky

Paperback je navržen především pro použití s klávesnicí. Toto jsou aktuální klávesové zkratky:

Níže uvedené zkratky platí pro Windows. Tam, kde se macOS liší, je ekvivalent uveden v závorkách — hlavně proto, že Ctrl+G, Ctrl+W a Alt+šipka vlevo/vpravo jsou na této platformě už obsazené jinými systémovými nebo aplikačními konvencemi.

### Nabídka Soubor

* `Ctrl+O`: Otevření dokumentu.
* `Ctrl+F4` (macOS: `Cmd+W`): Zavření aktuálního dokumentu.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Zavření všech otevřených dokumentů.
* `Ctrl+Shift+T`: Znovuotevření naposledy zavřeného dokumentu.
* `Ctrl+R`: Zobrazení dialogu "Všechny dokumenty" (z nabídky Nedávné dokumenty).
* `Ctrl+Q`: Ukončení aplikace (pouze ve Windows; na macOS je tato položka místo toho v nabídce aplikace).

### Nabídka Přejít

* `Ctrl+F`: Zobrazení dialogu Najít.
* `F3` (macOS: `Cmd+G`): Najít další.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Najít předchozí.
* `Ctrl+G` (macOS: `Cmd+L`): Přejít na řádek.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Přejít na procenta.
* `Ctrl+P`: Přejít na stránku (pokud je podporováno v aktuálním dokumentu).
* `=`: Oznámení aktuální pozice v dokumentu v procentech a aktuální stránky, např. „15 %, stránka 30“. U dokumentů bez čísel stránek se stránka vynechá.
* `Alt+Left` (macOS: `Cmd+[`): Posun zpět v historii navigace.
* `Alt+Right` (macOS: `Cmd+]`): Posun vpřed v historii navigace.
* `[`: Předchozí oddíl.
* `]`: Následující oddíl.
* `Shift+H`: Předchozí nadpis.
* `H`: Následující nadpis.
* `Shift+1` až `Shift+6`: Předchozí nadpis úrovně 1 až 6.
* `1` až `6`: Následující nadpis úrovně 1 až 6.
* `Shift+P`: Předchozí stránka.
* `P`: Následující stránka.
* `Shift+B`: Předchozí záložka.
* `B`: Následující záložka.
* `/`: Nastavení dočasné záložky.
* `\`: Přechod na dočasnou záložku.
* `Shift+N`: Předchozí poznámka.
* `N`: Následující poznámka.
* `Ctrl+B`: Zobrazení všech záložek a poznámek.
* `Ctrl+Alt+B`: Zobrazení pouze záložek.
* `Ctrl+Alt+M`: Zobrazení pouze poznámek.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tedy fyzická klávesa Control místo Cmd): Zobrazení textu poznámky na aktuální pozici.
* `Shift+K`: Předchozí odkaz.
* `K`: Následující odkaz.
* `Shift+G`: Předchozí obrázek.
* `G`: Následující obrázek.
* `Shift+F`: Předchozí ilustrace.
* `F`: Následující ilustrace.
* `Shift+T`: Předchozí tabulka.
* `T`: Následující tabulka.
* `Shift+M`: Předchozí vzorec.
* `M`: Následující vzorec.
* `Shift+S`: Předchozí oddělovač.
* `S`: Následující oddělovač.
* `Shift+L`: Předchozí seznam.
* `L`: Následující seznam.
* `Shift+I`: Předchozí položka seznamu.
* `I`: Následující položka seznamu.
* `Shift+,`: Přechod na začátek aktuálního kontejneru (seznamu nebo tabulky).
* `,`: Přechod za konec aktuálního kontejneru (seznamu nebo tabulky).

### Nabídka Nástroje

* `Ctrl+W` (macOS: `RawCtrl+W`, tedy fyzická klávesa Control místo Cmd): Zobrazení počtu slov v aktuálním dokumentu.
* `Ctrl+I`: Zobrazení informací o dokumentu.
* `Ctrl+T`: Zobrazení obsahu (osnovy) dokumentu.
* `F7`: Zobrazení seznamu prvků.
* `Ctrl+Shift+C`: Otevření nadřazené složky (kde je aktuální dokument uložen).
* `Ctrl+Shift+V`: Otevření aktuálního dokumentu ve webovém zobrazení.
* `Ctrl+U`: Zobrazení zdroje dokumentu (původního souboru) na novém panelu.
* `Ctrl+Shift+E`: Export dat dokumentu (`.paperback`).
* `Ctrl+Shift+I`: Import dat dokumentu (`.paperback`).
* `Ctrl+E`: Export aktuálního dokumentu do prostého textu.
* `Ctrl+Shift+B`: Přepnutí záložky v aktuálním výběru / na aktuální pozici.
* `Ctrl+Shift+N`: Přidání nebo úprava poznámky k záložce v aktuálním výběru / na aktuální pozici.
* `Ctrl+Alt+W`: Přepnutí zalamování řádků.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, tedy fyzická klávesa Control, protože Cmd+Space otevírá Spotlight): Přehrání/pozastavení zvukové stopy.
* `'`: Přetočení zvukové stopy dopředu.
* `;`: Přetočení zvukové stopy dozadu.
* `Shift+'`: Prodloužení intervalu přetáčení.
* `Shift+;`: Zkrácení intervalu přetáčení.
* `Ctrl+Shift+.`: Zrychlení přehrávání zvukové stopy.
* `Ctrl+Shift+,`: Zpomalení přehrávání zvukové stopy.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tedy Control+Command+F): Přepnutí režimu na celou obrazovku.
* `Ctrl+,`: Otevření dialogu Nastavení (na macOS v nabídce aplikace).
* `Ctrl+Shift+S`: Přepnutí časovače spánku.
* `Ctrl+Shift+O`: Rozpoznání rozsahu naskenovaných (obrázkových) stránek PDF pomocí OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Označení začátku výběru, takže vše od tohoto místa až po místo, kde se zastavíte, můžete zkopírovat najednou.
* `Alt+F10` (macOS: `Cmd+F10`): Zkopírování všeho od označeného začátku výběru po aktuální pozici.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Návrat na označený začátek výběru, přičemž značka zůstane na svém místě.

### Nabídka Nápověda

* `Ctrl+F1`: Zobrazení dialogu O Paperbacku.
* `F1`: Zobrazení této nápovědy ve výchozím webovém prohlížeči.
* `Shift+F1`: Zobrazení této nápovědy přímo v Paperbacku.
* `Ctrl+Shift+U`: Vyhledání aktualizací.
* `Ctrl+D`: Otevření stránky Přispět ve výchozím webovém prohlížeči.

### Další zkratky v zobrazení dokumentu

* `Delete` / `Numpad Delete` na seznamu otevřených panelů: Zavření panelu vybraného dokumentu.
* `Ctrl+1` až `Ctrl+9` (macOS: `Cmd+1` až `Cmd+9`) v textu dokumentu nebo na seznamu panelů: Přechod na prvních devět otevřených dokumentů v pořadí, v jakém byly otevřeny.
* `Enter` nebo `Mezerník` v textu dokumentu: Aktivuje odkaz pod kurzorem nebo otevře zobrazení tabulky či vzorce na pozici kurzoru.
* `Enter` na naskenované (obrázkové) stránce PDF: Rozpoznání stránky pomocí OCR.
* `Shift+F10` nebo klávesa kontextového menu / aplikace v textu dokumentu: Otevření kontextové nabídky.

## iOS a Android

Aplikace pro iOS a Android používají stejné čtecí jádro jako desktopová verze, takže otevírají stejné formáty a stejným způsobem si pamatují, kde jste skončili. Jsou navrženy pro použití s VoiceOverem na iOS a s TalkBackem na Androidu.

### Otevírání dokumentů

* Použijte tlačítko Otevřít knihu, nebo otevřete dokument z aplikace Soubory či z jiné aplikace a zvolte Paperback.
* Na Androidu můžete místo toho v Nastavení zapnout vestavěný prohlížeč souborů. Vyžaduje oprávnění k přístupu ke všem souborům a velké soubory otevírá okamžitě, místo aby je nejdřív kopíroval.
* Dlouhým stisknutím tlačítka Otevřít knihu můžete importovat nebo exportovat data dokumentu (`.paperback`), tedy stejné soubory, jaké používá desktopová aplikace.

### Čtení a poslech

Každá aplikace nabízí dva způsoby, jak dokument číst. V režimu textu čtete text pomocí odečítače obrazovky. V režimu čtení nahlas vám Paperback text čte hlasem, který si zvolíte v Nastavení, a pokračuje i na pozadí a na zamknuté obrazovce. Mezi režimy přepínáte v nabídce Další možnosti.

Audioknihy, například knihy ve formátu DAISY, M4B nebo MP3, místo toho přehrávají svou vlastní nahrávku.

### Čtecí lišta

Lišta ve spodní části obrazovky obsahuje zleva doprava:

* Navigační jednotku, například odstavec, nadpis, stránku nebo odkaz. Změníte ji přejetím prstem nahoru nebo dolů.
* Tlačítka Předchozí, Přehrát a Následující. Tlačítka Předchozí a Následující posouvají o navigační jednotku.
* Rychlost řeči. Přejetím prstem nahoru nebo dolů změníte, jak rychle Paperback čte.

Přejetím prstem nahoru nebo dolů po tlačítku Přehrát se také můžete posouvat o navigační jednotku, aniž byste museli sahat po tlačítkách Předchozí a Následující. Pokud používáte jen tento způsob, nastavení Skrýt tlačítka předchozí a následující je odklidí z cesty vašemu odečítači obrazovky. Nastavení Pohybem prstu nahoru se posunete vpřed určuje, kterým směrem přejetí prstem posouvá.

### Další možnosti

V nabídce Další možnosti najdete všechno ostatní. Některé položky fungují v každé aplikaci trochu jinak.

* **Přepnout do režimu TTS nebo Přepnout do režimu textu:** přepíná mezi režimem čtení nahlas a režimem textu, které jsou popsány výše. V režimu textu položka Číst nahlas spouští a pozastavuje čtení nahlas, aniž byste režim textu opustili.
* **Zobrazení obsahu:** kapitoly knihy, otevře se na té, kterou právě čtete. Výběrem kapitoly na ni rovnou přejdete. Položky, pod kterými jsou další kapitoly, lze rozbalovat a sbalovat pomocí akcí odečítače obrazovky.
* **Seznam prvků:** seznam nadpisů nebo odkazů v dokumentu. Mezi nimi přepínáte výběrem typu na iOS nebo pomocí karet na Androidu; výběrem položky na ni přejdete.
* **Najít:** zadejte, co hledáte, nebo vyberte dřívější hledání z Historie vyhledávání, a zvolte, zda rozlišovat velikost písmen, hledat pouze celá slova nebo použít regulární výraz. Tlačítka Najít předchozí a Najít následující přeskočí na shodu a oznámí, kde se nachází, a hledání zůstane otevřené, takže můžete pokračovat. V režimu čtení nahlas se Najít objeví i jako navigační jednotka na čtecí liště, takže můžete shody procházet i odtud.
* **Přejít na:** přechod na řádek, stránku nebo procento dokumentu. Co z toho chcete, určíte výběrem režimu.
* **Nedávné dokumenty:** všechny dokumenty, které jste otevřeli, každý označený jako otevřený, zavřený nebo chybějící soubor. Každý z nich má dvě akce odečítače obrazovky: Odstranit ho vyřadí ze seznamu a Dohledat vám umožní najít dokument, jehož soubor byl přesunut. Vymazat nedávné dokumenty seznam vyprázdní, aniž by smazal jakékoli dokumenty.
* **Počet slov:** počet slov v dokumentu.
* **Informace o dokumentu:** název, autor, název souboru a na iOS také počet řádků a znaků.
* **Export:** uloží dokument jako prostý text, HTML nebo Markdown.
* **Časovač spánku:** zastaví čtení po 5, 10, 15, 30, 45 nebo 60 minutách, případně po vámi zvoleném čase. Když ho za běhu otevřete znovu, uvidíte, kolik času zbývá, a můžete ho také zrušit.
* **Nápověda:** otevře tuto dokumentaci.
* **Nastavení:**
    * **Nastavení hlasu:** hlas, rychlost řeči a výška hlasu, tlačítko Přehrát ukázku, abyste si aktuální nastavení mohli poslechnout, a pauza mezi odstavci. Android vám navíc umožňuje zvolit engine řečové syntézy. Na iOS zde najdete i slovník výslovnosti: pravidla, která mění, jak se slova vyslovují, a to buď pro všechny hlasy, nebo jen pro některé.
    * **Čitelnost:** velikost textu, řádkování, mezery mezi odstavci, zarovnání a vysoce kontrastní text. iOS má navíc světlý a tmavý vzhled.
    * **Chování:** zda se mají při spuštění aplikace znovu otevírat vaše poslední dokumenty, kterým směrem se posouváte švihnutím prstu na tlačítku Přehrát a zda mají být skryta tlačítka Předchozí a Následující. Android zde má navíc vestavěný prohlížeč souborů.

### Klávesnice a náhlavní soupravy

S klávesnicí fungují desktopové klávesové zkratky pro otevírání knih, nedávné dokumenty, Najít, Přejít na, obsah, počet slov, informace o dokumentu, export a časovač spánku, přičemž na iOS se místo `Ctrl` používá `Cmd`. Totéž platí pro jednopísmenné klávesy pro pohyb po nadpisech, stránkách, odkazech a dalších prvcích a `Mezerník` přehrává a pozastavuje. Na iOS se jednopísmenné klávesy dostanou do Paperbacku jen tehdy, když je ve VoiceOveru vypnutá jednoznaková rychlá navigace.

Na Androidu tlačítko náhlavní soupravy jedním stisknutím přehrává a pozastavuje, dvěma stisknutími posouvá vpřed a třemi zpět.

## Podporované jazyky

Paperback je přeložen do mnoha různých jazyků a další neustále přibývají. Následuje kompletní výčet:

Pokud se chcete zapojit, přečtěte si prosím naši [příručku pro překladatele](translating.md).

* Bosenština
* Čeština
* Nizozemština
* Finština
* Francouzština
* Němčina
* Japonština
* Polština
* Brazilská portugalština
* Ruština
* Zjednodušená čínština
* Srbština
* Španělština
* Ukrajinština
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
* Michael Marshall
* Pratik Patel
* Roberto Perez
* Sean Randall
* Timothy Wynn
* Tyler Rodick

## Historie změn

### Verze 1.0

1.0 je první verze vydaná na všech pěti platformách: Windows, macOS, Linux, iOS a Android, přičemž aplikace pro iOS a Android jsou k dispozici v App Storu a na Google Play.

#### Přidáno

##### Obecné
* Podpora Linuxu, a to jako AppImage nebo tar.gz, včetně integrace do desktopu, takže se dokumenty otevírají přímo ze správce souborů.
* Začátek výběru můžete označit pomocí `Alt+F9`, vše od něj až po místo, kde jste se zastavili, zkopírovat pomocí `Alt+F10` a na označené místo se vrátit pomocí `Alt+Shift+F9`, takže dlouhý úsek textu zkopírujete, aniž byste ho museli procházet šipkami se shiftem. Všechny tři příkazy najdete v nabídce Nástroje > Vybrat a zkopírovat.
* Zkratka `=` nyní kromě procent oznamuje i stránku, např. „15 %, stránka 30“; u dokumentů bez čísel stránek zůstává beze změny.
* Dialog O Paperbacku nyní zobrazuje licenci Paperbacku a všechny překladatele.
* Ukrajinský překlad.

##### Nové formáty
* Archivy komiksů (`.cbz`).
* Audioknihy ve formátu M4B, rozdělené na kapitoly.
* Manuálové stránky, jak `man`, tak BSD `mdoc`, zkomprimované pomocí gzip i nezkomprimované.
* Audioknihy ve formátu MP3, rozdělené na kapitoly, pokud je soubor obsahuje.
* Dokumenty ve formátu reStructuredText.
* Soubory Windows Write (`.wri`).
* Soubory WinHelp (`.hlp`).
* Dokumenty Wordu 6 a Wordu 95.

##### OCR
* Naskenované stránky PDF lze nyní rozpoznat pomocí OCR vestavěného ve Windows a macOS. Stisknutím `enteru` na naskenované stránce ji rozpoznáte, případně můžete pro rozsah stránek použít Dávkové OCR (`Ctrl+Shift+O`).

##### Navigace
* Vzorce MathML v EPUB a HTML se pomocí MathCAT vykreslují jako AsciiMath. Mezi vzorci se pohybujete pomocí `M` a `Shift+M` a stisknutím `enteru` nebo `mezerníku` otevřete původní MathML v zobrazení vzorce.
* Tlačítko Najít vše v dialogu Najít, které vypíše každý řádek se shodou, takže můžete skočit rovnou na ten, který potřebujete.
* Zobrazení tabulek, seznamů a stránek v seznamu prvků (`F7`).
* Dialogy Přejít na řádek, Přejít na stránku a Přejít na procenta nyní akceptují i `+n` a `-n` pro posun relativně k aktuální pozici.
* Knihy EPUB, MOBI a CHM, které nemají vlastní nadpisy, nyní získají nadpisovou osnovu ze svého obsahu.
* Knihy KF8 (AZW3) nyní podporují navigaci po oddílech.
* Stránky EPUB, které obsahují jen obrázek, nyní zobrazují řádek, který tento obrázek zastupuje, takže na něj můžete přejít, místo abyste ho rovnou přeskočili.

##### Audioknihy
* Ovládání rychlosti přehrávání, od poloviční až po trojnásobnou. Použijte `Ctrl+Shift+.` a `Ctrl+Shift+,` nebo nabídku Nástroje.
* Záložky a poznámky v čistě zvukových knihách si nyní pamatují přesný čas, ve kterém jste je vytvořili.
* Přechod na předchozí a následující pozici (`Alt+Left` a `Alt+Right`) nyní funguje i v audioknihách.
* Postup v audioknize se nyní měří podle skutečné délky nahrávky, takže Přejít na procenta i stavový řádek odpovídají tomu, kde v ní skutečně jste.

##### Nedávné dokumenty
* Položka Vymazat nedávné dokumenty v podnabídce Nedávné dokumenty.

##### Dokumenty PDF
* Nastavení, které v PDF ponechá každý řádek samostatně, místo aby je spojovalo do odstavců.
* Obrázky a ilustrace v PDF se nyní oznamují.
* PDF, která obsahují strukturu pro čtení, ale netagují v ní obrázky, nyní tyto obrázky oznamují, místo aby je z knihy úplně vynechala.

##### Webové zobrazení
* Ve webovém zobrazení lze nyní otevřít jakýkoli dokument, nejen EPUB, HTML a Markdown.

##### Čitelnost
* Nadpisy se nyní vykreslují ve velikosti odpovídající jejich úrovni a obrázky a tabulky jsou oddělené od okolního textu.

##### pb
* `pb --list-formats` vypíše všechny formáty, které pb umí načíst.
* pb nyní uvádí, který soubor nedokázal načíst, a proč.

#### Opraveno

##### Obecné
* Kniha znovu otevřená při spuštění se nyní začne číst hned, místo aby zůstala potichu, dokud ji nezavřete a otevřete ručně.
* Dokument, jehož soubor zmizel, lze nyní odebrat z dialogu Všechny dokumenty, místo aby v seznamu zůstával bez ohledu na to, kolikrát jste jeho odebrání potvrdili.
* Opraven pád při zavírání Paperbacku.
* Při zavírání Paperbacku nyní okno okamžitě zmizí, místo aby zůstávalo na obrazovce, dokud se vše neuloží.
* Velké knihy s malým množstvím formátování se nyní otevírají zhruba za polovinu času.
* Hlášení vyvolaná z nabídky, jako třeba „Tento dokument nemá žádnou zvukovou stopu.“, už odečítač obrazovky nepřeruší dřív, než je uslyšíte.
* Otevření dokumentu už nenechá položku Znovu otevřít naposledy zavřený povolenou, když není co znovu otevřít.
* Paperback se už opakovaně nepokouší načíst dokumenty ze seznamu nedávných, které zmizely, a omezuje počet ukládaných nedávných dokumentů.
* Starý INI konfigurační soubor se nyní po převedení do nového formátu smaže.
* Názvy dialogů pro výběr fontu a barvy a nabídka Exportovat jako ve vietnamštině jsou nyní přeložené.
* Po aktualizaci se znovu spuštěné okno nyní přenese do popředí, místo aby zůstalo za všemi ostatními okny v přepínači Alt+Tab.
* Zalamování řádků se u velkých dokumentů nyní použije okamžitě, místo aby se znovu načítal celý dokument.

##### Navigace
* `Alt+šipka doleva` vás nyní vrátí tam, odkud jste přeskočili, místo na nějakou starší pozici.
* Zvuky záložek se nyní přehrají jen tehdy, když přejdete přes záložku, a ne když přistanete na řádku, na kterém se nachází.
* Zavření obsahu, seznamu prvků a dialogů Přejít na vás nyní přesune rovnou na řádek, na který jste se dostali, místo abyste museli pokaždé poslouchat, jak odečítač znovu čte celé okno.
* Dialogy Přejít na řádek, Přejít na stránku a Přejít na procenta nyní odmítají čísla mimo rozsah dokumentu, místo aby vás potichu přesunuly jinam.
* NVDA už nepřeruší oznámení, když dokument nemá žádné stránky.
* Stisknutí OK v obsahu bez pohybu po položkách nyní přejde na položku, která byla vybraná původně.
* Obsah, seznam prvků a seznam záložek se už u knih s tisíci položkami nezasekávají ani nezamrzají.
* Šipky nahoru a dolů si nyní pamatují sloupec pro každý dokument zvlášť, místo aby ho přenášely při přepnutí záložky.

##### Audioknihy
* Přehrávání zvuku na macOS nyní používá `Control+Space`, protože `Command+Space` patří Spotlightu.

##### Dokumenty PDF
* Opravena chyba, kdy se PDF exportovaná z Apple Pages četla jako prostý text bez nadpisů a seznamů.
* Opravena chyba, kdy se odstavce a nadpisy v PDF dělily na každém řádku a slova se rozpadala na mezerách.
* Opravena chyba, kdy číslované nadpisy v PDF splývaly do jednoho nadpisu.
* Opravena chyba, kdy se PDF, jejichž strom struktury nevede k žádnému textu, otevírala prázdná.
* Řádky vysázené neproporcionálním písmem, například kód, se už nespojují do odstavců.
* Záhlaví a zápatí stránek se už u netagovaných PDF nečtou na každé stránce.
* PDF, která svá záhlaví a zápatí stránek označují jako běžný text, už na každé stránce neopakují mezi dvěma odstavci název a číslo stránky.
* PDF nyní zobrazují svůj skutečný název místo názvu souboru.

##### Knihy MOBI/AZW3
* Velkým knihám MOBI už nedochází paměť a už se neořezávají po 20 MB.
* Knihy MOBI a AZW3 se nyní otevírají mnohem rychleji.
* Opravena chyba, kdy knihy MOBI přicházely o seznam kapitol.
* Opraven nesmyslný text v místech, kde knihy MOBI přecházejí z jednoho záznamu do dalšího.

##### Webové zobrazení
* Webové zobrazení už nenačítá obrovské knihy celé najednou.
* Webové zobrazení nyní zobrazuje dokumenty celé, když je celé zobrazuje i čtečka, místo aby ukazovalo jen jejich část.

##### Ostatní formáty
* Knihy FictionBook (.fb2) v kódování windows-1251, což je většina z nich, se nyní otevřou, místo aby se je vůbec nepodařilo načíst.
* Knihy FictionBook, které používají jmenný prostor nebo HTML entitu, kterou nikdy nedeklarovaly, se nyní otevřou, místo aby byly odmítnuty jako poškozené.
* Knihy ve starých kódováních se nyní otevírají mnohem rychleji.
* Opravena chyba, kdy se některé čínské textové soubory otevíraly jako změť nesmyslných znaků.
* Soubory OpenDocument chráněné heslem se nyní na heslo zeptají, místo aby byly hlášeny jako poškozené.
* Soubory starého PowerPointu chráněné heslem se nyní otevřou a snímky starého PowerPointu už nepřicházejí o svůj text.
* Soubory v prostém textu uložené s příponou `.rtf` se nyní otevřou jako text, místo aby selhaly s chybou.
* Řídicí slova RTF se už nezobrazují jako text.

#### iOS a Android

Aplikace pro iOS a Android otevírají všechny formáty, které podporuje desktopová verze, a nabízejí:

* Čtení nahlas vámi zvoleným hlasem, rychlostí a výškou hlasu, s ovládáním rychlosti řeči přímo na čtecí liště a volitelnou pauzou mezi odstavci.
* Přehrávání audioknih ve formátech DAISY, M4B a MP3, které pokračuje i na pozadí a na zamknuté obrazovce.
* Navigaci po nadpisech, stránkách, odkazech, tabulkách, seznamech a dalších prvcích ze čtecí lišty a k tomu obsah a hledání.
* Časovač spánku, počet slov a export dokumentu a na iOS navíc slovník výslovnosti. Na iOS export probíhá přes nabídku sdílení, takže knihu můžete poslat do jiné aplikace nebo do Souborů, a to v jiném formátu nebo přesně tak, jak je.
* Volby velikosti textu, mezer a vysoce kontrastního textu.
* Klávesové zkratky odpovídající desktopové verzi.

### Verze 0.9.2
* Audioknihy už při přechodu na textové pole nenutí odečítač obrazovky číst řadu mezer.
* Audioknihy nyní při pohybu po oddílech oznamují název souboru.
* Audioknihy nyní uvádějí skutečnou délku, místo aby u každého souboru tvrdily, že trvá 24 hodin.
* Zavření webového zobrazení klávesou Escape už nevyvolá ladicí hlášku poté, co jste v něm otevřeli nějaký odkaz.
* Kopírování po výběru veškerého textu nyní zkopíruje celý dokument, ne jen jeho právě načtenou část.
* Hledání vás nyní přesune rovnou na nalezený řádek, místo abyste při návratu fokusu do knihy museli pokaždé poslouchat, jak odečítač znovu čte celé okno.
* Opravena chyba, kdy se EPUB soubory obsahující zbloudilý blok ZIP64 odmítaly otevřít s hláškou „Invalid local file header“.
* Opravena chyba, kdy se dlouhé dokumenty při plynulém čtení odečítačem vracely na začátek.
* Odkazy ve webovém zobrazení vás nyní přesunou do oddílu, na který míří, místo aby selhaly s hláškou „File not found“.
* Automatické oznámení „Dokument obnoven“ už nepřeruší odečítač uprostřed věty, ale počká, až dokončí stávající promluvu.
* Na kartě Obecné v dialogu Nastavení se nyní tabulátorem prochází mezi volbami v pořadí, v jakém jsou na obrazovce, přičemž kanál aktualizací následuje hned za volbou pro vyhledávání aktualizací.
* Windows nyní v nabídce Otevřít v programu vždy zobrazí „Paperback“ místo celého sloganu programu.
* Počet slov a Informace o dokumentu nyní ukazují, kolik souborů audiokniha obsahuje a jak dlouho celkem trvá.

### Verze 0.9.1
* Zvuky záložek a poznámek se nyní přehrávají i na macOS.
* Knihy ve formátu DAISY nyní na macOS přehrávají zvuk, místo aby se jen otevřely a tiše pohybovaly po časové ose.
* Opravena chyba, kdy z RTF dokumentů mizely typografické uvozovky, dlouhé pomlčky a podobné znaky a slova kolem nich se slévala dohromady.
* Opravena chyba, kdy obrázky v RTF dokumentech propouštěly svá surová data do textu dokumentu v podobě nesmyslných znaků.
* Opravena chyba, kdy si podnabídka Nedávné dokumenty držela zastaralé položky, dokud ji něco jiného nesestavilo znovu.
* Klávesové akcelerátory se vrátily do všech překladů, takže nabídky v ruštině jsou opět přístupné z klávesnice.
* Velké dokumenty CHM se nyní otevírají až sedmkrát rychleji.
* Otevřené dokumenty se nyní registrují ve Windows, takže se objevují v seznamu odkazů na hlavním panelu a v seznamu nedávných položek v nabídce Start.
* Dialog Možnosti byl přejmenován na Nastavení, aby odpovídal mobilním aplikacím a na macOS i konvenci platformy.
* Paperback si nyní mezi spuštěními pamatuje pozici, velikost a maximalizaci svého okna.
* Nyní se překládají i množná čísla, takže zprávy, které něco počítají, zní správně i v jazycích, které potřebují víc než jeden tvar.
* Výběr souboru ncc.html u knihy ve formátu DAISY nyní otevře kompletní audioknihu místo pouhého textu.
* Názvy akcí v dialogu Nastavení klávesových zkratek je nyní možné přeložit.
* V záhlaví okna je nyní název dokumentu na prvním místě, takže otevřené knihy lze rozlišit na hlavním panelu a při přepínání zkratkou Alt+Tab.
* Dialog aktualizací nyní podporuje lokalizaci.

### Verze 0.9.0

#### Přidáno

##### Obecné
* Nástroj příkazové řádky s názvem pb, který rychle převede kterýkoli z formátů podporovaných Paperbackem do HTML, Markdownu nebo prostého textu.
* Možnost znovu načíst dokumenty, které na disku změnily jiné programy.
* Možnost Zobrazit zdroj, která otevře zdroj (původní soubor) dokumentu na nové záložce; hodí se například při úpravách Markdownu.
* Text dokumentu se nyní skutečně dělí na stránky, takže knihy s desítkami milionů slov se načtou během pár sekund. Prosím hlaste jakékoli podivnosti, na které v souvislosti s tím narazíte.

##### Podpora platforem
* Podpora ARM64 Windows!
* Nativní podpora macOS!
* Přepínač režimu na celou obrazovku.

##### Dialog Všechny dokumenty
* Tlačítko Dohledat pro dohledání chybějících souborů knih, kterým se jen změnila cesta.
* Filtr stavu a stavový řádek, takže můžete filtrovat podle stavu dokumentu a vidět, kolik dokumentů je zobrazeno a vybráno.
* Klávesová zkratka `Ctrl+Shift+A` pro zrušení výběru všech dokumentů.

##### Nastavení a čitelnost
* Karta Čitelnost s následujícími volbami:
    * Zalamování řádků (přesunuto z karty Obecné);
    * Vykreslování tabulek v toku textu (novinka v této verzi, viz níže);
    * Font;
    * Barva pozadí;
    * Velikost řádkování;
    * Mezery mezi odstavci;
    * Mezery mezi písmeny;
    * Zarovnání textu.
* Položka nabídky pro zalamování řádků a k ní příslušná klávesová zkratka.
* Přepínač, kterým určíte, jak se mají tabulky zobrazovat, a sjednocené zobrazování tabulek napříč dokumenty.

##### Navigace
* Podpora navigace po kontejnerech.
* Možnost automaticky přesouvat kurzor na začátek řádku při pohybu mezi řádky, podobně jako v režimu prohlížení u odečítačů obrazovky.
* Klávesová zkratka rovná se pro oznámení aktuální pozice v dokumentu v procentech.

##### Záložky
* Dočasné záložky: můžete mít jednu na každý dokument a zůstávají uložené. Lomítkem ji nastavíte, zpětným lomítkem na ni přejdete.

##### Počet slov
* Odhadovaná doba čtení v dialogu Počet slov a k tomu možnost nastavit si rychlost čtení, aby byl tento údaj opravdu k něčemu.
* Pokud máte při otevření dialogu Počet slov aktivní výběr, zobrazí se nyní i to, kolik slov jste vybrali.

##### Klávesové zkratky
* Možnost přizpůsobit si v jednoduchém dialogu každou klávesovou zkratku v aplikaci.
* Nastavitelná klávesová zkratka pro obnovení Paperbacku ze systémové lišty.

##### Jazyky
* Nizozemština, finština a polština.

##### Export
* Rozšířena položka nabídky pro export, takže kromě prostého textu umožňuje exportovat i do HTML a Markdownu.

##### Updater
* Tlačítko Zrušit v dialogu probíhající aktualizace.
* Updater nyní ověřuje, že stažený soubor nebyl pozměněn.

##### Webové zobrazení
* Webové zobrazení se nyní otevírá na aktuální pozici čtecího kurzoru.

##### Knihy DAISY
* Podpora knih ve formátu DAISY 2.0.
* Podpora přehrávání zvuku ve formátu DAISY 2.02.

##### Audioknihy
* Možnost přehrávat audioknihy, aktuálně jak ve formátu DAISY audio (včetně DAISY audio + text), tak zazipovaných zvukových souborů.
* Klávesové zkratky a položky nabídky pro přehrání/pozastavení zvukové stopy, přetáčení dopředu a dozadu a nastavení intervalu přetáčení.
* Volby pro synchronizaci čtecího kurzoru s přehrávaným zvukem, nastavení intervalu přetáčení a pro určení, zda přetočení za konec kapitoly pokračuje do další.

##### Dokumenty CHM
* Podpora seznamů, položek seznamů, ilustrací a obrázků.

##### PowerPoint
* Dokumenty PowerPointu nyní podporují tabulky.

#### Opraveno

##### Obecné
* Dokumenty ve starých CJK kódováních, jako je GBK, Big5 nebo Shift_JIS, se nyní vykreslí správně místo změti nesmyslných znaků.
* Funkce „Znovu otevřít naposledy zavřený“ se pokoušela znovu otevřít přiložené readme.
* Vybraný panel po restartu Paperbacku nezískával fokus, jak měl.
* Zacházení Paperbacku se soubory na síťových discích Windows: zobrazení souboru ve složce nyní správně vybere soubor na síťovém úložišti a cesty už neobsahují podivné znaky.
* Soubory .paperback se už při obnovování dokumentů nenačítají natvrdo; místo toho budete při nalezení takového souboru požádáni o potvrzení.
* Otevření nadřazené složky nyní daný soubor v Průzkumníku vybere.
* Otevření readme nyní respektuje vámi zvolený jazyk.
* Uživatelské rozhraní Paperbacku se nyní správně škáluje na displejích s vysokým DPI.
* Při otevření nápovědy v Paperbacku se nyní správně aktualizuje nabídka a fokus se přesune na textové pole.
* Ve Windows jsem přešel na mnohem bezpečnější způsob meziprocesové komunikace.
* Při přepínání mezi záložkami se nyní přečte název aktivního dokumentu.
* Sníženo využití paměti u velkých dokumentů zmenšením interních indexových tabulek pro jednotlivé znaky na polovinu.

##### Dialog Všechny dokumenty
* Klávesa Escape nezavírala dialogy Informace o dokumentu a Všechny dokumenty.
* Záhlaví okna se neaktualizovalo po zavření dokumentu z dialogu Všechny dokumenty.
* Readme.html se už při otevření pomocí Shift+F1 nepřidá do seznamu všech dokumentů.
* Odebrání dokumentů z dialogu nedávných dokumentů nyní zavře i jejich otevřenou záložku.
* Váš vyhledávací filtr nyní zůstane zachován i po odebrání dokumentu.

##### Navigace
* Pohyb po stránkách v některých situacích oznamoval nesprávný text řádku.
* Dialogy Přejít na řádek, Přejít na stránku a Přejít na procenta umisťovaly ve velkých dokumentech kurzor na nesprávnou pozici.
* Dialogy Najít a Najít další nerespektovaly ve velkých dokumentech načtený úsek dokumentu.

##### Záložky
* Zvuky záložek a poznámek by se nyní měly přehrát výhradně tehdy, když přejdete na slovo, které je obsahuje.

##### Čitelnost
* Zapnutí zalamování řádků skákalo na začátek dokumentu.

##### Webové zobrazení
* Dialog webového zobrazení nešel zvětšit a otevíral se ve velmi malé počáteční velikosti.
* Obrázky by se nyní měly ve vestavěném webovém zobrazení zobrazovat správně.

##### Updater
* Updater nyní v poznámkách k vydání správně zobrazuje obsah tagů Markdownu.

##### Knihy DAISY
* Knihy ve formátu DAISY zobrazovaly ve stavovém řádku nesprávné informace.
* Načítání knih ve formátu DAISY s nesmyslnými deklaracemi kódování.

##### Dokumenty RTF
* Parsování RTF dokumentů obsahujících nelatinkové znaky.
* Skupiny `\pict` v RTF, takže se data vložených obrázků už nepropisují do textu dokumentu.

##### Knihy Mobi/AZW3
* Kotvy filepos v knihách Mobi rozdělovaly HTML značky a vkládaly do textu knihy nesmysly.
* Odkazy ve starých knihách Mobi.
* Výrazně vylepšeno parsování AZW3.

##### Dokumenty Wordu
* Dokumenty Wordu s názvy stylů závislými na jazyce nevykreslovaly správně své nadpisy.

##### Dokumenty HTML/XHTML
* Prvky dl, dt a dd nevytvářely v dokumentech XHTML zalomení řádku.

##### Dokumenty PDF
* Paperback nyní u nesprávně tagovaných PDF přejde na extrakci prostého textu.
* PDF dokumenty obsahující řídicí znaky v názvu anebo v záložkách už při otevření Paperback neshodí.

### Verze 0.8.5
* Přidána podpora stránek pro knihy ve formátu epub.
* Přidána podpora šifrovaných dokumentů Microsoft Office. Aktuálně jsou podporovány formáty starého Wordu, moderního Wordu a moderního PowerPointu. Starý PowerPoint je plánován do budoucna.
* Přidána podpora starého formátu dokumentů Microsoft Wordu (*.doc)!
* Přidána podpora PowerPointových prezentací ve starém formátu (*.ppt)!
* Přidána podpora knih ve formátu mobi a AZW3!
* Přidána podpora tagovaných PDF souborů!
* Přidána klávesová zkratka ctrl+q pro ukončení aplikace.
* Přidána podpora zazipovaných knih z Bookshare (DAISY i Word)!
* U obrázků v dokumentech by se nyní měl správně zobrazovat alt text.
* Soubory CHM nyní správně podporují navigaci po vnitřních odkazech.
* Opravena chyba, kdy přechod na konkrétní číslo stránky byl posunutý o 1.
* Opravena chyba, kdy se dialog Otevřít jako nedal zavřít klávesou escape.
* Opravena chyba, kdy se kontextové menu ve čtečce nezobrazovalo při kliknutí pravým tlačítkem myši nebo stisknutí klávesy Aplikace.
* Opravena chyba, kdy se při otevírání dokumentů z příkazové řádky někdy fokusoval nesprávný dokument.
* PDF, která jsou pouze obrázková, jsou nyní znovu správně detekována a upozorní vás na tuto skutečnost.
* Nyní je možné navigovat po obrázcích a ilustracích pomocí zkratek G/shift+G a F/shift+F.
* Paperback bude nyní respektovat vaše nastavení tmavého režimu pro aplikace.
* Odstraněna podpora DAISY XML, protože už není potřeba.
* Ve stromovém zobrazení obsahu dokumentu jsem přešel zpátky na nativní Win32 navigaci prvními písmeny.
* Dialog o chybě při spuštění nyní zobrazuje podrobnější chybové hlášky.
* Webové zobrazení se nyní bude načítat mnohem rychleji a hladčeji.

### Verze 0.8.2
* Přidána podpora stránek pro RTF dokumenty!
* Opravena chyba, kdy se při otevření EPUB dokumentu ve webovém zobrazení automaticky aktivovaly odkazy, pokud je dokument obsahoval.
* Opravena chyba, kdy RTF parser ve vzácných případech nevkládal mezery mezi slova.
* Opraveno rozdělování odstavců v některých PDF dokumentech na několik krátkých řádků.
* PDF dokumenty teď mají základní podporu pro pohyb po odkazech a po nadpisech!
* RTF tabulátory a konce řádků se teď vykreslují přesně tak, jak se v dokumentu vyskytují.
* Přešel jsem zpátky na vyzkoušenou a osvědčenou knihovnu pdfium pro parsování PDF souborů, takže je jejich renderování opět výrazně spolehlivější.

### Verze 0.8.1
* Přidána klávesová zkratka Ctrl+Shift+T pro znovuotevření naposledy zavřeného dokumentu.
* Dialog Všechny dokumenty teď podporuje výběr více dokumentů, které se mají otevřít najednou.
* Opraveno několik chyb v RTF parseru.
* Opravena chyba, kdy cesty k souborům obsahující jiné než ascii znaky (například bosenské š, č, ć, ž) přestaly fungovat při otevření stejného souboru ve druhé instanci Paperbacku.
* Opraveno čtení textu z PDF v nesprávném pořadí a chybové vkládání mezer okolo slov s velkými písmeny.
* Opraveno pomalé načítání dokumentů při otevírání velkých souborů.
* Opravena lokalizace tlačítek Ano a Ne v potvrzovacích dialozích.

### Verze 0.8.0
* Přidány lokalizace pro japonštinu, vietnamštinu a zjednodušenou čínštinu!
* Přidán automatický updater, který nyní nahradí aktuálně nainstalovanou verzi Paperbacku, místo aby jenom stáhnul novou verzi!
* Přidána volitelná zvuková odezva pro přesun na záložku nebo poznámku (za zvuky děkuji Andre Louisovi)!
* Přidána podpora RTF dokumentů!
* Přidána podpora dokumentů ve formátu DAISY XML.
* Přidána podpora textových souborů Flat Open Document Text!
* Přidána podpora Flat Open Document prezentací!
* Přidána podpora oddělovačů a zkratky S a shift+S.
* Každý posun o víc než 300 znaků se nyní automaticky přidá do historie navigace.
* Opraveno obnovování okna Paperbacku ze systémové lišty.
* Opravena chyba, kdy se ve webovém zobrazení pro dokumenty v Markdownu zobrazoval původní text místo renderovaného HTML.
* Opraveno nesprávné renderování tabulek v Markdown souborech.
* PDF, která jsou pouze obrázková, vás na tuto skutečnost nyní upozorní, když se takový soubor pokusíte načíst.
* Ve spustitelném souboru Paperbacku jsou správně uvedeny informace o verzi.
* Dialog Možnosti byl rozdělen do záložek pro snadnější používání a pohodlnější navigaci.
* Přešel jsem na knihovnu Hayro pro parsování souborů PDF, což vede k větší spolehlivosti, rychlejší odezvě a menšímu počtu DLL knihoven.
* Přepsal jsem celou aplikaci do Rustu. Nový kód je bezpečnější, načítá dokumenty rychleji a je ho jednodušší udržovat a rozšiřovat.
* Kontextová nabídka na textovém prvku s obsahem dokumentu teď obsahuje příkazy specifické pro čtečku a ne obecné příkazy jako Vyjmout nebo Vložit.

### Verze 0.7.0
* Přidána podpora tabulek pro dokumenty založené na HTML a XHTML! Mezi tabulkami se můžete pohybovat pomocí T a Shift+T a stisknutím Enteru si některou zobrazit ve webovém zobrazení.
* Přidány základy funkce webového vykreslování! Stisknutím Ctrl+Shift+V otevřete aktuální část dokumentu ve webovém zobrazení. Může se to hodit zejména u obsahu, jako je složité formátování nebo ukázky kódu.
* Přidán ruský překlad (díky Ruslanu Gulmagomedovovi)!
* Do dialogu Všechny dokumenty přidáno tlačítko Vymazat vše.
* Kontrola aktualizací nyní při dostupnosti nové verze zobrazuje poznámky k vydání.
* Opraveno obnovování okna ze systémové lišty.
* Opraveny překlady tlačítek Ano/Ne v potvrzovacích dialozích.
* Opraveno načítání konfigurace při spuštění jako správce.
* Opraveno zpracování komentářů v dokumentech XML a HTML.
* Opraveno parsování obsahu v knihách Epub 2.
* Opraven přechod na další položku se stejným písmenem v obsahu.
* Opravena chyba, kdy se dialog Hledat při použití tlačítek další/předchozí neskrýval správně.
* Opravena chyba, kdy vás obsah v epub dokumentech občas přesunul na nesprávnou položku.
* Opraveny různé problémy se zpracováním bílých znaků v XML, HTML a značkách pre.
* Opraven posun o jeden odkaz mimo při navigaci po odkazech.
* Opravena chyba, kdy některé knihy měly na konci řádků nadbytečné bílé znaky.
* Opraveny různé problémy parseru.
* Položky nabídky související se záložkami i seznam prvků jsou nyní správně zakázány, když není otevřen žádný dokument.
* Vylepšeno zpracování seznamů v různých formátech dokumentů.
* Vylepšen pracovní postup pro překladatele.
* Mnoho interních refaktorizací, při nichž byla většina aplikační logiky přesunuta z C++ do Rustu kvůli vyššímu výkonu a lepší udržitelnosti.

### Verze 0.6.1
* Přidána podpora PDF chráněných heslem!
* Přidána velmi základní funkce pro přesun na předchozí/další pozici. Pokud stisknete Enter na vnitřním odkazu a kurzor se přesune, tato pozice se nyní zapamatuje a lze se na ni vracet pomocí Alt+šipka vlevo/vpravo.
* Přidán seznam prvků! Momentálně zobrazuje jen strom všech nadpisů v dokumentu nebo seznam odkazů, ale do budoucna počítám s jeho rozšířením.
* Přidána možnost spouštět Paperback ve výchozím nastavení maximalizovaný.
* Opravena chyba, kdy odkazy v některých dokumentech Epub nefungovaly správně.
* Opraveno parsování obsahu v Epub souborech obsahujících relativní cesty.
* Opravena chyba, kdy se u některých epub dokumentů nezobrazoval název nebo autor.
* Opravena chyba, kdy se názvy některých kapitol v epub dokumentech v dialogu obsahu nezobrazovaly správně.
* Opravena chyba, kdy v dialogu obsahu nebylo možné aktivovat tlačítka OK/Zrušit mezerníkem.
* Vylepšeno zpracování nadpisů v dokumentech Microsoft Wordu.
* Pokud se pokusíte vyvolat dialog Nedávné dokumenty a seznam bude prázdný, dostanete nyní hlasovou odezvu.

### Verze 0.6.0
* Do dialogu Možnosti přidána nová volba, která zobrazuje nabídku Přejít v mnohem kompaktnější podobě; ve výchozím stavu je zapnutá.
* Přidána možnost cyklické rychlé navigace po strukturních prvcích.
* Do nabídky Nástroje byla přidána možnost otevřít složku obsahující právě fokusovaný dokument.
* Přidán poměrně jednoduchý, ale velmi účinný systém aktualizací.
* Přidána základní funkce časovače spánku, dostupná pomocí Ctrl+Shift+S.
* Přidána podpora parsování e-knih ve formátu FB2!
* Přidána podpora parsování OpenDocument prezentací!
* Přidána podpora parsování OpenDocument textových souborů!
* Záložky nyní mohou označovat celý řádek nebo jen vybranou část textu. Pokud při vytvoření záložky nemáte nic označeno, chová se to stejně jako před verzí 0.6 a označí se celý řádek. Pokud ale nějaký text vyberete, bude do záložky zahrnut pouze tento text.
* K záložkám lze nyní připojit volitelné textové poznámky! Mezi záložkami obsahujícími poznámky se můžete pohybovat pomocí N a Shift+N, případně můžete vyvolat dialog záložek se zobrazením všech záložek, jen poznámek nebo jen záložek bez poznámek pomocí konkrétních klávesových zkratek.
* Záložky v dialogu Záložky už nebudou mít obtěžující předponu „záložka x“.
* Knihy Epub obsahující HTML kód, který se tváří jako XML, budou nyní zpracovávány správně.
* Opraveno načítání velkých dokumentů Markdown.
* Opravena chyba, kdy stisknutí mezerníku ve stromovém zobrazení obsahu aktivovalo tlačítko OK.
* Opraveno zpracování bílých znaků na začátku značek pre v dokumentech HTML i XHTML.
* Opravena chyba, kdy textové pole někdy po návratu do okna Paperbacku znovu nezískalo fokus.
* Opravena chyba, kdy textové pole v dialogu Přejít na procenta neaktualizovalo hodnotu posuvníku.
* Opraveno vykreslování vlastních HTML ID v dokumentech Markdown.
* HTML uvnitř bloků kódu v Markdownu se nyní bude vykreslovat správně.
* Při načítání knihy pomocí parametru příkazového řádku za běhu již existující instance Paperbacku už nedostanete chybu, pokud načtení dokumentu trvá déle než 5 sekund.
* Pokud Paperback spouštíte jako správce, konfigurace se nyní bude správně načítat i ukládat.
* Nyní je možné smazat záložku přímo z dialogu Záložky.
* Nyní je možné importovat a exportovat vaše záložky a pozici čtení pro konkrétní dokument. Vygenerovaný soubor se pojmenuje podle daného souboru s příponou .paperback. Pokud se takový soubor při načítání najde ve stejné složce jako dokument, načte se automaticky. Jinak ho můžete importovat ručně pomocí položky v nabídce Nástroje.
* Odkazy uvnitř dokumentů jsou nyní plně podporovány! Pomocí K a Shift+K se mezi nimi můžete pohybovat vpřed a vzad a stisknutím Enteru některý otevřít nebo aktivovat.
* Mnoho interních refaktorizací, díky nimž je aplikace rychlejší a binární soubor menší.
* Obsah v Markdownu se nyní před vykreslením předzpracovává tak, aby odpovídal CommonMarku.
* Navigace po seznamech a jejich položkách je nyní plně podporována! Pomocí L a Shift+L přecházíte po samotných seznamech a pomocí I a Shift+I po položkách seznamu.
* Kromě běžné klávesy Delete nyní také funguje klávesa Delete na numerické klávesnici pro odstraňování dokumentů ze seznamu záložek.
* Paperback se nyní může volitelně minimalizovat do systémové lišty! Tato volba je ve výchozím stavu vypnutá, ale po jejím zapnutí přesune možnost minimalizace v systémové nabídce Paperback do lišty, odkud jej bude možné obnovit kliknutím na vytvořenou ikonu.
* Paperback je nyní plně přeložitelný! Seznam podporovaných jazyků je zatím poměrně malý, ale neustále roste!
* Paperback má nyní oficiální web na adrese [paperback.dev](https://paperback.dev)!
* Dokumenty PPTX nyní budou zobrazovat základní obsah všech slidů.
* V dialogu s informacemi o dokumentu se nyní bude zobrazovat úplná cesta k otevřenému dokumentu.
* Instalační program nyní obsahuje možnost po instalaci zobrazit readme v prohlížeči.
* Seznam nedávných dokumentů byl výrazně rozšířen! Místo pouhého zobrazení posledních 10 otevřených dokumentů nyní zobrazí nastavitelný počet a ostatní dokumenty, které jste kdy otevřeli, budou dostupné přes malý dialog.
* Různá drobná vylepšení parserů napříč aplikací, včetně vložení prázdného řádku mezi slidy v prezentacích PPTX, opravy zpracování nových řádků uvnitř odstavců v dokumentech Wordu a přidání odrážek k položkám seznamů.

### Verze 0.5.0
* Přidána podpora dokumentů Microsoft Wordu!
* Přidána podpora PowerPointových prezentací!
* Opravena chyba, kdy některé položky nabídky nebyly zakázány, když nebyl otevřen žádný dokument.
* Opravena orientace posuvníku v dialogu Přejít na procenta.
* Opraven obsah v knihách Epub s URL-kódovanými cestami k souborům a/nebo ID fragmentů.
* Opraveno podivné odstraňování bílých znaků z nadpisů XHTML.
* Opraveno zpracování bílých znaků uvnitř vnořených značek pre v dokumentech HTML.
* Dokumenty HTML a Markdown nyní podporují funkci obsahu! Když načtete dokument HTML/Markdown, Paperback sestaví vlastní obsah ze struktury nadpisů v dokumentu a zobrazí vám ho v dialogu Ctrl+T.
* Dokumenty HTML nyní budou mít název nastavený podle značky title, pokud existuje. Jinak se nadále použije název souboru bez přípony.
* Místo UniversalSpeech se nově používá live region pro oznamování řeči. To znamená, že se spolu s programem už nedodávají žádné DLL soubory pro odečítače obrazovky a bude nyní podporováno více odečítačů, například Microsoft Narrator.
* Změněny ZIP knihovny, aby bylo možné otevírat širší škálu knih Epub.
* Dialog, který se vás ptá, zda chcete dokument otevřít jako prostý text, byl úplně přepracován a nyní umožňuje otevřít dokument jako prostý text, HTML nebo Markdown.
* Dialog Přejít na procenta nyní obsahuje textové pole, do něhož lze ručně zadat procento, na které se má přeskočit.
* HTML parser nyní rozpozná dd, dt a dl jako prvky seznamu.
* Obsah v knihách Epub bude opět zachován přesně.
* Unicode nezalomitelná mezera se nyní bere v úvahu při odstraňování prázdných řádků.
* Program se vás už nebude ptát, jak chcete otevřít nerozpoznaný soubor, při každém otevření, ale jen při prvním.

### Verze 0.4.1
* Do instalátoru byla přidána volitelná ikona v nabídce Start.
* Zobrazování obsahu by nyní mělo být v některých případech čistší; například pokud máte podřízenou i nadřazenou položku se stejným textem na stejné pozici, uvidíte nyní jen nadřazenou položku.
* Opraven obsah u některých dokumentů CHM.
* Opraven obsah v knihách Epub 3, které obsahovaly absolutní cesty.
* Dokumenty CHM by nyní měly zobrazovat název nastavený v souboru metadat.

### Verze 0.4.0
* Přidána podpora souborů CHM!
* Přidána podpora záložek! Můžete jich mít libovolné množství v libovolném počtu dokumentů. Můžete mezi nimi skákat vpřed a vzad pomocí B a Shift+B, nastavovat je pomocí Ctrl+Shift+B a vyvolat dialog pro skok na konkrétní záložku pomocí Ctrl+B.
* Kromě portable ZIP souboru byl přidán i instalátor! Ten nainstaluje Paperback do adresáře Program Files a automaticky za vás nastaví asociace souborů.
* Textové soubory se značkou BOM by se nyní měly dekódovat správně a BOM se už nebude zobrazovat ani na začátku textu.
* Do stavového řádku bylo přidáno mnohem více informací. Nyní se v něm zobrazí aktuální řádek, znak a procento přečtení.
* HTML komentáře ani obsah značek script a style se už v textovém výstupu nebudou zobrazovat.
* Pokud Paperbacku na příkazové řádce předáte relativní cestu, nyní ji správně vyhodnotí.
* Pohyb po procentech je nyní řešen vlastním dialogem s posuvníkem, dostupným pomocí Ctrl+Shift+G.
* Dokumenty bez známého názvu nebo autora budou nyní vždy mít výchozí hodnotu.
* Logika ukládání pozice je nyní mnohem chytřejší a na disk by měla zapisovat jen tehdy, když je to opravdu nutné.
* Dokument, který jste měli aktivní při zavření Paperbacku, se nyní znovu otevře po restartu aplikace.
* Vstup v dialozích Přejít na řádek a Přejít na stránku by nyní měl být přísněji čištěn.
* Opravena navigace v obsahu v knihách Epub 3 s relativními cestami v manifestech.

### Verze 0.3.0
* Opraven obsah v knihách Epub s URL-kódovanými manifesty.
* Opravena navigace po nadpisech v dokumentech HTML obsahujících vícebajtové znaky Unicode.
* Opraveno vysoké využití CPU u dokumentů s dlouhými názvy kvůli regresi ve wxWidgets.
* Opraveno načítání textových souborů UTF-8.
* Opravena chyba, kdy vnořené položky obsahu v knihách Epub umisťovaly kurzor na nesprávnou pozici.
* V některých případech opraven pád při ukončení aplikace.
* Do dialogu Možnosti bylo přidáno zaškrtávací políčko pro zapnutí nebo vypnutí zalamování řádků!
* Nyní je možné přispět na vývoj Paperbacku, a to buď přes novou položku Přispět v nabídce Nápověda, nebo přes odkaz sponsor this project ve spodní části hlavní stránky repozitáře na GitHubu.
* Dokumenty Markdown nyní budou mít vždy název a Paperback by nyní měl zvládnout načíst prakticky jakýkoli soubor Markdown.
* Dokumenty PDF nyní budou mít vždy název, i když metadata chybějí.
* Pro PDF byla nasazena knihovna používaná v Chromiu, což vede k výrazně spolehlivějšímu parsování PDF napříč aplikací.
* Nyní může být spuštěna vždy jen jedna instance Paperbacku. Spuštění paperback.exe s názvem souboru ve chvíli, kdy už Paperback běží, otevře tento dokument v již běžící instanci.
* Na libovolném dokumentu v seznamu záložek nyní můžete stisknout Delete a zavřít jej.

### Verze 0.2.1
* Do popisku stránky v dialogu Přejít na stránku byl přidán celkový počet stránek.
* Umožněn přesun tabulátorem z obsahu dokumentu na seznam otevřených dokumentů.
* Opravena chyba, kdy klávesy pro pohyb po nadpisech někdy otevíraly poslední dokumenty, pokud jich bylo dostatek.
* Paperback nyní z textového výstupu odstraňuje zbytečné měkké pomlčky.
* Opravena chyba, kdy navigace po nadpisech někdy umístila kurzor na nesprávný znak.

### Verze 0.2.0
* Přidána podpora dokumentů Markdown!
* Přidána podpora dokumentů PDF, včetně možnosti pohybovat se po stránkách!
* Přidány klávesové zkratky pro navigaci po nadpisech v HTML obsahu, včetně knih Epub a dokumentů Markdown. Tyto klávesové zkratky byly navrženy tak, aby fungovaly podobně jako u odečítačů obrazovky.
* Opraveno načítání Epubů s URL-kódovanými názvy souborů v manifestech.
* Opraveno načítání knih Epub 3 s vloženým obsahem XHTML.
* Pokud dokument nepodporuje obsah nebo oddíly, je nyní místo zakázání příslušných položek nabídky přečtena odpovídající zpráva.
* Přidána nabídka nedávných dokumentů! Momentálně ukládá posledních 10 otevřených dokumentů a stisknutí Enteru na některém z nich jej otevře pro čtení.
* Dialog Hledat byl kompletně přepsán, takže je teď mnohem jednodušší na používání, a zároveň byla přidána historie posledních 25 hledání a podpora regulárních výrazů!
* Dříve otevřené dokumenty nyní zůstanou zapamatovány i po restartu aplikace. Toto chování lze nastavit pomocí nové položky Možnosti v nabídce Nástroje.
* Přidána klávesová zkratka Shift+F1 pro otevření readme přímo v Paperbacku.

### Verze 0.1.0
* První vydaná verze.
