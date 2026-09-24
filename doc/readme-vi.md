<!-- machine-translated from doc/readme.md (source-hash: 06f1089b5f255d98; sections: 84030068,db723a70,df2f4c18,14335443,1387e8b7,3887c286,94527a25,ca4819ea,a9eba369,e9860ee8,80b9b9ca); please review and edit as needed -->

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

## Khả năng Tương thích với Trình Đọc Màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có hai vấn đề đã biết đối với người dùng JAWS.

### JAWS và Màn hình Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi cuộn về phía trước bằng các phím điều hướng của màn hình. Lệnh đọc đoạn văn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải là vấn đề trong chính Paperback, và đó là một vấn đề mất khá lâu để tìm ra giải pháp do sự chậm chạp của Vispero trong việc phản hồi các vấn đề với phần mềm mã nguồn mở.

Giải pháp khắc phục, cuối cùng được tìm thấy thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn văn hoạt động thay vì tiến lên. Với cả hai cài đặt này, cuộn sẽ hoạt động chính xác.

### JAWS và Các thông báo của Paperback

Paperback nói những điều như "No pages." hoặc "This document has no audio." như các thông báo accessibility, cho phép trình đọc màn hình phát âm chúng qua bất kỳ thứ gì nó đang nói. JAWS chỉ hoạt động dựa trên những thông báo đó khi "Enable accessible notification events" được bật cho ứng dụng, và trên một số máy tính thì không.

Nếu JAWS không nói gì khi bạn nhấn một phím mà nó sẽ báo cáo điều gì đó, hãy mở Settings Center với Paperback ở phía trước (`Insert+6`), tìm kiếm "notification", và đánh dấu "Enable accessible notification events". Thao tác này ghi cài đặt vào `paperback.jcf`, vì vậy nó chỉ áp dụng cho Paperback.

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

1.0 là bản phát hành đầu tiên trên cả năm nền tảng: Windows, macOS, Linux, iOS và Android, với các ứng dụng iOS và Android trong App Store và Google Play.

#### Được thêm

##### Tổng quát
* Hỗ trợ Linux dưới dạng AppImage hoặc tar.gz, với tích hợp máy tính để bàn để tài liệu mở từ trình quản lý tệp của bạn.
* Đánh dấu đầu của một lựa chọn bằng `Alt+F9`, sao chép mọi thứ từ đó đến nơi bạn đã đến bằng `Alt+F10`, và quay lại dấu bằng `Alt+Shift+F9`, để sao chép một khoảng văn bản dài mà không cần dùng shift-arrow. Cả ba đều nằm dưới Tools > Select and copy.
* Phím tắt `=` hiện thông báo trang cũng như phần trăm, ví dụ "15%, trang 30", và giữ nguyên như trước đối với tài liệu không có số trang.
* Hộp About hiện hiển thị giấy phép của Paperback và mọi dịch giả.
* Một bản dịch tiếng Ukraina.

##### Định dạng mới
* Lưu trữ truyện tranh (`.cbz`).
* Sách nói M4B, được chia thành các chương của chúng.
* Trang hướng dẫn, cả `man` và BSD `mdoc`, có nén gzip hoặc không.
* Sách nói MP3, được chia thành các chương khi tệp có chúng.
* Tài liệu reStructuredText.
* Tệp Windows Write (`.wri`).
* Tệp WinHelp (`.hlp`).
* Tài liệu Word 6 và Word 95.

##### OCR
* Các trang PDF được quét hiện có thể được nhận dạng bằng OCR được tích hợp sẵn trong Windows và macOS. Nhấn `Enter` trên một trang được quét để nhận dạng nó, hoặc sử dụng Batch OCR (`Ctrl+Shift+O`) cho một loạt trang.

##### Điều hướng
* Các công thức MathML trong EPUB và HTML được hiển thị dưới dạng AsciiMath bằng MathCAT. Sử dụng `M` hoặc `Shift+M` để điều hướng các công thức, sau đó nhấn `Enter` hoặc `Space` để mở MathML ban đầu trong Formula View.
* Nút Find All trong hộp thoại Find, liệt kê mọi dòng có kết quả khớp để bạn có thể nhảy trực tiếp đến dòng bạn muốn.
* Chế độ xem Tables, Lists và Pages trong danh sách các phần tử (`F7`).
* Go to Line, Go to Page và Go to Percent hiện nhận `+n` và `-n` để di chuyển tương đối so với vị trí bạn đang ở.
* Sách EPUB, MOBI và CHM không có các tiêu đề riêng của chúng hiện nhận điều hướng tiêu đề từ mục lục của chúng.
* Sách KF8 (AZW3) hiện hỗ trợ điều hướng phần.
* Các trang EPUB chỉ là hình ảnh hiện hiển thị một dòng cho nó, để bạn có thể hạ cánh trên chúng thay vì bỏ qua chúng.

