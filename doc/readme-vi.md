<!-- machine-translated from doc/readme.md (source-hash: d49e7044d9856698); please review and edit as needed -->

# Paperback - phiên bản 0.9.1

## Giới thiệu

Paperback là một ứng dụng đọc ebook và tài liệu nhẹ, nhanh và dễ tiếp cận cho mọi người, từ những độc giả thường xuyên đến những người dùng nặng. Nó được thiết kế để có khả năng tiếp cận với trình đọc màn hình, tốc độ nhanh và trải nghiệm không đầy đủ.

## Yêu cầu hệ thống

Paperback hiện chạy trên Windows 10/11 và tất cả các phiên bản hiện đại của ARM macOS. Các ứng dụng iOS và Android gốc đang được phát triển tích cực, với các bản dựng thử nghiệm công khai được dự kiến sẽ sớm sau bản phát hành máy tính để bàn 0.9.0, trước khi phát hành thống nhất 1.0 bao gồm cả bốn nền tảng.

## Tính năng

* Hoàn toàn độc lập, không cần bất kỳ phần mềm nào được cài đặt trên máy tính của bạn để bắt đầu đọc.
* Cực kỳ nhanh, ngay cả trên phần cứng cũ.
* Giao diện tab đơn giản, cho phép bạn mở nhiều tài liệu như bạn muốn cạnh nhau.
* Lưu lại vị trí đọc chính xác của bạn trên mọi tài liệu bạn mở.
* Tùy chọn ghi nhớ những tài liệu nào bạn đã mở khi đóng chương trình và khôi phục chúng khi khởi động lại.
* Bao gồm chức năng điều hướng tương tự như chức năng được tìm thấy trong chế độ duyệt web của nhiều trình đọc màn hình để nhanh chóng và dễ dàng điều hướng qua các tài liệu.
* Bao gồm một hộp thoại tìm kiếm mạnh mẽ, bao gồm các tính năng như lịch sử và hỗ trợ biểu thức chính quy.
* Có thể chạy hoàn toàn theo cách di động hoặc được cài đặt với các liên kết tệp được thiết lập tự động.
* Hỗ trợ một loạt các định dạng tệp phổ biến.

## Khả năng tương thích với trình đọc màn hình

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy nhiên, có một vấn đề được biết đến cho người dùng JAWS.

### JAWS và Hiển thị Braille

Nếu bạn sử dụng JAWS với hiển thị Braille, bạn có thể thấy rằng các đoạn văn dài bị cắt ngắn khi pan về phía trước với các phím điều hướng của hiển thị của bạn. Lệnh đọc đoạn hiện tại cũng bị ảnh hưởng. Đây là một lỗi trong cách xử lý kiểm soát văn bản RICHEDIT50W của JAWS, chứ không phải điều gì đó trong Paperback, và một lỗi mất khá lâu để tìm ra cách khắc phục vì sự nhiệt tình của Vispero đối với việc phản hồi các vấn đề với phần mềm nguồn mở.

Cách giải quyết, cuối cùng được phát hiện thông qua nhóm thảo luận JAWS sau những tháng chờ đợi, là chỉnh sửa `paperback.jcf` và đặt "Braille Presentation and Panning" thành "Always use DOM if available". Bạn cũng sẽ muốn bật "Pan Text by Paragraph", nếu không hiển thị của bạn sẽ ở trên đoạn hoạt động hơn là tiến lên. Với cả hai cài đặt, pan sẽ hoạt động đúng.

## Các loại tệp được hỗ trợ hiện tại

Paperback hỗ trợ các định dạng và tiện ích mở rộng sau:

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
* Tệp văn bản và nhật ký thuần túy (`.txt`, `.log`)

## Phím tắt bàn phím

Paperback được thiết kế cho việc sử dụng ưu tiên bàn phím. Dưới đây là các phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Khi macOS khác, phím tương đương được ghi chú trong dấu ngoặc đơn — chủ yếu vì Ctrl+G, Ctrl+W, và Alt+Left/Right đã được sử dụng bởi các quy ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu File

* `Ctrl+O`: Mở một tài liệu.
* `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
* `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu đang mở.
* `Ctrl+Shift+T`: Mở lại tài liệu đóng cuối cùng.
* `Ctrl+R`: Hiển thị hộp thoại "All Documents" (từ Recent Documents).
* `Ctrl+Q`: Thoát (Chỉ Windows; trên macOS, điều này nằm trong menu ứng dụng).

### Menu Go

* `Ctrl+F`: Hiển thị hộp thoại Find.
* `F3` (macOS: `Cmd+G`): Tìm tiếp theo.
* `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
* `Ctrl+G` (macOS: `Cmd+L`): Đi tới dòng.
* `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Đi tới phần trăm.
* `Ctrl+P`: Đi tới trang (khi được hỗ trợ bởi tài liệu hiện tại).
* `=`: Công bố tỷ lệ phần trăm đọc hiện tại của bạn.
* `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
* `Alt+Right` (macOS: `Cmd+]`): Đi tiếp trong lịch sử điều hướng.
* `[`: Phần trước đó.
* `]`: Phần tiếp theo.
* `Shift+H`: Tiêu đề trước đó.
* `H`: Tiêu đề tiếp theo.
* `Shift+1` đến `Shift+6`: Tiêu đề trước đó ở cấp 1-6.
* `1` đến `6`: Tiêu đề tiếp theo ở cấp 1-6.
* `Shift+P`: Trang trước đó.
* `P`: Trang tiếp theo.
* `Shift+B`: Dấu trang trước đó.
* `B`: Dấu trang tiếp theo.
* `/`: Đặt dấu trang tạm thời của bạn.
* `\`: Nhảy tới dấu trang tạm thời của bạn.
* `Shift+N`: Ghi chú trước đó.
* `N`: Ghi chú tiếp theo.
* `Ctrl+B`: Nhảy tới tất cả dấu trang và ghi chú.
* `Ctrl+Alt+B`: Nhảy tới dấu trang chỉ.
* `Ctrl+Alt+M`: Nhảy tới ghi chú chỉ.
* `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý chứ không phải Cmd): Xem văn bản ghi chú tại vị trí hiện tại.
* `Shift+K`: Liên kết trước đó.
* `K`: Liên kết tiếp theo.
* `Shift+G`: Hình ảnh trước đó.
* `G`: Hình ảnh tiếp theo.
* `Shift+F`: Hình tiếp theo trước đó.
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
* `,`: Đi quá cuối vùng chứa hiện tại (danh sách hoặc bảng).

