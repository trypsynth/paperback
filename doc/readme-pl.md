# Paperback - wersja 0.9.1

## Wprowadzenie

Paperback to lekki, szybki i dostępny czytnik e-booków i dokumentów dla wszystkich: od osób czytających okazjonalnie po zaawansowanych użytkowników. Został zaprojektowany z myślą o dostępności dla czytników ekranu, dużej szybkości działania i wygodzie bez zbędnych dodatków.

## Wymagania systemowe

Paperback działa obecnie w systemach Windows 10/11 oraz we wszystkich nowoczesnych wersjach macOS na procesorach ARM. Trwają intensywne prace nad natywnymi aplikacjami dla systemów iOS i Android. Publiczne wersje testowe są planowane niedługo po wydaniu wersji 0.9.0 na komputery. Nastąpi to przed wspólnym wydaniem 1.0, które obejmie wszystkie cztery platformy.

## Funkcje

* Całkowicie samodzielna aplikacja, która nie wymaga instalowania dodatkowego oprogramowania na komputerze, aby zacząć czytać.
* Bardzo szybkie działanie, nawet na starszym sprzęcie.
* Prosty interfejs z kartami, który pozwala otworzyć obok siebie tyle dokumentów, ile chcesz.
* Zapisuje dokładną pozycję czytania w każdym otwieranym dokumencie.
* Opcjonalnie zapamiętuje dokumenty otwarte przy zamknięciu programu i przywraca je przy następnym uruchomieniu.
* Zawiera funkcje nawigacji podobne do tych znanych z trybu przeglądania stron internetowych w wielu czytnikach ekranu, aby szybko i łatwo poruszać się po dokumentach.
* Zawiera rozbudowany dialog Znajdź, między innymi z historią i obsługą wyrażeń regularnych.
* Może działać w pełni przenośnie albo zostać zainstalowany z automatycznym skojarzeniem typów plików.
* Obsługuje ogromną liczbę popularnych formatów plików.

## Zgodność z czytnikami ekranu

Paperback dobrze współpracuje ze wszystkimi głównymi czytnikami ekranu. Istnieje jednak jeden znany problem dotyczący użytkowników JAWS.

### JAWS i linijki brajlowskie

Jeśli używasz JAWS z linijką brajlowską, możesz zauważyć, że długie akapity są ucinane przy przesuwaniu w przód klawiszami nawigacji linijki. Dotyczy to również polecenia odczytu bieżącego akapitu. To błąd w obsłudze kontrolki tekstowej RICHEDIT50W po stronie JAWS, a nie w samym Paperbacku. Na rozwiązanie trzeba było długo czekać, przy znanej opieszałości firmy Vispero w odpowiadaniu na zgłoszenia dotyczące otwartego oprogramowania.

Obejście, które po miesiącach oczekiwania wyszło w końcu na grupie dyskusyjnej JAWS, polega na edycji pliku `paperback.jcf` i ustawieniu opcji „Braille Presentation and Panning” na „Always use DOM if available”. Warto też włączyć „Pan Text by Paragraph”, bo inaczej linijka pozostanie na aktywnym akapicie, zamiast przesuwać się dalej. Przy obu ustawieniach przesuwanie linijki powinno działać poprawnie.

Nazwy tych opcji podano po angielsku, bo w takiej postaci występują w pliku konfiguracyjnym. Polska wersja JAWS ma odpowiadające im etykiety przetłumaczone, więc w okienku ustawień będą brzmiały inaczej.

## Aktualnie obsługiwane typy plików

Paperback obsługuje następujące formaty i rozszerzenia:

* Pliki pomocy CHM (`.chm`)
* Książki DAISY (`.opf`, `.zip`)
* Książki EPUB (`.epub`)
* E-booki FB2 (`.fb2`)
* Dokumenty HTML (`.htm`, `.html`, `.xhtml`)
* Dokumenty Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Dokumenty Microsoft Word (`.docx`, `.docm`, `.doc`)
* Książki MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Prezentacje OpenDocument (`.odp`, `.fodp`)
* Pliki tekstowe OpenDocument (`.odt`, `.fodt`)
* Dokumenty PDF (`.pdf`)
* Prezentacje PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Dokumenty RTF (`.rtf`)
* Pliki zwykłego tekstu i dzienników (`.txt`, `.log`)

## Skróty klawiszowe

Paperback został zaprojektowany przede wszystkim do pracy z klawiaturą. Poniżej znajdują się aktualne skróty.

Podane skróty dotyczą systemu Windows. Tam, gdzie macOS używa innych, odpowiednik podano w nawiasie. Wynika to głównie z tego, że skróty Ctrl+G, Ctrl+W oraz Alt+Strzałka w lewo i w prawo są w tym systemie zajęte przez inne konwencje systemowe lub aplikacyjne.

### Menu Plik

* `Ctrl+O`: Otwórz dokument.
* `Ctrl+F4` (macOS: `Cmd+W`): Zamknij bieżący dokument.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Zamknij wszystkie otwarte dokumenty.
* `Ctrl+Shift+T`: Otwórz ponownie ostatnio zamknięty dokument.
* `Ctrl+R`: Pokaż dialog Wszystkie dokumenty (z menu Ostatnie dokumenty).
* `Ctrl+Q`: Zakończ (tylko Windows; w systemie macOS znajduje się w menu aplikacji).

### Menu Przejdź

