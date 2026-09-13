<!-- machine-translated from doc/readme.md (source-hash: d583a89d8ac391f5; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,a028b7f6,af36028a,71df8e94,e9860ee8,93dd8dd6); please review and edit as needed -->

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

## Phím tắt

Paperback được thiết kế để sử dụng theo hướng keyboard-first. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Nơi macOS khác biệt, phím tương đương được ghi chú trong dấu ngoặc — chủ yếu vì `Ctrl+G`, `Ctrl+W`, và `Alt+Left`/`Alt+Right` đã được các quy ước hệ thống hoặc ứng dụng khác yêu cầu trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đóng gần đây nhất.
* `Ctrl+R`: Hiển thị hộp thoại "All Documents" (từ Recent Documents).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS điều này nằm trong menu ứng dụng).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Find.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi đến dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi đến phần trăm.
* `Ctrl+P`: Đi đến trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Công bố phần trăm đọc hiện tại và trang của bạn, ví dụ "15%, page 30". Trang bị bỏ qua đối với các tài liệu không có số trang.
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
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control thực tế chứ không phải Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
* `Shift+K`: Liên kết trước.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình trước.
* `F`: Hình tiếp theo.
* `Shift+T`: Bảng trước.
* `T`: Bảng tiếp theo.
* `Shift+S`: Bộ phân cách trước.
* `S`: Bộ phân cách tiếp theo.
* `Shift+L`: Danh sách trước.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi đến đầu bộ chứa hiện tại (danh sách hoặc bảng).
* `,`: Vượt qua cuối bộ chứa hiện tại (danh sách hoặc bảng).

### Menu Tools

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control thực tế chứ không phải Cmd): Hiển thị số từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem nguồn tài liệu trong một tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần.
* `Ctrl+Shift+B`: Chuyển đổi dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Chuyển đổi tự động xuống dòng.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh về phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh về phía sau.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Chuyển đổi toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, dưới menu ứng dụng).
* `Ctrl+Shift+S`: Chuyển đổi bộ hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra các bản cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím bổ sung trong chế độ xem tài liệu

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu đã chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết ở con trỏ, hoặc mở chế độ xem bảng khi ở trên đánh dấu bảng.
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
* Sách âm thanh không còn khiến trình đọc màn hình của bạn đọc to một loạt khoảng trắng khi bạn tập trung vào trường văn bản.
* Sách âm thanh hiện đặt tên cho tệp khi bạn bước qua chúng theo phần.
* Sách âm thanh hiện báo cáo thời lượng thực của chúng, thay vì tuyên bố mỗi tệp trong chúng chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị hộp thoại gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Chọn tất cả hiện cho bạn toàn bộ tài liệu, thay vì chỉ phần hiện đang được tải.
* Tìm kiếm bây giờ cắt thẳng đến dòng mà nó tìm thấy, thay vì khiến bạn phải nghe trình đọc màn hình đọc lại cửa sổ khi tiêu điểm trở lại cuốn sách.
* Đã sửa EPUB có khối ZIP64 lạc từ chối mở với "Invalid local file header".
* Đã sửa các tài liệu dài quay trở lại phần bắt đầu của chúng trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView hiện đưa bạn đến phần mà chúng chỉ, thay vì thất bại với "File not found".
* Phím tắt `=` hiện thông báo trang cũng như tỷ lệ phần trăm, ví dụ "15%, trang 30", và giữ nguyên với các tài liệu không có số trang.
* Thông báo tự động "Document reloaded" không còn cắt trình đọc màn hình giữa câu, thay vào đó chờ đợi nó hoàn thành những gì nó đang nói.
* Thẻ Tổng quát của hộp thoại Cài đặt hiện tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Cập nhật hiện đưa cửa sổ khởi chạy lại về phía trước, thay vì để nó phía sau mọi cửa sổ khác trong `Alt+Tab`.
* Windows hiện luôn hiển thị "Paperback" trong menu Mở với, thay vì khẩu hiệu đầy đủ của chương trình.
* Word Count và Document Info hiện hiển thị có bao nhiêu tệp trong sách âm thanh và nó chạy bao lâu tổng cộng.

