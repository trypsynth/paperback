<!-- machine-translated from doc/readme.md (source-hash: 651d0b411879a6d8; sections: 84030068,db723a70,df2f4c18,14335443,91be3b41,6c87c514,94527a25,ce87a64f,a9eba369,e9860ee8,007c0542); please review and edit as needed -->

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

Các ứng dụng iOS và Android sử dụng cùng công cụ đọc như máy tính để bàn, vì vậy chúng mở các định dạng giống nhau và ghi nhớ vị trí của bạn theo cách tương tự. Chúng được xây dựng để sử dụng với VoiceOver trên iOS và TalkBack trên Android.

### Mở tài liệu

* Sử dụng nút Mở Sách hoặc mở tài liệu từ ứng dụng Tệp hoặc ứng dụng khác và chọn Paperback.
* Trên Android, bạn có thể bật trình duyệt tệp trong ứng dụng trong Cài đặt. Nó cần quyền Truy cập Tất cả Tệp và mở các tệp lớn ngay lập tức thay vì sao chép chúng trước.
* Nhấn lâu nút Mở Sách để nhập hoặc xuất dữ liệu tài liệu (`.paperback`), các tệp giống như ứng dụng máy tính để bàn sử dụng.

### Đọc và nghe

Mỗi ứng dụng có hai cách để đọc tài liệu. Ở chế độ văn bản, bạn đọc văn bản bằng trình đọc màn hình của mình. Ở chế độ đọc to, Paperback đọc văn bản cho bạn với giọng nói bạn chọn trong Cài đặt, và tiếp tục ở nền và từ màn hình khóa. Chuyển đổi giữa chúng từ menu Tùy Chọn Khác.

Sách nói, như DAISY, M4B và sách MP3, phát bản ghi của riêng chúng thay thế.

### Thanh đọc

Thanh ở dưới cùng của màn hình có, từ trái sang phải:

* Đơn vị điều hướng, chẳng hạn như đoạn, tiêu đề, trang hoặc liên kết. Vuốt lên hoặc xuống để thay đổi nó.
* Các nút trước, phát và tiếp theo. Trước và tiếp theo di chuyển theo đơn vị điều hướng.
* Tốc độ bài nói. Vuốt lên hoặc xuống để thay đổi tốc độ Paperback đọc.

Bạn cũng có thể vuốt lên hoặc xuống trên nút phát để di chuyển theo đơn vị điều hướng, mà không cần đến các nút trước và tiếp theo. Nếu đó là tất cả những gì bạn sử dụng, cài đặt Ẩn các nút trước và tiếp theo sẽ loại bỏ chúng khỏi đường dẫn của trình đọc màn hình của bạn. Cài đặt Vuốt lên di chuyển về phía trước chọn hướng vuốt.

### Tùy chọn khác

Menu Tùy Chọn Khác là nơi mọi thứ khác nằm. Một số mục hoạt động hơi khác trên mỗi ứng dụng.

* **Chuyển sang Chế Độ TTS hoặc Chuyển sang Chế Độ Văn Bản:** di chuyển giữa chế độ đọc to và chế độ văn bản, được mô tả ở trên. Android cũng có mục Đọc To bắt đầu và tạm dừng đọc to.
* **Mục Lục:** các chương của sách. Chọn một để đi thẳng tới nó. Trên Android, các mục có chương dưới chúng có thể được mở rộng và thu gọn, sử dụng các hành động của trình đọc màn hình. Trên iOS, toàn bộ danh sách được hiển thị cùng một lúc.
* **Phần Tử:** danh sách các tiêu đề hoặc liên kết của tài liệu. Chuyển đổi giữa hai cái bằng bộ chọn Loại trên iOS hoặc các tab trên Android, sau đó chọn một để đi tới nó.
* **Tìm:** nhập những gì để tìm kiếm và chọn xem có khớp chữ hoa chữ thường, khớp toàn bộ từ hoặc sử dụng biểu thức chính quy. Trên Android, thanh có Tìm Trước Đó và Tìm Tiếp Theo ở dưới cùng của màn hình cho đến khi bạn đóng nó, và các tìm kiếm trước đó nằm dưới Lịch Sử Tìm Kiếm. Trên iOS, các nút Tìm Trước Đó và Tìm Tiếp Theo nằm trên màn hình Tìm, và Tìm cũng hiển thị dưới dạng đơn vị điều hướng trên thanh đọc, vì vậy bạn có thể bước qua các kết quả khớp từ đó.
* **Đi Tới:** nhảy đến một dòng, một trang hoặc một phần trăm qua tài liệu. Chọn cái nào bằng bộ chọn Chế Độ.
* **Tài Liệu Gần Đây:** mọi tài liệu bạn đã mở, mỗi tài liệu được đánh dấu là hiện đang mở, đã đóng hoặc tệp bị mất. Mỗi cái có hai hành động của trình đọc màn hình: Loại Bỏ loại bỏ nó khỏi danh sách, và Định Vị cho phép bạn tìm tài liệu có tệp đã di chuyển. Xóa Tài Liệu Gần Đây làm trống danh sách mà không xóa bất kỳ tài liệu nào.
* **Số Lượng Từ:** số từ trong tài liệu.
* **Thông Tin Tài Liệu:** tiêu đề, tác giả, tên tệp và trên iOS là số dòng và số ký tự.
* **Xuất:** lưu tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* **Hẹn Giờ Ngủ:** dừng đọc sau 5, 10, 15, 30, 45 hoặc 60 phút, hoặc một thời gian của riêng bạn. Mở nó lại khi nó đang chạy để xem còn lại bao lâu, hoặc để hủy nó.
* **Trợ Giúp:** mở tệp readme này.
* **Cài Đặt:**
    * **Chuyển đổi văn bản thành lời nói:** giọng nói, tốc độ bài nói và cao độ, nút Phát Mẫu để nghe chúng, và tạm dừng giữa các đoạn. Android cũng cho phép bạn chọn công cụ bài nói. Trên iOS, đây cũng là nơi đặt từ điển bài nói: các quy tắc thay đổi cách phát âm các từ, cho mọi giọng nói hoặc chỉ một số.
    * **Khả Năng Đọc:** kích thước văn bản, khoảng cách dòng, khoảng cách đoạn và căn chỉnh. iOS cũng có giao diện sáng và tối, và văn bản độ tương phản cao.
    * **Hành Vi:** có mở lại tài liệu của bạn khi ứng dụng bắt đầu, hướng nào mà vuốt trên nút phát di chuyển, và có ẩn các nút trước và tiếp theo không. Android cũng có trình duyệt tệp trong ứng dụng ở đây.

