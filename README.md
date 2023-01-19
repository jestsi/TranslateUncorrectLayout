📌<h1> This program is designed to translate text into the correct layout when typing is incorrect. </h1>📌
<h3> Example <code> ghbdtn -> привет </code> Just (by default) pressing Alt + T the line will translate the layout. </h3>

---

### ⚠️ It is imperative that at startup there is a file `culture_info.json` near the exe

#### Or use the `--culture-file %path%` option to specify the location of the `culture_info.json` file

Arguments can be specified at startup.
If any value of the parameter is not correct, it will be replaced by the standard one.

+ <code> --key {a-z} </code> (Default 'T') Specifies the key for the key combination that causes the translation.
+ <code> --sp-key Alt/Shift/Ctrl/Win </code> (Default 'Alt') Specifies an additional key for the keyboard shortcut that causes the translation.
+ <code> --culture-file %path% </code> (Default './') Specifies the location of the layout remapping configuration file
+ <code> --console-hide true/false </code> (Default 'true') Hide the console window if the value is true, but if an error occurs, the console window will be shown
