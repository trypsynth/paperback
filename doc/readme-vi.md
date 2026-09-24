<!-- machine-translated from doc/readme.md (source-hash: 6564745fd3218b1a; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ca4819ea,a9eba369,e9860ee8,3b8321f8); please review and edit as needed -->

# Paperback - phiên bản 1.0

## Giới thiệu

Paperback là một trình đọc nhẹ, nhanh và dễ tiếp cận cho sách điện tử, tài liệu và sách nói, dành cho mọi người, từ những độc giả bình thường đến những người dùng nâng cao. Nó được thiết kế cho khả năng tiếp cận bằng trình đọc màn hình, tốc độ nhanh và trải nghiệm không có tính năng thừa.

## Yêu cầu hệ thống

Paperback chạy trên Windows 10/11, tất cả các phiên bản ARM macOS hiện đại, Linux, iOS 17 trở lên và Android 7 trở lên. Các ứng dụng iOS và Android có sẵn trên App Store và Google Play.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu cài đặt bất kỳ phần mềm nào trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở nhiều tài liệu cạnh nhau như bạn muốn.
* Lưu lại vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đã mở khi đóng chương trình và khôi phục chúng khi khởi chạy lần sau.
* Bao gồm chức năng điều hướng tương tự như chế độ duyệt web của nhiều trình đọc màn hình để nhanh chóng và dễ dàng điều hướng qua các tài liệu.
* Bao gồm một hộp thoại tìm kiếm mạnh mẽ, có các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn di động hoặc cài đặt với liên kết tệp được thiết lập tự động.
* Hỗ trợ một loạt lớn các định dạng tệp phổ biến.
* Phát sách nói, với tốc độ có thể điều chỉnh và dấu trang ghi nhớ thời gian chính xác.
* Đọc các trang PDF được quét bằng OCR tích hợp trong Windows và macOS.
* Dấu trang và ghi chú, để bạn có thể đánh dấu vị trí của mình và quay lại nó.
* Mọi phím tắt bàn phím có thể được thay đổi.
* Đi kèm với `pb`, một công cụ dòng lệnh chuyển đổi bất kỳ tài liệu được hỗ trợ nào thành HTML, Markdown hoặc văn bản thuần túy.

## Tương thích trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết cho người dùng JAWS.

### JAWS và Màn hình Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi xoay tiến với các phím điều hướng của màn hình của bạn. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách xử lý điều khiển văn bản RICHEDIT50W của JAWS, không phải là điều gì trong chính Paperback, và một lỗi mất khá lâu để tìm ra một bản sửa chữa cho nó do sự nhiệt tình của Vispero trong việc phản ứng với các vấn đề liên quan đến phần mềm mã nguồn mở.

Giải pháp, cuối cùng được tìm ra thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn hoạt động thay vì tiến lên. Với cả hai cài đặt này, xoay tiến sẽ hoạt động chính xác.

