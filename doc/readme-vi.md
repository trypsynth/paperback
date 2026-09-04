<!-- machine-translated from doc/readme.md (source-hash: c5a741eb100e6fbc; sections: f48ce952,d4d8160f,a02f4421,a4ffb7f7,91be3b41,55bac79e,a548b5d0,71df8e94,e9860ee8,c7735cbe); please review and edit as needed -->

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

Paperback hỗ trợ các định dạng và tiện ích mở rộng sau:

* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Sách điện tử FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách âm thanh M4B (`.m4b`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bài thuyết trình PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng theo kiểu ưu tiên bàn phím. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Khi macOS khác biệt, phím tương đương được ghi chú trong dấu ngoặc — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã được yêu cầu bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đã đóng gần đây.
* `Ctrl+R`: Hiển thị hộp thoại "Tất cả tài liệu" (từ Tài liệu gần đây).
* `Ctrl+Q`: Thoát (chỉ Windows; trên macOS điều này nằm dưới menu ứng dụng thay thế).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Tìm.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi đến dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi đến phần trăm.
* `Ctrl+P`: Đi đến trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Thông báo phần trăm đọc hiện tại của bạn.
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
* `Ctrl+Alt+B`: Nhảy đến các dấu trang chỉ.
* `Ctrl+Alt+M`: Nhảy đến các ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem văn bản ghi chú ở vị trí hiện tại.
* `Shift+K`: Liên kết trước.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình vẽ trước.
* `F`: Hình vẽ tiếp theo.
* `Shift+T`: Bảng trước.
* `T`: Bảng tiếp theo.
* `Shift+S`: Dấu phân cách trước.
* `S`: Dấu phân cách tiếp theo.
* `Shift+L`: Danh sách trước.
* `L`: Danh sách tiếp theo.
* `Shift+I`: Mục danh sách trước.
* `I`: Mục danh sách tiếp theo.
* `Shift+,`: Đi đến đầu của vùng chứa hiện tại (danh sách hoặc bảng).
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
* `Ctrl+Shift+B`: Bật/tắt dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang ở lựa chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt ngắt dòng từ.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh về phía trước.
* `;`: Tìm kiếm kể chuyện âm thanh về phía sau.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, dưới menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím xem tài liệu bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu đã chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết tại con trỏ, hoặc mở chế độ xem bảng khi ở trên dấu bảng.
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
* Audiobooks không còn làm cho trình đọc màn hình đọc một loạt khoảng trắng khi bạn tập trung vào trường văn bản.
* Audiobooks hiện đặt tên tệp khi bạn bước qua chúng theo phần.
* Audiobooks hiện báo cáo độ dài thực tế của chúng, thay vì tuyên bố rằng mọi tệp trong chúng kéo dài 24 giờ.
* Đóng Web View bằng Escape không còn hiển thị cảnh báo gỡ lỗi sau khi bạn đã theo dõi một liên kết bên trong nó.
* Sao chép sau Select All hiện cung cấp cho bạn toàn bộ tài liệu, thay vì chỉ phần của nó được tải.
* Find hiện đi thẳng đến dòng mà nó tìm thấy, thay vì làm bạn nghe trình đọc màn hình đọc lại cửa sổ khi tập trung quay trở lại sách.
* Sửa EPUB có khối ZIP64 lạc lẫm từ chối mở với "Invalid local file header".
* Sửa các tài liệu dài quay lại điểm bắt đầu của chúng khi trình đọc màn hình đọc liên tục qua chúng.
* Các liên kết trong WebView hiện đưa bạn đến phần mà chúng trỏ đến, thay vì không thành công với "File not found".
* Thông báo tự động "Document reloaded" không còn ngắt trình đọc màn hình của bạn giữa câu, thay vào đó chờ nó hoàn thành những gì nó đang nói.
* Tab Chung của hộp thoại Cài đặt hiện tạo tab qua các tùy chọn của nó theo thứ tự chúng xuất hiện trên màn hình, với kênh cập nhật trực tiếp sau tùy chọn kiểm tra cập nhật.
* Windows hiện sẽ luôn hiển thị "Paperback" trong menu Open With, thay vì tagline đầy đủ của chương trình.
* Word Count và Document Info hiện cho biết audiobook chứa bao nhiêu tệp và kéo dài bao lâu tổng cộng.

### Phiên bản 0.9.1
* Âm thanh dấu trang và ghi chú hiện phát trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Sửa các dấu ngoặc kép xoắn, dấu gạch em và các ký tự tương tự biến mất từ ​​tài liệu RTF, chạy các từ xung quanh chúng lại với nhau.
* Sửa hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản rối loạn.
* Sửa menu Tài liệu gần đây giữ các mục cũ cho đến khi có điều gì khác xảy ra để xây dựng lại nó.
* Bàn phím tắt quay trở lại trong mọi bản dịch, vì vậy menu của Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn hiện mở nhanh hơn đến bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Tùy chọn đã được đổi tên thành Cài đặt, phù hợp với các ứng dụng di động và, trên macOS, quy ước nền tảng.
* Paperback hiện nhớ vị trí cửa sổ, kích thước và trạng thái được phóng to của nó giữa các lần chạy.
* Các dạng số nhiều hiện được dịch, vì vậy các thông báo đếm các thứ đọc chính xác trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn ncc.html của sách DAISY hiện mở toàn bộ audiobook thay vì chỉ văn bản của nó.
* Các tên hành động của hộp thoại Tùy chỉnh Phím tắt bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu hiện xuất hiện đầu tiên trong thanh tiêu đề, vì vậy các sách đang mở có thể được phân biệt trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật hiện được dịch.

### Phiên bản 0.9.0

#### Đã thêm

##### Chung
* Một công cụ CLI gọi là pb để nhanh chóng chuyển đổi bất kỳ định dạng nào Paperback hỗ trợ sang HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Tùy chọn View Source để mở nguồn tài liệu trong một tab mới, hữu ích để chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, có nghĩa là bạn có thể tải sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều lạ nào được tìm thấy với điều này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Bật tắt toàn màn hình.

##### Hộp thoại Tất cả các tài liệu
* Nút định vị để định vị các sách bị thiếu mà vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả các tài liệu.

##### Tùy chọn và Khả năng đọc
* Tab khả năng đọc với các tùy chọn sau:
    * Gói từ (được chuyển từ chung);
    * Kết xuất bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách ký tự;
    * Căn chỉnh văn bản.
* Mục menu gói từ và phím tắt tiếp theo.
* Bật tắt để xác định cách bạn muốn bảng được hiển thị, và hợp nhất cách bảng được hiển thị trong các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo container.
* Tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng nhau để công bố phần trăm hiện tại của bạn thông qua một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một dấu trang trên mỗi tài liệu, và chúng vẫn tồn tại. Sử dụng dấu gạch chéo để đặt một và dấu gạch chéo ngược để nhảy đến nó.

##### Số lượng từ
* Thời gian đọc ước tính trong hộp thoại số lượng từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu lựa chọn hoạt động khi bạn mở hộp thoại số lượng từ, bao nhiêu từ bạn đã chọn sẽ được hiển thị.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, tiếng Phần Lan và tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Bộ cập nhật
* Nút hủy trong hộp thoại cập nhật đang diễn ra.
* Bộ cập nhật hiện xác thực tệp được tải xuống chưa bị can thiệp.

##### Web View
* Webview hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Audiobooks
* Khả năng phát audiobooks, hiện hỗ trợ cả audio DAISY (bao gồm audio DAISY + văn bản) và các tệp zip của tệp âm thanh.
* Phím tắt bàn phím và mục menu để phát/tạm dừng lời tường thuật, tìm kiếm tiến và lùi, và điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn xem tìm kiếm quá cuối chương có tiếp tục vào chương tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, số liệu và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Sửa

##### Chung
* Tài liệu được mã hóa trong các mã hóa CJK kế thừa, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị đúng thay vì một loạt mojibake.
* "Reopen last closed" cố gắng mở lại readme được đính kèm.
* Tab đã chọn của bạn không được tập trung đúng sau khi khởi động lại Paperback.
* Cách Paperback xử lý các tệp trên ổ đĩa mạng Windows: nhấn hiển thị tệp trong thư mục hiện đúng tập trung vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Tệp .paperback sẽ không còn được tải một cách bắt buộc khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy.
* Mở thư mục chứa hiện tập trung vào tệp đã cho trong trình khám phá.
* Mở readme sẽ tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng Paperback hiện sẽ được chia tỷ lệ chính xác trên màn hình DPI cao.
* Menu hiện được cập nhật đúng và tập trung di chuyển đến điều khiển văn bản khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục nội bộ trên mỗi ký tự.

##### Hộp thoại Tất cả các tài liệu
* Escape không đóng các hộp thoại Document Info và All Documents.
* Thanh tiêu đề không được cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở qua Shift+F1.
* Xóa tài liệu khỏi hộp thoại recents sẽ hiện đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được bảo tồn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Go to Line, Go to Page và Go to Percent đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Find và Find Next không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú hiện phát chính xác độc quyền khi bạn điều hướng qua một từ chứa.

##### Khả năng đọc
* Áp dụng gói từ bắn bạn đến đầu tài liệu của bạn.

##### Web View
* Hộp thoại webview không thể thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh hiện phải hiển thị đúng trong webview được nhúng.

##### Bộ cập nhật
* Bộ cập nhật hiện hiển thị đúng nội dung của thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích cú pháp tài liệu RTF với các ký tự không phải Latin.
* Các nhóm RTF `\pict` vì vậy dữ liệu hình ảnh được nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các mỏ neo Filepos trong sách Mobi chia tags HTML và đặt rác vào văn bản sách.
* Liên kết trong sách Mobi kế thừa.
* Cải thiện phân tích cú pháp AZW3 nhiều hơn.

##### Tài liệu Word
* Tài liệu Word có tên kiểu dành riêng cho ngôn ngữ không kết xuất các tiêu đề của chúng đúng cách.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay lại trích xuất văn bản thuần túy cho các tệp PDF được gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang sẽ không còn làm Paperback gặp sự cố khi mở.

### Phiên bản 0.8.5
* Thêm hỗ trợ trang cho sách epub.
* Thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện Word kế thừa, Word hiện đại và Powerpoint hiện đại được hỗ trợ, với Powerpoint kế thừa được lên kế hoạch cho tương lai.
* Thêm hỗ trợ cho tài liệu Microsoft Word kế thừa!
* Thêm hỗ trợ cho bài thuyết trình Powerpoint kế thừa!
* Thêm hỗ trợ cho sách mobi và AZW3!
* Thêm hỗ trợ cho các tệp PDF được gắn thẻ!
* Thêm phím tắt ctrl+q để thoát ứng dụng.
* Thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng hiện phải được hiển thị đúng cách.
* Tài liệu CHM hiện hỗ trợ đúng điều hướng liên kết nội bộ.
* Sửa go to page bị tắt đi 1.
* Sửa phím Escape không hoạt động để đóng hộp thoại mở như.
* Sửa menu ngữ cảnh trình đọc không hiển thị khi nhấp chuột phải hoặc phím Ứng dụng.
* Sửa tài liệu sai đôi khi được tập trung khi mở tài liệu từ dòng lệnh.
* Các tệp PDF chỉ có hình ảnh một lần nữa được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Hiện có thể điều hướng qua hình ảnh và số liệu với g/shift+g và f/shift+f tương ứng.
* Paperback sẽ hiện tôn trọng cài đặt chế độ tối ứng dụng của bạn.
* Xóa hỗ trợ DAISY XML vì nó không còn cần thiết.
* Chuyển trở lại điều hướng chữ cái đầu tiên gốc Win32 trong chế độ xem cây mục lục.
* Hộp thoại lỗi tải hiện hiển thị các thông báo lỗi chi tiết hơn.
* Webview sẽ hiện mở nhanh hơn nhiều và mượt mà hơn.

### Phiên bản 0.8.2
* Thêm hỗ trợ trang cho tài liệu RTF!
* Sửa lỗi khi mở webview trong épubs chứa các liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Sửa lỗi khi trình phân tích cú pháp RTF sẽ không đặt khoảng cách giữa các từ trong các trường hợp hiếm.
* Sửa các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab và nguồn cấp dữ liệu hàng RTF hiện được kết xuất chính xác như chúng xuất hiện trong tài liệu.
* Chuyển trở lại thư viện pdfium được thử và đúng để phân tích các tệp PDF, làm cho kết xuất PDF đáng tin cậy hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Thêm Ctrl+Shift+T để mở lại tài liệu đã đóng cuối cùng.
* Hộp thoại Tất cả tài liệu hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Sửa một vài lỗi với trình phân tích RTF.
* Sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnia š, č, ć, ž) trở thành tham nhũng khi mở tệp qua phiên bản Paperback thứ hai.
* Sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ viết hoa.
* Sửa tải tài liệu chậm khi mở các tệp lớn.
* Sửa bản địa hóa của các nút Có/Không trong các hộp thoại xác nhận.

