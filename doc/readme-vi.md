<!-- machine-translated from doc/readme.md (source-hash: df18cffffe239932); please review and edit as needed -->

# Paperback - phiên bản 0.9.1

## Giới thiệu

Paperback là một trình đọc sách điện tử và tài liệu nhẹ, nhanh và dễ tiếp cận cho mọi người, từ những người đọc thường xuyên đến những người dùng năng suất cao. Nó được thiết kế với sự hỗ trợ screen reader, tốc độ nhanh và trải nghiệm không có tính năng không cần thiết.

## Yêu cầu Hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản hiện đại của ARM macOS. Các ứng dụng iOS và Android gốc đang được phát triển tích cực, với các bản dựng thử nghiệm công khai dự kiến sẽ được phát hành không lâu sau bản phát hành desktop 0.9.0, trước bản phát hành thống nhất 1.0 bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu cài đặt bất kỳ phần mềm nào trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở nhiều tài liệu cùng lúc.
* Lưu lại vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đã mở khi đóng chương trình và khôi phục chúng khi khởi chạy lần tiếp theo.
* Bao gồm chức năng điều hướng tương tự như chế độ duyệt web được tìm thấy trong nhiều screen reader để nhanh chóng và dễ dàng điều hướng qua các tài liệu.
* Bao gồm hộp thoại tìm kiếm mạnh mẽ, bao gồm các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn di động, hoặc được cài đặt với các liên kết tệp được thiết lập tự động.
* Hỗ trợ một mảng khổng lồ các định dạng tệp phổ biến.

## Khả năng tương thích Screen Reader

Paperback hoạt động tốt với tất cả các screen reader chính. Tuy nhiên, có một vấn đề được biết đến cho người dùng JAWS.

### JAWS và Màn hình Braille

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi panning tiến với các phím điều hướng của màn hình. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách xử lý điều khiển văn bản RICHEDIT50W của JAWS, không phải là điều gì đó trong chính Paperback, và đó là điều mất khá lâu để tìm ra bản sửa lỗi cho nó với mức độ nhiệt tình của Vispero trong việc phản hồi các vấn đề với phần mềm mã nguồn mở.

Cách khắc phục, cuối cùng được phát hiện thông qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ ở lại đoạn hoạt động thay vì tiến lên. Với cả hai cài đặt được thực hiện, panning sẽ hoạt động chính xác.

## Các loại tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và tiện ích mở rộng sau:

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
* Tệp văn bản thô và nhật ký (`.txt`, `.log`)

## Phím tắt

Paperback được thiết kế để sử dụng theo hướng tiên dụng bàn phím. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Nếu macOS khác, phím tương đương được ghi chú trong dấu ngoặc đơn — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã được sử dụng bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu Tệp

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu vừa đóng.
* `Ctrl+R`: Hiển thị hộp thoại "Tất cả tài liệu" (từ Tài liệu gần đây).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS điều này nằm dưới menu ứng dụng thay vào đó).

### Menu Đi tới

