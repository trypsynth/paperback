<!-- machine-translated from doc/readme.md (source-hash: 13c58fb50049f608); please review and edit as needed -->

# Paperback - phiên bản 0.9.1

## Giới thiệu

Paperback là một ứng dụng đọc sách điện tử và tài liệu nhẹ, nhanh chóng và dễ tiếp cập cho mọi người, từ những độc giả bình thường đến những người dùng nâng cao. Nó được thiết kế để có khả năng tiếp cập màn hình đọc, tốc độ nhanh và trải nghiệm không có tính năng không cần thiết.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản macOS ARM hiện đại. Các ứng dụng iOS và Android gốc đang được phát triển tích cực, với các bản dựng thử nghiệm công khai được lên kế hoạch sớm sau khi phát hành desktop 0.9.0, trước khi phát hành thống nhất 1.0 bao gồm cả bốn nền tảng.

## Các tính năng

* Hoàn toàn độc lập, không yêu cầu bất kỳ phần mềm nào được cài đặt trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh chóng, ngay cả trên phần cứng cũ.
* Giao diện được chia thành các tab đơn giản, cho phép bạn mở bao nhiêu tài liệu tùy thích cạnh nhau.
* Lưu vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đã mở khi đóng chương trình và khôi phục chúng khi khởi động lần tới.
* Bao gồm chức năng điều hướng tương tự như chế độ duyệt web của nhiều trình đọc màn hình để điều hướng nhanh chóng và dễ dàng qua các tài liệu.
* Bao gồm hộp thoại tìm kiếm mạnh mẽ, bao gồm các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn di động hoặc được cài đặt với các liên kết tệp được thiết lập tự động.
* Hỗ trợ một loạt lớn các định dạng tệp phổ biến.

## Khả năng tương thích với trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết cho người dùng JAWS.

### JAWS và Hiển thị Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn dài bị cắt ngắn khi xoay tiến với các phím điều hướng của màn hình. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách JAWS xử lý kiểm soát văn bản RICHEDIT50W, không phải là lỗi ở Paperback, và là lỗi mất khá nhiều thời gian để nâng lên từng bước để thể hiện một bản sửa chữa cho sự phấn khích của Vispero đối với việc trả lời các vấn đề với phần mềm nguồn mở.

Giải pháp, cuối cùng được tiết lộ thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không hiển thị của bạn sẽ ở lại đoạn hoạt động chứ không phải tiến hành. Với cả hai cài đặt, xoay ngang sẽ hoạt động chính xác.

## Các loại tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bài thuyết trình PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp văn bản thuần túy và nhật ký (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng ưu tiên bàn phím. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Khi macOS khác, phím tương đương được ghi chú trong dấu ngoặc — chủ yếu là vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã được yêu cầu bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu Tệp

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đã đóng lần cuối.
* `Ctrl+R`: Hiển thị hộp thoại "Tất cả Tài liệu" (từ Tài liệu Gần đây).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS tùy chọn này nằm trong menu ứng dụng thay vào đó).

### Menu Đi

* `Ctrl+F`: Hiển thị hộp thoại Tìm kiếm.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi đến dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi đến phần trăm.
* `Ctrl+P`: Đi đến trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo tỷ lệ phần trăm đọc hiện tại của bạn.
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
* `Ctrl+B`: Nhảy đến tất cả các dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy đến chỉ các dấu trang.
* `Ctrl+Alt+M`: Nhảy đến chỉ các ghi chú.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý chứ không phải Cmd): Xem văn bản ghi chú tại vị trí hiện tại.
* `Shift+K`: Liên kết trước.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình trước.
* `F`: Hình tiếp theo.
* `Shift+T`: Bảng trước.
* `T`: Bảng tiếp theo.
* `Shift+S`: Dấu phân cách trước.
* `S`: Dấu phân cách tiếp theo.
* `Shift+L`: Danh sách trước.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi đến đầu hộp chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi quá cuối hộp chứa hiện tại (danh sách hoặc bảng).

