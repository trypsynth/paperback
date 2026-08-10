<!-- machine-translated from doc/readme.md (source-hash: fd39958ee63d8b14); please review and edit as needed -->

# Wydanie w miękkiej oprawie -- wersja 0.8.5 {#paperback---version-0.8.5}

## Wprowadzenie {#introduction}

Paperback to lekki, szybki i przystępny czytnik e-booków i dokumentów
dla wszystkich, od zwykłych czytelników po zaawansowanych użytkowników.
Został zaprojektowany z myślą o dostępności dla czytników ekranu, dużej
szybkości działania i przejrzystym interfejsie.

## Wymagania systemowe {#system-requirements}

Paperback działa obecnie na systemach Windows, macOS, iOS i Android.

## Funkcje {#features}

-   Całkowicie samodzielna aplikacja, nie wymagająca instalacji żadnego
    oprogramowania na komputerze, aby rozpocząć czytanie.
-   Niezwykle szybki, nawet na starszym sprzęcie.
-   Prosty interfejs z zakładkami, umożliwiający otwieranie dowolnej
    liczby dokumentów obok siebie.
-   Zapamiętuje dokładną pozycję czytania we wszystkich otwartych
    dokumentach.
-   Opcjonalnie zapamiętuje, jakie dokumenty były otwarte w momencie
    zamknięcia programu, i przywraca je przy następnym uruchomieniu.
-   Zawiera funkcje nawigacyjne podobne do tych dostępnych w trybie
    przeglądania stron internetowych w wielu czytnikach ekranu,
    umożliwiające szybką i łatwą nawigację po dokumentach.
-   Zawiera rozbudowane okno dialogowe wyszukiwania, w tym funkcje takie
    jak historia i obsługa wyrażeń regularnych.
-   Można go uruchamiać całkowicie w trybie przenośnym lub zainstalować
    z automatycznie skonfigurowanymi skojarzeniami plików.
-   Obsługuje ogromną gamę popularnych formatów plików.

## Zgodność z czytnikami ekranu {#screen-reader-compatibility}

Program Paperback działa dobrze ze wszystkimi głównymi czytnikami
ekranu. Istnieje jednak jeden znany problem dla użytkowników programu
JAWS.

### JAWS i wyświetlacze brajlowskie {#jaws-and-braille-displays}

Jeśli korzystasz z programu JAWS wraz z wyświetlaczem brajlowskim,
możesz zauważyć, że długie akapity są ucinane podczas przewijania do
przodu za pomocą klawiszy nawigacyjnych wyświetlacza. Dotyczy to również
polecenia odczytu bieżącego akapitu. Jest to błąd w obsłudze przez JAWS
kontrolki tekstowej RICHEDIT50W, a nie problem leżący po stronie samego
programu Paperback. Znalezienie rozwiązania tego problemu zajęło sporo
czasu, biorąc pod uwagę entuzjazm firmy Vispero w reagowaniu na
zgłoszenia dotyczące oprogramowania open source.

Rozwiązaniem tymczasowym, które w końcu pojawiło się na grupie
dyskusyjnej JAWS po miesiącach oczekiwania, jest edycja `paperback.jcf`
i ustawienie opcji „Prezentacja brajlowska i przewijanie" na „Zawsze
używaj DOM, jeśli jest dostępny". Warto również włączyć opcję „Przewijaj
tekst według akapitów", w przeciwnym razie wyświetlacz pozostanie na
aktywnym akapicie zamiast przechodzić dalej. Po wprowadzeniu obu
ustawień przewijanie powinno działać poprawnie.

## Obecnie obsługiwane typy plików {#currently-supported-file-types}

Program Paperback obsługuje następujące formaty i rozszerzenia:

-   Pliki pomocy CHM (`.chm`)
-   książki DAISY (`.opf`, `.zip`)
-   Książki w formacie EPUB (`.epub`)
-   e-książki FB2 (`.fb2`)
-   dokumenty HTML (`.htm`, `.html`, `.xhtml`)
-   Dokumenty w formacie Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`,
    `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Dokumenty Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Książki w formacie MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Prezentacje OpenDocument (`.odp`, `.fodp`)
-   Pliki tekstowe OpenDocument (`.odt`, `.fodt`)
-   Dokumenty PDF (`.pdf`)
-   Prezentacje PowerPoint (`.pptx`, `.pptm`, `.ppt`)
-   Dokumenty RTF (`.rtf`)
-   Pliki tekstowe i pliki dziennika (`.txt`, `.log`)

## Skróty klawiaturowe {#keyboard-shortcuts}

Program Paperback został zaprojektowany z myślą o obsłudze przede
wszystkim za pomocą klawiatury. Oto aktualne skróty klawiszowe.

Poniższe skróty dotyczą systemu Windows. W przypadku różnic w systemie
macOS ich odpowiedniki podano w nawiasach --- głównie dlatego, że skróty
Ctrl+G, Ctrl+W oraz Alt+Strzałka w lewo/w prawo są już zajęte przez inne
konwencje systemowe lub aplikacyjne na tej platformie.

### Menu Plik {#file-menu}

-   `Ctrl+O`: Otwórz dokument.
-   `Ctrl+F4` (macOS: `Cmd+W`): Zamknij bieżący dokument.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Zamknij wszystkie otwarte
    dokumenty.
-   `Ctrl+Shift+T`: Ponownie otwórz ostatnio zamknięty dokument.
-   `Ctrl+R`: Wyświetl okno dialogowe „Wszystkie dokumenty" (z sekcji
    Ostatnie dokumenty).
-   `Ctrl+Q`: Zamknij program (tylko w systemie Windows; w systemie
    macOS opcja ta znajduje się w menu aplikacji).

### Menu „Idź" {#go-menu}

-   `Ctrl+F`: Wyświetl okno dialogowe „Znajdź".
-   `F3` (macOS: `Cmd+G`): Znajdź następny.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Znajdź poprzedni.
-   `Ctrl+G` (macOS: `Cmd+L`): Przejdź do wiersza.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Przejdź do wartości
    procentowej.
