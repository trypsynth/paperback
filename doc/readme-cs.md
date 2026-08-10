<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Brožovaná kniha -- verze 0.8.5 {#paperback---version-0.8.5}

## Úvod {#introduction}

Paperback je lehký, rychlý a přístupný čtečka elektronických knih a
dokumentů pro všechny, od příležitostných čtenářů až po náročné
uživatele. Je navržen s ohledem na přístupnost pro čtečky obrazovky,
vysokou rychlost a plynulý zážitek bez zbytečných prvků.

## Systémové požadavky {#system-requirements}

Paperback v současné době běží na Windows, macOS, iOS a Androidu.

## Funkce {#features}

-   Zcela samostatná aplikace, která k zahájení čtení nevyžaduje
    instalaci žádného softwaru do počítače.
-   Neuvěřitelně rychlý, a to i na starším hardwaru.
-   Jednoduché rozhraní s kartami, které vám umožňuje otevřít libovolný
    počet dokumentů vedle sebe.
-   Ukládá přesnou pozici čtení ve všech dokumentech, které otevřete.
-   Volitelně si pamatuje, které dokumenty jste měli otevřené při
    zavření programu, a obnoví je při příštím spuštění.
-   Obsahuje navigační funkce podobné těm, které najdete v režimu
    prohlížení webových stránek u mnoha čteček obrazovky, pro rychlou a
    snadnou navigaci v dokumentech.
-   Obsahuje robustní dialogové okno pro vyhledávání, včetně funkcí,
    jako je historie a podpora regulárních výrazů.
-   Lze spustit zcela přenosně nebo nainstalovat s automaticky
    nastavenými přidruženími souborů.
-   Podporuje širokou škálu běžných formátů souborů.

## Kompatibilita se čtečkami obrazovky {#screen-reader-compatibility}

Paperback dobře funguje se všemi hlavními čtečkami obrazovky. Existuje
však jeden známý problém pro uživatele programu JAWS.

### JAWS a braillské displeje {#jaws-and-braille-displays}

Pokud používáte JAWS s braillovým displejem, můžete zjistit, že dlouhé
odstavce jsou zkráceny při posunu vpřed pomocí navigačních kláves vašeho
displeje. Ovlivněn je také příkaz pro čtení aktuálního odstavce. Jedná
se o chybu ve zpracování textového ovládacího prvku RICHEDIT50W
programem JAWS, nikoli o problém v samotném programu Paperback, a trvalo
poměrně dlouho, než se objevila oprava, a to i přes nadšení společnosti
Vispero reagovat na problémy s open source softwarem.

Dočasné řešení, které se nakonec po měsících čekání objevilo v diskusní
skupině programu JAWS, spočívá v úpravě `paperback.jcf` a nastavit
položku „Zobrazení a posun v Braillově písmu" na „Vždy použít DOM, je-li
k dispozici". Dále je třeba povolit možnost „Posun textu po odstavcích",
jinak se váš displej zastaví na aktivním odstavci, místo aby pokračoval
dál. S oběma nastaveními by posun měl fungovat správně.

## V současné době podporované typy souborů {#currently-supported-file-types}

Paperback podporuje následující formáty a přípony:

-   Nápovědové soubory CHM (`.chm`)
-   knihy ve formátu DAISY (`.opf`, `.zip`)
-   knihy ve formátu EPUB (`.epub`)
-   elektronické knihy ve formátu FB2 (`.fb2`)
-   HTML dokumenty (`.htm`, `.html`, `.xhtml`)
-   Dokumenty ve formátu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`,
    `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Dokumenty Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Knihy ve formátu MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Prezentace ve formátu OpenDocument (`.odp`, `.fodp`)
-   Textové soubory OpenDocument (`.odt`, `.fodt`)
-   Dokumenty ve formátu PDF (`.pdf`)
-   Prezentace v PowerPointu (`.pptx`, `.pptm`, `.ppt`)
-   Dokumenty RTF (`.rtf`)
-   Soubory s prostým textem a soubory protokolů (`.txt`, `.log`)

## Klávesové zkratky {#keyboard-shortcuts}

Aplikace Paperback je navržena pro ovládání především pomocí klávesnice.
Zde jsou aktuální klávesové zkratky.

Níže uvedené klávesové zkratky platí pro Windows. Tam, kde se liší od
macOS, je ekvivalent uveden v závorkách --- hlavně proto, že kombinace
Ctrl+G, Ctrl+W a Alt+Šipka vlevo/vpravo jsou na této platformě již
obsazeny jinými systémovými nebo aplikačními konvencemi. V nabídce

### Nabídka Soubor {#file-menu}

-   `Ctrl+O`: Otevřít dokument.
-   `Ctrl+F4` (macOS: `Cmd+W`): Zavře aktuální dokument.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Zavřít všechny otevřené
    dokumenty.
-   `Ctrl+Shift+T`: Znovu otevřít naposledy zavřený dokument.
-   `Ctrl+R`: Zobrazí dialogové okno „Všechny dokumenty" (z nabídky
    Poslední dokumenty).
-   `Ctrl+Q`: Ukončit (pouze Windows; v systému macOS se tato volba
    nachází v nabídce aplikace).

### Nabídka Přejít {#go-menu}