### Menu Tools

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
* `Ctrl+Shift+B`: Bật/tắt dấu trang tại vị trí/con trỏ hiện tại.
* `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang tại vị trí/con trỏ hiện tại.
* `Ctrl+Alt+W`: Bật/tắt gói lời.
* `Ctrl+Space`: Phát/tạm dừng kể chuyện âm thanh.
* `'`: Tìm kiếm kể chuyện âm thanh tiến lên.
* `;`: Tìm kiếm kể chuyện âm thanh lùi lại.
* `Ctrl+'`: Tăng lượng tìm kiếm âm thanh.
* `Ctrl+;`: Giảm lượng tìm kiếm âm thanh.
* `F11` (macOS: `RawCtrl+Ctrl+F`, tức là Control+Command+F): Bật/tắt toàn màn hình.
* `Ctrl+,`: Mở tùy chọn (macOS: Preferences, trong menu ứng dụng).
* `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.

### Menu Help

* `Ctrl+F1`: Hiển thị hộp thoại About.
* `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
* `Shift+F1`: Xem trợ giúp trong Paperback.
* `Ctrl+Shift+U`: Kiểm tra cập nhật.
* `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím xem tài liệu bổ sung

* `Delete` / `Numpad Delete` trên điều khiển tab: Đóng tab tài liệu được chọn.
* `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết tại con trỏ, hoặc mở chế độ xem bảng khi ở trên dấu bảng.
* `Shift+F10` hoặc phím Menu/Application trong văn bản tài liệu: Mở menu ngữ cảnh.

## Ngôn ngữ được hỗ trợ

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

## Lời cảm ơn
### Phát triển
* Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
* Aryan Choudhary: nhà đóng góp chính.

### Quyên góp
Các cá nhân sau đây đã quyên góp một số tiền cho sự phát triển Paperback. Nếu bạn thực hiện quyên góp, tên của bạn sẽ không được tự động thêm vào đây, tôi chỉ thêm những người muốn quyên góp của họ được công khai.

Lưu ý: Tôi coi nhà tài trợ GitHub công khai là lý do tự động đưa vào danh sách này.

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

### Phiên bản 0.9.1
* Âm thanh đánh dấu trang và ghi chú hiện đã phát được trên macOS.
* Sách DAISY hiện phát âm thanh của chúng trên macOS, thay vì mở và theo dõi dòng thời gian của chúng im lặng.
* Sửa lỗi dấu ngoặc kép, dấu gạch dài và các ký tự tương tự biến mất khỏi tài liệu RTF, khiến các từ xung quanh chạy lại với nhau.
* Sửa lỗi hình ảnh RTF rò rỉ dữ liệu thô của chúng vào tài liệu dưới dạng văn bản lỗi.
* Sửa lỗi trình đơn Tài liệu gần đây giữ lại các mục cũ cho đến khi có điều gì khác xảy ra để xây dựng lại nó.
* Bộ tăng tốc bàn phím đã quay trở lại trong mọi bản dịch, vì vậy các menu của Nga lại có quyền truy cập bàn phím.
* Các tài liệu CHM lớn hiện mở nhanh hơn tới bảy lần.
* Các tài liệu đã mở hiện được đăng ký với Windows, vì vậy chúng sẽ xuất hiện trong danh sách nhảy thanh tác vụ và danh sách gần đây của menu Bắt đầu.
* Tùy chọn đã được đổi tên thành Cài đặt, phù hợp với các ứng dụng di động và, trên macOS, theo quy ước nền tảng.
* Paperback hiện nhớ vị trí, kích thước cửa sổ và trạng thái tối đa của nó giữa các lần chạy.
* Các dạng số nhiều hiện đã được dịch, vì vậy các thông báo đếm các thứ có thể được đọc chính xác trong các ngôn ngữ cần nhiều hơn một dạng.
* Chọn tệp ncc.html của sách DAISY hiện mở sách âm thanh hoàn chỉnh thay vì chỉ tText của nó.
* Tên hành động của hộp thoại Tùy chỉnh phím tắt bàn phím hiện có thể được dịch.
* Tiêu đề tài liệu hiện xuất hiện trước tiên trong thanh tiêu đề, vì vậy có thể phân biệt các sách đang mở trong thanh tác vụ và Alt+Tab.
* Hộp thoại cập nhật hiện đã được dịch.

### Phiên bản 0.9.0

#### Được thêm vào

