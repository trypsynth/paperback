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
Name: "desktopicon"; Description: "Create a desktop shortcut"; GroupDescription: "Additional icons:"

[Icons]
Name: "{autodesktop}\Paperback"; Filename: "{app}\paperback.exe"; Tasks: desktopicon

[Registry]
Root: HKCR; Subkey: ".epub"; ValueType: string; ValueName: ""; ValueData: "Paperback.Document"; Flags: uninsdeletevalue
Root: HKCR; Subkey: "Paperback.Document"; ValueType: string; ValueName: ""; ValueData: "Paperback Document"; Flags: uninsdeletekey
Root: HKCR; Subkey: "Paperback.Document\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: "{app}\paperback.exe,0"
Root: HKCR; Subkey: "Paperback.Document\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\paperback.exe"" ""%1"""
Root: HKCR; Subkey: ".pdf\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".html\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".htm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".md\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".txt\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".log\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue
Root: HKCR; Subkey: ".chm\OpenWithProgids"; ValueType: string; ValueName: "Paperback.Document"; ValueData: ""; Flags: uninsdeletevalue

[Run]
Filename: "{app}\paperback.exe"; Description: "Launch Paperback"; Flags: nowait postinstall skipifsilent
