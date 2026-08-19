<!-- machine-translated from doc/readme.md (source-hash: 88d5313cd5871ed4); please review and edit as needed -->

# Bản bìa mềm - phiên bản 0.8.5 {#paperback---version-0.8.5}

## Giới thiệu {#introduction}

Paperback là một trình đọc sách điện tử và tài liệu nhẹ, nhanh và dễ
tiếp cận dành cho mọi người, từ những người đọc bình thường đến những
người dùng chuyên sâu. Ứng dụng này được thiết kế để tương thích với
trình đọc màn hình, tốc độ nhanh và trải nghiệm không rườm rà.

## Yêu cầu hệ thống {#system-requirements}

Paperback hiện chạy trên Windows, macOS, iOS và Android.

## Tính năng {#features}

-   Hoàn toàn độc lập, không yêu cầu cài đặt bất kỳ phần mềm nào trên
    máy tính của bạn để bắt đầu đọc.
-   Tốc độ cực kỳ nhanh, ngay cả trên phần cứng cũ.
-   Giao diện tab đơn giản, cho phép bạn mở bao nhiêu tài liệu tùy ý và
    hiển thị song song.
-   Lưu chính xác vị trí đọc của bạn trên mọi tài liệu bạn mở.
-   Tùy chọn ghi nhớ các tài liệu bạn đã mở khi đóng chương trình và
    khôi phục chúng khi khởi động lại.
-   Bao gồm chức năng điều hướng tương tự như trong chế độ duyệt web của
    nhiều trình đọc màn hình để điều hướng nhanh chóng và dễ dàng qua
    các tài liệu.
-   Bao gồm hộp thoại tìm kiếm mạnh mẽ, với các tính năng như lịch sử và
    hỗ trợ biểu thức chính quy.
-   Có thể chạy hoàn toàn ở chế độ di động hoặc cài đặt với các liên kết
    tệp được thiết lập tự động.
-   Hỗ trợ một loạt các định dạng tệp phổ biến.

## Khả năng tương thích với trình đọc màn hình {#screen-reader-compatibility}

Paperback hoạt động tốt với tất cả các trình đọc màn hình chính. Tuy
nhiên, có một vấn đề đã biết đối với người dùng JAWS.

### JAWS và màn hình Braille {#jaws-and-braille-displays}

Nếu bạn sử dụng JAWS với màn hình Braille, bạn có thể thấy rằng các đoạn
văn dài bị cắt bớt khi cuộn về phía trước bằng các phím điều hướng của
màn hình. Lệnh đọc đoạn văn hiện tại cũng bị ảnh hưởng. Đây là một lỗi
trong cách JAWS xử lý điều khiển văn bản RICHEDIT50W, không phải do
Paperback gây ra, và phải mất khá lâu mới có bản sửa lỗi do Vispero rất
nhiệt tình trong việc giải quyết các vấn đề liên quan đến phần mềm nguồn
mở.

Giải pháp tạm thời, cuối cùng được đề xuất qua nhóm thảo luận JAWS sau
nhiều tháng chờ đợi, là chỉnh sửa `paperback.jcf` và thiết lập "Hiển thị
và di chuyển văn bản Braille" thành "Luôn sử dụng DOM nếu có sẵn". Bạn
cũng nên bật tùy chọn "Di chuyển văn bản theo đoạn", nếu không màn hình
sẽ dừng lại ở đoạn văn bản đang hoạt động thay vì di chuyển tiếp. Khi cả
hai thiết lập này được áp dụng, chức năng di chuyển văn bản sẽ hoạt động
chính xác.

## Các định dạng tệp hiện được hỗ trợ {#currently-supported-file-types}

Paperback hỗ trợ các định dạng và phần mở rộng sau:

-   Tệp trợ giúp CHM (`.chm`)
-   Sách DAISY (`.opf`, `.zip`)
-   Sách EPUB (`.epub`)
-   Sách điện tử FB2 (`.fb2`)
-   Tài liệu HTML (`.htm`, `.html`, `.xhtml`)
-   Tài liệu Markdown (`.md`, `.markdown`, `.mdx`, `.mdown`, `.mdwn`,
    `.mkd`, `.mkdn`, `.mkdown`, `.ronn`)
-   Tài liệu Microsoft Word (`.docx`, `.docm`, `.doc`)
-   Sách MOBI/Kindle (`.mobi`, `.azw`, `.azw3`)
-   Bài thuyết trình OpenDocument (`.odp`, `.fodp`)
-   Tệp văn bản OpenDocument (`.odt`, `.fodt`)
-   Tài liệu PDF (`.pdf`)
-   Bản trình bày PowerPoint (`.pptx`, `.pptm`, `.ppt`)
-   Tài liệu RTF (`.rtf`)
-   Văn bản thuần túy và tệp nhật ký (`.txt`, `.log`)

## Phím tắt {#keyboard-shortcuts}

Paperback được thiết kế để ưu tiên sử dụng bàn phím. Dưới đây là các
phím tắt hiện tại.

Các phím tắt dưới đây dành cho Windows. Trong trường hợp macOS có sự
khác biệt, phím tắt tương đương sẽ được ghi chú trong ngoặc đơn --- chủ
yếu là do Ctrl+G, Ctrl+W và Alt+Trái/Phải đã được sử dụng cho các quy
ước hệ thống hoặc ứng dụng khác trên nền tảng đó.

### Menu Tệp {#file-menu}

-   `Ctrl+O`: Mở tài liệu.
-   `Ctrl+F4` (macOS: `Cmd+W`): Đóng tài liệu hiện tại.
-   `Ctrl+Shift+F4` (macOS: `Cmd+Shift+W`): Đóng tất cả các tài liệu
    đang mở.
-   `Ctrl+Shift+T`: Mở lại tài liệu vừa đóng.
-   `Ctrl+R`: Hiển thị hộp thoại "Tất cả tài liệu" (từ mục Tài liệu gần
    đây).
-   `Ctrl+Q`: Thoát (chỉ dành cho Windows; trên macOS, tùy chọn này nằm
    trong menu ứng dụng).

### Menu "Go" {#go-menu}

