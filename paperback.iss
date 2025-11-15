[Setup]
	AppName=Paperback
	AppVersion=0.6.1
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
	Source: "build\langs\*"; DestDir: "{app}\langs"; Flags: ignoreversion recursesubdirs createallsubdirs

[Tasks]
	Name: "desktopicon"; Description: "Desktop Shortcut"
	Name: "startmenuicon"; Description: "Start Menu Shortcut"; Flags: unchecked
	Name: "assoc_chm"; Description: "Associate with Compiled HTML Help files (*.chm)"; Flags: unchecked
	Name: "assoc_docx"; Description: "Associate with Word Documents (*.docx, *.docm)"; Flags: unchecked
	Name: "assoc_epub"; Description: "Associate with Epub Books (*.epub)"
	Name: "assoc_fb2"; Description: "Associate with FB2 Books (*.fb2)"; Flags: unchecked
	Name: "assoc_html"; Description: "Associate with HTML Documents (*.htm, *.html, *.xhtml)"; Flags: unchecked
	Name: "assoc_markdown"; Description: "Associate with Markdown Documents (*.md, *.markdown, *.mdx, *.mdown, *.mdwn, *.mkd, *.mkdn, *.mkdown, *.ronn)"; Flags: unchecked
	Name: "assoc_odp"; Description: "Associate with OpenDocument presentations (*.odp)"; Flags: unchecked
	Name: "assoc_odt"; Description: "Associate with OpenDocument text files (*.odt)"; Flags: unchecked
	Name: "assoc_pdf"; Description: "Associate with PDF Documents (*.pdf)"
	Name: "assoc_pptx"; Description: "Associate with PowerPoint Presentations (*.pptx, *.pptm)"; Flags: unchecked
	Name: "assoc_text"; Description: "Associate with Text Files (*.txt, *.log)"; Flags: unchecked

[Icons]
	Name: "{autodesktop}\Paperback"; Filename: "{app}\paperback.exe"; Tasks: desktopicon
	Name: "{autoprograms}\Paperback"; Filename: "{app}\paperback.exe"; Tasks: startmenuicon

[Registry]
	; Core Paperback.Document class registration.
	Root: HKCR; Subkey: "Paperback.Document"; ValueType: string; ValueName: ""; ValueData: "Paperback Document"; Flags: uninsdeletekey
	Root: HKCR; Subkey: "Paperback.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\paperback.exe,0"
	Root: HKCR; Subkey: "Paperback.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\paperback.exe"" ""%1"""
	; Compiled HTML Help files.
	Root: HKCR; Subkey: ".chm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_chm
	; Word Documents.
	Root: HKCR; Subkey: ".docx\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_docx
	Root: HKCR; Subkey: ".docm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_docx
	; Epub Books.
	Root: HKCR; Subkey: ".epub"; ValueType: string; ValueName: ""; ValueData: "Paperback.Document"; Flags: uninsdeletevalue; Tasks: assoc_epub
	; FB2 Books.
	Root: HKCR; Subkey: ".fb2"; ValueType: string; ValueName: ""; ValueData: "Paperback.Document"; Flags: uninsdeletevalue; Tasks: assoc_fb2
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
	; OpenDocument presentation files.
	Root: HKCR; Subkey: ".odp\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_odp
	; OpenDocument text files.
	Root: HKCR; Subkey: ".odt\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_odt
	; PDF Documents.
	Root: HKCR; Subkey: ".pdf\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_pdf
	; PowerPoint Presentations.
	Root: HKCR; Subkey: ".pptx\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_pptx
	Root: HKCR; Subkey: ".pptm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_pptx
	; Text Files.
	Root: HKCR; Subkey: ".log\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_text
	Root: HKCR; Subkey: ".txt\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue; Tasks: assoc_text

[Run]
	Filename: "{app}\paperback.exe"; Description: "Launch Paperback"; Flags: nowait postinstall skipifsilent
	Filename: "{app}\readme.html"; Description: "View readme.html"; Flags: shellexec postinstall skipifsilent unchecked