### Phiên bản 0.8.0
* Thêm bản dịch tiếng Nhật, tiếng Trung Quốc đơn giản và tiếng Việt!
* Thêm trình cập nhật tự động sẽ hiện thay thế phiên bản hiện đang cài đặt của bạn thay vì chỉ tải xuống phiên bản mới!
* Thêm phản hồi âm thanh tùy chọn khi đạt tới dấu trang hoặc ghi chú, cảm ơn Andre Louis cho các âm thanh!
* Thêm hỗ trợ tài liệu RTF!
* Thêm hỗ trợ cho tài liệu DAISY XML.
* Thêm hỗ trợ cho tệp Văn bản Tài liệu Mở phẳng!
* Thêm hỗ trợ cho bài thuyết trình Tài liệu Mở phẳng!
* Thêm hỗ trợ cho các bộ tách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được kết xuất trong Web View.
* Sửa bảng không kết xuất đúng cách trong tệp Markdown.
* Các tệp PDF chỉ có hình ảnh sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một tệp.
* Nhúng thông tin phiên bản đúng cách trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích các tệp PDF, dẫn đến độ tin cậy, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng bằng Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ dàng bảo trì và mở rộng hơn.
* Menu ngữ cảnh của điều khiển văn bản sẽ hiện bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa bảng bằng T và Shift+T, rồi nhấn Enter để xem bảng trong trình kết xuất dựa trên web.
* Thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Thêm nút Clear All vào hộp thoại All Documents.
* Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi phiên bản mới có sẵn.
* Sửa khôi phục cửa sổ từ khay hệ thống.
* Sửa bản dịch nút Có/Không trong các hộp thoại xác nhận.
* Sửa cấu hình tải khi chạy dưới quyền quản trị viên.
* Sửa xử lý nhận xét trong tài liệu XML và HTML.
* Sửa phân tích TOC trong sách Epub 2.
* Sửa điều hướng đến mục tiếp theo với cùng một chữ cái trong mục lục.
* Sửa hộp thoại tìm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước đó.
* Sửa TOC epub đôi khi ném bạn đến mục sai.
* Sửa các vấn đề xử lý khoảng trắng khác nhau trong các thẻ XML, HTML và pre.
* Sửa lỗi off-by-one trong điều hướng liên kết.
* Sửa một số sách có khoảng trắng ở cuối trên các dòng của chúng.
* Sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử hiện được vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và tính duy trì.

