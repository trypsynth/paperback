<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc); please review and edit as needed -->

# Paperback - phiên bản 0.9.2

## Giới thiệu

Paperback là một trình đọc sách điện tử và tài liệu nhẹ, nhanh và dễ tiếp cận cho tất cả mọi người, từ những độc giả bình thường đến những người dùng nâng cao. Nó được thiết kế với khả năng tiếp cận của trình đọc màn hình, tốc độ nhanh và trải nghiệm không có tính năng không cần thiết.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản hiện đại của ARM macOS. Các ứng dụng iOS và Android gốc đang được phát triển tích cực, với các bản dựng thử nghiệm công khai dự kiến sẽ ra mắt không lâu sau bản phát hành desktop 0.9.0, trước khi phát hành bản thống nhất 1.0 bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu cài đặt bất kỳ phần mềm nào trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở nhiều tài liệu cùng lúc.
* Lưu vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn nhớ những tài liệu bạn đã mở khi đóng chương trình và khôi phục chúng vào lần khởi chạy tiếp theo.
* Bao gồm chức năng điều hướng tương tự như chế độ duyệt web của nhiều trình đọc màn hình để điều hướng nhanh và dễ dàng qua các tài liệu.
* Bao gồm một hộp thoại tìm kiếm mạnh mẽ, với các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn độc lập hoặc được cài đặt với các liên kết tệp được thiết lập tự động.
* Hỗ trợ một mảng lớn các định dạng tệp phổ biến.

## Khả năng tương thích với trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết đối với người dùng JAWS.

### JAWS và các thiết bị Braille

Nếu bạn sử dụng JAWS với thiết bị Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi cuộn về phía trước với các phím điều hướng của thiết bị của bạn. Lệnh đọc đoạn văn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải là điều gì đó trong chính Paperback, và điều mà cần khá lâu để tìm ra được một bản sửa chữa do sự nhiệt tình của Vispero trong việc phản hồi các vấn đề với phần mềm nguồn mở.

Cách giải quyết, cuối cùng được phát hiện thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng muốn bật "Pan Text by Paragraph", nếu không thiết bị của bạn sẽ ở lại đoạn văn hoạt động chứ không tiến lên. Với cả hai cài đặt đã có, cuộn sẽ hoạt động chính xác.

## Các loại tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách nói M4B (`.m4b`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bài thuyết trình PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Các phím tắt