* `Ctrl+F`: Pokaż dialog Znajdź.
* `F3` (macOS: `Cmd+G`): Znajdź następne.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Znajdź poprzednie.
* `Ctrl+G` (macOS: `Cmd+L`): Przejdź do wiersza.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Przejdź do procentu.
* `Ctrl+P`: Przejdź do strony (gdy jest obsługiwana przez bieżący dokument).
* `=`: Odczytaj bieżącą pozycję procentową w dokumencie.
* `Alt+Strzałka w lewo` (macOS: `Cmd+[`): Przejdź wstecz w historii nawigacji.
* `Alt+Strzałka w prawo` (macOS: `Cmd+]`): Przejdź do przodu w historii nawigacji.
* `[`: Poprzednia sekcja.
* `]`: Następna sekcja.
* `Shift+H`: Poprzedni nagłówek.
* `H`: Następny nagłówek.
* `Shift+1` do `Shift+6`: Poprzedni nagłówek poziomu 1-6.
* `1` do `6`: Następny nagłówek poziomu 1-6.
* `Shift+P`: Poprzednia strona.
* `P`: Następna strona.
* `Shift+B`: Poprzednia zakładka.
* `B`: Następna zakładka.
* `/`: Ustaw zakładkę tymczasową.
* `\`: Przejdź do zakładki tymczasowej.
* `Shift+N`: Poprzednia notatka.
* `N`: Następna notatka.
* `Ctrl+B`: Przejdź do wszystkich zakładek i notatek.
* `Ctrl+Alt+B`: Przejdź tylko do zakładek.
* `Ctrl+Alt+M`: Przejdź tylko do notatek.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, czyli fizyczny klawisz Control, a nie Cmd): Wyświetl tekst notatki w bieżącej pozycji.
* `Shift+K`: Poprzedni odnośnik.
* `K`: Następny odnośnik.
* `Shift+G`: Poprzedni obraz.
* `G`: Następny obraz.
* `Shift+F`: Poprzedni rysunek.
* `F`: Następny rysunek.
* `Shift+T`: Poprzednia tabela.
* `T`: Następna tabela.
* `Shift+S`: Poprzedni separator.
* `S`: Następny separator.
* `Shift+L`: Poprzednia lista.
* `L`: Następna lista.
* `Shift+I`: Poprzedni element listy.
* `I`: Następny element listy.
* `Shift+,`: Przejdź na początek bieżącego kontenera (listy lub tabeli).
* `,`: Przejdź poza koniec bieżącego kontenera (listy lub tabeli).

### Menu Narzędzia

* `Ctrl+W` (macOS: `RawCtrl+W`, czyli fizyczny klawisz Control, a nie Cmd): Pokaż liczbę słów w bieżącym dokumencie.
* `Ctrl+I`: Pokaż informacje o dokumencie.
* `Ctrl+T`: Pokaż Spis treści.
* `F7`: Pokaż Listę elementów.
* `Ctrl+Shift+C`: Otwórz folder zawierający.
* `Ctrl+Shift+V`: Otwórz bieżącą treść w Widoku WWW.
* `Ctrl+U`: Wyświetl źródło dokumentu w nowej karcie.
* `Ctrl+Shift+E`: Eksportuj dane dokumentu (`.paperback`).
* `Ctrl+Shift+I`: Importuj dane dokumentu (`.paperback`).
* `Ctrl+E`: Eksportuj bieżący dokument do zwykłego tekstu.
* `Ctrl+Shift+B`: Przełącz zakładkę przy bieżącym zaznaczeniu lub kursorze.
* `Ctrl+Shift+N`: Dodaj lub edytuj notatkę do zakładki przy bieżącym zaznaczeniu lub kursorze.
* `Ctrl+Alt+W`: Przełącz zawijanie wierszy.
* `Ctrl+Spacja`: Odtwórz lub wstrzymaj narrację dźwiękową.
* `'`: Przewiń narrację dźwiękową w przód.
* `;`: Przewiń narrację dźwiękową w tył.
* `Ctrl+'`: Zwiększ skok przewijania dźwięku.
* `Ctrl+;`: Zmniejsz skok przewijania dźwięku.
* `F11` (macOS: `RawCtrl+Ctrl+F`, czyli Control+Command+F): Przełącz tryb pełnoekranowy.
* `Ctrl+,`: Otwórz Opcje (w systemie macOS: Preferencje, w menu aplikacji).
* `Ctrl+Shift+S`: Przełącz Wyłącznik czasowy.

### Menu Pomoc

* `Ctrl+F1`: Pokaż dialog O programie Paperback.
* `F1`: Wyświetl pomoc w domyślnej przeglądarce.
* `Shift+F1`: Wyświetl pomoc w Paperbacku.
* `Ctrl+Shift+U`: Sprawdź aktualizacje.
* `Ctrl+D`: Otwórz stronę wsparcia w domyślnej przeglądarce.

### Dodatkowe klawisze w widoku dokumentu

* `Delete` / `Delete na klawiaturze numerycznej` na kontrolce kart: zamknij kartę wybranego dokumentu.
* `Enter` albo `Spacja` w tekście dokumentu: aktywuj odnośnik pod kursorem albo otwórz Widok tabeli, jeśli kursor znajduje się na znaczniku tabeli.
* `Shift+F10` albo klawisz Menu/Aplikacje w tekście dokumentu: otwórz menu kontekstowe.

## Obsługiwane języki

Paperback jest tłumaczony na wiele języków, a kolejne są stale dodawane. Pełna lista znajduje się poniżej.

Aby dowiedzieć się, jak pomóc w tłumaczeniu, przeczytaj [Przewodnik po tłumaczeniach](translating.md).

* Bośniacki
* Chiński uproszczony
* Czeski
* Fiński
* Francuski
* Hiszpański
* Holenderski
* Japoński
* Niemiecki
* Polski
* Portugalski (Brazylia)
* Rosyjski
* Serbski
* Wietnamski

## Podziękowania
### Tworzenie programu
* Quin Gillespie: główny programista i założyciel projektu.
* Aryan Choudhary: główny współtwórca.

### Darowizny
Poniższe osoby przekazały jakąkolwiek darowiznę na rozwój Paperbacka. Jeśli przekażesz darowiznę, Twoje imię i nazwisko nie zostanie tu dodane automatycznie. Dodaję tylko osoby, które chcą, aby ich wsparcie było publiczne.

Uwaga: publiczne sponsorowanie w GitHub traktuję jako podstawę do automatycznego dodania do tej listy.

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

## Lista zmian