### Phiên bản 0.9.1
* Âm thanh dấu trang và ghi chú hiện phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Đã sửa dấu ngoặc kép cong, dấu gạch dài và các ký tự tương tự biến mất khỏi tài liệu RTF, chạy các từ xung quanh chúng với nhau khi chúng đi.
* Đã sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản bị hỏng.
* Đã sửa trình đơn Tài liệu gần đây giữ các mục cũ cho đến khi một cái gì đó khác xảy ra để xây dựng lại nó.
* Bộ tăng tốc bàn phím quay trở lại trong mọi bản dịch, vì vậy menu của Nga có quyền truy cập bàn phím trở lại.
* Các tài liệu CHM lớn hiện mở nhanh gấp bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng hiển thị trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Tùy chọn đã được đổi tên thành Cài đặt, phù hợp với các ứng dụng di động và trên macOS, quy ước nền tảng.
* Paperback hiện nhớ vị trí cửa sổ, kích thước và trạng thái tối đa hóa của nó giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các tin nhắn đếm những thứ đọc đúng trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn tệp ncc.html của sách DAISY hiện mở sách âm thanh hoàn chỉnh thay vì chỉ có văn bản của nó.
* Tên hành động của hộp thoại Tùy chỉnh Phím tắt Bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu hiện xuất hiện trước tiên trên thanh tiêu đề, vì vậy các cuốn sách đã mở có thể được phân biệt trong thanh tác vụ và `Alt+Tab`.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Thêm

##### Tổng quát
* Một công cụ CLI, được gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được hỗ trợ bởi Paperback thành HTML, Markdown hoặc văn bản thuần túy.
* Tùy chọn tải lại tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Tùy chọn Xem nguồn để mở nguồn của tài liệu trong một thẻ mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, nghĩa là bạn có thể tải sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều kỳ lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Chuyển đổi toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Nút định vị để định vị sách bị mất chỉ thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và được chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Một thẻ khả năng đọc, với các tùy chọn sau:
    * Bao từ (được di chuyển từ chung);
    * Kết xuất bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ;
    * Căn chỉnh văn bản.
* Mục menu bao từ và phím tắt tiếp theo.
* Chuyển đổi để xác định cách bạn muốn bảng được hiển thị, và thống nhất cách bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo thùng chứa.
* Tùy chọn tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bàn phím bằng để thông báo tỷ lệ phần trăm hiện tại của bạn qua tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một cái trên mỗi tài liệu, và chúng tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Đếm từ
* Thời gian đọc ước tính trong hộp thoại đếm từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu một lựa chọn hoạt động khi bạn mở hộp thoại đếm từ, có bao nhiêu từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mỗi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Phím tắt bàn phím có thể định cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, Phần Lan và Ba Lan.

##### Xuất
* Mở rộng mục xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Bộ cập nhật
* Nút hủy vào hộp thoại cập nhật đang diễn ra.
* Bộ cập nhật hiện xác thực tệp được tải xuống chưa bị can thiệp.

##### Web View
* Web view hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát âm thanh DAISY 2.02.

##### Sách âm thanh
* Khả năng phát sách âm thanh, hiện hỗ trợ cả DAISY âm thanh (bao gồm DAISY âm thanh + văn bản) và zip của các tệp âm thanh.
* Phím tắt và mục menu để phát/tạm dừng lời tường thuật, tìm kiếm phía trước và phía sau, và điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu ngoặc kép đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn xem tìm kiếm vượt quá cuối chương có tiếp tục vào phần tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình vẽ và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Đã sửa

##### Tổng quát
* Tài liệu được mã hóa bằng các mã hóa CJK kế thừa, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị đúng thay vì là một bó mojibake.
* "Mở lại tài liệu đóng cuối cùng" cố gắng mở lại readme được gói.
* Thẻ đã chọn của bạn không được tập trung đúng cách sau khi khởi động lại Paperback.
* Cách xử lý tệp trên ổ đĩa mạng Windows của Paperback: nhấn hiển thị tệp trong thư mục hiện tạo tập trung đúng cách vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn được tải một cách cưỡng bức khi khôi phục tài liệu; thay vào đó, bạn sẽ được hỏi xác nhận khi tìm thấy một tệp.
* Mở thư mục chứa hiện tạo tập trung vào tệp đã cho trong trình khám phá.
* Mở readme sẽ tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng Paperback sẽ hiệu chuẩn đúng cách trên màn hình độ phân giải cao.
* Menu sẽ cập nhật đúng cách, và tiêu điểm chuyển đến điều khiển văn bản, khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển đổi giữa các thẻ.
* Giảm sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục theo ký tự nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi được mở qua `Shift+F1`.
* Loại bỏ tài liệu khỏi hộp thoại gần đây hiện cũng sẽ đóng thẻ hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được bảo tồn sau khi loại bỏ tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page, Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Tìm kiếm và Tìm kiếm tiếp theo không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú bây giờ sẽ phát đúng cách độc quyền khi bạn điều hướng trên một từ chứa một.