### Bàn phím và tai nghe

Với bàn phím, các phím tắt máy tính để bàn để mở sách, tài liệu gần đây, Tìm, Đi Tới, mục lục, số lượng từ, thông tin tài liệu, xuất và hẹn giờ ngủ đều hoạt động, sử dụng `Cmd` thay vì `Ctrl` trên iOS. Các phím chữ cái duy nhất để di chuyển theo tiêu đề, trang, liên kết và phần còn lại cũng vậy, và `Space` phát và tạm dừng. Trên iOS, các phím chữ cái duy nhất chỉ đạt Paperback khi Chuyển Hướng Nhanh chữ cái duy nhất của VoiceOver tắt.

Trên Android, nút tai nghe phát và tạm dừng bằng một lần nhấn, di chuyển về phía trước bằng hai, và quay lại bằng ba.

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

1.0 là bản phát hành đầu tiên trên tất cả năm nền tảng: Windows, macOS, Linux, iOS và Android, với các ứng dụng iOS và Android trong App Store và Google Play.

#### Đã thêm

##### Chung
* Hỗ trợ Linux, dưới dạng AppImage hoặc tar.gz, với tích hợp màn hình nền để tài liệu mở từ trình quản lý tệp của bạn.
* Đánh dấu phần đầu của lựa chọn bằng `Alt+F9`, sao chép mọi thứ từ đó đến nơi bạn đã tới bằng `Alt+F10`, và quay lại dấu hiệu bằng `Alt+Shift+F9`, để sao chép một khoảng văn bản dài mà không cần shift-arrow qua nó. Cả ba đều nằm dưới Tools > Select and copy.
* Phím tắt `=` bây giờ công bố trang cũng như phần trăm, ví dụ "15%, trang 30", và vẫn giữ nguyên như trước đối với các tài liệu không có số trang.
* Hộp About bây giờ hiển thị giấy phép Paperback và mọi nhà dịch.
* Bản dịch tiếng Ukraina.

##### Định dạng mới
* Comic book archives (`.cbz`).
* Sách nói M4B, chia thành các chương của chúng.
* Manual pages, cả `man` lẫn BSD `mdoc`, được nén gzip hoặc không.
* Sách nói MP3, chia thành các chương khi tệp có chúng.
* Tệp Windows Write (`.wri`).
* Tệp WinHelp (`.hlp`).
* Tài liệu Word 6 và Word 95.

##### OCR
* Các trang PDF được quét bây giờ có thể được nhận dạng bằng OCR được tích hợp sẵn trong Windows và macOS. Nhấn `Enter` trên một trang được quét để nhận dạng nó, hoặc sử dụng Batch OCR (`Ctrl+Shift+O`) cho một loạt trang.

##### Điều hướng
* Các công thức MathML trong EPUB và HTML được hiển thị dưới dạng AsciiMath bằng MathCAT. Sử dụng `M` hoặc `Shift+M` để điều hướng các công thức, sau đó `Enter` hoặc `Space` để mở MathML gốc trong Formula View.
* Nút Find All trong hộp thoại Find, liệt kê mọi dòng có kết quả khớp để bạn có thể nhảy trực tiếp đến dòng bạn muốn.
* Các chế độ xem Tables, Lists và Pages trong danh sách các phần tử (`F7`).
* Go to Line, Go to Page và Go to Percent bây giờ chấp nhận `+n` và `-n` để di chuyển tương đối so với nơi bạn đang ở.
* Các cuốn sách EPUB, MOBI và CHM không có tiêu đề riêng của chúng bây giờ nhận được điều hướng tiêu đề từ mục lục của chúng.
* Các cuốn sách KF8 (AZW3) bây giờ hỗ trợ điều hướng phần.
* Các trang EPUB chỉ là một bức ảnh bây giờ hiển thị một dòng cho nó, vì vậy bạn có thể hạ cánh trên chúng thay vì bỏ qua ngay.

