# Medley
Medleyは，LaTeXの数式環境を再現することで，
複雑な数式の計算を実現したCLI電卓です．

## 導入と削除
### 動作環境
Medleyは，Rust製のWindows専用ソフトウェアです．  
現時点では，単体のCLI用バイナリファイルとして構築されているため，
USBメモリ等に入れて持ち運ぶことも可能です．

### 導入
1. GitHubの[Releases](https://github.com/NXVZBGBFBEN/Medley/releases)ページにアクセスします．
2. **Latest**タグがついているリリースの，**Assets**タブを開き，`medley.7z`をダウンロードします．
3. ダウンロードしてきた`medley.7z`を解凍します．
4. 中に入っている`medley.exe`が本体です．

### 削除
1. `medley.exe`を削除します．

## 使い方
### 起動
`cmd.exe`やPowerShellから，`medley.exe`を実行することで起動できます．
(`medley.exe`をダブルクリックすることでも起動できます)

### 基本操作
Medley起動後，
```
This is Medley, Version x.x.x-<phase> (<release_date>)
>
```
と表示され，入力待ちの状態となります．  
ここにコマンドを打ち込むことで数式を入力できます．(半角スペースは無視されます)  
コマンドの詳細は，同梱の`manual.pdf`を参照してください．  
Enterキーで計算が行われ，結果が下に出力されます．

### 終了
次のように入力すると，Medleyを終了することができます．
```
> exit
```