### Wersja 0.9.2
* Audiobooki nie każą już czytnikowi ekranu odczytywać ciągu spacji, kiedy przejdziesz do pola tekstowego.
* Audiobooki podają teraz nazwę pliku, kiedy przechodzisz między nimi po sekcjach.
* Audiobooki pokazują teraz swoją prawdziwą długość, zamiast twierdzić, że każdy plik trwa 24 godziny.
* Zamknięcie widoku WWW klawiszem Escape nie wyświetla już komunikatu diagnostycznego, kiedy wcześniej otworzyłeś w nim odsyłacz.
* Kopiowanie po zaznaczeniu wszystkiego daje teraz cały dokument, a nie tylko tę jego część, która jest właśnie wczytana.
* Wyszukiwanie przenosi teraz od razu do znalezionego wiersza, bez wysłuchiwania, jak czytnik ekranu ponownie odczytuje całe okno przy powrocie do książki.
* Naprawiono otwieranie plików EPUB z pozostawionym blokiem ZIP64, które kończyło się komunikatem „Invalid local file header".
* Naprawiono wracanie długich dokumentów na początek, kiedy czytnik ekranu czytał je ciągiem.
* Odsyłacze w widoku WWW prowadzą teraz do wskazanej sekcji, zamiast kończyć się komunikatem „Nie znaleziono pliku".
* Automatyczny komunikat o przeładowaniu dokumentu nie przerywa już czytnikowi ekranu w połowie zdania, a czeka, aż skończy wypowiedź.
* Na karcie Ogólne w oknie ustawień tabulator przechodzi teraz przez opcje w kolejności, w jakiej są widoczne na ekranie, a kanał aktualizacji następuje bezpośrednio po opcji sprawdzania aktualizacji.
* Windows pokazuje teraz zawsze „Paperback" w menu Otwórz za pomocą, a nie pełne hasło programu.
* Licznik słów oraz informacje o dokumencie pokazują teraz, ile plików zawiera audiobook i jak długo trwa w całości.

### Wersja 0.9.1
* Dźwięki zakładek i notatek odtwarzają się teraz w systemie macOS.
* Książki DAISY odtwarzają teraz dźwięk w systemie macOS, zamiast otwierać się i odmierzać czas w ciszy.
* Naprawiono znikanie z dokumentów RTF cudzysłowów drukarskich, myślników i podobnych znaków, które przy okazji zlepiały ze sobą otaczające wyrazy.
* Naprawiono wyciekanie surowych danych obrazów RTF do treści dokumentu w postaci nieczytelnego tekstu.
* Naprawiono pozostawanie nieaktualnych pozycji w podmenu Ostatnie dokumenty, dopóki coś innego nie zbudowało go od nowa.
* Litery dostępu wróciły do wszystkich tłumaczeń, więc rosyjskie menu znów mają dostęp z klawiatury.
* Duże dokumenty CHM otwierają się teraz do siedmiu razy szybciej.
* Otwierane dokumenty są teraz zgłaszane systemowi Windows, więc pojawiają się na liście szybkiego dostępu na pasku zadań i wśród ostatnich elementów w menu Start.
* Pozycja Opcje została przemianowana na Ustawienia, spójnie z aplikacjami mobilnymi, a w systemie macOS zgodnie z konwencją platformy.
* Paperback pamięta teraz między uruchomieniami położenie i rozmiar okna oraz to, czy było zmaksymalizowane.
* Formy mnogie są teraz tłumaczone, więc komunikaty podające liczby czytają się poprawnie w językach, które potrzebują więcej niż jednej formy.
* Wybranie pliku ncc.html książki DAISY otwiera teraz cały audiobook, a nie tylko jego tekst.
* Nazwy akcji w dialogu Dostosuj skróty klawiszowe można teraz tłumaczyć.
* Tytuł dokumentu jest teraz pierwszy na pasku tytułu, więc otwarte książki da się rozróżnić na pasku zadań i w oknie Alt+Tab.
* Dialog aktualizacji jest teraz przetłumaczony.

### Wersja 0.9.0

#### Dodano

##### Ogólne
* Narzędzie wiersza poleceń o nazwie pb, które szybko przekształca dowolny format obsługiwany przez Paperbacka na HTML, Markdown albo zwykły tekst.
* Opcję ponownego wczytywania dokumentów zmienionych na dysku przez inne programy.
* Opcję Wyświetl źródło, która otwiera źródło dokumentu w nowej karcie. Jest przydatna na przykład przy edytowaniu plików Markdown.
* Tekst dokumentu jest teraz dzielony na strony, dzięki czemu książki liczące dziesiątki milionów słów wczytują się w kilka sekund. Prosimy o zgłaszanie wszelkich nieprawidłowości związanych z tą zmianą.

##### Obsługa platform
* Obsługę systemu Windows na procesorach ARM64!
* Natywną obsługę systemu macOS!
* Przełącznik trybu pełnoekranowego.

##### Dialog Wszystkie dokumenty
* Przycisk Zlokalizuj, który pozwala znaleźć brakujące książki po zmianie ich ścieżki.
* Filtr statusu i pasek statusu, dzięki którym można filtrować dokumenty według statusu oraz sprawdzić, ile dokumentów jest wyświetlanych i zaznaczonych.
* Skrót `Ctrl+Shift+A`, który odznacza wszystkie dokumenty.

##### Opcje i czytelność
* Kartę czytelności z następującymi opcjami:
    * zawijanie wierszy (przeniesione z karty ogólnej),
    * wyświetlanie tabel w treści (nowość w tym wydaniu, opis poniżej),
    * czcionka,
    * kolor tła,
    * odstęp między wierszami,
    * odstęp między akapitami,
    * odstęp między literami,
    * wyrównanie tekstu.
* Pozycję menu dla zawijania wierszy oraz przypisany do niej skrót klawiszowy.
* Przełącznik sposobu wyświetlania tabel oraz ujednolicony wygląd tabel we wszystkich dokumentach.

##### Nawigacja
* Obsługę nawigacji po kontenerach.
* Opcję automatycznego przenoszenia kursora na początek wiersza przy przechodzeniu między wierszami, podobnie jak w trybie przeglądania w czytnikach ekranu.
* Skrót klawiszowy ze znakiem równości, który odczytuje bieżącą pozycję procentową w dokumencie.

##### Zakładki
* Zakładki tymczasowe: można mieć jedną na dokument i zostaje ona zapamiętana między uruchomieniami. Ukośnik ustawia zakładkę, a ukośnik odwrotny przenosi do niej.