### Menu Công cụ

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý chứ không phải Cmd): Hiển thị số lượng từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần.
* `Ctrl+Shift+B`: Chuyển đổi dấu trang tại lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang tại lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Chuyển đổi bao ngoại từ.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh về phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh về phía sau.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Chuyển đổi toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Tùy chỉnh, dưới menu ứng dụng).
* `Ctrl+Shift+S`: Chuyển đổi hẹn giờ ngủ.

### Menu Trợ giúp

* `Ctrl+F1`: Hiển thị hộp thoại Giới thiệu.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím bổ sung trong chế độ xem tài liệu

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu được chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết ở con trỏ, hoặc mở chế độ xem bảng khi trên dấu tích bảng.
* `Shift+F10` hoặc phím Menu/Ứng dụng trong văn bản tài liệu: Mở menu ngữ cảnh.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, với ngày càng nhiều thêm vào. Danh sách đầy đủ theo dõi bên dưới.

Để tìm hiểu cách đóng góp, vui lòng đọc [Hướng dẫn Dịch thuật](translating.md) của chúng tôi.

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

## Ghi nhận
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: cộng tác viên chính.

### Quyên góp
Các người sau đây đã thực hiện các khoản quyên góp có kích thước cho phát triển Paperback. Nếu bạn thực hiện quyên góp, tên của bạn sẽ không được tự động thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi nhà tài trợ GitHub công khai là lý do để tự động đưa vào danh sách này.

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
* Sách nói không còn khiến trình đọc màn hình của bạn đọc một loạt khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách nói hiện đặt tên tệp khi bạn điều hướng qua chúng theo phần.
* Sách nói hiện báo cáo độ dài thực tế của chúng, thay vì khẳng định mọi tệp trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn làm hiện cảnh báo gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Select All hiện cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần hiện được tải.
* Find hiện chuyển thẳng đến dòng nó tìm thấy, thay vì khiến bạn phải chờ trình đọc màn hình đọc cửa sổ lại khi con trỏ quay trở lại sách.
* Đã sửa EPUB mang khối ZIP64 lạc từng không mở được với "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại điểm bắt đầu của chúng khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView hiện đưa bạn đến phần mà chúng trỏ tới, thay vì thất bại với "File not found".
* Thông báo tự động "Document reloaded" không còn cắt đứt trình đọc màn hình của bạn ở giữa câu, thay vào đó chờ nó kết thúc những gì nó đang nói.
* Tab General của hộp thoại Settings hiện lần qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật trực tiếp sau tùy chọn kiểm tra cập nhật.
* Windows sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì tagline đầy đủ của chương trình.
* Word Count và Document Info hiện cho biết có bao nhiêu tệp trong sách nói, và tổng thời gian nó chạy.

### Phiên bản 0.9.1
* Âm thanh trang sách và ghi chú hiện phát được trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Đã sửa dấu ngoặc cong, dấu gạch dài và các ký tự tương tự biến mất từ các tài liệu RTF, làm chạy các từ xung quanh lại với nhau.
* Đã sửa các ảnh RTF lọt dữ liệu thô của chúng vào tài liệu dưới dạng văn bản bị hỏng.
* Đã sửa menu Documents gần đây giữ các mục cũ cho đến khi có thứ gì khác xảy ra để xây dựng lại nó.
* Các bộ tăng tốc bàn phím đã quay trở lại trong mọi bản dịch, vì vậy menu của Nga hiện có quyền truy cập bàn phím.
* Các tài liệu CHM lớn hiện mở nhanh hơn tới bảy lần.
* Các tài liệu được mở hiện được đăng ký với Windows, vì vậy chúng hiển thị trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Start.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback hiện ghi nhớ vị trí cửa sổ, kích thước và trạng thái phóng to giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm những thứ đọc đúng cách trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của một sách DAISY hiện mở toàn bộ sách âm thanh thay vì chỉ văn bản của nó.
* Các tên hành động của hộp thoại Customize Keyboard Shortcuts hiện có thể được dịch.
* Tiêu đề tài liệu hiện xuất hiện trước tiên trên thanh tiêu đề, vì vậy các sách mở có thể phân biệt được trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Được thêm vào