-   `Ctrl+F`: Hiển thị hộp thoại Tìm kiếm.
-   `F3` (macOS: `Cmd+G`): Tìm tiếp.
-   `Shift+F3` (macOS: `Cmd+Shift+G`): Tìm trước đó.
-   `Ctrl+G` (macOS: `Cmd+L`): Chuyển đến dòng.
-   `Ctrl+Shift+G` (macOS: `Cmd+Shift+L`): Chuyển đến phần trăm.
-   `Ctrl+P`: Chuyển đến trang (nếu được tài liệu hiện tại hỗ trợ).
-   `Alt+Left` (macOS: `Cmd+[`): Quay lại trong lịch sử điều hướng.
-   `Alt+Right` (macOS: `Cmd+]`): Tiến tới trong lịch sử duyệt web.
-   `[`: Phần trước.
-   `]`: Phần tiếp theo.
-   `Shift+H`: Tiêu đề trước.
-   `H`: Tiêu đề tiếp theo.
-   `Shift+1` qua `Shift+6`: Tiêu đề trước ở cấp độ 1-6.
-   `1` qua `6`: Tiêu đề tiếp theo ở cấp độ 1-6.
-   `Shift+P`: Trang trước.
-   `P`: Trang tiếp theo.
-   `Shift+B`: Dấu trang trước.
-   `B`: Dấu trang tiếp theo.
-   `Shift+N`: Ghi chú trước.
-   `N`: Ghi chú tiếp theo.
-   `Ctrl+B`: Chuyển đến tất cả dấu trang và ghi chú.
-   `Ctrl+Alt+B`: Chuyển đến chỉ các dấu trang.
-   `Ctrl+Alt+M`: Chuyển đến chỉ các ghi chú.
-   `Ctrl+Shift+W` (macOS: `RawCtrl+Shift+W`, tức là phím Control vật lý
    thay vì phím Cmd): Xem nội dung ghi chú tại vị trí hiện tại.
-   `Shift+K`: Liên kết trước.
-   `K`: Liên kết tiếp theo.
-   `Shift+G`: Hình ảnh trước.
-   `G`: Hình tiếp theo.
-   `Shift+F`: Hình trước.
-   `F`: Hình tiếp theo.
-   `Shift+T`: Bảng trước.
-   `T`: Bảng tiếp theo.
-   `Shift+S`: Dấu phân cách trước.
-   `S`: Dấu phân cách tiếp theo.
-   `Shift+L`: Danh sách trước.
-   `L`: Danh sách tiếp theo.
-   `Shift+I`: Mục danh sách trước.
-   `I`: Mục danh sách tiếp theo.
-   `Shift+,`: Chuyển đến đầu của vùng chứa hiện tại (danh sách hoặc
    bảng).
-   `,`: Di chuyển qua phần cuối của vùng chứa hiện tại (danh sách hoặc
    bảng).

### Menu Công cụ {#tools-menu}

-   `Ctrl+W` (macOS: `RawCtrl+W`, tức là phím Control vật lý thay vì
    phím Cmd): Hiển thị số từ của tài liệu hiện tại.
-   `Ctrl+I`: Hiển thị thông tin tài liệu.
-   `Ctrl+T`: Hiển thị mục lục.
-   `F7`: Hiển thị danh sách các phần tử.
-   `Ctrl+Shift+C`: Mở thư mục chứa tài liệu.
-   `Ctrl+Shift+V`: Mở nội dung hiện tại trong Web View.
-   `Ctrl+U`: Xem nguồn tài liệu trong một tab mới.
-   `Ctrl+Shift+E`: Xuất dữ liệu tài liệu (`.paperback`).
-   `Ctrl+Shift+I`: Nhập dữ liệu tài liệu (`.paperback`).
-   `Ctrl+E`: Xuất tài liệu hiện tại sang định dạng văn bản thuần túy.
-   `Ctrl+Shift+B`: Bật/tắt dấu trang tại vị trí đang chọn/con trỏ.
-   `Ctrl+Shift+N`: Thêm hoặc chỉnh sửa ghi chú dấu trang tại vị trí
    đang chọn/con trỏ.
-   `Ctrl+Alt+W`: Bật/tắt tự động xuống dòng.
-   `Ctrl+,`: Mở tùy chọn (macOS: Tùy chọn, trong menu ứng dụng ).
-   `Ctrl+Shift+S`: Bật/tắt bộ hẹn giờ ngủ.

### Menu Trợ giúp {#help-menu}

-   `Ctrl+F1`: Hiển thị hộp thoại Giới thiệu.
-   `F1`: Xem trợ giúp trong trình duyệt mặc định của bạn.
-   `Shift+F1`: Xem trợ giúp trong Paperback.
-   `Ctrl+Shift+U`: Kiểm tra các bản cập nhật.
-   `Ctrl+D`: Mở trang quyên góp trong trình duyệt mặc định của bạn.

### Các phím điều khiển xem tài liệu bổ sung {#additional-document-view-keys}

-   `Delete` / `Numpad Delete` trên thanh điều khiển tab: Đóng tab tài
    liệu đã chọn.
-   `Enter` hoặc `Space` trong văn bản tài liệu: Kích hoạt liên kết tại
    vị trí con trỏ, hoặc mở chế độ xem bảng khi con trỏ nằm trên dấu
    hiệu bảng.
-   `Shift+F10` hoặc phím Menu/Ứng dụng trong nội dung tài liệu: Mở menu
    ngữ cảnh.

## Các ngôn ngữ được hỗ trợ {#supported-languages}

Paperback đã được dịch sang nhiều ngôn ngữ khác nhau, và luôn có thêm
các ngôn ngữ mới được bổ sung. Danh sách đầy đủ được liệt kê bên dưới.

Để tìm hiểu cách đóng góp, vui lòng đọc [Hướng dẫn dịch
thuật](translating.md) của chúng tôi.

-   Tiếng Bosnia
-   Tiếng Séc
-   Tiếng Hà Lan
-   Tiếng Phần Lan
-   Tiếng Pháp
-   Tiếng Đức
-   Tiếng Nhật
-   Tiếng Ba Lan
-   Tiếng Bồ Đào Nha (Brazil)
-   Tiếng Nga
-   Tiếng Trung giản thể
-   Tiếng Serbia
-   Tiếng Tây Ban Nha
-   Tiếng Việt

## Tín dụng {#credits}

### Phát triển {#development}

-   Quin Gillespie: nhà phát triển chính và người sáng lập dự án.
-   Aryan Choudhary: cộng tác viên chính.

### Đóng góp {#donations}

Những người sau đây đã đóng góp một khoản tiền đáng kể cho việc phát
triển Paperback. Nếu bạn đóng góp, tên của bạn sẽ không tự động được
thêm vào đây; tôi chỉ thêm những người muốn công khai khoản đóng góp của
họ.

Lưu ý: Tôi coi việc trở thành nhà tài trợ công khai trên GitHub là cơ sở
để tự động được đưa vào danh sách này.