##### Chung
* Một công cụ CLI được gọi là pb, để nhanh chóng chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ thành HTML, Markdown hoặc văn bản thuần túy.
* Một tùy chọn để tải lại các tài liệu đã được sửa đổi bởi các chương trình khác trên đĩa.
* Tùy chọn Xem nguồn để mở nguồn của tài liệu trong tab mới, hữu ích cho việc chỉnh sửa Markdown chẳng hạn.
* Văn bản tài liệu hiện được phân trang, có nghĩa là bạn có thể tải các cuốn sách có hàng chục triệu từ chỉ trong vài giây. Vui lòng báo cáo bất kỳ điều kỳ lạ nào được tìm thấy với nó.

##### Hỗ trợ nền tảng
* Hỗ trợ Windows ARM64!
* Hỗ trợ macOS bản địa!
* Bật tắt toàn màn hình.

##### Hộp thoại Tất cả tài liệu
* Nút định vị để định vị các sách bị mất mà vừa thay đổi đường dẫn của chúng.
* Bộ lọc trạng thái và thanh trạng thái, vì vậy bạn có thể lọc theo trạng thái tài liệu và xem có bao nhiêu tài liệu được hiển thị và được chọn.
* Phím tắt `Ctrl+Shift+A` để bỏ chọn tất cả tài liệu.

##### Tùy chọn và khả năng đọc
* Tab khả năng đọc, với các tùy chọn sau:
    * Ngắt dòng chữ (di chuyển từ chung);
    * Kết xuất bảng nội tuyến (mới trong bản phát hành này, xem bên dưới);
    * Phông chữ;
    * Màu nền;
    * Khoảng cách dòng;
    * Khoảng cách đoạn;
    * Khoảng cách chữ;
    * Căn chỉnh văn bản.
* Mục menu ngắt dòng chữ và phím tắt tiếp theo.
* Bật tắt để xác định cách bạn muốn bảng được hiển thị, và thống nhất cách bảng được hiển thị trên các tài liệu.

##### Điều hướng
* Hỗ trợ điều hướng theo thùng chứa.
* Tùy chọn để tự động di chuyển con trỏ đến đầu dòng khi điều hướng giữa các dòng, tương tự như chế độ duyệt trong trình đọc màn hình.
* Phím tắt bằng dấu bằng để công bố tỷ lệ phần trăm hiện tại của bạn trong một tài liệu.

##### Dấu trang
* Dấu trang tạm thời: bạn có thể có một dấu trang trên mỗi tài liệu, và chúng tồn tại lâu dài. Sử dụng dấu gạch chéo để đặt một dấu và dấu gạch ngược để nhảy đến nó.

##### Số từ
* Thời gian đọc ước tính trong hộp thoại số từ, cũng như khả năng đặt tốc độ đọc của bạn để làm cho số liệu này thực sự hữu ích.
* Nếu lựa chọn đang hoạt động khi bạn mở hộp thoại số từ, hiện sẽ hiển thị bao nhiêu từ bạn đã chọn.

##### Phím tắt bàn phím
* Khả năng tùy chỉnh mọi phím tắt bàn phím trong ứng dụng thông qua một hộp thoại đơn giản.
* Phím tắt bàn phím có thể cấu hình để khôi phục Paperback từ khay hệ thống.

##### Ngôn ngữ
* Hà Lan, Phần Lan và Ba Lan.

##### Xuất
* Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown, ngoài văn bản thuần túy.

##### Trình cập nhật
* Nút hủy vào hộp thoại cập nhật đang diễn ra.
* Trình cập nhật hiện xác thực tệp đã tải xuống chưa bị giả mạo.

##### Chế độ xem web
* Chế độ xem web hiện được mở ở vị trí đọc hiện tại của bạn.

##### Sách DAISY
* Hỗ trợ cho sách DAISY 2.0.
* Hỗ trợ phát lại âm thanh DAISY 2.02.

##### Sách âm thanh
* Khả năng phát sách âm thanh, hiện hỗ trợ cả DAISY âm thanh (bao gồm DAISY âm thanh + văn bản) và các tệp zip của tệp âm thanh.
* Phím tắt và mục menu để phát/tạm dừng lời thuyết minh, tìm kiếm phía trước và phía sau, và điều chỉnh lượng tìm kiếm.
* Tùy chọn để đồng bộ hóa dấu đọc với phát lại âm thanh, đặt lượng tìm kiếm âm thanh và chọn liệu tìm kiếm vượt quá phần cuối của chương có tiếp tục vào phần tiếp theo hay không.

##### Tài liệu CHM
* Hỗ trợ cho danh sách, mục danh sách, hình và hình ảnh.

##### PowerPoint
* Tài liệu PowerPoint hiện hỗ trợ bảng.

#### Sửa lỗi

##### Chung
* Các tài liệu được mã hóa trong các bộ mã CJK cũ, chẳng hạn như GBK, Big5 và Shift_JIS, sẽ hiển thị chính xác thay vì toàn bộ mojibake.
* "Mở lại lần cuối cùng" cố gắng mở lại readme đi kèm.
* Tab đã chọn của bạn không được lấy nét đúng cách sau khi khởi động lại Paperback.
* Xử lý các tệp trên ổ đĩa mạng Windows của Paperback: nhấn hiển thị tệp trong thư mục bây giờ đúng cách tập trung vào tệp trên bộ nhớ mạng, và các đường dẫn không còn chứa các ký tự lạ.
* Các tệp .paperback sẽ không còn được tải đúng cách khi khôi phục tài liệu; thay vào đó, bạn sẽ được yêu cầu xác nhận khi tìm thấy một.
* Mở thư mục chứa bây giờ tập trung vào tệp nhất định trong trình khám phá.
* Mở readme sẽ lúc này tôn trọng ngôn ngữ đã chọn của bạn.
* Giao diện người dùng Paperback sẽ lúc này tỷ lệ đúng cách trên màn hình độ phân giải cao.
* Trình đơn hiện đúng cách cập nhật, và tiêu điểm di chuyển đến điều khiển văn bản, khi mở trợ giúp trong Paperback.
* Chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
* Tiêu đề tài liệu hoạt động sẽ lúc này được đọc khi chuyển đổi giữa các tab.
* Giảm mức sử dụng bộ nhớ trên các tài liệu lớn bằng cách giảm một nửa kích thước của các bảng chỉ mục mỗi ký tự nội bộ.