### Phiên bản 0.6.1
* Thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Thêm tính năng go to previous/next position rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ được ghi nhớ và có thể điều hướng đến bằng các phím mũi tên alt+left/right.
* Thêm danh sách các phần tử! Hiện tại nó chỉ hiển thị cây tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Thêm tùy chọn để bắt đầu Paperback ở chế độ được phóng to theo mặc định.
* Sửa các liên kết trong một số tài liệu Epub không hoạt động đúng cách.
* Sửa phân tích cú pháp Epub TOCs chứa đường dẫn tương đối.
* Sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Sửa tiêu đề của một số chương epub không hiển thị đúng cách trong hộp thoại TOC.
* Sửa không thể sử dụng thanh cách để kích hoạt các nút OK/cancel trong hộp thoại TOC.
* Cải thiện xử lý tiêu đề trong tài liệu Word.
* Bạn sẽ nhận được phản hồi nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng đưa hộp thoại lên.

### Phiên bản 0.6.0
* Đã thêm tùy chọn mới để hiển thị menu đi ở dạng nhỏ gọn hơn nhiều vào hộp thoại tùy chọn, được kiểm tra theo mặc định.
* Thêm tùy chọn để điều hướng bằng các phần tử cấu trúc bao quanh.
* Thêm tùy chọn vào menu công cụ để mở thư mục chứa tài liệu hiện tại được tập trung.
* Thêm hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Thêm tính năng hẹn giờ ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Thêm hỗ trợ cho sách phân tích FB2!
* Thêm hỗ trợ cho bài thuyết trình OpenDocument!
* Thêm hỗ trợ cho tệp Văn bản OpenDocument!
* Dấu trang hiện có thể được tạo để đánh dấu toàn bộ một dòng, hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như trước 0.6, nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được bao gồm trong dấu trang.
* Dấu trang hiện có thể có ghi chú văn bản tùy chọn được đính kèm với chúng! Điều hướng giữa các dấu trang chứa ghi chú bằng N và Shift+N, hoặc bật hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ các ghi chú không được chọn bằng các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "bookmark x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML hiện sẽ được xử lý đúng cách.
* Sửa tải các tài liệu Markdown lớn.
* Sửa nhấn phím cách trong chế độ xem cây mục lục kích hoạt nút OK.
* Sửa xử lý khoảng trắng ở đầu thẻ pre trong cả tài liệu HTML và XHTML.
* Sửa điều khiển văn bản không lấy lại tiêu điểm đôi khi khi quay trở lại cửa sổ Paperback.
* Sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Sửa kết xuất ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown sẽ hiện được kết xuất đúng cách.
* Nếu tải một cuốn sách với tham số dòng lệnh khi một phiên bản Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình sẽ được tải và lưu đúng cách.
* Hiện có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Hiện có thể nhập và xuất dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có tiện ích .paperback. Nếu tệp như vậy được tìm thấy trong cùng một thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng theo cách thủ công bằng mục trong menu công cụ.
* Liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng k và shift+k để di chuyển tiến và lùi qua chúng, rồi nhấn enter để mở/kích hoạt một liên kết.
* Nhiều tái cấu trúc nội bộ, làm cho ứng dụng nhanh hơn và nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi theo danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Phím xóa trên bàn phím số hiện hoạt động để xóa tài liệu khỏi thanh tab ngoài phím xóa bình thường.
* Paperback hiện có thể tùy chọn thu nhỏ vào khay hệ thống của bạn! Tùy chọn này được tắt theo mặc định, nhưng bật nó sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng sinh ra.
* Paperback hiện hoàn toàn có thể dịch được! Danh sách các ngôn ngữ nó hỗ trợ hiện khá nhỏ, nhưng nó đang liên tục phát triển!
* Paperback hiện có trang web chính thức tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ hiện hiển thị mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu đã mở sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem tệp readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó sẽ hiển thị một số có thể tùy chỉnh, với phần còn lại của các tài liệu bạn từng mở có thể truy cập được thông qua một hộp thoại nhỏ.
* Nhiều cải tiến nhỏ cho các trình phân tích cú pháp trên bảng, bao gồm đặt một dòng trống giữa các slide trong bài thuyết trình PPTX, sửa xử lý dòng mới bên trong các đoạn trong tài liệu word và thêm các dấu đầu dòng cho các mục danh sách.