##### Khả năng đọc
* Áp dụng bao từ đưa bạn đến phần bắt đầu của tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên với kích thước ban đầu rất nhỏ.
* Hình ảnh hiện sẽ hiển thị đúng cách trong webview được nhúng.

##### Bộ cập nhật
* Bộ cập nhật hiện hiển thị đúng nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích các tài liệu RTF có các ký tự không phải Latin trong chúng.
* RTF `\pict` nhóm để dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các liên kết filepos trong sách Mobi tách các thẻ HTML và đặt rác vào văn bản sách.
* Các liên kết trong sách Mobi kế thừa.
* Phân tích AZW3 được cải thiện rất nhiều.

##### Tài liệu Word
* Tài liệu Word có tên kiểu cụ thể theo ngôn ngữ không hiển thị tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt, và dd không tạo ra ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay trở lại trích xuất văn bản thuần túy cho các PDF được gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện tại, Word kế thừa, Word hiện đại và Powerpoint hiện đại được hỗ trợ, với Powerpoint kế thừa được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho tài liệu Microsoft Word kế thừa!
* Thêm hỗ trợ cho bản trình bày Powerpoint kế thừa!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho tệp PDF được gắn thẻ!
* Thêm phím tắt `ctrl+q` để thoát ứng dụng.
* Thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh được nhúng hiện sẽ được hiển thị đúng cách.
* Tài liệu CHM hiện hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Đã sửa go to page bị tắt 1.
* Đã sửa phím Escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Đã sửa tài liệu sai đôi khi được tập trung khi mở tài liệu từ dòng lệnh.
* PDF chỉ có hình ảnh được phát hiện lại và cảnh báo bạn về sự tồn tại của chúng.
* Hiện có thể điều hướng qua hình ảnh và hình vẽ với `g`/`shift+g` và `f`/`shift+f` tương ứng.
* Paperback sẽ hiện tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Loại bỏ hỗ trợ DAISY XML, vì nó không còn cần thiết.
* Chuyển ngược lại điều hướng chữ cái đầu tiên Win32 gốc trong cây mục lục.
* Hộp thoại lỗi tải hiện hiển thị các thông báo lỗi chi tiết hơn.
* Webview bây giờ sẽ mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi trong đó mở webview trong epub chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi trong đó trình phân tích RTF sẽ không đặt khoảng trắng giữa các từ trong trường hợp hiếm.
* Các đoạn được tách thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab RTF và nguồn cấp dữ liệu dòng hiện được kết xuất chính xác như chúng xuất hiện trong tài liệu.
* Chuyển ngược lại thư viện pdfium được thử và kiểm chứng để phân tích PDF, giúp kết xuất PDF đáng tin cậy hơn nhiều lần.

### Phiên bản 0.8.1
* Thêm `Ctrl+Shift+T` để mở lại tài liệu đóng cuối cùng.
* Hộp thoại Tất cả tài liệu hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích RTF.
* Đã sửa các đường dẫn tệp chứa ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp thông qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai, và khoảng cách không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa các nút Có/Không trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Thêm bộ cập nhật tự động sẽ hiện thay thế phiên bản Paperback hiện được cài đặt của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn để đạt đến dấu trang hoặc ghi chú, cảm ơn Andre Louis vì các âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho tài liệu DAISY XML.
* Thêm hỗ trợ cho tệp Văn bản Tài liệu Mở được phẳng!
* Thêm hỗ trợ cho bản trình bày Tài liệu Mở được phẳng!
* Thêm hỗ trợ cho dấu phân cách với `s` và `shift+s`.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Đã sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được kết xuất trong Web View.
* Đã sửa bảng không kết xuất đúng cách trong tệp Markdown.
* PDF chỉ có hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một hình ảnh.
* Nhúng đúng cách thông tin phiên bản trong tệp thực thi Paperback.
* Tách hộp thoại tùy chọn thành các thẻ để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy cao hơn, tốc độ nhanh hơn và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn, và dễ dàng hơn để duy trì và mở rộng.
* Menu ngữ cảnh của điều khiển văn bản sẽ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng `T` và `Shift+T`, và nhấn Enter để xem một trong webview.
* Thêm tính năng kết xuất web cơ bản! Nhấn `Ctrl+Shift+V` để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi phiên bản mới có sẵn.
* Đã sửa khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Có/Không trong hộp thoại xác nhận.
* Đã sửa tải cấu hình khi chạy với quyền quản trị viên.
* Đã sửa xử lý nhận xét trong tài liệu XML và HTML.
* Đã sửa phân tích TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Đã sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước.
* Đã sửa epub TOC đôi khi ném bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng khác nhau trong thẻ XML, HTML và pre.
* Đã sửa lỗi off-by-one trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng ở cuối dòng.
* Đã sửa các vấn đề trình phân tích khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện bị vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Xử lý danh sách được cải thiện trong các định dạng tài liệu khác nhau.
* Quy trình dịch được cải thiện cho những người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ hiện tại, và có thể được điều hướng với các phím `alt+left`/`right`.
* Thêm danh sách phần tử! Hiện tại, nó chỉ hiển thị cây của tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Đã sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích Epub TOC chứa các đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Xử lý tiêu đề trong tài liệu Word được cải thiện.
* Bạn sẽ nhận được phản hồi nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng đưa lên hộp thoại.