##### Sách nói
* Các điều khiển tốc độ phát lại, từ nửa tốc độ đến ba lần nhanh hơn. Sử dụng `Ctrl+Shift+.` và `Ctrl+Shift+,`, hoặc menu Tools.
* Các dấu trang và ghi chú trong các cuốn sách chỉ âm thanh bây giờ ghi nhớ thời gian chính xác mà bạn đặt chúng.
* Next and previous position (`Alt+Left` và `Alt+Right`) bây giờ hoạt động trong sách nói.
* Tiến độ qua một cuốn sách nói bây giờ được đo bằng bản ghi âm của nó, vì vậy Go to Percent và thanh trạng thái phù hợp với mức độ bạn thực sự tiến hành.

##### Tài liệu gần đây
* Một mục Clear Recent Documents trong menu con Recent Documents.

##### Tài liệu PDF
* Một cài đặt để giữ mọi dòng của PDF riêng biệt, thay vì kết hợp chúng thành các đoạn văn.
* Hình ảnh và hình vẽ trong PDF bây giờ được công bố.
* Các PDF mang cấu trúc đọc nhưng không gắn thẻ bất kỳ bức ảnh nào của chúng bây giờ công bố những bức ảnh đó, thay vì bỏ chúng ra khỏi cuốn sách hoàn toàn.

##### Web View
* Bất kỳ tài liệu nào bây giờ có thể được mở trong chế độ xem web, không chỉ EPUB, HTML và Markdown.

##### Khả năng đọc
* Tiêu đề bây giờ được vẽ ở kích thước phù hợp với cấp độ của chúng, và hình ảnh và bảng được tách biệt khỏi văn bản xung quanh chúng.

##### pb
* `pb --list-formats` liệt kê mọi định dạng mà pb có thể đọc.
* pb bây giờ cho biết tệp nào nó không thể đọc được và lý do tại sao.

#### Đã sửa

##### Chung
* Sửa lỗi crash khi đóng Paperback.
* Đóng Paperback giờ đây sẽ ẩn cửa sổ ngay lập tức, thay vì để nó ở trên màn hình trong khi lưu.
* Mở một tài liệu không còn để "Mở lại tài liệu đã đóng gần đây" được bật khi không có gì để mở lại.
* Paperback không còn tiếp tục thử lại những tài liệu trong danh sách gần đây của bạn mà đã biến mất, và giới hạn số lượng tài liệu gần đây mà nó lưu trữ.
* Tệp cài đặt INI cũ giờ đây được xóa khi nó được chuyển sang định dạng mới.
* Tiêu đề của các hộp thoại phông chữ và màu sắc, và menu "Xuất As" trong tiếng Việt, giờ đây đã được dịch.
* Cập nhật giờ đây sẽ đưa cửa sổ được khởi động lại lên phía trước, thay vì để nó ở phía sau các cửa sổ khác trong Alt+Tab.
* Gói dòng giờ đây áp dụng ngay lập tức trên các tài liệu lớn, thay vì tải lại toàn bộ tài liệu.

##### Điều hướng
* `Alt+Left` giờ đây sẽ quay lại nơi bạn nhảy từ đó, thay vì đến một vị trí cũ hơn.
* Âm thanh dấu trang giờ đây chỉ phát khi bạn di chuột qua dấu trang, không phải khi bạn hạ cánh trên dòng mà nó ở.
* Đóng mục lục, danh sách các phần tử và các hộp thoại "Đi tới" giờ đây sẽ đưa bạn thẳng đến dòng bạn hạ cánh, thay vì khiến bạn phải chờ trình đọc màn hình đọc cửa sổ lại.
* "Đi tới dòng", "Đi tới trang" và "Đi tới phần trăm" giờ đây từ chối những số nằm ngoài tài liệu thay vì im lặng đi đâu đó khác.
* NVDA không còn cắt ngắn thông báo khi một tài liệu không có trang.
* Nhấn OK trong mục lục mà không di chuyển giờ đây sẽ đi tới mục nhập đã được chọn.
* Mục lục, danh sách các phần tử và danh sách dấu trang không còn bị trễ hoặc đóng băng trên những cuốn sách có hàng nghìn mục nhập.
* Phím mũi tên "Lên" và "Xuống" giờ đây sẽ nhớ cột của chúng mỗi tài liệu, thay vì mang nó qua khi bạn chuyển đổi tab.

