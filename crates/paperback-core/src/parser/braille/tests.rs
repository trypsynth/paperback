use super::*;
use crate::util::test_support::TempDir;

fn parse_with(
	parser: &dyn Parser,
	name: &str,
	contents: impl AsRef<[u8]>,
	context: impl FnOnce(ParserContext) -> ParserContext,
) -> Result<Document> {
	let dir = TempDir::new("braille-parser");
	let path = dir.write_str(name, contents);
	parser.parse(&context(ParserContext::new(path)))
}

fn parse(parser: &dyn Parser, name: &str, contents: impl AsRef<[u8]>) -> Document {
	parse_with(parser, name, contents, |context| context).expect("parse braille document")
}

fn page_markers(document: &Document) -> Vec<(usize, String)> {
	document
		.buffer
		.markers
		.iter()
		.filter(|marker| marker.mtype == MarkerType::PageBreak)
		.map(|marker| (marker.position.get(), marker.text.clone()))
		.collect()
}

fn cells(ascii: &str) -> String {
	ascii.bytes().map(ascii_to_cell).collect()
}

fn layout(pages: &[&[&str]], strip_running_heads: bool) -> Layout {
	let pages: Vec<Page> = pages.iter().map(|page| page.iter().map(|line| cells(line)).collect()).collect();
	Layout::of(&pages, strip_running_heads)
}

#[test]
fn braille_ascii_names_each_of_the_64_cells_once() {
	let mut characters = BRAILLE_ASCII.to_vec();
	characters.sort_unstable();
	characters.dedup();
	assert_eq!(characters.len(), 64);
	for (dots, &character) in BRAILLE_ASCII.iter().enumerate() {
		assert_eq!(u32::from(ascii_to_cell(character)), 0x2800 + u32::try_from(dots).unwrap());
	}
}

/// Many BRF files write the letters, and the four signs above them, in lowercase.
#[test]
fn reads_lowercase_as_the_same_cells() {
	assert_eq!(cells("abc `{|}~"), cells("ABC @[\\]^"));
}

#[test]
fn back_translates_with_the_default_table() {
	let document = parse(&BrfParser, "orbit.brf", ",! ,ORBIT ,R1D] #BJ IS ! _W'S MO/\n");
	assert_eq!(document.buffer.content, "The Orbit Reader 20 is the world's most\n");
}

#[test]
fn back_translates_with_the_table_asked_for() {
	let document =
		parse_with(&BrfParser, "g1.brf", "AB\n", |context| ParserContext { braille_table: "en-ueb-g1.ctb", ..context })
			.unwrap();
	assert_eq!(document.buffer.content, "ab\n");
	assert_eq!(parse(&BrfParser, "g2.brf", "AB\n").buffer.content, "about\n");
}

#[test]
fn refuses_a_table_it_does_not_have() {
	let error = parse_with(&BrfParser, "book.brf", "AB\n", |context| ParserContext {
		braille_table: "no-such-table.ctb",
		..context
	})
	.unwrap_err();
	assert!(error.to_string().contains("no-such-table.ctb"), "{error}");
}

/// A paragraph that runs onto the next page is one paragraph, and that page's marker falls on its first word there.
#[test]
fn joins_a_paragraph_across_a_page_and_marks_where_the_page_begins() {
	let document = parse(&BrfParser, "pages.brf", "  HELLO HELLO HELLO HELLO HELLO HELLOS\r\n\x0c_W\r\n\x0c");
	assert_eq!(document.buffer.content, "hello hello hello hello hello hellos world\n");
	assert_eq!(page_markers(&document), [(0, "Page 1".to_string()), (37, "Page 2".to_string())]);
}

/// Older BRF files, such as the Internet Archive's, end their lines with a carriage return alone.
#[test]
fn reads_lines_ended_by_a_carriage_return_alone() {
	let document = parse(&BrfParser, "cr.brf", "  HELLO HELLO HELLO HELLO HELLO HELLOS\r_W\r\r  ,! ,ORBIT\r");
	assert_eq!(document.buffer.content, "hello hello hello hello hello hellos world\nThe Orbit\n");
}