* `Ctrl+F`: Hiển thị hộp thoại Tìm kiếm.
* `F3` (macOS: `Cmd+G`): Tìm kiếm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm kiếm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi tới dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi tới phần trăm.
* `Ctrl+P`: Đi tới trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo phần trăm đọc hiện tại của bạn.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến lên trong lịch sử điều hướng.
* `[`: Phần trước.
* `]`: Phần tiếp theo.
* `Shift+H`: Tiêu đề trước.
* `H`: Tiêu đề tiếp theo.
* `Shift+1` đến `Shift+6`: Tiêu đề trước ở cấp 1-6.
* `1` đến `6`: Tiêu đề tiếp theo ở cấp 1-6.
* `Shift+P`: Trang trước.
* `P`: Trang tiếp theo.
* `Shift+B`: Dấu trang trước.
* `B`: Dấu trang tiếp theo.
* `/`: Đặt dấu trang tạm thời của bạn.
* `\`: Nhảy tới dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Nhảy tới tất cả dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy tới dấu trang chỉ.
* `Ctrl+Alt+M`: Nhảy tới ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
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
* `Shift+,`: Đi tới đầu vùng chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi vượt qua cuối vùng chứa hiện tại (danh sách hoặc bảng).

### Menu Công cụ

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý thay vì Cmd): Hiển thị số từ cho tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Chế độ xem Web.
* `Ctrl+U`: Xem nguồn tài liệu trong tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại dưới dạng văn bản thuần túy.
* `Ctrl+Shift+B`: Bật/tắt dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt ngắt dòng chữ.
* `Ctrl+Space`: Phát/tạm dừng tường thuật âm thanh.
* `'`: Tua nhanh tường thuật âm thanh.
* `;`: Tua lại tường thuật âm thanh.
* `Ctrl+'`: Tăng lượng tua âm thanh.
* `Ctrl+;`: Giảm lượng tua âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Tùy chỉnh, dưới menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.

### Menu Trợ giúp

* `Ctrl+F1`: Hiển thị hộp thoại Giới thiệu.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím xem tài liệu bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu đã chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết ở con trỏ, hoặc mở chế độ xem bảng khi ở trên dấu bảng.
* `Shift+F10` hoặc phím Menu/Ứng dụng trong văn bản tài liệu: Mở menu bối cảnh.

## Ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, và luôn có thêm ngôn ngữ được bổ sung. Danh sách đầy đủ như sau.

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

## Ghi công
### Phát triển
* Quin Gillespie: nhà phát triển chính và nhà sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Quyên góp
Các người sau đây đã quyên góp một số tiền cho việc phát triển Paperback. Nếu bạn quyên góp, tên của bạn sẽ không tự động được thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi bất kỳ nhà tài trợ GitHub công khai nào là cơ sở để tự động đưa vào danh sách này.

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
* Sách âm thanh không còn làm trình đọc màn hình đọc một loạt khoảng trắng khi bạn focus trường văn bản.
* Sách âm thanh giờ đây đặt tên tệp khi bạn bước qua từng phần.
* Sách âm thanh giờ đây báo cáo độ dài thực tế của chúng, thay vì tuyên bố mọi tệp trong đó chạy trong 24 giờ.
* Đóng Web View bằng Escape không còn đưa ra cảnh báo gỡ lỗi sau khi bạn đã theo một liên kết bên trong nó.
* Sao chép sau Select All giờ đây cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần hiện đang được tải.
* Find giờ đây đi thẳng đến dòng mà nó tìm thấy, thay vì yêu cầu bạn chịu đựng trình đọc màn hình đọc lại cửa sổ khi focus quay lại sách.
* Đã sửa các EPUB có khối ZIP64 lạc lẫm từ chối mở bằng "Invalid local file header".
* Đã sửa các tài liệu dài quay lại điểm bắt đầu của chúng trong khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong Web View giờ đây đưa bạn đến phần mà chúng trỏ đến, thay vì thất bại với "File not found".
* Thông báo tự động "Document reloaded" không còn cắt trình đọc màn hình của bạn giữa câu, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab General của hộp thoại Settings giờ đây tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật ngay sau tùy chọn kiểm tra cập nhật.
* Windows sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì thẻ đầu đủ của chương trình.
* Word Count và Document Info giờ đây hiển thị có bao nhiêu tệp mà một sách âm thanh chứa, và nó chạy bao lâu tổng cộng.