### Phiên bản 0.5.0
* Thêm hỗ trợ tài liệu Microsoft Word!
* Thêm hỗ trợ cho bài thuyết trình PowerPoint!
* Sửa các mục menu nhất định không được vô hiệu hóa khi không có tài liệu nào được mở.
* Sửa hướng của thanh trượt go to percent.
* Sửa mục lục trong sách Epub có đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Sửa khoảng trắng bị tước khỏi các tiêu đề XHTML theo những cách lạ.
* Sửa xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục của riêng nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị điều đó cho bạn trong hộp thoại ctrl+t.
* Tài liệu HTML sẽ hiện có tiêu đề được đặt trong thẻ tiêu đề nếu nó tồn tại. Nếu không, họ sẽ tiếp tục sử dụng tên tệp mà không có tiện ích.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL trình đọc màn hình nào được gửi cùng với chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở một loạt sách epub rộng hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu của bạn dưới dạng văn bản thuần túy đã được hoàn toàn làm lại, hiện nó cho phép bạn mở tài liệu của bạn dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm một trường văn bản cho phép bạn thủ công nhập phần trăm để nhảy đến.
* Trình phân tích HTML sẽ hiện công nhận dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub sẽ một lần nữa được bảo tồn chính xác.
* Ký tự không phá vỡ unicode hiện được xem xét khi tước các dòng trống.
* Bạn sẽ không còn được hỏi muốn mở tệp không được công nhận như thế nào mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Thêm biểu tượng menu Bắt đầu tùy chọn vào trình cài đặt.
* Mục lục hiện phải sạch hơn trong một vài trường hợp, ví dụ nếu bạn có mục con và mục cha với cùng một văn bản ở cùng vị trí, bạn sẽ chỉ thấy mục cha.
* Sửa mục lục trong các tài liệu CHM nhất định.
* Sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM hiện phải hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Thêm hỗ trợ tệp CHM!
* Thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang tùy thích trong nhiều tài liệu. Bạn có thể nhảy tiến và lùi qua chúng bằng b và shift+b, đặt một bằng control+shift+b và đưa hộp thoại lên để nhảy đến dấu trang cụ thể bằng control+b.
* Thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập các liên kết tệp cho bạn.
* Tệp văn bản có BOM hiện phải được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Thêm thông tin chi tiết hơn nhiều vào thanh trạng thái. Nó sẽ hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Nhận xét HTML, cũng như nội dung của các thẻ kịch bản và kiểu, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ giải quyết nó đúng cách.
* Chuyển động phần trăm hiện được xử lý bởi hộp thoại dựa trên thanh trượt của riêng nó, có thể truy cập bằng control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả đã biết hiện sẽ luôn có mặc định.
* Logic lưu vị trí hiện thông minh hơn nhiều và chỉ phải viết vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã tập trung khi đóng Paperback hiện được ghi nhớ giữa các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại go to line và go to page hiện phải được vệ sinh chặt chẽ hơn.
* Sửa điều hướng mục lục trong sách epub 3 có đường dẫn tương đối trong bản kê khai của chúng.

