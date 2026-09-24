<!-- machine-translated from doc/readme.md (source-hash: 4d3bd6acdc082011; sections: 84030068,db723a70,df2f4c18,14335443,d44bf4c8,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,fabb029c); please review and edit as needed -->

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

## Khả năng tương thích với Trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết đối với người dùng JAWS.

### JAWS và Màn hình Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi di chuyển tiến với các phím điều hướng của màn hình của bạn. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải điều gì trong chính Paperback, và lỗi này đã mất khá lâu để tìm ra cách sửa chữa do sự nhiệt tình của Vispero trong việc phản hồi các vấn đề với phần mềm mã nguồn mở.

Cách khắc phục, cuối cùng được tìm thấy thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn văn đang hoạt động thay vì tiến lên. Với cả hai cài đặt được thiết lập, di chuyển pan sẽ hoạt động bình thường.

### JAWS và các thông báo của Paperback

Paperback nói những điều như "No pages." hoặc "This document has no audio." như các thông báo về khả năng tiếp cận, đó là điều cho phép trình đọc màn hình phát âm chúng trên bất kỳ những gì nó đang nói. JAWS chỉ hoạt động trên những thông báo đó khi "Enable accessible notification events" được bật cho ứng dụng, và trên một số máy tính nó không phải vậy.

Nếu JAWS không nói gì khi bạn nhấn một phím mà nó nên báo cáo điều gì, hãy mở Settings Center với Paperback ở trước (`Insert+6`), tìm kiếm "notification", và đánh dấu "Enable accessible notification events". Điều đó ghi cài đặt vào `paperback.jcf`, vì vậy nó chỉ áp dụng cho Paperback.