/// French braille ASCII writes the capital sign as ¨ and é as itself, as `fr-bfu-comp6.utb` defines them.
#[test]
fn reads_the_braille_ascii_of_the_tables_country() {
	let french = |context: ParserContext| ParserContext { braille_table: "fr-bfu-comp6.utb", ..context };
	let utf8 = parse_with(&BrfParser, "fr.brf", "¨le ¨président\n", french).unwrap();
	assert_eq!(utf8.buffer.content, "Le Président\n");
	let windows_1252 = parse_with(&BrfParser, "fr.brf", b"\xa8le \xa8pr\xe9sident\n", french).unwrap();
	assert_eq!(windows_1252.buffer.content, "Le Président\n");
}

/// DAISY Pipeline writes Norwegian with Braillo's table, whose capital sign is `*` rather than liblouis's `´`.
#[test]
fn reads_norwegian_written_with_another_countrys_braille_ascii() {
	let norwegian = |context: ParserContext| ParserContext { braille_table: "no-no-g0.utb", ..context };
	let document = parse_with(&BrfParser, "no.brf", "  *HEI *VERDEN, DETTE ER EN BOK.\r\n", norwegian).unwrap();
	assert_eq!(document.buffer.content, "Hei Verden, dette er en bok.\n");
}

/// Danish files are written in code page 850, so æ, ø and å are bytes Windows-1252 has other characters for.
#[test]
fn reads_danish_written_in_code_page_850() {
	let danish = |context: ParserContext| ParserContext { braille_table: "da-dk-g16.ctb", ..context };
	let document =
		parse_with(&BrfParser, "da.brf", b"det skal v\x91re en bog om s\x9b og \x86er.\r\n", danish).unwrap();
	assert_eq!(document.buffer.content, "det skal være en bog om sø og åer.\n");
}

/// French braille ASCII writes the digits as themselves, as `fr-bfu-comp6.utb` defines them, rather than as the lowered North American cells.
#[test]
fn reads_digits_in_the_braille_ascii_of_the_tables_country() {
	let document = parse_with(&BrfParser, "fr.brf", "000 morts\n", |context| ParserContext {
		braille_table: "fr-bfu-comp6.utb",
		..context
	})
	.unwrap();
	assert_eq!(document.buffer.content, "000 morts\n");
}

/// Slovak braille files are written in North American Braille ASCII, although `sk-g1.ctb` defines `,` as dot 2 and `*` as dots 3 and 5 and has no `display` rules.
#[test]
fn reads_slovak_files_as_north_american_braille_ascii() {
	let document = parse_with(&BrfParser, "sk.brf", ",betsabe3 \\a!ko sk+:an* kr*sa\n", |context| ParserContext {
		braille_table: "sk-g1.ctb",
		..context
	})
	.unwrap();
	assert_eq!(document.buffer.content, "Betsabe: ťažko skúšaná krása\n");
}

/// A Czech file in North American Braille ASCII reads as one, rather than in Dotify's Czech braille ASCII, as `cs-g1.ctb` defines the same cells as `sk-g1.ctb` for it; the letters here are shared by both languages.
#[test]
fn reads_czech_files_as_north_american_braille_ascii() {
	let document = parse_with(&BrfParser, "cs.brf", ",autor3 ,angela ,hunt1 \\a!ko sk+:an* kr*sa\n", |context| {
		ParserContext { braille_table: "cs-g1.ctb", ..context }
	})
	.unwrap();
	assert_eq!(document.buffer.content, "Autor: Angela Hunt, ťažko skúšaná krása\n");
}

/// Hungarian braille files are written in North American Braille ASCII, although `hu-hu-g1.ctb` writes `1` as dots 1 and 6 and `@` as dots 4 and 5, and has no cell for `*`. This is a library book's title page, in capitals with `@ [ ( )` for á ó ü ű.
#[test]
fn reads_hungarian_files_as_north_american_braille_ascii() {
	let title_page = concat!(
		"    .ARTHUR .CONAN .DOYLE3\r\n",
		"      .SHERLOCK .HOLMES\r\n",
		"         EML*KIRATAI\r\n",
		"   !.B)N(?I ELBE:*L*SEKZ\r\n",
		"\r\n",
		"    .KRITERION .KQ$VKIAD[1\r\n",
		"       .BUKAREST1 #AIGF\r\n",
		"         .FORD/TOTTA3\r\n",
		"        .KATONA .TAM@S\r\n",
	);
	let document = parse_with(&BrfParser, "hu.brf", title_page, |context| ParserContext {
		braille_table: "hu-hu-g1.ctb",
		..context
	})
	.unwrap();
	assert_eq!(
		document.buffer.content,
		"Arthur Conan Doyle:\nSherlock Holmes\nemlékiratai\n(Bűnügyi elbeszélések)\nKriterion Könyvkiadó,\nBukarest, 1976\nFordította:\nKatona Tamás\n"
	);
}

