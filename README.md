<h1>📌 This program is designed to translate text into the correct layout when typing is incorrect.📌 </h1>

<img alt="Icon" src="https://github.com/jestsi/TranslateUncorrectLayout/blob/master/assets/header.png?raw=true" />
<h3> Example <code> ghbdtn -> привет </code> Just (by default) pressing Alt + T the line will translate the layout. </h3>

---

### ⚠️ It is desirable that there is a `culture_info.json` file near the exe

#### Or use the `--culture-file %path%` option to specify the location of the `culture_info.json` file

Arguments can be specified at startup.
If any value of the parameter is not correct, it will be replaced by the standard one.

+ `--key|-k {a-z}`  (Default 'T') Specifies the key for the key combination that causes the translation.
+ `--sp-key|-spk Alt/Shift/Ctrl/Win` (Default 'Alt') Specifies an additional key for the keyboard shortcut that causes the translation.
+ `--culture-file|-cf %path%` (Default './') Specifies the location of the layout remapping configuration file
+ `--console-hide|-ch true/false`  (Default 'true') Hide the console window if the value is true, but if an error occurs, the console window will be shown
+ `--culture-generate|-cg true/false`  (Default 'true') Generate `culture_info.json` 
+ `--one-key|-ok true/false` (Default 'false') Use for disable shortcut and enable translate only on one key

#### ⚒️ TODO ⚒️
+ Add app in tray / create another GUI repository based on this