-   `Ctrl+P`: Przejdź do strony (jeśli jest to obsługiwane przez bieżący
    dokument).
-   `Alt+Left` (macOS: `Cmd+[`): Cofnij się w historii nawigacji.
-   `Alt+Right` (macOS: `Cmd+]`): Przejdź do przodu w historii
    nawigacji.
-   `[`: Poprzednia sekcja.
-   `]`: Następna sekcja.
-   `Shift+H`: Poprzedni nagłówek.
-   `H`: Następny nagłówek.
-   `Shift+1` przez `Shift+6`: Poprzedni nagłówek na poziomie 1--6.
-   `1` przez `6`: Następny nagłówek na poziomie 1--6.
-   `Shift+P`: Poprzednia strona.
-   `P`: Następna strona.
-   `Shift+B`: Poprzednia zakładka.
-   `B`: Następna zakładka.
-   `Shift+N`: Poprzednia notatka.
-   `N`: Następna notatka.
-   `Ctrl+B`: Przejdź do wszystkich zakładek i notatek.
-   `Ctrl+Alt+B`: Przejdź tylko do zakładek.
-   `Ctrl+Alt+M`: Przejdź wyłącznie do notatek.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tj. fizyczny klawisz
    Control zamiast Cmd): Wyświetl tekst notatki w bieżącej pozycji.
-   `Shift+K`: Poprzedni link.
-   `K`: Następny link.
-   `Shift+G`: Poprzedni obrazek.
-   `G`: Następny obrazek.
-   `Shift+F`: Poprzedni rysunek.
-   `F`: Następny rysunek.
-   `Shift+T`: Poprzednia tabela.
-   `T`: Następna tabela.
-   `Shift+S`: Poprzedni separator.
-   `S`: Następny separator.
-   `Shift+L`: Poprzednia lista.
-   `L`: Następna lista.
-   `Shift+I`: Poprzednia pozycja listy.
-   `I`: Następna pozycja listy.
-   `Shift+,`: Przejdź do początku bieżącego kontenera (listy lub
    tabeli).
-   `,`: Przejdź poza koniec bieżącego kontenera (listy lub tabeli).

### Menu Narzędzia {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, tj. fizyczny klawisz Control zamiast
    Cmd): Wyświetl liczbę słów w bieżącym dokumencie.
-   `Ctrl+I`: Wyświetl informacje o dokumencie.
-   `Ctrl+T`: Wyświetl spis treści.
-   `F7`: Wyświetl listę elementów.
-   `Ctrl+Shift+C`: Otwórz folder zawierający dokument.
-   `Ctrl+Shift+V`: Otwórz bieżącą treść w widoku internetowym.
-   `Ctrl+U`: Wyświetl kod źródłowy dokumentu w nowej karcie.
-   `Ctrl+Shift+E`: Eksportuj dane dokumentu (`.paperback`).
-   `Ctrl+Shift+I`: Importuj dane dokumentu (`.paperback`).
-   `Ctrl+E`: Eksportuj bieżący dokument jako zwykły tekst.
-   `Ctrl+Shift+B`: Ustaw lub usuń zakładkę w miejscu bieżącego
    zaznaczenia/kursora.
-   `Ctrl+Shift+N`: Dodaj lub edytuj notatkę zakładki w miejscu
    bieżącego zaznaczenia/kursora.
-   `Ctrl+Alt+W`: Włącz/wyłącz zawijanie tekstu.
-   `Ctrl+,`: Otwórz opcje (macOS: Preferencje, w menu aplikacji ).
-   `Ctrl+Shift+S`: Włącz/wyłącz wyłącznik czasowy.

### Menu Pomoc {#help-menu}

-   `Ctrl+F1`: Wyświetl okno dialogowe „Informacje".
-   `F1`: Wyświetl pomoc w domyślnej przeglądarce.
-   `Shift+F1`: Wyświetl pomoc w aplikacji Paperback.
-   `Ctrl+Shift+U`: Sprawdź dostępność aktualizacji.
-   `Ctrl+D`: Otwórz stronę darowizn w domyślnej przeglądarce.

### Dodatkowe klawisze do przeglądania dokumentów {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` na panelu kart: Zamknij kartę wybranego
    dokumentu.
-   `Enter` lub `Space` w tekście dokumentu: Aktywuj link w miejscu
    kursora lub otwórz widok tabeli, gdy kursor znajduje się na
    znaczniku tabeli.
-   `Shift+F10` lub klawisz Menu/Aplikacja w tekście dokumentu : Otwórz
    menu kontekstowe.

## Obsługiwane języki {#supported-languages}

Aplikacja Paperback została przetłumaczona na wiele różnych języków, a
kolejne są dodawane na bieżąco. Pełna lista znajduje się poniżej.

Aby dowiedzieć się, jak wnieść swój wkład, przeczytaj nasz [Przewodnik
po tłumaczeniach](translating.md).

-   bośniacki
-   Czeski
-   Holenderski
-   Fiński
-   Francuski
-   niemiecki
-   Japoński
-   polski
-   portugalski (Brazylia)
-   Rosyjski
-   Chiński uproszczony
-   Serbski
-   hiszpański
-   Wietnamski

## Podziękowania {#credits}

### Rozwój {#development}

-   Quin Gillespie: główny programista i założyciel projektu.
-   Aryan Choudhary: główny współtwórca.

### Darowizny {#donations}

Następujące osoby przekazały darowizny o znacznej wartości na rzecz
rozwoju projektu Paperback. Jeśli przekażesz darowiznę, Twoje imię i
nazwisko nie zostanie automatycznie dodane do tej listy -- umieszczam na
niej wyłącznie osoby, które wyrażą zgodę na upublicznienie informacji o
swojej darowiźnie.

Uwaga: Uważam, że publiczne wsparcie na GitHubie stanowi podstawę do
automatycznego umieszczenia na tej liście.

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

## Lista zmian {#changelog}

### Wersja 0.9.0 (nieopublikowana) {#version-0.9.0-unreleased}