##### Liczba słów
* Szacowany czas czytania w dialogu liczby słów oraz możliwość ustawienia własnej szybkości czytania, dzięki której ta informacja staje się naprawdę użyteczna.
* Gdy przy otwieraniu dialogu liczby słów aktywne jest zaznaczenie, pokazywana jest też liczba zaznaczonych słów.

##### Skróty klawiszowe
* Możliwość dostosowania każdego skrótu klawiszowego w aplikacji za pomocą prostego dialogu.
* Konfigurowalny skrót klawiszowy przywracający Paperbacka z zasobnika systemowego.

##### Języki
* Holenderski, fiński i polski.

##### Eksport
* Rozszerzono pozycję menu eksportu, aby umożliwić eksport do HTML i Markdown, obok zwykłego tekstu.

##### Aktualizator
* Przycisk anulowania w dialogu trwającej aktualizacji.
* Aktualizator sprawdza teraz, czy w pobrany plik nikt nie ingerował.

##### Widok WWW
* Widok WWW otwiera się teraz na bieżącej pozycji czytania.

##### Książki DAISY
* Obsługę książek DAISY 2.0.
* Obsługę odtwarzania dźwięku w formacie DAISY 2.02.

##### Audiobooki
* Możliwość odtwarzania audiobooków. Obsługiwane są audiobooki DAISY, również z tekstem, oraz archiwa ZIP z plikami dźwiękowymi.
* Skróty klawiszowe i pozycje menu do odtwarzania i wstrzymywania narracji, przewijania w przód i w tył oraz zmiany skoku przewijania.
* Opcje synchronizacji kursora czytania z odtwarzanym dźwiękiem, ustawienia skoku przewijania oraz wyboru, czy przewijanie poza koniec rozdziału przenosi do następnego.

##### Dokumenty CHM
* Obsługę list, elementów list, rysunków i obrazów.

##### PowerPoint
* Dokumenty PowerPoint obsługują teraz tabele.

#### Naprawiono

##### Ogólne
* Dokumenty zapisane w starszych kodowaniach chińskich, japońskich i koreańskich, takich jak GBK, Big5 i Shift_JIS, wyświetlają się teraz poprawnie, a nie jako ciąg nieczytelnych znaków.
* Polecenie „Otwórz ponownie ostatnio zamknięty”, które próbowało otwierać dołączony plik pomocy.
* Wybrana karta nie otrzymywała poprawnie fokusa po ponownym uruchomieniu Paperbacka.
* Obsługę plików na dyskach sieciowych Windows: polecenie pokazania pliku w folderze prawidłowo wskazuje teraz plik na dysku sieciowym, a ścieżki nie zawierają już dziwnych znaków.
* Pliki .paperback nie są już wczytywane samoczynnie przy przywracaniu dokumentów. Zamiast tego po znalezieniu takiego pliku pojawia się pytanie o potwierdzenie.
* Polecenie otwarcia folderu zawierającego wskazuje teraz dany plik w Eksploratorze.
* Otwarcie pliku pomocy uwzględnia teraz wybrany język.
* Interfejs Paperbacka skaluje się teraz poprawnie na ekranach o dużej gęstości pikseli.
* Menu aktualizuje się teraz prawidłowo, a fokus przenosi się na kontrolkę tekstu przy otwieraniu pomocy w Paperbacku.
* Zastosowano znacznie bezpieczniejszą metodę komunikacji między procesami w systemie Windows.
* Tytuł aktywnego dokumentu jest teraz odczytywany przy przechodzeniu między kartami.
* Zmniejszono zużycie pamięci przy dużych dokumentach przez zmniejszenie o połowę wewnętrznych tablic indeksu znaków.

##### Dialog Wszystkie dokumenty
* Klawisz Escape nie zamykał dialogów Informacje o dokumencie i Wszystkie dokumenty.
* Pasek tytułu nie aktualizował się po zamknięciu dokumentu z dialogu Wszystkie dokumenty.
* Plik readme.html nie będzie już dodawany do listy wszystkich dokumentów po otwarciu skrótem Shift+F1.
* Usunięcie dokumentu z dialogu ostatnich dokumentów zamyka teraz również jego aktywną kartę.
* Filtr wyszukiwania jest teraz zachowywany po usunięciu dokumentu.

##### Nawigacja
* Odczytywanie nieprawidłowego tekstu wiersza przy nawigacji po stronach w niektórych sytuacjach.
* Ustawianie kursora w niewłaściwym miejscu przez polecenia Przejdź do wiersza, Przejdź do strony i Przejdź do procentu w dużych dokumentach.
* Nieuwzględnianie wczytanego fragmentu dokumentu przez polecenia Znajdź i Znajdź następne w dużych dokumentach.

##### Zakładki
* Dźwięki zakładek i notatek odtwarzają się teraz wyłącznie przy przejściu przez słowo, które je zawiera.

##### Czytelność
* Przeskok na początek dokumentu przy włączaniu zawijania wierszy.

##### Widok WWW
* Okna Widoku WWW nie dało się rozciągnąć, a otwierało się w bardzo małym rozmiarze.
* Obrazy wyświetlają się teraz poprawnie w osadzonym Widoku WWW.

##### Aktualizator
* Aktualizator pokazuje teraz poprawnie treść znaczników kodu w informacjach o wydaniu.

##### Książki DAISY
* Nieprawidłowe informacje na pasku statusu przy książkach DAISY.
* Wczytywanie książek DAISY z błędnymi deklaracjami kodowania.

##### Dokumenty RTF
* Przetwarzanie dokumentów RTF ze znakami spoza alfabetu łacińskiego.
* Grupy `\pict` w RTF, dzięki czemu dane osadzonych obrazów nie trafiają już do tekstu dokumentu.

##### Książki Mobi/AZW3
* Kotwice pozycji w pliku (filepos) w książkach Mobi, które rozrywały znaczniki HTML i wstawiały śmieci do tekstu książki.
* Odnośniki w starszych książkach Mobi.
* Znacznie ulepszono przetwarzanie plików AZW3.