### Phiên bản 0.6.0
* Tùy chọn mới để hiển thị menu go ở dạng nhỏ gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được chọn theo mặc định.
* Thêm tùy chọn để điều hướng bằng các phần tử cấu trúc được bao bọc.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện được tập trung.
* Thêm hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập với `Ctrl+Shift+S`.
* Thêm hỗ trợ phân tích sách điện tử FB2!
* Thêm hỗ trợ phân tích bản trình bày OpenDocument!
* Thêm hỗ trợ phân tích tệp Văn bản OpenDocument!
* Dấu trang bây giờ có thể được dùng để đánh dấu toàn bộ dòng, hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi sẽ giống như trước 0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được bao gồm trong dấu trang.
* Dấu trang bây giờ có thể có các ghi chú văn bản tùy chọn được đính kèm! Điều hướng giữa các dấu trang chứa ghi chú với `N` và `Shift+N`, hoặc bật hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ không có ghi chú được chọn với các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML sẽ được xử lý đúng cách.
* Đã sửa tải các tài liệu Markdown lớn.
* Đã sửa phím cách trong cây xem mục lục kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở phần đầu của các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa điều khiển văn bản không lấy lại tiêu điểm đôi khi khi trở lại cửa sổ Paperback.
* Đã sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Đã sửa kết xuất ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown sẽ được kết xuất đúng cách.
* Nếu tải một cuốn sách có tham số dòng lệnh trong khi phiên bản Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback với quyền quản trị viên, cấu hình sẽ được tải và lưu đúng cách.
* Hiện có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Hiện có thể nhập và xuất dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có tiện ích mở rộng .paperback. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng theo cách thủ công bằng cách sử dụng mục trong menu công cụ.
* Liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng `k` và `shift+k` để di chuyển về phía trước và về phía sau qua chúng, và nhấn enter để mở/kích hoạt một.
* Nhiều tái cấu trúc nội bộ, giúp ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng `L` và `Shift+L` để đi theo danh sách, và `I` và `Shift+I` để đi qua các mục danh sách.
* Phím xóa Numpad hiện hoạt động để loại bỏ tài liệu khỏi thanh tab ngoài việc xóa thông thường.
* Paperback hiện có thể tối thiểu hóa sang khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ khiến tùy chọn tối thiểu hóa trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback hiện có thể dịch đầy đủ! Danh sách các ngôn ngữ nó hỗ trợ hiện tại khá nhỏ, nhưng nó đang phát triển không ngừng!
* Paperback hiện có một trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu được mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó sẽ hiển thị cho bạn một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn đã mở khi nào có thể truy cập thông qua một hộp thoại nhỏ.
* Nhiều cải tiến nhỏ cho các trình phân tích trên toàn bộ bảng, bao gồm đặt dòng trống giữa các slide trong bản trình bày PPTX, sửa xử lý dòng mới bên trong các đoạn văn trong tài liệu word và thêm các dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ bản trình bày PowerPoint!
* Đã sửa một số mục menu nhất định không bị vô hiệu hóa khi không có tài liệu nào được mở.
* Đã sửa hướng của thanh trượt go to percent.
* Đã sửa mục lục trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng bị tước khỏi tiêu đề XHTML theo những cách lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục riêng của nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị điều đó cho bạn trong hộp thoại `ctrl+t`.
* Tài liệu HTML sẽ có tiêu đề được đặt trong thẻ tiêu đề, nếu nó tồn tại. Nếu không, họ sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL trình đọc màn hình được gửi cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại yêu cầu bạn nếu bạn muốn mở tài liệu của mình dưới dạng văn bản thuần túy đã được viết lại hoàn toàn, và nó hiện cho phép bạn mở tài liệu của mình dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm trường văn bản cho phép bạn nhập phần trăm một cách thủ công để nhảy tới.
* Trình phân tích HTML sẽ nhận ra dd, dt, và dl làm các phần tử danh sách.
* Mục lục trong sách Epub sẽ được bảo tồn chính xác một lần nữa.
* Không gian không phân tách unicode hiện được coi khi tước các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp không được công nhận mỗi lần tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Thêm biểu tượng menu Bắt đầu tùy chọn vào trình cài đặt.
* Mục lục sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có mục con và cha có cùng văn bản ở cùng vị trí bạn sẽ chỉ thấy mục cha.
* Đã sửa mục lục trong một số tài liệu CHM nhất định.
* Đã sửa mục lục trong sách Epub 3 với đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang tùy thích trong bao nhiêu tài liệu tùy thích. Bạn có thể nhảy về phía trước và phía sau qua chúng với `b` và `shift+b`, đặt một cái bằng `control+shift+b`, và đưa lên hộp thoại để nhảy đến dấu trang cụ thể bằng `control+b`.
* Thêm trình cài đặt bên cạnh tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn, và tự động thiết lập các liên kết tệp cho bạn.
* Tệp văn bản có BOM hiện sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở phần đầu của văn bản.
* Thêm thông tin nhiều hơn đáng kể vào thanh trạng thái. Nó sẽ cho bạn biết dòng hiện tại, ký tự và tỷ lệ phần trăm đọc của bạn.
* Nhận xét HTML, cũng như nội dung của các thẻ tập lệnh và kiểu, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ giải quyết nó đúng cách.
* Chuyển động theo tỷ lệ phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt riêng của nó, có thể truy cập với `control+shift+g`.
* Tài liệu không có tiêu đề hoặc tác giả được biết đến hiện luôn có mặc định.
* Luôn logic lưu vị trí hiện thông minh hơn và chỉ nên ghi vào đĩa khi tuyệt đối cần thiết.
* Tài liệu bạn đã tập trung khi đóng Paperback hiện được ghi nhớ trên toàn bộ khởi động lại ứng dụng.
* Nhập vào các hộp thoại go to line và go to page hiện sẽ được vệ sinh nghiêm ngặt hơn.
* Đã sửa điều hướng mục lục trong sách epub 3 với đường dẫn tương đối trong kê khai của chúng.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub với kê khai được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Đã sửa sử dụng CPU cao trong tài liệu có tiêu đề dài do một hồi quy trong wxWidgets.
* Đã sửa tải tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa sự cố khi thoát ứng dụng trong một số trường hợp.
* Thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt bao từ!
* Hiện có thể quyên góp cho sự phát triển Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc liên kết tài trợ dự án ở phía dưới của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown sẽ luôn có tiêu đề, và Paperback hiện sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị mất.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích PDF đáng tin cậy hơn nhiều trên toàn bộ bảng.
* Bạn chỉ có thể chạy một phiên bản Paperback tại một thời điểm. Chạy paperback.exe với tên tệp trong khi nó đang chạy sẽ mở tài liệu đó trong phiên bản đã chạy.
* Bạn hiện có thể nhấn xóa trên tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu sang danh sách các tài liệu đã mở của bạn.
* Đã sửa các phím điều hướng tiêu đề đôi khi mở tài liệu gần đây nếu bạn có đủ số lượng của chúng.
* Paperback hiện sẽ loại bỏ gạch dưới mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đặt bạn trên ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm phím bấm để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím bấm này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa tải epubs với tên tệp được mã hóa URL trong kê khai của chúng.
* Đã sửa tải sách epub 3 với XHTML được nhúng bên trong chúng.
* Thông báo hiện được nói nếu tài liệu không hỗ trợ mục lục hoặc phần, đối với các mục menu bị vô hiệu hóa.
* Thêm menu tài liệu gần đây! Hiện nó lưu 10 tài liệu được mở cuối cùng của bạn, và nhấn enter trên tài liệu sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm, làm cho nó đơn giản hơn để sử dụng, đồng thời thêm lịch sử 25 tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu đã mở trước đây hiện được ghi nhớ trên khởi động lại ứng dụng. Điều này có thể định cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Thêm `shift+f1` để mở readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