##### Sách nói
* Kiểm soát tốc độ phát lại, từ nửa tốc độ đến nhanh gấp ba lần. Sử dụng `Ctrl+Shift+.` và `Ctrl+Shift+,`, hoặc menu Tools.
* Dấu trang và ghi chú trong các sách chỉ có âm thanh hiện ghi nhớ thời gian chính xác bạn đặt chúng.
* Vị trí tiếp theo và vị trí trước (`Alt+Left` và `Alt+Right`) hiện hoạt động trong sách nói.
* Tiến độ thông qua một sách nói hiện được đo bằng bản ghi của nó, vì vậy Go to Percent và thanh trạng thái khớp với cách bạn thực sự tiến hành.

##### Tài liệu gần đây
* Một mục Clear Recent Documents trong trình đơn con Recent Documents.

##### Tài liệu PDF
* Một cài đặt để giữ mỗi dòng của PDF riêng biệt, thay vì kết hợp chúng thành các đoạn.
* Hình ảnh và hình trong các tệp PDF hiện được thông báo.
* Các tệp PDF có cấu trúc đọc nhưng không gắn thẻ cho bất kỳ hình ảnh nào hiện thông báo những hình ảnh đó, thay vì bỏ qua chúng hoàn toàn khỏi sách.

##### Web View
* Bất kỳ tài liệu nào hiện có thể được mở trong chế độ xem web, không chỉ EPUB, HTML và Markdown.

##### Khả năng đọc
* Các tiêu đề hiện được vẽ ở kích thước phù hợp với cấp độ của chúng, và hình ảnh cũng như bảng được tách biệt với văn bản xung quanh chúng.

##### pb
* `pb --list-formats` liệt kê mọi định dạng mà pb có thể đọc.
* pb hiện cho biết tệp nào không thể đọc được và tại sao.

#### Đã sửa lỗi

##### Chung
* Một cuốn sách được mở lại khi khởi động sẽ đọc ngay lập tức, thay vì im lặng cho đến khi nó được đóng và mở lại.
* Một tài liệu có tệp bị mất có thể được xoá khỏi Tất cả tài liệu, thay vì ở lại trong danh sách cho dù bạn xác nhận bao nhiêu lần.
* Sửa lỗi khi đóng Paperback.
* Đóng Paperback sẽ ẩn cửa sổ ngay lập tức, thay vì để lại nó trên màn hình khi nó lưu.
* Các cuốn sách lớn với ít định dạng giờ đây mở trong khoảng nửa thời gian.
* Các thông báo được chọn từ một menu, chẳng hạn như "Tài liệu này không có âm thanh", không còn bị cắt đứt bởi trình đọc màn hình trước khi bạn nghe chúng.
* Mở một tài liệu không còn để Mở lại tài liệu đóng lần cuối được bật khi không có gì để mở lại.
* Paperback không còn thử lại các tài liệu trong danh sách gần đây của bạn bị mất, và giới hạn bao nhiêu tài liệu gần đây nó lưu trữ.
* Tệp cài đặt INI cũ giờ đây được xoá sau khi nó được chuyển sang định dạng mới.
* Tiêu đề của hộp thoại phông chữ và màu sắc, và menu Xuất ra dưới dạng trong tiếng Việt, giờ đây đã được dịch.
* Cập nhật giờ đây sẽ đưa cửa sổ được khởi động lại lên trước, thay vì để lại nó phía sau mọi cửa sổ khác trong Alt+Tab.
* Gói dòng giờ đây áp dụng ngay trên các tài liệu lớn, thay vì tải lại toàn bộ.