##### Hộp thoại Tất cả tài liệu
* Escape không đóng hộp thoại Thông tin tài liệu và Tất cả tài liệu.
* Thanh tiêu đề không cập nhật sau khi đóng tài liệu từ hộp thoại tất cả tài liệu.
* Readme.html sẽ không còn được thêm vào danh sách tất cả tài liệu của bạn khi mở qua Shift+F1.
* Xóa tài liệu khỏi hộp thoại gần đây sẽ lúc này đóng tab hoạt động của chúng.
* Bộ lọc tìm kiếm của bạn hiện được bảo tồn sau khi xóa tài liệu.

##### Điều hướng
* Điều hướng trang thông báo văn bản dòng không chính xác trong một số tình huống.
* Đi đến Dòng, Đi đến Trang và Đi đến Phần trăm đặt con trỏ của bạn ở vị trí sai trong các tài liệu lớn.
* Tìm và Tìm tiếp theo không tôn trọng cửa sổ tài liệu được tải trong các tài liệu lớn.

##### Dấu trang
* Âm thanh đánh dấu trang/ghi chú hiện sẽ phát chính xác khi bạn điều hướng qua một từ chứa một từ.

##### Khả năng đọc
* Áp dụng ngắt dòng chữ bắn bạn đến đầu tài liệu của bạn.

##### Chế độ xem web
* Hộp thoại chế độ xem web không thay đổi kích thước và bật lên ở kích thước ban đầu rất nhỏ.
* Hình ảnh hiện sẽ hiển thị chính xác trong chế độ xem web nhúng.

##### Trình cập nhật
* Trình cập nhật hiện đúng cách hiển thị nội dung của thẻ mã markdown trong ghi chú phát hành.

##### Sách DAISY
* Sách DAISY hiển thị thông tin không chính xác trong thanh trạng thái.
* Tải sách DAISY với các khai báo mã hóa giả.

##### Tài liệu RTF
* Phân tích cú pháp tài liệu RTF với các ký tự không phải Latinh trong chúng.
* Các nhóm RTF `\pict` vì vậy dữ liệu hình ảnh nhúng không còn rò rỉ vào văn bản tài liệu.

##### Sách Mobi/AZW3
* Các mỏ neo filepos trong sách Mobi chia tách thẻ HTML và đặt rác trong văn bản sách.
* Liên kết trong sách Mobi cũ.
* Phân tích cú pháp AZW3 được cải thiện đáng kể.

##### Tài liệu Word
* Tài liệu Word có tên kiểu dùng theo ngôn ngữ không kết xuất tiêu đề của chúng chính xác.

##### Tài liệu HTML/XHTML
* Các phần tử dl, dt và dd không tạo ngắt dòng trong tài liệu XHTML.

##### Tài liệu PDF
* Paperback hiện quay trở lại trích xuất văn bản thuần túy cho các tệp PDF được gắn thẻ sai.
* Tài liệu PDF chứa các ký tự điều khiển trong tiêu đề và/hoặc dấu trang của chúng sẽ không còn làm Paperback bị sập khi mở.

### Phiên bản 0.8.5
* Đã thêm hỗ trợ trang cho sách epub.
* Đã thêm hỗ trợ cho tài liệu Microsoft Office được mã hóa. Hiện tại hỗ trợ word cũ, Word hiện đại và Powerpoint hiện đại, với Powerpoint cũ được lên kế hoạch cho tương lai.
* Đã thêm hỗ trợ cho tài liệu Microsoft Word cũ (*.doc)!
* Đã thêm hỗ trợ cho bản trình bày Powerpoint cũ (*.ppt)!
* Đã thêm hỗ trợ cho sách mobi và AZW3!
* Đã thêm hỗ trợ cho tệp PDF được gắn thẻ!
* Đã thêm phím tắt ctrl+q để thoát ứng dụng.
* Đã thêm hỗ trợ cho sách được nén từ Bookshare (cả DAISY và Word)!
* Văn bản thay thế cho hình ảnh nhúng hiện sẽ được hiển thị chính xác.
* Tài liệu CHM hiện đúng cách hỗ trợ điều hướng liên kết nội bộ.
* Sửa âm thanh đánh dấu trang kích hoạt ở phần đầu đoạn thay vì vị trí của dấu trang.
* Sửa trang đi bị tắt 1.
* Sửa phím thoát không hoạt động để đóng hộp thoại mở dưới dạng.
* Sửa trình đơn ngữ cảnh trình đọc không hiển thị trên nhấp chuột phải hoặc phím Ứng dụng.
* Sửa tài liệu sai đôi khi được lấy nét khi mở tài liệu từ dòng lệnh.
* Các tệp PDF chỉ có hình ảnh một lần nữa được phát hiện và cảnh báo bạn về sự tồn tại của chúng.
* Hiện có thể điều hướng qua hình ảnh và hình với g/shift+g và f/shift+f, tương ứng.
* Paperback hiện sẽ tôn trọng cài đặt chế độ tối của ứng dụng của bạn.
* Đã xóa hỗ trợ XML DAISY vì nó không còn cần thiết.
* Chuyển trở lại điều hướng chữ cái đầu tiên gốc Win32 trong chế độ xem cây mục lục.
* Hộp thoại lỗi tải hiện hiển thị các thông báo lỗi chi tiết hơn.
* Chế độ xem web hiện sẽ mở nhanh hơn và mượt mà hơn.