/// Other Hungarian books are written in lowercase, with `` ` { } `` for á ó ő, as on this library book's title page.
#[test]
fn reads_lowercase_hungarian_files_as_north_american_braille_ascii() {
	let title_page = concat!(
		"   .alan .alexander .milne\r\n",
		"\r\n",
		"          .micimack{\r\n",
		"\r\n",
		".ford/totta3 .karinthy .fri?es\r\n",
		"\r\n",
		"       .m{ra .kq$vkiad{\r\n",
		"\r\n",
		"..isbn3 #igh-#ifc-#dae-#hea-#f\r\n",
		"\r\n",
		"      .braille-v`lto<at\r\n",
		"        ..mv?o:3 #bjbb\r\n",
		"\r\n",
		"          k*tkqtetes\r\n",
		"\r\n",
		"         .els} kqtet\r\n",
	);
	let document = parse_with(&BrfParser, "hu.brf", title_page, |context| ParserContext {
		braille_table: "hu-hu-g1.ctb",
		..context
	})
	.unwrap();
	assert_eq!(
		document.buffer.content,
		"Alan Alexander Milne\nMicimackó\nFordította: Karinthy Frigyes\nMóra Könyvkiadó\nISBN: 978-963-415-851-6\nBraille-változat\nMVGYOSZ: 2022\nkétkötetes\nElső kötet\n"
	);
}

/// Spanish files from ONCE, as NLS lends them, are written in North American Braille ASCII, so a capital sign is `.` and á is `(`. This is a library book's title page.
#[test]
fn reads_spanish_files_as_north_american_braille_ascii() {
	let title_page = concat!(
		"          .EDUARDO .MENDOZA\r\n",
		"\r\n",
		"            .EL REY RECIBE\r\n",
		"\r\n",
		"               .O.N.C.E\r\n",
		"       .SERVICIO .BIBLIOGR(FICO\r\n",
		"          #JHJAE .BARCELONA\r\n",
	);
	let document =
		parse_with(&BrfParser, "es.brf", title_page, |context| ParserContext { braille_table: "es.tbl", ..context })
			.unwrap();
	assert_eq!(
		document.buffer.content,
		"Eduardo Mendoza\nEl rey recibe\nONCE\nServicio Bibliográfico\n08015 Barcelona\n"
	);
}

/// A table's braille ASCII reads a NUL or a tab as the blank cell, so another country's has to as well, or a single one on a title page or a blank line would leave it a character unread and lose it the file.
#[test]
fn reads_nul_and_tabs_as_blank_in_another_countrys_braille_ascii() {
	let line = ".:*P ]:I *G3 .@D@M1 .*VA *S .?URI +JRA ITT VOLT1 DE NEM :[LT SEMMIT'\n";
	let document = parse_with(&BrfParser, "hu.brf", format!("\0\n{line}\t\n{line}"), |context| ParserContext {
		braille_table: "hu-hu-g1.ctb",
		..context
	})
	.unwrap();
	assert!(document.buffer.content.contains("Szép őszi ég: Ádám, Éva"), "{:?}", document.buffer.content);
}

/// UEB's tables display a capital A as the computer braille cell with dot 7, which is no six-dot braille file's.
#[test]
fn reads_capitals_as_north_american_braille_ascii_for_english() {
	assert_eq!(parse(&BrfParser, "upper.brf", "HELLO\n").buffer.content, "hello\n");
}

#[test]
fn reads_a_brf_written_in_unicode_braille() {
	let document = parse(&BrfParser, "unicode.brf", "  ⠓⠑⠇⠇⠕⠀⠓⠑⠇⠇⠕⠀⠓⠑⠇⠇⠕⠀⠓⠑⠇⠇⠕⠀⠓⠑⠇⠇⠕⠀⠓⠑⠇⠇⠕⠎\r\n⠸⠺\r\n");
	assert_eq!(document.buffer.content, "hello hello hello hello hello hellos world\n");
}