##### Điều hướng
* `Alt+Left` giờ đây quay lại nơi bạn nhảy từ, thay vì đến một vị trí cũ hơn.
* Các âm thanh đánh dấu giờ đây chỉ phát khi bạn di chuyển qua một đánh dấu, không phải khi bạn hạ cánh trên dòng mà nó ở trên.
* Đóng mục lục, danh sách phần tử và hộp thoại Đi tới giờ đây sẽ đưa bạn trực tiếp đến dòng bạn hạ cánh, thay vì yêu cầu bạn nghe trình đọc màn hình đọc lại cửa sổ.
* Đi tới dòng, Đi tới trang và Đi tới phần trăm giờ đây từ chối các số ngoài tài liệu thay vì im lặng đi đến nơi khác.
* NVDA không còn cắt đứt thông báo khi tài liệu không có trang.
* Nhấn OK trong mục lục mà không di chuyển giờ đây sẽ đi tới mục nhập đã được chọn.
* Mục lục, danh sách phần tử và danh sách đánh dấu không còn chậm hoặc đóng băng trên các cuốn sách có hàng nghìn mục nhập.
* Mũi tên Lên và Xuống giờ đây nhớ cột của chúng trên mỗi tài liệu, thay vì mang nó theo khi bạn chuyển đổi tab.

##### Sách nói
* Phát lại âm thanh giờ đây sử dụng `Control+Space` trên macOS, vì `Command+Space` thuộc về Spotlight.

##### Tài liệu PDF
* Sửa lỗi PDF được xuất từ Apple Pages đọc dưới dạng văn bản thuần túy, không có bất kỳ tiêu đề và danh sách nào được viết với.
* Sửa lỗi các đoạn PDF và tiêu đề chia nhỏ ở mỗi dòng, và các từ tách biệt ở các khoảng trắng.
* Sửa lỗi các tiêu đề PDF được đánh số chạy vào nhau thành một tiêu đề.
* Sửa lỗi PDF có cây cấu trúc dẫn đến không có văn bản mở trống.
* Các dòng được đặt trong phông chữ monospaced, như mã, không còn được nối thành các đoạn.
* Đầu trang và chân trang không còn được đọc trên mọi trang của PDF không được gắn thẻ.
* PDF gắn thẻ đầu trang và chân trang của chúng dưới dạng văn bản thông thường không còn lặp lại tiêu đề và số trang giữa hai đoạn trên mọi trang.
* PDF giờ đây hiển thị tiêu đề thực của chúng, thay vì tên tệp của chúng.

##### Sách MOBI/AZW3
* Các cuốn sách MOBI lớn không còn hết bộ nhớ, và không còn bị cắt đứt sau 20 MB.
* Sách MOBI và AZW3 giờ đây mở nhanh hơn nhiều.
* Sửa lỗi sách MOBI mất danh sách chương của chúng.
* Sửa lỗi văn bản bị kỳ lạ nơi sách MOBI vượt qua từ một bản ghi sang bản ghi tiếp theo.

##### Chế độ xem web
* Chế độ xem web không còn tải toàn bộ một cuốn sách khổng lồ cùng một lúc.
* Chế độ xem web giờ đây hiển thị tài liệu hoàn chỉnh khi trình đọc hiển thị chúng hoàn chỉnh, thay vì chỉ một phần của chúng.

##### Các định dạng khác
* Sách FictionBook (.fb2) được viết bằng windows-1251, hầu hết trong số đó, giờ đây mở thay vì không đọc được hoàn toàn.
* Sách FictionBook sử dụng không gian tên hoặc thực thể HTML mà chúng không bao giờ khai báo giờ đây mở, thay vì bị từ chối vì bị hỏng.
* Sách ở mã hóa cũ giờ đây mở nhanh hơn nhiều.
* Sửa lỗi một số tệp văn bản Trung Quốc mở dưới dạng văn bản bị kỳ lạ.
* Tệp OpenDocument được bảo vệ bằng mật khẩu giờ đây yêu cầu mật khẩu, thay vì được báo cáo là bị hỏng.
* Tệp PowerPoint cũ được bảo vệ bằng mật khẩu giờ đây mở, và các slide PowerPoint cũ không còn mất văn bản của chúng.
* Tệp văn bản thuần túy được lưu với phần mở rộng `.rtf` giờ đây mở dưới dạng văn bản, thay vì thất bại kèm theo lỗi.
* Các từ điều khiển RTF không còn xuất hiện dưới dạng văn bản.

#### iOS và Android

Các ứng dụng iOS và Android mở mọi định dạng mà máy tính để bàn làm, và bao gồm:

* Đọc to, với sự lựa chọn của bạn về giọng nói, tốc độ và cao độ, một điều khiển tốc độ phát ngay trên thanh đọc, và một khoảng tạm dừng tùy chọn giữa các đoạn.
* Phát lại sách nói DAISY, M4B và MP3, hoạt động trong nền và từ màn hình khóa.
* Điều hướng theo tiêu đề, trang, liên kết, bảng, danh sách và nhiều hơn nữa từ thanh đọc, cộng với mục lục và Tìm.
* Bộ hẹn giờ ngủ, số từ và xuất tài liệu, cộng với từ điển phát biểu trên iOS. Trên iOS, xuất qua bảng chia sẻ, vì vậy một cuốn sách có thể đi đến ứng dụng khác hoặc đến Tệp, ở định dạng khác hoặc chính xác như nó vốn có.
* Tùy chọn kích thước văn bản, khoảng cách và văn bản tương phản cao.
* Phím tắt bàn phím phù hợp với máy tính để bàn.

### Phiên bản 0.9.2
* Sách nói không còn làm trình đọc màn hình của bạn đọc một loạt khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách nói giờ đây đặt tên tệp khi bạn bước qua chúng theo phần.
* Sách nói giờ đây báo cáo thời lượng thực tế của chúng, thay vì tuyên bố mọi tệp trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã theo dõi một liên kết bên trong nó.
* Sao chép sau Select All giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần hiện tại được tải.
* Find giờ đây cắt thẳng đến dòng được tìm thấy, thay vì làm bạn chịu đựng trình đọc màn hình đọc lại cửa sổ khi tiêu điểm quay trở lại sách.
* Đã sửa EPUB có khối ZIP64 lạc lẫm từ chối mở với "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại phần bắt đầu của chúng trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView giờ đây đưa bạn đến phần mà chúng chỉ đến, thay vì không hoạt động với "File not found".
* Thông báo "Document reloaded" tự động không còn cắt trình đọc màn hình của bạn giữa chừng câu, thay vào đó chờ nó kết thúc những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt giờ đây đi qua các tùy chọn của nó theo thứ tự xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Windows giờ đây sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì thẻ tagline đầy đủ của chương trình.
* Word Count và Document Info giờ đây hiển thị có bao nhiêu tệp một sách nói chứa và thời lượng chạy tổng cộng của nó.

### Phiên bản 0.9.1
* Âm thanh Bookmark và ghi chú giờ đây phát trên macOS.
* Sách DAISY giờ đây phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng trong im lặng.
* Đã sửa dấu ngoặc kép cong, dấu gạch ngang em và các ký tự tương tự biến mất từ các tài liệu RTF, chạy các từ xung quanh chúng với nhau khi chúng đi.
* Đã sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản méo mó.
* Đã sửa menu Con gần đây giữ các mục cũ cho đến khi điều gì đó khác xảy ra để xây dựng lại nó.
* Các phím tắt bàn phím quay trở lại trong mỗi bản dịch, vì vậy menu của Tiếng Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn giờ đây mở nhanh hơn tới bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng hiển thị trong danh sách jump của taskbar và danh sách gần đây của menu Start.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và, trên macOS, quy ước của nền tảng.
* Paperback giờ đây ghi nhớ vị trí, kích thước cửa sổ và trạng thái tối đa của nó giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm những thứ đọc đúng cách trong các ngôn ngữ cần nhiều hơn một hình thức.
* Chọn ncc.html của sách DAISY giờ đây mở sách nói hoàn chỉnh thay vì chỉ văn bản của nó.
* Các tên hành động của hộp thoại Tùy chỉnh Phím tắt bàn phím giờ đây có thể được dịch.
* Tiêu đề tài liệu giờ đây xuất hiện trước tiên trong thanh tiêu đề, vì vậy các sách mở có thể được phân biệt trong taskbar và Alt+Tab.
* Hộp thoại cập nhật giờ đây được dịch.

### Phiên bản 0.9.0

#### Được Thêm

##### Tổng Quát
* Một công cụ CLI gọi là `pb`, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn View Source để mở nguồn của tài liệu trong một tab mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, nghĩa là bạn có thể tải các sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều kỳ lạ nào được tìm thấy với điều này.

##### Hỗ Trợ Nền Tảng
* Hỗ trợ Windows ARM64!
* Hỗ trợ macOS gốc!
* Một công tắc toàn màn hình.

##### Hộp Thoại Tất Cả Tài Liệu
* Một nút định vị để định vị các sách bị mất đã thay đổi đường dẫn của chúng.
* Một bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả các tài liệu.