Paperback được thiết kế để sử dụng với bàn phím trước tiên. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Nếu macOS khác, phím tương đương được ghi chú trong dấu ngoặc — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã bị chiếm dụng bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đã đóng lần cuối.
* `Ctrl+R`: Hiển thị hộp thoại "Tất cả Tài liệu" (từ Tài liệu Gần đây).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS điều này nằm trong menu ứng dụng thay vào đó).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Tìm.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi tới dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi tới phần trăm.
* `Ctrl+P`: Đi tới trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo phần trăm đọc hiện tại của bạn.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến lên trong lịch sử điều hướng.
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
* `\`: Chuyển tới dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước đó.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Chuyển tới tất cả dấu trang và ghi chú.
* `Ctrl+Alt+B`: Chuyển tới dấu trang chỉ.
* `Ctrl+Alt+M`: Chuyển tới ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
* `Shift+K`: Liên kết trước đó.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước đó.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình trước đó.
* `F`: Hình tiếp theo.
* `Shift+T`: Bảng trước đó.
* `T`: Bảng tiếp theo.
* `Shift+S`: Dấu phân cách trước đó.
* `S`: Dấu phân cách tiếp theo.
* `Shift+L`: Danh sách trước đó.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước đó.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi tới đầu vùng chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi qua cuối vùng chứa hiện tại (danh sách hoặc bảng).

### Menu Tools

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý thay vì Cmd): Hiển thị số từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần.
* `Ctrl+Shift+B`: Chuyển đổi dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Chuyển đổi gói dòng từ.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh về phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh về phía sau.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Chuyển đổi toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Tùy chọn, trong menu ứng dụng).
* `Ctrl+Shift+S`: Chuyển đổi bộ hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Phím chế độ xem tài liệu bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu được chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết ở con trỏ, hoặc mở chế độ xem bảng khi ở trên dấu bảng.
* `Shift+F10` hoặc phím Menu/Application trong văn bản tài liệu: Mở menu ngữ cảnh.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, với ngày càng nhiều thêm. Danh sách đầy đủ như sau.

Để tìm hiểu cách đóng góp, vui lòng đọc [Hướng dẫn Dịch của chúng tôi](translating.md).

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

## Lời cảm ơn
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Quyên góp
Các người sau đây đã quyên góp với sự phát triển của Paperback. Nếu bạn quyên góp, tên của bạn sẽ không tự động được thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi một nhà tài trợ GitHub công khai là cơ sở để tự động đưa vào danh sách này.

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
* Sách âm thanh không còn làm trình đọc màn hình của bạn đọc một chuỗi khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách âm thanh hiện đặt tên tệp khi bạn bước qua chúng theo phần.
* Sách âm thanh hiện báo cáo thời lượng thực của chúng, thay vì yêu cầu mọi tệp trong đó chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã theo dõi liên kết bên trong nó.
* Sao chép sau Chọn tất cả hiện cung cấp toàn bộ tài liệu, thay vì chỉ phần đã tải hiện tại.
* Tìm kiếm bây giờ đi thẳng đến dòng mà nó tìm thấy, thay vì làm bạn ngồi chờ trình đọc màn hình đọc lại cửa sổ khi tiêu điểm trở lại sách.
* Đã sửa EPUB có khối ZIP64 lạc từng từ chối mở với "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại điểm bắt đầu trong khi trình đọc màn hình đọc liên tục.
* Các liên kết trong WebView hiện đưa bạn đến phần mà chúng trỏ tới, thay vì gặp lỗi "File not found".
* Thông báo tự động "Document reloaded" không còn cắt trình đọc màn hình của bạn giữa câu, thay vào đó chờ cho nó kết thúc những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt hiện tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Windows sẽ luôn hiển thị "Paperback" trong menu Mở bằng, thay vì dòng thẻ đầy đủ của chương trình.
* Số từ và Thông tin tài liệu hiện hiển thị có bao nhiêu tệp mà một cuốn sách âm thanh giữ và thời lượng chạy của nó tổng cộng.

### Phiên bản 0.9.1
* Âm thanh đánh dấu trang và ghi chú bây giờ phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Đã sửa dấu ngoặc kép cong, dấu gạch ngang dài và các ký tự tương tự biến mất khỏi tài liệu RTF, chạy các từ xung quanh chúng lại với nhau khi chúng đi.
* Đã sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản lộn xộn.
* Đã sửa menu Tài liệu gần đây giữ các mục cũ cho đến khi điều gì đó khác xảy ra để xây dựng lại nó.
* Các phím tắt bàn phím đã quay trở lại trong mọi bản dịch, vì vậy menu của Tiếng Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn bây giờ mở nhanh hơn tới bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Tùy chọn đã được đổi tên thành Cài đặt, khớp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback hiện nhớ vị trí cửa sổ, kích thước và trạng thái tối đa hóa của nó giữa các lần chạy.
* Các hình thức số nhiều hiện được dịch, vì vậy các thông báo đếm các điều đọc đúng cách trong các ngôn ngữ cần nhiều hơn một hình thức.
* Chọn ncc.html của sách DAISY hiện mở cuốn sách âm thanh hoàn chỉnh thay vì chỉ văn bản của nó.
* Các tên hành động của hộp thoại Tùy chỉnh phím tắt bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu hiện đặt trước trong thanh tiêu đề, vì vậy các cuốn sách đang mở có thể được phân biệt trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Đã thêm

##### Chung
* Một công cụ CLI, gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ sang HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã bị sửa đổi bởi các chương trình khác trên đĩa.
* Một tùy chọn Xem nguồn để mở nguồn của tài liệu trong một tab mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, nghĩa là bạn có thể tải sách với hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ sự lạ hoắc nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Một chế độ toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Một nút định vị để định vị các sách bị mất chỉ thay đổi đường dẫn của chúng.
* Một bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Xuống dòng từ (được chuyển từ chung);
    * Hiển thị bảng nội tuyến (mới trong phiên bản này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn văn;
    * Khoảng cách chữ;
    * Căn chỉnh văn bản.
* Một mục menu xuống dòng từ và phím nóng tiếp theo.
* Một bộ chuyển đổi để xác định cách bạn muốn hiển thị bảng và thống nhất cách hiển thị bảng trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo vùng chứa.
* Một tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để công bố phần trăm hiện tại của bạn trong tài liệu.

##### Đánh dấu trang
* Đánh dấu trang tạm thời: bạn có thể có một mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Số từ
* Thời gian đọc ước tính trong hộp thoại số từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại số từ, số lượng từ bạn đã chọn bây giờ sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, tiếng Phần Lan và tiếng Ba Lan.

##### Xuất khẩu
* Mở rộng mục xuất khẩu để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình cập nhật
* Một nút hủy để cập nhật hộp thoại đang diễn ra.
* Trình cập nhật hiện xác thực tệp đã tải xuống chưa bị giả mạo.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát DAISY 2.02 âm thanh.

##### Sách âm thanh
* Khả năng phát sách âm thanh, hiện hỗ trợ cả DAISY âm thanh (bao gồm DAISY âm thanh + văn bản) và zip các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời nói, tìm kiếm tiến và lùi, và điều chỉnh lượng tìm kiếm.
* Các tùy chọn để đồng bộ hóa dấu đọc với phát âm thanh, đặt lượng tìm kiếm âm thanh và chọn liệu tìm kiếm vượt quá cuối chương có tiếp tục sang chương tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình vẽ và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã sửa

##### Chung
* Các tài liệu được mã hóa trong mã hóa CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị chính xác thay vì một loạt mojibake.
* "Mở lại cái được đóng cuối cùng" cố gắng mở lại tệp readme được đóng gói.
* Tab đã chọn của bạn không được tập trung đúng cách sau khi khởi động lại Paperback.
* Cách xử lý các tệp trên ổ đĩa mạng Windows của Paperback: nhấn hiển thị tệp trong thư mục bây giờ đặt tiêu điểm chính xác vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn bị tải một cách bắt buộc trên khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một tệp.
* Mở thư mục chứa hiện tập trung tệp đã cho trong trình khám phá.
* Mở tệp readme hiện sẽ tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng của Paperback hiện sẽ được chia tỷ lệ đúng cách trên các hiển thị DPI cao.
* Menu hiện cập nhật đúng cách, và tiêu điểm di chuyển đến điều khiển văn bản, khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm việc sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm kích thước của các bảng chỉ số trên mỗi ký tự nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng các hộp thoại Thông tin tài liệu và Tất cả tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi được mở thông qua Shift+F1.
* Xóa tài liệu khỏi hộp thoại gần đây bây giờ sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được bảo tồn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Đi tới dòng, Đi tới trang và Đi tới phần trăm đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Tìm và Tìm tiếp theo không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Đánh dấu trang
* Âm thanh đánh dấu trang/ghi chú hiện tại phát bất độc quyền khi bạn điều hướng qua một từ chứa một.

##### Khả năng đọc
* Áp dụng xuống dòng từ bắn bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh bây giờ phải hiển thị đúng cách trong webview nhúng.

##### Trình cập nhật
* Trình cập nhật hiện hiển thị đúng nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả mạo.

##### Tài liệu RTF
* Phân tích các tài liệu RTF với các ký tự không phải Latin trong chúng.
* Nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo filepos trong sách Mobi phá vỡ các thẻ HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Cải thiện đáng kể phân tích AZW3.

##### Tài liệu Word
* Tài liệu Word có tên kiểu cụ thể theo ngôn ngữ không hiển thị tiêu đề đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay trở lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang sẽ không còn làm Paperback bị sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại, Word cũ và Word hiện đại và Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho tài liệu Microsoft Word cũ!
* Thêm hỗ trợ cho các bài thuyết trình Powerpoint cũ!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt ctrl+q để thoát ứng dụng.
* Thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản alt cho hình ảnh nhúng bây giờ phải được hiển thị đúng cách.
* Tài liệu CHM hiện hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Đã sửa go to page bị lệch 1.
* Đã sửa phím Escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu ngữ cảnh trình đọc không hiển thị trên nhấp chuột phải hoặc phím Ứng dụng.
* Đã sửa tài liệu sai đôi khi được tập trung khi mở tài liệu từ dòng lệnh.
* PDF chỉ có hình ảnh một lần nữa được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Bây giờ có thể điều hướng qua hình ảnh và hình vẽ với g/shift+g và f/shift+f tương ứng.
* Paperback hiện sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã xóa hỗ trợ DAISY XML, vì nó không còn cần thiết.
* Chuyển trở lại điều hướng chữ cái đầu tiên Win32 gốc trong chế độ xem cây mục lục.
* Hộp thoại tải lỗi hiện hiển thị các thông báo lỗi chi tiết hơn.
* Webview hiện sẽ mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi trong đó mở webview trong épubs chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi trong đó trình phân tích RTF sẽ không đặt khoảng trắng giữa các từ trong các trường hợp hiếm gặp.
* Các đoạn văn được tách thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Tab RTF và dòng mới hiện được hiển thị chính xác như chúng xuất hiện trong tài liệu.
* Chuyển trở lại thư viện pdfium đã thử và đúng để phân tích PDF, làm cho quá trình hiển thị PDF đáng tin cậy hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Thêm Ctrl+Shift+T để mở lại tài liệu được đóng cuối cùng.
* Hộp thoại Tất cả tài liệu hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích RTF.
* Đã sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng trắng không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa các nút Có/Không trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung đơn giản và tiếng Việt!
* Thêm trình cập nhật tự động sẽ thay thế phiên bản hiện được cài đặt của Paperback thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn để đạt đến dấu trang hoặc ghi chú, cảm ơn Andre Louis vì những âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho tài liệu DAISY XML.
* Thêm hỗ trợ cho các tệp Văn bản Tài liệu Mở phẳng!
* Thêm hỗ trợ cho các bài thuyết trình Tài liệu Mở phẳng!
* Thêm hỗ trợ cho bộ phân tách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Đã sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Web View.
* Đã sửa bảng không hiển thị đúng cách trong các tệp Markdown.
* Tệp PDF chỉ có hình ảnh hiện sẽ cảnh báo bạn về sự tồn tại khi bạn cố gắng tải một tệp.
* Nhúng đúng cách thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Mã mới an toàn hơn, tải tài liệu nhanh hơn và dễ duy trì cũng như mở rộng hơn.
* Menu ngữ cảnh của điều khiển văn bản sẽ đưa vào các hành động dành riêng cho trình đọc thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem bảng trong webview.
* Thêm tính năng hiển thị web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Xóa tất cả vào hộp thoại Tất cả tài liệu.
* Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi phiên bản mới có sẵn.
* Đã sửa khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch của các nút Có/Không trong các hộp thoại xác nhận.
* Đã sửa tải cấu hình khi chạy dưới dạng quản trị viên.
* Đã sửa xử lý bình luận trong tài liệu XML và HTML.
* Đã sửa phân tích TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Đã sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Đã sửa TOC epub đôi khi ném bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Đã sửa lỗi lệch 1 trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng ở cuối dòng của chúng.
* Đã sửa các vấn đề trình phân tích khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho các cộng tác viên.
* Nhiều tái cấu trúc nội bộ, di chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng đi đến vị trí trước đó/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ, và có thể điều hướng đến bằng các phím mũi tên alt+left/right.
* Thêm danh sách phần tử! Hiện tại, nó chỉ hiển thị cây của tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Đã sửa liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích TOC của Epub chứa các đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh spacebar để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Cải thiện xử lý các tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói ra nếu danh sách tài liệu gần đây trống khi bạn cố gắng hiển thị hộp thoại.

### Phiên bản 0.6.0
* Thêm tùy chọn để hiển thị menu đi ở dạng nhỏ gọn hơn nhiều vào hộp thoại tùy chọn, được chọn theo mặc định.
* Thêm tùy chọn để điều hướng theo các phần tử cấu trúc bao quanh.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại được tập trung.
* Thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Thêm hỗ trợ để phân tích sách điện tử FB2!
* Thêm hỗ trợ để phân tích các bài thuyết trình OpenDocument!
* Thêm hỗ trợ để phân tích các tệp Văn bản OpenDocument!
* Dấu trang hiện có thể được tạo để đánh dấu toàn bộ dòng hoặc để chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như pre-0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Dấu trang hiện có thể có ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú bằng N và Shift+N, hoặc bật lên hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ dấu trang không ghi chú được chọn bằng các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML hiện sẽ được xử lý đúng cách.
* Đã sửa tải các tài liệu Markdown lớn.
* Đã sửa nhấn spacebar trong chế độ xem cây mục lục kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa điều khiển văn bản không lấy lại tiêu điểm đôi khi khi quay lại cửa sổ Paperback.
* Đã sửa trường văn bản trong hộp thoại đi tới phần trăm không cập nhật giá trị của thanh trượt.
* Đã sửa kết xuất các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong các khối mã Markdown hiện sẽ được kết xuất đúng cách.
* Nếu tải sách bằng tham số dòng lệnh trong khi phiên bản Paperback hiện có sẵn đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới dạng quản trị viên, cấu hình hiện sẽ được tải và lưu đúng cách.
* Bây giờ có thể xóa dấu trang trực tiếp từ hộp thoại dấu trang.
* Bây giờ có thể nhập và xuất dấu trang và vị trí đọc cho một tài liệu cụ thể. Tệp được tạo ra được đặt tên theo tệp có phần mở rộng .paperback. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập thủ công chúng bằng mục trong menu công cụ.
* Liên kết bên trong tài liệu hiện được hỗ trợ hoàn toàn! Sử dụng k và shift+k để di chuyển về phía trước và lùi qua chúng, và nhấn enter để mở/kích hoạt một.
* Nhiều tái cấu trúc nội bộ, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để phù hợp với CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và mục của chúng hiện được hỗ trợ hoàn toàn! Sử dụng L và Shift+L để đi theo danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Xóa bàn phím hiện tại hoạt động để xóa tài liệu khỏi thanh tab ngoài xóa thường xuyên.
* Paperback hiện có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback trong khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được sinh ra.
* Paperback hiện hoàn toàn có thể dịch! Danh sách các ngôn ngữ mà nó hỗ trợ hiện khá nhỏ, nhưng nó đang phát triển liên tục!
* Paperback hiện có trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX hiện sẽ hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu mở hiện sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem tệp readme trong trình duyệt sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó sẽ hiển thị số lượng có thể tùy chỉnh, với phần còn lại của các tài liệu bạn từng mở có thể truy cập thông qua hộp thoại nhỏ.
* Nhiều cải tiến nhỏ cho các trình phân tích trên toàn bộ bảng, bao gồm đặt một dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý dòng mới bên trong đoạn văn trong tài liệu word và thêm các điểm đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ cho bài thuyết trình PowerPoint!
* Đã sửa một số mục menu không được vô hiệu hóa với không có tài liệu nào mở.
* Đã sửa hướng của thanh trượt đi tới phần trăm.
* Đã sửa mục lục trong sách Epub có đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng được cắt bớt từ các tiêu đề XHTML theo những cách lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục của riêng nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị điều đó cho bạn trong hộp thoại ctrl+t.
* Tài liệu HTML hiện sẽ có tiêu đề như được đặt trong thẻ tiêu đề, nếu nó tồn tại. Ngoài ra, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL trình đọc màn hình nào được gửi cùng với chương trình, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở nhiều sách epub hơn.
* Hộp thoại yêu cầu bạn nếu bạn muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được hoàn toàn làm lại, và hiện nó cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại đi tới phần trăm hiện bao gồm một trường văn bản cho phép bạn nhập thủ công phần trăm để nhảy đến.
* Trình phân tích HTML hiện sẽ nhận ra dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo tồn chính xác một lần nữa.
* Khoảng trắng không ngắt Unicode hiện được xem xét khi cắt bớt các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở một tệp không được nhận dạng mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Thêm biểu tượng menu bắt đầu tùy chọn vào trình cài đặt.
* Mục lục bây giờ sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có một mục con và cha có cùng văn bản ở cùng vị trí, bạn sẽ chỉ nhìn thấy mục cha.
* Đã sửa mục lục trong một số tài liệu CHM.
* Đã sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối trong đó.
* Tài liệu CHM hiện sẽ hiển thị tiêu đề của chúng khi được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang trong bao nhiêu tài liệu tùy thích. Bạn có thể nhảy tiến và lùi qua chúng với b và shift+b, đặt một cái với control+shift+b, và mang lên hộp thoại để nhảy đến dấu trang cụ thể với control+b.
* Thêm trình cài đặt bên cạnh tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập các liên kết tệp cho bạn.
* Tệp văn bản có BOM hiện sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Thêm nhiều thông tin hơn vào thanh trạng thái. Nó bây giờ sẽ hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Bình luận HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối đến Paperback trên dòng lệnh, nó hiện sẽ giải quyết nó đúng cách.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt riêng của nó, có thể truy cập bằng control+shift+g.
* Tài liệu mà không có tiêu đề hoặc tác giả đã biết hiện sẽ luôn có một mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ phải ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi bạn đóng Paperback hiện được ghi nhớ trên toàn bộ khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại đi tới dòng và đi tới trang hiện phải được làm sạch nghiêm ngặt hơn.
* Đã sửa mục lục điều hướng trong sách epub 3 có đường dẫn tương đối trong bản kê khai của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub có bản kê khai được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Đã sửa sử dụng CPU cao trong tài liệu có tiêu đề dài do suy thoái trong wxWidgets.
* Đã sửa tải tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa sự cố khi thoát ứng dụng trong một số trường hợp.
* Thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt xuống dòng từ!
* Bây giờ có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án nhà tài trợ ở dưới cùng của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown hiện sẽ luôn có tiêu đề, và Paperback bây giờ sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF hiện sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang cái được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều trên toàn bộ bảng.
* Bạn hiện chỉ có thể có một phiên bản Paperback chạy cùng một lúc. Chạy paperback.exe với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bây giờ bạn có thể nhấn xóa trên tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Thêm tổng số trang vào nhãn trang trong hộp thoại đi tới trang.
* Cho phép tab từ nội dung tài liệu đến danh sách tài liệu đã mở của bạn.
* Đã sửa các phím tiêu đề đôi khi mở các tài liệu gần đây nếu bạn có đủ số lượng.
* Paperback hiện sẽ xóa các gạch nối mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn trên ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa tải epubs có tên tệp được mã hóa URL trong bản kê khai của chúng.
* Đã sửa tải sách epub 3 có XHTML nhúng bên trong chúng.
* Thông báo hiện được nói ra nếu tài liệu không hỗ trợ mục lục hoặc phần, trái ngược với các mục menu được vô hiệu hóa.
* Thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu mở cuối cùng của bạn, và nhấn enter trên một sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử 25 tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây hiện được ghi nhớ trên toàn bộ khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Thêm shift+f1 để mở tệp readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