#[test]
fn joins_wrapped_lines_and_words_broken_at_a_hyphen() {
	let layout = layout(&[&["  SELF-", "3TA9$ BOOK ) ! SAME DEGREE ( LOVE & C>E", "R1D]"]], true);
	assert_eq!(layout.paragraphs, [cells("SELF-3TA9$ BOOK ) ! SAME DEGREE ( LOVE & C>E R1D]")]);
}

#[test]
fn reads_guide_dots_as_one_space() {
	let contents = layout(
		&[&[
			"  ,HOT ,COCOA \"\"\"\"\"\"\"\"\"\"\"\" #DF",
			"  ,A LONG ,T\\LE",
			"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"",
			"#DG",
		]],
		true,
	);
	assert_eq!(contents.paragraphs, [cells(",HOT ,COCOA #DF"), cells(",A LONG ,T\\LE #DG")]);
	let nearly_full = layout(&[&["  ,SALADS & ,SALAD ,DRESS+S \"\" #BFA", "  ,WORD\"\"S"]], true);
	assert_eq!(nearly_full.paragraphs, [cells(",SALADS & ,SALAD ,DRESS+S #BFA"), cells(",WORD\"\"S")]);
}

/// A contents page starts each entry at the margin and indents its run-on lines, the other way round from paragraphs, so an entry ends at its page number.
#[test]
fn ends_a_contents_entry_after_its_guide_dots() {
	let contents = layout(
		&[&[
			",,TABLE ,,SERVICE & ,,ETIQUETTE \"\"\"\" #CA",
			",,BEV]AGES \"\"\"\"\"\"\"\"\"\"\"\"\"\"\" #DD",
			",A LONG ,T\\LE",
			"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"",
			"#DG",
			",SUMM>Y \"\"\"\"\"\"\"\"\" #DH",
			"    ,EQUIVAL5T ,WEI<TS &",
			"  ,MEASURES \"\"\"\"\"\"\"\" #A",
			",,M1L ,,PLANN+3",
			"    ,M1L ,PLANN+ \"\"\"\"\"\" #AA",
			"  ,AN INDENT$ ,P>AGRAPH",
		]],
		true,
	);
	assert_eq!(
		contents.paragraphs,
		[
			cells(",,TABLE ,,SERVICE & ,,ETIQUETTE #CA"),
			cells(",,BEV]AGES #DD"),
			cells(",A LONG ,T\\LE #DG"),
			cells(",SUMM>Y #DH"),
			cells(",EQUIVAL5T ,WEI<TS & ,MEASURES #A"),
			cells(",,M1L ,,PLANN+3"),
			cells(",M1L ,PLANN+ #AA"),
			cells(",AN INDENT$ ,P>AGRAPH")
		]
	);
}

/// Poetry contents run an entry over onto lines indented two cells beyond the deepest entry, as in this collection from NLS, where the entry's first line has no page number, unlike a heading's entries under it.
#[test]
fn runs_a_contents_entry_over_onto_deeper_lines() {
	let contents = layout(
		&[&[
			"  XXX4 ,FI% @& ,WILDLIFE \"\"\"\"\"\" #BFB",
			"  XXXI4 ,D\\BLE ,VIEW ( !",
			"    ,ADIRONDACKS Z ,REFLECT$ OV]",
			"    ,P>K \"\"\"\"\"\"\"\"\"\"\" #BFD",
			"  XXXII4 ,! ,VALKYRIE \"\"\"\"\"\" #BFE",
			",! ,ABSURD ,MAN ,SUITE \"\"\"\"\"\" #BFF",
			"  ,AUGU/9IAN \"\"\"\"\"\"\"\"\"\" #BFG",
			",,P>T ,,TWO",
			"  ,A ,P\\M \"\"\"\"\"\"\"\"\"\"\"\" #BFH",
			"  ,ANO!R ,P\\M \"\"\"\"\"\"\"\" #BFI",
		]],
		true,
	);
	assert_eq!(
		contents.paragraphs,
		[
			cells("XXX4 ,FI% @& ,WILDLIFE #BFB"),
			cells("XXXI4 ,D\\BLE ,VIEW ( ! ,ADIRONDACKS Z ,REFLECT$ OV] ,P>K #BFD"),
			cells("XXXII4 ,! ,VALKYRIE #BFE"),
			cells(",! ,ABSURD ,MAN ,SUITE #BFF"),
			cells(",AUGU/9IAN #BFG"),
			cells(",,P>T ,,TWO"),
			cells(",A ,P\\M #BFH"),
			cells(",ANO!R ,P\\M #BFI"),
		]
	);
}