##### Dokumenty Word
* Dokumenty Word z nazwami stylów zależnymi od języka, w których nagłówki nie wyświetlały się poprawnie.

##### Dokumenty HTML/XHTML
* Elementy HTML o nazwach dl, dt i dd, które nie powodowały podziału wiersza w dokumentach XHTML.

##### Dokumenty PDF
* Paperback wraca teraz do zwykłego wyodrębniania tekstu w przypadku błędnie otagowanych plików PDF.
* Dokumenty PDF zawierające znaki sterujące w tytułach lub zakładkach nie powodują już awarii Paperbacka przy otwieraniu.

### Wersja 0.8.5
* Dodano obsługę stron w książkach EPUB.
* Dodano obsługę zaszyfrowanych dokumentów Microsoft Office. Obecnie obsługiwane są starsze dokumenty Word, nowoczesne dokumenty Word i nowoczesne prezentacje PowerPoint; obsługa starszych prezentacji PowerPoint jest planowana.
* Dodano obsługę starszych dokumentów Microsoft Word (*.doc)!
* Dodano obsługę starszych prezentacji PowerPoint (*.ppt)!
* Dodano obsługę książek MOBI i AZW3!
* Dodano obsługę tagowanych plików PDF!
* Dodano skrót Ctrl+Q do zakończenia aplikacji.
* Dodano obsługę spakowanych książek z Bookshare (zarówno DAISY, jak i Word)!
* Tekst alternatywny osadzonych obrazów powinien być teraz poprawnie pokazywany.
* Dokumenty CHM prawidłowo obsługują teraz nawigację po odnośnikach wewnętrznych.
* Naprawiono przesunięcie o 1 przy poleceniu Przejdź do strony.
* Naprawiono brak możliwości zamknięcia dialogu Otwórz jako klawiszem Escape.
* Naprawiono niewyświetlanie menu kontekstowego czytnika po kliknięciu prawym przyciskiem myszy albo użyciu klawisza Aplikacje.
* Naprawiono sytuację, w której podczas otwierania dokumentów z wiersza poleceń fokus czasami trafiał do niewłaściwego dokumentu.
* Pliki PDF zawierające wyłącznie obrazy są ponownie wykrywane i aplikacja informuje o ich istnieniu.
* Można teraz nawigować po obrazach i rysunkach odpowiednio za pomocą `G`/`Shift+G` oraz `F`/`Shift+F`.
* Paperback respektuje teraz ustawienie ciemnego trybu aplikacji.
* Usunięto obsługę DAISY XML, ponieważ nie jest już potrzebna.
* W drzewie Spisu treści przywrócono natywną nawigację Win32 po pierwszych literach.
* Dialog błędu wczytywania pokazuje teraz bardziej szczegółowe komunikaty.
* Widok WWW otwiera się teraz znacznie szybciej i płynniej.

### Wersja 0.8.2
* Dodano obsługę stron w dokumentach RTF!
* Naprawiono błąd, przez który otwarcie Widoku WWW w plikach EPUB zawierających odnośniki zewnętrzne automatycznie je aktywowało.
* Naprawiono błąd, przez który parser RTF w rzadkich przypadkach nie dodawał spacji między słowami.
* Naprawiono dzielenie akapitów na wiele krótkich wierszy w niektórych dokumentach PDF.
* Dokumenty PDF mają teraz podstawową obsługę nawigacji po odnośnikach i nagłówkach!
* Tabulatory i znaki końca wiersza w RTF są teraz renderowane dokładnie tak, jak występują w dokumencie.
* Przywrócono sprawdzoną bibliotekę pdfium do parsowania plików PDF, dzięki czemu renderowanie PDF jest ponownie znacznie bardziej niezawodne.

### Wersja 0.8.1
* Dodano skrót Ctrl+Shift+T do ponownego otwarcia ostatnio zamkniętego dokumentu.
* Dialog Wszystkie dokumenty obsługuje teraz wybór wielu dokumentów do jednoczesnego otwarcia.
* Naprawiono kilka błędów w parserze RTF.
* Naprawiono uszkadzanie ścieżek plików zawierających znaki spoza ASCII (na przykład bośniackie š, č, ć, ž) przy otwieraniu pliku przez drugą instancję Paperbacka.
* Naprawiono odczytywanie tekstu z PDF w niewłaściwej kolejności oraz błędne odstępy wokół słów pisanych wielkimi literami.
* Naprawiono powolne wczytywanie dużych dokumentów.
* Naprawiono lokalizację przycisków Tak/Nie w dialogach potwierdzenia.

### Wersja 0.8.0
* Dodano tłumaczenia na japoński, chiński uproszczony i wietnamski!
* Dodano automatyczny aktualizator, który zastępuje obecnie zainstalowaną wersję Paperbacka, zamiast tylko pobierać nową wersję!
* Dodano opcjonalną informację dźwiękową przy dotarciu do zakładki lub notatki. Dziękujemy Andre Louisowi za dźwięki!
* Dodano obsługę dokumentów RTF!
* Dodano obsługę dokumentów DAISY XML.
* Dodano obsługę płaskich plików tekstowych OpenDocument!
* Dodano obsługę płaskich prezentacji OpenDocument!
* Dodano obsługę separatorów z użyciem `S` i `Shift+S`.
* Każde przesunięcie o więcej niż 300 znaków jest teraz automatycznie dodawane do historii nawigacji.
* Naprawiono przywracanie okna Paperbacka z zasobnika systemowego.
* Naprawiono pokazywanie surowego tekstu zamiast wyrenderowanego HTML w Widoku WWW dla dokumentów Markdown.
* Naprawiono nieprawidłowe renderowanie tabel w plikach Markdown.
* Pliki PDF zawierające wyłącznie obrazy wyświetlają teraz ostrzeżenie przy próbie wczytania.
* Poprawnie osadzono informacje o wersji w pliku wykonywalnym Paperbacka.
* Podzielono dialog Opcje na karty, aby ułatwić używanie i nawigację.
* Przejście na bibliotekę Hayro do parsowania PDF zwiększyło niezawodność i szybkość oraz zmniejszyło liczbę bibliotek DLL.
* Cała aplikacja została przepisana w Rust. Nowy kod jest bezpieczniejszy, szybciej wczytuje dokumenty i łatwiej go utrzymywać oraz rozwijać.
* Menu kontekstowe kontrolki tekstu z treścią dokumentu zawiera teraz działania specyficzne dla czytnika, zamiast ogólnych pozycji takich jak wytnij i wklej.

