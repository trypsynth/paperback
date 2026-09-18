<!-- machine-translated from doc/readme.md (source-hash: 11f05688d690d71a; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,2fb18876,71df8e94,e9860ee8,a7ac6234); please review and edit as needed -->

# Paperback - phiên bản 0.9.2

## Giới thiệu

Paperback là một trình đọc ebook và tài liệu nhẹ, nhanh chóng và dễ tiếp cận cho mọi người, từ những độc giả bình thường đến những người dùng nâng cao. Nó được thiết kế để có khả năng truy cập bằng trình đọc màn hình, tốc độ nhanh và không có tính năng thừa.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản macOS ARM hiện đại. Các ứng dụng iOS và Android gốc đang trong quá trình phát triển tích cực, với các bản dựng thử nghiệm công khai được lên kế hoạch sớm sau khi phát hành desktop 0.9.0, trước khi phát hành 1.0 thống nhất bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu bất kỳ phần mềm nào được cài đặt trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh chóng, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở bao nhiêu tài liệu tùy thích cạnh nhau.
* Lưu lại vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đã mở khi đóng chương trình, và khôi phục chúng khi khởi động lần tiếp theo.
* Bao gồm chức năng điều hướng tương tự như chế độ duyệt web của nhiều trình đọc màn hình để điều hướng nhanh chóng và dễ dàng qua các tài liệu.
* Bao gồm hộp thoại tìm kiếm mạnh mẽ, với các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn theo cách di động, hoặc được cài đặt với các liên kết tệp tự động được thiết lập.
* Hỗ trợ một loạt lớn các định dạng tệp phổ biến.

## Tính tương thích trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết dành cho người dùng JAWS.

### JAWS và Màn hình Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể nhận thấy rằng các đoạn dài bị cắt ngắn khi di chuyển về phía trước bằng các phím điều hướng của màn hình. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách xử lý của JAWS với điều khiển văn bản RICHEDIT50W, không phải điều gì trong chính Paperback, và đó là một lỗi mất khá lâu mới tìm ra cách sửa chữa được cho rằng Vispero rất nhiệt tình trong việc đáp ứng các vấn đề với phần mềm mã nguồn mở.

Cách khắc phục, cuối cùng được tiết lộ thông qua nhóm thảo luận JAWS sau khi chờ đợi hàng tháng, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn hiện hoạt động chứ không tiến tới. Với cả hai cài đặt này, quá trình di chuyển phải hoạt động đúng cách.