-   Dodano przycisk anulowania do okna dialogowego trwającej
    aktualizacji.
-   Dodano narzędzie CLI o nazwie pb, umożliwiające szybką konwersję
    dowolnego z formatów obsługiwanych przez Paperback do HTML, Markdown
    lub zwykłego tekstu.
-   Dodano konfigurowalny skrót klawiaturowy do przywracania programu
    Paperback z paska zadań.
-   Dodano przycisk „Znajdź" w oknie dialogowym „Wszystkie dokumenty",
    umożliwiający zlokalizowanie brakujących książek, których ścieżka
    dostępu uległa zmianie.
-   Dodano zakładkę „Czytelność" do okna dialogowego opcji, zawierającą
    następujące opcje:
    -   Zawijanie tekstu (przeniesione z sekcji „Ogólne");
    -   Renderowanie tabel w tekście (nowość w tej wersji, patrz
        poniżej);
    -   Czcionka;
    -   Kolor tła;
    -   Odstępy między wierszami;
    -   Odstępy między akapitami;
    -   Odstępy między literami;
    -   Wyrównanie tekstu.
-   Dodano przełącznik umożliwiający określenie sposobu wyświetlania
    tabel oraz ujednolicono sposób wyświetlania tabel we wszystkich
    dokumentach.
-   Dodano opcję „Wyświetl źródło", która otwiera kod źródłowy dokumentu
    w nowej karcie, co jest przydatne na przykład podczas edycji
    Markdown.
-   Do okna dialogowego liczenia słów dodano szacowany czas czytania, a
    także możliwość ustawienia własnej prędkości czytania, dzięki czemu
    wskaźnik ten jest rzeczywiście przydatny.
-   Dodano obsługę systemu Windows na architekturze ARM64!
-   Dodano obsługę systemu Android!
-   Dodano obsługę systemu iOS!
-   Dodano obsługę systemu macOS!
-   Dodano nowe języki: holenderski, fiński i polski.
-   Dodano obsługę nawigacji według kontenerów.
-   Dodano obsługę list, pozycji listy, rysunków i obrazów w dokumentach
    CHM .
-   Dodano pozycję menu „Zawijanie tekstu" oraz odpowiadający jej skrót
    klawiszowy.
-   Dźwięki zakładek/notatek powinny teraz prawidłowo odtwarzać się
    wyłącznie podczas przechodzenia kursorem nad słowem, w którym się
    znajdują.
-   Dokumenty zakodowane w starszych kodowaniach CJK, takich jak GBK,
    Big5 i Shift_JIS, będą teraz wyświetlane poprawnie, a nie jako zbiór
    zniekształconych znaków.
-   Rozszerzono pozycję menu eksportu, aby umożliwić eksport do formatu
    HTML i Markdown oprócz zwykłego tekstu.
-   Naprawiono błąd, w wyniku którego zastosowanie zawijania tekstu
    powodowało powrót do początku dokumentu.
-   Naprawiono wyświetlanie nieprawidłowych informacji w pasku stanu w
    przypadku książek w formacie Daisy.
-   Naprawiono błąd, w wyniku którego elementy dl, dt i dd nie
    powodowały przełomów linii w dokumentach XHTML.
-   Naprawiono błąd, w wyniku którego klawisz Escape nie zamykał okien
    dialogowych „Informacje o dokumencie" i „Wszystkie dokumenty".
    Naprawiono błąd, w wyniku którego kotwice filepos w książkach w
    formacie Mobi rozdzielały tagi HTML i umieszczały
-   Naprawiono problem z kotwicami filepos w książkach Mobi, które
    rozdzielały tagi HTML i umieszczały niepotrzebne znaki w tekście
    książki.
-   Naprawiono opóźnienie przy zbliżaniu się do końca pola tekstowego w
    dużych dokumentach.
-   Naprawiono linki w starszych książkach w formacie Mobi.
-   Naprawiono ładowanie książek DAISY z nieprawidłowymi deklaracjami
    kodowania.
-   Naprawiono błąd, w wyniku którego nawigacja po stronach wyświetlała
    nieprawidłowy tekst wiersza w niektórych sytuacjach.
-   Naprawiono parsowanie dokumentów RTF zawierających znaki spoza
    alfabetu łacińskiego.
-   Naprawiono błąd, w wyniku którego opcja „Otwórz ostatnio zamknięty"
    próbowała ponownie otworzyć dołączony plik readme.
-   Naprawiono brak aktualizacji paska tytułu po zamknięciu dokumentu z
    okna dialogowego „Wszystkie dokumenty".
-   Naprawiono problem, w wyniku którego okno dialogowe widoku
    internetowego nie dało się skalować i pojawiało się w bardzo małym
    rozmiarze początkowym.
-   Naprawiono błąd, w wyniku którego dokumenty Worda zawierające nazwy
    stylów specyficzne dla danego ustawienia regionalnego nie
    wyświetlały prawidłowo nagłówków.
-   Naprawiono błąd, w wyniku którego zaznaczona karta nie była
    prawidłowo aktywowana po ponownym uruchomieniu programu Paperback.
-   Jeśli podczas otwierania okna dialogowego liczenia słów zaznaczenie
    jest aktywne, wyświetlana będzie teraz liczba zaznaczonych słów.
-   Obrazy powinny teraz prawidłowo wyświetlać się w osadzonym widoku
    internetowym.
-   Poprawiono obsługę plików na sieciowych dyskach systemu Windows
    przez aplikację Paperback: naciśnięcie opcji „Pokaż plik w folderze"
    powoduje teraz prawidłowe ustawienie fokusu na pliku w magazynie
    sieciowym, a ścieżki nie zawierają już dziwnych znaków.
-   Znacznie poprawiono parsowanie formatu AZW3.
-   Zrezygnowano z biblioteki chmlib na rzecz naszego własnego czytnika
    plików CHM napisanego w czystym języku Rust.
-   Na komputerach stacjonarnych pliki .paperback nie będą już
    przymusowo ładowane podczas przywracania dokumentów. Zamiast tego
    pojawi się prośba o potwierdzenie, gdy plik zostanie znaleziony.
-   Paperback przechodzi teraz na ekstrakcję zwykłego tekstu w przypadku
    błędnie oznaczonych plików PDF.
-   Otwarcie folderu zawierającego plik powoduje teraz wyświetlenie
    danego pliku w Eksploratorze.
-   Otwieranie pliku readme będzie teraz uwzględniało wybrany język.
-   Dokumenty PowerPoint obsługują teraz tabele.
-   Prawidłowo aktualizuj menu i ustaw fokus na polu tekstowym podczas
    otwierania pomocy w programie Paperback.
-   Plik „Readme.html" nie będzie już dodawany do listy wszystkich
    dokumentów po otwarciu za pomocą skrótu Shift+F1.
-   Usunięcie dokumentów z okna dialogowego „Ostatnio używane" powoduje
    teraz również zamknięcie ich aktywnej karty.
-   W systemie Windows zastosowano znacznie bezpieczniejszą metodę
    komunikacji międzyprocesowej (IPC).
-   Tytuł aktywnego dokumentu będzie teraz odczytywany podczas
    przełączania się między kartami.
-   Narzędzie do aktualizacji poprawnie wyświetla teraz zawartość tagów
    kodu Markdown w informacjach o wydaniu.
-   Program aktualizujący sprawdza teraz, czy pobrany plik nie został
    zmodyfikowany.
-   Widok internetowy jest teraz otwierany w miejscu, w którym aktualnie
    się znajdujesz.
-   Filtr wyszukiwania w oknie dialogowym „Wszystkie dokumenty" jest
    teraz zachowywany po usunięciu dokumentu.

### Wersja 0.8.5 {#version-0.8.5}

-   Dodano obsługę stron w książkach w formacie ePub.
-   Dodano obsługę zaszyfrowanych dokumentów Microsoft Office. Obecnie
    obsługiwane są starsze wersje programu Word, nowoczesna wersja
    programu Word oraz nowoczesna wersja programu PowerPoint, a obsługa
    starszych wersji programu PowerPoint jest planowana na przyszłość.
-   Dodano obsługę starszych dokumentów Microsoft Word (\*.doc)!
-   Dodano obsługę starszych prezentacji programu PowerPoint (\*.ppt)!
-   Dodano obsługę książek w formatach mobi i AZW3!
-   Dodano obsługę plików PDF z tagami!
-   Dodano skrót klawiszowy Ctrl+Q do zamykania aplikacji.
-   Dodano obsługę skompresowanych książek z serwisu Bookshare (zarówno
    w formacie DAISY, jak i Word)!
-   Tekst alternatywny dla osadzonych obrazów powinien być teraz
    wyświetlany poprawnie.
-   Dokumenty CHM obsługują teraz poprawnie nawigację za pomocą linków
    wewnętrznych.
-   Naprawiono problem, w wyniku którego dźwięki zakładek uruchamiały
    się na początku akapitu zamiast w miejscu, w którym znajdowała się
    zakładka.
-   Naprawiono błąd, w wyniku którego funkcja „Przejdź do strony"
    wskazywała stronę o jeden numer wyżej.
-   Naprawiono błąd, w wyniku którego klawisz Esc nie zamykał okna
    dialogowego „Otwórz jako".
-   Naprawiono problem, w wyniku którego menu kontekstowe czytnika nie
    pojawiało się po kliknięciu prawym przyciskiem myszy lub naciśnięciu
    klawisza „Aplikacje".
-   Naprawiono błąd, w wyniku którego podczas otwierania dokumentów z
    wiersza poleceń czasami wybierany był niewłaściwy dokument.
-   Pliki PDF zawierające wyłącznie obrazy są ponownie wykrywane i
    wyświetlane są powiadomienia o ich obecności.
-   Możliwe jest teraz poruszanie się po obrazach i rysunkach za pomocą
    odpowiednio klawiszy g/Shift+g oraz f/Shift+f.
-   Aplikacja Paperback będzie teraz uwzględniać ustawienie trybu
    ciemnego w systemie.
-   Usunięto obsługę formatu DAISY XML, ponieważ nie jest już potrzebna.
-   W drzewie spisu treści przywrócono natywną nawigację Win32 opartą na
    pierwszej literze.
-   Okno dialogowe błędu ładowania wyświetla teraz bardziej szczegółowe
    komunikaty o błędach.
-   Widok internetowy otwiera się teraz znacznie szybciej i płynniej.

### Wersja 0.8.2 {#version-0.8.2}

-   Dodano obsługę stron w dokumentach RTF!
-   Naprawiono błąd, w wyniku którego otwieranie widoku internetowego w
    plikach ePub zawierających linki zewnętrzne powodowało ich
    automatyczną aktywację.
-   Naprawiono błąd, w wyniku którego parser RTF w rzadkich przypadkach
    nie wstawiał spacji między słowami .
-   Naprawiono problem z dzielenie akapitów na wiele krótkich wierszy w
    niektórych plikach PDF .
-   Dokumenty PDF mają teraz podstawową obsługę nawigacji po linkach i
    nagłówkach !
-   Tabulatory i znaki końca linii w plikach RTF są teraz renderowane
    dokładnie tak, jak pojawiają się w dokumencie.
-   Powrócono do sprawdzonej biblioteki pdfium do analizowania plików
    PDF, dzięki czemu renderowanie plików PDF znów stało się znacznie
    bardziej niezawodne.

### Wersja 0.8.1 {#version-0.8.1}

-   Dodano skrót klawiszowy Ctrl+Shift+T, aby ponownie otworzyć ostatnio
    zamknięty dokument.
-   Okno dialogowe „Wszystkie dokumenty" obsługuje teraz wybór wielu
    dokumentów do jednoczesnego otwarcia.
-   Naprawiono kilka błędów związanych z parserem RTF.
-   Naprawiono problem z uszkadzaniem się ścieżek plików zawierających
    znaki spoza zestawu ASCII (takie jak bośniackie š, č, ć, ž) podczas
    otwierania pliku za pośrednictwem drugiej instancji programu
    Paperback .
-   Naprawiono błąd powodujący odczytywanie tekstu w pliku PDF w
    niewłaściwej kolejności oraz nieprawidłowe odstępy wokół słów
    pisanych wielką literą.
-   Naprawiono powolne ładowanie dokumentów podczas otwierania dużych
    plików.
-   Naprawiono lokalizację przycisków „Tak"/„Nie" w oknach dialogowych
    potwierdzających.

### Wersja 0.8.0 {#version-0.8.0}

-   Dodano tłumaczenia na język japoński, chiński uproszczony i
    wietnamski !
-   Dodano automatyczną aktualizację, która teraz zastąpi aktualnie
    zainstalowaną wersję Paperback, zamiast jedynie pobierać nową
    wersję!
-   Dodano opcjonalne sygnały dźwiękowe informujące o dotarciu do
    zakładki lub notatki, dziękujemy Andre Louisowi za dźwięki!
-   Dodano obsługę dokumentów RTF!
-   Dodano obsługę dokumentów DAISY XML.
-   Dodano obsługę plików tekstowych Flat Open Document!
-   Dodano obsługę prezentacji w formacie Flat Open Document!
-   Dodano obsługę separatorów za pomocą klawiszy „s" i „Shift+s".
-   Każde przesunięcie o więcej niż 300 znaków będzie teraz
    automatycznie dodawane do historii nawigacji.
-   Naprawiono przywracanie okna Paperback z paska zadań.
-   Naprawiono błąd, w wyniku którego dokumenty Markdown wyświetlały
    surowy tekst zamiast renderowanego kodu HTML w widoku internetowym.
-   Naprawiono nieprawidłowe renderowanie tabel w plikach Markdown.
-   Pliki PDF zawierające wyłącznie obrazy będą teraz wyświetlać
    ostrzeżenie o ich istnieniu podczas próby ich załadowania.
-   Podczas sprawdzania aktualizacji można teraz wybrać opcję
    wyszukiwania nowych kompilacji deweloperskich zamiast stabilnych
    wersji.
-   Prawidłowo osadzono informacje o wersji w pliku wykonywalnym
    Paperback.
-   Podzielono okno dialogowe opcji na zakładki w celu ułatwienia
    obsługi i nawigacji.
-   Przechodzimy na bibliotekę Hayro do analizowania plików PDF, co
    zapewnia większą niezawodność, szybkość oraz mniej bibliotek DLL.
-   Cała aplikacja została przepisana w języku Rust. Nowa baza kodu jest
    bezpieczniejsza, szybciej ładuje dokumenty oraz jest łatwiejsza w
    utrzymaniu i rozbudowie.
-   Menu kontekstowe kontrolki tekstowej będzie teraz zawierało
    działania specyficzne dla danego czytnika zamiast ogólnych pozycji,
    takich jak wycinanie i wklejanie.

### Wersja 0.7.0 {#version-0.7.0}

-   Dodano obsługę tabel w dokumentach opartych na HTML i XHTML!
    Przechodź między tabelami za pomocą klawiszy T i Shift+T, a
    naciśnięcie klawisza Enter powoduje wyświetlenie tabeli w widoku
    internetowym.
-   Dodano podstawową funkcję renderowania stron internetowych! Naciśnij
    Ctrl+Shift+V, aby otworzyć bieżącą sekcję dokumentu w rendererze
    internetowym -- przydatne w przypadku treści takich jak złożone
    formatowanie lub fragmenty kodu.
-   Dodano rosyjskie tłumaczenie -- dziękujemy Ruslanowi
    Gulmagomedovowi!
-   Dodano przycisk „Wyczyść wszystko" w oknie dialogowym „Wszystkie
    dokumenty".
-   Narzędzie sprawdzające aktualizacje wyświetla teraz informacje o
    wydaniu, gdy dostępna jest nowa wersja. Naprawiono
-   Naprawiono przywracanie okna z paska zadań.
-   Naprawiono tłumaczenia przycisków „Tak"/„Nie" w oknach dialogowych
    potwierdzenia.
-   Naprawiono ładowanie konfiguracji podczas uruchamiania z
    uprawnieniami administratora.
-   Naprawiono obsługę komentarzy w dokumentach XML i HTML.
-   Naprawiono parsowanie spisu treści w książkach w formacie Epub 2.
-   Naprawiono nawigację do następnej pozycji o tej samej literze w
    spisie treści.
-   Naprawiono nieprawidłowe ukrywanie okna dialogowego wyszukiwania
    podczas korzystania z przycisków „Dalej" i „Wstecz".
-   Naprawiono błąd, w wyniku którego spisy treści w formacie ePub
    czasami przenosiły użytkownika do niewłaściwej pozycji.
-   Naprawiono różne problemy związane z obsługą spacji w tagach XML,
    HTML i pre .
-   Naprawiono błąd „off-by-one" w nawigacji po linkach.
-   Naprawiono problem z końcowymi spacjami na końcu wierszy w
    niektórych książkach.
-   Naprawiono różne problemy związane z parserem.
-   Pozycje menu związane z zakładkami, a także lista elementów są teraz
    prawidłowo wyłączane, gdy żaden dokument nie jest otwarty.
-   Ulepszono obsługę list w różnych formatach dokumentów.
-   Ulepszono proces tłumaczenia dla współpracowników.
-   Przeprowadzono wiele wewnętrznych refaktoryzacji, przenosząc
    większość logiki biznesowej aplikacji z języka C++ do Rust w celu
    poprawy wydajności i łatwości utrzymania.

### Wersja 0.6.1 {#version-0.6.1}

-   Dodano obsługę plików PDF chronionych hasłem!
-   Dodano bardzo podstawową funkcję przechodzenia do
    poprzedniej/następnej pozycji. Jeśli naciśniesz klawisz Enter na
    linku wewnętrznym i spowoduje to przesunięcie kursora, pozycja ta
    zostanie teraz zapamiętana i będzie można do niej przejść za pomocą
    klawiszy Alt + strzałki w lewo/w prawo.
-   Dodano listę elementów! Obecnie wyświetla ona jedynie drzewo
    wszystkich nagłówków w dokumencie lub listę linków, ale planowane
    jest jej rozszerzenie w przyszłości.
-   Dodano opcję uruchamiania programu Paperback w trybie
    zmaksymalizowanym jako ustawienie domyślne.
-   Naprawiono nieprawidłowe działanie linków w niektórych dokumentach w
    formacie ePub.
-   Naprawiono parsowanie spisów treści w formacie ePub zawierających
    ścieżki względne.
-   Naprawiono błąd, w wyniku którego niektóre dokumenty ePub nie
    wyświetlały tytułu ani autora.
-   Naprawiono błąd, w wyniku którego tytuły niektórych rozdziałów w
    formacie ePub nie wyświetlały się poprawnie w oknie dialogowym spisu
    treści.
-   Naprawiono problem uniemożliwiający użycie klawisza spacji do
    aktywacji przycisków „OK"/„Anuluj" w oknie dialogowym spisu treści.
-   Ulepszono obsługę nagłówków w dokumentach programu Word.
-   Teraz usłyszysz komunikat głosowy, jeśli lista ostatnich dokumentów
    jest pusta podczas próby wywołania okna dialogowego.

### Wersja 0.6.0 {#version-0.6.0}

-   Do okna dialogowego opcji dodano nową opcję wyświetlania menu
    „Przejdź do" w znacznie bardziej zwięzłej formie; opcja ta jest
    domyślnie zaznaczona.
-   Dodano opcję umożliwiającą zawijanie nawigacji według elementów
    strukturalnych.
-   Do menu „Narzędzia" dodano opcję otwierania folderu zawierającego
    dokument, na którym aktualnie znajduje się fokus.
-   Dodano dość prosty, ale bardzo skuteczny system aktualizacji.
-   Dodano podstawową funkcję wyłącznika czasowego, dostępną za pomocą
    skrótu klawiszowego Ctrl+Shift+S.
-   Dodano obsługę analizowania ebooków w formacie FB2!
-   Dodano obsługę analizowania prezentacji OpenDocument!
-   Dodano obsługę analizowania plików tekstowych OpenDocument!
-   Zakładki można teraz tworzyć w celu zaznaczenia całego wiersza lub
    tylko określonego fragmentu tekstu. Jeśli podczas umieszczania
    zakładki nie ma aktywnego zaznaczenia, zachowanie jest takie samo
    jak w wersjach sprzed 0.6 i zaznaczony zostanie cały wiersz. Jeśli
    jednak zaznaczysz fragment tekstu, tylko ten tekst zostanie
    uwzględniony w zakładce.
-   Do zakładek można teraz dołączać opcjonalne notatki tekstowe!
    Przechodź między zakładkami zawierającymi notatki za pomocą klawiszy
    N i Shift+N lub wyświetlaj okno dialogowe zakładek z wszystkimi
    zakładkami, tylko notatkami lub tylko zakładkami bez notatek,
    wybierając je za pomocą określonych skrótów klawiszowych.
-   Zakładki w oknie dialogowym zakładek nie będą już miały irytującego
    przedrostka „zakładka x".
-   Książki w formacie Epub zawierające treści HTML udające XML będą
    teraz obsługiwane poprawnie.
-   Naprawiono ładowanie dużych dokumentów Markdown.
-   Naprawiono błąd, w wyniku którego naciśnięcie spacji w widoku drzewa
    spisu treści aktywowało przycisk OK.
-   Naprawiono obsługę spacji na początku tagów pre zarówno w
    dokumentach HTML, jak i XHTML.
-   Naprawiono błąd, w wyniku którego kontrolka tekstowa czasami nie
    odzyskiwała fokusu po powrocie do okna programu Paperback.
-   Naprawiono błąd, w wyniku którego pole tekstowe w oknie dialogowym
    „Przejdź do procentu" nie aktualizowało wartości suwaka.
-   Naprawiono renderowanie niestandardowych identyfikatorów HTML w
    dokumentach Markdown.
-   Kod HTML wewnątrz bloków kodu Markdown będzie teraz renderowany
    poprawnie.
-   W przypadku ładowania książki z parametrem wiersza poleceń, gdy
    działa już istniejąca instancja programu Paperback, nie pojawi się
    już błąd, jeśli ładowanie dokumentu potrwa dłużej niż 5 sekund.
-   W przypadku uruchamiania Paperback jako administrator konfiguracja
    będzie teraz prawidłowo ładowana i zapisywana.
-   Możliwe jest teraz usuwanie zakładek bezpośrednio z poziomu okna
    dialogowego zakładek.
-   Możliwe jest teraz importowanie i eksportowanie zakładek oraz
    pozycji czytania dla konkretnego dokumentu. Wygenerowany plik nosi
    nazwę zgodną z nazwą pliku z rozszerzeniem .paperback. Jeśli taki
    plik zostanie znaleziony w tym samym katalogu co plik podczas jego
    ładowania, zostanie automatycznie załadowany. W przeciwnym razie
    można je ręcznie zaimportować, korzystając z opcji w menu narzędzi.
-   Linki wewnątrz dokumentów są teraz w pełni obsługiwane! Użyj k i
    shift+k, aby przechodzić między nimi do przodu i do tyłu, a naciśnij
    Enter, aby otworzyć/aktywować jeden z nich.
-   Wprowadzono wiele wewnętrznych zmian, dzięki czemu aplikacja działa
    szybciej, a plik wykonywalny jest mniejszy.
-   Treść w formacie Markdown jest teraz wstępnie przetwarzana w celu
    zapewnienia zgodności ze standardem CommonMark przed renderowaniem.
-   Nawigacja po listach i ich pozycjach jest teraz w pełni obsługiwana!
    Użyj klawiszy L i Shift+L, aby poruszać się po samych listach, a
    klawiszy I i Shift+I, aby poruszać się po pozycjach na liście.
-   Klawisz Delete na klawiaturze numerycznej służy teraz do usuwania
    dokumentów z paska kart, oprócz zwykłego usuwania.
-   Paperback może teraz opcjonalnie zminimalizować się do paska zadań!
    Ta opcja jest domyślnie wyłączona, ale jej włączenie sprawi, że
    opcja minimalizacji w menu systemowym umieści Paperback na pasku
    zadań, skąd można go przywrócić, klikając wygenerowaną ikonę.
-   Paperback można teraz w pełni przetłumaczyć! Lista obsługiwanych
    języków jest obecnie dość krótka, ale stale się powiększa!
-   Paperback ma teraz oficjalną stronę internetową pod adresem
    [paperback.dev](https://paperback.dev)!
-   Dokumenty PPTX będą teraz wyświetlać podstawowy spis treści
    zawierający wszystkie slajdy.
-   Pełna ścieżka do otwartego dokumentu będzie teraz wyświetlana w
    oknie dialogowym informacji o dokumencie.
-   Instalator zawiera teraz opcję wyświetlenia pliku readme w
    przeglądarce po zakończeniu instalacji.
-   Lista ostatnio otwieranych dokumentów została znacznie rozszerzona!
    Zamiast wyświetlać po prostu 10 ostatnio otwartych dokumentów,
    pokazuje teraz konfigurowalną liczbę, a pozostałe dokumenty, które
    kiedykolwiek otworzyłeś, są dostępne za pośrednictwem małego okna
    dialogowego.
-   Wprowadzono różne drobne ulepszenia w parserach, w tym wstawianie
    pustej linii między slajdami w prezentacjach PPTX, poprawkę obsługi
    znaków nowej linii wewnątrz akapitów w dokumentach Worda oraz
    dodanie punktorów do elementów listy.

### Wersja 0.5.0 {#version-0.5.0}

-   Dodano obsługę dokumentów Microsoft Word!
-   Dodano obsługę prezentacji PowerPoint!
-   Naprawiono błąd, w wyniku którego niektóre pozycje menu nie były
    wyłączane, gdy nie było otwartych dokumentów.
-   Naprawiono orientację suwaka „Przejdź do procentu".
-   Naprawiono spis treści w książkach w formacie ePub zawierających
    ścieżki do plików zakodowane jako adresy URL i/lub identyfikatory
    fragmentów.
-   Naprawiono nieprawidłowe usuwanie spacji z nagłówków XHTML w dziwny
    sposób.
-   Naprawiono obsługę spacji wewnątrz zagnieżdżonych tagów \`pre\` w
    dokumentach HTML. Dokumenty
-   Dokumenty HTML i Markdown obsługują teraz funkcję spisu treści ! Po
    załadowaniu dokumentu HTML/Markdown aplikacja Paperback utworzy
    własny spis treści na podstawie struktury nagłówków w dokumencie i
    wyświetli go w oknie dialogowym wywołanym skrótem Ctrl+T.
-   Dokumenty HTML będą teraz miały tytuł ustawiony w tagu title, jeśli
    taki istnieje. W przeciwnym razie nadal będą używać nazwy pliku bez
    rozszerzenia.
-   Przełączono się z biblioteki UniversalSpeech na wykorzystanie
    regionu na żywo do generowania mowy. Oznacza to, że wraz z programem
    nie są już dostarczane biblioteki DLL czytników ekranu, a
    obsługiwanych będzie teraz więcej czytników ekranu, takich jak
    Microsoft Narrator.
-   Zmieniono biblioteki ZIP, aby umożliwić otwieranie szerszej gamy
    książek w formacie EPUB.
-   Okno dialogowe z pytaniem, czy chcesz otworzyć dokument jako zwykły
    tekst, zostało całkowicie przerobione i pozwala teraz na otwarcie
    dokumentu jako zwykłego tekstu, HTML lub Markdown.
-   Okno dialogowe „Przejdź do procentu" zawiera teraz pole tekstowe, w
    którym można ręcznie wprowadzić wartość procentową, do której ma
    nastąpić przejście.
-   Parser HTML rozpoznaje teraz elementy dd, dt i dl jako elementy
    listy.
-   Spis treści w książkach w formacie ePub będzie ponownie zachowywany
    dokładnie.
-   Podczas usuwania pustych wierszy brana jest teraz pod uwagę spacja
    nierozdzielająca Unicode.
-   Użytkownik nie będzie już pytany o sposób otwarcia nierozpoznanego
    pliku przy każdym jego załadowaniu, a jedynie przy pierwszym
    uruchomieniu.

### Wersja 0.4.1 {#version-0.4.1}

-   Do instalatora dodano opcjonalną ikonę w menu Start.
-   Spis treści powinien być teraz bardziej przejrzysty w kilku
    przypadkach, na przykład jeśli element nadrzędny i podrzędny mają
    ten sam tekst w tej samej pozycji, widoczny będzie teraz tylko
    element nadrzędny.
-   Naprawiono spis treści w niektórych dokumentach CHM.
-   Naprawiono spis treści w książkach w formacie Epub 3 zawierających
    ścieżki bezwzględne .
-   Dokumenty CHM powinny teraz wyświetlać tytuł zgodny z tym, który
    został ustawiony w pliku metadanych .

### Wersja 0.4.0 {#version-0.4.0}

-   Dodano obsługę plików CHM!
-   Dodano obsługę zakładek! Możesz mieć dowolną liczbę zakładek w
    dowolnej liczbie dokumentów. Możesz przechodzić między nimi do
    przodu i do tyłu za pomocą klawiszy b i Shift+b, utworzyć zakładkę
    za pomocą Control+Shift+b oraz wyświetlić okno dialogowe, aby
    przejść do konkretnej zakładki za pomocą Control+b.
-   Oprócz przenośnego pliku ZIP dodano instalator! Instalator
    zainstaluje program Paperback w katalogu Program Files i
    automatycznie skonfiguruje skojarzenia plików.
-   Pliki tekstowe z BOM powinny być teraz poprawnie dekodowane, a BOM
    nie będzie już wyświetlany na początku tekstu.
-   Do paska stanu dodano znacznie więcej informacji. Teraz wyświetla on
    bieżącą linię, znak oraz procent przeczytania.
-   Komentarze HTML, a także zawartość tagów skryptowych i stylowych,
    nie będą już wyświetlane w tekście wyjściowym.
-   W przypadku przekazania ścieżki względnej do programu Paperback w
    wierszu poleceń, zostanie ona teraz poprawnie rozpoznana.
-   Przesuwanie o określony procent jest teraz obsługiwane przez osobne
    okno dialogowe z suwakiem, dostępne za pomocą skrótu klawiszowego
    Ctrl+Shift+G.
-   Dokumenty bez znanych tytułów lub autorów będą teraz zawsze miały
    wartość domyślną.
-   Logika zapisywania pozycji jest teraz znacznie bardziej inteligentna
    i powinna zapisywać dane na dysku tylko wtedy, gdy jest to
    absolutnie konieczne.
-   Dokument, który był aktywny w momencie zamknięcia programu
    Paperback, jest teraz zapamiętywany po ponownym uruchomieniu
    aplikacji.
-   Dane wprowadzane w oknach dialogowych „Przejdź do wiersza" i
    „Przejdź do strony" powinny być teraz poddawane bardziej
    rygorystycznej weryfikacji.
-   Naprawiono nawigację po spisie treści w książkach w formacie ePub 3
    zawierających względne ścieżki w swoich manifestach.

### Wersja 0.3.0 {#version-0.3.0}

-   Naprawiono spis treści w książkach w formacie ePub z manifestami z
    kodowaniem URL.
-   Naprawiono nawigację po nagłówkach w dokumentach HTML zawierających
    wielobajtowe znaki Unicode.
-   Naprawiono wysokie zużycie procesora w dokumentach z długimi
    tytułami spowodowane regresją w bibliotece wxWidgets.
-   Naprawiono ładowanie plików tekstowych w formacie UTF-8.
-   Naprawiono problem z zagnieżdżonymi pozycjami spisu treści w
    książkach w formacie ePub, które powodowały umieszczenie kursora w
    niewłaściwym miejscu.
-   Naprawiono awarię aplikacji podczas jej zamykania w niektórych
    przypadkach.
-   Dodano pole wyboru w oknie dialogowym opcji, umożliwiające włączenie
    lub wyłączenie zawijania tekstu!
-   Teraz można wesprzeć finansowo rozwój programu Paperback, albo
    poprzez nową opcję „wesprzyj" w menu pomocy, albo poprzez link
    „wesprzyj ten projekt" znajdujący się na dole strony głównej
    repozytorium GitHub.
-   Dokumenty Markdown będą teraz zawsze miały tytuł, a Paperback
    powinien być w stanie załadować praktycznie każdy plik Markdown.
-   Dokumenty PDF będą teraz zawsze miały tytuł, nawet jeśli brakuje
    metadanych.
-   Przełączono biblioteki PDF na te używane w Chromium, co zapewnia
    znacznie bardziej niezawodne przetwarzanie plików PDF we wszystkich
    przypadkach.
-   Teraz w danym momencie może być uruchomiona tylko jedna instancja
    programu Paperback. Uruchomienie pliku paperback.exe z nazwą pliku,
    gdy program jest już uruchomiony, spowoduje otwarcie tego dokumentu
    w już uruchomionej instancji.
-   Teraz można nacisnąć klawisz Delete na dokumencie w panelu kart, aby
    go zamknąć.

### Wersja 0.2.1 {#version-0.2.1}

-   Dodano całkowitą liczbę stron do etykiety strony w oknie dialogowym
    „Przejdź do strony". Dodano
-   Umożliwiono przechodzenie za pomocą klawisza Tab z treści dokumentu
    do listy otwartych dokumentów.
-   Naprawiono błąd, w wyniku którego naciśnięcie klawiszy nagłówkowych
    czasami otwierało ostatnio używane dokumenty, jeśli było ich
    wystarczająco dużo.
-   Paperback będzie teraz usuwać zbędne łuki miękkie z generowanego
    tekstu.
-   Naprawiono błąd, w wyniku którego nawigacja po nagłówkach czasami
    przenosiła użytkownika do niewłaściwego znaku.

### Wersja 0.2.0 {#version-0.2.0}

-   Dodano obsługę dokumentów w formacie Markdown!
-   Dodano obsługę dokumentów PDF, w tym możliwość nawigacji między
    stronami!
-   Dodano skróty klawiszowe do nawigacji po nagłówkach w treści HTML, w
    tym w książkach w formacie epub i dokumentach Markdown. Skróty te
    zostały zaprojektowane tak, aby działały podobnie jak czytniki
    ekranu.
-   Naprawiono ładowanie plików ePub z nazwami plików zakodowanymi w
    formacie URL w ich manifestach.
-   Naprawiono ładowanie książek w formacie epub 3 z osadzonym kodem
    XHTML.
-   Teraz odtwarzany jest komunikat, jeśli dokument nie obsługuje spisu
    treści lub sekcji, zamiast wyłączania odpowiednich pozycji menu.
-   Dodano menu ostatnich dokumentów! Obecnie przechowuje ono 10
    ostatnio otwartych dokumentów, a naciśnięcie klawisza Enter na
    jednym z nich spowoduje jego otwarcie w celu czytania.
-   Całkowicie przeprojektowano okno dialogowe „Znajdź", dzięki czemu
    jest ono znacznie prostsze w obsłudze, a jednocześnie dodano
    historię ostatnich 25 wyszukiwań oraz obsługę wyrażeń regularnych!
-   Wcześniej otwarte dokumenty są teraz zapamiętywane nawet po ponownym
    uruchomieniu aplikacji. Można to skonfigurować za pomocą nowej
    pozycji opcji w menu „Narzędzia".
-   Dodano skrót Shift+F1, który otwiera plik readme bezpośrednio w
    programie Paperback.

### Wersja 0.1.0 {#version-0.1.0}

-   Pierwsze wydanie.