### Phiên bản 0.9.1
* Âm thanh dấu trang và ghi chú giờ đây phát trên macOS.
* Sách DAISY giờ đây phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng trong im lặng.
* Đã sửa dấu ngoặc kép cong, dấu gạch ngang dài và các ký tự tương tự biến mất từ các tài liệu RTF, chạy các từ xung quanh chúng với nhau khi chúng đi.
* Đã sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản lộn xộn.
* Đã sửa trình đơn Recent Documents giữ lại các mục cũ cho đến khi có gì khác xảy ra để xây dựng lại nó.
* Các phím tắt bàn phím hiện đã quay lại trong mỗi bản dịch, vì vậy menu tiếng Nga có quyền truy cập bàn phím lại.
* Các tài liệu CHM lớn giờ đây mở nhanh hơn gấp bảy lần.
* Các tài liệu được mở giờ đây được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Start.
* Options đã được đổi tên thành Settings, phù hợp với các ứng dụng di động và trên macOS, quy ước nền tảng.
* Paperback giờ đây ghi nhớ vị trí cửa sổ, kích thước và trạng thái tối đa hóa của nó giữa các lần chạy.
* Các dạng số nhiều giờ đây được dịch, vì vậy các thông báo đếm những thứ đọc đúng cách trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY giờ đây mở sách âm thanh hoàn chỉnh thay vì chỉ văn bản của nó.
* Tên hành động của hộp thoại Customize Keyboard Shortcuts giờ đây có thể được dịch.
* Tiêu đề tài liệu giờ đây xuất hiện trước tiên trong thanh tiêu đề, vì vậy các sách mở có thể được phân biệt trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật giờ đây đã được dịch.

### Phiên bản 0.9.0

#### Thêm

##### Chung
* Một công cụ CLI gọi là pb để nhanh chóng chuyển đổi bất kỳ định dạng được hỗ trợ nào của Paperback sang HTML, Markdown hoặc văn bản thuần.
* Một tùy chọn để tải lại các tài liệu đã bị các chương trình khác trên đĩa sửa đổi.
* Một tùy chọn View Source để mở nguồn của tài liệu trong một tab mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu giờ đây được phân trang, có nghĩa là bạn có thể tải các sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều kỳ lạ nào được tìm thấy với cái này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS asli!
* Một toggle toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Một nút định vị để định vị các sách bị thiếu vừa thay đổi đường dẫn của chúng.
* Một bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Bao quanh từ (được chuyển từ chung);
    * Hiển thị bảng nội tuyến (mới trong phiên bản này, xem bên dưới);
    * Font chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ;
    * Căn chỉnh văn bản.
* Một mục menu bao quanh từ và phím nóng tiếp theo.
* Một toggle để xác định cách bạn muốn bảng được hiển thị, và hợp nhất cách bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo vùng chứa.
* Một tùy chọn để tự động di chuyển con trỏ đến điểm bắt đầu của dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng để thông báo phần trăm hiện tại của bạn qua một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một cho mỗi tài liệu, và chúng vẫn còn tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy tới nó.

##### Số lượng từ
* Thời gian đọc ước tính trong hộp thoại số lượng từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu lựa chọn đang hoạt động khi bạn mở hộp thoại số lượng từ, có bao nhiêu từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Hà Lan, Phần Lan và Ba Lan.

##### Xuất
* Mở rộng mục xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần.

##### Trình cập nhật
* Một nút hủy cho hộp thoại cập nhật đang diễn ra.
* Trình cập nhật giờ đây xác thực tệp đã tải xuống chưa bị giả mạo.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách âm thanh
* Khả năng phát lại sách âm thanh, hiện hỗ trợ cả DAISY âm thanh (bao gồm DAISY âm thanh + văn bản) và zip các tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời kể, tìm kiếm về phía trước và phía sau, và điều chỉnh mức tìm kiếm.
* Các tùy chọn để đồng bộ hóa dấu đọc với phát lại âm thanh, đặt mức tìm kiếm âm thanh và chọn liệu tìm kiếm vượt quá cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, các mục danh sách, hình vẽ và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint giờ đây hỗ trợ bảng.

#### Đã sửa