-   `Ctrl+F`: Zobrazit dialogové okno „Najít".
-   `F3` (macOS: `Cmd+G`): Najít další.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Najít předchozí.
-   `Ctrl+G` (macOS: `Cmd+L`): Přejít na řádek.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Přejít na procento.
-   `Ctrl+P`: Přejít na stránku (pokud to aktuální dokument podporuje).
-   `Alt+Left` (macOS: `Cmd+[`): Vrátit se zpět v historii navigace.
-   `Alt+Right` (macOS: `Cmd+]`): Přejít vpřed v historii navigace.
-   `[`: Předchozí část.
-   `]`: Další část.
-   `Shift+H`: Předchozí nadpis.
-   `H`: Další nadpis.
-   `Shift+1` až po `Shift+6`: Předchozí nadpis na úrovni 1--6.
-   `1` až po `6`: Další nadpis na úrovni 1--6.
-   `Shift+P`: Předchozí stránka.
-   `P`: Další stránka.
-   `Shift+B`: Předchozí záložka.
-   `B`: Další záložka.
-   `Shift+N`: Předchozí poznámka.
-   `N`: Další poznámka.
-   `Ctrl+B`: Přejít na všechny záložky a poznámky.
-   `Ctrl+Alt+B`: Přejít pouze na záložky.
-   `Ctrl+Alt+M`: Přejít pouze k poznámkám.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tj. fyzická klávesa
    Control namísto Cmd): Zobrazit text poznámky na aktuální pozici.
-   `Shift+K`: Předchozí odkaz.
-   `K`: Další odkaz.
-   `Shift+G`: Předchozí obrázek.
-   `G`: Další obrázek.
-   `Shift+F`: Předchozí obrázek.
-   `F`: Další obrázek.
-   `Shift+T`: Předchozí tabulka.
-   `T`: Další tabulka.
-   `Shift+S`: Předchozí oddělovač.
-   `S`: Další oddělovač.
-   `Shift+L`: Předchozí seznam.
-   `L`: Další seznam.
-   `Shift+I`: Předchozí položka seznamu.
-   `I`: Další položka seznamu.
-   `Shift+,`: Přejít na začátek aktuálního kontejneru (seznamu nebo
    tabulky).
-   `,`: Přejít za konec aktuálního kontejneru (seznamu nebo tabulky).

### Nabídka Nástroje {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, tj. fyzická klávesa Control namísto
    Cmd): Zobrazit počet slov v aktuálním dokumentu.
-   `Ctrl+I`: Zobrazit informace o dokumentu.
-   `Ctrl+T`: Zobrazit obsah.
-   `F7`: Zobrazit seznam prvků.
-   `Ctrl+Shift+C`: Otevřít obsahovou složku.
-   `Ctrl+Shift+V`: Otevřít aktuální obsah ve Web View.
-   `Ctrl+U`: Zobrazit zdrojový kód dokumentu v nové záložce.
-   `Ctrl+Shift+E`: Exportovat data dokumentu (`.paperback`).
-   `Ctrl+Shift+I`: Importovat data dokumentu (`.paperback`).
-   `Ctrl+E`: Exportovat aktuální dokument jako prostý text.
-   `Ctrl+Shift+B`: Přepnout záložku na aktuální výběr/pozici kurzoru.
-   `Ctrl+Shift+N`: Přidat nebo upravit poznámku k záložce na místě
    aktuálního výběru/kurzoru.
-   `Ctrl+Alt+W`: Zapnout/vypnout zalomení řádků.
-   `Ctrl+,`: Otevřít možnosti (macOS: Předvolby, v nabídce aplikace ).
-   `Ctrl+Shift+S`: Zapnout/vypnout časovač uspání.

### Nabídka Nápověda {#help-menu}

-   `Ctrl+F1`: Zobrazit dialogové okno „O aplikaci".
-   `F1`: Zobrazit nápovědu ve výchozím prohlížeči.
-   `Shift+F1`: Zobrazit nápovědu v aplikaci Paperback.
-   `Ctrl+Shift+U`: Zkontrolovat dostupnost aktualizací.
-   `Ctrl+D`: Otevřít stránku pro dary ve vašem výchozím prohlížeči.

### Další klávesové zkratky pro prohlížení dokumentů {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` na ovládacím panelu karet: Zavřít
    vybranou kartu dokumentu.
-   `Enter` nebo `Space` v textu dokumentu: Aktivujte odkaz v místě
    kurzoru nebo otevřete zobrazení tabulky, pokud se kurzor nachází na
    značce tabulky.
-   `Shift+F10` nebo klávesa Menu/Aplikace v textu dokumentu : Otevře
    kontextové menu.

## Podporované jazyky {#supported-languages}

Aplikace Paperback je přeložena do mnoha různých jazyků a další se
neustále přidávají. Kompletní seznam najdete níže.

Chcete-li se dozvědět, jak přispět, přečtěte si prosím naši [Příručku
pro překlad](translating.md).

-   Bosenský
-   Čeština
-   nizozemština
-   Finský
-   Francouzština
-   němčina
-   japonsky
-   polština
-   portugalština (Brazílie)
-   ruština
-   Zjednodušená čínština
-   Srbština
-   španělština
-   Vietnamština

## Zásluhy {#credits}

### Vývoj {#development}

-   Quin Gillespie: hlavní vývojář a zakladatel projektu.
-   Aryan Choudhary: hlavní přispěvatel.

### Dary {#donations}

Následující osoby poskytly dary v určité výši na vývoj projektu
Paperback. Pokud poskytnete dar, vaše jméno sem nebude automaticky
přidáno; přidávám pouze ty, kteří si přejí, aby jejich dar byl
zveřejněn.

