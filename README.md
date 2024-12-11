## What's This
A wallpaper tool, which can get Bing Daily picture as the UHD version and set it to desktop background.

## Getting Started
- Press Win + R, type regedit and press Enter to open the registry editor.
- Find the following path:

For all users:
HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Run

For the current user:
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run

- Right-click the right blank area, select “New” > “String Value”, and name it as the program name.
Double-click the newly created string value and fill in the full path of the program into the “Numeric Data” field.