-   Alex Hall
-   Brandon McGinty
-   Brian Hartgen
-   Debbie Yuille
-   Devin Prater
-   Felix Steindorff
-   Hamish Mackenzie
-   James Scholes
-   Jayson Smith
-   Jonathan Rodriguez
-   Jonathan Schuster
-   Keao Wright
-   Michael Marshall
-   Pratik Patel
-   Roberto Perez
-   Sean Randall
-   Timothy Wynn
-   Tyler Rodick

## Nhật ký thay đổi {#changelog}

### Phiên bản 0.9.0 (chưa phát hành) {#version-0.9.0-unreleased}

-   Đã thêm nút hủy vào hộp thoại cập nhật đang diễn ra.
-   Đã thêm một công cụ dòng lệnh (CLI) có tên là pb để nhanh chóng
    chuyển đổi bất kỳ định dạng nào được Paperback hỗ trợ sang HTML,
    Markdown hoặc văn bản thuần túy.
-   Đã thêm phím tắt có thể tùy chỉnh để khôi phục Paperback từ khay hệ
    thống.
-   Đã thêm nút \"Tìm kiếm\" vào hộp thoại \"Tất cả tài liệu\" để tìm
    các cuốn sách bị thiếu vừa thay đổi đường dẫn.
-   Đã thêm bộ lọc trạng thái và thanh trạng thái vào hộp thoại \"Tất cả
    tài liệu\", để bạn có thể lọc theo trạng thái tài liệu và xem có bao
    nhiêu tài liệu được hiển thị và được chọn.
-   Đã thêm `Ctrl+Shift+A` phím tắt để hủy chọn tất cả tài liệu trong
    hộp thoại \"Tất cả tài liệu\".