## Các loại tệp hiện được hỗ trợ

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Lưu trữ truyện tranh (`.cbz`)
* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Trang hướng dẫn, cả `man` và BSD `mdoc` (`.1` đến `.9`, `.man`, `.roff` và các dạng nén của mỗi trang)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách nói M4B (`.m4b`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Sách nói MP3 (`.mp3`)
* Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bài thuyết trình PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tài liệu Windows Write (`.wri`)
* Tệp WinHelp (`.hlp`)
* Tệp văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng với bàn phím trước tiên. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây là cho Windows. Nơi macOS khác, phím tương đương được ghi chú trong dấu ngoặc — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã bị chiếm bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu Tệp

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đã đóng lần cuối.
* `Ctrl+R`: Hiển thị hộp thoại "Tất cả tài liệu" (từ Tài liệu gần đây).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS điều này nằm trong menu ứng dụng thay vào đó).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Tìm.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi đến dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi đến phần trăm.
* `Ctrl+P`: Đi đến trang (khi được tài liệu hiện tại hỗ trợ).
* `=`: Thông báo phần trăm đọc hiện tại của bạn và trang, ví dụ: "15%, trang 30". Trang bị bỏ qua đối với các tài liệu không có số trang.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến lên trong lịch sử điều hướng.
* `[`: Phần trước.
* `]`: Phần tiếp theo.
* `Shift+H`: Tiêu đề trước.
* `H`: Tiêu đề tiếp theo.
* `Shift+1` đến `Shift+6`: Tiêu đề trước ở mức 1-6.
* `1` đến `6`: Tiêu đề tiếp theo ở mức 1-6.
* `Shift+P`: Trang trước.
* `P`: Trang tiếp theo.
* `Shift+B`: Dấu trang trước.
* `B`: Dấu trang tiếp theo.
* `/`: Đặt dấu trang tạm thời của bạn.
* `\`: Nhảy đến dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Nhảy đến tất cả dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy đến dấu trang chỉ.
* `Ctrl+Alt+M`: Nhảy đến ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý chứ không phải Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
* `Shift+K`: Liên kết trước.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình trước.
* `F`: Hình tiếp theo.
* `Shift+T`: Bảng trước.
* `T`: Bảng tiếp theo.
* `Shift+M`: Công thức trước.
* `M`: Công thức tiếp theo.
* `Shift+S`: Dấu phân cách trước.
* `S`: Dấu phân cách tiếp theo.
* `Shift+L`: Danh sách trước.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi đến đầu vùng chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi quá cuối vùng chứa hiện tại (danh sách hoặc bảng).

### Menu Công cụ

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý chứ không phải Cmd): Hiển thị số từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong một tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần.
* `Ctrl+Shift+B`: Bật/tắt dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt tự động ngắt dòng.
* `Ctrl+Space` (macOS: `RawCtrl+Space`, tức là phím Control vật lý, vì Cmd+Space mở Spotlight): Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh về phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh về phía sau.
* `Shift+'`: Tăng lượng tìm kiếm âm thanh.
* `Shift+;`: Giảm lượng tìm kiếm âm thanh.
* `Ctrl+Shift+.`: Tăng tốc độ kể chuyện âm thanh.
* `Ctrl+Shift+,`: Giảm tốc độ kể chuyện âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở Cài đặt (macOS: trong menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.
* `Ctrl+Shift+O`: Nhận diện một loạt các trang PDF được quét bằng OCR.
* `Alt+F9` (macOS: `Cmd+F9`): Đánh dấu đầu lựa chọn, để mọi thứ từ đây đến nơi bạn đến có thể được sao chép cùng một lúc.
* `Alt+F10` (macOS: `Cmd+F10`): Sao chép mọi thứ từ đầu lựa chọn được đánh dấu đến vị trí hiện tại.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Nhảy lại đầu lựa chọn được đánh dấu, để lại dấu tại chỗ.

### Menu Trợ giúp

* `Ctrl+F1`: Hiển thị hộp thoại Giới thiệu.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím xem tài liệu bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu được chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Theo dõi liên kết hoặc mở chế độ xem bảng hoặc công thức ở con trỏ.
* `Enter` trên trang PDF được quét: Nhận diện trang bằng OCR.
* `Shift+F10` hoặc phím Menu/Ứng dụng trong văn bản tài liệu: Mở menu ngữ cảnh.

## iOS và Android

Các ứng dụng iOS và Android sử dụng cùng một công cụ đọc như trên máy tính để bàn, vì vậy chúng mở các định dạng tương tự và ghi nhớ vị trí của bạn theo cách tương tự. Chúng được xây dựng để sử dụng với VoiceOver trên iOS và TalkBack trên Android.

### Mở tài liệu

* Sử dụng nút Open Book hoặc mở tài liệu từ ứng dụng Files hoặc ứng dụng khác và chọn Paperback.
* Trên Android, bạn có thể bật trình duyệt tệp trong ứng dụng ở Settings. Nó cần quyền All Files Access và mở các tệp lớn ngay lập tức thay vì sao chép chúng trước.
* Nhấn giữ nút Open Book để nhập hoặc xuất dữ liệu tài liệu (`.paperback`), những tệp giống như ứng dụng máy tính để bàn sử dụng.

### Đọc và lắng nghe

Mỗi ứng dụng có hai cách để đọc tài liệu. Ở chế độ văn bản, bạn đọc văn bản bằng trình đọc màn hình của mình. Ở chế độ đọc to, Paperback đọc văn bản cho bạn bằng giọng nói bạn chọn trong Settings, và tiếp tục ở nền và từ màn hình khóa. Chuyển đổi giữa chúng từ menu More Options.

Các audiobook, như DAISY, M4B và sách MP3, phát bản ghi âm của riêng chúng thay thế.

### Thanh đọc

Thanh ở dưới cùng của màn hình có, từ trái sang phải:

* Đơn vị điều hướng, chẳng hạn như đoạn văn, tiêu đề, trang hoặc liên kết. Vuốt lên hoặc xuống trên nó để thay đổi nó.
* Các nút Trước, Phát và Tiếp theo. Trước và Tiếp theo di chuyển theo đơn vị điều hướng.
* Tốc độ phát âm. Vuốt lên hoặc xuống trên nó để thay đổi tốc độ đọc của Paperback.

Bạn cũng có thể vuốt lên hoặc xuống trên nút phát để di chuyển theo đơn vị điều hướng, mà không cần tiếp cận các nút Trước và Tiếp theo. Nếu đó là tất cả những gì bạn sử dụng, cài đặt Hide previous and next buttons sẽ loại bỏ chúng khỏi đường dẫn của trình đọc màn hình. Cài đặt Swipe up moves forward chọn hướng vuốt đi.

### More Options

Menu More Options là nơi mọi thứ khác nằm. Một số mục hoạt động khác một chút trên mỗi ứng dụng.

* **Switch to TTS Mode hoặc Switch to Text Mode:** di chuyển giữa chế độ đọc to và chế độ văn bản, được mô tả ở trên. Ở chế độ văn bản, mục Read Aloud bắt đầu và tạm dừng đọc to mà không rời khỏi chế độ văn bản.
* **Table of Contents:** các chương của sách, được mở tại chương bạn đang đọc. Chọn một để đi thẳng tới nó. Các mục có các chương dưới chúng có thể được mở rộng và thu gọn bằng các hành động của trình đọc màn hình.
* **Elements:** danh sách các tiêu đề hoặc liên kết của tài liệu. Chuyển đổi giữa hai cái bằng Bộ chọn Type trên iOS hoặc các thẻ trên Android, sau đó chọn một để đi tới nó.
* **Find:** nhập những gì cần tìm hoặc chọn tìm kiếm trước đó từ Search History, và chọn xem có so khớp chữ hoa chữ thường, chỉ khớp toàn bộ từ hay sử dụng biểu thức chính quy. Find Previous và Find Next nhảy đến một kết quả khớp và cho biết nó được đặt ở đâu, và Find vẫn mở để bạn có thể tiếp tục. Ở chế độ đọc to, Find cũng xuất hiện dưới dạng đơn vị điều hướng trên thanh đọc, vì vậy bạn có thể bước qua các kết quả khớp từ đó.
* **Go To:** nhảy tới một dòng, một trang hoặc một phần trăm trong tài liệu. Chọn cái nào bằng Bộ chọn Mode.
* **Recent Documents:** mọi tài liệu bạn đã mở, mỗi tài liệu được đánh dấu là hiện đang mở, đã đóng hoặc tệp bị thiếu. Mỗi tài liệu có hai hành động của trình đọc màn hình: Remove loại bỏ nó khỏi danh sách và Locate cho phép bạn tìm tài liệu có tệp đã được di chuyển. Clear Recent Documents làm trống danh sách mà không xóa bất kỳ tài liệu nào.
* **Word Count:** số từ trong tài liệu.
* **Document Info:** tiêu đề, tác giả, tên tệp và trên iOS cũng là số dòng và ký tự.
* **Export:** lưu tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* **Sleep Timer:** dừng đọc sau 5, 10, 15, 30, 45 hoặc 60 phút, hoặc một thời gian của riêng bạn. Mở nó lại khi nó đang chạy để xem còn bao lâu hoặc để hủy nó.
* **Help:** mở readme này.
* **Settings:**
    * **Text to speech:** giọng nói, tốc độ phát âm và cao độ, nút Play Sample để nghe chúng và tạm dừng giữa các đoạn văn. Android cũng cho phép bạn chọn công cụ phát âm. Trên iOS, đây cũng là nơi từ điển phát âm: các quy tắc thay đổi cách các từ được phát âm, cho mọi giọng nói hoặc chỉ một số.
    * **Readability:** kích thước văn bản, khoảng cách dòng, khoảng cách đoạn văn, căn chỉnh và văn bản độ tương phản cao. iOS cũng có giao diện sáng và tối.
    * **Behavior:** xem có bật lại tài liệu của bạn khi ứng dụng khởi động, hướng vuốt trên nút phát di chuyển và xem có ẩn các nút Trước và Tiếp theo. Android cũng có trình duyệt tệp trong ứng dụng ở đây.

### Bàn phím và tai nghe

Với bàn phím, các phím tắt máy tính để bàn để mở sách, tài liệu gần đây, Find, Go To, mục lục, số từ, thông tin tài liệu, xuất và bộ định thời ngủ đều hoạt động, sử dụng `Cmd` thay vì `Ctrl` trên iOS. Tương tự như vậy với các phím chữ cái duy nhất để di chuyển theo tiêu đề, trang, liên kết và các phím khác, và `Space` phát và tạm dừng. Trên iOS, các phím chữ cái duy nhất chỉ tiếp cận Paperback khi Tắt Quick Nav chữ cái duy nhất của VoiceOver.

Trên Android, nút tai nghe phát và tạm dừng bằng một lần nhấn, di chuyển về phía trước bằng hai lần, và quay lại bằng ba lần.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, và nhiều ngôn ngữ khác đang được thêm vào mọi lúc. Danh sách đầy đủ như sau.

Để tìm hiểu cách đóng góp, vui lòng đọc [Hướng Dẫn Dịch Thuật](translating.md) của chúng tôi.

* Bosnian
* Czech
* Dutch
* Finnish
* French
* German
* Japanese
* Polish
* Portuguese (Brazil)
* Russian
* Simplified Chinese
* Serbian
* Spanish
* Ukrainian
* Vietnamese

## Các nhân viên
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Quyên góp
Những người sau đây đã quyên góp cho sự phát triển của Paperback. Nếu bạn quyên góp, tên của bạn sẽ không được tự động thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi một nhà tài trợ GitHub công khai là căn cứ để tự động đưa vào danh sách này.

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

## Nhật ký thay đổi

### Phiên bản 1.0

1.0 là bản phát hành đầu tiên trên cả năm nền tảng: Windows, macOS, Linux, iOS và Android, với các ứng dụng iOS và Android trên App Store và Google Play.

#### Đã thêm

##### Chung
* Hỗ trợ Linux, dưới dạng AppImage hoặc tar.gz, với tích hợp màn hình nền để các tài liệu mở từ trình quản lý tệp của bạn.
* Đánh dấu phần đầu của một lựa chọn bằng `Alt+F9`, sao chép mọi thứ từ đó đến nơi bạn đã đến bằng `Alt+F10`, và quay lại dấu bằng `Alt+Shift+F9`, để sao chép một đoạn văn bản dài mà không cần shift-arrow qua nó. Cả ba đều nằm dưới Tools > Select and copy.
* Phím tắt `=` hiện công bố trang cũng như phần trăm, ví dụ như "15%, trang 30", và vẫn giữ nguyên như trước đối với các tài liệu không có số trang.
* Hộp About hiện hiển thị giấy phép Paperback và mọi dịch giả.
* Bản dịch tiếng Ukraina.

##### Định dạng mới
* Lưu trữ truyện tranh (`.cbz`).
* Sách nói M4B, chia thành các chương của chúng.
* Trang hướng dẫn, cả `man` và BSD `mdoc`, được nén gzip hay không.
* Sách nói MP3, chia thành các chương khi tệp có chúng.
* Tệp Windows Write (`.wri`).
* Tệp WinHelp (`.hlp`).
* Tài liệu Word 6 và Word 95.

##### OCR
* Các trang PDF được quét hiện có thể được nhận dạng bằng OCR tích hợp trong Windows và macOS. Nhấn `Enter` trên một trang được quét để nhận dạng nó, hoặc sử dụng Batch OCR (`Ctrl+Shift+O`) cho một loạt trang.

##### Điều hướng
* Công thức MathML trong EPUB và HTML được hiển thị dưới dạng AsciiMath bằng MathCAT. Sử dụng `M` hoặc `Shift+M` để điều hướng công thức, sau đó nhấn `Enter` hoặc `Space` để mở MathML gốc trong Formula View.
* Nút Find All trong hộp thoại Find, liệt kê mọi dòng có trùng khớp để bạn có thể nhảy thẳng đến dòng bạn muốn.
* Chế độ xem Tables, Lists và Pages trong danh sách phần tử (`F7`).
* Go to Line, Go to Page và Go to Percent hiện chấp nhận `+n` và `-n` để di chuyển tương đối so với vị trí hiện tại của bạn.
* Sách EPUB, MOBI và CHM không có tiêu đề riêng của chúng hiện được hưởng lợi từ điều hướng tiêu đề từ mục lục của chúng.
* Sách KF8 (AZW3) hiện hỗ trợ điều hướng phần.
* Các trang EPUB chỉ là hình ảnh hiện hiển thị một dòng cho nó, vì vậy bạn có thể hạ cánh trên chúng thay vì bỏ qua ngay lập tức.

##### Sách nói
* Điều khiển tốc độ phát lại, từ nửa tốc độ đến ba lần nhanh hơn. Sử dụng `Ctrl+Shift+.` và `Ctrl+Shift+,`, hoặc menu Tools.
* Dấu trang và ghi chú trong sách chỉ âm thanh hiện ghi nhớ thời gian chính xác bạn đặt chúng.
* Vị trí tiếp theo và vị trí trước (`Alt+Left` và `Alt+Right`) hiện hoạt động trong sách nói.
* Tiến trình thông qua một cuốn sách nói hiện được đo bằng bản ghi của nó, vì vậy Go to Percent và thanh trạng thái khớp với mức độ bạn thực sự ở.

##### Tài liệu gần đây
* Mục Clear Recent Documents trong menu con Recent Documents.

##### Tài liệu PDF
* Một cài đặt để giữ mọi dòng của PDF riêng biệt, thay vì nối chúng thành các đoạn.
* Hình ảnh và hình vẽ trong các PDF hiện được công bố.
* Các PDF mang cấu trúc đọc nhưng không gắn thẻ bất kỳ hình ảnh nào của chúng hiện công bố những hình ảnh đó, thay vì bỏ qua hoàn toàn trong cuốn sách.

##### Web View
* Bất kỳ tài liệu nào hiện có thể được mở trong chế độ xem web, không chỉ EPUB, HTML và Markdown.

##### Khả năng đọc
* Tiêu đề hiện được vẽ ở kích thước phù hợp với mức độ của chúng, và hình ảnh cũng như bảng được tách biệt khỏi văn bản xung quanh chúng.

##### pb
* `pb --list-formats` liệt kê mọi định dạng mà pb có thể đọc.
* pb hiện nói tệp nào mà nó không thể đọc được, và tại sao.

#### Đã sửa

##### Chung
* Đã sửa lỗi crash khi đóng Paperback.
* Đóng Paperback giờ đây sẽ ẩn cửa sổ ngay lập tức, thay vì để nó trên màn hình trong khi nó lưu.
* Mở tài liệu không còn để Mở lại tài liệu đóng gần đây được bật khi không có gì để mở lại.
* Paperback không còn tiếp tục thử lại các tài liệu trong danh sách gần đây của bạn đã biến mất, và giới hạn số lượng tài liệu gần đây mà nó lưu trữ.
* Tệp cài đặt INI cũ giờ đây được xóa sau khi nó đã được chuyển sang định dạng mới.
* Tiêu đề của các hộp thoại phông chữ và màu sắc, và menu Xuất dưới dạng bằng tiếng Việt, giờ đây được dịch.
* Cập nhật giờ đây sẽ đưa cửa sổ được khởi động lại ra phía trước, thay vì để nó phía sau tất cả các cửa sổ khác trong `Alt+Tab`.
* Tự động xuống dòng giờ đây áp dụng ngay trên các tài liệu lớn, thay vì tải lại toàn bộ.

##### Điều hướng
* `Alt+Left` giờ đây sẽ quay lại nơi bạn nhảy từ, thay vì đến một vị trí cũ hơn.
* Âm thanh dấu trang giờ đây chỉ phát khi bạn di chuyển qua dấu trang, không phải khi bạn hạ cánh trên dòng nó nằm trên.
* Đóng mục lục, danh sách phần tử và các hộp thoại Đi đến giờ đây sẽ đưa bạn thẳng đến dòng bạn hạ cánh, thay vì khiến bạn ngồi trong khi trình đọc màn hình đọc lại cửa sổ.
* Đi tới Dòng, Đi tới Trang và Đi tới Phần trăm giờ đây từ chối các số bên ngoài tài liệu thay vì im lặng đi đâu đó khác.
* NVDA không còn cắt bỏ thông báo khi tài liệu không có trang.
* Nhấn OK trong mục lục mà không di chuyển giờ đây sẽ đi đến mục nhập đã được chọn.
* Mục lục, danh sách phần tử và danh sách dấu trang không còn chậm hoặc đông cứng trên các cuốn sách có hàng nghìn mục nhập.
* Mũi tên Lên và Xuống giờ đây ghi nhớ cột của chúng cho mỗi tài liệu, thay vì mang nó theo khi bạn chuyển đổi tab.

##### Sách nói
* Phát lại âm thanh giờ đây sử dụng `Control+Space` trên macOS, vì `Command+Space` thuộc về Spotlight.

##### Tài liệu PDF
* Đã sửa các PDF được xuất từ Apple Pages đọc dưới dạng văn bản thuần túy, mà không có bất kỳ tiêu đề và danh sách nào mà chúng được viết.
* Đã sửa các đoạn PDF và tiêu đề chia tách ở mỗi dòng, và các từ tách riêng ở các khoảng trắng.
* Đã sửa các tiêu đề PDF được đánh số chạy vào nhau thành một tiêu đề.
* Đã sửa các PDF có cây cấu trúc dẫn đến không có văn bản mở trống.
* Đầu trang và chân trang không còn được đọc trên mỗi trang của các PDF không được gắn thẻ.
* Các PDF gắn thẻ đầu trang và chân trang của chúng dưới dạng văn bản thông thường không còn lặp lại tiêu đề và số trang giữa hai đoạn trên mỗi trang.
* Các PDF giờ đây hiển thị tiêu đề thực của chúng, thay vì tên tệp của chúng.
* Các dòng được đặt trong phông chữ monospaced, như mã, không còn được nối vào các đoạn.

##### Sách MOBI/AZW3
* Các cuốn sách MOBI lớn không còn hết bộ nhớ và không còn bị cắt bỏ sau 20 MB.
* Sách MOBI và AZW3 giờ đây mở nhanh hơn nhiều.
* Đã sửa sách MOBI mất danh sách chương của họ.
* Đã sửa văn bản bị xáo trộn khi sách MOBI chuyển từ bản ghi này sang bản ghi khác.

##### Chế độ xem Web
* Chế độ xem web không còn tải toàn bộ một cuốn sách khổng lồ cùng một lúc.
* Chế độ xem web giờ đây hiển thị tài liệu toàn bộ khi trình đọc hiển thị chúng toàn bộ, thay vì chỉ một phần của chúng.

##### Định dạng khác
* Sách FictionBook (.fb2) được viết bằng windows-1251, phần lớn trong số đó, giờ đây mở thay vì hoàn toàn không đọc được.
* Sách FictionBook sử dụng không gian tên hoặc thực thể HTML mà chúng chưa bao giờ khai báo giờ đây mở, thay vì bị từ chối dưới dạng bị hỏng.
* Sách ở các mã hóa kế thừa giờ đây mở nhanh hơn nhiều.
* Đã sửa một số tệp văn bản tiếng Trung mở dưới dạng văn bản bị xáo trộn.
* Tệp OpenDocument được bảo vệ bằng mật khẩu giờ đây yêu cầu mật khẩu của chúng, thay vì được báo cáo là bị hỏng.
* Tệp PowerPoint kế thừa được bảo vệ bằng mật khẩu giờ đây mở, và các slide PowerPoint kế thừa không còn mất văn bản của chúng.
* Tệp văn bản thuần túy được lưu với phần mở rộng `.rtf` giờ đây mở dưới dạng văn bản, thay vì không chạy với lỗi.
* Các từ kiểm soát RTF không còn hiển thị dưới dạng văn bản.

#### iOS và Android

Các ứng dụng iOS và Android mở mọi định dạng mà máy tính để bàn làm được, và bao gồm:

* Đọc to, với lựa chọn giọng nói, tốc độ và cao độ của bạn, kiểm soát tốc độ nói chuyện ngay trên thanh đọc, và một khoảng tạm dừng tùy chọn giữa các đoạn.
* Phát lại sách nói DAISY, M4B và MP3, hoạt động trong nền và từ màn hình khóa.
* Điều hướng theo tiêu đề, trang, liên kết, bảng, danh sách và hơn thế nữa từ thanh đọc, cộng với mục lục và Tìm.
* Bộ hẹn giờ ngủ, số từ và xuất tài liệu, cộng với từ điển nói trên iOS.
* Tùy chọn kích thước văn bản, khoảng cách và văn bản tương phản cao.
* Phím tắt bàn phím khớp với máy tính để bàn.

### Phiên bản 0.9.2
* Sách nói không còn làm trình đọc màn hình của bạn đọc một loạt khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách nói giờ đây đặt tên tệp khi bạn bước qua chúng theo phần.
* Sách nói giờ đây báo cáo độ dài thực của chúng, thay vì tuyên bố mọi tệp trong chúng chạy 24 giờ.
* Đóng Chế độ xem Web bằng Escape không còn hiện thị cảnh báo gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Chọn tất cả giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần của nó hiện đang tải.
* Tìm giờ đây cắt thẳng đến dòng nó tìm thấy, thay vì khiến bạn ngồi trong khi trình đọc màn hình đọc lại cửa sổ khi tiêu điểm quay lại sách.
* Đã sửa EPUB chứa một khối ZIP64 lạc từ chối mở với "Invalid local file header".
* Đã sửa các tài liệu dài bước quay lại đầu của chúng trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView giờ đây đưa bạn đến phần mà chúng trỏ đến, thay vì không chạy với "File not found".
* Thông báo tự động "Document reloaded" không còn cắt trình đọc màn hình của bạn giữa câu, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt giờ đây tab thông qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật trực tiếp sau tùy chọn kiểm tra cập nhật.
* Windows giờ đây sẽ luôn hiển thị "Paperback" trong menu Mở bằng, thay vì tagline đầy đủ của chương trình.
* Số từ và Thông tin tài liệu giờ đây hiển thị có bao nhiêu tệp mà sách nói chứa và nó chạy trong tổng cộng bao lâu.

### Phiên bản 0.9.1
* Âm thanh dấu trang và ghi chú hiện phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng trong im lặng.
* Đã sửa các dấu ngoặc kép cong, dấu gạch em và các ký tự tương tự biến mất từ các tài liệu RTF, làm cho các từ xung quanh chạy lại với nhau.
* Đã sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản rối.
* Đã sửa menu Tài liệu gần đây giữ các mục cũ cho đến khi có điều gì khác xảy ra để xây dựng lại nó.
* Phím tắt bàn phím quay trở lại trong mọi bản dịch, vì vậy menu của Tiếng Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn hiện mở nhanh hơn đến bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và trên macOS, quy ước nền tảng.
* Paperback hiện nhớ vị trí cửa sổ, kích thước và trạng thái phóng to của nó giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm những thứ được đọc đúng cách trong các ngôn ngữ cần nhiều hơn một hình thức.
* Chọn ncc.html của sách DAISY hiện mở toàn bộ audiobook thay vì chỉ văn bản của nó.
* Tên hành động trong hộp thoại Tùy chỉnh phím tắt bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu hiện xuất hiện trước tiên trên thanh tiêu đề, vì vậy các cuốn sách đang mở có thể được phân biệt trong thanh tác vụ và `Alt+Tab`.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Được thêm vào

##### Chung
* Công cụ CLI, được gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần.
* Tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Tùy chọn View Source để mở nguồn tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, có nghĩa là bạn có thể tải các cuốn sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Công tắc toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Nút định vị để định vị các cuốn sách bị mất vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Tab khả năng đọc, với các tùy chọn sau:
    * Ngắt dòng từ (được di chuyển từ mục chung);
    * Hiển thị bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Độ giãn dòng;
    * Độ giãn đoạn;
    * Độ giãn chữ;
    * Căn chỉnh văn bản.
* Mục menu ngắt dòng từ và phím nóng tiếp theo.
* Công tắc để xác định cách bạn muốn hiển thị bảng và cách thống nhất bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo vùng chứa.
* Tùy chọn tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để thông báo tỷ lệ phần trăm hiện tại của bạn thông qua một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một cái cho mỗi tài liệu và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Số lượng từ
* Thời gian đọc ước tính trong hộp thoại số lượng từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu lựa chọn hoạt động khi bạn mở hộp thoại số lượng từ, số lượng từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh từng phím tắt trong ứng dụng thông qua một hộp thoại đơn giản.
* Phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, Tiếng Phần Lan và Tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần.

##### Trình cập nhật
* Nút hủy trong hộp thoại cập nhật đang diễn ra.
* Trình cập nhật hiện xác thực tệp đã tải xuống chưa bị giả mạo.

##### Chế độ xem web
* Chế độ xem web hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ cho sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Audiobooks
* Khả năng phát các audiobook, hiện hỗ trợ cả DAISY audio (bao gồm DAISY audio + text) và zip của các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời tường thuật, tìm kiếm về phía trước và phía sau, và điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu nháy đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn liệu tìm kiếm vượt quá cuối chương tiếp tục vào chương tiếp theo.

##### Tài liệu CHM
* Hỗ trợ cho danh sách, mục danh sách, số liệu và hình ảnh.

##### PowerPoint
* Các tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã Sửa

##### Chung
* Các tài liệu được mã hóa bằng các bảng mã CJK cũ như GBK, Big5 và Shift_JIS giờ đây sẽ hiển thị đúng cách thay vì hiển thị một đống mojibake.
* "Mở lại cái đã đóng" đang cố gắng mở lại readme được gói trong ứng dụng.
* Tab được chọn của bạn không được focus đúng cách sau khi khởi động lại Paperback.
* Cách Paperback xử lý các tệp trên ổ đĩa mạng Windows: nhấn hiển thị tệp trong thư mục giờ đây sẽ đúng cách focus tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp `.paperback` sẽ không còn được tải ép buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy tệp.
* Mở thư mục chứa giờ đây sẽ focus tệp đã cho trong trình khám phá.
* Mở readme giờ đây sẽ tôn trọng ngôn ngữ được chọn của bạn.
* Giao diện người dùng của Paperback giờ đây sẽ được chia tỷ lệ đúng cách trên các màn hình DPI cao.
* Thực đơn giờ đây sẽ cập nhật đúng cách và focus sẽ chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động giờ đây sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục theo ký tự bên trong.

##### Hộp thoại Tất cả Tài liệu
* Escape không đóng các hộp thoại Thông tin Tài liệu và Tất cả Tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở qua `Shift+F1`.
* Loại bỏ tài liệu từ hộp thoại gần đây giờ đây sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ đây được bảo tồn sau khi loại bỏ tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Đi đến Dòng, Đi đến Trang và Đi đến Phần trăm đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Tìm và Tìm Tiếp theo không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh Dấu trang/ghi chú giờ đây sẽ phát đúng cách độc quyền khi bạn điều hướng trên một từ chứa một.

##### Khả năng đọc
* Áp dụng ngắt dòng chữ đưa bạn đến đầu tài liệu của bạn.

##### Chế độ xem Web
* Hộp thoại webview không thể thay đổi kích thước và xuất hiện ở một kích thước ban đầu rất nhỏ.
* Hình ảnh giờ đây sẽ hiển thị đúng cách trong webview được nhúng.

##### Trình cập nhật
* Trình cập nhật giờ đây sẽ hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Các sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích cú pháp các tài liệu RTF có ký tự không phải Latin trong chúng.
* Các nhóm RTF `\pict` để dữ liệu hình ảnh nhúng không còn thoát vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo filepos trong sách Mobi chia tách các thẻ HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Cải thiện phân tích cú pháp AZW3 rất lớn.

##### Tài liệu Word
* Các tài liệu Word có tên kiểu dáng cụ thể theo ngôn ngữ không hiển thị các tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong các tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ đây quay trở lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang cho các sách epub.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại, Word cũ và Word hiện đại cũng như Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Word cũ!
* Đã thêm hỗ trợ cho các bản trình bày Powerpoint cũ!
* Đã thêm hỗ trợ cho các sách mobi và AZW3!
* Đã thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Đã thêm phím tắt ctrl+q để thoát ứng dụng.
* Đã thêm hỗ trợ cho các sách nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh được nhúng giờ đây sẽ hiển thị đúng cách.
* Các tài liệu CHM giờ đây sẽ hỗ trợ chính xác điều hướng liên kết nội bộ.
* Đã sửa đi đến trang bị lệch 1.
* Đã sửa phím escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa thực đơn ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Ứng dụng.
* Đã sửa tài liệu sai đôi khi được focus khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ có hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua hình ảnh và hình vẽ bằng g/shift+g và f/shift+f tương ứng.
* Paperback giờ đây sẽ tôn trọng cài đặt chế độ tối ứng dụng của bạn.
* Đã loại bỏ hỗ trợ DAISY XML vì không còn cần thiết.
* Chuyển trở lại điều hướng chữ cái đầu tiên Win32 gốc trong cây mục lục.
* Hộp thoại lỗi tải giờ đây sẽ hiển thị các thông báo lỗi chi tiết hơn.
* Webview giờ đây sẽ mở nhanh hơn nhiều và mịn hơn.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang cho các tài liệu RTF!
* Đã sửa lỗi khi mở webview trong các epub chứa liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi khi trình phân tích RTF sẽ không đặt dấu cách giữa các từ trong các trường hợp hiếm gặp.
* Đã sửa các đoạn văn bị chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Các tài liệu PDF giờ đây có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab RTF và dòng feed giờ đây sẽ được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Chuyển trở lại thư viện pdfium đã được kiểm chứng để phân tích cú pháp PDF, làm cho kết xuất PDF đáng tin cậy hơn nhiều lần nữa.

### Phiên bản 0.8.1
* Đã thêm `Ctrl+Shift+T` để mở lại tài liệu đã đóng cuối cùng.
* Hộp thoại Tất cả Tài liệu giờ đây hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích RTF.
* Đã sửa đường dẫn tệp chứa các ký tự không phải ASCII (như Bosnian š, č, ć, ž) bị hỏng khi mở tệp qua thực thể Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa của các nút Có/Không trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Đã thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Đã thêm trình cập nhật tự động sẽ thay thế phiên bản Paperback hiện tại của bạn thay vì chỉ tải xuống phiên bản mới!
* Đã thêm phản hồi âm thanh tùy chọn khi đạt đến dấu trang hoặc ghi chú, cảm ơn Andre Louis vì những âm thanh!
* Đã thêm hỗ trợ tài liệu RTF!
* Đã thêm hỗ trợ cho tài liệu DAISY XML.
* Đã thêm hỗ trợ cho tệp Open Document Text phẳng!
* Đã thêm hỗ trợ cho bài thuyết trình Open Document phẳng!
* Đã thêm hỗ trợ cho dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động được thêm vào lịch sử điều hướng của bạn.
* Đã sửa lỗi khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa lỗi tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Chế độ xem Web.
* Đã sửa lỗi bảng không hiển thị đúng trong tệp Markdown.
* Các PDF chỉ chứa hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một.
* Nhúng đúng thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ dàng sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy cao hơn, tốc độ nhanh hơn và ít DLL hơn.
* Viết lại toàn bộ ứng dụng bằng Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu ngữ cảnh của điều khiển văn bản giờ sẽ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Đã thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một bảng trong trình hiển thị web.
* Đã thêm tính năng hiển thị web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình hiển thị dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Đã thêm nút Xóa tất cả vào hộp thoại Tất cả tài liệu.
* Trình kiểm tra cập nhật giờ sẽ hiển thị ghi chú phát hành khi có phiên bản mới.
* Đã sửa lỗi khôi phục cửa sổ từ khay hệ thống.
* Đã sửa lỗi dịch các nút Có/Không trong hộp thoại xác nhận.
* Đã sửa lỗi tải cấu hình khi chạy với quyền quản trị viên.
* Đã sửa lỗi xử lý bình luận trong tài liệu XML và HTML.
* Đã sửa lỗi phân tích cú pháp TOC trong các cuốn sách Epub 2.
* Đã sửa lỗi điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Đã sửa lỗi hộp thoại tìm kiếm không ẩn đúng khi sử dụng các nút tiếp theo/trước đó.
* Đã sửa lỗi epub TOC thỉnh thoảng đưa bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng trong các thẻ XML, HTML và pre.
* Đã sửa lỗi so sánh sai trong điều hướng liên kết.
* Đã sửa một số cuốn sách có khoảng trắng ở cuối trên các dòng của chúng.
* Đã sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Đã thêm hỗ trợ PDF bảo vệ bằng mật khẩu!
* Đã thêm tính năng rất cơ bản để đi đến vị trí trước/tiếp theo. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể được điều hướng bằng alt+left/right arrows.
* Đã thêm danh sách phần tử! Hiện tại nó chỉ hiển thị một cây của tất cả các tiêu đề trong tài liệu của bạn hoặc một danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Đã thêm tùy chọn để khởi động Paperback ở chế độ phóng đại theo mặc định.
* Đã sửa lỗi liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa lỗi phân tích cú pháp Epub TOC chứa các đường dẫn tương đối.
* Đã sửa lỗi một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa lỗi tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa lỗi bạn không thể sử dụng thanh space bar để kích hoạt các nút OK/hủy bỏ trong hộp thoại TOC.
* Cải thiện xử lý các tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi bằng giọng nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0
* Đã thêm một tùy chọn mới để hiển thị menu go ở dạng gọn gàng hơn vào hộp thoại tùy chọn, được bật theo mặc định.
* Đã thêm tùy chọn để điều hướng theo các phần tử cấu trúc có thể quay lại.
* Đã thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại được tập trung vào.
* Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập bằng `Ctrl+Shift+S`.
* Đã thêm hỗ trợ phân tích cú pháp sách điện tử FB2!
* Đã thêm hỗ trợ phân tích cú pháp các bản trình bày OpenDocument!
* Đã thêm hỗ trợ phân tích cú pháp các tệp Văn bản OpenDocument!
* Bookmarks bây giờ có thể đánh dấu toàn bộ một dòng hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn nào hoạt động khi đặt bookmark, hành vi giống như pre-0.6 và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào bookmark.
* Bookmarks bây giờ có thể có ghi chú văn bản tùy chọn được đính kèm! Điều hướng giữa các bookmarks chứa ghi chú bằng `N` và `Shift+N`, hoặc bật cửa sổ hộp thoại bookmarks với tất cả bookmarks, chỉ ghi chú hoặc chỉ các ghi chú không được chọn bằng các phím tắt cụ thể.
* Các bookmarks trong hộp thoại bookmarks sẽ không còn có tiền tố "bookmark x" khó chịu.
* Các cuốn sách Epub chứa nội dung HTML giả làm XML bây giờ sẽ được xử lý đúng cách.
* Đã sửa lỗi tải các tài liệu Markdown lớn.
* Đã sửa lỗi nhấn phím space trong chế độ xem cây mục lục kích hoạt nút OK.
* Đã sửa lỗi xử lý khoảng trắng ở đầu thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa lỗi điều khiển văn bản không nhận lại tiêu điểm đôi khi khi quay lại cửa sổ Paperback.
* Đã sửa lỗi trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Đã sửa lỗi hiển thị các ID HTML tùy chỉnh trong các tài liệu Markdown.
* HTML bên trong các khối mã Markdown bây giờ sẽ được hiển thị đúng cách.
* Nếu tải một cuốn sách bằng tham số dòng lệnh trong khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback với quyền quản trị viên, cấu hình bây giờ sẽ được tải và lưu đúng cách.
* Bây giờ có thể xóa một bookmark trực tiếp từ trong hộp thoại bookmarks.
* Bây giờ có thể nhập và xuất các bookmarks và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo sẽ được đặt tên theo tệp với phần mở rộng `.paperback`. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng theo cách thủ công bằng cách sử dụng một mục trong menu công cụ.
* Các liên kết bên trong tài liệu bây giờ được hỗ trợ đầy đủ! Sử dụng `k` và `shift+k` để di chuyển về phía trước và phía sau qua chúng, và nhấn `Enter` để mở/kích hoạt một liên kết.
* Nhiều tái cấu trúc nội bộ, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown bây giờ được xử lý trước để tuân thủ CommonMark trước khi hiển thị.
* Điều hướng theo danh sách và các mục của chúng bây giờ được hỗ trợ đầy đủ! Sử dụng `L` và `Shift+L` để đi theo các danh sách, và `I` và `Shift+I` để đi qua các mục danh sách.
* Phím xóa trên Numpad bây giờ hoạt động để xóa tài liệu khỏi thanh tab ngoài phím xóa bình thường.
* Paperback bây giờ có thể thu nhỏ vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback bây giờ có thể dịch đầy đủ! Danh sách các ngôn ngữ nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang phát triển liên tục!
* Paperback bây giờ có trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Các tài liệu PPTX bây giờ sẽ hiển thị một bảng mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu đã mở sẽ bây giờ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt bây giờ bao gồm một tùy chọn để xem tệp readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách các tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ hiển thị cho bạn 10 tài liệu cuối cùng bạn đã mở, nó bây giờ sẽ hiển thị một số tùy chỉnh, với phần còn lại của các tài liệu bạn đã mở được truy cập thông qua một hộp thoại nhỏ.
* Nhiều cải tiến nhỏ cho các trình phân tích cú pháp trên toàn bảng, bao gồm đặt một dòng trống giữa các slide trong các bài trình bày PPTX, sửa lỗi xử lý newline bên trong các đoạn văn trong tài liệu word và thêm các dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho các bài thuyết trình PowerPoint!
* Sửa lỗi: một số mục menu không bị vô hiệu hóa khi không có tài liệu nào được mở.
* Sửa lỗi: hướng của thanh trượt go to percent.
* Sửa lỗi: mục lục trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Sửa lỗi: khoảng trắng bị loại bỏ từ các tiêu đề XHTML theo cách kỳ lạ.
* Sửa lỗi: xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ tạo mục lục riêng từ cấu trúc các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại `ctrl+t`.
* Tài liệu HTML hiện sẽ có tiêu đề được đặt trong thẻ tiêu đề, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo lời nói. Điều này có nghĩa là không còn các DLL trình đọc màn hình nào được gửi kèm với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển đổi thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được làm lại hoàn toàn, và hiện nó cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm một trường văn bản cho phép bạn nhập thủ công một phần trăm để nhảy đến.
* Trình phân tích cú pháp HTML hiện sẽ nhận dạng dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo tồn chính xác một lần nữa.
* Unicode không-phá-vỡ-dòng hiện được xem xét khi loại bỏ các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp không được nhận dạng mỗi lần bạn tải nó nữa, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm một biểu tượng menu Bắt đầu tùy chọn vào trình cài đặt.
* Mục lục hiện sẽ sạch hơn trong một số trường hợp, ví dụ nếu bạn có mục con và mục cha có cùng văn bản ở cùng vị trí, bạn hiện chỉ sẽ thấy mục cha.
* Sửa lỗi: mục lục trong một số tài liệu CHM.
* Sửa lỗi: mục lục trong sách Epub 3 với đường dẫn tuyệt đối trong đó.
* Tài liệu CHM hiện sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ đánh dấu! Bạn có thể có bao nhiêu đánh dấu tùy thích trong bao nhiêu tài liệu tùy thích. Bạn có thể nhảy tới và lùi lại giữa chúng với b và `shift+b`, đặt một cái với `control+shift+b`, và mở một hộp thoại để nhảy đến một đánh dấu cụ thể với `control+b`.
* Đã thêm một trình cài đặt cùng với tệp zip có thể di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập các liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện sẽ được giải mã chính xác, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Hiện nó sẽ hiển thị cho bạn dòng hiện tại, ký tự và tỷ lệ phần trăm đọc của bạn.
* Các bình luận HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu truyền một đường dẫn tương đối cho Paperback trên dòng lệnh, nó hiện sẽ phân giải nó chính xác.
* Chuyển động theo phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt riêng của nó, có thể truy cập bằng `control+shift+g`.
* Tài liệu không có tiêu đề hoặc tác giả đã biết hiện sẽ luôn có một mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ nên ghi vào đĩa khi tuyệt đối cần thiết.
* Tài liệu bạn đã tập trung vào khi bạn đóng Paperback hiện được nhớ lại trong các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page hiện phải được tẩy sạch chặt chẽ hơn.
* Sửa lỗi: điều hướng mục lục trong sách epub 3 với đường dẫn tương đối trong các bộ kê khai của chúng.

### Phiên bản 0.3.0
* Sửa lỗi: mục lục trong sách epub với bộ kê khai được mã hóa URL.
* Sửa lỗi: điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Sửa lỗi: sử dụng CPU cao trong tài liệu có tiêu đề dài do một hồi quy trong wxWidgets.
* Sửa lỗi: tải các tệp văn bản UTF-8.
* Sửa lỗi: các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Sửa lỗi: một sự cố khi thoát ứng dụng trong một số trường hợp.
* Đã thêm một hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt ngắt dòng từng từ!
* Hiện có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án nhà tài trợ này ở dưới cùng của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown hiện sẽ luôn có tiêu đề, và Paperback hiện có thể tải hầu hết mọi tệp Markdown.
* Tài liệu PDF hiện sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển đổi thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF đáng tin cậy hơn nhiều trên toàn bộ.
* Bạn hiện chỉ có thể chạy một phiên bản Paperback tại một thời điểm. Chạy paperback.exe với tên tệp khi nó đang chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bạn hiện có thể nhấn delete trên một tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách các tài liệu đã mở của bạn.
* Sửa lỗi: các phím tắt tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ.
* Paperback hiện sẽ loại bỏ các dấu gạch ngang mềm không cần thiết từ đầu ra văn bản.
* Sửa lỗi: điều hướng tiêu đề đôi khi đặt bạn ở ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa lỗi tải epub có tên tệp được mã hóa URL trong kê khai của chúng.
* Đã sửa lỗi tải sách epub 3 có XHTML nhúng bên trong chúng.
* Giờ đây sẽ phát một tin nhắn nếu tài liệu không hỗ trợ mục lục hoặc phần, thay vì vô hiệu hóa các mục menu.
* Đã thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu được mở gần đây nhất của bạn, và nhấn enter trên một trong số chúng sẽ mở nó để đọc.
* Đã viết lại hoàn toàn hộp thoại Tìm kiếm, làm cho nó dễ sử dụng hơn nhiều, đồng thời cũng thêm lịch sử 25 lần tìm kiếm gần đây nhất của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đó giờ đây được ghi nhớ qua các lần khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Đã thêm `Shift+F1` để mở tệp readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
