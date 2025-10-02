[Setup]
	AppName=Paperback
	AppVersion=0.4
	AppPublisher=Quin Gillespie
	AppPublisherURL=https://github.com/trypsynth/paperback
	AppSupportURL=https://github.com/trypsynth/paperback/issues
	AppUpdatesURL=https://github.com/trypsynth/paperback/releases
	DefaultDirName={autopf}\Paperback
	DisableProgramGroupPage=yes
	DisableDirPage=yes
	LicenseFile=
	OutputDir=build
	OutputBaseFilename=paperback_setup
	Compression=lzma2
	SolidCompression=yes
	WizardStyle=modern
	UninstallDisplayIcon={app}\paperback.exe
	ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
	Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
	Source: "build\paperback.exe"; DestDir: "{app}"; Flags: ignoreversion
	Source: "build\*.dll"; DestDir: "{app}"; Flags: ignoreversion
	Source: "build\readme.html"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
	Name: "desktopicon"; Description: "Desktop Shortcut"
	Name: "assoc_chm"; Description: "Associate with Compiled HTML Help files (*.chm)"
	Name: "assoc_epub"; Description: "Associate with Epub Books (*.epub)"; Flags: checkedonce
	Name: "assoc_html"; Description: "Associate with HTML Documents (*.htm, *.html, *.xhtml)"
	Name: "assoc_markdown"; Description: "Associate with Markdown Documents (*.md, *.markdown, *.mdx, *.mdown, *.mdwn, *.mkd, *.mkdn, *.mkdown, *.ronn)"
	Name: "assoc_pdf"; Description: "Associate with PDF Documents (*.pdf)"; Flags: checkedonce
	Name: "assoc_text"; Description: "Associate with Text Files (*.txt, *.log)"

[Icons]
	Name: "{autodesktop}\Paperback"; Filename: "{app}\paperback.exe"; Tasks: desktopicon

[Registry]
	; Core Paperback.Document class registration.
	Root: HKCR; Subkey: "Paperback.Document"; ValueType: string; ValueName: ""; ValueData: "Paperback Document"; Flags: uninsdeletekey
	Root: HKCR; Subkey: "Paperback.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\paperback.exe,0"
	Root: HKCR; Subkey: "Paperback.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\paperback.exe"" ""%1"""
	; Compiled HTML Help files.
	Root: HKCR; Subkey: ".chm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_chm
	; Epub Books.
	Root: HKCR; Subkey: ".epub"; ValueType: string; ValueName: ""; ValueData: "Paperback.Document"; Flags: uninsdeletevalue; Tasks: assoc_epub
	; HTML Documents.
	Root: HKCR; Subkey: ".htm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_html
	Root: HKCR; Subkey: ".html\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_html
	Root: HKCR; Subkey: ".xhtml\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_html
	; Markdown Documents.
	Root: HKCR; Subkey: ".markdown\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".md\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mdown\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mdwn\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mdx\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mkd\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mkdn\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".mkdown\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	Root: HKCR; Subkey: ".ronn\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_markdown
	; PDF Documents.
	Root: HKCR; Subkey: ".pdf\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_pdf
	; Text Files.
	Root: HKCR; Subkey: ".log\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_text
	Root: HKCR; Subkey: ".txt\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_text

[Run]
	Filename: "{app}\paperback.exe"; Description: "Launch Paperback"; Flags: nowait postinstall skipifsilent