##### Chung
* Các tài liệu được mã hóa trong mã hóa CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, giờ đây sẽ hiển thị đúng thay vì một loạt mojibake.
* "Reopen last closed" cố gắng mở lại readme được đóng gói.
* Tab được chọn của bạn không được lấy focus đúng cách sau khi khởi động lại Paperback.
* Xử lý các tệp trên ổ đĩa mạng Windows của Paperback: nhấn show file in folder giờ đây lấy focus đúng cách tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn bị tải bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một.
* Open containing folder giờ đây lấy focus tệp đã cho trong explorer.
* Mở readme giờ đây sẽ tôn trọng ngôn ngữ được chọn của bạn.
* Giao diện người dùng Paperback giờ đây sẽ được chia tỷ lệ đúng cách trên màn hình DPI cao.
* Menu giờ đây cập nhật đúng cách, và focus di chuyển đến kiểm soát văn bản, khi mở trợ giúp trong Paperback.
* Chuyển sang một phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục trên mỗi ký tự nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi được mở qua Shift+F1.
* Xóa các tài liệu từ hộp thoại recents giờ đây cũng sẽ đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ đây được bảo toàn sau khi xóa một tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú giờ đây sẽ phát đúng cách độc quyền khi bạn điều hướng qua một từ chứa một.

##### Khả năng đọc
* Áp dụng bao quanh từ bắn bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh giờ đây sẽ hiển thị đúng cách trong webview được nhúng.

##### Trình cập nhật
* Trình cập nhật giờ đây hiển thị đúng cách nội dung của các thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích cú pháp các tài liệu RTF có chứa các ký tự không phải Latin.
* Nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo Filepos trong các sách Mobi chia tách các thẻ HTML và đưa rác vào văn bản sách.
* Liên kết trong sách Mobi cũ.
* Phân tích cú pháp AZW3 được cải thiện rất nhiều.

