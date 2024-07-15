# RusToDo

RusToDo は Rust で書かれたシンプルな CLI ベースの TODO アプリケーションです。

RusToDo is a simple CLI-based TODO application written in Rust.

## インストール方法 / Installation

### 必要条件 / Prerequisites

- Rust と Cargo がインストールされていること
- Rust and Cargo installed on your system

### ビルド手順 / Build Instructions

1. このリポジトリをクローンします:
   Clone this repository:

   ```
   git clone https://github.com/yourusername/rustodo.git
   cd rustodo
   ```

2. リリースビルドを作成します:
   Create a release build:

   ```
   cargo build --release
   ```

3. 実行ファイルを適切な場所にコピーします:
   Copy the executable to an appropriate location:

   - Unix 系:

     ```
     sudo cp target/release/rtd /usr/local/bin/
     ```

   - Windows:
     ```
     copy target\release\rtd.exe C:\Windows\System32\
     ```

## 使い方 / Usage

RusToDo は以下のコマンドで操作できます:
RusToDo can be operated with the following commands:

1. TODO の追加 / Add a TODO:

   ```
   rtd --add "タスク名" --description "タスクの説明" --priority high
   ```

2. TODO の更新 / Update a TODO:

   ```
   rtd --update <ID> --name "新しいタスク名" --status completed
   ```

3. TODO の削除 / Delete a TODO:

   ```
   rtd --delete <ID>
   ```

4. TODO 一覧の表示 / List all TODOs:

   ```
   rtd --list
   ```

5. 特定の TODO の詳細表示 / Show details of a specific TODO:
   ```
   rtd --show <ID>
   ```

### オプション / Options

- `--add <TITLE>`: 新しい TODO を追加 / Add a new TODO
- `--update <ID>`: 指定 ID の TODO を更新 / Update a TODO with the specified ID
- `--delete <ID>`: 指定 ID の TODO を削除 / Delete a TODO with the specified ID
- `--list`: すべての TODO を表示 / List all TODOs
- `--show <ID>`: 指定 ID の TODO の詳細を表示 / Show details of a TODO with the specified ID
- `--name <TITLE>`: TODO のタイトルを設定（追加・更新時に使用） / Set the title of a TODO (used with add or update)
- `--description <DESC>`: TODO の説明を設定 / Set the description of a TODO
- `--priority <PRIORITY>`: TODO の優先度を設定（low/medium/high） / Set the priority of a TODO (low/medium/high)
- `--status <STATUS>`: TODO の状態を設定（pending/inprogress/completed） / Set the status of a TODO (pending/inprogress/completed)
- `--due_date <DATE>`: TODO の期限を設定（YYYY/MM/DD HH:MM:SS 形式） / Set the due date of a TODO (format: YYYY/MM/DD HH:MM:SS)

## ライセンス / License

このプロジェクトは [GPL ライセンス](LICENSE) の下で公開されています。
This project is released under the [GPL License](LICENSE).