Poznámka: Veřejný status sponzora na GitHubu považuji za důvod pro
automatické zařazení do tohoto seznamu.

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

## Seznam změn {#changelog}

### Verze 0.9.0 (nevydaná) {#version-0.9.0-unreleased}

-   Do dialogového okna s probíhající aktualizací bylo přidáno tlačítko
    pro zrušení.
-   Byl přidán nástroj CLI s názvem pb, který umožňuje rychle převést
    jakýkoli z formátů podporovaných aplikací Paperback do formátu HTML,
    Markdown nebo prostého textu.
-   Přidána konfigurovatelná klávesová zkratka pro obnovení aplikace
    Paperback z oblasti oznamovacího panelu.
-   Do dialogového okna „Všechny dokumenty" bylo přidáno tlačítko pro
    vyhledání chybějících knih, u kterých došlo ke změně cesty.
-   Do dialogového okna „Možnosti" byla přidána záložka „Čitelnost" s
    následujícími možnostmi:
    -   Zalamování textu (přesunuto z části „Obecné");
    -   Zobrazení tabulek v textu (novinka v této verzi, viz níže);
    -   Písmo;
    -   Barva pozadí;
    -   Řádkování;
    -   Mezery mezi odstavci;
    -   Mezery mezi písmeny;
    -   Zarovnání textu.
-   Bylo přidáno přepínací tlačítko pro nastavení způsobu zobrazení
    tabulek a bylo sjednoceno zobrazení tabulek napříč dokumenty.
-   Byla přidána možnost „Zobrazit zdroj", která otevírá zdrojový kód
    dokumentu v nové záložce, což je užitečné například při úpravách
    Markdownu.
-   Do dialogového okna pro počítání slov byla přidána odhadovaná doba
    čtení, stejně jako možnost nastavit svou rychlost čtení, aby byla
    tato metrika skutečně užitečná.
-   Přidána podpora ARM64 pro Windows!
-   Přidána podpora pro Android!
-   Přidána podpora pro iOS!
-   Přidána podpora pro macOS!
-   Přidány nové jazyky: nizozemština, finština a polština.
-   Přidána podpora navigace podle kontejnerů.
-   Přidána podpora seznamů, položek seznamů, obrázků a fotografií v
    dokumentech CHM .
-   Přidána položka nabídky pro zalomení textu a příslušná klávesová
    zkratka.
-   Zvuky záložek a poznámek by se nyní měly správně přehrávat výhradně
    při pohybu kurzoru nad slovem, které je obsahuje.
-   Dokumenty kódované ve starších kódováních CJK, jako jsou GBK, Big5 a
    Shift_JIS, se nyní budou správně zobrazovat namísto zobrazení jako
    nesrozumitelné znaky.
-   Rozšířena položka nabídky „Exportovat", která nyní kromě prostého
    textu umožňuje export do formátů HTML a Markdown.
-   Opravena chyba, kdy přepnutí na zalomení řádků přesunulo kurzor na
    začátek dokumentu.
-   Opravena chyba, kvůli které knihy ve formátu Daisy zobrazovaly
    nesprávné informace ve stavovém řádku.
-   Opraveno, že prvky dl, dt a dd nevytvářely konce řádků v dokumentech
    XHTML .
-   Opravena chyba, kdy klávesa Escape nezavírala dialogová okna
    „Informace o dokumentu" a „Všechny dokumenty". Opravena chyba, kdy
    kotvy filepos v knihách ve formátu Mobi rozdělovaly HTML tagy a
    vkládaly
-   Opraveno, že kotvy filepos v knihách ve formátu Mobi rozdělovaly
    HTML tagy a vkládaly nesmyslné znaky do textu knihy.
-   Opraveno zpoždění při přibližování se ke konci textového pole ve
    velkých dokumentech.
-   Opraveny odkazy ve starších knihách ve formátu Mobi.
-   Opraveno načítání knih ve formátu DAISY s nesprávnými deklaracemi
    kódování.
-   Opraveno hlášení nesprávného textu řádku při navigaci mezi stránkami
    v některých situacích.
-   Opraveno parsování dokumentů RTF obsahujících nelatinské znaky.
-   Opravena funkce „Znovu otevřít naposledy zavřený", která se
    pokoušela znovu otevřít přiložený soubor readme.
-   Opraveno neaktualizování záhlaví po zavření dokumentu z dialogového
    okna „Všechny dokumenty".
-   Opraveno, že dialogové okno webového prohlížeče nebylo možné změnit
    a objevovalo se ve velmi malé počáteční velikosti.
-   Opraveno nesprávné vykreslování nadpisů v dokumentech Word s názvy
    stylů specifickými pro dané národní prostředí.
-   Opravena chyba, kdy se vybraná záložka po restartování aplikace
    Paperback správně nezvýraznila.
-   Pokud je při otevření dialogového okna pro počítání slov aktivní
    výběr, nyní se zobrazí počet slov, které jste vybrali.
-   Obrázky by se nyní měly správně zobrazovat ve vloženém webovém
    prohlížeči.
-   Vylepšeno zpracování souborů aplikací Paperback na síťových discích
    systému Windows: stisknutím tlačítka „Zobrazit soubor ve složce" se
    nyní správně zaostří na soubor v síťovém úložišti a cesty již
    neobsahují podivné znaky.
-   Výrazně vylepšeno parsování formátu AZW3.
-   Přešli jsme z knihovny chmlib na náš vlastní čtečku souborů CHM
    napsanou čistě v jazyce Rust.
-   Na počítači již nebudou soubory .paperback při obnově dokumentu
    násilně načítány. Místo toho budete požádáni o potvrzení, jakmile
    bude soubor nalezen.
-   Paperback nyní u nesprávně označených souborů PDF používá jako
    záložní řešení extrakci prostého textu.
-   Funkce „Otevřít obsahovou složku" nyní v Průzkumníku zvýrazní daný
    soubor.
-   Při otevření souboru readme se nyní zohlední vámi vybraný jazyk.
-   Dokumenty PowerPoint nyní podporují tabulky.
-   Při otevření nápovědy v aplikaci Paperback se správně aktualizuje
    nabídka a zaměří se na textové pole.
-   Soubor „Readme.html" již nebude přidán do seznamu všech dokumentů,
    pokud je otevřen pomocí klávesové zkratky Shift+F1.
-   Odstranění dokumentů z dialogového okna „Nedávné" nyní také zavře
    jejich aktivní kartu.
-   Ve Windows jsme přešli na mnohem bezpečnější metodu IPC.
-   Při přepínání mezi kartami se nyní načte název aktivního dokumentu.
-   Aktualizační program nyní správně zobrazuje obsah značek kódu
    Markdown v poznámkách k vydání.
-   Aktualizační program nyní ověřuje, zda stažený soubor nebyl
    manipulován.
-   Webový prohlížeč se nyní otevírá na vaší aktuální pozici ve čtení.
-   Váš vyhledávací filtr v dialogovém okně „Všechny dokumenty" se nyní
    zachová i po odstranění dokumentu.

### Verze 0.8.5 {#version-0.8.5}

-   Přidána podpora stránek pro knihy ve formátu EPUB.
-   Přidána podpora šifrovaných dokumentů Microsoft Office. V současné
    době jsou podporovány starší verze Wordu, moderní Word a moderní
    PowerPoint, přičemž podpora starších verzí PowerPointu je plánována
    na budoucnost.
-   Přidána podpora starších dokumentů Microsoft Word (\*.doc)!
-   Přidána podpora starších prezentací v Powerpointu (\*.ppt)!
-   Přidána podpora knih ve formátech mobi a AZW3!
-   Přidána podpora pro PDF soubory s tagy!
-   Přidána klávesová zkratka Ctrl+Q pro ukončení aplikace.
-   Přidána podpora pro knihy ve formátu ZIP z Bookshare (jak DAISY, tak
    Word)!
-   Alternativní text pro vložené obrázky by se nyní měl správně
    zobrazovat.
-   Dokumenty CHM nyní správně podporují navigaci pomocí interních
    odkazů.
-   Opraveno spouštění zvuků záložek na začátku odstavce namísto pozice
    záložky.
-   Opraveno posunutí funkce „Přejít na stránku" o 1.
-   Opravena nefunkčnost klávesy Esc při zavírání dialogového okna
    „Otevřít jako".
-   Opraveno nezobrazování kontextového menu čtečky při kliknutí pravým
    tlačítkem myši nebo stisknutí klávesy „Aplikace".
-   Opraveno, že při otevírání dokumentů z příkazového řádku byl někdy
    aktivován nesprávný dokument.
-   PDF soubory obsahující pouze obrázky jsou opět detekovány a uživatel
    je na jejich existenci upozorněn.
-   Nyní je možné procházet obrázky a grafy pomocí klávesových zkratek
    g/Shift+g a f/Shift+f.
-   Aplikace Paperback nyní respektuje nastavení tmavého režimu vaší
    aplikace.
-   Byla odstraněna podpora formátu DAISY XML, protože již není
    potřebná.
-   V stromu obsahu jsme se vrátili k nativní navigaci podle prvního
    písmene v prostředí Win32.
-   Dialogové okno s chybou při načítání nyní zobrazuje podrobnější
    chybové zprávy.
-   Webový prohlížeč se nyní otevírá mnohem rychleji a plynuleji.

### Verze 0.8.2 {#version-0.8.2}

-   Do dokumentů RTF byla přidána podpora stránek!
-   Opravena chyba, kvůli které se při otevření webového prohlížeče v
    souborech ePub obsahujících externí odkazy tyto odkazy automaticky
    aktivovaly.
-   Opravena chyba, kvůli které analyzátor RTF v ojedinělých případech
    nevkládal mezeru mezi slova .
-   Opraveno rozdělování odstavců na několik krátkých řádků v některých
    PDF dokumentech.
-   Dokumenty PDF nyní podporují základní navigaci pomocí odkazů a
    nadpisů !
-   Zarážky a konce řádků v RTF se nyní vykreslují přesně tak, jak se
    objevují v dokumentu.
-   Pro analýzu souborů PDF jsme se vrátili k osvědčené knihovně pdfium,
    díky čemuž je vykreslování PDF opět mnohem spolehlivější.

### Verze 0.8.1 {#version-0.8.1}

-   Přidána klávesová zkratka Ctrl+Shift+T pro opětovné otevření
    posledního zavřeného dokumentu.
-   Dialogové okno „Všechny dokumenty" nyní podporuje výběr více
    dokumentů k současnému otevření.
-   Bylo opraveno několik chyb v analyzátoru RTF.
-   Opravena chyba, při které se při otevírání souboru prostřednictvím
    druhé instance aplikace Paperback poškozovaly cesty k souborům
    obsahující znaky mimo znakovou sadu ASCII (například bosenské š, č,
    ć, ž).
-   Opraveno nesprávné pořadí čtení textu v souborech PDF a nesprávné
    mezery kolem slov s velkými počátečními písmeny.
-   Opraveno pomalé načítání dokumentů při otevírání velkých souborů.
-   Opravena lokalizace tlačítek „Ano"/„Ne" v potvrzovacích dialogových
    oknech.

### Verze 0.8.0 {#version-0.8.0}

-   Přidány překlady do japonštiny, zjednodušené čínštiny a vietnamštiny
    !
-   Přidán automatický aktualizační modul, který nyní nahradí vaši
    aktuálně nainstalovanou verzi aplikace Paperback, namísto pouhého
    stažení nové verze!
-   Přidána volitelná zvuková zpětná vazba při dosažení záložky nebo
    poznámky, děkujeme Andre Louisovi za zvuky!
-   Přidána podpora dokumentů RTF!
-   Přidána podpora dokumentů DAISY XML.
-   Přidána podpora textových souborů Flat Open Document!
-   Přidána podpora prezentací ve formátu Flat Open Document!
-   Přidána podpora oddělovačů pomocí kláves s a Shift+s.
-   Jakýkoli posun o více než 300 znaků se nyní automaticky přidá do
    vaší historie navigace.
-   Opravena obnova okna aplikace Paperback z oznamovací oblasti.
-   Opravena chyba, kvůli které se dokumenty Markdown zobrazovaly ve Web
    View jako nezpracovaný text namísto vykresleného HTML. Opraveno
    nesprávné vykreslování tabulek v souborech Markdown.
-   Opraveno nesprávné vykreslování tabulek v souborech Markdown.
-   PDF soubory obsahující pouze obrázky vás nyní upozorní na svou
    existenci, když se pokusíte některý z nich načíst.
-   Při kontrole aktualizací je nyní možné vyhledávat nové vývojové
    verze namísto stabilních verzí.
-   Informace o verzi jsou nyní správně vloženy do spustitelného souboru
    aplikace Paperback.
-   Rozdělili jsme dialogové okno s nastavením na záložky pro snadnější
    používání a orientaci.
-   Pro analýzu souborů PDF jsme přešli na Hayro, což přineslo vyšší
    spolehlivost, rychlost a menší počet DLL souborů.
-   Celá aplikace byla přepsána v jazyce Rust. Nový kód je bezpečnější,
    načítá dokumenty rychleji a je snazší jej udržovat a rozšiřovat.
-   Kontextové menu textového ovládacího prvku nyní obsahuje akce
    specifické pro daný čtečku namísto obecných položek, jako je vyjmout
    a vložit.

### Verze 0.7.0 {#version-0.7.0}

-   Přidána podpora tabulek pro dokumenty založené na HTML a XHTML! Mezi
    tabulkami se můžete pohybovat pomocí kláves T a Shift+T a stisknutím
    klávesy Enter je můžete zobrazit ve webovém prohlížeči.
-   Přidána základní funkce vykreslování webového obsahu! Stisknutím
    kláves Ctrl+Shift+V otevřete aktuální část dokumentu ve webovém
    prohlížeči, což je užitečné pro obsah, jako je složité formátování
    nebo ukázky kódu.
-   Přidán ruský překlad, děkujeme Ruslanu Gulmagomedovovi!
-   Do dialogového okna „Všechny dokumenty" bylo přidáno tlačítko
    „Vymazat vše".
-   Kontrola aktualizací nyní zobrazuje poznámky k verzi, pokud je k
    dispozici nová verze .
-   Opraveno obnovení okna z systémové lišty.
-   Opraveny překlady tlačítek „Ano"/„Ne" v potvrzovacích dialogových
    oknech.
-   Opraveno načítání konfigurací při spuštění jako správce.
-   Opraveno zpracování komentářů v dokumentech XML a HTML.
-   Opraveno parsování obsahu v knihách ve formátu Epub 2.
-   Opravena navigace na další položku se stejným písmenem v obsahu .
-   Opraveno nesprávné skrytí dialogového okna pro vyhledávání při
    použití tlačítek „další" a „předchozí".
-   Opraveny chyby v obsahu formátu ePub, které občas přesměrovaly na
    nesprávnou položku.
-   Opraveny různé problémy se zpracováním mezer v XML, HTML a značkách
    pre.
-   Opravena chyba „off-by-one" při navigaci pomocí odkazů.
-   Opraveny některé knihy, které měly na konci řádků zbývající mezery.
-   Opraveny různé problémy s parsováním.
-   Položky nabídky související se záložkami i seznam prvků jsou nyní
    správně deaktivovány, pokud není otevřen žádný dokument.
-   Vylepšena práce se seznamy v různých formátech dokumentů.
-   Vylepšen pracovní postup překladu pro přispěvatele.
-   Provedlo se mnoho interních refaktorizací, přičemž většina obchodní
    logiky aplikace byla přesunuta z C++ do Rustu za účelem zlepšení
    výkonu a udržovatelnosti.

### Verze 0.6.1 {#version-0.6.1}

-   Přidána podpora PDF chráněných heslem!
-   Přidána velmi základní funkce přechodu na předchozí/následující
    pozici. Pokud stisknete klávesu Enter na interním odkazu a kurzor se
    posune, tato pozice si nyní bude zapamatována a lze k ní přejít
    pomocí kláves Alt + šipky vlevo/vpravo.
-   Přidán seznam prvků! V současné době zobrazuje pouze strom všech
    nadpisů ve vašem dokumentu nebo seznam odkazů, ale v budoucnu se
    plánuje jeho rozšíření.
-   Přidána možnost spustit Paperback ve výchozím nastavení v
    maximalizovaném režimu.
-   Opraveny odkazy v některých dokumentech ve formátu ePub, které
    nefungovaly správně.
-   Opraveno parsování obsahu (TOC) dokumentů ePub obsahujících
    relativní cesty.
-   Opraveno, že některé dokumenty ve formátu ePub nezobrazovaly název
    ani autora.
-   Opraveno nesprávné zobrazení názvů některých kapitol v souborech
    ePub v dialogovém okně obsahu.
-   Opravena chyba, kvůli které nebylo možné použít mezerník k aktivaci
    tlačítek OK/Zrušit v dialogovém okně obsahu.
-   Bylo vylepšeno zpracování nadpisů v dokumentech Word.
-   Nyní uslyšíte hlasovou zpětnou vazbu, pokud je seznam posledních
    dokumentů prázdný při pokusu o otevření dialogového okna.

### Verze 0.6.0 {#version-0.6.0}

-   Do dialogového okna s nastavením byla přidána nová možnost zobrazit
    nabídku „Přejít" v mnohem kompaktnější podobě, která je ve výchozím
    nastavení zaškrtnuta.
-   Byla přidána možnost, aby se navigace podle strukturálních prvků
    automaticky zalomila.
-   Do nabídky „Nástroje" byla přidána možnost otevřít nadřazenou složku
    aktuálně vybraného dokumentu.
-   Byl přidán poměrně jednoduchý, ale velmi účinný systém aktualizace.
-   Přidána základní funkce časovače uspání, přístupná pomocí klávesové
    zkratky Ctrl+Shift+S.
-   Byla přidána podpora pro analýzu elektronických knih ve formátu FB2!
-   Přidána podpora pro analýzu prezentací ve formátu OpenDocument!
-   Přidána podpora pro analýzu textových souborů OpenDocument!
-   Záložky lze nyní vytvářet tak, aby označovaly celý řádek, nebo aby
    označovaly pouze určitý zadaný text. Pokud při vkládání záložky
    nemáte aktivní žádný výběr, chování je stejné jako před verzí 0.6 a
    bude označen celý řádek. Pokud však vyberete nějaký text, do záložky
    bude zahrnut pouze tento text.
-   K záložkám lze nyní připojit volitelné textové poznámky! Mezi
    záložkami obsahujícími poznámky můžete přecházet pomocí kláves N a
    Shift+N, nebo můžete otevřít dialogové okno se záložkami, kde jsou
    vybrány všechny záložky, pouze poznámky nebo pouze záložky bez
    poznámek, a to pomocí konkrétních klávesových zkratek.
-   Záložky v dialogovém okně záložek již nebudou mít otravnou předponu
    „záložka x".
-   Knihy ve formátu Epub obsahující HTML obsah vydávající se za XML
    budou nyní správně zpracovány.
-   Opraveno načítání velkých dokumentů ve formátu Markdown.
-   Opraveno stisknutí mezerníku ve stromovém zobrazení obsahu, které
    aktivovalo tlačítko OK.
-   Opraveno zpracování mezer na začátku značek pre v dokumentech HTML i
    XHTML.
-   Opraveno, že textové pole někdy nezískalo zpět fokus při návratu do
    okna aplikace Paperback.
-   Opravena chyba, kdy textové pole v dialogovém okně „Přejít na %"
    neaktualizovalo hodnotu posuvníku.
-   Opraveno vykreslování vlastních HTML ID v dokumentech Markdown.
-   HTML uvnitř bloků kódu Markdown se nyní bude vykreslovat správně.
-   Při načítání knihy s parametrem příkazového řádku, zatímco je
    spuštěna stávající instance aplikace Paperback, již nedojde k chybě,
    pokud načítání dokumentu trvá déle než 5 sekund.
-   Při spuštění aplikace Paperback jako správce se nyní konfigurace
    správně načte a uloží.
-   Nyní je možné odstranit záložku přímo z dialogového okna záložek.
-   Nyní je možné importovat a exportovat záložky a pozici čtení pro
    konkrétní dokument. Vygenerovaný soubor nese název souboru s
    příponou .paperback. Pokud je takový soubor nalezen ve stejném
    adresáři jako načítaný soubor, bude automaticky načten. V opačném
    případě je můžete importovat ručně pomocí položky v nabídce
    Nástroje.
-   Odkazy uvnitř dokumentů jsou nyní plně podporovány! Pomocí kláves k
    a Shift+k se můžete mezi nimi pohybovat dopředu a dozadu a
    stisknutím klávesy Enter je můžete otevřít/aktivovat .
-   Bylo provedeno mnoho interních refaktorizací, díky nimž je aplikace
    rychlejší a binární soubor menší.
-   Obsah ve formátu Markdown je nyní před vykreslením předzpracován
    tak, aby byl v souladu se standardem CommonMark .
-   Navigace podle seznamů a jejich položek je nyní plně podporována!
    Pomocí kláves L a Shift+L se můžete pohybovat po samotných seznamech
    a pomocí kláves I a Shift+I procházet položky seznamu.
-   Klávesa Delete na numerické klávesnici nyní slouží k odstranění
    dokumentů z lišty záložek, kromě běžného mazání.
-   Paperback lze nyní volitelně minimalizovat do systémové lišty! Tato
    možnost je ve výchozím nastavení vypnutá, ale její zapnutí způsobí,
    že volba minimalizace v systémovém menu umístí Paperback do
    systémové lišty, odkud jej lze obnovit kliknutím na zobrazenou
    ikonu.
-   Paperback je nyní plně přeložitelný! Seznam jazyků, které podporuje,
    je zatím poměrně malý, ale neustále se rozšiřuje!
-   Paperback má nyní oficiální webovou stránku na adrese
    [paperback.dev](https://paperback.dev)!
-   Dokumenty PPTX nyní zobrazují základní obsah obsahující všechny
    snímky.
-   V dialogovém okně s informacemi o dokumentu se nyní zobrazí úplná
    cesta k otevřenému dokumentu.
-   Instalační program nyní obsahuje možnost zobrazit soubor readme ve
    vašem prohlížeči po instalaci.
-   Seznam naposledy otevřených dokumentů byl výrazně rozšířen! Místo
    prostého zobrazení posledních 10 dokumentů, které jste otevřeli, vám
    nyní zobrazí nastavitelný počet, přičemž zbytek dokumentů, které
    jste kdy otevřeli, je přístupný prostřednictvím malého dialogového
    okna.
-   Různá drobná vylepšení parserů napříč aplikacemi, včetně vkládání
    prázdného řádku mezi snímky v prezentacích PPTX, opravy zpracování
    nových řádků uvnitř odstavců v dokumentech Word a přidání odrážek k
    položkám seznamu.

### Verze 0.5.0 {#version-0.5.0}

-   Přidána podpora dokumentů Microsoft Word!
-   Přidána podpora prezentací PowerPoint!
-   Opravena chyba, kdy některé položky nabídky nebyly deaktivovány,
    pokud nebyly otevřeny žádné dokumenty.
-   Opravena orientace posuvníku pro přechod na určité procento.
-   Opraven obsah v knihách ve formátu Epub s URL-kódovanými cestami k
    souborům a/nebo ID fragmentů.
-   Opravena podivná ztráta mezer v nadpisech XHTML .
-   Opraveno zpracování mezer uvnitř vnořených značek \`pre\` v HTML
    dokumentech.
-   Dokumenty HTML a Markdown nyní podporují funkci obsahu ! Když
    načtete dokument HTML/Markdown, Paperback vytvoří vlastní obsah na
    základě struktury nadpisů ve vašem dokumentu a zobrazí vám jej v
    dialogovém okně ctrl+t.
-   HTML dokumenty budou nyní mít název nastavený v tagu title, pokud
    existuje. V opačném případě budou i nadále používat název souboru
    bez přípony.
-   Přešli jsme z UniversalSpeech na použití živé oblasti pro přečtení
    textu. To znamená, že s programem již nejsou dodávány žádné DLL
    soubory pro čtečky obrazovky a nyní bude podporováno více čteček,
    jako je například Microsoft Narrator.
-   Byly změněny knihovny ZIP, aby bylo možné otevírat širší škálu knih
    ve formátu EPUB .
-   Dialogové okno s dotazem, zda chcete dokument otevřít jako prostý
    text, bylo kompletně přepracováno a nyní umožňuje otevřít dokument
    jako prostý text, HTML nebo Markdown.
-   Dialogové okno „Přejít na procentuální pozici" nyní obsahuje textové
    pole, do kterého můžete ručně zadat procentuální pozici, na kterou
    chcete přejít.
-   HTML parser nyní rozpozná dd, dt a dl jako prvky seznamu. Obsah v
    knihách ve formátu ePub bude opět zachován
-   Obsah v knihách ve formátu ePub bude opět zachován přesně.
-   Při odstraňování prázdných řádků se nyní bere v úvahu nerozdělitelná
    mezera Unicode.
-   Už nebudete pokaždé při načtení neznámého souboru dotazováni, jak
    jej chcete otevřít, ale pouze při prvním načtení.

### Verze 0.4.1 {#version-0.4.1}

-   Do instalačního programu byla přidána volitelná ikona v nabídce
    Start.
-   Obsah by nyní měl být v některých případech přehlednější, například
    pokud máte podřízenou a nadřazenou položku se stejným textem na
    stejné pozici, uvidíte nyní pouze nadřazenou položku.
-   Opraven obsah v některých dokumentech CHM.
-   Opraven obsah v knihách ve formátu Epub 3, které obsahují absolutní
    cesty. Dokumenty ve formátu
-   Dokumenty CHM by nyní měly zobrazovat název tak, jak je nastaven v
    souboru metadat .

### Verze 0.4.0 {#version-0.4.0}

-   Přidána podpora souborů CHM!
-   Přidána podpora záložek! Můžete mít libovolný počet záložek v
    libovolném počtu dokumentů. Mezi nimi můžete přeskakovat dopředu a
    dozadu pomocí kláves b a Shift+b, záložku vytvoříte pomocí
    Ctrl+Shift+b a pomocí Ctrl+b otevřete dialogové okno pro přeskočení
    na konkrétní záložku.
-   K přenosnému souboru ZIP byl přidán instalační program! Instalační
    program nainstaluje Paperback do adresáře Program Files a
    automaticky za vás nastaví přiřazení souborů.
-   Textové soubory s BOM by nyní měly být správně dekódovány a BOM se
    již nebude zobrazovat na začátku textu.
-   Do stavového řádku bylo přidáno mnohem více informací. Nyní vám
    ukáže aktuální řádek, znak a procento přečteného textu.
-   HTML komentáře, stejně jako obsah tagů skriptů a stylů, se již
    nebudou zobrazovat v textovém výstupu.
-   Pokud zadáte relativní cestu k programu Paperback v příkazovém
    řádku, bude nyní správně vyhodnocena.
-   Posun o určité procento se nyní ovládá pomocí vlastního dialogového
    okna s posuvníkem, které lze otevřít klávesovou zkratkou
    Ctrl+Shift+G.
-   Dokumenty bez známého názvu nebo autora budou nyní vždy mít výchozí
    hodnotu.
-   Logika ukládání pozice je nyní mnohem chytřejší a měla by zapisovat
    na disk pouze v případě, že je to nezbytně nutné.
-   Dokument, na kterém jste měli aktivní fokus při zavření aplikace
    Paperback, je nyní zapamatován i po restartu aplikace.
-   Zadávané údaje v dialozích „Přejít na řádek" a „Přejít na stránku"
    by nyní měly být přísněji ověřovány.
-   Opravena navigace v obsahu knih ve formátu ePub 3 s relativními
    cestami v jejich manifestech.

### Verze 0.3.0 {#version-0.3.0}

-   Opraven obsah v knihách ve formátu ePub s manifesty s URL kódováním.
    Opravena navigace v nadpisech v HTML dokumentech obsahujících
    vícejbytové
-   Opravena navigace v nadpisech v HTML dokumentech obsahujících
    vícejbytové znaky Unicode.
-   Opraveno vysoké zatížení procesoru v dokumentech s dlouhými názvy
    způsobené regresí ve wxWidgets.
-   Opraveno načítání textových souborů v kódování UTF-8.
-   Opraveno vnoření položek obsahu v knihách ve formátu ePub, které
    způsobovalo umístění kurzoru na nesprávnou pozici.
-   Opraven pád aplikace při ukončení v určitých případech.
-   Do dialogového okna s nastavením bylo přidáno zaškrtávací políčko
    pro zapnutí nebo vypnutí zalomení řádků!
-   Nyní je možné přispět na vývoj aplikace Paperback, a to buď
    prostřednictvím nové položky „Přispět" v nabídce Nápověda, nebo
    prostřednictvím odkazu „Sponzorovat tento projekt" v dolní části
    hlavní stránky repozitáře GitHub.
-   Dokumenty ve formátu Markdown budou nyní vždy mít název a aplikace
    Paperback by nyní měla být schopna načíst prakticky jakýkoli soubor
    ve formátu Markdown.
-   Dokumenty PDF budou nyní vždy obsahovat název, i když metadata
    chybí.
-   Byla provedena změna knihovny PDF na tu, která se používá v Chromiu,
    což vede k mnohem spolehlivějšímu parsování PDF v celém programu.
-   Nyní můžete mít spuštěnou pouze jednu instanci aplikace Paperback
    najednou. Spuštění souboru paperback.exe s názvem souboru, zatímco
    je aplikace již spuštěna, otevře daný dokument v již spuštěné
    instanci.
-   Nyní můžete stisknout klávesu Delete na dokumentu v ovládacím prvku
    záložek, abyste jej zavřeli.

### Verze 0.2.1 {#version-0.2.1}

-   Do popisku stránky v dialogovém okně „Přejít na stránku" byl přidán
    celkový počet stránek.
-   Umožňuje přechod pomocí klávesy Tab z obsahu dokumentu do seznamu
    otevřených dokumentů.
-   Opravena chyba, kdy klávesové zkratky pro záhlaví někdy otevíraly
    nedávno otevřené dokumenty, pokud jich bylo dostatek.
-   Paperback nyní odstraní z výstupního textu zbytečné měkké pomlčky .
-   Opravena chyba, kvůli které vás navigace podle nadpisů někdy
    přesunula na nesprávný znak.

### Verze 0.2.0 {#version-0.2.0}

-   Přidána podpora dokumentů ve formátu Markdown!
-   Přidána podpora dokumentů PDF, včetně možnosti navigace mezi
    stránkami!
-   Přidány klávesové zkratky pro navigaci podle nadpisů v obsahu HTML,
    včetně knih ve formátu epub a dokumentů v Markdownu. Tyto klávesové
    zkratky byly navrženy tak, aby fungovaly podobně jako čtečka
    obrazovky.
-   Opraveno načítání souborů ePub s názvy souborů kódovanými pomocí URL
    v jejich manifestech.
-   Opraveno načítání knih ve formátu EPUB 3 s vloženým XHTML.
-   Pokud dokument nepodporuje obsah nebo oddíly, je nyní přečtena
    hlasová zpráva, namísto toho, aby byly položky nabídky deaktivovány.
-   Přidáno menu s nedávno otevřenými dokumenty! V současné době ukládá
    vašich posledních 10 otevřených dokumentů a stisknutím klávesy Enter
    na některém z nich jej otevřete ke čtení.
-   Zcela jsme přepracovali dialogové okno „Najít", díky čemuž je jeho
    používání mnohem jednodušší, a zároveň jsme přidali historii
    posledních 25 vyhledávání a podporu regulárních výrazů!
-   Dříve otevřené dokumenty se nyní pamatují i po restartu aplikace.
    Tuto funkci lze nastavit prostřednictvím nové položky v nabídce
    Nástroje. Přidána klávesová zkratka Shift+F1 pro otevření souboru
    readme přímo v aplikaci Paperback.
-   Přidána klávesová zkratka Shift+F1 pro otevření souboru Readme přímo
    v aplikaci Paperback.

### Verze 0.1.0 {#version-0.1.0}

-   První vydání.