##### Tài liệu Word
* Tài liệu Word với tên kiểu cụ thể locale không hiển thị các tiêu đề đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ra ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ đây quay lại trích xuất văn bản thuần cho các PDF bị gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện tại Word cũ, Word hiện đại và Powerpoint hiện đại được hỗ trợ, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho các tài liệu Microsoft Word cũ (*.doc)!
* Thêm hỗ trợ cho các bản trình bày Powerpoint cũ (*.ppt)!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt ctrl+q để thoát ứng dụng.
* Thêm hỗ trợ cho các sách nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng giờ đây sẽ được hiển thị đúng cách.
* Các tài liệu CHM giờ đây hỗ trợ đúng cách điều hướng liên kết nội bộ.
* Đã sửa các âm thanh dấu trang kích hoạt tại điểm bắt đầu đoạn thay vì vị trí của dấu trang.
* Đã sửa go to page bị sai 1.
* Đã sửa phím escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa menu ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Applications.
* Đã sửa tài liệu sai đôi khi được lấy focus khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ chứa hình ảnh một lần nữa được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đây có thể điều hướng qua hình ảnh và hình vẽ với g/shift+g và f/shift+f, tương ứng.
* Paperback giờ đây sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã xóa hỗ trợ DAISY XML, vì nó không còn cần thiết nữa.
* Quay lại điều hướng chữ cái đầu tiên Win32 gốc trong chế độ xem cây nội dung.
* Hộp thoại lỗi tải giờ đây hiển thị các thông báo lỗi chi tiết hơn.
* Webview giờ đây sẽ mở nhanh hơn và mượt hơn nhiều.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi khi mở webview trong epub chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi khi trình phân tích cú pháp RTF sẽ không đặt khoảng trắng giữa các từ trong các trường hợp hiếm.
* Đã sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF giờ đây có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab RTF và dòng feed giờ đây được hiển thị chính xác khi chúng xuất hiện trong tài liệu.
* Quay lại thư viện pdfium đã thử và kiểm tra đúng để phân tích PDF, làm cho kết xuất PDF đáng tin cậy hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Thêm Ctrl+Shift+T để mở lại tài liệu được đóng gần đây.
* Hộp thoại All Documents giờ đây hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Đã sửa một vài lỗi với trình phân tích cú pháp RTF.
* Đã sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như š, č, ć, ž của Bosnian) bị hỏng khi mở tệp qua phiên bản Paperback thứ hai.
* Đã sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Đã sửa tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa bản địa hóa các nút Có/Không trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung đơn giản và tiếng Việt!
* Thêm trình cập nhật tự động sẽ giờ đây thay thế phiên bản Paperback hiện được cài đặt của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn để đạt được dấu trang hoặc ghi chú, cảm ơn Andre Louis cho các âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho các tài liệu DAISY XML.
* Thêm hỗ trợ cho các tệp Open Document Text phẳng!
* Thêm hỗ trợ cho các bản trình bày Open Document phẳng!
* Thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Đã sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được hiển thị trong Web View.
* Đã sửa bảng không hiển thị đúng cách trong các tệp Markdown.
* PDF chỉ chứa hình ảnh sẽ giờ đây cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một.
* Giờ đây có thể kiểm tra các bản dựng phát triển mới thay vì các bản phát hành ổn định khi kiểm tra cập nhật.
* Nhúng đúng cách thông tin phiên bản trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng trong Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Menu ngữ cảnh kiểm soát văn bản giờ đây sẽ bao gồm các hành động đọc cụ thể thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một bảng trong webview.
* Thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật giờ đây hiển thị ghi chú phát hành khi có phiên bản mới được cung cấp.
* Đã sửa khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Có/Không trong các hộp thoại xác nhận.
* Đã sửa tải cấu hình khi chạy dưới quyền quản trị viên.
* Đã sửa xử lý nhận xét trong tài liệu XML và HTML.
* Đã sửa phân tích cú pháp TOC trong sách Epub 2.
* Đã sửa điều hướng đến mục tiếp theo có cùng chữ cái trong nội dung.
* Đã sửa hộp thoại tìm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước.
* Đã sửa TOC epub đôi khi ném bạn đến mục sai.
* Đã sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Đã sửa lỗi sai số một trong điều hướng liên kết.
* Đã sửa một số sách có khoảng trắng thừa ở cuối dòng của chúng.
* Đã sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử giờ đây được tắt đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều yếu tố nội bộ được tối ưu hóa, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được nhớ, và có thể điều hướng đến bằng các phím mũi tên alt+left/right.
* Thêm danh sách phần tử! Hiện tại nó chỉ hiển thị một cây tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Đã sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Đã sửa phân tích cú pháp Epub TOCs chứa đường dẫn tương đối.
* Đã sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Đã sửa bạn không thể sử dụng thanh cách để kích hoạt các nút OK/cancel trong hộp thoại TOC.
* Cải thiện xử lý tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng đưa ra hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu go ở dạng gọn gàng hơn nhiều đã được thêm vào hộp thoại tùy chọn, được kiểm tra theo mặc định.
* Thêm tùy chọn để điều hướng theo các phần tử cấu trúc quấn.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu được lấy focus hiện tại.
* Thêm một hệ thống cập nhật khá đơn giản, nhưng rất hiệu quả.
* Thêm tính năng bộ định thời ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Thêm hỗ trợ cho phân tích cú pháp sách điện tử FB2!
* Thêm hỗ trợ cho phân tích cú pháp bản trình bày OpenDocument!
* Thêm hỗ trợ cho phân tích cú pháp các tệp OpenDocument Text!
* Dấu trang giờ đây có thể được tạo để đánh dấu toàn bộ dòng, hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi sẽ như trước 0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được bao gồm trong dấu trang.
* Dấu trang giờ đây có thể có ghi chú văn bản tùy chọn được đính kèm! Điều hướng giữa các dấu trang chứa ghi chú với N và Shift+N, hoặc bật hộp thoại dấu trang với tất cả các dấu trang, chỉ ghi chú hoặc chỉ không ghi chú được chọn với phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả mạo thành XML giờ đây sẽ được xử lý đúng cách.
* Đã sửa tải các tài liệu Markdown lớn.
* Đã sửa nhấn space trong chế độ xem cây nội dung kích hoạt nút OK.
* Đã sửa xử lý khoảng trắng ở đầu thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa kiểm soát văn bản không lấy focus lại đôi khi khi quay lại cửa sổ Paperback.
* Đã sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Đã sửa kết xuất các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown giờ đây sẽ được hiển thị đúng cách.
* Nếu tải sách có tham số dòng lệnh trong khi phiên bản Paperback hiện có đang chạy, bạn sẽ không còn nhận được lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình giờ đây sẽ được tải và lưu đúng cách.
* Giờ đây có thể xóa dấu trang trực tiếp từ hộp thoại dấu trang.
* Giờ đây có thể nhập và xuất các dấu trang và vị trí đọc cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có phần mở rộng .paperback. Nếu tìm thấy tệp như vậy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngược lại, bạn có thể nhập chúng theo cách thủ công bằng mục trong menu công cụ.
* Các liên kết bên trong tài liệu giờ đây được hỗ trợ đầy đủ! Sử dụng k và shift+k để di chuyển về phía trước và phía sau qua chúng, và nhấn enter để mở/kích hoạt một.
* Nhiều yếu tố nội bộ được tối ưu hóa, làm cho ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown giờ đây được xử lý trước để tuân theo CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và các mục của chúng giờ đây được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi qua danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Numpad delete giờ đây hoạt động để xóa tài liệu từ thanh tab ngoài delete thông thường.
* Paperback giờ đây có thể tùy chọn giảm thiểu vào khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback giờ đây có thể dịch đầy đủ! Danh sách các ngôn ngữ mà nó hỗ trợ hiện khá nhỏ, nhưng nó luôn phát triển!
* Paperback giờ đây có một trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX giờ đây sẽ hiển thị nội dung cơ bản, chứa tất cả các trang trình bày.
* Đường dẫn đầy đủ đến tài liệu được mở giờ đây sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt giờ đây bao gồm một tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị cho bạn 10 tài liệu cuối cùng bạn đã mở, giờ đây nó sẽ hiển thị cho bạn một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn đã mở được truy cập qua một hộp thoại nhỏ.
* Các cải tiến nhỏ khác nhau cho các trình phân tích cú pháp trên toàn bộ bảng, bao gồm đặt dòng trống giữa các trang trình bày trong bản trình bày PPTX, sửa xử lý dòng mới bên trong các đoạn văn trong tài liệu word và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ cho bản trình bày PowerPoint!
* Đã sửa các mục menu nhất định không bị tắt khi không có tài liệu nào được mở.
* Đã sửa hướng của thanh trượt go to percent.
* Đã sửa nội dung trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Đã sửa khoảng trắng bị tước từ tiêu đề XHTML theo những cách lạ.
* Đã sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown giờ đây hỗ trợ tính năng nội dung! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng nội dung của riêng mình từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại ctrl+t.
* Tài liệu HTML giờ đây sẽ có tiêu đề được đặt trong thẻ tiêu đề, nếu nó tồn tại. Ngược lại, họ sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo lời nói. Điều này có nghĩa là không có DLL trình đọc màn hình được gửi cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ bây giờ, chẳng hạn như Microsoft Narrator.
* Chuyển đổi thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu dưới dạng văn bản thuần đã được viết lại hoàn toàn, và bây giờ nó cho phép bạn mở tài liệu dưới dạng văn bản thuần, HTML hoặc Markdown.
* Hộp thoại go to percent giờ đây bao gồm trường văn bản cho phép bạn nhập thủ công phần trăm để nhảy tới.
* Trình phân tích cú pháp HTML giờ đây sẽ nhận ra dd, dt và dl là các phần tử danh sách.
* Nội dung trong sách Epub giờ đây sẽ được bảo toàn chính xác.
* Khoảng trắng không phá vỡ Unicode giờ đây được xem xét khi tước các dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp chưa được công nhận mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Thêm biểu tượng menu Start tùy chọn vào trình cài đặt.
* Nội dung giờ đây sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có một mục con và cha với cùng văn bản ở cùng vị trí, bạn sẽ chỉ thấy mục cha.
* Đã sửa nội dung trong một số tài liệu CHM nhất định.
* Đã sửa nội dung trong sách Epub 3 với các đường dẫn tuyệt đối.
* Tài liệu CHM giờ đây sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang tùy thích trên bao nhiêu tài liệu tùy thích. Bạn có thể nhảy về phía trước và phía sau qua chúng với b và shift+b, đặt một bằng control+shift+b, và đưa ra hộp thoại để nhảy đến dấu trang cụ thể với control+b.
* Thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn, và tự động thiết lập liên kết tệp cho bạn.
* Tệp văn bản với BOM giờ đây sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Thêm thông tin nhiều hơn vào thanh trạng thái. Giờ đây nó sẽ hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Nhận xét HTML, cũng như nội dung của các thẻ tập lệnh và kiểu, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển một đường dẫn tương đối đến Paperback trên dòng lệnh, nó giờ đây sẽ giải quyết nó đúng cách.
* Chuyển động phần trăm giờ đây được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập bằng control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả được biết đến giờ đây sẽ luôn có mặc định.
* Logic lưu vị trí giờ đây thông minh hơn nhiều và sẽ chỉ ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu mà bạn đã lấy focus khi bạn đóng Paperback giờ đây được nhớ lại trên các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page giờ đây sẽ được vệ sinh chặt chẽ hơn.
* Đã sửa điều hướng nội dung trong sách epub 3 với các đường dẫn tương đối trong biểu thức tuyên bố của chúng.