/// An entry that leaves no room for guide dots ends in blank cells and its page number, as in this anthology from NLS.
#[test]
fn ends_a_contents_entry_at_a_page_number_after_blank_cells() {
	let contents = layout(
		&[&[
			"        ,,,\"P I3 LOVE & C>E,' \"\"\"\" #AB",
			".1,UNSPOOL+ ,NICOLE ,LEE ,S*RO$]   #AB",
			".7,ROSA%>ON ,T1*ES ,ME TO ,BR1!3",
			"  ,ON ,ANIMALS1 ,4ABILITIES1 &",
			"  ,9TIMACIES.' ,ADA ,HUBRIG \"\"\"\"\"\" #BG",
			".7,! ,LA/ ,WALK.' ,MELISSA ,HUNG   #CI",
			".7,! ,5D \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\" #EA",
		]],
		true,
	);
	assert_eq!(
		contents.paragraphs,
		[
			cells(",,,\"P I3 LOVE & C>E,' #AB"),
			cells(".1,UNSPOOL+ ,NICOLE ,LEE ,S*RO$] #AB"),
			cells(".7,ROSA%>ON ,T1*ES ,ME TO ,BR1!3 ,ON ,ANIMALS1 ,4ABILITIES1 & ,9TIMACIES.' ,ADA ,HUBRIG #BG"),
			cells(".7,! ,LA/ ,WALK.' ,MELISSA ,HUNG #CI"),
			cells(".7,! ,5D #EA"),
		]
	);
}

#[test]
fn keeps_the_space_after_a_dash_that_ends_a_line() {
	let layout = layout(
		&[&["  ,! HAIR ,- ,BOBI'S THICK1 %9Y COAT,-", ",CAN'T ,TOM $ITS ! L;GE/ LIV+ DOG M]E--", "?AN ANY O!R4"]],
		true,
	);
	assert_eq!(
		layout.paragraphs,
		[cells(",! HAIR ,- ,BOBI'S THICK1 %9Y COAT,- ,CAN'T ,TOM $ITS ! L;GE/ LIV+ DOG M]E-- ?AN ANY O!R4")]
	);
}

#[test]
fn starts_a_paragraph_at_an_indent_or_a_blank_line() {
	let layout = layout(&[&["  ,WE WANT TO IMBUE ? BOOK ) ! SAME", "DEGREE4", "  ?REE", "", "F\\R"]], true);
	assert_eq!(layout.paragraphs, [cells(",WE WANT TO IMBUE ? BOOK ) ! SAME DEGREE4"), cells("?REE"), cells("F\\R")]);
}

/// Embossed braille wraps a line only where the next word doesn't fit, so a line with room left for it ends there on purpose, whatever the indents: as each line of a poem does, here from a collection from NLS, which runs a long line over two cells further in.
#[test]
fn keeps_a_line_that_ends_on_purpose() {
	let layout = layout(
		&[&[
			",E S (T5 \"O HAS TO MAKE A S.D",
			"( T]RIBLE PA9 Z ?\\< 9 B.D.S WOODS",
			"\"O'S C&LELIT D9N] AT ,ELEV5 ,MADISON",
			"  ,P>K",
			") XS :ITE L95S SAD Z MILK & SILV]W>E",
		]],
		true,
	);
	assert_eq!(
		layout.paragraphs,
		[
			cells(",E S (T5 \"O HAS TO MAKE A S.D"),
			cells("( T]RIBLE PA9 Z ?\\< 9 B.D.S WOODS"),
			cells("\"O'S C&LELIT D9N] AT ,ELEV5 ,MADISON ,P>K"),
			cells(") XS :ITE L95S SAD Z MILK & SILV]W>E"),
		]
	);
}

/// A paragraph whose last line has no room for the next paragraph's first word still ends there: the next one starts at the indent of its first line, where its own words would go on at the margin, as in this novel from NLS.
#[test]
fn starts_a_paragraph_after_one_whose_last_line_is_full() {
	let layout = layout(
		&[&[
			"  8,NO4 ,X'S N L X'S .18 W$D+1 ,$DIE40",
			"  8,! 9VITA;N SAYS .1:ITE .1TIE1",
			",CECILIA40",
			"  8,X'S .1,S+APORE1 ,$DIE1 & NO \"O \"!",
		]],
		true,
	);
	assert_eq!(
		layout.paragraphs,
		[
			cells("8,NO4 ,X'S N L X'S .18 W$D+1 ,$DIE40"),
			cells("8,! 9VITA;N SAYS .1:ITE .1TIE1 ,CECILIA40"),
			cells("8,X'S .1,S+APORE1 ,$DIE1 & NO \"O \"!"),
		]
	);
}