### Phiên bản 0.8.2
* Đã thêm hỗ trợ trang cho tài liệu RTF!
* Sửa lỗi khi mở chế độ xem web trong épub chứa liên kết bên ngoài sẽ tự động kích hoạt chúng.
* Sửa lỗi khi trình phân tích cú pháp RTF sẽ không đặt khoảng trắng giữa các từ trong các trường hợp hiếm hoi.
* Các đoạn được chia thành nhiều dòng ngắn trong một số tài liệu PDF.
* Tài liệu PDF hiện có hỗ trợ điều hướng liên kết và tiêu đề cơ bản!
* Tab RTF và nguồn cấp dữ liệu dòng hiện được kết xuất chính xác như chúng xuất hiện trong tài liệu.
* Chuyển trở lại thư viện pdfium được thử và kiểm tra để phân tích cú pháp PDF, giúp kết xuất PDF đáng tin cây hơn nhiều một lần nữa.

### Phiên bản 0.8.1
* Đã thêm Ctrl+Shift+T để mở lại tài liệu đóng lần cuối cùng.
* Hộp thoại Tất cả tài liệu hiện hỗ trợ chọn nhiều tài liệu để mở cùng một lúc.
* Sửa một vài lỗi với trình phân tích cú pháp RTF.
* Sửa đường dẫn tệp chứa các ký tự không phải ASCII (chẳng hạn như Bosnian š, č, ć, ž) bị hỏng khi mở tệp qua phiên bản thứ hai của Paperback.
* Sửa văn bản PDF được đọc theo thứ tự sai và khoảng cách không chính xác xung quanh các từ in hoa.
* Sửa tải tài liệu chậm khi mở các tệp lớn.
* Sửa bản địa hóa của các nút Có/Không trong hộp thoại xác nhận.

### Phiên bản 0.8.0
* Đã thêm bản dịch tiếng Nhật, tiếng Trung đơn giản và tiếng Việt!
* Đã thêm trình cập nhật tự động sẽ hiện thay thế phiên bản Paperback hiện được cài đặt thay vì chỉ tải phiên bản mới!
* Đã thêm phản hồi âm thanh tùy chọn để đạt được dấu trang hoặc ghi chú, cảm ơn Andre Louis vì các âm thanh!
* Đã thêm hỗ trợ tài liệu RTF!
* Đã thêm hỗ trợ cho tài liệu XML DAISY.
* Đã thêm hỗ trợ cho tệp Văn bản Tài liệu mở phẳng!
* Đã thêm hỗ trợ cho bản trình bày Tài liệu mở phẳng!
* Đã thêm hỗ trợ cho các dấu phân cách với s và shift+s.
* Bất kỳ chuyển động nào lớn hơn 300 ký tự sẽ tự động thêm vào lịch sử điều hướng của bạn.
* Sửa khôi phục cửa sổ Paperback từ khay hệ thống.
* Sửa tài liệu Markdown hiển thị văn bản thô thay vì HTML được kết xuất trong Chế độ xem web.
* Sửa bảng không kết xuất chính xác trong tệp Markdown.
* Các tệp PDF chỉ có hình ảnh sẽ lúc này cảnh báo bạn về sự tồn tại của chúng khi bạn cố gắng tải một.
* Hiện có thể kiểm tra các bản dựng phát triển mới thay vì các bản phát hành ổn định khi kiểm tra cập nhật.
* Nhúng thông tin phiên bản đúng cách trong tệp thực thi Paperback.
* Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và điều hướng.
* Chuyển sang Hayro để phân tích cú pháp PDF, dẫn đến độ tin cậy hơn, tốc độ và ít DLL hơn.
* Viết lại toàn bộ ứng dụng bằng Rust. Codebase mới an toàn hơn, tải tài liệu nhanh hơn và dễ bảo trì và mở rộng hơn.
* Trình đơn ngữ cảnh của điều khiển văn bản sẽ lúc này bao gồm các hành động dành riêng cho trình đọc thay vì các mục chung như cắt và dán.

### Phiên bản 0.7.0
* Đã thêm hỗ trợ bảng cho tài liệu dựa trên HTML và XHTML! Điều hướng giữa các bảng bằng T và Shift+T, và nhấn Enter để xem bảng trong chế độ xem web.
* Đã thêm tính năng kết xuất web cơ bản! Nhấn Ctrl+Shift+V để mở phần tài liệu hiện tại của bạn trong trình kết xuất dựa trên web, hữu ích cho nội dung như định dạng phức tạp hoặc mẫu mã.
* Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
* Đã thêm nút Xóa tất cả vào hộp thoại Tất cả tài liệu.
* Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi một phiên bản mới khả dụng.
* Sửa khôi phục cửa sổ từ khay hệ thống.
* Sửa bản dịch của nút Có/Không trong hộp thoại xác nhận.
* Sửa tải cấu hình khi chạy dưới quyền quản trị viên.
* Sửa xử lý nhận xét trong tài liệu XML và HTML.
* Sửa phân tích cú pháp TOC trong sách Epub 2.
* Sửa điều hướng đến mục tiếp theo có cùng chữ cái trong mục lục.
* Sửa hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút tiếp theo/trước.
* Sửa TOC trong Epub đôi khi ném bạn đến mục sai.
* Sửa các vấn đề xử lý khoảng trắng khác nhau trong thẻ XML, HTML và pre.
* Sửa lỗi tắt một trong điều hướng liên kết.
* Sửa một số sách có khoảng trắng ở cuối dòng của chúng.
* Sửa các vấn đề trình phân tích cú pháp khác nhau.
* Các mục menu liên quan đến dấu trang cũng như danh sách yếu tố hiện được vô hiệu hóa đúng cách khi không có tài liệu nào được mở.
* Cải thiện xử lý danh sách trong các định dạng tài liệu khác nhau.
* Cải thiện quy trình dịch cho những người đóng góp.
* Nhiều hoán đổi nội bộ, di chuyển phần lớn logic kinh doanh của ứng dụng từ C++ sang Rust để cải thiện hiệu suất và khả năng bảo trì.