### Phiên bản 0.3.0
* Đã sửa nội dung trong sách epub với biểu thức được mã hóa URL.
* Đã sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Đã sửa mức sử dụng CPU cao trong các tài liệu có tiêu đề dài do hồi quy trong wxWidgets.
* Đã sửa tải các tệp văn bản UTF-8.
* Đã sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa sự cố khi thoát ứng dụng trong một số trường hợp.
* Thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt bao quanh từ!
* Giờ đây có thể quyên góp cho sự phát triển của Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án tài trợ ở dưới cùng của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown giờ đây sẽ luôn có tiêu đề, và Paperback giờ đây sẽ có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF giờ đây sẽ luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF đáng tin cậy hơn nhiều trên toàn bộ bảng.
* Bạn giờ đây chỉ có thể có một phiên bản Paperback chạy cùng một lúc. Chạy paperback.exe với tên tệp trong khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bạn giờ đây có thể nhấn delete trên tài liệu trong kiểm soát tab để đóng nó.

### Phiên bản 0.2.1
* Thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách các tài liệu được mở của bạn.
* Đã sửa các phím gạch đầu dòng đôi khi mở các tài liệu gần đây nếu bạn có đủ số chúng.
* Paperback giờ đây sẽ xóa gạch nối mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa điều hướng tiêu đề đôi khi đưa bạn đến ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm các phím cho điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Đã sửa tải epub với tên tệp được mã hóa URL trong biểu thức tuyên bố của chúng.
* Đã sửa tải sách epub 3 với XHTML được nhúng bên trong chúng.
* Một thông báo bây giờ được nói nếu tài liệu không hỗ trợ nội dung hoặc phần, đối với các mục menu bị tắt.
* Thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu được mở gần đây của bạn, và nhấn enter trên một sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử 25 lần tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Các tài liệu được mở trước đây giờ đây được nhớ lại trên các lần khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Thêm shift+f1 để mở readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Phát hành ban đầu.