##### General
* Một công cụ CLI gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn View Source để mở mã nguồn của tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, có nghĩa là bạn có thể tải sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Một nút bật/tắt toàn màn hình.

##### Hộp thoại All Documents
* Một nút định vị để định vị sách bị thiếu vừa thay đổi đường dẫn của chúng.
* Một bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Lối tắt `Ctrl+Shift+A` để bỏ chọn tất cả các tài liệu.

##### Tùy chọn và khả năng đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Word wrap (được chuyển từ general);
    * Render tables inline (mới trong bản phát hành này, xem bên dưới);
    * Font;
    * Background color;
    * Line spacing;
    * Paragraph spacing;
    * Letter spacing;
    * Text alignment.
* Một mục menu word wrap và phím tắt tiếp theo.
* Một nút bật/tắt để xác định cách bạn muốn các bảng được hiển thị và hợp nhất cách các bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo container.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Lối tắt bàn phím bằng để thông báo phần trăm hiện tại của bạn qua tài liệu.

##### Trang sách
* Trang sách tạm thời: bạn có thể có một trang trên mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một cái và dấu gạch chéo ngược để nhảy tới nó.

##### Word Count
* Thời gian đọc ước tính trong hộp thoại số từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại số từ, sẽ hiển thị bao nhiêu từ bạn đã chọn.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, tiếng Phần Lan và tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Bộ cập nhật
* Một nút hủy để cập nhật hộp thoại đang diễn ra.
* Bộ cập nhật hiện xác thực tệp đã tải xuống chưa bị giả mạo.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát DAISY 2.02.

##### Sách nói
* Khả năng phát sách nói, hiện hỗ trợ cả âm thanh DAISY (bao gồm âm thanh DAISY + văn bản) và các tệp zip của các tệp âm thanh.
* Phím tắt bàn phím và các mục menu để phát/tạm dừng tiếng kể, tìm kiếm về phía trước và phía sau, và điều chỉnh lượng tìm kiếm.
* Các tùy chọn để đồng bộ hóa dấu hiệu đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn liệu tìm kiếm qua phần cuối của chương có tiếp tục vào phần tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Được sửa

##### General
* Các tài liệu được mã hóa trong các bảng mã CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị đúng cách thay vì như một loạt mojibake.
* "Reopen last closed" cố gắng mở lại tệp readme được đóng gói.
* Tab được chọn của bạn không được tập trung đúng cách sau khi khởi động lại Paperback.
* Cách xử lý các tệp trên ổ đĩa mạng Windows của Paperback: nhấn show file in folder hiện đúng tập trung vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn được tải bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được hỏi xác nhận khi tìm thấy một tệp.
* Open containing folder hiện tập trung vào tệp đã cho trong explorer.
* Mở readme hiện sẽ tôn trọng ngôn ngữ được chọn của bạn.
* Giao diện người dùng của Paperback hiện sẽ được mở rộng đúng cách trên các màn hình có DPI cao.
* Menu hiện được cập nhật đúng cách, và con trỏ di chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang một phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động hiện sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục mỗi ký tự nội bộ.

##### Hộp thoại All Documents
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại all documents.
* Readme.html sẽ không còn được thêm vào danh sách all documents của bạn khi được mở thông qua Shift+F1.
* Loại bỏ tài liệu khỏi hộp thoại recents hiện cũng sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được giữ nguyên sau khi loại bỏ tài liệu.

##### Điều hướng
* Điều hướng trang báo cáo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu đã tải trong các tài liệu lớn.

##### Trang sách
* Âm thanh trang sách/ghi chú hiện phát đúng cách khi bạn điều hướng qua một từ chứa một từ.

##### Khả năng đọc
* Áp dụng word wrap đưa bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Các ảnh hiện phải hiển thị đúng cách trong webview nhúng.

##### Bộ cập nhật
* Bộ cập nhật hiện hiển thị đúng nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Sách DAISY tải với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích các tài liệu RTF với các ký tự không phải Latin trong chúng.
* Các nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các anchor filepos trong sách Mobi chia các thẻ HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Phân tích AZW3 được cải thiện rất nhiều.