##### Sách nói
* Phát lại âm thanh giờ đây sử dụng `Control+Space` trên macOS, vì `Command+Space` thuộc về Spotlight.

##### Tài liệu PDF
* Sửa lỗi những PDF được xuất từ Apple Pages đọc như văn bản thuần túy, không có bất kỳ tiêu đề và danh sách nào mà chúng được viết với.
* Sửa lỗi các đoạn và tiêu đề PDF bị tách ở mỗi dòng, và các từ bị tách rời tại dấu cách.
* Sửa lỗi những tiêu đề PDF được đánh số chạy cùng nhau thành một tiêu đề.
* Sửa lỗi những PDF có cây cấu trúc dẫn đến không có văn bản mở trống.
* Đầu trang và chân trang không còn được đọc thành tiếng trên mỗi trang của những PDF không được gắn thẻ.
* Những PDF gắn thẻ đầu trang và chân trang của chúng là văn bản thường xuyên không còn lặp lại tiêu đề và số trang giữa hai đoạn trên mỗi trang.
* Những PDF giờ đây hiển thị tiêu đề thực của chúng, thay vì tên tệp của chúng.
* Những dòng được đặt trong phông chữ không tỉ lệ, như mã, không còn được nối vào các đoạn văn.

##### Sách MOBI/AZW3
* Những cuốn sách MOBI lớn không còn hết bộ nhớ, và không còn bị cắt ngắn sau 20 MB.
* Những cuốn sách MOBI và AZW3 giờ đây mở nhanh hơn nhiều.
* Sửa lỗi những cuốn sách MOBI mất danh sách chương của chúng.
* Sửa lỗi văn bản bị lỗi khi những cuốn sách MOBI chuyển từ bản ghi này sang bản ghi khác.

##### Web View
* Web view không còn tải toàn bộ một cuốn sách khổng lồ cùng một lúc.
* Web view giờ đây hiển thị tài liệu toàn bộ khi trình đọc hiển thị chúng toàn bộ, thay vì chỉ một phần của chúng.

##### Những định dạng khác
* Những cuốn sách FictionBook (.fb2) được viết bằng windows-1251, là phần lớn trong số chúng, giờ đây mở thay vì không đọc được hoàn toàn.
* Những cuốn sách FictionBook sử dụng một không gian tên hoặc thực thể HTML mà chúng không bao giờ khai báo giờ đây mở, thay vì bị từ chối như bị hỏng.
* Những cuốn sách trong mã hóa cũ giờ đây mở nhanh hơn nhiều.
* Sửa lỗi một số tệp văn bản tiếng Trung mở dưới dạng văn bản bị lỗi.
* Những tệp OpenDocument được bảo vệ bằng mật khẩu giờ đây yêu cầu mật khẩu của chúng, thay vì được báo cáo là bị hỏng.
* Những tệp PowerPoint cũ được bảo vệ bằng mật khẩu giờ đây mở, và các slide PowerPoint cũ không còn mất văn bản của chúng.
* Những tệp văn bản thuần túy được lưu bằng phần mở rộng `.rtf` giờ đây mở dưới dạng văn bản, thay vì thất bại với lỗi.
* Những từ điều khiển RTF không còn hiển thị dưới dạng văn bản.

#### iOS và Android

Các ứng dụng iOS và Android mở mọi định dạng mà máy tính để bàn làm, và bao gồm:

* Đọc to lên, với lựa chọn giọng nói, tốc độ và cao độ, một điều khiển tốc độ nói ngay trên thanh đọc, và một tạm dừng tùy chọn giữa các đoạn.
* Phát lại sách nói DAISY, M4B và MP3, điều này tiếp tục chạy trong nền và từ màn hình khóa.
* Điều hướng theo tiêu đề, trang, liên kết, bảng, danh sách và hơn thế nữa từ thanh đọc, cộng với mục lục và "Tìm".
* Bộ hẹn giờ ngủ, số từ và xuất tài liệu, cộng với từ điển nói trên iOS.
* Tùy chọn kích thước và khoảng cách văn bản, cộng với văn bản độ tương phản cao trên iOS.
* Các phím tắt bàn phím phù hợp với máy tính để bàn.

### Phiên bản 0.9.2
* Sách nói không còn khiến trình đọc màn hình của bạn đọc một loạt khoảng trắng khi bạn focus vào trường văn bản.
* Sách nói giờ đây đặt tên tệp theo khi bạn di chuyển qua các phần của chúng.
* Sách nói giờ đây báo cáo độ dài thực tế của chúng, thay vì tuyên bố mọi tệp trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã nhấp vào liên kết bên trong nó.
* Sao chép sau Select All giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần của nó hiện được tải.
* Find giờ đây đi thẳng đến dòng mà nó tìm thấy, thay vì buộc bạn phải nghe trình đọc màn hình đọc lại cửa sổ khi focus trở về sách.
* Đã sửa các EPUB có khối ZIP64 lạc mà từ chối mở bằng "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại điểm bắt đầu trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView giờ đây đưa bạn đến phần mà chúng chỉ, thay vì không thành công với "File not found".
* Thông báo "Document reloaded" tự động không còn ngắt trình đọc màn hình của bạn ở giữa câu, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt giờ đây tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Windows giờ đây sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì đủ dòng của chương trình.
* Word Count và Document Info giờ đây hiển thị có bao nhiêu tệp mà sách nói chứa, và nó chạy bao lâu trong tổng số.