### Phiên bản 0.3.0
* Sửa mục lục trong sách epub có bản kê khai được mã hóa URL.
* Sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode đa byte.
* Sửa mức sử dụng CPU cao trong các tài liệu có tiêu đề dài do một hồi quy trong wxWidgets.
* Sửa tải tệp văn bản UTF-8.
* Sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Sửa sự cố khi thoát ứng dụng trong một số trường hợp.
* Thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt gói từ!
* Hiện có thể quyên góp cho sự phát triển Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc thông qua liên kết dự án nhà tài trợ ở dưới cùng của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown sẽ hiện luôn có tiêu đề, và Paperback hiện phải có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF sẽ hiện luôn có tiêu đề, ngay cả khi siêu dữ liệu bị mất.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF đáng tin cậy hơn nhiều trên bảng.
* Bạn hiện chỉ có thể chạy một phiên bản Paperback tại một thời điểm. Chạy paperback.exe với tên tệp khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bạn hiện có thể nhấn xóa trên tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Thêm số trang tổng cộng vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách tài liệu đã mở của bạn.
* Sửa các phím tiêu đề đôi khi mở tài liệu gần đây nếu bạn có đủ số lượng.
* Paperback hiện sẽ loại bỏ các dấu gạch ngoặc kép không cần thiết khỏi đầu ra văn bản.
* Sửa điều hướng tiêu đề đôi khi đặt bạn ở ký tự sai.

### Phiên bản 0.2.0
* Thêm hỗ trợ tài liệu markdown!
* Thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Thêm phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Sửa tải épubs với tên tệp được mã hóa URL trong bản kê khai của chúng.
* Sửa tải sách epub 3 với XHTML được nhúng bên trong chúng.
* Thông báo hiện được phát nói nếu tài liệu không hỗ trợ mục lục hoặc phần, trái ngược với các mục menu được vô hiệu hóa.
* Thêm menu tài liệu gần đây! Hiện tại nó lưu trữ 10 tài liệu đã mở cuối cùng của bạn, và nhấn enter trên một tài liệu sẽ mở nó để đọc.
* Viết lại hoàn toàn hộp thoại Tìm, làm cho nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử của 25 lần tìm kiếm cuối cùng và hỗ trợ biểu thức chính quy!
* Các tài liệu đã mở trước đây hiện được ghi nhớ trong các lần khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong menu công cụ.
* Thêm shift+f1 để mở tệp readme trực tiếp trong Paperback.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