##### Tùy Chọn và Khả Đọc
* Một tab khả đọc, với các tùy chọn sau:
    * Ngắt từ (được di chuyển từ tổng quát);
    * Hiển thị bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách ký tự;
    * Căn chỉnh văn bản.
* Một mục menu ngắt từ và phím nóng tiếp theo.
* Một công tắc để xác định cách bạn muốn bảng được hiển thị và thống nhất cách bảng được hiển thị trên các tài liệu.

##### Điều Hướng
* Hỗ trợ điều hướng theo vùng chứa.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để công bố tỷ lệ phần trăm hiện tại của bạn trong một tài liệu.

##### Dấu Trang
* Dấu trang tạm thời: bạn có thể có một dấu trang trên mỗi tài liệu, và chúng được lưu giữ. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Số Từ
* Thời gian đọc ước tính trong hộp thoại số từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại số từ, số lượng từ bạn đã chọn sẽ được hiển thị.

##### Phím Tắt Bàn Phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn Ngữ
* Tiếng Hà Lan, Tiếng Phần Lan và Tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình Cập Nhật
* Một nút hủy cho hộp thoại cập nhật đang diễn ra.
* Trình cập nhật hiện xác thực rằng tệp đã tải xuống chưa bị can thiệp.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ cho các sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách Âm Thanh
* Khả năng phát các sách âm thanh, hiện hỗ trợ cả DAISY audio (bao gồm DAISY audio + text) và zips của các tệp âm thanh.
* Phím tắt bàn phím và các mục menu để phát/tạm dừng lời kể, tìm kiếm về phía trước và phía sau, và điều chỉnh lượng tìm kiếm.
* Các tùy chọn để đồng bộ hóa mũi tên đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn liệu tìm kiếm qua cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài Liệu CHM
* Hỗ trợ cho danh sách, mục danh sách, hình vẽ và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã sửa

##### Chung
* Các tài liệu được mã hóa bằng bảng mã CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, giờ đây sẽ hiển thị đúng cách thay vì hiển thị dạng ký tự lỗi.
* "Mở lại lần cuối cùng" cố gắng mở lại tệp readme được đóng gói.
* Tab đã chọn của bạn không được lấy tiêu điểm đúng cách sau khi khởi động lại Paperback.
* Xử lý tệp trên ổ đĩa mạng Windows của Paperback: nhấn show file in folder giờ đây sẽ lấy tiêu điểm tệp đó trên bộ lưu trữ mạng đúng cách, và các đường dẫn không còn chứa các ký tự lạ nữa.
* Các tệp `.paperback` sẽ không còn bị tải bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một tệp.
* Mở thư mục chứa giờ đây sẽ lấy tiêu điểm tệp đã cho trong trình khám phá.
* Mở tệp readme giờ đây sẽ tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng Paperback giờ đây sẽ được tỷ lệ đúng cách trên màn hình có DPI cao.
* Menu giờ đây sẽ cập nhật đúng cách và tiêu điểm sẽ chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động giờ đây sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục cho từng ký tự nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng một tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở thông qua `Shift+F1`.
* Xóa các tài liệu khỏi hộp thoại gần đây giờ đây cũng sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ đây được bảo toàn sau khi xóa một tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu đã tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú giờ đây nên phát đúng cách một cách độc quyền khi bạn điều hướng qua một từ chứa một từ.

##### Khả năng đọc
* Áp dụng ngắt dòng chữ khiến bạn bắn đến đầu tài liệu của mình.

##### Chế độ xem Web
* Hộp thoại webview không được thay đổi kích thước và hiển thị ở kích thước ban đầu rất nhỏ.
* Hình ảnh giờ đây nên hiển thị đúng cách trong webview được nhúng.

##### Trình cập nhật
* Trình cập nhật giờ đây hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Đang tải sách DAISY với các khai báo mã hóa giả tạo.

##### Tài liệu RTF
* Phân tích cú pháp tài liệu RTF với các ký tự không phải Latin trong đó.
* RTF `\pict` nhóm để dữ liệu hình ảnh được nhúng không còn bị rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Filepos anchors trong sách Mobi chia tách các thẻ HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Cải thiện phân tích cú pháp AZW3 một cách lớn lao.