### Phiên bản 0.6.1
* Đã thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
* Đã thêm tính năng đi đến vị trí trước/tiếp theo rất cơ bản. Nếu bạn nhấn enter trên liên kết nội bộ và nó di chuyển con trỏ của bạn, vị trí đó sẽ lúc này được ghi nhớ, và có thể điều hướng bằng các phím mũi tên alt+left/right.
* Đã thêm danh sách yếu tố! Hiện tại nó chỉ hiển thị cây của tất cả các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết, nhưng có kế hoạch mở rộng nó trong tương lai.
* Đã thêm tùy chọn để bắt đầu Paperback ở chế độ tối đa hóa theo mặc định.
* Sửa liên kết trong một số tài liệu Epub không hoạt động chính xác.
* Sửa phân tích cú pháp TOC Epub chứa đường dẫn tương đối.
* Sửa một số tài liệu epub không hiển thị tiêu đề hoặc tác giả.
* Sửa tiêu đề của một số chương epub không xuất hiện chính xác trong hộp thoại TOC.
* Sửa bạn không có thể sử dụng thanh cách để kích hoạt các nút OK/hủy trong hộp thoại TOC.
* Cải thiện xử lý tiêu đề trong tài liệu Word.
* Bạn hiện sẽ nhận được phản hồi được nói nếu danh sách tài liệu gần đây trống khi bạn cố gắng mang lên hộp thoại.

