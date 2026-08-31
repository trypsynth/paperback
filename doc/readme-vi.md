<!-- machine-translated from doc/readme.md (source-hash: efe922e94821c70e); please review and edit as needed -->

# Paperback - phiên bản 0.9.2

## Giới thiệu

Paperback là một ứng dụng đọc sách điện tử và tài liệu nhẹ, nhanh và có khả năng tiếp cận được cho mọi người, từ những người đọc bình thường đến những người dùng nâng cao. Nó được thiết kế để có khả năng tiếp cận screen reader, tốc độ nhanh và trải nghiệm không có phần thừa.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản hiện đại của macOS ARM. Các ứng dụng iOS và Android gốc đang được phát triển tích cực, với các bản dựng thử nghiệm công khai được lên kế hoạch sớm sau bản phát hành trên máy tính để bàn 0.9.0, trước bản phát hành thống nhất 1.0 bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu phần mềm nào được cài đặt trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở bao nhiêu tài liệu tùy thích cạnh nhau.
* Lưu lại vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đã mở khi đóng chương trình và khôi phục chúng khi khởi động lần tiếp theo.
* Bao gồm chức năng điều hướng tương tự như chức năng được tìm thấy trong chế độ duyệt web của nhiều screen reader để điều hướng nhanh chóng và dễ dàng qua các tài liệu.
* Bao gồm hộp thoại tìm kiếm mạnh mẽ, với các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn di động hoặc được cài đặt với liên kết tệp được thiết lập tự động.
* Hỗ trợ một loạt lớn các định dạng tệp phổ biến.

## Khả năng tương thích Screen Reader

Paperback hoạt động tốt với tất cả các screen reader chính. Tuy nhiên, có một vấn đề đã biết đối với người dùng JAWS.

### JAWS và Braille Displays

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi kéo sang trước bằng các phím điều hướng của màn hình của bạn. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải là điều gì trong Paperback, và điều mà mất khá nhiều thời gian để phát hiện ra một bản sửa lỗi vì sự nhiệt tình của Vispero trong việc đáp ứng các vấn đề với phần mềm mã nguồn mở.