### Phiên bản 0.9.1
* Các âm thanh dấu trang và ghi chú giờ đây phát trên macOS.
* Các sách DAISY giờ đây phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Đã sửa các dấu ngoặc kép cong, dấu gạch ngang em và các ký tự tương tự biến mất khỏi các tài liệu RTF, chạy các từ xung quanh chúng với nhau khi chúng đi.
* Đã sửa các hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản lộn xộn.
* Đã sửa menu Tài liệu gần đây giữ các mục cũ cho đến khi điều gì đó khác xảy ra để xây dựng lại nó.
* Các phím tắt bàn phím trở lại trong mọi bản dịch, vì vậy các menu của Nga có quyền truy cập bàn phím một lần nữa.
* Các tài liệu CHM lớn giờ đây mở nhanh hơn tới bảy lần.
* Các tài liệu được mở giờ đây được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách jump list của thanh tác vụ và danh sách gần đây của menu Start.
* Tùy chọn đã được đổi tên thành Cài đặt, khớp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback giờ đây nhớ vị trí cửa sổ, kích thước và trạng thái tối đa hóa của nó giữa các lần chạy.
* Các dạng số nhiều giờ đây được dịch, vì vậy các thông báo đếm mọi thứ đọc đúng cách trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY giờ đây mở sách nói hoàn chỉnh thay vì chỉ văn bản của nó.
* Các tên hành động của hộp thoại Tùy chỉnh Phím tắt bàn phím giờ đây có thể được dịch.
* Tiêu đề tài liệu giờ đây xuất hiện trước tiên trong thanh tiêu đề, vì vậy các cuốn sách mở có thể được phân biệt trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật giờ đây được dịch.

### Phiên bản 0.9.0

#### Đã Thêm

##### Tổng Quát
* Một công cụ CLI gọi là pb để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn View Source để mở mã nguồn của tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, nghĩa là bạn có thể tải sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều gì kỳ lạ được tìm thấy với điều này.

##### Hỗ Trợ Nền Tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Một bật/tắt toàn màn hình.