## Các định dạng tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Lưu trữ sách truyện tranh (`.cbz`)
* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Các trang hướng dẫn, cả `man` và BSD `mdoc` (`.1` đến `.9`, `.man`, `.roff` và các dạng nén gzip của mỗi trang)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách nói M4B (`.m4b`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Sách nói MP3 (`.mp3`)
* Trình chiếu OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Trình chiếu PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu reStructuredText (`.rst`, `.rest`)
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

1.0 là bản phát hành đầu tiên trên cả năm nền tảng: Windows, macOS, Linux, iOS và Android, với các ứng dụng iOS và Android có sẵn trên App Store và Google Play.

#### Đã thêm

##### Chung
* Hỗ trợ Linux, dưới dạng AppImage hoặc tar.gz, với tích hợp desktop để các tài liệu mở từ trình quản lý tệp của bạn.
* Đánh dấu phần đầu của một lựa chọn bằng `Alt+F9`, sao chép mọi thứ từ đó đến nơi bạn đạt được bằng `Alt+F10`, và quay lại dấu đó bằng `Alt+Shift+F9`, để sao chép một khoảng văn bản dài mà không cần phải nhấn shift và mũi tên. Cả ba đều nằm dưới Tools > Select and copy.
* Phím tắt `=` giờ đây công bố trang cũng như phần trăm, ví dụ "15%, trang 30", và giữ nguyên như trước đối với các tài liệu không có số trang.
* Hộp About bây giờ hiển thị giấy phép Paperback và mọi người dịch.
* Một bản dịch tiếng Ukraina.

##### Định dạng mới
* Kho lưu trữ truyện tranh (`.cbz`).
* Sách nói M4B, được chia thành các chương của chúng.
* Trang hướng dẫn, cả `man` và BSD `mdoc`, được nén gzip hoặc không.
* Sách nói MP3, được chia thành các chương khi tệp có chúng.
* Tài liệu reStructuredText.
* Tệp Windows Write (`.wri`).
* Tệp WinHelp (`.hlp`).
* Tài liệu Word 6 và Word 95.

##### OCR
* Các trang PDF được quét giờ đây có thể được nhận dạng bằng OCR tích hợp trong Windows và macOS. Nhấn `Enter` trên một trang được quét để nhận dạng nó, hoặc sử dụng Batch OCR (`Ctrl+Shift+O`) cho một loạt trang.

##### Điều hướng
* Các công thức MathML trong EPUB và HTML được hiển thị dưới dạng AsciiMath bằng MathCAT. Sử dụng `M` hoặc `Shift+M` để điều hướng công thức, sau đó nhấn `Enter` hoặc `Space` để mở MathML gốc trong Formula View.
* Một nút Find All trong hộp thoại Find, liệt kê mọi dòng với một kết quả khớp để bạn có thể nhảy thẳng đến dòng bạn muốn.
* Các chế độ xem Bảng, Danh sách và Trang trong danh sách phần tử (`F7`).
* Go to Line, Go to Page và Go to Percent giờ đây chấp nhận `+n` và `-n` để di chuyển tương đối với nơi bạn đang ở.
* Các sách EPUB, MOBI và CHM không có tiêu đề riêng của chúng giờ đây nhận được điều hướng tiêu đề từ bảng nội dung của chúng.
* Các sách KF8 (AZW3) giờ đây hỗ trợ điều hướng phần.
* Các trang EPUB chỉ là một bức ảnh giờ đây hiển thị một dòng cho nó, để bạn có thể hạ cánh trên chúng thay vì bỏ qua.

##### Sách nói
* Các điều khiển tốc độ phát lại, từ nửa tốc độ đến ba lần nhanh hơn. Sử dụng `Ctrl+Shift+.` và `Ctrl+Shift+,`, hoặc menu Tools.
* Các dấu trang và ghi chú trong các sách chỉ âm thanh giờ đây ghi nhớ thời gian chính xác mà bạn đặt chúng.
* Vị trí tiếp theo và vị trí trước (`Alt+Left` và `Alt+Right`) giờ đây hoạt động trong các sách nói.
* Tiến trình thông qua một sách nói giờ đây được đo bằng bản ghi của nó, vì vậy Go to Percent và thanh trạng thái khớp với mức độ bạn thực sự đi qua.

##### Tài liệu gần đây
* Một mục Clear Recent Documents trong trình đơn con Recent Documents.

##### Tài liệu PDF
* Một cài đặt để giữ mỗi dòng của PDF riêng biệt, thay vì kết hợp chúng thành các đoạn.
* Hình ảnh và hình vẽ trong PDF giờ đây được công bố.
* Các PDF mang cấu trúc đọc nhưng không gắn thẻ bất kỳ hình ảnh nào của chúng giờ đây công bố những hình ảnh đó, thay vì bỏ chúng ra khỏi cuốn sách hoàn toàn.

##### Web View
* Bất kỳ tài liệu nào bây giờ có thể được mở trong chế độ xem web, không chỉ EPUB, HTML và Markdown.

##### Khả năng đọc
* Các tiêu đề giờ đây được vẽ ở một kích thước phù hợp với mức độ của chúng, và hình ảnh và bảng được tách biệt khỏi văn bản xung quanh chúng.

##### pb
* `pb --list-formats` liệt kê mọi định dạng mà pb có thể đọc.
* pb giờ đây nói biết file nào mà nó không thể đọc được và tại sao.

#### Đã sửa

##### Chung
* Đã sửa lỗi khi đóng Paperback.
* Đóng Paperback giờ đây sẽ ẩn cửa sổ ngay lập tức, thay vì để nó ở trên màn hình khi đang lưu.
* Mở tài liệu không còn để Reopen Last Closed bật khi không có gì để mở lại.
* Paperback không còn thử lại các tài liệu gần đây bị mất, và hạn chế số lượng tài liệu gần đây được lưu trữ.
* Tệp cài đặt INI cũ giờ đây sẽ bị xóa sau khi được chuyển sang định dạng mới.
* Tiêu đề của các hộp thoại phông chữ và màu sắc, cũng như menu Xuất As trong Tiếng Việt, giờ đây được dịch.
* Cập nhật giờ đây sẽ đưa cửa sổ khởi động lại ra phía trước, thay vì để nó phía sau các cửa sổ khác trong Alt+Tab.
* Ngắt dòng giờ đây được áp dụng ngay lập tức trên các tài liệu lớn, thay vì tải lại toàn bộ.

##### Điều hướng
* `Alt+Left` giờ đây quay lại nơi bạn đã nhảy từ, thay vì đến một vị trí cũ hơn.
* Âm thanh dấu trang giờ đây chỉ phát khi bạn di chuyển qua một dấu trang, không phải khi bạn đặt chân trên dòng nó ở.
* Đóng mục lục, danh sách phần tử và các hộp thoại Go giờ đây sẽ đưa bạn thẳng đến dòng bạn đặt chân trên, thay vì khiến bạn phải nghe trình đọc màn hình đọc lại cửa sổ.
* Go to Line, Go to Page và Go to Percent giờ đây từ chối các số nằm ngoài tài liệu thay vì đi đến một nơi khác một cách âm thầm.
* NVDA không còn cắt ngắt thông báo khi tài liệu không có trang.
* Nhấn OK trong mục lục mà không di chuyển giờ đây sẽ đi đến mục đã được chọn.
* Mục lục, danh sách phần tử và danh sách dấu trang không còn chậm hay bị đóng băng trên các cuốn sách có hàng ngàn mục nhập.
* Phím Up và Down arrow giờ đây ghi nhớ cột của chúng cho mỗi tài liệu, thay vì mang nó qua khi bạn chuyển đổi tab.

##### Sách nói
* Phát lại âm thanh giờ đây sử dụng `Control+Space` trên macOS, vì `Command+Space` thuộc về Spotlight.

##### Tài liệu PDF
* Đã sửa các tệp PDF được xuất từ Apple Pages được đọc dưới dạng văn bản thuần túy, mà không có tiêu đề và danh sách nào được viết.
* Đã sửa các đoạn PDF và tiêu đề tách ở mỗi dòng, và các từ tách rời ở các khoảng trắng.
* Đã sửa các tiêu đề PDF được đánh số chạy lại vào một tiêu đề.
* Đã sửa các tệp PDF mà cây cấu trúc dẫn đến không có văn bản mở trống.
* Đầu trang và chân trang không còn được đọc trên mỗi trang của các tệp PDF không được gắn thẻ.
* Các tệp PDF gắn thẻ đầu trang và chân trang của chúng dưới dạng văn bản thường không còn lặp lại tiêu đề và số trang giữa hai đoạn trên mỗi trang.
* Các tệp PDF giờ đây hiển thị tiêu đề thực của chúng, thay vì tên tệp của chúng.
* Các dòng được đặt bằng phông chữ monospaced, như mã, không còn được nối vào các đoạn.

##### Sách MOBI/AZW3
* Các cuốn sách MOBI lớn không còn hết bộ nhớ, và không còn bị cắt ngắn sau 20 MB.
* Các cuốn sách MOBI và AZW3 giờ đây mở nhanh hơn nhiều.
* Đã sửa các cuốn sách MOBI mất danh sách chương của chúng.
* Đã sửa văn bản bị hỏng trong đó các cuốn sách MOBI chuyển từ bản ghi này sang bản ghi khác.

##### Web View
* Web view không còn tải toàn bộ một cuốn sách khổng lồ cùng một lúc.
* Web view giờ đây hiển thị các tài liệu toàn bộ khi trình đọc hiển thị chúng toàn bộ, thay vì chỉ một phần của chúng.

##### Các định dạng khác
* Các cuốn sách FictionBook (.fb2) được viết bằng windows-1251, chiếm phần lớn trong số đó, giờ đây mở thay vì không đọc được.
* Các cuốn sách FictionBook sử dụng không gian tên hoặc thực thể HTML mà chúng chưa bao giờ khai báo giờ đây mở, thay vì bị từ chối vì bị hỏng.
* Các cuốn sách trong mã hóa cũ giờ đây mở nhanh hơn nhiều.
* Đã sửa một số tệp văn bản Tiếng Trung mở dưới dạng văn bản bị hỏng.
* Các tệp OpenDocument được bảo vệ bằng mật khẩu giờ đây yêu cầu mật khẩu của chúng, thay vì được báo cáo là bị hỏng.
* Các tệp PowerPoint cũ được bảo vệ bằng mật khẩu giờ đây mở, và các slide PowerPoint cũ không còn mất văn bản của chúng.
* Các tệp văn bản thuần túy được lưu bằng phần mở rộng `.rtf` giờ đây mở dưới dạng văn bản, thay vì thất bại với lỗi.
* Các từ điều khiển RTF không còn hiển thị dưới dạng văn bản.

#### iOS và Android

Các ứng dụng iOS và Android mở mọi định dạng mà máy tính để bàn mở, và bao gồm:

* Đọc to, với lựa chọn giọng nói, tốc độ và cao độ của bạn, điều khiển tốc độ đọc ngay trên thanh đọc, và một pauze tùy chọn giữa các đoạn.
* Phát lại sách nói DAISY, M4B và MP3, tiếp tục chạy trong nền và từ màn hình khóa.
* Điều hướng theo tiêu đề, trang, liên kết, bảng, danh sách và hơn thế nữa từ thanh đọc, cộng với mục lục và Tìm kiếm.
* Hẹn giờ ngủ, số từ và xuất tài liệu, cộng với từ điển đọc trên iOS.
* Tùy chọn kích thước văn bản, khoảng cách và văn bản độ tương phản cao.
* Phím tắt phù hợp với máy tính để bàn.

### Phiên bản 0.9.2
* Sách nói không còn khiến trình đọc màn hình của bạn đọc một loạt dấu cách khi bạn tập trung vào trường văn bản.
* Sách nói giờ đây đặt tên tệp khi bạn bước qua chúng theo phần.
* Sách nói giờ đây báo cáo độ dài thực của chúng, thay vì tuyên bố mọi tệp trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã theo dõi liên kết bên trong nó.
* Sao chép sau Chọn tất cả giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần được tải hiện tại.
* Find giờ đây cắt thẳng đến dòng nó tìm thấy, thay vì khiến bạn nghe trình đọc màn hình đọc lại cửa sổ khi tiêu điểm quay lại cuốn sách.
* Đã sửa EPUB mang khối ZIP64 lạc từ chối mở bằng "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại khởi đầu của chúng trong khi trình đọc màn hình đọc liên tục thông qua chúng.
* Liên kết trong WebView giờ đây đưa bạn đến phần mà chúng trỏ đến, thay vì thất bại bằng "File not found".
* Thông báo "Document reloaded" tự động không còn cắt trình đọc màn hình của bạn giữa chừng, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt giờ đây tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Windows giờ đây sẽ luôn hiển thị "Paperback" trong menu Mở với, thay vì dòng tagline đầy đủ của chương trình.
* Word Count và Document Info giờ đây hiển thị bao nhiêu tệp một cuốn sách nói nắm giữ, và bao lâu nó chạy trong tổng số.

### Phiên bản 0.9.1
* Âm thanh đánh dấu trang và ghi chú hiện phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Đã sửa lỗi dấu ngoặc kép, dấu gạch ngang em và các ký tự tương tự biến mất khỏi tài liệu RTF, làm cho các từ xung quanh chạy lại với nhau.
* Đã sửa lỗi hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản rối.
* Đã sửa lỗi trình đơn Tài liệu gần đây giữ các mục cũ cho đến khi điều gì đó khác xảy ra để xây dựng lại nó.
* Bộ tăng tốc bàn phím đã quay trở lại trong mọi bản dịch, do đó các menu của Nga có quyền truy cập bàn phím một lần nữa.
* Tài liệu CHM lớn hiện mở nhanh hơn tới bảy lần.
* Các tài liệu mở hiện được đăng ký với Windows, vì vậy chúng sẽ xuất hiện trong danh sách nhảy trên thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Tùy chọn đã được đổi tên thành Cài đặt, phù hợp với các ứng dụng di động và trên macOS, quy ước nền tảng.
* Paperback hiện lưu nhớ vị trí cửa sổ, kích thước và trạng thái tối đa hóa giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các thông báo đếm những điều đọc đúng cách bằng các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY hiện mở sách âm thanh hoàn chỉnh thay vì chỉ văn bản của nó.
* Tên hành động của hộp thoại Tùy chỉnh Phím tắt bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu bây giờ xuất hiện trước tiên trên thanh tiêu đề, vì vậy các cuốn sách mở có thể được phân biệt trên thanh tác vụ và `Alt+Tab`.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Được thêm vào

##### Chung
* Một công cụ CLI có tên pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn Xem nguồn để mở nguồn tài liệu trong một tab mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, nghĩa là bạn có thể tải sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều kỳ lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ Windows ARM64!
* Hỗ trợ macOS gốc!
* Bật tắt toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Nút định vị để định vị sách bị mất mà vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và được chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Tab khả năng đọc với các tùy chọn sau:
    * Ngắt dòng (được chuyển từ chung);
    * Hiển thị bảng nội tuyến (mới trong phiên bản này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ cái;
    * Căn chỉnh văn bản.
* Mục menu ngắt dòng và phím nóng tiếp theo.
* Một bật tắt để xác định cách bạn muốn bảng được hiển thị, và thống nhất cách bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo container.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để công bố tỷ lệ phần trăm hiện tại của bạn thông qua tài liệu.

##### Đánh dấu trang
* Đánh dấu trang tạm thời: bạn có thể có một mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Đếm từ
* Thời gian đọc ước tính trong hộp thoại đếm từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu có lựa chọn hoạt động khi bạn mở hộp thoại đếm từ, số lượng từ bạn đã chọn sẽ hiện được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, tiếng Phần Lan và tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình cập nhật
* Nút hủy cho hộp thoại cập nhật đang diễn ra.
* Trình cập nhật hiện xác thực tệp đã tải xuống chưa bị giả mạo.

##### Chế độ xem Web
* Chế độ xem web hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách âm thanh
* Khả năng phát các sách âm thanh, hiện hỗ trợ cả âm thanh DAISY (bao gồm âm thanh DAISY + văn bản) và các tệp zip của các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời kể, tìm kiếm về phía trước và phía sau, và điều chỉnh số tiền tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu đọc với phát lại âm thanh, đặt số tiền tìm kiếm âm thanh, và chọn liệu tìm kiếm vượt quá cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã sửa

##### Chung
* Các tài liệu được mã hóa bằng các bộ mã CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị đúng thay vì hiển thị thành một đống mojibake.
* "Mở lại tài liệu đóng lần cuối" cố gắng mở lại tệp readme đi kèm.
* Tab được chọn của bạn không được lấy tiêu điểm đúng cách sau khi khởi động lại Paperback.
* Cách xử lý tệp trên ổ đĩa mạng Windows của Paperback: nhấn hiển thị tệp trong thư mục bây giờ sẽ lấy tiêu điểm đúng cách tệp trên bộ lưu trữ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp `.paperback` sẽ không còn được tải ép buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy tệp.
* Mở thư mục chứa bây giờ sẽ lấy tiêu điểm đúng cách tệp trong trình khám phá.
* Mở tệp readme sẽ tôn trọng ngôn ngữ được chọn của bạn.
* Giao diện người dùng của Paperback sẽ bây giờ được mở rộng đúng cách trên các màn hình có độ phân giải cao.
* Menu bây giờ sẽ cập nhật đúng cách và tiêu điểm sẽ chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang một phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ bây giờ được đọc khi chuyển đổi giữa các tab.
* Giảm sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục cho mỗi ký tự nội bộ.

##### Hộp thoại Tất cả Tài liệu
* Phím Escape không đóng các hộp thoại Thông tin Tài liệu và Tất cả Tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở qua `Shift+F1`.
* Xóa tài liệu từ hộp thoại gần đây bây giờ cũng sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn bây giờ sẽ được bảo tồn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh Dấu trang/ghi chú bây giờ sẽ phát đúng cách khi bạn điều hướng qua một từ chứa một từ.

##### Khả năng đọc
* Áp dụng ngắt dòng từ đưa bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh bây giờ sẽ hiển thị đúng cách trong webview được nhúng.

##### Trình cập nhật
* Trình cập nhật bây giờ sẽ hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích các tài liệu RTF có các ký tự không phải Latinh trong đó.
* RTF `\pict` groups để dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo filepos trong sách Mobi tách các thẻ HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Cải thiện phân tích cú pháp AZW3 đáng kể.

##### Tài liệu Word
* Tài liệu Word có tên kiểu cụ thể của ngôn ngữ không hiển thị các tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra dấu ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback bây giờ quay lại trích xuất văn bản thuần túy cho các tệp PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback bị sập khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang vào sách epub.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại hỗ trợ Word cũ, Word hiện đại và Powerpoint hiện đại, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Word cũ!
* Đã thêm hỗ trợ cho các bài thuyết trình Powerpoint cũ!
* Đã thêm hỗ trợ cho sách mobi và AZW3!
* Đã thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Đã thêm phím tắt `Ctrl+Q` để thoát ứng dụng.
* Đã thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng bây giờ sẽ được hiển thị đúng cách.
* Các tài liệu CHM bây giờ hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Sửa go to page bị lệch 1.
* Sửa phím Escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Sửa menu ngữ cảnh của trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Sửa tài liệu sai đôi khi được lấy tiêu điểm khi mở tài liệu từ dòng lệnh.
* Các tệp PDF chỉ có hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Bây giờ có thể điều hướng qua hình ảnh và hình vẽ bằng `g`/`Shift+G` và `f`/`Shift+F`, tương ứng.
* Paperback sẽ tôn trọng cài đặt chế độ tối ứng dụng của bạn.
* Đã xóa hỗ trợ DAISY XML vì nó không còn cần thiết nữa.
* Chuyển trở lại điều hướng chữ cái đầu tiên Win32 gốc trong cây mục lục.
* Hộp thoại tải lỗi bây giờ hiển thị các thông báo lỗi chi tiết hơn.
* Webview bây giờ sẽ mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang vào tài liệu RTF!
* Sửa một lỗi trong đó mở webview trong epub chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Sửa một lỗi trong đó trình phân tích RTF sẽ không đặt một dấu cách giữa các từ trong các trường hợp hiếm.
* Sửa các đoạn được tách thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF bây giờ có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab RTF và dòng mới bây giờ được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Chuyển trở lại thư viện pdfium được thử và đúng để phân tích PDF, làm cho kết xuất PDF lạnh lùng hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Đã thêm `Ctrl+Shift+T` để mở lại tài liệu đóng lần cuối.
* Hộp thoại Tất cả Tài liệu bây giờ hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Sửa một vài lỗi với trình phân tích RTF.
* Sửa các đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp qua một phiên bản Paperback thứ hai.
* Sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Sửa quá trình tải tài liệu chậm khi mở các tệp lớn.
* Sửa việc bản địa hóa các nút Có/Không trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung Quốc đơn giản hóa và tiếng Việt!
* Thêm trình cập nhật tự động sẽ thay thế phiên bản Paperback hiện tại của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn khi đạt được dấu trang hoặc ghi chú, cảm ơn Andre Louis về các âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho tài liệu DAISY XML.
* Thêm hỗ trợ cho tệp Flat Open Document Text!
* Thêm hỗ trợ cho bản trình bày Flat Open Document!
* Thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Sửa lỗi khôi phục cửa sổ Paperback từ khay hệ thống.
* Sửa lỗi tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Chế độ xem Web.
* Sửa lỗi bảng không hiển thị đúng trong các tệp Markdown.
* Các tệp PDF chỉ có hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một tệp.
* Nhúng đúng thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy cao hơn, tốc độ nhanh hơn và ít DLL hơn.
* Viết lại toàn bộ ứng dụng bằng Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu bối cảnh của điều khiển văn bản sẽ bây giờ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một bảng trong chế độ xem web.
* Thêm tính năng hiển thị web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Xóa tất cả vào hộp thoại Tất cả Tài liệu.
* Trình kiểm tra cập nhật bây giờ hiển thị ghi chú phát hành khi có phiên bản mới.
* Sửa lỗi khôi phục cửa sổ từ khay hệ thống.
* Sửa lỗi dịch các nút Có/Không trong hộp thoại xác nhận.
* Sửa lỗi tải cấu hình khi chạy với tư cách quản trị viên.
* Sửa lỗi xử lý nhận xét trong tài liệu XML và HTML.
* Sửa lỗi phân tích cú pháp TOC trong sách Epub 2.
* Sửa lỗi điều hướng đến mục tiếp theo có cùng ký tự trong mục lục.
* Sửa lỗi hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Sửa lỗi TOC epub đôi khi ném bạn đến mục sai.
* Sửa lỗi các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Sửa lỗi lỗi off-by-one trong điều hướng liên kết.
* Sửa lỗi một số cuốn sách có khoảng trắng ở cuối dòng của chúng.
* Sửa lỗi các vấn đề phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử bây giờ được tắt một cách chính xác khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều cải cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể điều hướng bằng các phím mũi tên alt+left/right.
* Thêm danh sách phần tử! Hiện tại nó chỉ hiển thị một cây tất cả các tiêu đề trong tài liệu của bạn hoặc một danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Sửa lỗi các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Sửa lỗi phân tích cú pháp TOC Epub chứa các đường dẫn tương đối.
* Sửa lỗi một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Sửa lỗi tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Sửa lỗi bạn không thể sử dụng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Cải thiện xử lý các tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0
* Đã thêm một tùy chọn mới để hiển thị menu chuyển đến dưới dạng nhỏ gọn hơn vào hộp thoại tùy chọn, được bật theo mặc định.
* Đã thêm tùy chọn để điều hướng theo các phần tử cấu trúc có thể bao quanh.
* Đã thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại.
* Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập bằng `Ctrl+Shift+S`.
* Đã thêm hỗ trợ phân tích cú pháp sách điện tử FB2!
* Đã thêm hỗ trợ phân tích cú pháp bản trình bày OpenDocument!
* Đã thêm hỗ trợ phân tích cú pháp tệp OpenDocument Text!
* Có thể tạo dấu trang để đánh dấu toàn bộ một dòng hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn nào hoạt động khi đặt dấu trang, hành vi sẽ giống như trước phiên bản 0.6 và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Dấu trang hiện có thể có ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú bằng N và `Shift+N`, hoặc bật hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ những dấu trang không có ghi chú được chọn bằng các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Các cuốn sách Epub chứa nội dung HTML giả dạng XML sẽ được xử lý đúng cách.
* Đã sửa việc tải các tài liệu Markdown lớn.
* Đã sửa việc nhấn dấu cách trong chế độ xem cây mục lục kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa lỗi kiểm soát văn bản đôi khi không lấy lại tiêu điểm khi quay lại cửa sổ Paperback.
* Đã sửa lỗi trường văn bản trong hộp thoại chuyển đến phần trăm không cập nhật giá trị thanh trượt.
* Đã sửa việc hiển thị ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown sẽ được hiển thị đúng cách.
* Nếu tải một cuốn sách với tham số dòng lệnh khi có phiên bản Paperback hiện tại đang chạy, bạn sẽ không còn gặp lỗi nếu việc tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình sẽ được tải và lưu đúng cách.
* Bây giờ có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Bây giờ có thể nhập và xuất dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có phần mở rộng `.paperback`. Nếu tìm thấy một tệp như vậy trong cùng thư mục với một tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng thủ công bằng một mục trong menu công cụ.
* Các liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng k và `shift+k` để di chuyển về phía trước và phía sau thông qua chúng, và nhấn enter để mở/kích hoạt một liên kết.
* Nhiều refactors nội bộ, làm cho ứng dụng nhanh hơn và nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi hiển thị.
* Điều hướng theo danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng L và `Shift+L` để chuyển đến các danh sách, và I và `Shift+I` để chuyển qua các mục danh sách.
* Phím xóa trên bàn phím số hiện hoạt động để xóa tài liệu khỏi thanh tab ngoài phím xóa thông thường.
* Paperback hiện có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này được tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback hiện hoàn toàn có thể dịch được! Danh sách các ngôn ngữ nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang liên tục phát triển!
* Paperback hiện có một trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ hiển thị bảng mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu được mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng drastically! Thay vì chỉ hiển thị cho bạn 10 tài liệu cuối cùng mà bạn đã mở, nó hiện sẽ hiển thị cho bạn một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn đã từng mở có thể truy cập thông qua một hộp thoại nhỏ.
* Các cải tiến nhỏ khác nhau đối với các trình phân tích cú pháp trên toàn bộ bảng, bao gồm đặt một dòng trống giữa các slide trong bản trình bày PPTX, sửa xử lý ký tự xuống dòng bên trong các đoạn văn trong tài liệu word và thêm các dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho các bài thuyết trình PowerPoint!
* Đã sửa các mục menu nhất định không bị tắt khi không có tài liệu nào mở.
* Đã sửa hướng của thanh trượt phần trăm chuyển đến.
* Đã sửa mục lục trong sách Epub có đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng bị loại bỏ từ các tiêu đề XHTML theo những cách lạ lùng.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục riêng của nó từ cấu trúc các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại `ctrl+t`.
* Tài liệu HTML hiện sẽ có tiêu đề được đặt trong thẻ title, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Đã chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không còn các DLL trình đọc màn hình nào được vận chuyển cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Đã chuyển các thư viện zip để cho phép mở một loạt tệp epub rộng hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được hoàn toàn làm lại, và bây giờ nó cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại đi đến phần trăm bây giờ bao gồm một trường văn bản cho phép bạn thủ công nhập một phần trăm để nhảy đến.
* Bộ phân tích HTML hiện sẽ nhận dạng `dd`, `dt` và `dl` là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo toàn chính xác một lần nữa.
* Ký tự không ngắt Unicode hiện được xem xét khi loại bỏ các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp không được nhận dạng mỗi lần tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng menu bắt đầu tùy chọn cho trình cài đặt.
* Mục lục hiện sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có mục con và cha mẹ có cùng văn bản ở cùng vị trí, bạn sẽ chỉ thấy mục cha mẹ.
* Đã sửa mục lục trong các tài liệu CHM nhất định.
* Đã sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM hiện sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ đánh dấu! Bạn có thể có bao nhiêu dấu trang tùy thích trong bao nhiêu tài liệu tùy thích. Bạn có thể nhảy tiến và lùi qua chúng bằng `b` và `shift+b`, đặt một cái bằng `control+shift+b`, và mở hộp thoại để nhảy đến một dấu trang cụ thể bằng `control+b`.
* Đã thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Nó sẽ hiển thị cho bạn dòng hiện tại, ký tự và phần trăm đọc của bạn.
* Các nhận xét HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ giải quyết nó một cách chính xác.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập bằng `control+shift+g`.
* Các tài liệu không có tiêu đề hoặc tác giả đã biết hiện sẽ luôn có giá trị mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ nên ghi vào đĩa khi tuyệt đối cần thiết.
* Tài liệu bạn đã tập trung khi bạn đóng Paperback bây giờ được ghi nhớ trong các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại đi đến dòng và đi đến trang hiện sẽ được vệ sinh nghiêm ngặt hơn.
* Đã sửa điều hướng mục lục trong sách epub 3 có đường dẫn tương đối trong kê khai của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub với kê khai được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Đã sửa mức sử dụng CPU cao trong tài liệu có tiêu đề dài do một suy thoái trong wxWidgets.
* Đã sửa tải tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa một sự cố khi thoát ứng dụng trong những trường hợp nhất định.
* Đã thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt ngắt dòng từ!
* Bây giờ có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án tài trợ này ở cuối trang chính của kho lưu trữ GitHub.
* Tài liệu Markdown bây giờ sẽ luôn có tiêu đề, và Paperback hiện sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF bây giờ sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Đã chuyển các thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều trên toàn bộ.
* Bây giờ bạn chỉ có thể chạy một phiên bản Paperback cùng một lúc. Chạy `paperback.exe` với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bây giờ bạn có thể nhấn xóa trên một tài liệu trong kiểm soát tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại đi đến trang.
* Cho phép chuyển tab từ nội dung tài liệu đến danh sách tài liệu đã mở của bạn.
* Đã sửa các phím tắt tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ.
* Paperback hiện sẽ loại bỏ gạch nối mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn trên ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Những phím tắt này được thiết kế để hoạt động tương tự như một trình đọc màn hình.
* Đã sửa lỗi tải các epub có tên tệp được mã hóa URL trong các bản kê khai của chúng.
* Đã sửa lỗi tải sách epub 3 có XHTML được nhúng bên trong.
* Một tin nhắn sẽ được phát ra nếu tài liệu không hỗ trợ mục lục hoặc các phần, thay vì các mục menu bị vô hiệu hóa.
* Đã thêm menu tài liệu gần đây! Nó hiện lưu trữ 10 tài liệu đã mở cuối cùng của bạn, và nhấn Enter trên một trong những tài liệu này sẽ mở nó để đọc.
* Đã viết lại hoàn toàn hộp thoại Tìm kiếm, giúp nó dễ sử dụng hơn nhiều, đồng thời thêm lịch sử 25 lần tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu đã mở trước đây hiện được nhớ qua các lần khởi động lại ứng dụng. Điều này có thể được cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Đã thêm `Shift+F1` để mở tệp readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Phiên bản phát hành đầu tiên.