/// A book can leave a cell blank down the left of every line, so a paragraph's wrapped lines are indented as well: it's still told by its first line being indented further, as in this book from NLS.
#[test]
fn reads_a_paragraph_whose_lines_are_all_indented() {
	let layout = layout(
		&[&[
			"   ,S =A FEW \"DS WE _H BE5 CAT*+ UP 9",
			" E>LY-MORN+ 3V]S,N :ILE SITT+ 2S ! POOL",
			" (! ,$ISON ,WA?ALL ,HOTEL 9 ,JACKSON1",
			"   ,MISSISSIPPI4",
		]],
		true,
	);
	assert_eq!(
		layout.paragraphs,
		[
			cells(
				",S =A FEW \"DS WE _H BE5 CAT*+ UP 9 E>LY-MORN+ 3V]S,N :ILE SITT+ 2S ! POOL (! ,$ISON ,WA?ALL ,HOTEL 9 ,JACKSON1"
			),
			cells(",MISSISSIPPI4"),
		]
	);
}

/// A file that never indents a paragraph, as DAISY Pipeline's BRFs don't, is one long paragraph.
#[test]
fn reads_a_file_that_never_indents_a_paragraph() {
	let document = parse_with(&BrfParser, "long.brf", "CAT AND DOG\n".repeat(1000), |context| ParserContext {
		braille_table: "en-us-g1.ctb",
		..context
	})
	.unwrap();
	assert_eq!(document.buffer.content, format!("{}\n", ["cat and dog"; 1000].join(" ")));
}

/// A sidebar is boxed in by a line of dots 2356 above it and one of dots 1245 below, which are only formatting, as in this cookbook from NLS.
#[test]
fn reads_a_box_without_its_box_lines() {
	let layout = layout(
		&[&[
			"  ,LEONEL'S NEI<BORS SD X BE/3 ? OLD DOG",
			"DID .1\"S?+ \"R4",
			"",
			"7777777777777777777777777777777777777777",
			"^7,FATTY ,ACIDS ,HELP ,HAIR ,GR[3^' ,F !",
			"F/ MO;T WE MET ,BOBI1 \"O ?+ T /RUCK U 0",
			"GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG",
			"  ,WE WANT TO IMBUE ? BOOK ) ! SAME",
		]],
		true,
	);
	assert_eq!(
		layout.paragraphs,
		[
			cells(",LEONEL'S NEI<BORS SD X BE/3 ? OLD DOG DID .1\"S?+ \"R4"),
			cells("^7,FATTY ,ACIDS ,HELP ,HAIR ,GR[3^' ,F ! F/ MO;T WE MET ,BOBI1 \"O ?+ T /RUCK U 0"),
			cells(",WE WANT TO IMBUE ? BOOK ) ! SAME"),
		]
	);
}

#[test]
fn keeps_a_centered_heading_apart_from_the_text_after_it() {
	let layout = layout(&[&["          ,! ,ORBIT", ",! ,ORBIT IS"]], true);
	assert_eq!(layout.paragraphs, [cells(",! ,ORBIT"), cells(",! ,ORBIT IS")]);
}

#[test]
fn drops_page_numbers_and_padding() {
	let layout = layout(
		&[&[
			"                                      #A",
			"  ,LEONEL'S NEI<BORS SD X BE/3 ? OLD DOG  ",
			"_H BE5       #A#B",
			"",
			"",
		]],
		true,
	);
	assert_eq!(layout.paragraphs, [cells(",LEONEL'S NEI<BORS SD X BE/3 ? OLD DOG _H BE5")]);
}

/// A continuation page numbers itself with a letter before the number, as in ⠁⠼⠁.
#[test]
fn drops_a_continuation_page_number() {
	assert_eq!(without_page_number(&cells("SERVICES HAVE SOME       A#A")), cells("SERVICES HAVE SOME"));
	assert_eq!(without_page_number(&cells("IN #ABC YE>S")), cells("IN #ABC YE>S"));
}