### Phiên bản 0.6.0
* Tùy chọn mới để hiển thị trình đơn đi theo hình thức nhỏ gọn hơn nhiều đã được thêm vào hộp thoại tùy chọn, được kiểm tra theo mặc định.
* Đã thêm tùy chọn để điều hướng bằng các yếu tố cấu trúc wrap.
* Đã thêm tùy chọn vào trình đơn công cụ để mở thư mục chứa của tài liệu hiện được lấy nét.
* Đã thêm hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
* Đã thêm tính năng bộ hẹn giờ ngủ cơ bản, có thể truy cập bằng Ctrl+Shift+S.
* Đã thêm hỗ trợ để phân tích cú pháp sách điện tử FB2!
* Đã thêm hỗ trợ để phân tích cú pháp bản trình bày OpenDocument!
* Đã thêm hỗ trợ để phân tích cú pháp tệp Văn bản OpenDocument!
* Dấu trang hiện có thể được tạo để đánh dấu toàn bộ dòng hoặc chỉ đánh dấu một số văn bản được chỉ định. Nếu bạn không có lựa chọn hoạt động khi đặt dấu trang, hành vi giống như trước 0.6, và nó sẽ đánh dấu toàn bộ dòng. Tuy nhiên, nếu bạn chọn một số văn bản, chỉ văn bản đó sẽ được bao gồm trong dấu trang.
* Dấu trang hiện có thể có các ghi chú văn bản tùy chọn được gắn kèm! Điều hướng giữa các dấu trang chứa ghi chú với N và Shift+N, hoặc bật hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ không ghi chú được chọn bằng các phím tắt cụ thể.
* Dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "dấu trang x" khó chịu.
* Sách Epub chứa nội dung HTML giả vờ là XML sẽ lúc này được xử lý đúng cách.
* Sửa tải các tài liệu Markdown lớn.
* Sửa nhấn phím cách trong chế độ xem cây mục lục kích hoạt nút OK.
* Sửa xử lý khoảng trắng ở đầu thẻ pre trong cả tài liệu HTML và XHTML.
* Sửa điều khiển văn bản không lấy lại tiêu điểm đôi khi khi quay lại cửa sổ Paperback.
* Sửa trường văn bản trong hộp thoại go to percent không cập nhật giá trị của thanh trượt.
* Sửa kết xuất ID HTML tùy chỉnh trong tài liệu Markdown.
* HTML bên trong khối mã Markdown sẽ lúc này được kết xuất chính xác.
* Nếu tải sách có tham số dòng lệnh trong khi phiên bản Paperback hiện có đang chạy, bạn sẽ không còn nhận được lỗi nếu tải tài liệu của bạn mất hơn 5 giây.
* Nếu chạy Paperback dưới quyền quản trị viên, cấu hình sẽ lúc này được tải và lưu đúng cách.
* Hiện có thể xóa dấu trang trực tiếp từ trong hộp thoại dấu trang.
* Hiện có thể nhập và xuất dấu trang và vị trí đọc của bạn cho một tài liệu cụ thể. Tệp được tạo được đặt tên theo tệp có phần mở rộng .paperback. Nếu tệp như vậy được tìm thấy trong cùng thư mục với tệp khi tải nó, nó sẽ được tải tự động. Ngoài ra, bạn có thể nhập chúng theo cách thủ công bằng mục trong trình đơn công cụ.
* Liên kết bên trong tài liệu hiện được hỗ trợ đầy đủ! Sử dụng k và shift+k để di chuyển về phía trước và phía sau qua chúng, và nhấn enter để mở/kích hoạt một.
* Nhiều hoán đổi nội bộ, giúp ứng dụng nhanh hơn và tệp nhị phân nhỏ hơn.
* Nội dung Markdown hiện được xử lý trước để tuân thủ CommonMark trước khi kết xuất.
* Điều hướng theo danh sách và các mục của chúng hiện được hỗ trợ đầy đủ! Sử dụng L và Shift+L để đi theo danh sách chính nó, và I và Shift+I để đi qua các mục danh sách.
* Xóa phím Numpad hiện hoạt động để xóa tài liệu khỏi thanh tab ngoài việc xóa bình thường.
* Paperback bây giờ có thể tùy chọn thu nhỏ thành khay hệ thống của bạn! Tùy chọn này được tắt theo mặc định, nhưng bật nó sẽ khiến tùy chọn thu nhỏ trong trình đơn hệ thống đặt Paperback vào khay của bạn, có thể được khôi phục bằng cách nhấp vào biểu tượng được tạo.
* Paperback hiện có thể dịch đầy đủ! Danh sách các ngôn ngữ nó hỗ trợ hiện khá nhỏ, nhưng nó liên tục phát triển!
* Paperback hiện có một trang web chính thức, tại [paperback.dev](https://paperback.dev)!
* Tài liệu PPTX sẽ lúc này hiển thị mục lục cơ bản, chứa tất cả các trang chiếu.
* Đường dẫn đầy đủ đến tài liệu đã mở sẽ lúc này được hiển thị trong hộp thoại thông tin tài liệu.
* Trình cài đặt hiện bao gồm tùy chọn để xem readme trong trình duyệt của bạn sau khi cài đặt.
* Danh sách tài liệu gần đây đã được mở rộng rất nhiều! Thay vì chỉ hiển thị 10 tài liệu cuối cùng bạn mở, nó sẽ lúc này hiển thị số có thể tùy chỉnh, với phần còn lại của tài liệu bạn đã mở bất kỳ lúc nào có thể truy cập được thông qua hộp thoại nhỏ.
* Cải thiện nhỏ khác nhau cho các trình phân tích cú pháp trên toàn bộ, bao gồm đặt dòng trống giữa các trang chiếu trong bản trình bày PPTX, sửa xử lý dòng mới bên trong đoạn trong tài liệu word, và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0
* Đã thêm hỗ trợ tài liệu Microsoft Word!
* Đã thêm hỗ trợ cho bản trình bày PowerPoint!
* Sửa các mục menu nhất định không bị vô hiệu hóa mà không có tài liệu nào được mở.
* Sửa hướng của thanh trượt go to percent.
* Sửa mục lục trong sách Epub với đường dẫn tệp được mã hóa URL và/hoặc ID đoạn.
* Sửa khoảng trắng bị tước từ tiêu đề XHTML theo những cách kỳ lạ.
* Sửa xử lý khoảng trắng bên trong thẻ pre lồng nhau trong tài liệu HTML.
* Tài liệu HTML và Markdown hiện hỗ trợ tính năng mục lục! Khi bạn tải tài liệu HTML/Markdown, Paperback sẽ xây dựng mục lục riêng của nó từ cấu trúc của các tiêu đề trong tài liệu của bạn, và nó sẽ hiển thị cho bạn trong hộp thoại ctrl+t.
* Tài liệu HTML sẽ lúc này có tiêu đề được đặt trong thẻ tiêu đề, nếu nó tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà không có phần mở rộng.
* Chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo bài phát biểu. Điều này có nghĩa là không có DLL trình đọc màn hình được gửi cùng chương trình nữa, và nhiều trình đọc màn hình sẽ được hỗ trợ, chẳng hạn như Microsoft Narrator.
* Chuyển thư viện zip để cho phép mở loạt sách epub rộng hơn.
* Hộp thoại yêu cầu bạn nếu bạn muốn mở tài liệu dưới dạng văn bản thuần túy đã được hoàn toàn redo, và bây giờ nó cho phép bạn mở tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
* Hộp thoại go to percent hiện bao gồm trường văn bản cho phép bạn nhập thủ công phần trăm để nhảy đến.
* Trình phân tích cú pháp HTML sẽ bây giờ nhận ra dd, dt và dl là các yếu tố danh sách.
* Mục lục trong sách Epub sẽ một lần nữa được bảo tồn chính xác.
* Unicode không-break space hiện được cân nhắc khi tước dòng trống.
* Bạn sẽ không còn được hỏi cách bạn muốn mở tệp không được nhận dạng mỗi lần bạn tải nó, chỉ lần đầu tiên.

### Phiên bản 0.4.1
* Đã thêm biểu tượng trình đơn Bắt đầu tùy chọn vào trình cài đặt.
* Mục lục hiện sẽ sạch hơn trong một vài trường hợp, ví dụ nếu bạn có mục con và mục cha cùng văn bản ở cùng vị trí bạn sẽ chỉ nhìn thấy mục cha.
* Sửa mục lục trong một số tài liệu CHM.
* Sửa mục lục trong sách Epub 3 với đường dẫn tuyệt đối trong chúng.
* Tài liệu CHM hiện sẽ hiển thị tiêu đề của chúng được đặt trong tệp siêu dữ liệu.

### Phiên bản 0.4.0
* Đã thêm hỗ trợ tệp CHM!
* Đã thêm hỗ trợ dấu trang! Bạn có thể có bao nhiêu dấu trang tùy ý trong bao nhiêu tài liệu tùy ý. Bạn có thể nhảy về phía trước và phía sau qua chúng với b và shift+b, đặt một dấu bằng control+shift+b, và mang lên hộp thoại để nhảy đến dấu trang cụ thể bằng control+b.
* Đã thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài đặt Paperback vào thư mục Tệp chương trình của bạn, và tự động thiết lập liên kết tệp cho bạn.
* Các tệp văn bản có BOM hiện sẽ được giải mã đúng cách, và BOM sẽ không còn được hiển thị ở đầu văn bản nữa.
* Đã thêm nhiều thông tin hơn vào thanh trạng thái. Nó sẽ lúc này hiển thị dòng, ký tự và phần trăm đọc hiện tại của bạn.
* Nhận xét HTML, cũng như nội dung của các thẻ tập lệnh và kiểu, sẽ không còn được hiển thị trong đầu ra văn bản.
* Nếu chuyển đường dẫn tương đối đến Paperback trên dòng lệnh, nó sẽ lúc này giải quyết nó đúng cách.
* Chuyển động phần trăm bây giờ được xử lý bởi hộp thoại dựa trên thanh trượt riêng của nó, có thể truy cập bằng control+shift+g.
* Tài liệu mà không biết tiêu đề hoặc tác giả sẽ lúc này luôn có mặc định.
* Logic tiết kiệm vị trí hiện thông minh hơn nhiều và chỉ nên ghi vào đĩa khi hoàn toàn cần thiết.
* Tài liệu bạn đã lấy nét khi bạn đóng Paperback hiện được ghi nhớ trong các lần khởi động lại ứng dụng.
* Đầu vào vào các hộp thoại đi đến dòng và đi đến trang hiện nên được vệ sinh chặt chẽ hơn.
* Sửa điều hướng mục lục trong sách epub 3 với đường dẫn tương đối trong tờ khai của chúng.

### Phiên bản 0.3.0
* Sửa mục lục trong sách epub với tờ khai được mã hóa URL.
* Sửa điều hướng tiêu đề trong tài liệu HTML chứa các ký tự Unicode nhiều byte.
* Sửa mức sử dụng CPU cao trong tài liệu có tiêu đề dài do hồi quy trong wxWidgets.
* Sửa tải các tệp văn bản UTF-8.
* Sửa các mục TOC lồng nhau trong sách Epub đặt con trỏ của bạn ở vị trí sai.
* Sửa sự cố trên lối thoát ứng dụng trong một số trường hợp.
* Đã thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt ngắt dòng chữ!
* Hiện có thể quyên góp cho sự phát triển Paperback, thông qua mục quyên góp mới trong trình đơn trợ giúp hoặc thông qua liên kết tài trợ dự án này ở dưới cùng của trang chính kho lưu trữ GitHub.
* Tài liệu Markdown sẽ lúc này luôn có tiêu đề, và Paperback sẽ lúc này có thể tải hầu như mọi tệp Markdown.
* Tài liệu PDF sẽ lúc này luôn có tiêu đề, ngay cả khi siêu dữ liệu bị thiếu.
* Chuyển thư viện PDF sang thư viện được sử dụng trong Chromium, dẫn đến phân tích cú pháp PDF đáng tin cây hơn nhiều trên toàn bộ.
* Bạn hiện chỉ có thể có một phiên bản Paperback chạy cùng một lúc. Chạy paperback.exe với tên tệp khi nó đã chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
* Bạn hiện có thể nhấn xóa trên tài liệu trong điều khiển tab để đóng nó.

### Phiên bản 0.2.1
* Đã thêm tổng số trang vào nhãn trang trong hộp thoại go to page.
* Cho phép tab từ nội dung tài liệu đến danh sách các tài liệu đã mở của bạn.
* Sửa các nhấn phím tiêu đề đôi khi mở tài liệu gần đây nếu bạn có đủ.
* Paperback hiện sẽ xóa dấu gạch ngang mềm không cần thiết khỏi đầu ra văn bản.
* Sửa điều hướng tiêu đề đôi khi đặt bạn ở ký tự sai.

### Phiên bản 0.2.0
* Đã thêm hỗ trợ tài liệu markdown!
* Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các trang!
* Đã thêm các nhấn phím để điều hướng theo tiêu đề trong nội dung HTML, bao gồm sách epub và tài liệu markdown. Các nhấn phím này được thiết kế để hoạt động tương tự như trình đọc màn hình.
* Sửa tải épub với tên tệp được mã hóa URL trong tờ khai của chúng.
* Sửa tải sách epub 3 với XHTML nhúng bên trong của chúng.
* Thông báo hiện được nói nếu tài liệu không hỗ trợ mục lục hoặc phần, đối lập với các mục trình đơn bị vô hiệu hóa.
* Đã thêm trình đơn tài liệu gần đây! Nó hiện lưu trữ 10 tài liệu đã mở cuối cùng của bạn, và nhấn enter trên một sẽ mở nó để đọc.
* Hoàn toàn viết lại hộp thoại Tìm, giúp nó đơn giản hơn nhiều để sử dụng, cũng như thêm lịch sử 25 lần tìm kiếm cuối cùng của bạn và hỗ trợ biểu thức chính quy!
* Tài liệu đã mở trước đây hiện được ghi nhớ trong các lần khởi động lại ứng dụng. Điều này có thể cấu hình thông qua mục tùy chọn mới trong trình đơn công cụ.
* Đã thêm shift+f1 để mở readme trực tiếp trong Paperback chính nó.

### Phiên bản 0.1.0
* Bản phát hành ban đầu.