## Các loại tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Các kho lưu trữ sách truyện tranh (`.cbz`, `.cbr`)
* Các tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Các trang hướng dẫn, cả `man` và BSD `mdoc` (`.1` đến `.9`, `.man`, `.roff`, và các dạng nén gzip của mỗi cái)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách nói M4B (`.m4b`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bài thuyết trình PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp WinHelp (`.hlp`)
* Tệp văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng ưu tiên bàn phím. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Trong trường hợp macOS khác, phương án tương đương được ghi chú trong ngoặc đơn — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã được các quy ước hệ thống hoặc ứng dụng khác sử dụng trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đóng lần cuối.
* `Ctrl+R`: Hiển thị hộp thoại "All Documents" (từ Recent Documents).
* `Ctrl+Q`: Thoát (chỉ dành cho Windows; trên macOS, tính năng này nằm trong menu ứng dụng).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Find.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi đến dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi đến phần trăm.
* `Ctrl+P`: Đi đến trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo phần trăm đọc hiện tại và trang của bạn, ví dụ "15%, page 30". Trang được bỏ qua đối với các tài liệu không có số trang.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến tới trong lịch sử điều hướng.
* `[`: Phần trước đó.
* `]`: Phần tiếp theo.
* `Shift+H`: Tiêu đề trước đó.
* `H`: Tiêu đề tiếp theo.
* `Shift+1` đến `Shift+6`: Tiêu đề trước đó ở mức 1-6.
* `1` đến `6`: Tiêu đề tiếp theo ở mức 1-6.
* `Shift+P`: Trang trước đó.
* `P`: Trang tiếp theo.
* `Shift+B`: Dấu trang trước đó.
* `B`: Dấu trang tiếp theo.
* `/`: Đặt dấu trang tạm thời của bạn.
* `\`: Nhảy đến dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước đó.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Nhảy đến tất cả các dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy đến các dấu trang chỉ.
* `Ctrl+Alt+M`: Nhảy đến ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem văn bản ghi chú tại vị trí hiện tại.
* `Shift+K`: Liên kết trước đó.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước đó.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình trước đó.
* `F`: Hình tiếp theo.
* `Shift+T`: Bảng trước đó.
* `T`: Bảng tiếp theo.
* `Shift+M`: Công thức trước đó.
* `M`: Công thức tiếp theo.
* `Shift+S`: Dấu phân cách trước đó.
* `S`: Dấu phân cách tiếp theo.
* `Shift+L`: Danh sách trước đó.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước đó.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi đến đầu vùng chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi qua cuối vùng chứa hiện tại (danh sách hoặc bảng).

### Menu Tools

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý thay vì Cmd): Hiển thị số lượng từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong một tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần túy.
* `Ctrl+Shift+B`: Bật/tắt dấu trang tại lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang tại lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt gói dòng.
* `Ctrl+Space`: Phát/tạm dừng âm thanh kể chuyện.
* `'`: Tua nhanh âm thanh kể chuyện.
* `;`: Tua lại âm thanh kể chuyện.
* `Ctrl+'`: Tăng lượng tua âm thanh.
* `Ctrl+;`: Giảm lượng tua âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, trong menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.
* `Alt+F9` (macOS: `Cmd+F9`): Đánh dấu đầu của một lựa chọn, để mọi thứ từ đây đến nơi bạn đến có thể được sao chép cùng một lúc.
* `Alt+F10` (macOS: `Cmd+F10`): Sao chép mọi thứ từ đầu được đánh dấu của lựa chọn đến vị trí hiện tại.
* `Alt+Shift+F9` (macOS: `Cmd+Shift+F9`): Nhảy lại đầu được đánh dấu của lựa chọn, để lại dấu hiệu tại chỗ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím khác trong chế độ xem tài liệu

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu được chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Theo liên kết hoặc mở chế độ xem bảng hoặc công thức tại con trỏ.
* `Shift+F10` hoặc phím Menu/Application trong văn bản tài liệu: Mở menu ngữ cảnh.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, với nhiều ngôn ngữ được thêm vào mọi lúc. Danh sách đầy đủ như sau.

Để tìm hiểu cách đóng góp, vui lòng đọc [Hướng dẫn dịch của chúng tôi](translating.md).

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
* Vietnamese

## Đóng góp
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Quyên góp
Các người sau đây đã quyên góp một số tiền cho sự phát triển Paperback. Nếu bạn quyên góp, tên của bạn sẽ không được tự động thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi một nhà tài trợ GitHub công khai là lý do tự động đưa vào danh sách này.

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

### Phiên bản 0.9.2
* Sách nói không còn làm màn hình của bạn đọc một loạt khoảng trống khi bạn focus vào trường văn bản.
* Sách nói giờ đây đặt tên cho file khi bạn bước qua chúng theo phần.
* Sách nói giờ đây báo cáo độ dài thực tế của chúng, thay vì tuyên bố mỗi file trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn bật lên cảnh báo gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Select All giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần của nó được tải hiện tại.
* Find giờ đây cắt thẳng đến dòng tìm được, thay vì làm bạn nghe qua màn hình của bạn đọc ra lại cửa sổ khi focus quay trở lại sách.
* Sửa EPUB có chứa khối ZIP64 lạc lẫm từ chối mở với "Invalid local file header".
* Sửa các tài liệu dài quay lại đầu của chúng trong khi màn hình của bạn đọc liên tục qua chúng.
* Liên kết trong WebView giờ đây đưa bạn đến phần họ chỉ, thay vì thất bại với "File not found".
* Đánh dấu đầu của một lựa chọn bằng `Alt+F9`, sao chép mọi thứ từ đó đến nơi bạn đã đến bằng `Alt+F10`, và quay lại dấu hiệu bằng `Alt+Shift+F9`, để sao chép một khoảng văn bản dài mà không cần shift-mũi tên qua nó. Cả ba đều nằm dưới Tools > Select and copy.
* Phím tắt `=` giờ đây công bố trang cũng như phần trăm, ví dụ "15%, page 30", và giữ nguyên như trước đây đối với các tài liệu không có số trang.
* Thông báo "Document reloaded" tự động không còn cắt ngang màn hình của bạn đọc giữa câu, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab General của hộp thoại Settings giờ đây tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật trực tiếp sau tùy chọn kiểm tra cập nhật.
* Cập nhật giờ đây đưa cửa sổ được khởi chạy lại về phía trước, thay vì để nó ở phía sau mọi cửa sổ khác trong Alt+Tab.
* Windows sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì dòng tagline đầy đủ của chương trình.
* Word Count và Document Info giờ đây hiển thị có bao nhiêu file mà một sách nói giữ, và nó chạy bao lâu tổng cộng.

### Phiên bản 0.9.1
* Âm thanh dấu trang và ghi chú giờ đây phát trên macOS.
* Sách DAISY giờ đây phát âm thanh của chúng trên macOS, thay vì mở và theo dõi timeline của chúng im lặng.
* Sửa dấu ngoặc cong, dấu gạch ngang em và các ký tự tương tự biến mất khỏi tài liệu RTF, chạy các từ xung quanh với nhau khi chúng đi.
* Sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản bị méo.
* Sửa thực đơn con Recent Documents giữ lại các mục cũ cho đến khi điều gì đó khác xảy ra để tái xây dựng nó.
* Phím tắt bàn phím trở lại ở mọi bản dịch, vì vậy menu của Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn giờ đây mở nhanh hơn đến bảy lần.
* Các tài liệu được mở giờ đây được đăng ký với Windows, vì vậy chúng hiển thị trong danh sách jump list của taskbar và danh sách gần đây của menu Start.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback giờ đây ghi nhớ vị trí cửa sổ của nó, kích thước và trạng thái tối đa hóa giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm thứ gì đó đọc đúng cách trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY giờ đây mở toàn bộ sách nói thay vì chỉ văn bản của nó.
* Tên hành động của hộp thoại Customize Keyboard Shortcuts giờ đây có thể được dịch.
* Tiêu đề tài liệu giờ đây đến trước tiên trong thanh tiêu đề, vì vậy các sách được mở có thể được phân biệt trong taskbar và Alt+Tab.
* Hộp thoại cập nhật giờ đây được dịch.

### Phiên bản 0.9.0

#### Đã thêm

##### Chung
* Một công cụ CLI, gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn View Source để mở nguồn của tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu giờ đây được phân trang, có nghĩa là bạn có thể tải các sách với hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều gì kỳ lạ được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Chuyển đổi toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Một nút định vị để định vị các sách bị mất mà vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả các tài liệu.

##### Tùy chọn và Khả năng đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Bao quanh từ (chuyển từ chung);
    * Kết xuất bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ cái;
    * Căn chỉnh văn bản.
* Một mục menu bao quanh từ và phím nóng tiếp theo.
* Một chuyển đổi để xác định cách bạn muốn các bảng được hiển thị, và hợp nhất cách các bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Công thức MathML trong EPUB và HTML được kết xuất dưới dạng AsciiMath bằng MathCAT. Sử dụng `M` hoặc `Shift+M` để điều hướng công thức, sau đó nhấn `Enter` hoặc `Space` để mở MathML gốc trong Formula View.
* Hỗ trợ điều hướng theo bộ chứa.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong màn hình của bạn đọc.
* Phím tắt bằng cách để công bố phần trăm hiện tại của bạn qua một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một dấu mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một cái và dấu gạch chéo để nhảy đến nó.

##### Số từ
* Thời gian đọc ước tính trong hộp thoại số từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại số từ, bao nhiêu từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Hà Lan, Phần Lan và Ba Lan.

##### Xuất khẩu
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình cập nhật
* Một nút hủy vào hộp thoại cập nhật đang tiến hành.
* Trình cập nhật giờ đây xác thực tệp đã tải xuống chưa bị giả mạo.

##### Web View
* Webview giờ đây được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ cho các sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách nói
* Khả năng phát các sách nói, hiện hỗ trợ cả DAISY audio (bao gồm DAISY audio + text) và zip của các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời tường thuật, tìm kiếm về phía trước và phía sau, và điều chỉnh số lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu ngoặc đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn xem việc tìm kiếm quá cuối của một chương có tiếp tục vào chương tiếp theo không.

##### Tài liệu CHM
* Hỗ trợ cho danh sách, mục danh sách, hình và hình ảnh.

##### PowerPoint
* Các tài liệu PowerPoint hiện hỗ trợ các bảng.

#### Được sửa

##### Chung
* Các tài liệu được mã hóa trong các bảng mã CJK kế thừa, chẳng hạn như GBK, Big5 và Shift_JIS, giờ đây sẽ kết xuất đúng thay vì một đống mojibake.
* "Reopen last closed" cố gắng mở lại readme được đóng gói.
* Tab được chọn của bạn không được focus đúng cách sau khi khởi động lại Paperback.
* Xử lý Paperback của các tệp trên ổ cứng mạng Windows: nhấn show file in folder giờ đây đúng focus tệp trên lưu trữ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn bị tải bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một cái.
* Open containing folder giờ đây focus tệp đã cho trong trình khám phá.
* Mở readme giờ đây sẽ tôn trọng ngôn ngữ được chọn của bạn.
* Giao diện người dùng Paperback giờ đây sẽ được thu phóng đúng cách trên các màn hình DPI cao.
* Menu giờ đây cập nhật đúng cách, và focus di chuyển đến kiểm soát văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục per-character nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng một tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở thông qua Shift+F1.
* Xóa các tài liệu từ hộp thoại gần đây giờ đây cũng sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ đây được giữ lại sau khi xóa một tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú hiện nên phát đúng cách chỉ khi bạn điều hướng qua một từ chứa một từ.

##### Khả năng đọc
* Áp dụng word wrap bắn bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không có kích thước được thay đổi và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh giờ đây nên hiển thị đúng cách trong webview nhúng.

##### Trình cập nhật
* Trình cập nhật hiện nay đúng cách hiển thị nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải các sách DAISY có khai báo mã hóa giả mạo.

##### Tài liệu RTF
* Phân tích các tài liệu RTF có các ký tự không phải Latin.
* Các nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo filepos trong sách Mobi chia tách các thẻ HTML và đưa rác vào văn bản sách.
* Liên kết trong sách Mobi kế thừa.
* Phân tích AZW3 được cải thiện đáng kể.

##### Tài liệu Word
* Tài liệu Word có tên kiểu dáng cụ thể theo cụm từ không kết xuất tiêu đề đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong các tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ đây quay lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại hỗ trợ Word kế thừa, Word hiện đại và Powerpoint hiện đại, với Powerpoint kế thừa được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho các tài liệu Microsoft Word kế thừa!
* Thêm hỗ trợ cho các bản trình bày Powerpoint kế thừa!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt ctrl+q để thoát ứng dụng.
* Thêm hỗ trợ cho các sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản alt cho hình ảnh nhúng giờ đây nên được hiển thị đúng cách.
* Các tài liệu CHM giờ đây đúng cách hỗ trợ điều hướng liên kết nội bộ.
* Sửa go to page bị lệch 1.
* Sửa phím escape không hoạt động để đóng hộp thoại mở như.
* Sửa menu ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Sửa tài liệu sai đôi khi được focus khi mở các tài liệu từ dòng lệnh.
* Các PDF chỉ có hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua hình ảnh và hình với g/shift+g và f/shift+f tương ứng.
* Paperback giờ đây sẽ tôn trọng cài đặt chế độ tối ứng dụng của bạn.
* Loại bỏ hỗ trợ DAISY XML, vì nó không còn cần thiết.
* Chuyển lại để điều hướng ký tự đầu tiên Win32 gốc trong cây nội dung hình nón.
* Hộp thoại lỗi tải giờ đây hiển thị thông báo lỗi chi tiết hơn.
* Webview giờ đây sẽ mở nhanh hơn nhiều và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Sửa một lỗi khi mở webview trong epub chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Sửa một lỗi khi trình phân tích RTF sẽ không đặt dấu cách giữa các từ trong các trường hợp hiếm.
* Sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF giờ đây có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Tab RTF và dòng feed giờ đây được kết xuất chính xác như chúng xuất hiện trong tài liệu.
* Chuyển lại thư viện pdfium được thử nghiệm và đúng để phân tích PDF, làm cho kết xuất PDF đáng tin cậy hơn nhiều lần nữa.

### Phiên bản 0.8.1
* Thêm Ctrl+Shift+T để mở lại tài liệu đã đóng cuối cùng.
* Hộp thoại All Documents giờ đây hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Sửa một vài lỗi với trình phân tích RTF.
* Sửa các đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua một phiên bản Paperback thứ hai.
* Sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các chữ viết hoa.
* Sửa tải tài liệu chậm khi mở các tệp lớn.
* Sửa bản địa hóa các nút Có/Không trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung đơn giản và tiếng Việt!
* Thêm một trình cập nhật tự động sẽ giờ đây thay thế phiên bản Paperback đang cài đặt của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn để đạt được dấu trang hoặc ghi chú, cảm ơn Andre Louis về âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho các tài liệu DAISY XML.
* Thêm hỗ trợ cho các tệp Open Document Text được làm phẳng!
* Thêm hỗ trợ cho các bản trình bày Open Document được làm phẳng!
* Thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự giờ đây sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Sửa các tài liệu Markdown hiển thị văn bản thô thay vì HTML được kết xuất trong Web View.
* Sửa các bảng không kết xuất đúng cách trong các tệp Markdown.
* Các PDF chỉ có hình ảnh giờ đây sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một cái.
* Nhúng đúng cách thông tin phiên bản vào tệp Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu ngữ cảnh của kiểm soát văn bản giờ đây sẽ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng cách sử dụng T và Shift+T, và nhấn Enter để xem một trong một webview.
* Thêm một tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong một trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật giờ đây hiển thị ghi chú phát hành khi có phiên bản mới có sẵn.
* Sửa khôi phục cửa sổ từ khay hệ thống.
* Sửa bản dịch các nút Có/Không trong hộp thoại xác nhận.
* Sửa tải configs khi chạy với tư cách là quản trị viên.
* Sửa xử lý nhận xét trong các tài liệu XML và HTML.
* Sửa phân tích TOC trong sách Epub 2.
* Sửa điều hướng đến mục tiếp theo với cùng một chữ cái trong bảng nội dung.
* Sửa hộp thoại tìm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước.
* Sửa epub TOC đôi khi ném bạn đến mục sai.
* Sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Sửa lỗi off-by-one trong điều hướng liên kết.
* Sửa một số sách có khoảng trắng ở cuối dòng.
* Sửa các vấn đề trình phân tích khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách các phần tử giờ đây được tắt đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình lành dịch cho những người đóng góp.
* Nhiều cấu trúc lại nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm một tính năng go to previous/next position rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể điều hướng đến với các phím mũi tên alt+left/right.
* Thêm một danh sách các phần tử! Hiện tại nó chỉ hiển thị cây của tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm một tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Sửa liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Sửa phân tích Epub TOC chứa các đường dẫn tương đối.
* Sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/cancel trong hộp thoại TOC.
* Cải thiện xử lý tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói được nếu danh sách các tài liệu gần đây trống khi bạn cố gắng đưa lên hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu go ở dạng gọn gàng hơn nhiều đã được thêm vào hộp thoại tùy chọn, được chọn theo mặc định.
* Thêm một tùy chọn để điều hướng theo các phần tử cấu trúc bao quanh.
* Thêm một tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại được focus.
* Thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng bộ đếm thời gian ngủ cơ bản, có thể truy cập với Ctrl+Shift+S.
* Thêm hỗ trợ để phân tích sách điện tử FB2!
* Thêm hỗ trợ để phân tích bản trình bày OpenDocument!
* Thêm hỗ trợ để phân tích các tệp OpenDocument Text!
* Dấu trang giờ đây có thể được đặt để đánh dấu toàn bộ một dòng hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như pre-0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Dấu trang giờ đây có thể có ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú với N và Shift+N, hoặc bật lên hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ các không ghi chú được chọn với các phím nóng cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML giờ đây sẽ được xử lý đúng cách.
* Sửa tải các tài liệu Markdown lớn.
* Sửa nhấn dấu cách trong cây chế độ xem bảng nội dung kích hoạt nút OK.
* Sửa xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Sửa kiểm soát văn bản không lấy lại focus đôi khi khi quay lại cửa sổ Paperback.
* Sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Sửa kết xuất các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown giờ đây sẽ được kết xuất đúng cách.
* Nếu tải một sách với tham số dòng lệnh trong khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback với tư cách là quản trị viên, cấu hình giờ đây sẽ được tải và lưu đúng cách.
* Giờ đây có thể xóa một dấu trang trực tiếp từ bên trong hộp thoại dấu trang.
* Giờ đây có thể nhập và xuất dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp với phần mở rộng .paperback. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp trong khi tải nó, nó sẽ được tải tự động. Nếu không, bạn có thể tải thủ công bằng một mục trong menu công cụ.
* Liên kết bên trong tài liệu giờ đây được hỗ trợ đầy đủ! Sử dụng k và shift+k để di chuyển về phía trước và phía sau qua chúng, và nhấn enter để mở/kích hoạt một cái.
* Nhiều cấu trúc lại nội bộ, làm cho ứng dụng nhanh hơn và nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và các mục của chúng giờ đây được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi theo các danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Numpad delete giờ đây hoạt động để loại bỏ tài liệu khỏi thanh tab ngoài delete thông thường.
* Paperback giờ đây có thể tùy chọn giảm thiểu vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn giảm thiểu trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback giờ đây hoàn toàn có thể dịch được! Danh sách các ngôn ngữ nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang tăng lên liên tục!
* Paperback giờ đây có một trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX giờ đây sẽ hiển thị bảng nội dung cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu được mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt giờ đây bao gồm một tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách các tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó sẽ hiển thị cho bạn một số có thể tùy chỉnh được, với các tài liệu còn lại mà bạn đã từng mở có thể truy cập thông qua một hộp thoại nhỏ.
* Những cải thiện nhỏ khác nhau trên toàn bộ các trình phân tích, bao gồm đặt một dòng trống giữa các slide trong bản trình bày PPTX, sửa xử lý newline bên trong các đoạn trong tài liệu word và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ cho bản trình bày PowerPoint!
* Sửa các mục menu nhất định không bị tắt khi không có tài liệu nào được mở.
* Sửa định hướng của thanh trượt go to percent.
* Sửa bảng nội dung trong sách Epub với các đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Sửa khoảng trắng bị tước khỏi tiêu đề XHTML theo những cách lạ.
* Sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown giờ đây hỗ trợ tính năng bảng nội dung! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ xây dựng bảng nội dung của riêng nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị điều đó cho bạn trong hộp thoại ctrl+t.
* Tài liệu HTML giờ đây sẽ có tiêu đề được đặt trong thẻ tiêu đề, nếu nó tồn tại. Nếu không, họ sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng một khu vực trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL màn hình của bạn được gửi cùng với chương trình nữa, và nhiều màn hình của bạn đọc sẽ được hỗ trợ ngay bây giờ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt các sách epub rộng hơn.
* Hộp thoại hỏi bạn nếu bạn muốn mở tài liệu của bạn dưới dạng văn bản thuần túy đã được làm lại hoàn toàn, và giờ đây nó cho phép bạn mở tài liệu của bạn dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent giờ đây bao gồm một trường văn bản cho phép bạn thủ công nhập một phần trăm để nhảy đến.
* Trình phân tích HTML giờ đây sẽ nhận ra dd, dt và dl là các phần tử danh sách.
* Bảng nội dung trong sách Epub sẽ một lần nữa được bảo tồn chính xác.
* Khoảng trắng không ngắt unicode giờ đây được xem xét khi tước các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở một tệp không được công nhận mỗi lần bạn tải nó, chỉ là lần đầu tiên.

### Phiên bản 0.4.1
* Thêm một biểu tượng menu Start tùy chọn vào trình cài đặt.
* Bảng nội dung bây giờ nên sạch sẽ hơn trong một vài trường hợp, ví dụ nếu bạn có một mục con và cha với cùng một văn bản ở cùng vị trí bạn sẽ chỉ thấy mục cha.
* Sửa bảng nội dung trong một số tài liệu CHM.
* Sửa bảng nội dung trong sách Epub 3 với các đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM giờ đây nên hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang như bạn muốn trên bao nhiêu tài liệu như bạn muốn. Bạn có thể nhảy về phía trước và phía sau qua chúng với b và shift+b, đặt một cái với control+shift+b, và mang lên một hộp thoại để nhảy đến một dấu trang cụ thể với control+b.
* Thêm một trình cài đặt bên cạnh tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn, và tự động thiết lập các liên kết tệp cho bạn.
* Tệp văn bản với BOM giờ đây nên được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Thêm thông tin chi tiết hơn nhiều vào thanh trạng thái. Nó giờ đây sẽ hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Bình luận HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối cho Paperback trên dòng lệnh, nó giờ đây sẽ giải quyết nó đúng cách.
* Chuyển động theo phần trăm giờ đây được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập với control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả đã biết giờ đây sẽ luôn có mặc định.
* Logic lưu vị trí giờ đây thông minh hơn nhiều và chỉ nên ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã focus khi bạn đóng Paperback giờ đây được ghi nhớ trên các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page giờ đây nên được vệ sinh chặt chẽ hơn.
* Sửa điều hướng bảng nội dung trong sách epub 3 với các đường dẫn tương đối trong các bản kê khai của chúng.

### Phiên bản 0.3.0
* Sửa bảng nội dung trong sách epub với các bản kê khai được mã hóa URL.
* Sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Sửa sử dụng CPU cao trong tài liệu có các tiêu đề dài do một suy thoái trong wxWidgets.
* Sửa tải tệp văn bản UTF-8.
* Sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Sửa một tai nạn khi thoát ứng dụng trong một số trường hợp.
* Thêm một hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt bao quanh từ!
* Giờ đây có thể quyên góp cho sự phát triển Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc liên kết tài trợ dự án này ở dưới cùng của trang chính của kho lưu trữ GitHub.
* Tài liệu Markdown giờ đây sẽ luôn có một tiêu đề, và Paperback giờ đây có thể tải hầu hết bất kỳ tệp Markdown nào.
* Tài liệu PDF giờ đây sẽ luôn có một tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều trên toàn bộ.
* Bạn giờ đây chỉ có thể chạy một phiên bản Paperback tại một thời điểm. Chạy paperback.exe với một tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bạn giờ đây có thể nhấn delete trên một tài liệu trong kiểm soát tab để đóng nó.

### Phiên bản 0.2.1
* Thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách tài liệu được mở của bạn.
* Sửa các phím tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ của chúng.
* Paperback giờ đây sẽ loại bỏ các dấu gạch ngang mềm không cần thiết khỏi đầu ra văn bản.
* Sửa điều hướng tiêu đề đôi khi đặt bạn vào ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm phím tiêu đề để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Những phím tiêu đề này được thiết kế để hoạt động tương tự như một màn hình của bạn đọc.
* Sửa tải epub với tên tệp được mã hóa URL trong bản kê khai của chúng.
* Sửa tải sách epub 3 với XHTML nhúng bên trong chúng.
* Một thông báo giờ đây được nói nếu tài liệu không hỗ trợ bảng nội dung hoặc phần, trái ngược với các mục menu bị tắt.
* Thêm một menu tài liệu gần đây! Nó hiện tại lưu 10 tài liệu được mở cuối cùng của bạn, và nhấn enter trên một cái sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm một lịch sử 25 lần tìm kiếm cuối cùng và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây giờ đây được ghi nhớ trên các lần khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Thêm shift+f1 để mở readme trực tiếp trong Paperback itself.

### Phiên bản 0.1.0
* Phát hành ban đầu.