### Wersja 0.7.0
* Dodano obsługę tabel w dokumentach opartych na HTML i XHTML! Między tabelami można przechodzić za pomocą `T` i `Shift+T`, a po naciśnięciu `Enter` można wyświetlić tabelę w Widoku WWW.
* Dodano podstawową funkcję renderowania WWW! Naciśnij `Ctrl+Shift+V`, aby otworzyć bieżącą sekcję dokumentu w Widoku WWW; jest to przydatne przy treściach takich jak złożone formatowanie lub przykłady kodu.
* Dodano rosyjskie tłumaczenie. Dziękujemy Ruslanowi Gulmagomedovowi!
* Dodano przycisk Wyczyść wszystko w dialogu Wszystkie dokumenty.
* Kontroler aktualizacji pokazuje teraz informacje o wydaniu, gdy dostępna jest nowa wersja.
* Naprawiono przywracanie okna z zasobnika systemowego.
* Naprawiono tłumaczenia przycisków Tak/Nie w dialogach potwierdzenia.
* Naprawiono wczytywanie konfiguracji podczas uruchamiania jako administrator.
* Naprawiono obsługę komentarzy w dokumentach XML i HTML.
* Naprawiono parsowanie Spisu treści w książkach EPUB 2.
* Naprawiono przechodzenie do następnego elementu o tej samej pierwszej literze w Spisie treści.
* Naprawiono nieprawidłowe ukrywanie dialogu Znajdź podczas używania przycisków następne/poprzednie.
* Naprawiono sytuacje, w których Spis treści w dokumentach EPUB czasami przenosił do niewłaściwego elementu.
* Naprawiono różne problemy z obsługą białych znaków w XML, HTML i znacznikach pre.
* Naprawiono przesunięcie o jeden odnośnik podczas nawigacji po odnośnikach.
* Naprawiono problem z nadmiarowymi białymi znakami na końcach wierszy w niektórych książkach.
* Naprawiono różne problemy parsera.
* Pozycje menu związane z zakładkami oraz Lista elementów są teraz prawidłowo wyłączone, gdy żaden dokument nie jest otwarty.
* Usprawniono obsługę list w różnych formatach dokumentów.
* Usprawniono proces pracy nad tłumaczeniami dla współtwórców.
* Wprowadzono wiele wewnętrznych refaktoryzacji, przenosząc większość logiki aplikacji z C++ do Rusta, aby poprawić wydajność i łatwość utrzymania.

### Wersja 0.6.1
* Dodano obsługę plików PDF chronionych hasłem!
* Dodano bardzo prostą funkcję przechodzenia do poprzedniej/następnej pozycji. Jeśli naciśniesz `Enter` na odnośniku wewnętrznym i kursor zostanie przeniesiony, ta pozycja zostanie zapamiętana i będzie można do niej wracać za pomocą `Alt+Strzałka w lewo`/`Alt+Strzałka w prawo`.
* Dodano Listę elementów! Obecnie pokazuje tylko drzewo wszystkich nagłówków w dokumencie albo listę odnośników, ale w przyszłości planowane jest jej rozszerzenie.
* Dodano opcję uruchamiania Paperbacka w zmaksymalizowanym oknie domyślnie.
* Naprawiono nieprawidłowe działanie odnośników w niektórych dokumentach EPUB.
* Naprawiono parsowanie Spisu treści EPUB zawierającego ścieżki względne.
* Naprawiono brak tytułu lub autora w niektórych dokumentach EPUB.
* Naprawiono nieprawidłowe wyświetlanie tytułów niektórych rozdziałów EPUB w dialogu Spis treści.
* Naprawiono brak możliwości aktywowania przycisków OK/Anuluj w dialogu Spis treści za pomocą spacji.
* Ulepszono obsługę nagłówków w dokumentach Word.
* Teraz otrzymasz informację głosową, jeśli lista ostatnich dokumentów jest pusta, gdy próbujesz otworzyć dialog.