##### Tài liệu Word
* Tài liệu Word với tên kiểu cụ thể từng vị trí không hiển thị tiêu đề đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay trở lại trích xuất văn bản thuần túy cho PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang sẽ không còn làm cho Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang cho sách epub.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại hỗ trợ Legacy Word, modern Word và modern Powerpoint, với legacy Powerpoint được lên kế hoạch cho tương lai.
* Đã thêm hỗ trợ cho các tài liệu Microsoft Word cũ!
* Đã thêm hỗ trợ cho các bài thuyết trình Powerpoint cũ!
* Đã thêm hỗ trợ cho sách mobi và AZW3!
* Đã thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Đã thêm lối tắt ctrl+q để thoát ứng dụng.
* Đã thêm hỗ trợ cho các sách zip từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng hiện phải được hiển thị đúng cách.
* Tài liệu CHM hiện hỗ trợ đúng điều hướng liên kết nội bộ.
* Đã sửa go to page sai 1.
* Đã sửa phím Escape không hoạt động để đóng hộp thoại open as.
* Đã sửa menu bối cảnh của trình đọc không hiển thị trên chuột phải hoặc phím Applications.
* Đã sửa tài liệu sai đôi khi được tập trung khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ chứa hình ảnh một lần nữa được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Hiện có thể điều hướng qua hình ảnh và hình vẽ bằng g/shift+g và f/shift+f, tương ứng.
* Paperback hiện sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã loại bỏ hỗ trợ DAISY XML vì không còn cần thiết.
* Chuyển ngược trở lại điều hướng chữ cái đầu tiên Win32 gốc trong cây Mục lục.
* Hộp thoại lỗi tải hiện hiển thị các tin nhắn lỗi chi tiết hơn.
* Webview hiện sẽ mở nhanh hơn nhiều và mượt mà hơn.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi khi mở webview trong epub chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi khi trình phân tích cú pháp RTF sẽ không đặt khoảng trắng giữa các từ trong trường hợp hiếm hoi.
* Đã sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Các tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Tab RTF và nguồn cấp dữ liệu dòng hiện được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Chuyển ngược trở lại thư viện pdfium đã được thử nghiệm, làm cho kết xuất PDF nhiều lần tin cây hơn.