Cách khắc phục, cuối cùng được phát hiện thông qua nhóm thảo luận JAWS sau những tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn văn hoạt động hiện tại thay vì tiếp tục. Với cả hai cài đặt này, kéo phải hoạt động chính xác.

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
* Bản trình bày OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bản trình bày PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng theo hướng bàn phím trước tiên. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Nơi macOS khác biệt, phím tương đương được ghi chú trong ngoặc đơn — chủ yếu vì Ctrl+G, Ctrl+W và Alt+Left/Right đã được sử dụng bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đóng cuối cùng.
* `Ctrl+R`: Hiển thị hộp thoại "All Documents" (từ Tài liệu gần đây).
* `Ctrl+Q`: Thoát (chỉ dành cho Windows; trên macOS điều này nằm trong menu ứng dụng).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Find.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi tới dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi tới phần trăm.
* `Ctrl+P`: Đi tới trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo tỷ lệ phần trăm đọc hiện tại của bạn.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến tới trong lịch sử điều hướng.
* `[`: Phần trước đó.
* `]`: Phần tiếp theo.
* `Shift+H`: Tiêu đề trước đó.
* `H`: Tiêu đề tiếp theo.
* `Shift+1` qua `Shift+6`: Tiêu đề trước đó ở mức 1-6.
* `1` qua `6`: Tiêu đề tiếp theo ở mức 1-6.
* `Shift+P`: Trang trước đó.
* `P`: Trang tiếp theo.
* `Shift+B`: Dấu trang trước đó.
* `B`: Dấu trang tiếp theo.
* `/`: Đặt dấu trang tạm thời của bạn.
* `\`: Nhảy tới dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước đó.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Nhảy tới tất cả các dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy tới chỉ dấu trang.
* `Ctrl+Alt+M`: Nhảy tới chỉ ghi chú.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
* `Shift+K`: Liên kết trước đó.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước đó.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình vẽ trước đó.
* `F`: Hình vẽ tiếp theo.
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
* `F7`: Hiển thị danh sách các phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong một tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần túy.
* `Ctrl+Shift+B`: Bật/tắt dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt ngắt dòng từ.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh phía sau.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, nằm trong menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím tài liệu-xem bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu đã chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết ở con trỏ, hoặc mở chế độ xem bảng khi ở trên dấu bảng.
* `Shift+F10` hoặc phím Menu/Application trong văn bản tài liệu: Mở menu ngữ cảnh.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, với ngày càng nhiều được thêm vào. Danh sách đầy đủ theo sau.

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

## Credits
### Development
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Donations
Các người sau đây đã quyên góp một số tiền cho phát triển Paperback. Nếu bạn quyên góp, tên của bạn sẽ không được tự động thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Ghi chú: Tôi coi người bảo trợ GitHub công khai là cơ sở để tự động đưa vào danh sách này.

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
* Sách nói không còn làm trình đọc màn hình đọc liên tiếp những khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách nói hiện đặt tên tập tin khi bạn đi qua từng phần.
* Sách nói hiện báo cáo độ dài thực tế của chúng, thay vì cho rằng mọi tập tin trong đó chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Select All hiện cho bạn toàn bộ tài liệu, thay vì chỉ phần hiện đang được tải.
* Find hiện đi thẳng đến dòng mà nó tìm thấy, thay vì để bạn ngồi nghe trình đọc màn hình đọc lại cửa sổ khi tiêu điểm quay lại sách.
* Đã sửa EPUB có chứa khối ZIP64 lạc từ chối mở với "Invalid local file header".
* Đã sửa các tài liệu dài quay lại điểm bắt đầu của chúng trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView hiện đưa bạn đến phần mà chúng chỉ đến, thay vì không thành công với "File not found".
* Thông báo tự động "Document reloaded" không còn cắt trình đọc màn hình của bạn giữa câu, thay vào đó chờ nó hoàn thành điều mà nó đang nói.
* Tab General của hộp thoại Settings hiện tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật trực tiếp sau tùy chọn kiểm tra cập nhật.
* Windows sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì tiêu đề đầy đủ của chương trình.
* Word Count và Document Info hiện hiển thị bao nhiêu tập tin một sách nói chứa và nó chạy trong tổng cộng bao lâu.

### Phiên bản 0.9.1
* Âm thanh Bookmark và note hiện được phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng trong im lặng.
* Đã sửa dấu ngoặc cong, em dash và các ký tự tương tự biến mất khỏi tài liệu RTF, kết hợp các từ xung quanh khi chúng đi.
* Đã sửa ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản bị hỏng.
* Đã sửa menu Tài liệu gần đây giữ các mục cũ cho đến khi điều gì đó khác xảy ra để xây dựng lại nó.
* Các phím tắt bàn phím đã quay trở lại trong mọi bản dịch, vì vậy menu của Nga có truy cập bàn phím lại.
* Các tài liệu CHM lớn hiện mở nhanh hơn đến bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Start.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback hiện nhớ vị trí, kích thước cửa sổ và trạng thái được phóng to giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm những thứ đọc đúng cách trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY hiện mở toàn bộ sách âm thanh thay vì chỉ văn bản của nó.
* Các tên hành động trong hộp thoại Customize Keyboard Shortcuts hiện có thể được dịch.
* Tiêu đề tài liệu hiện đứng đầu tiên trong thanh tiêu đề, vì vậy các sách mở có thể được phân biệt trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Thêm

##### Chung
* Một công cụ CLI, được gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng được hỗ trợ nào của Paperback thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Tùy chọn View Source để mở nguồn của tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, có nghĩa là bạn có thể tải sách có hàng chục triệu từ trong chỉ vài giây. Vui lòng báo cáo bất kỳ tính lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ Windows ARM64!
* Hỗ trợ macOS gốc!
* Chuyển đổi toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Nút định vị để định vị các sách còn thiếu vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả các tài liệu.

##### Tùy chọn và Khả năng đọc
* Tab khả năng đọc, với các tùy chọn sau:
    * Dòng từ (được di chuyển từ chung);
    * Kết xuất các bảng nội tuyến (mới trong phiên bản này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ cái;
    * Căn chỉnh văn bản.
* Mục menu dòng từ và phím nóng tiếp theo.
* Bật tắt để xác định cách bạn muốn hiển thị các bảng và thống nhất cách hiển thị các bảng trong các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng bằng vùng chứa.
* Tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong các trình đọc màn hình.
* Phím tắt bằng nhau để thông báo phần trăm hiện tại của bạn thông qua một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một dấu trang trên mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một dấu và dấu gạch chéo ngược để nhảy đến nó.

##### Đếm từ
* Thời gian đọc ước tính trong hộp thoại đếm từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại đếm từ, bao nhiêu từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua hộp thoại đơn giản.
* Phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, Tiếng Phần Lan và Tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Bộ cập nhật
* Nút hủy để cập nhật hộp thoại đang diễn ra.
* Bộ cập nhật hiện xác thực rằng tệp đã tải xuống chưa bị giả mạo.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách nói
* Khả năng phát các sách nói, hiện hỗ trợ cả DAISY audio (bao gồm DAISY audio + text) và zip của các tệp âm thanh.
* Phím tắt bàn phím và các mục menu để phát/tạm dừng lời kể, tìm kiếm về phía trước và phía sau, cũng như điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu ngoặc đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn xem tìm kiếm vượt quá cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình vẽ và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ các bảng.

#### Đã sửa

##### Chung
* Tài liệu được mã hóa bằng các bảng mã CJK kế thừa, chẳng hạn như GBK, Big5 và Shift_JIS, hiện sẽ được kết xuất đúng cách thay vì một loạt mojibake.
* "Reopen last closed" cố gắng mở lại readme được gói.
* Tab đã chọn của bạn không được lấy tiêu điểm đúng cách sau khi khởi động lại Paperback.
* Cách xử lý của Paperback đối với các tệp trên ổ đĩa mạng Windows: nhấn hiển thị tệp trong thư mục hiện phù hợp với tệp trên bộ lưu trữ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn được tải một cách bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một tệp.
* Mở thư mục chứa hiện tiêu điểm vào tệp đã cho trong trình khám phá.
* Mở readme sẽ hiện tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng của Paperback sẽ hiện tỷ lệ đúng cách trên các màn hình DPI cao.
* Menu hiện cập nhật đúng cách và tiêu điểm chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm kích thước của các bảng chỉ mục phần tử nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi được mở qua Shift+F1.
* Xóa tài liệu khỏi hộp thoại gần đây sẽ đóng luôn tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được bảo toàn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo sai văn bản dòng trong một số tình huống.
* Go to Line, Go to Page, và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh Bookmark/note hiện phải phát lại độc quyền khi bạn điều hướng qua một từ có chứa một.

##### Khả năng đọc
* Áp dụng dòng từ đưa bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh hiện phải hiển thị đúng cách trong webview nhúng.

##### Bộ cập nhật
* Bộ cập nhật hiện hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích các tài liệu RTF với các ký tự không phải tiếng Latin trong chúng.
* Các nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo Filepos trong sách Mobi chia HTML tags và đặt rác vào văn bản sách.
* Các liên kết trong sách Mobi kế thừa.
* Phân tích cú pháp AZW3 được cải thiện rất nhiều.

##### Tài liệu Word
* Tài liệu Word với các tên phong cách dành riêng cho địa phương không kết xuất các tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra các dấu ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay trở lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Các tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho các sách epub.
* Thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện tại Word cũ, Word hiện đại và Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho tài liệu Microsoft Word cũ!
* Thêm hỗ trợ cho các bản trình bày Powerpoint cũ!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt ctrl+q để thoát ứng dụng.
* Thêm hỗ trợ cho các sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho các hình ảnh nhúng hiện phải được hiển thị đúng cách.
* Tài liệu CHM hiện hỗ trợ điều hướng liên kết nội bộ một cách đúng đắn.
* Đã sửa go to page bị lệch 1.
* Đã sửa phím Escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Đã sửa tài liệu sai đôi khi được tập trung khi mở tài liệu từ dòng lệnh.
* Các tệp PDF chỉ có hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua các hình ảnh và hình vẽ bằng g/shift+g và f/shift+f tương ứng.
* Paperback hiện sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Xóa hỗ trợ DAISY XML vì nó không còn cần thiết.
* Chuyển ngược lại điều hướng chữ cái đầu tiên Win32 gốc trong chế độ xem cây của nội dung.
* Hộp thoại tải lỗi hiện hiển thị các thông báo lỗi chi tiết hơn.
* Webview sẽ hiện mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa một lỗi trong đó mở webview trong epubs chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa một lỗi trong đó trình phân tích RTF sẽ không đặt khoảng cách giữa các từ trong những trường hợp hiếm hoi.
* Đã sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Tab và dòng nguồn cấp dữ liệu RTF hiện được kết xuất chính xác khi chúng xuất hiện trong tài liệu.
* Chuyển ngược lại thư viện pdfium đã được thử và đúng để phân tích PDF, làm cho kết xuất PDF đáng tin cậy hơn nhiều lần.

### Phiên bản 0.8.1
* Thêm Ctrl+Shift+T để mở lại tài liệu đóng cuối cùng.
* Hộp thoại Tất cả tài liệu hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích RTF.
* Đã sửa các đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa của các nút Yes/No trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung Quốc đơn giản và tiếng Việt!
* Thêm bộ cập nhật tự động sẽ hiện thay thế phiên bản hiện tại đã cài đặt của Paperback thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn để đạt được dấu trang hoặc ghi chú, cảm ơn Andre Louis vì các âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho các tài liệu DAISY XML.
* Thêm hỗ trợ cho các tệp Văn bản Tài liệu Mở Phẳng!
* Thêm hỗ trợ cho các bài thuyết trình Tài liệu Mở Phẳng!
* Thêm hỗ trợ cho dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Đã sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được kết xuất trong Web View.
* Đã sửa các bảng không kết xuất đúng cách trong tệp Markdown.
* Các tệp PDF chỉ có hình ảnh sẽ hiện cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một tệp.
* Nhúng thông tin phiên bản trong tệp thực thi Paperback một cách đúng đắn.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ dàng hơn để duy trì và mở rộng.
* Menu ngữ cảnh của điều khiển văn bản sẽ hiện bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một cái trong webview.
* Thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại Tất cả tài liệu.
* Bộ kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi có phiên bản mới có sẵn.
* Đã sửa khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Yes/No trong hộp thoại xác nhận.
* Đã sửa tải cấu hình khi chạy dưới quyền quản trị viên.
* Đã sửa xử lý bình luận trong tài liệu XML và HTML.
* Đã sửa phân tích TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ cái trong nội dung.
* Đã sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng nút next/previous.
* Đã sửa TOC epub thỉnh thoảng ném bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Đã sửa lỗi lệch một trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng dòng ở cuối dòng của chúng.
* Đã sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào mở.
* Xử lý danh sách được cải thiện trong các định dạng tài liệu khác nhau.
* Quy trình dịch được cải thiện cho những người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng go to previous/next position rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được nhớ lại và có thể điều hướng với các phím mũi tên alt+left/right.
* Thêm danh sách phần tử! Hiện tại nó chỉ hiển thị cây tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ được phóng to theo mặc định.
* Đã sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích Epub TOC chứa các đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa các tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/cancel trong hộp thoại TOC.
* Xử lý các tiêu đề trong tài liệu Word được cải thiện.
* Bạn sẽ nhận được phản hồi được nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu go ở dạng nhỏ gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được kiểm tra theo mặc định.
* Thêm tùy chọn để điều hướng bằng các phần tử cấu trúc bao quanh.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa của tài liệu hiện được tập trung.
* Thêm hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng bộ định thời ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Thêm hỗ trợ để phân tích các sách FB2!
* Thêm hỗ trợ để phân tích các bài thuyết trình OpenDocument!
* Thêm hỗ trợ để phân tích các tệp Văn bản OpenDocument!
* Dấu trang hiện có thể được thực hiện để đánh dấu toàn bộ một dòng, hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như trước 0.6 và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ có văn bản đó sẽ được bao gồm trong dấu trang.
* Dấu trang hiện có thể có ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú với N và Shift+N, hoặc bật hộp thoại dấu trang với tất cả các dấu trang, chỉ ghi chú hoặc chỉ không ghi chú được chọn bằng các phím nóng cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML sẽ được xử lý đúng cách.
* Đã sửa tải tài liệu Markdown lớn.
* Đã sửa phím cách trong chế độ xem cây nội dung kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa điều khiển văn bản không regaining focus đôi khi khi quay trở lại cửa sổ của Paperback.
* Đã sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Đã sửa kết xuất các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown sẽ được kết xuất đúng cách.
* Nếu tải một cuốn sách có tham số dòng lệnh khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn nhận được lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình sẽ được tải và lưu đúng cách.
* Giờ đây có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Giờ đây có thể nhập và xuất dấu trang và vị trí đọc cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có phần mở rộng .paperback. Nếu tệp như vậy được tìm thấy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài cách khác, bạn có thể nhập chúng thủ công bằng mục trong menu công cụ.
* Các liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng k và shift+k để di chuyển về phía trước và phía sau qua chúng, và nhấn enter để mở/kích hoạt một liên kết.
* Nhiều tái cấu trúc nội bộ, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng bằng danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi bằng các danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Numpad xóa hiện hoạt động để xóa tài liệu khỏi thanh tab ngoài việc xóa bình thường.
* Paperback hiện có thể tối thiểu hóa thành khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback trong khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được sinh ra.
* Paperback hiện có thể dịch đầy đủ! Danh sách các ngôn ngữ mà nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang phát triển liên tục!
* Paperback hiện có một trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ hiện hiển thị một nội dung cơ bản, chứa tất cả các trang trình bày.
* Đường dẫn đầy đủ đến tài liệu đã mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị cho bạn 10 tài liệu cuối cùng mà bạn đã mở, nó sẽ hiển thị cho bạn một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn đã từng mở có thể truy cập thông qua hộp thoại nhỏ.
* Các cải tiến nhỏ khác nhau đối với các trình phân tích cú pháp trên bảng, bao gồm đặt một dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý newline bên trong đoạn trong tài liệu word và thêm điểm đầu vào các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ cho các bài thuyết trình PowerPoint!
* Đã sửa các mục menu nhất định không được vô hiệu hóa khi không có tài liệu nào mở.
* Đã sửa hướng của thanh trượt go to percent.
* Đã sửa nội dung trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng bị loại bỏ khỏi tiêu đề XHTML theo những cách lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng nội dung! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ xây dựng nội dung của riêng nó ra khỏi cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ cho bạn thấy điều đó trong hộp thoại ctrl+t.
* Tài liệu HTML sẽ hiện có tiêu đề như được đặt trong thẻ tiêu đề, nếu nó tồn tại. Ngoài cách khác, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL trình đọc màn hình nào được gửi cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại yêu cầu bạn có muốn mở tài liệu dưới dạng văn bản thuần túy đã được hoàn toàn làm lại và hiện cho phép bạn mở tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm trường văn bản cho phép bạn nhập thủ công phần trăm để nhảy đến.
* Trình phân tích HTML sẽ hiện nhận ra dd, dt và dl là các phần tử danh sách.
* Nội dung trong sách Epub sẽ được bảo toàn chính xác lần nữa.
* Không gian không phá vỡ unicode hiện được xem xét khi loại bỏ các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp không được công nhận mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Thêm biểu tượng menu Start tùy chọn vào trình cài đặt.
* Nội dung sẽ hiện sạch hơn trong một vài trường hợp, ví dụ nếu bạn có mục con và mục cha có cùng văn bản ở cùng vị trí bạn sẽ chỉ thấy mục cha.
* Đã sửa nội dung trong các tài liệu CHM nhất định.
* Đã sửa nội dung trong sách Epub 3 với các đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM hiện phải hiển thị tiêu đề của chúng như được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có nhiều dấu trang như bạn muốn trong bao nhiêu tài liệu. Bạn có thể nhảy về phía trước và phía sau qua chúng với b và shift+b, đặt một với control+shift+b, và mở hộp thoại để nhảy đến dấu trang cụ thể với control+b.
* Thêm trình cài đặt bên cạnh tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn, và tự động thiết lập các liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện phải được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản.
* Thêm thông tin nhiều hơn vào thanh trạng thái. Nó sẽ hiển thị cho bạn dòng hiện tại, ký tự và phần trăm đọc của bạn.
* Bình luận HTML, cũng như nội dung của các thẻ tập lệnh và kiểu, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ giải quyết nó đúng cách.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập bằng control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả được biết đến hiện luôn có tiêu đề mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ phải ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi bạn đóng Paperback hiện được nhớ lại khi khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page hiện phải được vệ sinh chặt chẽ hơn.
* Đã sửa điều hướng nội dung trong sách epub 3 với các đường dẫn tương đối trong bản kê của chúng.

### Phiên bản 0.3.0
* Đã sửa nội dung trong sách epub với các bản kê được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode đa byte.
* Đã sửa mức sử dụng CPU cao trong tài liệu có tiêu đề dài do hồi quy trong wxWidgets.
* Đã sửa tải tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa sự cố trên thoát ứng dụng trong những trường hợp nhất định.
* Thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt dòng từ!
* Hiện có thể quyên góp cho sự phát triển của Paperback, hoặc thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết sponsor project này ở dưới cùng trang chính của kho GitHub.
* Tài liệu Markdown sẽ luôn có tiêu đề, và Paperback hiện phải có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều trên bảng.
* Bạn chỉ có thể chạy một phiên bản Paperback cùng một lúc. Chạy paperback.exe với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bạn hiện có thể nhấn xóa vào một tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu sang danh sách các tài liệu đã mở của bạn.
* Đã sửa các phím tắt tiêu đề đôi khi mở tài liệu gần đây nếu bạn có đủ số lượng.
* Paperback sẽ xóa các dấu gạch ngang mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn trên ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm phím tắt để điều hướng bằng tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa tải epubs với tên tệp được mã hóa URL trong bản kê của chúng.
* Đã sửa tải sách epub 3 với XHTML nhúng bên trong chúng.
* Thông báo hiện được nói nếu tài liệu không hỗ trợ nội dung hoặc phần, ngược lại với các mục menu được vô hiệu hóa.
* Thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu cuối cùng của bạn được mở, và nhấn enter trên một sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Find, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử 25 tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây hiện được nhớ lại khi khởi động lại ứng dụng. Điều này có thể cấu hình được thông qua mục tùy chọn mới trong menu công cụ.
* Thêm shift+f1 để mở readme trực tiếp trong chính Paperback.

### Phiên bản 0.1.0
* Phiên bản ban đầu.