#[test]
fn drops_a_running_head_only_when_asked() {
	let pages: &[&[&str]] = &[
		&["  ,WE WANT TO IMBUE ? BOOK ) ! SAME"],
		&["   ,H", "DEGREE ( LOVE & C>E ! ,=\"E ,PETS WE'VE"],
		&["   ,H", "DESCRIB$ 5JOY$ \"?\\T _! L;G LIVES4 ,\"? A"],
		&["   ,H", "J\\RNEY4"],
	];
	assert_eq!(
		layout(pages, true).paragraphs,
		[cells(
			",WE WANT TO IMBUE ? BOOK ) ! SAME DEGREE ( LOVE & C>E ! ,=\"E ,PETS WE'VE DESCRIB$ 5JOY$ \"?\\T _! L;G LIVES4 ,\"? A J\\RNEY4"
		)]
	);
	// Kept, a running head is a line of its own, which the text below it doesn't wrap onto.
	assert_eq!(
		layout(pages, false).paragraphs,
		[
			cells(",WE WANT TO IMBUE ? BOOK ) ! SAME"),
			cells(",H"),
			cells("DEGREE ( LOVE & C>E ! ,=\"E ,PETS WE'VE"),
			cells(",H"),
			cells("DESCRIB$ 5JOY$ \"?\\T _! L;G LIVES4 ,\"? A"),
			cells(",H"),
			cells("J\\RNEY4"),
		]
	);
}

#[test]
fn marks_pages_without_text_where_the_next_text_starts() {
	let layout = layout(&[&["  ,WE WANT TO IMBUE ? BOOK ) ! SAME"], &[], &["DEGREE4"], &[]], true);
	assert_eq!(layout.page_starts, [Some((0, 0)), Some((0, 34)), Some((0, 34)), None]);
}

#[test]
fn separates_a_print_page_line_from_the_text_around_it() {
	let layout = layout(&[&["  ONE", "-----------------------#BC", "TWO"]], true);
	assert_eq!(layout.paragraphs, [cells("ONE"), cells("-----------------------#BC"), cells("TWO")]);
}

#[test]
fn titles_a_brf_after_its_file() {
	let document = parse(&BrfParser, "The Orbit Reader.BRF", "A\n");
	assert_eq!(document.title, "The Orbit Reader");
}

fn pef(meta: &str, volumes: &str) -> String {
	format!(
		r#"<?xml version="1.0" encoding="UTF-8"?><pef xmlns="http://www.daisy.org/ns/2008/pef" version="2008-1"><head><meta xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:format>application/x-pef+xml</dc:format>{meta}</meta></head><body>{volumes}</body></pef>"#
	)
}