-   Đã thêm tab \"Độ dễ đọc\" vào hộp thoại tùy chọn, với các tùy chọn
    sau:
    -   Tự động xuống dòng (được chuyển từ phần \"Chung\");
    -   Hiển thị bảng trong dòng (tính năng mới trong bản phát hành này,
        xem bên dưới);
    -   Phông chữ;
    -   Màu nền;
    -   Khoảng cách dòng;
    -   Khoảng cách đoạn văn;
    -   Khoảng cách giữa các chữ cái;
    -   Căn chỉnh văn bản.
-   Đã thêm một nút chuyển đổi để xác định cách hiển thị bảng, và đồng
    nhất cách hiển thị bảng trên các tài liệu.
-   đã thêm tùy chọn \"Xem Nguồn\" để mở mã nguồn của tài liệu trong một
    tab mới, rất hữu ích khi chỉnh sửa Markdown chẳng hạn.
-   Đã thêm thời gian đọc ước tính vào hộp thoại đếm từ, cũng như khả
    năng thiết lập tốc độ đọc của bạn để chỉ số này thực sự hữu ích.
-   Đã thêm hỗ trợ ARM64 cho Windows!
-   Đã thêm hỗ trợ Android!
-   Đã thêm hỗ trợ iOS!
-   Đã thêm hỗ trợ macOS!
-   Đã thêm các ngôn ngữ mới: tiếng Hà Lan, tiếng Phần Lan và tiếng Ba
    Lan.
-   Đã thêm hỗ trợ điều hướng theo container.
-   Đã thêm hỗ trợ cho danh sách, mục danh sách, hình vẽ và hình ảnh
    trong các tài liệu CHM .
-   Đã thêm mục menu \"Tự động xuống dòng\" và phím tắt tương ứng.
-   Âm thanh dấu trang/ghi chú giờ đây sẽ phát chính xác khi bạn di
    chuyển con trỏ qua một từ chứa dấu trang hoặc ghi chú.
-   Các tài liệu được mã hóa bằng các bộ mã hóa CJK cũ, chẳng hạn như
    GBK, Big5 và Shift_JIS, giờ đây sẽ hiển thị đúng thay vì chỉ là một
    đống ký tự lộn xộn.
-   Mở rộng mục menu xuất để cho phép xuất sang HTML và Markdown ngoài
    định dạng văn bản thuần túy.
-   Đã khắc phục lỗi khiến việc áp dụng tính năng tự động xuống dòng đưa
    bạn trở lại đầu tài liệu.
-   Đã khắc phục lỗi sách daisy hiển thị thông tin không chính xác trên
    thanh trạng thái.
-   Đã khắc phục lỗi các thẻ dl, dt và dd không tạo ra dấu ngắt dòng
    trong các tài liệu XHTML.
-   Đã khắc phục lỗi phím Escape không đóng được các hộp thoại Thông tin
    Tài liệu và Tất cả Tài liệu. Đã khắc phục lỗi các neo filepos trong
    sách Mobi làm tách các thẻ HTML và chèn
-   Đã khắc phục lỗi các điểm neo filepos trong sách Mobi làm tách các
    thẻ HTML và chèn dữ liệu rác vào nội dung sách.
-   Đã khắc phục tình trạng giật lag khi di chuyển gần cuối trường văn
    bản trong các tài liệu lớn.
-   Đã khắc phục các liên kết trong các sách Mobi phiên bản cũ.
-   Đã khắc phục lỗi tải sách DAISY có khai báo mã hóa không hợp lệ.
-   Đã khắc phục lỗi điều hướng trang thông báo sai dòng văn bản trong
    một số trường hợp.
-   Đã khắc phục lỗi phân tích tài liệu RTF có chứa các ký tự không phải
    chữ La-tinh.
-   Đã khắc phục lỗi "Mở lại tài liệu vừa đóng" cố gắng mở lại tệp
    readme đi kèm.
-   Đã khắc phục lỗi thanh tiêu đề không được cập nhật sau khi đóng tài
    liệu từ hộp thoại "Tất cả tài liệu".
-   Đã khắc phục lỗi hộp thoại webview không thể thay đổi kích thước và
    bật lên với kích thước ban đầu rất nhỏ.
-   Đã khắc phục lỗi hiển thị tiêu đề không đúng trong các tài liệu Word
    có tên kiểu định dạng phụ thuộc vào vùng ngôn ngữ.
-   Đã khắc phục lỗi tab đã chọn không được lấy tiêu điểm đúng cách sau
    khi khởi động lại Paperback.
-   Nếu vùng chọn đang hoạt động khi bạn mở hộp thoại đếm từ, số lượng
    từ bạn đã chọn giờ đây sẽ được hiển thị.
-   Hình ảnh giờ đây sẽ hiển thị đúng cách trong webview nhúng.
-   Cải thiện khả năng xử lý tệp của Paperback trên các ổ đĩa mạng
    Windows: khi nhấn \"hiển thị tệp trong thư mục\", tệp trên bộ lưu
    trữ mạng sẽ được lấy tiêu điểm đúng cách và đường dẫn không còn chứa
    các ký tự lạ.
-   Đã cải thiện đáng kể việc phân tích cú pháp AZW3.
-   Đã chuyển từ chmlib sang trình đọc tệp CHM thuần Rust của riêng
    chúng tôi.
-   Trên máy tính để bàn, các tệp .paperback sẽ không còn bị tải bắt
    buộc khi khôi phục tài liệu. Thay vào đó, bạn sẽ được yêu cầu xác
    nhận khi tệp được tìm thấy.
-   Paperback hiện sẽ chuyển sang trích xuất văn bản thuần túy đối với
    các tệp PDF được gắn thẻ sai.
-   Tính năng \"Mở thư mục chứa\" giờ đây sẽ tập trung vào tệp đã chỉ
    định trong Explorer.
-   Việc mở tệp readme giờ đây sẽ tuân theo ngôn ngữ bạn đã chọn.
-   Các tài liệu PowerPoint hiện đã hỗ trợ bảng.
-   Cập nhật menu đúng cách và đặt tiêu điểm vào ô nhập văn bản khi mở
    phần trợ giúp trong Paperback.
-   Tệp Readme.html sẽ không còn được thêm vào danh sách \"Tất cả tài
    liệu\" khi được mở qua phím tắt Shift+F1.
-   Việc xóa tài liệu khỏi hộp thoại \"Các tài liệu gần đây\" giờ đây
    cũng sẽ đóng tab đang hoạt động của chúng.
-   Đã chuyển sang phương pháp IPC an toàn hơn nhiều trên Windows.
-   Tiêu đề tài liệu đang hoạt động giờ đây sẽ được đọc khi chuyển đổi
    giữa các tab.
-   Trình cập nhật hiện hiển thị chính xác nội dung của các thẻ mã
    Markdown trong ghi chú phát hành.
-   Trình cập nhật hiện sẽ xác minh rằng tệp đã tải xuống chưa bị can
    thiệp bất hợp pháp.
-   Trình xem web hiện được mở tại vị trí đọc hiện tại của bạn.
-   Bộ lọc tìm kiếm của bạn trong hộp thoại \"Tất cả tài liệu\" giờ đây
    được giữ nguyên sau khi xóa một tài liệu.

### Phiên bản 0.8.5 {#version-0.8.5}

-   Đã thêm hỗ trợ trang cho sách EPUB.
-   Đã thêm hỗ trợ cho các tài liệu Microsoft Office được mã hóa. Hiện
    tại Word phiên bản cũ, Word hiện đại và PowerPoint hiện đại được hỗ
    trợ, với PowerPoint phiên bản cũ dự kiến sẽ được hỗ trợ trong tương
    lai.
-   Đã thêm hỗ trợ cho các tài liệu Microsoft Word phiên bản cũ
    (\*.doc)!
-   Đã thêm hỗ trợ cho các bản trình bày PowerPoint phiên bản cũ
    (\*.ppt)!
-   Đã thêm hỗ trợ cho sách định dạng mobi và AZW3!
-   Đã thêm hỗ trợ cho các tệp PDF có thẻ!
-   Đã thêm phím tắt Ctrl+Q để thoát khỏi ứng dụng.
-   Đã thêm hỗ trợ cho các sách nén từ Bookshare (cả định dạng DAISY và
    Word)!
-   Văn bản thay thế (Alt text) cho hình ảnh nhúng giờ đây sẽ được hiển
    thị đúng cách.
-   Các tài liệu CHM hiện đã hỗ trợ điều hướng liên kết nội bộ đúng
    cách.
-   Đã khắc phục lỗi âm thanh dấu trang phát ra khi bắt đầu đoạn văn
    thay vì tại vị trí của dấu trang.
-   Đã khắc phục lỗi chức năng \"Đi đến trang\" bị lệch 1 trang.
-   Đã khắc phục lỗi phím Esc không hoạt động khi đóng hộp thoại \"Mở
    dưới dạng\".
-   Đã khắc phục lỗi menu ngữ cảnh của trình đọc không hiển thị khi nhấp
    chuột phải hoặc nhấn phím Applications.
-   Đã khắc phục lỗi đôi khi tài liệu sai được chọn khi mở tài liệu từ
    dòng lệnh.
-   Các tệp PDF chỉ chứa hình ảnh lại được phát hiện và thông báo cho
    bạn về sự tồn tại của chúng.
-   Giờ đây, bạn có thể điều hướng qua các hình ảnh và hình vẽ bằng các
    phím g/Shift+g và f/Shift+f tương ứng.
-   Paperback giờ đây sẽ tuân thủ cài đặt chế độ tối của ứng dụng của
    bạn.
-   Đã loại bỏ hỗ trợ DAISY XML vì không còn cần thiết nữa.
-   Đã chuyển trở lại chế độ điều hướng theo chữ cái đầu tiên gốc của
    Win32 trong cây mục lục.
-   Hộp thoại lỗi tải hiện hiển thị các thông báo lỗi chi tiết hơn.
-   Webview giờ đây sẽ mở nhanh hơn và mượt mà hơn nhiều.

### Phiên bản 0.8.2 {#version-0.8.2}

-   Đã thêm hỗ trợ trang cho các tài liệu RTF!
-   Đã khắc phục lỗi khiến việc mở webview trong các tệp epub chứa liên
    kết bên ngoài sẽ tự động kích hoạt chúng.
-   Đã khắc phục lỗi khiến trình phân tích cú pháp RTF không chèn khoảng
    trắng giữa các từ trong một số trường hợp hiếm gặp.
-   Đã khắc phục lỗi các đoạn văn bị chia thành nhiều dòng ngắn trong
    một số tài liệu PDF .
-   Các tài liệu PDF hiện đã hỗ trợ điều hướng cơ bản qua liên kết và
    tiêu đề !
-   Các tab và ký tự xuống dòng trong RTF giờ đây được hiển thị chính
    xác như trong tài liệu.
-   Đã quay trở lại sử dụng thư viện pdfium đã được kiểm chứng và đáng
    tin cậy để phân tích các tệp PDF, giúp việc hiển thị PDF trở nên
    đáng tin cậy hơn rất nhiều.

### Phiên bản 0.8.1 {#version-0.8.1}

-   Đã thêm phím tắt Ctrl+Shift+T để mở lại tài liệu vừa đóng gần nhất.
-   Hộp thoại \"Tất cả tài liệu\" hiện hỗ trợ chọn nhiều tài liệu để mở
    cùng lúc.
-   Đã khắc phục một số lỗi liên quan đến trình phân tích cú pháp RTF.
-   Đã khắc phục lỗi đường dẫn tệp chứa các ký tự không phải ASCII
    (chẳng hạn như các ký tự š, č, ć, ž trong tiếng Bosnia) bị hỏng khi
    mở tệp qua một phiên bản Paperback thứ hai.
-   Đã khắc phục lỗi văn bản PDF bị đọc sai thứ tự và khoảng cách không
    chính xác xung quanh các từ viết hoa.
-   Đã khắc phục tình trạng tải tài liệu chậm khi mở các tệp có dung
    lượng lớn.
-   Đã khắc phục vấn đề bản địa hóa các nút "Có"/"Không" trong các hộp
    thoại xác nhận.

### Phiên bản 0.8.0 {#version-0.8.0}

-   Đã thêm bản dịch tiếng Nhật, tiếng Trung giản thể và tiếng Việt! Đã
    thêm trình cập nhật tự động, giờ đây sẽ thay thế phiên bản
-   Đã thêm trình cập nhật tự động, giờ đây sẽ thay thế phiên bản
    Paperback hiện đang cài đặt của bạn thay vì chỉ tải xuống phiên bản
    mới!
-   Đã thêm phản hồi âm thanh tùy chọn khi đến một dấu trang hoặc ghi
    chú, cảm ơn Andre Louis đã cung cấp các âm thanh này!
-   Đã thêm hỗ trợ tài liệu RTF!
-   Đã thêm hỗ trợ cho các tài liệu DAISY XML.
-   Đã thêm hỗ trợ cho các tệp văn bản Flat Open Document!
-   Đã thêm hỗ trợ cho các bản trình bày Flat Open Document!
-   Đã thêm hỗ trợ cho các dấu phân cách bằng phím s và shift+s.
-   Bất kỳ thao tác di chuyển nào vượt quá 300 ký tự giờ đây sẽ tự động
    được thêm vào lịch sử điều hướng của bạn.
-   Đã khắc phục lỗi khôi phục cửa sổ Paperback từ khay hệ thống.
-   Đã khắc phục lỗi các tài liệu Markdown hiển thị văn bản thô thay vì
    HTML đã được hiển thị trong Web View.
-   Đã khắc phục lỗi bảng không hiển thị đúng trong các tệp Markdown.
-   Các tệp PDF chỉ chứa hình ảnh giờ đây sẽ cảnh báo bạn về sự tồn tại
    của chúng khi bạn cố gắng tải một tệp như vậy.
-   Giờ đây, bạn có thể kiểm tra các bản dựng phát triển mới thay vì các
    bản phát hành ổn định khi kiểm tra cập nhật.
-   Đã nhúng thông tin phiên bản đúng cách vào tệp thực thi Paperback.
-   Chia hộp thoại tùy chọn thành các tab để dễ sử dụng và dễ điều
    hướng.
-   Chuyển sang sử dụng Hayro để phân tích PDF, giúp tăng độ tin cậy,
    tốc độ và giảm số lượng DLL.
-   Viết lại toàn bộ ứng dụng bằng Rust. Cơ sở mã mới an toàn hơn, tải
    tài liệu nhanh hơn và dễ bảo trì cũng như mở rộng hơn.
-   Menu ngữ cảnh của điều khiển văn bản giờ đây sẽ bao gồm các hành
    động dành riêng cho trình đọc thay vì các mục chung chung như cắt và
    dán.

### Phiên bản 0.7.0 {#version-0.7.0}

-   Đã thêm hỗ trợ bảng cho các tài liệu dựa trên HTML và XHTML! Điều
    hướng giữa các bảng bằng phím T và Shift+T, và nhấn Enter để xem một
    bảng trong trình xem web.
-   Đã thêm tính năng hiển thị web cơ bản! Nhấn Ctrl+Shift+V để mở phần
    hiện tại của tài liệu trong trình hiển thị dựa trên web, rất hữu ích
    cho nội dung như định dạng phức tạp hoặc các mẫu mã.
-   Đã thêm bản dịch tiếng Nga, cảm ơn Ruslan Gulmagomedov!
-   Đã thêm nút \"Xóa tất cả\" vào hộp thoại \"Tất cả tài liệu\".
-   Trình kiểm tra cập nhật hiện hiển thị ghi chú phát hành khi có phiên
    bản mới sẵn sàng.
-   Đã khắc phục lỗi khôi phục cửa sổ từ khay hệ thống.
-   Đã sửa lỗi bản dịch các nút \"Có\"/\"Không\" trong các hộp thoại xác
    nhận.
-   Đã khắc phục lỗi tải cấu hình khi chạy với quyền quản trị viên.
-   Đã khắc phục lỗi xử lý nhận xét trong các tài liệu XML và HTML.
-   Đã khắc phục lỗi phân tích mục lục (TOC) trong sách Epub 2.
-   Đã khắc phục lỗi khi chuyển đến mục tiếp theo có cùng chữ cái trong
    mục lục.
-   Đã sửa lỗi hộp thoại tìm kiếm không ẩn đúng cách khi sử dụng các nút
    Tiếp theo/Trước đó.
-   Đã khắc phục lỗi mục lục (TOC) của sách EPUB đôi khi chuyển người
    dùng đến mục sai.
-   Đã khắc phục các vấn đề khác nhau liên quan đến xử lý khoảng trắng
    trong các thẻ XML, HTML và pre .
-   Đã khắc phục lỗi lệch một đơn vị trong điều hướng liên kết.
-   Đã khắc phục tình trạng một số cuốn sách có khoảng trắng thừa ở cuối
    dòng.
-   Đã khắc phục các vấn đề khác nhau liên quan đến trình phân tích cú
    pháp.
-   Các mục menu liên quan đến dấu trang cũng như danh sách các phần tử
    hiện đã được vô hiệu hóa đúng cách khi không có tài liệu nào được
    mở.
-   Đã cải thiện việc xử lý danh sách trong các định dạng tài liệu khác
    nhau.
-   Đã cải thiện quy trình dịch thuật cho các cộng tác viên.
-   Đã thực hiện nhiều cải tiến nội bộ, chuyển phần lớn logic nghiệp vụ
    của ứng dụng từ C++ sang Rust để nâng cao hiệu suất và khả năng bảo
    trì.

### Phiên bản 0.6.1 {#version-0.6.1}

-   Đã thêm hỗ trợ PDF được bảo vệ bằng mật khẩu!
-   Đã thêm tính năng chuyển đến vị trí trước/sau rất cơ bản. Nếu bạn
    nhấn phím Enter trên một liên kết nội bộ và nó di chuyển con trỏ của
    bạn, vị trí đó giờ đây sẽ được ghi nhớ và bạn có thể điều hướng đến
    đó bằng phím Alt + mũi tên trái/phải.
-   Đã thêm danh sách các phần tử! Hiện tại, danh sách này chỉ hiển thị
    cây các tiêu đề trong tài liệu của bạn hoặc danh sách các liên kết,
    nhưng có kế hoạch mở rộng nó trong tương lai.
-   Đã thêm tùy chọn khởi động Paperback ở chế độ toàn màn hình theo mặc
    định.
-   Đã khắc phục lỗi liên kết trong một số tài liệu Epub không hoạt động
    đúng cách.
-   Đã khắc phục lỗi phân tích mục lục (TOC) của tệp EPUB chứa đường dẫn
    tương đối.
-   Đã khắc phục lỗi một số tài liệu EPUB không hiển thị tiêu đề hoặc
    tác giả.
-   Đã khắc phục lỗi tiêu đề của một số chương EPUB không hiển thị đúng
    trong hộp thoại Mục lục.
-   Đã khắc phục lỗi không thể sử dụng phím cách để kích hoạt các nút
    OK/Hủy trong hộp thoại Mục lục.
-   Đã cải thiện việc xử lý các tiêu đề trong tài liệu Word.
-   Giờ đây, bạn sẽ nhận được phản hồi bằng giọng nói nếu danh sách tài
    liệu gần đây trống khi bạn cố gắng mở hộp thoại.

### Phiên bản 0.6.0 {#version-0.6.0}

-   Một tùy chọn mới để hiển thị menu \"Go\" dưới dạng gọn gàng hơn
    nhiều đã được thêm vào hộp thoại tùy chọn, được chọn mặc định.
-   Đã thêm tùy chọn để điều hướng theo các phần tử cấu trúc được tự
    động xuống dòng.
-   Đã thêm một tùy chọn vào menu \"Công cụ\" để mở thư mục chứa tài
    liệu đang được chọn.
-   Đã thêm một hệ thống cập nhật khá đơn giản nhưng rất hiệu quả.
-   Đã thêm tính năng hẹn giờ ngủ cơ bản, có thể truy cập bằng phím tắt
    Ctrl+Shift+S.
-   Đã thêm hỗ trợ phân tích cú pháp sách điện tử FB2!
-   Đã thêm hỗ trợ phân tích các bản trình bày OpenDocument!
-   Đã thêm hỗ trợ phân tích các tệp văn bản OpenDocument!
-   Giờ đây, bạn có thể tạo dấu trang để đánh dấu toàn bộ dòng hoặc chỉ
    đánh dấu một số văn bản cụ thể. Nếu bạn không chọn bất kỳ phần nào
    khi đặt dấu trang, hành vi sẽ giống như trước phiên bản 0.6 và toàn
    bộ dòng sẽ được đánh dấu. Tuy nhiên, nếu bạn chọn một đoạn văn bản,
    chỉ đoạn văn bản đó mới được đưa vào dấu trang.
-   Giờ đây, dấu trang có thể kèm theo ghi chú văn bản tùy chọn! Di
    chuyển giữa các dấu trang có ghi chú bằng phím N và Shift+N, hoặc mở
    hộp thoại dấu trang với tất cả dấu trang, chỉ ghi chú hoặc chỉ các
    dấu trang không có ghi chú được chọn bằng các phím tắt cụ thể.
-   Các dấu trang trong hộp thoại dấu trang sẽ không còn có tiền tố "dấu
    trang x" gây phiền toái nữa.
-   Các cuốn sách Epub chứa nội dung HTML giả làm XML giờ đây sẽ được xử
    lý đúng cách.
-   Đã khắc phục lỗi khi tải các tài liệu Markdown có dung lượng lớn.
-   Đã khắc phục lỗi nhấn phím cách trong chế độ xem cây mục lục sẽ kích
    hoạt nút OK.
-   Đã khắc phục việc xử lý khoảng trắng ở đầu các thẻ pre trong cả tài
    liệu HTML và XHTML.
-   Đã khắc phục lỗi đôi khi trường văn bản không lấy lại tiêu điểm khi
    quay trở lại cửa sổ của Paperback.
-   Đã khắc phục lỗi trường văn bản trong hộp thoại \"Đi đến phần trăm\"
    không cập nhật giá trị của thanh trượt.
-   Đã khắc phục lỗi hiển thị các ID HTML tùy chỉnh trong tài liệu
    Markdown.
-   HTML bên trong các khối mã Markdown giờ đây sẽ được hiển thị đúng
    cách.
-   Nếu tải một cuốn sách bằng tham số dòng lệnh trong khi một phiên bản
    Paperback hiện có đang chạy, bạn sẽ không còn gặp lỗi nếu việc tải
    tài liệu của bạn mất hơn 5 giây.
-   Nếu chạy Paperback với tư cách quản trị viên, cấu hình giờ đây sẽ
    được tải và lưu đúng cách.
-   Giờ đây, bạn có thể xóa dấu trang trực tiếp từ hộp thoại dấu trang.
-   Giờ đây, bạn có thể nhập và xuất dấu trang cũng như vị trí đọc của
    một tài liệu cụ thể. Tệp được tạo ra sẽ được đặt tên theo tên tệp
    với phần mở rộng .paperback. Nếu tệp này được tìm thấy trong cùng
    thư mục với tệp đang được tải, nó sẽ được tự động tải. Nếu không,
    bạn có thể nhập chúng thủ công bằng cách sử dụng một mục trong menu
    công cụ.
-   Các liên kết bên trong tài liệu hiện đã được hỗ trợ đầy đủ! Sử dụng
    phím k và shift+k để di chuyển tới và lùi qua các liên kết, và nhấn
    enter để mở/kích hoạt một liên kết.
-   Nhiều cải tiến nội bộ, giúp ứng dụng chạy nhanh hơn và tệp nhị phân
    nhỏ hơn.
-   Nội dung Markdown hiện được xử lý trước để tuân thủ tiêu chuẩn
    CommonMark trước khi hiển thị.
-   Việc điều hướng theo danh sách và các mục trong danh sách hiện đã
    được hỗ trợ đầy đủ! Sử dụng phím L và Shift+L để di chuyển giữa các
    danh sách, và phím I cùng Shift+I để di chuyển qua các mục trong
    danh sách.
-   Phím Delete trên bàn phím số giờ đây cũng có thể xóa tài liệu khỏi
    thanh tab ngoài chức năng xóa thông thường.
-   Paperback giờ đây có thể thu nhỏ tùy chọn vào khay hệ thống của bạn!
    Tùy chọn này bị tắt theo mặc định, nhưng khi bật lên, tùy chọn thu
    nhỏ trong menu hệ thống sẽ đưa Paperback vào khay của bạn, có thể
    khôi phục bằng cách nhấp vào biểu tượng được tạo ra.
-   Paperback hiện đã hỗ trợ dịch hoàn toàn! Danh sách các ngôn ngữ mà
    nó hỗ trợ hiện tại còn khá hạn chế, nhưng đang không ngừng mở rộng!
-   Paperback hiện đã có trang web chính thức tại
    [paperback.dev](https://paperback.dev)!
-   Các tài liệu PPTX giờ đây sẽ hiển thị mục lục cơ bản, bao gồm tất cả
    các trang chiếu.
-   Đường dẫn đầy đủ đến tài liệu đang mở giờ đây sẽ được hiển thị trong
    hộp thoại thông tin tài liệu.
-   Trình cài đặt hiện đã bao gồm tùy chọn để xem tệp readme trong trình
    duyệt của bạn sau khi cài đặt.
-   Danh sách các tài liệu gần đây đã được mở rộng đáng kể! Thay vì chỉ
    hiển thị 10 tài liệu gần nhất mà bạn đã mở, giờ đây nó sẽ hiển thị
    một số lượng có thể tùy chỉnh, với phần còn lại của các tài liệu mà
    bạn từng mở có thể truy cập thông qua một hộp thoại nhỏ.
-   Nhiều cải tiến nhỏ đối với các trình phân tích cú pháp trên toàn bộ
    hệ thống, bao gồm chèn một dòng trống giữa các trang chiếu trong bản
    trình bày PPTX, sửa lỗi xử lý dòng mới bên trong các đoạn văn trong
    tài liệu Word và thêm dấu đầu dòng vào các mục danh sách.

### Phiên bản 0.5.0 {#version-0.5.0}

-   Đã thêm hỗ trợ cho tài liệu Microsoft Word!
-   Đã thêm hỗ trợ cho các bản trình bày PowerPoint!
-   Đã khắc phục lỗi một số mục menu không bị vô hiệu hóa khi không có
    tài liệu nào đang mở.
-   Đã khắc phục hướng của thanh trượt \"Go to Percent\".
-   Đã khắc phục lỗi mục lục trong sách Epub có đường dẫn tệp được mã
    hóa URL và/hoặc ID đoạn.
-   Đã khắc phục lỗi khoảng trắng bị loại bỏ khỏi các tiêu đề XHTML theo
    những cách kỳ lạ.
-   Đã khắc phục việc xử lý khoảng trắng bên trong các thẻ pre lồng nhau
    trong tài liệu HTML.
-   Các tài liệu HTML và Markdown giờ đây đã hỗ trợ tính năng mục lục !
    Khi bạn tải một tài liệu HTML/Markdown, Paperback sẽ tự động tạo mục
    lục dựa trên cấu trúc các tiêu đề trong tài liệu của bạn và hiển thị
    cho bạn trong hộp thoại Ctrl+T.
-   Các tài liệu HTML giờ đây sẽ có tiêu đề được đặt trong thẻ title,
    nếu thẻ này tồn tại. Nếu không, chúng sẽ tiếp tục sử dụng tên tệp mà
    không có phần mở rộng.
-   Đã chuyển từ UniversalSpeech sang sử dụng vùng trực tiếp để báo cáo
    lời nói. Điều này có nghĩa là các DLL trình đọc màn hình sẽ không
    còn được cung cấp cùng với chương trình nữa, và giờ đây sẽ hỗ trợ
    nhiều trình đọc màn hình hơn, chẳng hạn như Microsoft Narrator.
-   Đã chuyển sang các thư viện nén zip để cho phép mở nhiều loại sách
    epub hơn.
-   Hộp thoại hỏi bạn có muốn mở tài liệu dưới dạng văn bản thuần túy
    hay không đã được thiết kế lại hoàn toàn, và giờ đây nó cho phép bạn
    mở tài liệu dưới dạng văn bản thuần túy, HTML hoặc Markdown.
-   Hộp thoại \"Đi đến phần trăm\" hiện bao gồm một trường văn bản cho
    phép bạn nhập thủ công tỷ lệ phần trăm để chuyển đến.
-   Trình phân tích cú pháp HTML giờ đây sẽ nhận diện dd, dt và dl là
    các thành phần danh sách.
-   Mục lục trong sách EPUB giờ đây sẽ được giữ nguyên chính xác.
-   Khoảng trắng không phân tách Unicode giờ đây được tính đến khi loại
    bỏ các dòng trống.
-   Bạn sẽ không còn bị hỏi cách mở tệp không được nhận diện mỗi lần tải
    tệp đó nữa, mà chỉ lần đầu tiên thôi.

### Phiên bản 0.4.1 {#version-0.4.1}

-   Đã thêm biểu tượng menu bắt đầu tùy chọn vào trình cài đặt.
-   Mục lục giờ đây sẽ gọn gàng hơn trong một số trường hợp, ví dụ nếu
    bạn có một mục con và mục cha có cùng văn bản ở cùng vị trí, giờ đây
    bạn sẽ chỉ thấy mục cha.
-   Đã sửa lỗi mục lục trong một số tài liệu CHM nhất định.
-   Đã sửa lỗi mục lục trong các cuốn sách Epub 3 có chứa đường dẫn
    tuyệt đối trong đó.
-   Các tài liệu CHM giờ đây sẽ hiển thị tiêu đề của chúng theo như đã
    được thiết lập trong tệp siêu dữ liệu .

### Phiên bản 0.4.0 {#version-0.4.0}

-   Đã thêm hỗ trợ tệp CHM!
-   Đã thêm hỗ trợ dấu trang! Bạn có thể tạo bao nhiêu dấu trang trong
    bao nhiêu tài liệu tùy thích. Bạn có thể chuyển tiếp và lùi lại giữa
    các dấu trang bằng phím b và shift+b, đặt dấu trang bằng tổ hợp phím
    control+shift+b, và mở hộp thoại để chuyển đến một dấu trang cụ thể
    bằng tổ hợp phím control+b.
-   Đã thêm trình cài đặt cùng với tệp zip di động! Trình cài đặt sẽ cài
    đặt Paperback vào thư mục Program Files của bạn và tự động thiết lập
    các liên kết tệp cho bạn.
-   Các tệp văn bản có BOM giờ đây sẽ được giải mã chính xác, và BOM
    cũng sẽ không còn được hiển thị ở đầu văn bản nữa.
-   Đã bổ sung nhiều thông tin hơn vào thanh trạng thái. Giờ đây, thanh
    trạng thái sẽ hiển thị dòng hiện tại, ký tự hiện tại và tỷ lệ phần
    trăm đã đọc.
-   Các bình luận HTML, cũng như nội dung của các thẻ script và style,
    sẽ không còn được hiển thị trong kết quả văn bản.
-   Nếu truyền đường dẫn tương đối đến Paperback trên dòng lệnh, chương
    trình sẽ giải quyết đường dẫn đó một cách chính xác.
-   Việc điều chỉnh phần trăm giờ đây được xử lý bằng hộp thoại riêng
    dựa trên thanh trượt, có thể truy cập bằng tổ hợp phím
    Control+Shift+G.
-   Các tài liệu không có tiêu đề hoặc tác giả đã biết giờ đây sẽ luôn
    có giá trị mặc định.
-   Cơ chế lưu vị trí hiện đã thông minh hơn nhiều và chỉ ghi lên đĩa
    khi thực sự cần thiết.
-   Tài liệu mà bạn đang tập trung khi đóng Paperback giờ đây sẽ được
    ghi nhớ ngay cả khi khởi động lại ứng dụng.
-   Dữ liệu nhập vào các hộp thoại \"Đi đến dòng\" và \"Đi đến trang\"
    giờ đây sẽ được kiểm tra nghiêm ngặt hơn.
-   Đã khắc phục lỗi điều hướng mục lục trong các sách EPUB 3 có đường
    dẫn tương đối trong tệp manifest.

### Phiên bản 0.3.0 {#version-0.3.0}

-   Đã khắc phục lỗi mục lục trong các sách epub có tệp manifest được mã
    hóa theo định dạng URL. Đã khắc phục lỗi điều hướng tiêu đề trong
    các tài liệu HTML chứa các ký tự Unicode nhiều byte.
-   Đã khắc phục sự cố điều hướng tiêu đề trong các tài liệu HTML chứa
    các ký tự Unicode đa byte.
-   Đã khắc phục tình trạng sử dụng CPU cao trong các tài liệu có tiêu
    đề dài do sự thoái lui trong wxWidgets.
-   Đã khắc phục lỗi tải tệp văn bản UTF-8.
-   Đã khắc phục lỗi các mục mục lục lồng nhau trong sách EPUB khiến con
    trỏ chuột đặt sai vị trí.
-   Đã khắc phục sự cố ứng dụng bị treo khi thoát trong một số trường
    hợp nhất định.
-   Đã thêm hộp kiểm trong hộp thoại tùy chọn để bật hoặc tắt tính năng
    tự động xuống dòng!
-   Giờ đây, bạn có thể đóng góp cho sự phát triển của Paperback, thông
    qua mục "đóng góp" mới trong menu trợ giúp hoặc qua liên kết "tài
    trợ cho dự án này" ở cuối trang chính của kho lưu trữ GitHub.
-   Các tài liệu Markdown giờ đây sẽ luôn có tiêu đề, và Paperback hiện
    có thể tải hầu như bất kỳ tệp Markdown nào.
-   Các tài liệu PDF giờ đây sẽ luôn có tiêu đề, ngay cả khi thiếu siêu
    dữ liệu.
-   Đã chuyển sang sử dụng thư viện PDF được dùng trong Chromium, giúp
    việc phân tích cú pháp PDF trở nên đáng tin cậy hơn rất nhiều trên
    mọi phương diện.
-   Giờ đây, bạn chỉ có thể chạy một phiên bản Paperback tại một thời
    điểm. Chạy paperback.exe với tên tệp trong khi chương trình đang
    chạy sẽ mở tài liệu đó trong phiên bản đang chạy.
-   Giờ đây, bạn có thể nhấn phím Delete trên một tài liệu trong thanh
    điều khiển tab để đóng tài liệu đó.

### Phiên bản 0.2.1 {#version-0.2.1}

-   Đã thêm tổng số trang vào nhãn trang trong hộp thoại \"Đi đến
    trang\". Đã
-   Cho phép chuyển từ nội dung tài liệu sang danh sách các tài liệu
    đang mở bằng phím Tab.
-   Đã khắc phục lỗi các phím tắt tiêu đề đôi khi mở các tài liệu gần
    đây nếu bạn có đủ số lượng tài liệu đó.
-   Paperback giờ đây sẽ loại bỏ các dấu gạch nối mềm không cần thiết
    khỏi văn bản được xuất ra.
-   Đã khắc phục lỗi điều hướng tiêu đề đôi khi đưa bạn đến ký tự sai.

### Phiên bản 0.2.0 {#version-0.2.0}

-   Đã thêm hỗ trợ tài liệu Markdown!
-   Đã thêm hỗ trợ tài liệu PDF, bao gồm khả năng điều hướng giữa các
    trang!
-   Đã thêm các phím tắt để điều hướng theo tiêu đề trong nội dung HTML,
    bao gồm cả sách epub và tài liệu Markdown. Các phím tắt này được
    thiết kế để hoạt động tương tự như trình đọc màn hình.
-   Đã khắc phục lỗi tải sách epub có tên tệp được mã hóa theo URL trong
    tệp manifest.
-   Đã khắc phục lỗi tải sách epub 3 có XHTML nhúng bên trong.
-   Giờ đây, một thông báo sẽ được đọc lên nếu tài liệu không hỗ trợ mục
    lục hoặc các phần, thay vì các mục menu bị vô hiệu hóa.
-   Đã thêm menu tài liệu gần đây! Hiện tại, menu này lưu trữ 10 tài
    liệu cuối cùng bạn đã mở, và khi nhấn phím Enter vào một tài liệu,
    tài liệu đó sẽ được mở ra để đọc.
-   Đã viết lại hoàn toàn hộp thoại Tìm kiếm, giúp nó dễ sử dụng hơn
    nhiều, đồng thời thêm lịch sử 25 lần tìm kiếm gần nhất và hỗ trợ
    biểu thức chính quy!
-   Các tài liệu đã mở trước đó giờ đây sẽ được ghi nhớ ngay cả khi ứng
    dụng được khởi động lại. Tính năng này có thể được cấu hình thông
    qua mục tùy chọn mới trong menu công cụ .
-   Đã thêm phím tắt Shift+F1 để mở tệp readme trực tiếp trong chính
    Paperback.

### Phiên bản 0.1.0 {#version-0.1.0}

-   Phiên bản phát hành đầu tiên.