##### Hộp Thoại Tất Cả Tài Liệu
* Một nút định vị để định vị sách còn thiếu vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, để bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy Chọn và Khả Năng Đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Gói từ (được chuyển từ tổng quát);
    * Hiển thị bảng nội tuyến (mới trong phiên bản này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ cái;
    * Căn chỉnh văn bản.
* Một mục menu gói từ và phím nóng tiếp theo.
* Một bật/tắt để xác định cách bạn muốn hiển thị bảng và thống nhất cách bảng được hiển thị trên các tài liệu.

##### Điều Hướng
* Hỗ trợ điều hướng theo container.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để công bố tỷ lệ phần trăm hiện tại của bạn thông qua một tài liệu.

##### Dấu Trang
* Dấu trang tạm thời: bạn có thể có một dấu trang trên mỗi tài liệu và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một dấu và dấu gạch chéo ngược để nhảy đến nó.

##### Số Lượng Từ
* Thời gian đọc ước tính trong hộp thoại số lượng từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn đang hoạt động khi bạn mở hộp thoại số lượng từ, số lượng từ bạn đã chọn sẽ được hiển thị.

##### Phím Tắt Bàn Phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn Ngữ
* Tiếng Hà Lan, Tiếng Phần Lan và Tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình Cập Nhật
* Một nút hủy vào hộp thoại cập nhật đang diễn ra.
* Trình cập nhật bây giờ xác thực rằng tệp đã tải xuống chưa bị thao túng.

##### Chế Độ Xem Web
* Chế độ xem web hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách Âm Thanh
* Khả năng phát sách âm thanh, hiện hỗ trợ cả DAISY âm thanh (bao gồm DAISY âm thanh + văn bản) và zip của các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời tường thuật, tìm kiếm tiến và lùi, và điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn xem việc tìm kiếm vượt quá cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài Liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã sửa

##### Chung
* Các tài liệu được mã hóa bằng các bộ mã CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị đúng cách thay vì hiển thị một loạt ký tự lỗi.
* "Mở lại" cố gắng mở lại tệp readme được đóng gói.
* Tab đã chọn của bạn không được lấy tiêu điểm đúng cách sau khi khởi động lại Paperback.
* Cách Paperback xử lý các tệp trên ổ đĩa mạng Windows: nhấn "hiển thị tệp trong thư mục" sẽ lấy tiêu điểm đúng cách vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn bị tải buộc obligatory khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một tệp.
* "Mở thư mục chứa" sẽ giờ đây lấy tiêu điểm vào tệp đã cho trong trình khám phá.
* Mở readme sẽ giờ đây tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng của Paperback sẽ giờ đây được chia tỷ lệ đúng cách trên màn hình DPI cao.
* Menu sẽ giờ đây cập nhật đúng cách và tiêu điểm sẽ di chuyển sang điều khiển văn bản khi mở trợ giúp trong Paperback.
* Đã chuyển sang một phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu đang hoạt động sẽ giờ đây được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục cho từng ký tự bên trong.

##### Hộp thoại Tất cả tài liệu
* Phím Escape không đóng các hộp thoại Thông tin tài liệu và Tất cả tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở thông qua `Shift+F1`.
* Xóa tài liệu khỏi hộp thoại gần đây sẽ giờ đây cũng đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ đây được bảo tồn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* "Đi tới dòng", "Đi tới trang" và "Đi tới phần trăm" đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* "Tìm" và "Tìm tiếp theo" không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú sẽ giờ đây phát đúng cách chỉ khi bạn điều hướng qua một từ chứa một trong số chúng.

##### Khả năng đọc
* Áp dụng ngắt dòng từ đưa bạn tới đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên với kích thước ban đầu rất nhỏ.
* Hình ảnh sẽ giờ đây hiển thị đúng cách trong webview được nhúng.

##### Trình cập nhật
* Trình cập nhật sẽ giờ đây hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Đang tải các sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích cú pháp tài liệu RTF với các ký tự không phải Latin trong chúng.
* Các nhóm RTF `\pict` để dữ liệu hình ảnh được nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các anchors filepos trong sách Mobi chia thẻ HTML và đưa rác vào văn bản sách.
* Liên kết trong sách Mobi kế thừa.
* Cải thiện phân tích cú pháp AZW3 rất lớn.

##### Tài liệu Word
* Tài liệu Word có tên kiểu dáng cụ thể theo miền địa phương không hiển thị tiêu đề đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ đây quay lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm cho Paperback bị sập khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang vào sách epub.
* Đã thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện tại, Word cũ và Word hiện đại cũng như Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Đã thêm hỗ trợ cho tài liệu Microsoft Word cũ!
* Đã thêm hỗ trợ cho bài thuyết trình Powerpoint cũ!
* Đã thêm hỗ trợ cho sách mobi và AZW3!
* Đã thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Đã thêm phím tắt ctrl+q để thoát ứng dụng.
* Đã thêm hỗ trợ cho các sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh được nhúng sẽ giờ đây được hiển thị đúng cách.
* Tài liệu CHM giờ đây hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Đã sửa "đi tới trang" bị sai lệch 1.
* Đã sửa phím escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu bối cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Ứng dụng.
* Đã sửa tài liệu sai đôi khi được lấy tiêu điểm khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ có hình ảnh được phát hiện lại một lần nữa và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua hình ảnh và hình bằng g/shift+g và f/shift+f tương ứng.
* Paperback sẽ giờ đây tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã xóa hỗ trợ DAISY XML, vì không còn cần thiết.
* Đã chuyển lại điều hướng chữ cái đầu tiên Win32 gốc trong cây mục lục.
* Hộp thoại lỗi tải sẽ giờ đây hiển thị các thông báo lỗi chi tiết hơn.
* Webview sẽ giờ đây mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang vào tài liệu RTF!
* Đã sửa lỗi trong đó mở webview trong các epub chứa liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi trong đó trình phân tích cú pháp RTF sẽ không đặt khoảng cách giữa các từ trong những trường hợp hiếm.
* Đã sửa các đoạn văn bị chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF giờ đây có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab RTF và dòng nguồn cấp dữ liệu giờ đây được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Đã chuyển lại thư viện pdfium được thử nghiệm và đáng tin cậy để phân tích PDF, giúp kết xuất PDF trở nên đáng tin cậy hơn nhiều lần nữa.

### Phiên bản 0.8.1
* Đã thêm Ctrl+Shift+T để mở lại tài liệu đã đóng lần cuối cùng.
* Hộp thoại Tất cả tài liệu giờ đây hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích cú pháp RTF.
* Đã sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua một phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa của các nút Có/Không trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Đã thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Đã thêm tính năng cập nhật tự động sẽ thay thế phiên bản Paperback hiện tại của bạn thay vì chỉ tải xuống phiên bản mới!
* Đã thêm phản hồi âm thanh tùy chọn để tới một dấu trang hoặc ghi chú, cảm ơn Andre Louis vì những âm thanh!
* Đã thêm hỗ trợ tài liệu RTF!
* Đã thêm hỗ trợ cho tài liệu DAISY XML.
* Đã thêm hỗ trợ cho tệp Flat Open Document Text!
* Đã thêm hỗ trợ cho bản trình bày Flat Open Document!
* Đã thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ di chuyển nào lớn hơn 300 ký tự sẽ tự động được thêm vào lịch sử điều hướng của bạn.
* Đã sửa việc khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa các tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Web View.
* Đã sửa các bảng không được hiển thị chính xác trong các tệp Markdown.
* Các PDF chỉ có hình ảnh sẽ hiển thị cảnh báo khi bạn cố gắng tải một.
* Nhúng thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy cao hơn, tốc độ nhanh hơn và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Cơ sở mã mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì cũng như mở rộng hơn.
* Menu ngữ cảnh của điều khiển văn bản sẽ bây giờ bao gồm các hành động đặc thù cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Đã thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng cách sử dụng T và Shift+T, và nhấn Enter để xem bảng trong webview.
* Đã thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình hiển thị dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Đã thêm nút Xóa tất cả vào hộp thoại Tất cả tài liệu.
* Trình kiểm tra cập nhật sẽ hiển thị ghi chú phát hành khi có phiên bản mới.
* Đã sửa việc khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Có/Không trong các hộp thoại xác nhận.
* Đã sửa tải cấu hình khi chạy dưới quyền quản trị viên.
* Đã sửa xử lý nhận xét trong các tài liệu XML và HTML.
* Đã sửa phân tích TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Đã sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Đã sửa TOC epub đôi khi đưa bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Đã sửa lỗi off-by-one trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng ở cuối trên các dòng của chúng.
* Đã sửa các vấn đề trình phân tích khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được tắt đúng cách khi không có tài liệu nào mở.
* Đã cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Đã cải thiện quy trình dịch cho các người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Đã thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Đã thêm tính năng đi tới vị trí trước đó/tiếp theo rất cơ bản. Nếu bạn nhấn Enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể điều hướng bằng các phím mũi tên alt+left/right.
* Đã thêm danh sách phần tử! Hiện tại, nó chỉ hiển thị một cây tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Đã thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Đã sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích TOC Epub chứa các đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa các tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Đã cải thiện xử lý các tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói ra nếu danh sách tài liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0
* Đã thêm một tùy chọn mới để hiển thị menu đi tới dưới dạng nhỏ gọn hơn nhiều vào hộp thoại tùy chọn, được bật theo mặc định.
* Đã thêm tùy chọn để điều hướng bằng các phần tử cấu trúc có thể quấn lại.
* Đã thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại được tập trung.
* Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập bằng `Ctrl+Shift+S`.
* Đã thêm hỗ trợ phân tích cú pháp sách điện tử FB2!
* Đã thêm hỗ trợ phân tích cú pháp bài thuyết trình OpenDocument!
* Đã thêm hỗ trợ phân tích cú pháp tệp OpenDocument Text!
* Bây giờ có thể tạo dấu trang để đánh dấu toàn bộ một dòng, hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn nào hoạt động khi đặt dấu trang, hành vi giống như trước 0.6 và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Bây giờ dấu trang có thể có các ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú bằng `N` và `Shift+N`, hoặc mở cửa sổ hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ không ghi chú được chọn bằng các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Giờ đây, các sách Epub chứa nội dung HTML giả vờ là XML sẽ được xử lý đúng cách.
* Đã sửa lỗi tải các tài liệu Markdown lớn.
* Đã sửa lỗi nhấn phím cách trong chế độ xem cây mục lục kích hoạt nút OK.
* Đã sửa lỗi xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa lỗi điều khiển văn bản không lấy lại tiêu điểm đôi khi khi quay lại cửa sổ Paperback.
* Đã sửa lỗi trường văn bản trong hộp thoại đi tới phần trăm không cập nhật giá trị của thanh trượt.
* Đã sửa lỗi hiển thị các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong các khối mã Markdown giờ đây sẽ được hiển thị đúng cách.
* Nếu tải một sách bằng tham số dòng lệnh trong khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn nhận được lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới dạng quản trị viên, cấu hình giờ đây sẽ được tải và lưu đúng cách.
* Bây giờ có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Bây giờ có thể nhập và xuất các dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo ra được đặt tên theo tệp có phần mở rộng `.paperback`. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp trong khi tải, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng thủ công bằng một mục trong menu công cụ.
* Các liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng `k` và `shift+k` để di chuyển về phía trước và phía sau thông qua chúng, và nhấn `enter` để mở/kích hoạt một liên kết.
* Nhiều cấu trúc lại nội bộ, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown bây giờ được xử lý trước để tuân thủ CommonMark trước khi hiển thị.
* Điều hướng bằng các danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng `L` và `Shift+L` để đi theo các danh sách, và `I` và `Shift+I` để đi qua các mục danh sách.
* Phím xóa Numpad bây giờ hoạt động để loại bỏ tài liệu khỏi thanh tab ngoài phím xóa bình thường.
* Paperback bây giờ có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này được tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback hiện có thể dịch được hoàn toàn! Danh sách các ngôn ngữ mà nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang phát triển không ngừng!
* Paperback hiện có một trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX giờ đây sẽ hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu đã mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt giờ đây bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó giờ đây sẽ hiển thị một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn từng mở có thể truy cập thông qua một hộp thoại nhỏ.
* Nhiều cải tiến nhỏ cho các trình phân tích cú pháp trên toàn bộ, bao gồm đặt một dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý dòng mới bên trong các đoạn trong tài liệu word và thêm các dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho các bản trình bày PowerPoint!
* Đã sửa một số mục menu không bị vô hiệu hóa khi không có tài liệu nào mở.
* Đã sửa hướng của thanh trượt đi tới phần trăm.
* Đã sửa mục lục trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng bị loại bỏ khỏi các tiêu đề XHTML theo những cách kỳ lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Các tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục của riêng mình từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại `ctrl+t`.
* Các tài liệu HTML hiện sẽ có tiêu đề được đặt trong thẻ title, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Đã chuyển từ UniversalSpeech để sử dụng một vùng trực tiếp để báo cáo lời nói. Điều này có nghĩa là không còn các DLL trình đọc màn hình được gửi kèm chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Đã chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại yêu cầu bạn có muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được làm lại hoàn toàn, và hiện nó cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại đi tới phần trăm hiện bao gồm một trường văn bản cho phép bạn nhập thủ công một phần trăm để nhảy tới.
* Trình phân tích HTML hiện sẽ nhận dạng dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo tồn chính xác một lần nữa.
* Ký tự không ngắt Unicode hiện được coi khi loại bỏ các dòng trống.
* Bạn sẽ không còn được yêu cầu cách bạn muốn mở một tệp không được nhận dạng mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng menu bắt đầu tùy chọn vào trình cài đặt.
* Mục lục bây giờ sẽ sạch hơn trong một vài trường hợp, ví dụ như nếu bạn có một mục con và mục cha có cùng văn bản ở cùng vị trí, bạn sẽ chỉ thấy mục cha.
* Đã sửa mục lục trong các tài liệu CHM nhất định.
* Đã sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối trong chúng.
* Các tài liệu CHM bây giờ sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ đánh dấu! Bạn có thể có bao nhiêu đánh dấu tùy thích trong bao nhiêu tài liệu tùy thích. Bạn có thể nhảy tiến và lùi qua chúng bằng `b` và `shift+b`, đặt một đánh dấu bằng `control+shift+b`, và mở một hộp thoại để nhảy tới một đánh dấu cụ thể bằng `control+b`.
* Đã thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập các liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Hiện nó sẽ hiển thị cho bạn dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Các bình luận HTML, cũng như nội dung của các thẻ script và style sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu truyền một đường dẫn tương đối cho Paperback trên dòng lệnh, nó hiện sẽ phân giải nó đúng cách.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập bằng `control+shift+g`.
* Các tài liệu không có tiêu đề hoặc tác giả đã biết hiện sẽ luôn có một mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ nên ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi đóng Paperback hiện được ghi nhớ trên các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại đi tới dòng và đi tới trang hiện sẽ được làm sạch chặt chẽ hơn.
* Đã sửa điều hướng mục lục trong sách epub 3 có đường dẫn tương đối trong các bản kê của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub có bản kê được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Đã sửa việc sử dụng CPU cao trong các tài liệu có tiêu đề dài do một hồi quy trong wxWidgets.
* Đã sửa tải tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa một sự cố khi thoát ứng dụng trong các trường hợp nhất định.
* Đã thêm một hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt ngắt dòng từ!
* Hiện nay có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết tài trợ dự án ở cuối trang chính của kho lưu trữ GitHub.
* Các tài liệu Markdown hiện sẽ luôn có tiêu đề, và Paperback hiện sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Các tài liệu PDF hiện sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Đã chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều.
* Bạn hiện chỉ có thể chạy một phiên bản Paperback cùng một lúc. Chạy `paperback.exe` với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bạn hiện có thể nhấn delete trên một tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại đi tới trang.
* Cho phép nhấn tab từ nội dung tài liệu tới danh sách các tài liệu đã mở.
* Đã sửa các phím tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ số lượng.
* Paperback hiện sẽ loại bỏ các dấu gạch ngang mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đưa bạn tới ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa lỗi tải epub có tên tệp được mã hóa URL trong bản kê khai của chúng.
* Đã sửa lỗi tải sách epub 3 có XHTML nhúng bên trong.
* Hiện tại, một thông báo sẽ được phát âm nếu tài liệu không hỗ trợ mục lục hoặc các phần, thay vì các mục menu bị vô hiệu hóa.
* Đã thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu được mở gần đây nhất của bạn, và nhấn enter trên một tài liệu sẽ mở nó để đọc.
* Hoàn toàn viết lại hộp thoại Tìm kiếm, làm cho nó dễ sử dụng hơn nhiều, đồng thời thêm lịch sử 25 lần tìm kiếm gần đây nhất và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây hiện được ghi nhớ khi khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Đã thêm `Shift+F1` để mở readme trực tiếp trong chính Paperback.

### Phiên bản 0.1.0
* Phiên bản ban đầu.