##### Tài liệu Word
* Tài liệu Word có tên kiểu dáng dành riêng cho ngôn ngữ không hiển thị tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ đây quay lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự kiểm soát trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại, Word cũ và Word hiện đại cũng như Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho tài liệu Microsoft Word cũ!
* Thêm hỗ trợ cho bài thuyết trình Powerpoint cũ!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt `ctrl+q` để thoát ứng dụng.
* Thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh được nhúng giờ đây nên được hiển thị đúng cách.
* Tài liệu CHM giờ đây hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Đã sửa go to page bị lệch 1.
* Đã sửa phím Escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu bối cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Đã sửa tài liệu sai đôi khi được lấy tiêu điểm khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ chứa hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua các hình ảnh và hình minh họa với `g`/`shift+g` và `f`/`shift+f` tương ứng.
* Paperback giờ đây sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã xóa hỗ trợ DAISY XML vì nó không còn cần thiết nữa.
* Chuyển lại điều hướng chữ cái đầu tiên Win32 gốc trong cây mục lục.
* Hộp thoại lỗi tải giờ đây hiển thị các thông báo lỗi chi tiết hơn.
* Webview giờ đây sẽ mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi khi mở webview trong các epub chứa liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi khi trình phân tích cú pháp RTF sẽ không đặt khoảng cách giữa các từ trong những trường hợp hiếm hoi.
* Đã sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF giờ đây có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab và nguồn cấp dữ liệu dòng RTF giờ đây được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Chuyển lại thư viện pdfium đã được thử nghiệm và đáng tin cậy để phân tích cú pháp PDF, giúp kết xuất PDF trở nên đáng tin cậy hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Thêm `Ctrl+Shift+T` để mở lại tài liệu đã đóng lần cuối.
* Hộp thoại All Documents giờ đây hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích cú pháp RTF.
* Đã sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ được viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa các nút Yes/No trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm các bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Thêm một bộ cập nhật tự động sẽ thay thế phiên bản Paperback hiện tại của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn khi đạt đến dấu trang hoặc ghi chú, cảm ơn Andre Louis vì những âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho các tài liệu DAISY XML.
* Thêm hỗ trợ cho các tệp Flat Open Document Text!
* Thêm hỗ trợ cho các bài thuyết trình Flat Open Document!
* Thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ di chuyển nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Sửa lỗi khôi phục cửa sổ Paperback từ khay hệ thống.
* Sửa lỗi tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Web View.
* Sửa lỗi bảng không hiển thị chính xác trong tệp Markdown.
* Các PDF chỉ chứa hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một tệp.
* Nhúng chính xác thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy cao hơn, tốc độ nhanh hơn và ít DLL hơn.
* Viết lại toàn bộ ứng dụng bằng Rust. Cơ sở mã mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu ngữ cảnh của điều khiển văn bản sẽ bao gồm các hành động dành riêng cho người đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem bảng trong webview.
* Thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong một trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật hiện sẽ hiển thị ghi chú phát hành khi có phiên bản mới.
* Sửa lỗi khôi phục cửa sổ từ khay hệ thống.
* Sửa lỗi dịch các nút Có/Không trong các hộp thoại xác nhận.
* Sửa lỗi tải cấu hình khi chạy với tư cách quản trị viên.
* Sửa lỗi xử lý bình luận trong các tài liệu XML và HTML.
* Sửa lỗi phân tích cú pháp TOC trong sách Epub 2.
* Sửa lỗi điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Sửa lỗi hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Sửa lỗi TOC epub thỉnh thoảng ném bạn đến mục sai.
* Sửa lỗi các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Sửa lỗi sai lệch một trong điều hướng liên kết.
* Sửa lỗi một số sách có khoảng trắng theo sau trên các dòng của chúng.
* Sửa lỗi các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho các cộng tác viên.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn Enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể điều hướng đến bằng các phím mũi tên alt+left/right.
* Thêm danh sách phần tử! Hiện tại nó chỉ hiển thị một cây tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Sửa lỗi các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Sửa lỗi phân tích cú pháp Epub TOC chứa các đường dẫn tương đối.
* Sửa lỗi một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Sửa lỗi tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Sửa lỗi bạn không thể sử dụng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Cải thiện xử lý các tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói rõ nếu danh sách tài liệu gần đây trống khi bạn cố gắng hiển thị hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu đi tới ở dạng nhỏ gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được bật theo mặc định.
* Thêm tùy chọn để điều hướng theo các phần tử cấu trúc có thể quay lại.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu đang được tập trung.
* Thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng bộ định thời ngủ cơ bản, có thể truy cập bằng `Ctrl+Shift+S`.
* Thêm hỗ trợ phân tích các ebook FB2!
* Thêm hỗ trợ phân tích các bài thuyết trình OpenDocument!
* Thêm hỗ trợ phân tích các tệp OpenDocument Text!
* Các dấu trang giờ đây có thể được tạo để đánh dấu toàn bộ một dòng hoặc chỉ một số văn bản được chỉ định. Nếu bạn không có lựa chọn nào hoạt động khi đặt dấu trang, hành vi sẽ giống như trước 0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Các dấu trang giờ đây có thể có các ghi chú văn bản tùy chọn được đính kèm! Điều hướng giữa các dấu trang chứa ghi chú bằng `N` và `Shift+N`, hoặc bật hộp thoại dấu trang với tất cả các dấu trang, chỉ ghi chú hoặc chỉ không có ghi chú được chọn bằng các phím tắt cụ thể.
* Các dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "dấu trang x" khó chịu.
* Các cuốn sách Epub chứa nội dung HTML giả vờ là XML giờ đây sẽ được xử lý đúng cách.
* Đã sửa việc tải các tài liệu Markdown lớn.
* Đã sửa việc nhấn dấu cách trong chế độ xem cây mục lục kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa điều khiển văn bản không lấy lại tiêu điểm đôi khi khi quay lại cửa sổ Paperback.
* Đã sửa trường văn bản trong hộp thoại đi tới phần trăm không cập nhật giá trị của thanh trượt.
* Đã sửa hiển thị các ID HTML tùy chỉnh trong các tài liệu Markdown.
* HTML bên trong các khối mã Markdown giờ đây sẽ được hiển thị đúng cách.
* Nếu tải một cuốn sách có tham số dòng lệnh trong khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback với quyền quản trị viên, cấu hình giờ đây sẽ được tải và lưu đúng cách.
* Giờ đây có thể xóa một dấu trang trực tiếp từ hộp thoại dấu trang.
* Giờ đây có thể nhập và xuất các dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo có tên theo tệp với phần mở rộng `.paperback`. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải tệp đó, nó sẽ được tải tự động. Nếu không, bạn có thể nhập chúng theo cách thủ công bằng cách sử dụng một mục trong menu công cụ.
* Các liên kết bên trong tài liệu giờ đây được hỗ trợ đầy đủ! Sử dụng `k` và `shift+k` để di chuyển về phía trước và phía sau chúng, và nhấn `enter` để mở/kích hoạt một liên kết.
* Nhiều tái cấu trúc nội bộ, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown giờ đây được xử lý trước để tuân thủ CommonMark trước khi hiển thị.
* Điều hướng theo danh sách và các mục của chúng giờ đây được hỗ trợ đầy đủ! Sử dụng `L` và `Shift+L` để đi theo danh sách, và `I` và `Shift+I` để đi qua các mục danh sách.
* Numpad delete giờ đây hoạt động để xóa tài liệu khỏi thanh tab ngoài delete thông thường.
* Paperback giờ đây có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback giờ đây có thể dịch được hoàn toàn! Danh sách các ngôn ngữ mà nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang phát triển không ngừng!
* Paperback giờ đây có một trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Các tài liệu PPTX giờ đây sẽ hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu được mở sẽ giờ đây được hiển thị trong hộp thoại thông tin tài liệu.
* Bộ cài đặt giờ đây bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách các tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn đã mở, nó giờ đây sẽ hiển thị một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn đã mở được truy cập thông qua một hộp thoại nhỏ.
* Nhiều cải tiến nhỏ khác nhau cho các trình phân tích cú pháp trên toàn bảng, bao gồm đặt một dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý dòng mới bên trong các đoạn trong tài liệu word và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho các bản trình bày PowerPoint!
* Đã sửa các mục menu nhất định không bị vô hiệu hóa khi không có tài liệu nào mở.
* Đã sửa định hướng của thanh trượt đi tới phần trăm.
* Đã sửa mục lục trong sách Epub có đường dẫn tệp được mã hóa URL và/hoặc ID phân đoạn.
* Đã sửa khoảng trắng bị loại bỏ từ các tiêu đề XHTML theo những cách kỳ lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Các tài liệu HTML và Markdown giờ đây hỗ trợ tính năng mục lục! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục của riêng mình từ cấu trúc các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại `ctrl+t`.
* Các tài liệu HTML giờ đây sẽ có tiêu đề được đặt trong thẻ title, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo lời nói. Điều này có nghĩa là không có DLL trình đọc màn hình nào được gửi cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ bây giờ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được làm lại hoàn toàn, và bây giờ nó cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại đi tới phần trăm giờ đây bao gồm một trường văn bản cho phép bạn nhập thủ công một phần trăm để nhảy tới.
* Trình phân tích cú pháp HTML giờ đây sẽ nhận ra dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo toàn chính xác một lần nữa.
* Khoảng trắng không ngắt Unicode giờ đây được tính đến khi loại bỏ các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở một tệp không được nhận ra mỗi lần bạn tải nó nữa, chỉ là lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng menu Bắt đầu tùy chọn vào trình cài đặt.
* Mục lục bây giờ sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có một mục con và mục cha có cùng văn bản ở cùng vị trí, bạn sẽ chỉ nhìn thấy mục cha.
* Đã sửa mục lục trong các tài liệu CHM nhất định.
* Đã sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối trong đó.
* Các tài liệu CHM giờ đây sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang tùy thích trong nhiều tài liệu. Bạn có thể nhảy về phía trước và phía sau thông qua chúng bằng `b` và `shift+b`, đặt một bằng `control+shift+b`, và mở hộp thoại để nhảy tới một dấu trang cụ thể bằng `control+b`.
* Đã thêm trình cài đặt bên cạnh tệp zip có thể di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập các liên kết tệp cho bạn.
* Các tệp văn bản với BOM giờ đây sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Bây giờ nó sẽ hiển thị cho bạn dòng, ký tự và tỷ lệ phần trăm đọc hiện tại của bạn.
* Các nhận xét HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu vượt qua một đường dẫn tương đối tới Paperback trên dòng lệnh, nó sẽ giải quyết nó một cách chính xác.
* Chuyển động phần trăm giờ đây được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập được bằng `control+shift+g`.
* Các tài liệu không có tiêu đề hoặc tác giả đã biết sẽ luôn có một tiêu đề mặc định.
* Logic lưu vị trí giờ đây thông minh hơn nhiều và chỉ nên ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi bạn đóng Paperback giờ đây được ghi nhớ trên các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại đi tới dòng và đi tới trang giờ đây sẽ được xử lý chặt chẽ hơn.
* Đã sửa điều hướng mục lục trong sách epub 3 có đường dẫn tương đối trong bản kê khai của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub có bản kê khai được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong các tài liệu HTML chứa các ký tự Unicode đa byte.
* Đã sửa sử dụng CPU cao trong các tài liệu có tiêu đề dài do một quay trở lại trong wxWidgets.
* Đã sửa tải các tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa một sự cố khi thoát ứng dụng trong một số trường hợp.
* Đã thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt xuống dòng từ!
* Giờ đây có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án tài trợ này ở dưới cùng của trang chính kho lưu trữ GitHub.
* Các tài liệu Markdown giờ đây sẽ luôn có tiêu đề, và Paperback giờ đây sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Các tài liệu PDF giờ đây sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF đáng tin cậy hơn nhiều trên toàn bộ.
* Bạn bây giờ chỉ có thể có một phiên bản Paperback chạy cùng một lúc. Chạy `paperback.exe` với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bây giờ bạn có thể nhấn xóa trên một tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại đi tới trang.
* Cho phép tab từ nội dung tài liệu đến danh sách các tài liệu đã mở của bạn.
* Đã sửa các phím tắt tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ chúng.
* Paperback giờ đây sẽ loại bỏ các dấu gạch ngang mềm không cần thiết từ đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn ở ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như một trình đọc màn hình.
* Đã sửa lỗi tải epub với tên tệp được mã hóa URL trong bản kê khai của chúng.
* Đã sửa lỗi tải sách epub 3 có XHTML được nhúng bên trong.
* Một thông báo sẽ được phát âm nếu tài liệu không hỗ trợ mục lục hoặc phần, thay vì các mục menu bị vô hiệu hóa.
* Đã thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu được mở gần đây nhất của bạn, và nhấn Enter trên một tài liệu sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm kiếm, làm cho nó dễ sử dụng hơn nhiều, đồng thời thêm lịch sử 25 tìm kiếm gần đây nhất của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây hiện được ghi nhớ khi khởi động lại ứng dụng. Điều này có thể định cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Đã thêm `shift+f1` để mở tệp readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
