<!-- machine-translated from doc/readme.md (source-hash: bdf582cc25a739ea); please review and edit as needed -->

# Paperback - phiên bản 0.9.0

## Giới thiệu

Paperback là một trình đọc ebook và tài liệu nhẹ, nhanh và dễ tiếp cận dành cho mọi người, từ người đọc bình thường đến người dùng nâng cao. Nó được thiết kế để tương thích với trình đọc màn hình, tốc độ nhanh và trải nghiệm không rườm rà.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản macOS ARM hiện đại. Các ứng dụng gốc cho iOS và Android đang được phát triển tích cực, với các bản dựng thử nghiệm công khai được dự kiến sẽ ra mắt ngay sau bản phát hành 0.9.0 cho máy tính, trước khi có bản phát hành 1.0 hợp nhất bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không yêu cầu cài đặt bất kỳ phần mềm nào trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, thậm chí trên phần cứng cũ.
* Giao diện thẻ đơn giản, cho phép bạn mở bao nhiêu tài liệu cạnh nhau tùy ý.
* Lưu chính xác vị trí đọc của bạn trong mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu bạn đang mở khi đóng chương trình, và khôi phục chúng ở lần khởi chạy tiếp theo.
* Bao gồm chức năng điều hướng tương tự như trong chế độ duyệt web của nhiều trình đọc màn hình để điều hướng qua tài liệu một cách nhanh chóng và dễ dàng.
* Bao gồm một hộp thoại tìm kiếm mạnh mẽ, với các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn di động, hoặc được cài đặt với các liên kết tệp được thiết lập tự động.
* Hỗ trợ một loạt lớn các định dạng tệp phổ biến.

## Tương thích với trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề đã biết đối với người dùng JAWS.

### JAWS và màn hình chữ nổi

Nếu bạn dùng JAWS với màn hình chữ nổi, bạn có thể thấy rằng các đoạn văn dài bị cắt bớt khi lướt tiến bằng các phím điều hướng của màn hình. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là lỗi trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải lỗi của chính Paperback, và là lỗi đã mất khá nhiều thời gian mới tìm ra cách khắc phục, xét đến sự "nhiệt tình" của Vispero trong việc phản hồi các vấn đề liên quan đến phần mềm nguồn mở.

Cách xử lý tạm thời, cuối cùng được đưa ra qua nhóm thảo luận JAWS sau nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng nên bật "Pan Text by Paragraph", nếu không màn hình của bạn sẽ dừng lại ở đoạn văn đang hoạt động thay vì tiến lên. Khi cả hai thiết lập đã được áp dụng, việc lướt sẽ hoạt động đúng.

## Các loại tệp hiện được hỗ trợ

Paperback hỗ trợ các định dạng và phần mở rộng sau:

* Tệp trợ giúp CHM (`.chm`)
* Sách DAISY (`.opf`, `.zip`)
* Sách EPUB (`.epub`)
* Ebook FB2 (`.fb2`)
* Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
* Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`, `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
* Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
* Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
* Bản trình bày OpenDocument (`.odp`, `.fodp`)
* Tệp văn bản OpenDocument (`.odt`, `.fodt`)
* Tài liệu PDF (`.pdf`)
* Bản trình bày PowerPoint (`.pptx`, `.pptm`, `.ppt`)
* Tài liệu RTF (`.rtf`)
* Tệp văn bản thuần và tệp log (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế để sử dụng ưu tiên bàn phím. Dưới đây là các phím tắt hiện có.

Các phím tắt bên dưới dành cho Windows. Ở những chỗ macOS khác biệt, phím tương đương được ghi trong ngoặc đơn — chủ yếu vì Ctrl+G, Ctrl+W và Alt+Left/Right đã được các quy ước hệ thống hoặc ứng dụng khác chiếm dụng trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu vừa đóng.
* `Ctrl+R`: Hiển thị hộp thoại "All Documents" (từ Recent Documents).
* `Ctrl+Q`: Thoát (chỉ trên Windows; trên macOS mục này nằm trong menu ứng dụng).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Tìm kiếm.
* `F3` (macOS: `Cmd+G`): Tìm tiếp.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm ngược.
* `Ctrl+G` (macOS: `Cmd+L`): Đi tới dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi tới phần trăm.
* `Ctrl+P`: Đi tới trang (khi tài liệu hiện tại hỗ trợ).
* `=`: Thông báo phần trăm đọc hiện tại của bạn.
* `Alt+Left` (macOS: `Cmd+[`): Lùi lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Tiến lên trong lịch sử điều hướng.
* `[`: Phần trước.
* `]`: Phần sau.
* `Shift+H`: Tiêu đề trước.
* `H`: Tiêu đề sau.
* `Shift+1` đến `Shift+6`: Tiêu đề trước ở cấp 1-6.
* `1` đến `6`: Tiêu đề sau ở cấp 1-6.
* `Shift+P`: Trang trước.
* `P`: Trang sau.
* `Shift+B`: Dấu trang trước.
* `B`: Dấu trang sau.
* `/`: Đặt dấu trang tạm thời.
* `\`: Nhảy tới dấu trang tạm thời.
* `Shift+N`: Ghi chú trước.
* `N`: Ghi chú sau.
* `Ctrl+B`: Nhảy tới tất cả dấu trang và ghi chú.
* `Ctrl+Alt+B`: Chỉ nhảy tới dấu trang.
* `Ctrl+Alt+M`: Chỉ nhảy tới ghi chú.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý thay vì Cmd): Xem nội dung ghi chú tại vị trí hiện tại.
* `Shift+K`: Liên kết trước.
* `K`: Liên kết sau.
* `Shift+G`: Hình ảnh trước.
* `G`: Hình ảnh sau.
* `Shift+F`: Hình minh họa trước.
* `F`: Hình minh họa sau.
* `Shift+T`: Bảng trước.
* `T`: Bảng sau.
* `Shift+S`: Dấu phân cách trước.
* `S`: Dấu phân cách sau.
* `Shift+L`: Danh sách trước.
* `L`: Danh sách sau.
* `Shift+I`: Mục danh sách trước.
* `I`: Mục danh sách sau.
* `Shift+,`: Đi tới đầu của khối chứa hiện tại (danh sách hoặc bảng).
* `,`: Đi tới sau khi kết thúc khối chứa hiện tại (danh sách hoặc bảng).

### Menu Tools

* `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý thay vì Cmd): Hiển thị số từ của tài liệu hiện tại.
* `Ctrl+I`: Hiển thị thông tin tài liệu.
* `Ctrl+T`: Hiển thị mục lục.
* `F7`: Hiển thị danh sách phần tử.
* `Ctrl+Shift+C`: Mở thư mục chứa.
* `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
* `Ctrl+U`: Xem mã nguồn tài liệu trong tab mới.
* `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
* `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
* `Ctrl+E`: Xuất tài liệu hiện tại sang văn bản thuần.
* `Ctrl+Shift+B`: Bật/tắt dấu trang tại vùng chọn/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc sửa ghi chú dấu trang tại vùng chọn/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt ngắt dòng.
* `Ctrl+Space`: Phát/tạm dừng thuyết minh âm thanh.
* `'`: Tua thuyết minh âm thanh về phía trước.
* `;`: Tua thuyết minh âm thanh về phía sau.
* `Ctrl+'`: Tăng khoảng tua âm thanh.
* `Ctrl+;`: Giảm khoảng tua âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, nằm trong menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại Giới thiệu.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím bổ sung trong chế độ xem tài liệu

* `Delete` / `Numpad Delete` trên thanh tab: Đóng tab tài liệu đang chọn.
* `Enter` hoặc `Space` trong nội dung tài liệu: Kích hoạt liên kết tại con trỏ, hoặc mở chế độ xem bảng khi đang ở dấu hiệu bảng.
* `Shift+F10` hoặc phím Menu/Application trong nội dung tài liệu: Mở menu ngữ cảnh.

## Các ngôn ngữ được hỗ trợ

Paperback được dịch sang nhiều ngôn ngữ khác nhau, và ngày càng có thêm nhiều ngôn ngữ mới. Danh sách đầy đủ như sau.

Để biết cách đóng góp, vui lòng đọc [Hướng dẫn dịch thuật](translating.md) của chúng tôi.

* Tiếng Bosnia
* Tiếng Séc
* Tiếng Hà Lan
* Tiếng Phần Lan
* Tiếng Pháp
* Tiếng Đức
* Tiếng Nhật
* Tiếng Ba Lan
* Tiếng Bồ Đào Nha (Brazil)
* Tiếng Nga
* Tiếng Trung giản thể
* Tiếng Serbia
* Tiếng Tây Ban Nha
* Tiếng Việt

## Ghi công
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: người đóng góp chính.

### Quyên góp
Những người sau đây đã quyên góp một khoản nào đó cho việc phát triển Paperback. Nếu bạn quyên góp, tên bạn sẽ không tự động được thêm vào đây; tôi chỉ thêm những người muốn khoản quyên góp của họ được công khai.

Lưu ý: Tôi coi việc trở thành nhà tài trợ công khai trên GitHub là lý do để tự động đưa vào danh sách này.

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

### Phiên bản 0.9.0

#### Đã thêm