### Phiên bản 0.8.1
* Đã thêm Ctrl+Shift+T để mở lại tài liệu đóng cuối cùng.
* Hộp thoại All Documents hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích cú pháp RTF.
* Đã sửa các đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng trắng không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa của các nút Yes/No trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Đã thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Đã thêm một bộ cập nhật tự động sẽ thay thế phiên bản Paperback hiện được cài đặt của bạn thay vì chỉ tải xuống phiên bản mới!
* Đã thêm phản hồi âm thanh tùy chọn để tới dấu trang hoặc ghi chú, cảm ơn Andre Louis vì các âm thanh!
* Đã thêm hỗ trợ tài liệu RTF!
* Đã thêm hỗ trợ cho các tài liệu DAISY XML.
* Đã thêm hỗ trợ cho các tệp Open Document Text phẳng!
* Đã thêm hỗ trợ cho các bài thuyết trình Open Document phẳng!
* Đã thêm hỗ trợ cho dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Đã sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa các tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Web View.
* Đã sửa bảng không hiển thị đúng cách trong các tệp Markdown.
* Các PDF chỉ chứa hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một hình ảnh.
* Nhúng đúng cách thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Cơ sở mã mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu bối cảnh của điều khiển văn bản hiện sẽ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Đã thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một cái trong webview.
* Đã thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Đã thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi có phiên bản mới.
* Đã sửa khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Yes/No trong hộp thoại xác nhận.
* Đã sửa tải configs khi chạy dưới quyền quản trị viên.
* Đã sửa xử lý nhận xét trong tài liệu XML và HTML.
* Đã sửa phân tích TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ trong mục lục.
* Đã sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Đã sửa TOC epub đôi khi đưa bạn đến mục sai.
* Đã sửa nhiều vấn đề xử lý khoảng trắng trong thẻ XML, HTML và pre.
* Đã sửa lỗi sai một trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng ở cuối dòng của chúng.
* Đã sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến trang sách cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều cấu trúc lại nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Đã thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Đã thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó hiện sẽ được ghi nhớ và có thể điều hướng đến bằng các mũi tên alt+left/right.
* Đã thêm danh sách phần tử! Hiện tại nó chỉ hiển thị cây của tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Đã thêm tùy chọn để khởi động Paperback ở chế độ tối đa theo mặc định.
* Đã sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích Epub TOC chứa đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/cancel trong hộp thoại TOC.
* Cải thiện xử lý tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi được nói nếu danh sách các tài liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu đi theo hình thức nhỏ gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được kiểm tra theo mặc định.
* Đã thêm tùy chọn để điều hướng theo các phần tử cấu trúc bọc lại.
* Đã thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện được tập trung.
* Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng trình hẹn giờ ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Đã thêm hỗ trợ để phân tích cú pháp sách điện tử FB2!
* Đã thêm hỗ trợ để phân tích cú pháp bài thuyết trình OpenDocument!
* Đã thêm hỗ trợ để phân tích cú pháp các tệp OpenDocument Text!
* Trang sách hiện có thể được tạo để đánh dấu toàn bộ dòng hoặc để đánh dấu chỉ một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như pre-0.6 và sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Trang sách hiện có thể có ghi chú văn bản tùy chọn được đính kèm! Điều hướng giữa các trang sách chứa ghi chú với N và Shift+N, hoặc bật hộp thoại trang sách với tất cả các trang sách, chỉ ghi chú hoặc chỉ các trang sách không có ghi chú được chọn bằng các phím nóng cụ thể.
* Trang sách trong hộp thoại trang sách sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML hiện sẽ được xử lý đúng cách.
* Đã sửa tải các tài liệu Markdown lớn.
* Đã sửa nhấn dấu cách trong cây chế độ xem Mục lục kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa điều khiển văn bản đôi khi không lấy lại tiêu điểm khi quay trở lại cửa sổ Paperback.
* Đã sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Đã sửa hiển thị các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong các khối mã Markdown hiện sẽ được hiển thị đúng cách.
* Nếu tải một sách với tham số dòng lệnh khi phiên bản Paperback hiện có sẵn đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình hiện sẽ được tải và lưu đúng cách.
* Hiện có thể xóa trang sách trực tiếp từ trong hộp thoại trang sách.
* Hiện có thể nhập và xuất trang sách và vị trí đọc cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp với phần mở rộng .paperback. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải nó, nó sẽ được tự động tải. Ngoài ra, bạn có thể nhập chúng theo cách thủ công bằng cách sử dụng mục trong menu công cụ.
* Các liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng k và shift+k để chuyển động về phía trước và về phía sau qua chúng, và nhấn enter để mở/kích hoạt một.
* Nhiều cấu trúc lại nội bộ, làm cho ứng dụng nhanh hơn và nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi hiển thị.
* Điều hướng theo danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi theo chính danh sách, và I và Shift+I để đi qua các mục danh sách.
* Numpad delete hiện hoạt động để loại bỏ tài liệu từ thanh tab ngoài delete thông thường.
* Paperback hiện có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback hiện hoàn toàn có thể dịch được! Danh sách các ngôn ngữ mà nó hỗ trợ hiện khá nhỏ, nhưng nó luôn phát triển!
* Paperback hiện có trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu được mở hiện sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách các tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị cho bạn 10 tài liệu cuối cùng bạn đã mở, nó sẽ hiển thị cho bạn một số tùy chỉnh, với phần còn lại của các tài liệu bạn đã mở được truy cập qua một hộp thoại nhỏ.
* Các cải tiến nhỏ khác nhau cho các trình phân tích cú pháp trên toàn bộ bảng, bao gồm đặt dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý dòng mới bên trong các đoạn trong tài liệu word và thêm các dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho bài thuyết trình PowerPoint!
* Đã sửa một số mục menu không được vô hiệu hóa khi không có tài liệu nào mở.
* Đã sửa hướng của thanh trượt go to percent.
* Đã sửa mục lục trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng được tách từ tiêu đề XHTML theo những cách lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Các tài liệu dựa trên HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục của riêng nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị điều đó cho bạn trong hộp thoại ctrl+t.
* Các tài liệu HTML sẽ hiện có tiêu đề như được đặt trong thẻ tiêu đề, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không cần phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo lời nói. Điều này có nghĩa là không có DLL trình đọc màn hình được vận chuyển cùng với chương trình, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại yêu cầu bạn có muốn mở tài liệu dưới dạng văn bản thuần túy đã được viết lại hoàn toàn, và hiện cho phép bạn mở tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm một trường văn bản cho phép bạn nhập thủ công tỷ lệ phần trăm để nhảy tới.
* Trình phân tích cú pháp HTML sẽ nhận ra dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo toàn chính xác một lần nữa.
* Không gian không ngắt unicode hiện được xem xét khi tách các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở một tệp không được nhận ra mỗi lần bạn tải nó, chỉ khi lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng menu start tùy chọn vào trình cài đặt.
* Mục lục hiện phải sạch hơn trong một vài trường hợp, ví dụ: nếu bạn có mục con và cha có cùng văn bản ở cùng vị trí, bạn sẽ chỉ thấy mục cha.
* Đã sửa mục lục trong một số tài liệu CHM.
* Đã sửa mục lục trong sách Epub 3 với đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM hiện phải hiển thị tiêu đề của chúng như được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ trang sách! Bạn có thể có bao nhiêu trang sách trên bao nhiêu tài liệu tùy thích. Bạn có thể nhảy về phía trước và phía sau qua chúng với b và shift+b, đặt một cái với control+shift+b, và mở hộp thoại để nhảy tới trang sách cụ thể với control+b.
* Đã thêm trình cài đặt bên cạnh tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện phải được giải mã đúng cách, và BOM sẽ không còn hiển thị ở đầu văn bản nữa.
* Đã thêm thông tin chi tiết hơn nhiều vào thanh trạng thái. Nó sẽ hiển thị cho bạn dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Nhận xét HTML cũng như nội dung của các thẻ tập lệnh và kiểu sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ hiện giải quyết nó đúng cách.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập được với control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả đã biết sẽ luôn có một mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ phải ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi đóng Paperback hiện được ghi nhớ trên toàn bộ khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page hiện phải được tiêu trùng hơn.
* Đã sửa điều hướng mục lục trong sách epub 3 với đường dẫn tương đối trong bản kê khai của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub với bản kê khai được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode đa byte.
* Đã sửa mức sử dụng CPU cao trong tài liệu có tiêu đề dài do suy thoái trong wxWidgets.
* Đã sửa tải các tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa sự cố khi thoát ứng dụng trong một số trường hợp.
* Đã thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt word wrap!
* Hiện có thể quyên góp cho phát triển Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết tài trợ dự án ở dưới cùng của trang chính của kho GitHub.
* Tài liệu Markdown sẽ luôn có tiêu đề, và Paperback hiện phải có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF nhiều lần tin cây trên toàn bảng.
* Bạn hiện chỉ có thể chạy một phiên bản Paperback cùng một lúc. Chạy paperback.exe với tên tệp khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bạn hiện có thể nhấn xóa trên tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm số trang tổng cộng vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách các tài liệu được mở của bạn.
* Đã sửa các phím gõ tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ chúng.
* Paperback hiện sẽ loại bỏ dấu gạch dưới mềm không cần thiết từ đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn trên ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím gõ để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím gõ này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa tải epub với tên tệp được mã hóa URL trong bản kê khai của chúng.
* Đã sửa tải sách epub 3 với XHTML nhúng bên trong chúng.
* Một tin nhắn hiện được nói nếu tài liệu không hỗ trợ mục lục hoặc phần, ngược lại với các mục menu bị vô hiệu hóa.
* Đã thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu được mở cuối cùng của bạn, và nhấn enter trên một cái sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm kiếm, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử 25 tìm kiếm cuối cùng và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây hiện được ghi nhớ trên toàn bộ khởi động lại ứng dụng. Điều này có thể được cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Đã thêm shift+f1 để mở readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
