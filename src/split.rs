use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::debug;
use markdown::{
    mdast::{
        Node,
        Node::{Heading, Root},
    },
    to_mdast, ParseOptions,
};

/// Split a markdown text into sections based on headings
///
/// # Arguments
///
/// * `text`: A string slice containing the markdown text to split.
///
/// # Returns
///
/// A vector of string slices, each containing a section of the original markdown text.
///
/// # Errors
///
/// Returns an error if the markdown text cannot be parsed by the `markdown` crate.
pub fn split<'a>(text: &'a str, options: Option<&ParseOptions>) -> Result<Vec<&'a str>> {
    let options = if let Some(o) = options { o } else { &ParseOptions::gfm() };
    let ast = to_mdast(text, options).map_err(|e| anyhow!("{e}"))?;
    let mut split_points = find_split_points(&ast);

    // The very last split point is always the end of the text.
    split_points.push(text.len());
    debug!("Split points: {split_points:?}");

    let sections: Vec<&str> = split_points
        .iter()
        .tuple_windows()
        .map(|(start, end)| &text[*start..*end])
        .collect::<_>();
    debug!("Found {} sections", sections.len());

    Ok(sections)
}

/// Find the offsets of headings in an AST, and use them as split points for the text.
fn find_split_points(node: &Node) -> Vec<usize> {
    let mut split_points = vec![];

    fn traverse(node: &Node, split_points: &mut Vec<usize>) {
        match node {
            Root(root) => {
                root.children.iter().for_each(|c| traverse(c, split_points));
            }
            Heading(heading) if heading.position.as_ref().is_some() => {
                split_points.push(heading.position.as_ref().unwrap().start.offset);
            }
            _ => {}
        }
    }
    traverse(node, &mut split_points);

    // The very first split point should always be 0 (the start of the text.)
    if let Some(&first) = split_points.first() {
        if first != 0 {
            split_points.insert(0, 0);
        }
    }

    split_points
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_en() {
        let text = read_to_string("tests/fixtures/ch01-01-installation.en.md").unwrap();

        let sections = split(&text, None).unwrap();
        assert_eq!(sections.len(), 7);
        assert_eq!(
            sections[0],
            r#"<!-- This is from https://github.com/rust-lang/book/blob/98dd2c1d7fbef0fdd4a377e4dbb7af71bbdc9bae/src/ch01-01-installation.md -->

"#
        );
        assert_eq!(
            sections[1],
            r#"## Installation

The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.

> Note: If you prefer not to use `rustup` for some reason, please see the
> [Other Rust Installation Methods page][otherinstall] for more options.

The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions because Rust often improves error messages and
warnings. In other words, any newer, stable version of Rust you install using
these steps should work as expected with the content of this book.

> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type the `$` character; it’s the command line prompt shown to
> indicate the start of each command. Lines that don’t start with `$` typically
> show the output of the previous command. Additionally, PowerShell-specific
> examples will use `>` rather than `$`.

"#
        );

        assert_eq!(
            sections[2],
            r#"### Installing `rustup` on Linux or macOS

If you’re using Linux or macOS, open a terminal and enter the following command:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:

```text
Rust is installed now. Great!
```

You will also need a *linker*, which is a program that Rust uses to join its
compiled outputs into one file. It is likely you already have one. If you get
linker errors, you should install a C compiler, which will typically include a
linker. A C compiler is also useful because some common Rust packages depend on
C code and will need a C compiler.

On macOS, you can get a C compiler by running:

```console
$ xcode-select --install
```

Linux users should generally install GCC or Clang, according to their
distribution’s documentation. For example, if you use Ubuntu, you can install
the `build-essential` package.

"#
        );
        assert_eq!(
            sections[3],
            r#"### Installing `rustup` on Windows

On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
be prompted to install Visual Studio. This provides a linker and the native
libraries needed to compile programs. If you need more help with this step, see
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]

The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.

"#
        );
        assert_eq!(
            sections[4],
            r#"### Troubleshooting

To check whether you have Rust installed correctly, open a shell and enter this
line:

```console
$ rustc --version
```

You should see the version number, commit hash, and commit date for the latest
stable version that has been released, in the following format:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

If you see this information, you have installed Rust successfully! If you don’t
see this information, check that Rust is in your `%PATH%` system variable as
follows.

In Windows CMD, use:

```console
> echo %PATH%
```

In PowerShell, use:

```powershell
> echo $env:Path
```

In Linux and macOS, use:

```console
$ echo $PATH
```

If that’s all correct and Rust still isn’t working, there are a number of
places you can get help. Find out how to get in touch with other Rustaceans (a
silly nickname we call ourselves) on [the community page][community].

"#
        );
        assert_eq!(
            sections[5],
            r#"### Updating and Uninstalling

Once Rust is installed via `rustup`, updating to a newly released version is
easy. From your shell, run the following update script:

```console
$ rustup update
```

To uninstall Rust and `rustup`, run the following uninstall script from your
shell:

```console
$ rustup self uninstall
```

"#
        );
        assert_eq!(
            sections[6],
            r#"### Local Documentation

The installation of Rust also includes a local copy of the documentation so
that you can read it offline. Run `rustup doc` to open the local documentation
in your browser.

Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
"#
        );
    }

    #[test]
    fn test_ja() {
        let text = read_to_string("tests/fixtures/ch01-01-installation.ja.md").unwrap();

        let sections = split(&text, None).unwrap();
        assert_eq!(sections.len(), 7);
        assert_eq!(
            sections[0],
            r#"<!-- This is from https://github.com/rust-lang-ja/book-ja/blob/822ffbb7b5ecf28ff5393e4057c8b9189a5d3fe1/src/ch01-01-installation.md -->

<!--
## Installation
-->

"#
        );
        assert_eq!(
            sections[1],
            r#"## インストール

<!--
The first step is to install Rust. We’ll download Rust through `rustup`, a
command line tool for managing Rust versions and associated tools. You’ll need
an internet connection for the download.
-->

最初の手順は、Rustをインストールすることです。Rustは、Rustのバージョンと関連するツールを管理する、`rustup`というコマンドラインツールを使用してダウンロードします。ダウンロードには、インターネットへの接続が必要になります。

<!--
> Note: If you prefer not to use `rustup` for some reason, please see [the Rust
> installation page](https://www.rust-lang.org/tools/install) for other options.
-->

> 注釈: なんらかの理由で`rustup`を使用したくない場合、[Rustインストールページ][rust-installation-page]で、
> 他の選択肢をご覧になってください。

> 訳注：日本語版のRustインストールページは[こちら][rust-installation-page-ja]です。

[rust-installation-page]: https://www.rust-lang.org/tools/install/
[rust-installation-page-ja]: https://www.rust-lang.org/ja/tools/install/

<!--
The following steps install the latest stable version of the Rust compiler.
Rust’s stability guarantees ensure that all the examples in the book that
compile will continue to compile with newer Rust versions. The output might
differ slightly between versions, because Rust often improves error messages
and warnings. In other words, any newer, stable version of Rust you install
using these steps should work as expected with the content of this book.
-->

以下の手順で最新の安定版のRustコンパイラをインストールします。
Rustは安定性 (stability) を保証しているので、現在この本の例でコンパイルできるものは、新しいバージョンになってもコンパイルでき続けることが保証されます。
出力は、バージョンによって多少異なる可能性があります。Rustは頻繁にエラーメッセージと警告を改善しているからです。
言い換えると、どんな新しいバージョンでもこの手順に従ってインストールした安定版なら、
この本の内容で想定通りに動くはずです。

<!--
> ### Command Line Notation
>
> In this chapter and throughout the book, we’ll show some commands used in the
> terminal. Lines that you should enter in a terminal all start with `$`. You
> don’t need to type in the `$` character; it indicates the start of each
> command. Lines that don’t start with `$` typically show the output of the
> previous command. Additionally, PowerShell-specific examples will use `>`
> rather than `$`.
-->

> ### コマンドラインの記法
>
> この章及び、本を通して、端末で使用するなんらかのコマンドを示すことがあります。読者が入力するべき行は、
> 全て`$`で始まります。ただし、読者が`$`文字を入力する必要はありません; これは各コマンドの開始を示しているだけです。
> `$`で始まらない行は、典型的には直前のコマンドの出力を示します。また、PowerShell限定の例には、
> `$`ではなく、`>`を使用します。

<!--
### Installing `rustup` on Linux or macOS
-->

"#
        );
        assert_eq!(
            sections[2],
            r#"### LinuxとmacOSに`rustup`をインストールする

<!--
If you’re using Linux or macOS, open a terminal and enter the following command:
-->

LinuxかmacOSを使用しているなら、端末（ターミナル）を開き、以下のコマンドを入力してください:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

<!--
The command downloads a script and starts the installation of the `rustup`
tool, which installs the latest stable version of Rust. You might be prompted
for your password. If the install is successful, the following line will appear:
-->

このコマンドはスクリプトをダウンロードし、`rustup`ツールのインストールを開始し、Rustの最新の安定版をインストールします。
パスワードを求められる可能性があります。インストールがうまく行けば、以下の行が出現するでしょう:

```text
Rust is installed now. Great!
```

<!--
Additionally, you’ll need a linker of some kind. It’s likely one is already
installed, but when you try to compile a Rust program and get errors indicating
that a linker could not execute, that means a linker isn’t installed on your
system and you’ll need to install one manually. C compilers usually come with
the correct linker. Check your platform’s documentation for how to install a C
compiler. Also, some common Rust packages depend on C code and will need a C
compiler. Therefore, it might be worth installing one now.
-->

これに加えて、なんらかのリンカが必要になるでしょう。既にインストールされている可能性は高いものの、
Rustプログラムをコンパイルしようとした時、リンカが実行できないというエラーが出たら、
システムにリンカがインストールされていないということなので、手動でインストールする必要があるでしょう。
Cコンパイラは通常正しいリンカとセットになっています。
自分のプラットフォームのドキュメンテーションを見てCコンパイラのインストール方法を確認してください。
一般的なRustパッケージの中には、Cコードに依存し、Cコンパイラが必要になるものもあります。
ですので、Cコンパイラは今のうちにインストールしておく価値があるかもしれません。

<!--
### Installing `rustup` on Windows
-->

"#
        );
        assert_eq!(
            sections[3],
            r#"### Windowsで`rustup`をインストールする


<!--
On Windows, go to [https://www.rust-lang.org/tools/install][install] and follow
the instructions for installing Rust. At some point in the installation, you’ll
receive a message explaining that you’ll also need the C++ build tools for
Visual Studio 2013 or later. The easiest way to acquire the build tools is to
install [Build Tools for Visual Studio 2019][visualstudio]. When asked which
workloads to install make sure "C++ build tools" is selected and that the Windows 10 SDK and the English language pack components are included.

-->

Windowsでは、[https://www.rust-lang.org/tools/install][install]に行き、手順に従ってRustをインストールしてください。
インストールの途中で、Visual Studio 2013以降用のC++ビルドツールも必要になるという旨のメッセージが出るでしょう。
ビルドツールを取得する最も簡単な方法は、[Visual Studio 2019用のビルドツール][visualstudio]をインストールすることです。
どのワークロード (workloads) をインストールするかと質問されたときは、"C++ build tools"が選択されており、Windows 10 SDKと英語の言語パック (English language pack) が含まれていることを確かめてください。

> 訳注：Windowsの言語を日本語にしている場合は言語パックのところで「日本語」が選択されており、そのままの設定でインストールしても基本的に問題ないはずです。しかし、サードパーティーのツールやライブラリの中には英語の言語パックを必要とするものがあるため、「日本語」に加えて「英語」も選択することをお勧めします。

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

<!--
The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.
-->

これ以降、*cmd.exe*とPowerShellの両方で動くコマンドを使用します。
特段の違いがあったら、どちらを使用すべきか説明します。

<!--
### Updating and Uninstalling
-->

"#
        );
        assert_eq!(
            sections[4],
            r#"### 更新及びアンインストール

<!--
After you’ve installed Rust via `rustup`, updating to the latest version is
easy. From your shell, run the following update script:
-->

`rustup`経由でRustをインストールしたなら、最新版へ更新するのは簡単です。
シェルから以下の更新スクリプトを実行してください:

```console
$ rustup update
```

<!--
To uninstall Rust and `rustup`, run the following uninstall script from your
shell:
-->

Rustと`rustup`をアンインストールするには、シェルから以下のアンインストールスクリプトを実行してください:

```console
$ rustup self uninstall
```

<!--
### Troubleshooting
-->

"#
        );
        assert_eq!(
            sections[5],
            r#"### トラブルシューティング

<!--
To check whether you have Rust installed correctly, open a shell and enter this
line:
-->

Rustが正常にインストールされているか確かめるには、シェルを開いて以下の行を入力してください:

```console
$ rustc --version
```

<!--
You should see the version number, commit hash, and commit date for the latest
stable version that has been released in the following format:
-->

バージョンナンバー、コミットハッシュ、最新の安定版がリリースされたコミット日時が以下のフォーマットで表示されるのを目撃するはずです。

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

<!--
If you see this information, you have installed Rust successfully! If you don’t
see this information and you’re on Windows, check that Rust is in your `%PATH%`
system variable. If that’s all correct and Rust still isn’t working, there are
a number of places you can get help. The easiest is the #beginners channel on
[the official Rust Discord][discord]. There, you can chat with other Rustaceans
(a silly nickname we call ourselves) who can help you out. Other great
resources include [the Users forum][users] and [Stack Overflow][stackoverflow].
-->

この情報が見られたなら、Rustのインストールに成功しています！この情報が出ず、Windowsを使っているなら、
Rustが`%PATH%`システム環境変数にあることを確認してください。これらが全て正常であるのに、それでもRustがうまく動かないなら、
助力を得られる場所はたくさんあります。最も簡単なのが[Rustの公式Discord][discord]の#beginnersチャンネルです。そのアドレスで、助けてくれる他のRustacean (Rustユーザが自分たちのことを呼ぶ、冗談めいたニックネーム) たちとチャットできます。
他にも、素晴らしいリソースとして[ユーザ・フォーラム][users]と[Stack Overflow][stackoverflow]が挙げられます。

> 訳注1：Rustaceanについて、いらないかもしれない補足です。[公式Twitter曰く、Rustaceanはcrustaceans（甲殻類）から来ている][twitter]そうです。
> そのため、Rustのマスコットは（非公式らしいですが）[カニ][mascott]。上の会話でCの欠点を削ぎ落としているからcを省いてるの？みたいなことを聞いていますが、
> 違うそうです。検索したら、堅牢性が高いから甲殻類という意見もありますが、真偽は不明です。
> 明日使えるかもしれないトリビアでした。

> 訳注2：上にある公式Discordは英語話者のコミュニティです。日本語話者のためのコミュニティが[Zulip rust-lang-jpにあり][zulip_jp]、こちらでもRustaceanたちが活発に議論をしています。
> 公式Discord同様、初心者向けの#beginnersチャンネルが存在するので、気軽に質問してみてください。

[discord]: https://discord.gg/rust-lang
[users]: https://users.rust-lang.org/
[stackoverflow]: https://stackoverflow.com/questions/tagged/rust
[twitter]: https://mobile.twitter.com/rustlang/status/916284650674323457
[mascott]: https://www.slideshare.net/wolf-dog/ss-64026540
[zulip_jp]: https://rust-lang-jp.zulipchat.com

<!--
### Local Documentation
-->

"#
        );
        assert_eq!(
            sections[6],
            r#"### ローカルのドキュメンテーション

<!--
The installation of Rust also includes a copy of the documentation locally, so
you can read it offline. Run `rustup doc` to open the local documentation in
your browser.
-->

インストールされたRustには、ローカルに複製されたドキュメンテーションのコピーが含まれているので、これをオフラインで閲覧することができます。
ブラウザでローカルのドキュメンテーションを開くには、`rustup doc`を実行してください。

<!--
Any time a type or function is provided by the standard library and you’re not
sure what it does or how to use it, use the application programming interface
(API) documentation to find out!
-->

標準ライブラリにより提供される型や関数がなんなのかや、それをどう使えば良いのかがよくわからないときは、いつでもAPIのドキュメンテーションを検索してみてください！"#
        );
    }
}