### Wersja 0.6.0
* Do dialogu Opcje dodano nową opcję pokazywania menu Przejdź w dużo bardziej kompaktowej formie. Jest ona domyślnie włączona.
* Dodano opcję zawijania nawigacji po elementach strukturalnych.
* Do menu Narzędzia dodano opcję otwierania folderu zawierającego aktualnie aktywny dokument.
* Dodano prosty, ale skuteczny system aktualizacji.
* Dodano podstawowy Wyłącznik czasowy dostępny skrótem `Ctrl+Shift+S`.
* Dodano obsługę parsowania e-booków FB2!
* Dodano obsługę parsowania prezentacji OpenDocument!
* Dodano obsługę parsowania plików tekstowych OpenDocument!
* Zakładki mogą teraz obejmować cały wiersz albo tylko wskazany tekst. Jeśli podczas dodawania zakładki nic nie jest zaznaczone, działanie pozostaje takie jak przed wersją 0.6 i zakładka obejmuje cały wiersz. Jeśli zaznaczysz tekst, w zakładce znajdzie się tylko ten tekst.
* Zakładki mogą teraz mieć opcjonalne notatki tekstowe. Między zakładkami z notatkami można przechodzić klawiszami `N` i `Shift+N`, a dialog zakładek można otwierać z widokiem wszystkich zakładek, tylko notatek albo tylko zakładek bez notatek za pomocą osobnych skrótów.
* Zakładki w dialogu zakładek nie mają już irytującego prefiksu "zakładka x".
* Książki EPUB zawierające treść HTML udającą XML są teraz obsługiwane poprawnie.
* Naprawiono wczytywanie dużych dokumentów Markdown.
* Naprawiono aktywowanie przycisku OK po naciśnięciu spacji w widoku drzewa Spisu treści.
* Naprawiono obsługę białych znaków na początku znaczników pre w dokumentach HTML i XHTML.
* Naprawiono sytuacje, w których kontrolka tekstu czasami nie odzyskiwała fokusu po powrocie do okna Paperbacka.
* Naprawiono brak aktualizacji wartości suwaka przez pole tekstowe w dialogu Przejdź do procentu.
* Naprawiono renderowanie własnych identyfikatorów HTML w dokumentach Markdown.
* HTML wewnątrz bloków kodu Markdown jest teraz renderowany poprawnie.
* Jeśli książka jest wczytywana przez parametr wiersza poleceń, gdy działa już istniejąca instancja Paperbacka, nie pojawi się już błąd, jeśli wczytywanie dokumentu trwa dłużej niż 5 sekund.
* Jeśli Paperback działa jako administrator, konfiguracja jest teraz poprawnie wczytywana i zapisywana.
* Można teraz usunąć zakładkę bezpośrednio z dialogu zakładek.
* Można teraz importować i eksportować zakładki oraz pozycję czytania dla konkretnego dokumentu. Wygenerowany plik otrzymuje nazwę dokumentu z rozszerzeniem `.paperback`. Jeśli taki plik zostanie znaleziony w tym samym katalogu podczas wczytywania dokumentu, zostanie wczytany automatycznie. W przeciwnym razie można go zaimportować ręcznie z menu Narzędzia.
* Odnośniki wewnątrz dokumentów są teraz w pełni obsługiwane. Użyj `K` i `Shift+K`, aby przechodzić po nich do przodu i do tyłu, a `Enter`, aby otworzyć albo aktywować odnośnik.
* Wprowadzono wiele wewnętrznych refaktoryzacji, dzięki którym aplikacja jest szybsza, a plik binarny mniejszy.
* Treść Markdown jest teraz wstępnie przetwarzana do zgodności z CommonMark przed renderowaniem.
* Nawigacja po listach i ich elementach jest teraz w pełni obsługiwana. Użyj `L` i `Shift+L`, aby przechodzić po listach, oraz `I` i `Shift+I`, aby przechodzić po elementach listy.
* `Delete na klawiaturze numerycznej` działa teraz przy usuwaniu dokumentów z paska kart, tak samo jak zwykły `Delete`.
* Paperback może teraz opcjonalnie minimalizować się do zasobnika systemowego. Ta opcja jest domyślnie wyłączona, ale po jej włączeniu minimalizacja z menu systemowego przenosi Paperback do zasobnika, skąd można go przywrócić kliknięciem utworzonej ikony.
* Paperback jest teraz w pełni tłumaczalny! Lista obsługiwanych języków jest obecnie dość krótka, ale stale rośnie.
* Paperback ma teraz oficjalną stronę: [paperback.dev](https://paperback.dev)!
* Dokumenty PPTX pokazują teraz podstawowy Spis treści zawierający wszystkie slajdy.
* W dialogu Informacje o dokumencie pokazywana jest teraz pełna ścieżka do otwartego dokumentu.
* Instalator zawiera teraz opcję wyświetlenia pliku readme w przeglądarce po instalacji.
* Lista ostatnich dokumentów została znacznie rozszerzona. Zamiast pokazywać tylko 10 ostatnio otwartych dokumentów, pokazuje teraz konfigurowalną liczbę dokumentów, a pozostałe dokumenty kiedykolwiek otwarte są dostępne w małym dialogu.
* Wprowadzono różne drobne ulepszenia parserów, między innymi dodanie pustego wiersza między slajdami w prezentacjach PPTX, naprawienie obsługi nowych wierszy wewnątrz akapitów w dokumentach Word oraz dodanie punktorów do elementów listy.

### Wersja 0.5.0
* Dodano obsługę dokumentów Microsoft Word!
* Dodano obsługę prezentacji PowerPoint!
* Naprawiono brak wyłączania niektórych pozycji menu, gdy żaden dokument nie jest otwarty.
* Naprawiono orientację suwaka Przejdź do procentu.
* Naprawiono Spis treści w książkach EPUB ze ścieżkami plików kodowanymi jako URL i/lub identyfikatorami fragmentów.
* Naprawiono nietypowe usuwanie białych znaków z nagłówków XHTML.
* Naprawiono obsługę białych znaków wewnątrz zagnieżdżonych znaczników pre w dokumentach HTML.
* Dokumenty HTML i Markdown obsługują teraz funkcję Spisu treści. Po wczytaniu dokumentu HTML/Markdown Paperback zbuduje własny Spis treści na podstawie struktury nagłówków w dokumencie i pokaże go w dialogu `Ctrl+T`.
* Dokumenty HTML mają teraz tytuł ustawiony w znaczniku title, jeśli taki istnieje. W przeciwnym razie nadal używana jest nazwa pliku bez rozszerzenia.
* Zamiast UniversalSpeech używany jest teraz live region do zgłaszania mowy. Dzięki temu wraz z programem nie są już dostarczane biblioteki DLL czytników ekranu, a obsługiwanych jest więcej czytników, na przykład Microsoft Narrator.
* Zmieniono bibliotekę ZIP, aby umożliwić otwieranie szerszego zakresu książek EPUB.
* Dialog pytający, czy chcesz otworzyć dokument jako zwykły tekst, został całkowicie przebudowany i pozwala teraz otworzyć dokument jako zwykły tekst, HTML albo Markdown.
* Dialog Przejdź do procentu zawiera teraz pole tekstowe pozwalające ręcznie wpisać procent, do którego chcesz przejść.
* Parser HTML rozpoznaje teraz `dd`, `dt` i `dl` jako elementy list.
* Spis treści w książkach EPUB jest ponownie zachowywany dokładnie.
* Podczas usuwania pustych wierszy uwzględniana jest teraz nierozdzielająca spacja Unicode.
* Program nie pyta już, jak otworzyć nierozpoznany plik przy każdym jego wczytaniu, a tylko za pierwszym razem.

### Wersja 0.4.1
* Dodano opcjonalną ikonę w menu Start w instalatorze.
* Spis treści powinien być teraz w kilku przypadkach czytelniejszy; na przykład jeśli element podrzędny i nadrzędny mają ten sam tekst w tej samej pozycji, zobaczysz tylko element nadrzędny.
* Naprawiono Spis treści w niektórych dokumentach CHM.
* Naprawiono Spis treści w książkach EPUB 3 ze ścieżkami bezwzględnymi.
* Dokumenty CHM powinny teraz pokazywać tytuł ustawiony w pliku metadanych.

### Wersja 0.4.0
* Dodano obsługę plików CHM!
* Dodano obsługę zakładek! Możesz mieć dowolną liczbę zakładek w dowolnej liczbie dokumentów. Do przodu i do tyłu przechodzisz po nich klawiszami `B` i `Shift+B`, ustawiasz je skrótem `Ctrl+Shift+B`, a dialog przejścia do konkretnej zakładki otwierasz skrótem `Ctrl+B`.
* Dodano instalator obok przenośnego pliku ZIP! Instalator zainstaluje Paperbacka w katalogu Program Files i automatycznie skonfiguruje skojarzenia plików.
* Pliki tekstowe z BOM powinny być teraz poprawnie dekodowane, a BOM nie będzie już pokazywany na początku tekstu.
* Dodano znacznie więcej informacji do paska stanu. Pokazuje on teraz bieżący wiersz, znak i procent czytania.
* Komentarze HTML oraz zawartość znaczników script i style nie są już pokazywane w wyjściu tekstowym.
* Jeśli w wierszu poleceń podasz ścieżkę względną do Paperbacka, zostanie ona teraz poprawnie rozwiązana.
* Przechodzenie procentowe jest teraz obsługiwane przez osobny dialog oparty na suwaku, dostępny skrótem `Ctrl+Shift+G`.
* Dokumenty bez znanych tytułów lub autorów mają teraz zawsze wartość domyślną.
* Logika zapisywania pozycji jest teraz dużo inteligentniejsza i zapisuje na dysk tylko wtedy, gdy jest to naprawdę potrzebne.
* Dokument, który był aktywny podczas zamykania Paperbacka, jest teraz zapamiętywany między uruchomieniami aplikacji.
* Dane wejściowe w dialogach Przejdź do wiersza i Przejdź do strony są teraz bardziej rygorystycznie oczyszczane.
* Naprawiono nawigację po Spisie treści w książkach EPUB 3 ze ścieżkami względnymi w manifestach.

### Wersja 0.3.0
* Naprawiono Spis treści w książkach EPUB z manifestami kodowanymi jako URL.
* Naprawiono nawigację po nagłówkach w dokumentach HTML zawierających wielobajtowe znaki Unicode.
* Naprawiono wysokie użycie CPU w dokumentach z długimi tytułami spowodowane regresją w wxWidgets.
* Naprawiono wczytywanie plików tekstowych UTF-8.
* Naprawiono umieszczanie kursora w niewłaściwej pozycji przez zagnieżdżone elementy Spisu treści w książkach EPUB.
* Naprawiono awarię przy zamykaniu aplikacji w niektórych przypadkach.
* Dodano pole wyboru w dialogu Opcje do włączania lub wyłączania zawijania wierszy!
* Można teraz wspierać rozwój Paperbacka darowizną, przez nową pozycję Wesprzyj w menu Pomoc albo przez odnośnik sponsorowania projektu na dole głównej strony repozytorium GitHub.
* Dokumenty Markdown mają teraz zawsze tytuł, a Paperback powinien być w stanie wczytać praktycznie każdy plik Markdown.
* Dokumenty PDF mają teraz zawsze tytuł, nawet jeśli brakuje go w metadanych.
* Zmieniono bibliotekę PDF na tę używaną w Chromium, co daje znacznie bardziej niezawodne parsowanie PDF.
* W danym momencie może działać tylko jedna instancja Paperbacka. Uruchomienie `paperback.exe` z nazwą pliku, gdy program już działa, otworzy ten dokument w istniejącej instancji.
* Możesz teraz nacisnąć `Delete` na dokumencie w kontrolce kart, aby go zamknąć.

### Wersja 0.2.1
* Dodano całkowitą liczbę stron do etykiety strony w dialogu Przejdź do strony.
* Zezwolono na przechodzenie klawiszem Tab od treści dokumentu do listy otwartych dokumentów.
* Naprawiono sporadyczne otwieranie ostatnich dokumentów przez skróty nagłówków, jeśli było ich wystarczająco dużo.
* Paperback usuwa teraz zbędne miękkie dywizy z wyjścia tekstowego.
* Naprawiono sytuacje, w których nawigacja po nagłówkach czasami ustawiała kursor na niewłaściwym znaku.

### Wersja 0.2.0
* Dodano obsługę dokumentów Markdown!
* Dodano obsługę dokumentów PDF, w tym możliwość nawigacji między stronami!
* Dodano skróty do nawigacji po nagłówkach w treści HTML, w tym w książkach EPUB i dokumentach Markdown. Skróty te zaprojektowano tak, aby działały podobnie do czytnika ekranu.
* Naprawiono wczytywanie książek EPUB z nazwami plików kodowanymi jako URL w manifestach.
* Naprawiono wczytywanie książek EPUB 3 z osadzonym XHTML.
* Komunikat jest teraz wypowiadany, jeśli dokument nie obsługuje Spisu treści lub sekcji; wcześniej pozycje menu były po prostu wyłączane.
* Dodano menu Ostatnie dokumenty! Obecnie przechowuje 10 ostatnio otwartych dokumentów, a naciśnięcie `Enter` na jednym z nich otwiera go do czytania.
* Całkowicie przepisano dialog Znajdź, dzięki czemu jest znacznie prostszy w użyciu, a jednocześnie dodano historię ostatnich 25 wyszukiwań i obsługę wyrażeń regularnych!
* Poprzednio otwarte dokumenty są teraz zapamiętywane między uruchomieniami aplikacji. Można to skonfigurować przez nową pozycję Opcje w menu Narzędzia.
* Dodano `Shift+F1`, aby otwierać plik readme bezpośrednio w Paperbacku.

### Wersja 0.1.0
* Pierwsze wydanie.