##### Tổng quát
* Một công cụ CLI, tên là pb, để chuyển đổi nhanh bất kỳ định dạng nào Paperback hỗ trợ sang HTML, Markdown hoặc văn bản thuần.
* Một tùy chọn để tải lại các tài liệu đã bị các chương trình khác sửa đổi trên đĩa.
* Tùy chọn Xem nguồn để mở nguồn của tài liệu trong một tab mới, hữu ích chẳng hạn khi chỉnh sửa Markdown.
* Văn bản tài liệu giờ đã được phân trang, nghĩa là bạn có thể tải những cuốn sách với hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều bất thường nào bạn gặp phải với tính năng này.

##### Hỗ trợ nền tảng
* Hỗ trợ ARM64 Windows!
* Hỗ trợ macOS gốc!
* Một nút bật/tắt toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Một nút định vị để tìm những cuốn sách bị thiếu vừa thay đổi đường dẫn.
* Một bộ lọc trạng thái và thanh trạng thái, để bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu đang được hiển thị và chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và Khả năng đọc
* Một tab khả năng đọc, với các tùy chọn sau:
    * Ngắt dòng (chuyển từ tab tổng quát sang);
    * Hiển thị bảng nội dòng (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Giãn dòng;
    * Giãn đoạn;
    * Giãn chữ;
    * Căn lề văn bản.
* Một mục menu ngắt dòng và phím nóng tương ứng.
* Một nút bật/tắt để quyết định bạn muốn bảng được hiển thị như thế nào, và thống nhất cách hiển thị bảng giữa các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo vùng chứa.
* Một tùy chọn để tự động di chuyển con trỏ về đầu dòng khi điều hướng giữa các dòng, tương tự chế độ duyệt trong trình đọc màn hình.
* Phím tắt dấu bằng để thông báo phần trăm vị trí hiện tại của bạn trong tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một dấu trang cho mỗi tài liệu, và chúng vẫn được lưu lại. Dùng dấu gạch chéo để đặt và dấu gạch chéo ngược để nhảy đến nó.

##### Đếm từ
* Thời gian đọc ước tính trong hộp thoại đếm từ, cùng với khả năng đặt tốc độ đọc của bạn để chỉ số này thực sự hữu ích.
* Nếu có một vùng chọn đang hoạt động khi bạn mở hộp thoại đếm từ, số từ bạn đã chọn giờ sẽ được hiển thị.

##### Phím tắt
* Khả năng tùy chỉnh mọi phím tắt trong ứng dụng thông qua một hộp thoại đơn giản.
* Một phím tắt có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Tiếng Hà Lan, tiếng Phần Lan và tiếng Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần.

##### Bộ cập nhật
* Một nút hủy trong hộp thoại đang cập nhật.
* Bộ cập nhật giờ xác thực rằng tệp đã tải về không bị can thiệp.

##### Chế độ xem web
* Chế độ xem web giờ được mở tại vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ sách DAISY 2.0.
* Hỗ trợ phát âm thanh DAISY 2.02.

##### Sách nói
* Khả năng phát sách nói, hiện hỗ trợ cả âm thanh DAISY (bao gồm DAISY âm thanh + văn bản) và các tệp zip chứa tệp âm thanh.
* Phím tắt và mục menu để phát/tạm dừng thuyết minh, tua tiến và lùi, và điều chỉnh mức tua.
* Tùy chọn để đồng bộ con trỏ đọc với phát âm thanh, đặt mức tua âm thanh, và chọn xem việc tua quá cuối một chương có tiếp tục sang chương sau hay không.

##### Tài liệu CHM
* Hỗ trợ danh sách, mục danh sách, hình minh họa và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint giờ hỗ trợ bảng.

#### Đã sửa

##### Tổng quát
* Các tài liệu được mã hóa bằng những bảng mã CJK cũ, như GBK, Big5 và Shift_JIS, giờ sẽ hiển thị đúng thay vì thành một loạt ký tự lỗi.
* "Mở lại tài liệu vừa đóng" cố mở lại tệp readme kèm theo.
* Tab bạn đã chọn không được đặt tiêu điểm đúng cách sau khi khởi động lại Paperback.
* Cách Paperback xử lý tệp trên ổ đĩa mạng Windows: nhấn hiện tệp trong thư mục giờ đặt tiêu điểm đúng vào tệp trên bộ lưu trữ mạng, và đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn bị buộc tải khi khôi phục tài liệu; thay vào đó, bạn sẽ được hỏi xác nhận khi tìm thấy một tệp như vậy.
* Mở thư mục chứa giờ đặt tiêu điểm vào tệp đó trong explorer.
* Mở tệp readme giờ sẽ tôn trọng ngôn ngữ bạn đã chọn.
* Giao diện người dùng của Paperback giờ sẽ được điều chỉnh tỷ lệ đúng trên màn hình DPI cao.
* Menu giờ cập nhật đúng cách, và tiêu điểm chuyển đến vùng điều khiển văn bản, khi mở trợ giúp trong Paperback.
* Chuyển sang một phương thức IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu đang hoạt động giờ sẽ được đọc khi chuyển giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước các bảng chỉ mục nội bộ theo từng ký tự.

##### Hộp thoại Tất cả tài liệu
* Phím Escape không đóng được hộp thoại Thông tin tài liệu và Tất cả tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng một tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở qua Shift+F1.
* Xóa tài liệu khỏi hộp thoại tài liệu gần đây giờ cũng sẽ đóng tab đang hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn giờ được giữ lại sau khi xóa một tài liệu.

##### Điều hướng
* Điều hướng theo trang thông báo sai văn bản dòng trong một số trường hợp.
* Đi đến dòng, Đi đến trang và Đi đến phần trăm đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Tìm và Tìm tiếp không tôn trọng cửa sổ tài liệu đã tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh dấu trang/ghi chú giờ sẽ chỉ phát đúng khi bạn di chuyển qua một từ có chứa chúng.

##### Khả năng đọc
* Áp dụng ngắt dòng đưa bạn về đầu tài liệu.

##### Chế độ xem web
* Hộp thoại chế độ xem web không thể thay đổi kích thước và hiện ra với kích thước ban đầu rất nhỏ.
* Hình ảnh giờ sẽ hiển thị đúng trong chế độ xem web nhúng.

##### Bộ cập nhật
* Bộ cập nhật giờ hiển thị đúng nội dung của các thẻ code markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin sai trên thanh trạng thái.
* Tải sách DAISY có khai báo bảng mã không hợp lệ.

##### Tài liệu RTF
* Phân tích các tài liệu RTF có ký tự không thuộc hệ Latin trong đó.
* Nhóm `\pict` của RTF nên dữ liệu hình ảnh nhúng không còn lọt vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các neo filepos trong sách Mobi làm chia tách thẻ HTML và đưa rác vào văn bản sách.
* Liên kết trong các sách Mobi cũ.
* Cải thiện đáng kể việc phân tích AZW3.

##### Tài liệu Word
* Tài liệu Word có tên kiểu (style) theo ngôn ngữ vùng không hiển thị đúng các tiêu đề.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback giờ chuyển sang trích xuất văn bản thuần cho các PDF được gắn thẻ sai.
* Tài liệu PDF chứa ký tự điều khiển trong tiêu đề và/hoặc dấu trang sẽ không còn làm Paperback bị lỗi khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang cho sách epub.
* Đã thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện hỗ trợ Word cũ, Word hiện đại và Powerpoint hiện đại, với Powerpoint cũ được dự kiến trong tương lai.
* Đã thêm hỗ trợ cho tài liệu Microsoft Word cũ (*.doc)!
* Đã thêm hỗ trợ cho bài trình bày Powerpoint cũ (*.ppt)!
* Đã thêm hỗ trợ cho sách mobi và AZW3!
* Đã thêm hỗ trợ cho tệp PDF được gắn thẻ!
* Đã thêm phím tắt ctrl+q để thoát ứng dụng.
* Đã thêm hỗ trợ cho sách nén zip từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng giờ sẽ được hiển thị đúng.
* Tài liệu CHM giờ hỗ trợ đúng việc điều hướng liên kết nội bộ.
* Đã sửa lỗi âm thanh dấu trang phát ở đầu đoạn thay vì tại vị trí của dấu trang.
* Đã sửa lỗi đi đến trang bị lệch 1.
* Đã sửa lỗi phím escape không hoạt động để đóng hộp thoại mở dưới dạng.
* Đã sửa lỗi menu ngữ cảnh của trình đọc không hiện ra khi nhấn chuột phải hoặc phím Applications.
* Đã sửa lỗi đôi khi sai tài liệu được đặt tiêu điểm khi mở tài liệu từ dòng lệnh.
* Các PDF chỉ có hình ảnh lại được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Giờ đã có thể điều hướng qua hình ảnh và hình minh họa lần lượt bằng g/shift+g và f/shift+f.
* Paperback giờ sẽ tôn trọng thiết lập chế độ tối của ứng dụng của bạn.
* Đã loại bỏ hỗ trợ DAISY XML, vì nó không còn cần thiết.
* Đã chuyển trở lại điều hướng theo chữ cái đầu gốc của Win32 trong cây mục lục.
* Hộp thoại lỗi khi tải giờ hiển thị thông báo lỗi chi tiết hơn.
* Chế độ xem web giờ sẽ mở nhanh và mượt hơn nhiều.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang cho tài liệu RTF!
* Đã sửa lỗi khi mở chế độ xem web trong các epub chứa liên kết ngoài sẽ tự động kích hoạt chúng.
* Đã sửa lỗi trình phân tích RTF không chèn khoảng trắng giữa các từ trong một số trường hợp hiếm.
* Đã sửa lỗi các đoạn bị chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF giờ đã có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Các tab và ký tự xuống dòng của RTF giờ được hiển thị đúng như trong tài liệu.
* Đã chuyển trở lại thư viện pdfium đã được kiểm chứng để phân tích PDF, làm việc hiển thị PDF đáng tin cậy hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Đã thêm Ctrl+Shift+T để mở lại tài liệu vừa đóng.
* Hộp thoại Tất cả tài liệu giờ hỗ trợ chọn nhiều tài liệu để mở cùng lúc.
* Đã sửa một vài lỗi với trình phân tích RTF.
* Đã sửa lỗi các đường dẫn tệp chứa ký tự không thuộc ASCII (như š, č, ć, ž của tiếng Bosnia) bị hỏng khi mở tệp qua một phiên bản Paperback thứ hai.
* Đã sửa lỗi văn bản PDF được đọc sai thứ tự, và khoảng trắng không đúng quanh các từ viết hoa.
* Đã sửa lỗi tải tài liệu chậm khi mở các tệp lớn.
* Đã sửa việc bản địa hóa các nút Có/Không trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Đã thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt!
* Đã thêm bộ cập nhật tự động giờ sẽ thay thế phiên bản Paperback bạn đang cài đặt thay vì chỉ tải về phiên bản mới!
* Đã thêm phản hồi âm thanh tùy chọn khi đến một dấu trang hoặc ghi chú, cảm ơn Andre Louis vì các âm thanh này!
* Đã thêm hỗ trợ tài liệu RTF!
* Đã thêm hỗ trợ cho tài liệu DAISY XML.
* Đã thêm hỗ trợ cho tệp Flat Open Document Text!
* Đã thêm hỗ trợ cho bài trình bày Flat Open Document!
* Đã thêm hỗ trợ điều hướng theo dấu phân cách với s và shift+s.
* Bất kỳ di chuyển nào lớn hơn 300 ký tự giờ sẽ tự động được thêm vào lịch sử điều hướng của bạn.
* Đã sửa việc khôi phục cửa sổ Paperback từ khay hệ thống.
* Đã sửa lỗi tài liệu Markdown hiển thị văn bản thô thay vì HTML đã kết xuất trong Chế độ xem web.
* Đã sửa lỗi bảng không hiển thị đúng trong tệp Markdown.
* Các PDF chỉ có hình ảnh giờ sẽ cảnh báo bạn về sự tồn tại của chúng khi bạn cố tải một tệp như vậy.
* Giờ đã có thể kiểm tra các bản dựng dev mới thay vì các bản phát hành ổn định khi kiểm tra cập nhật.
* Nhúng thông tin phiên bản đúng cách vào tệp thực thi Paperback.
* Đã chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Đã chuyển sang Hayro để phân tích PDF, dẫn đến độ tin cậy, tốc độ cao hơn và ít DLL hơn.
* Đã viết lại toàn bộ ứng dụng bằng Rust. Cơ sở mã mới an toàn hơn, tải tài liệu nhanh hơn, và dễ bảo trì cũng như mở rộng hơn.
* Menu ngữ cảnh của vùng điều khiển văn bản giờ sẽ bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Đã thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem một bảng trong chế độ xem web.
* Đã thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần hiện tại của tài liệu trong một bộ kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Đã thêm nút Xóa tất cả vào hộp thoại Tất cả tài liệu.
* Bộ kiểm tra cập nhật giờ hiển thị ghi chú phát hành khi có phiên bản mới.
* Đã sửa việc khôi phục cửa sổ từ khay hệ thống.
* Đã sửa bản dịch nút Có/Không trong hộp thoại xác nhận.
* Đã sửa việc tải cấu hình khi chạy với quyền quản trị viên.
* Đã sửa việc xử lý chú thích trong tài liệu XML và HTML.
* Đã sửa việc phân tích mục lục trong sách Epub 2.
* Đã sửa việc điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Đã sửa lỗi hộp thoại tìm kiếm không ẩn đúng cách khi dùng các nút tiếp/trước.
* Đã sửa lỗi mục lục epub đôi khi đưa bạn đến mục sai.
* Đã sửa nhiều vấn đề xử lý khoảng trắng trong XML, HTML và các thẻ pre.
* Đã sửa lỗi lệch một đơn vị trong điều hướng liên kết.
* Đã sửa lỗi một số sách có khoảng trắng ở cuối dòng.
* Đã sửa nhiều vấn đề của trình phân tích.
* Các mục menu liên quan đến dấu trang cũng như danh sách phần tử giờ được vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Đã cải thiện việc xử lý danh sách trong nhiều định dạng tài liệu.
* Đã cải thiện quy trình dịch cho những người đóng góp.
* Nhiều tái cấu trúc nội bộ, chuyển phần lớn logic nghiệp vụ của ứng dụng từ C++ sang Rust để cải thiện hiệu năng và khả năng bảo trì.

### Phiên bản 0.6.1
* Đã thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Đã thêm tính năng đi đến vị trí trước/sau rất cơ bản. Nếu bạn nhấn enter trên một liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó giờ sẽ được ghi nhớ, và có thể điều hướng đến bằng alt+mũi tên trái/phải.
* Đã thêm danh sách phần tử! Hiện nó chỉ hiển thị một cây tất cả các tiêu đề trong tài liệu của bạn hoặc một danh sách liên kết, nhưng có kế hoạch mở rộng trong tương lai.
* Đã thêm tùy chọn để khởi động Paperback ở chế độ cực đại theo mặc định.
* Đã sửa lỗi liên kết trong một số tài liệu Epub không hoạt động đúng.
* Đã sửa việc phân tích mục lục Epub chứa đường dẫn tương đối.
* Đã sửa lỗi một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Đã sửa lỗi tiêu đề của một số chương epub không hiển thị đúng trong hộp thoại mục lục.
* Đã sửa lỗi bạn không thể dùng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại mục lục.
* Đã cải thiện việc xử lý tiêu đề trong tài liệu Word.
* Bạn giờ sẽ nhận được phản hồi bằng giọng nói nếu danh sách tài liệu gần đây trống khi bạn cố mở hộp thoại.

### Phiên bản 0.6.0
* Một tùy chọn mới để hiển thị menu đi đến ở dạng gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được chọn theo mặc định.
* Đã thêm tùy chọn để việc điều hướng theo phần tử cấu trúc lặp vòng.
* Đã thêm một mục vào menu công cụ để mở thư mục chứa tài liệu đang được tiêu điểm.
* Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng hẹn giờ ngủ cơ bản, truy cập bằng Ctrl+Shift+S.
* Đã thêm hỗ trợ phân tích sách điện tử FB2!
* Đã thêm hỗ trợ phân tích bài trình bày OpenDocument!
* Đã thêm hỗ trợ phân tích tệp OpenDocument Text!
* Dấu trang giờ có thể được tạo để đánh dấu cả một dòng, hoặc chỉ đánh dấu một đoạn văn bản cụ thể. Nếu bạn không có vùng chọn nào khi đặt dấu trang, hành vi sẽ giống như trước 0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một đoạn văn bản, chỉ văn bản đó sẽ được đưa vào dấu trang.
* Dấu trang giờ có thể có ghi chú văn bản tùy chọn kèm theo! Điều hướng giữa các dấu trang có ghi chú bằng N và Shift+N, hoặc mở hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú, hoặc chỉ những dấu trang không có ghi chú bằng các phím nóng cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn tiền tố "dấu trang x" gây khó chịu.
* Sách Epub chứa nội dung HTML giả làm XML giờ sẽ được xử lý đúng cách.
* Đã sửa việc tải tài liệu Markdown lớn.
* Đã sửa lỗi nhấn thanh cách trong cây mục lục kích hoạt nút OK.
* Đã sửa việc xử lý khoảng trắng ở đầu các thẻ pre trong cả tài liệu HTML và XHTML.
* Đã sửa lỗi vùng điều khiển văn bản đôi khi không lấy lại tiêu điểm khi trở về cửa sổ Paperback.
* Đã sửa lỗi trường văn bản trong hộp thoại đi đến phần trăm không cập nhật giá trị của thanh trượt.
* Đã sửa việc hiển thị các ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong các khối mã Markdown giờ sẽ được kết xuất đúng cách.
* Nếu tải một cuốn sách bằng tham số dòng lệnh khi đã có một phiên bản Paperback đang chạy, bạn sẽ không còn nhận được lỗi nếu việc tải tài liệu mất hơn 5 giây.
* Nếu chạy Paperback với quyền quản trị viên, cấu hình giờ sẽ được tải và lưu đúng cách.
* Giờ đã có thể xóa một dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Giờ đã có thể nhập và xuất dấu trang cùng vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo ra được đặt tên theo tệp gốc với phần mở rộng .paperback. Nếu tìm thấy một tệp như vậy trong cùng thư mục với tệp khi tải, nó sẽ được tự động tải. Nếu không, bạn có thể nhập thủ công bằng một mục trong menu công cụ.
* Liên kết bên trong tài liệu giờ đã được hỗ trợ đầy đủ! Dùng k và shift+k để di chuyển tiến và lùi qua chúng, và nhấn enter để mở/kích hoạt một liên kết.
* Nhiều tái cấu trúc nội bộ, làm ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown giờ được tiền xử lý để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và mục danh sách giờ đã được hỗ trợ đầy đủ! Dùng L và Shift+L để đi theo chính các danh sách, và I và Shift+I để đi qua các mục danh sách.
* Phím delete trên bàn phím số giờ cũng hoạt động để xóa tài liệu khỏi thanh tab, ngoài phím delete thường.
* Paperback giờ có thể tùy chọn thu nhỏ xuống khay hệ thống của bạn! Tùy chọn này tắt theo mặc định, nhưng khi bật lên sẽ làm cho tùy chọn thu nhỏ trong menu hệ thống đưa Paperback vào khay, có thể khôi phục bằng cách nhấn vào biểu tượng được tạo ra.
* Paperback giờ đã có thể dịch hoàn toàn! Danh sách ngôn ngữ nó hỗ trợ hiện còn khá nhỏ, nhưng đang liên tục tăng lên!
* Paperback giờ đã có website chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX giờ sẽ hiển thị một mục lục cơ bản, chứa tất cả các slide.
* Đường dẫn đầy đủ đến tài liệu đang mở giờ sẽ được hiển thị trong hộp thoại thông tin tài liệu.
* Bộ cài đặt giờ bao gồm một tùy chọn để xem tệp readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ hiển thị 10 tài liệu bạn mở gần nhất, giờ nó sẽ hiển thị một số lượng có thể tùy chỉnh, còn tất cả các tài liệu khác bạn từng mở có thể truy cập qua một hộp thoại nhỏ.
* Nhiều cải thiện nhỏ cho các trình phân tích trên mọi phương diện, bao gồm chèn một dòng trống giữa các slide trong bài trình bày PPTX, sửa việc xử lý ký tự xuống dòng bên trong các đoạn trong tài liệu word, và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho bài trình bày PowerPoint!
* Đã sửa lỗi một số mục menu không bị vô hiệu hóa khi không có tài liệu nào được mở.
* Đã sửa hướng của thanh trượt đi đến phần trăm.
* Đã sửa mục lục trong sách Epub có đường dẫn tệp và/hoặc ID phân đoạn được mã hóa URL.
* Đã sửa lỗi khoảng trắng bị loại bỏ khỏi các tiêu đề XHTML theo những cách kỳ lạ.
* Đã sửa việc xử lý khoảng trắng bên trong các thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown giờ hỗ trợ tính năng mục lục! Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ tự xây dựng mục lục riêng từ cấu trúc các tiêu đề trong tài liệu của bạn, và sẽ hiển thị nó trong hộp thoại ctrl+t.
* Tài liệu HTML giờ sẽ có tiêu đề như được đặt trong thẻ title, nếu có. Nếu không, chúng sẽ tiếp tục dùng tên tệp không có phần mở rộng.
* Đã chuyển từ UniversalSpeech sang dùng vùng trực tiếp (live region) để thông báo giọng nói. Điều này có nghĩa là không còn DLL của trình đọc màn hình được đi kèm với chương trình, và giờ nhiều trình đọc màn hình hơn sẽ được hỗ trợ, như Microsoft Narrator.
* Đã đổi thư viện zip để cho phép mở nhiều loại sách epub hơn.
* Hộp thoại hỏi bạn có muốn mở tài liệu dưới dạng văn bản thuần đã được làm lại hoàn toàn, và giờ nó cho phép bạn mở tài liệu dưới dạng văn bản thuần, HTML hoặc Markdown.
* Hộp thoại đi đến phần trăm giờ bao gồm một trường văn bản cho phép bạn nhập thủ công phần trăm muốn nhảy tới.
* Trình phân tích HTML giờ sẽ nhận dd, dt và dl là các phần tử danh sách.
* Mục lục trong sách Epub một lần nữa sẽ được giữ nguyên chính xác.
* Ký tự khoảng trắng không ngắt unicode giờ được xem xét khi loại bỏ các dòng trống.
* Bạn sẽ không còn bị hỏi cách mở một tệp không được nhận dạng mỗi lần tải nó, chỉ ở lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng menu bắt đầu tùy chọn vào bộ cài đặt.
* Mục lục giờ sẽ gọn gàng hơn trong một vài trường hợp, ví dụ nếu bạn có một mục con và mục cha với cùng văn bản ở cùng vị trí, giờ bạn sẽ chỉ thấy mục cha.
* Đã sửa mục lục trong một số tài liệu CHM.
* Đã sửa mục lục trong sách Epub 3 có đường dẫn tuyệt đối.
* Tài liệu CHM giờ sẽ hiển thị tiêu đề như được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang trong bao nhiêu tài liệu tùy ý. Bạn có thể nhảy tiến và lùi qua chúng bằng b và shift+b, đặt một dấu trang bằng control+shift+b, và mở hộp thoại để nhảy đến một dấu trang cụ thể bằng control+b.
* Đã thêm một bộ cài đặt cùng với tệp zip di động! Bộ cài đặt sẽ cài Paperback vào thư mục Program Files của bạn, và tự động thiết lập liên kết tệp cho bạn.
* Tệp văn bản có BOM giờ sẽ được giải mã đúng cách, và BOM cũng sẽ không còn được hiển thị ở đầu văn bản.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Giờ nó sẽ hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Chú thích HTML, cũng như nội dung của các thẻ script và style, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu truyền một đường dẫn tương đối cho Paperback trên dòng lệnh, giờ nó sẽ phân giải đúng cách.
* Việc di chuyển theo phần trăm giờ được xử lý bởi hộp thoại dựa trên thanh trượt riêng, truy cập bằng control+shift+g.
* Tài liệu không có tiêu đề hoặc tác giả đã biết giờ sẽ luôn có giá trị mặc định.
* Logic lưu vị trí giờ thông minh hơn nhiều và chỉ ghi vào đĩa khi thực sự cần thiết.
* Tài liệu bạn đang tiêu điểm khi đóng Paperback giờ được ghi nhớ giữa các lần khởi động lại ứng dụng.
* Dữ liệu nhập vào các hộp thoại đi đến dòng và đi đến trang giờ sẽ được kiểm tra chặt chẽ hơn.
* Đã sửa việc điều hướng mục lục trong sách epub 3 có đường dẫn tương đối trong manifest.

### Phiên bản 0.3.0
* Đã sửa mục lục trong sách epub có manifest được mã hóa URL.
* Đã sửa việc điều hướng tiêu đề trong tài liệu HTML chứa ký tự Unicode nhiều byte.
* Đã sửa việc sử dụng CPU cao trong tài liệu có tiêu đề dài do một lỗi hồi quy trong wxWidgets.
* Đã sửa việc tải tệp văn bản UTF-8.
* Đã sửa lỗi các mục mục lục lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Đã sửa một lỗi treo khi thoát ứng dụng trong một số trường hợp.
* Đã thêm một hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt ngắt dòng!
* Giờ đã có thể quyên góp cho việc phát triển Paperback, thông qua mục quyên góp mới trong menu trợ giúp hoặc qua liên kết tài trợ dự án này ở cuối trang chính của kho GitHub.
* Tài liệu Markdown giờ sẽ luôn có tiêu đề, và Paperback giờ có thể tải hầu như bất kỳ tệp Markdown nào.
* Tài liệu PDF giờ sẽ luôn có tiêu đề, ngay cả khi thiếu siêu dữ liệu.
* Đã chuyển sang thư viện PDF được dùng trong Chromium, dẫn đến việc phân tích PDF đáng tin cậy hơn nhiều trên mọi phương diện.
* Bạn giờ chỉ có thể chạy một phiên bản Paperback tại một thời điểm. Chạy paperback.exe với một tên tệp khi nó đang chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bạn giờ có thể nhấn delete trên một tài liệu trong vùng điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại đi đến trang.
* Cho phép dùng tab từ nội dung tài liệu sang danh sách các tài liệu đang mở.
* Đã sửa lỗi các phím tắt tiêu đề đôi khi mở tài liệu gần đây nếu bạn có đủ nhiều tài liệu.
* Paperback giờ sẽ loại bỏ các dấu gạch nối mềm không cần thiết khỏi đầu ra văn bản.
* Đã sửa lỗi điều hướng tiêu đề đôi khi đưa bạn đến ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các phím tắt này được thiết kế để hoạt động tương tự một trình đọc màn hình.
* Đã sửa việc tải các epub có tên tệp được mã hóa URL trong manifest.
* Đã sửa việc tải sách epub 3 có XHTML nhúng bên trong.
* Một thông báo giờ sẽ được đọc nếu tài liệu không hỗ trợ mục lục hoặc các phần, thay vì các mục menu bị vô hiệu hóa.
* Đã thêm menu tài liệu gần đây! Hiện nó lưu 10 tài liệu bạn mở gần nhất, và nhấn enter trên một tài liệu sẽ mở nó để đọc.
* Đã viết lại hoàn toàn hộp thoại Tìm, làm nó đơn giản hơn nhiều để sử dụng, đồng thời thêm lịch sử 25 lần tìm kiếm gần nhất và hỗ trợ biểu thức chính quy!
* Các tài liệu đã mở trước đó giờ được ghi nhớ giữa các lần khởi động lại ứng dụng. Điều này có thể cấu hình qua mục tùy chọn mới trong menu công cụ.
* Đã thêm shift+f1 để mở tệp readme trực tiếp trong chính Paperback.

### Phiên bản 0.1.0
* Bản phát hành đầu tiên.