fn volume(pages: &[&[&str]]) -> String {
	let mut markup = String::new();
	for rows in pages {
		markup.push_str("<page>");
		for row in *rows {
			markup.push_str("<row>");
			markup.push_str(&cells(row));
			markup.push_str("</row>");
		}
		markup.push_str("</page>");
	}
	let pages = markup;
	format!(r#"<volume cols="28" rows="4" rowgap="0" duplex="false"><section>{pages}</section></volume>"#)
}

#[test]
fn reads_pef_rows_as_lines_and_pages() {
	let document = parse(
		&PefParser,
		"ncbi.pef",
		pef("", &volume(&[&[",ABOUT ,,NCBI       #A", "  ,SOME FACTS"], &["  HELLO       A#A"]])),
	);
	assert_eq!(document.buffer.content, "About NCBI\nSome facts\nhello\n");
	assert_eq!(page_markers(&document), [(0, "Page 1".to_string()), (22, "Page 2".to_string())]);
	assert!(document.toc_items.is_empty(), "a single volume needs no table of contents");
}

#[test]
fn ignores_line_breaks_inside_pef_rows() {
	let row = |text: &str| format!("<row>{}\n</row>\n", cells(text));
	let volumes = format!(
		r#"<volume cols="40" rows="4" rowgap="0" duplex="false"><section><page>{}{}</page></section></volume>"#,
		row("  HELLO HELLO HELLO HELLO HELLO HELLOS"),
		row("_W")
	);
	assert_eq!(
		parse(&PefParser, "rows.pef", pef("", &volumes)).buffer.content,
		"hello hello hello hello hello hellos world\n"
	);
}

#[test]
fn lists_pef_volumes_in_the_table_of_contents() {
	let document =
		parse(&PefParser, "two.pef", pef("", &(volume(&[&["  ONE"], &["  TWO"]]) + &volume(&[&["  ?REE"]]))));
	let toc: Vec<_> = document.toc_items.iter().map(|item| (item.name.as_str(), item.offset.get())).collect();
	assert_eq!(toc, [("Volume 1", 0), ("Volume 2", 8)]);
}

#[test]
fn takes_title_and_author_from_pef_metadata() {
	let meta = "<dc:title>The Fall of the House of Usher</dc:title><dc:creator>Edgar Allan Poe</dc:creator>";
	let document = parse(&PefParser, "usher.pef", pef(meta, &volume(&[&["A"]])));
	assert_eq!(document.title, "The Fall of the House of Usher");
	assert_eq!(document.author, "Edgar Allan Poe");
}

#[test]
fn titles_a_pef_without_one_after_its_file() {
	let document = parse(&PefParser, "about ncbi.pef", pef("", ""));
	assert_eq!(document.title, "about ncbi");
}

#[test]
fn refuses_xml_that_is_not_pef() {
	let result = parse_with(
		&PefParser,
		"other.pef",
		r#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"/>"#,
		|context| context,
	);
	assert!(result.is_err());
}

#[test]
fn lists_the_tables_people_choose_by_name() {
	let tables = braille_tables();
	let ueb = tables.iter().find(|table| table.file_name == DEFAULT_TABLE).expect("the default table is listed");
	assert!(ueb.display_name.contains("nified"), "{}", ueb.display_name);
	assert!(
		tables.iter().all(|table| {
			!std::path::Path::new(table.file_name)
				.extension()
				.is_some_and(|extension| extension.eq_ignore_ascii_case("cti") || extension.eq_ignore_ascii_case("uti"))
		}),
		"included pieces are not offered"
	);
	let us =
		tables.iter().find(|table| table.file_name == "en-us-g2.ctb").expect("tables without metadata are listed too");
	assert_eq!(us.display_name, "English, U.S. Grade 2 (ABAE) table");
	assert!(tables.windows(2).all(|pair| pair[0].display_name.to_lowercase() <= pair[1].display_name.to_lowercase()));
	let slovak = tables.iter().position(|table| table.file_name == "sk-g1.ctb").expect("Slovak grade 1 is listed");
	let unnamed = tables.iter().position(|table| table.file_name == "sk-sk-g1.utb").expect("unnamed tables are listed");
	assert!(unnamed.abs_diff(slovak) < 5, "a table named by its file sits among the named ones for its language");
	assert_eq!(tables[slovak].label(), "Slovak braille (sk-g1.ctb)");
	assert_eq!(tables[unnamed].label(), "sk-sk-g1.utb");
	assert_eq!(table_name(DEFAULT_TABLE), Some(DEFAULT_TABLE));
	assert_eq!(table_name("no-such-table.ctb"), None);
}

#[rstest::rstest]
#[case("en-Brai-US", "UEB grade2", Some(6), Some("en-ueb-g2.ctb"))]
#[case("en-Brai-US", "UEB grade1", Some(6), Some("en-ueb-g1.ctb"))]
#[case("en-Brai-US", "EBAE grade2", None, Some("en-us-g2.ctb"))]
#[case("de-Brai", "German grade2", Some(6), Some("de-g2.ctb"))]
// liblouis marks only the detailed German grade 0 and 1 tables as reading both ways.
#[case("de-Brai", "German grade1", None, Some("de-g1-detailed.ctb"))]
#[case("de-Brai", "German grade0", None, Some("de-g0-detailed.utb"))]
#[case("fr-Brai", "French grade2", None, Some("fr-bfu-g2.ctb"))]
#[case("fr-Brai", "French uncontracted no-grade", None, Some("fr-bfu-comp6.utb"))]
#[case("nl-Brai-NL", "Dutch grade0", Some(6), Some("nl-NL-g0.utb"))]
#[case("nl-Brai", "Dutch no-grade", None, Some("nl-NL-g0.utb"))]
#[case("xx-Brai", "Unknown grade2", None, None)]
fn finds_the_table_an_ebraille_book_names(
	#[case] language: &str,
	#[case] system: &str,
	#[case] cells: Option<u8>,
	#[case] table: Option<&str>,
) {
	assert_eq!(table_for_code(BrailleCode { language, system, cells }), table);
}
