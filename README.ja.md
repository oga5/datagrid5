# DataGrid5

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-WASM-blueviolet.svg)](https://webassembly.org/)

**モダンブラウザ向けの超高速WebAssemblyベースのグリッドコントロール**

DataGrid5は、RustとWebAssemblyで構築された高性能で機能豊富なデータグリッドコンポーネントです。WebGLを使用したGPUアクセラレーションレンダリングにより、Excelライクな機能を提供します。

🌐 **[ライブデモ](https://oga5.github.io/datagrid5/)** | [English README](./README.md) | [ドキュメント](./docs/) | [サンプル](./examples/) | [DataGridWrapper ガイド](./www/README.md)

## ✨ 特徴

- **🚀 高パフォーマンス**: WebGL GPU レンダリング + WebAssembly で 100k+ 行でも 60 FPS
- **📊 Excelライクなインターフェース**: キーボードナビゲーション付きの使い慣れたスプレッドシートUI
- **✏️ 完全な編集サポート**: ダブルクリックで編集、コピー/ペースト、元に戻す/やり直し
- **📋 Excel互換クリップボード**: Ctrl+C/X/Vでコピー/カット/ペースト、Excelとの互換性のためのTSV形式
- **✅ 入力検証**: 列ベースの正規表現検証とカスタムエラーメッセージ
- **🎨 豊富なスタイリング**: セルの色、フォント、罫線、カスタムスタイリングAPI
- **🔍 高度な検索**: テキスト検索、正規表現、検索と置換、マッチのハイライト
- **📑 ソートとフィルタリング**: 複数列ソート、カスタムフィルタ、列ベースフィルタリング
- **📊 列のグループ化**: データを整理して表示するための複数レベルの階層ヘッダー
- **❄️ 固定ペイン**: Excelのように行と列を固定
- **📋 コンテキストメニュー**: 行の右クリック操作（挿入、削除、移動、コピー、カット）
- **⚡ ワーカースレッドサポート**: 大規模データセットのバックグラウンド処理
- **📝 列の設定**: データ型（テキスト、数値、日付、ブール値）、カスタム幅
- **🔒 読み取り専用モード**: グリッド全体または列ごとの編集制御
- **🎯 差分レンダリング**: パフォーマンス向上のため変更されたセルのみを再レンダリング
- **💾 遅延読み込み**: 大規模データセットの段階的データロード
- **🎁 DataGridWrapper**: ボイラープレートコードを約50-80%削減する高レベルJavaScriptラッパー

## 🎯 主な利点

| 機能 | DataGrid5 | 従来のJSグリッド |
|---------|-----------|---------------------|
| パフォーマンス | **高性能** (WebGL + WASM) | JavaScript + DOM |
| メモリ使用量 | **疎な格納** (HashMap) | 密な配列 |
| 大規模データセット | ✅ 100万行以上 | ❌ 約5万行に制限 |
| 仮想スクロール | ✅ GPUアクセラレーション | ✅ CPU依存 |
| データ型 | テキスト、数値、日付、ブール値 | 限定的 |
| ワーカースレッド | ✅ バックグラウンド処理 | ❌ UIをブロック |

## 🏗️ アーキテクチャ

```
┌─────────────────────────────────────┐
│      JavaScriptアプリケーション      │
│   (DataGrid APIを使用)              │
└──────────────┬──────────────────────┘
               │ wasm-bindgen
┌──────────────▼──────────────────────┐
│         Rustコア (WASM)             │
│  ┌──────────────────────────────┐   │
│  │  グリッドデータ構造           │   │
│  │  - 疎な格納 (HashMap)         │   │
│  │  - 仮想スクロール             │   │
│  │  - 列の設定                   │   │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  WebGLレンダラー              │   │
│  │  - GPUアクセラレーション描画  │   │
│  │  - シェーダーベースレンダリング│  │
│  └──────────────────────────────┘   │
│  ┌──────────────────────────────┐   │
│  │  イベントハンドラ             │   │
│  │  - マウス、キーボード、ホイール│  │
│  │  - コンテキストメニュー対応   │   │
│  └──────────────────────────────┘   │
└─────────────────────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         ブラウザAPI                 │
│  - WebGL, Canvas 2D, DOMイベント    │
└─────────────────────────────────────┘
```

### 🔧 技術スタック

- **言語**: Rust 2021 edition
- **WebAssembly**: wasm-bindgen, web-sys, js-sys
- **グラフィックス**: WebGL 1.0（広範な互換性のため）
- **ビルドツール**: wasm-pack
- **メモリアロケータ**: wee_alloc（軽量）

## 🚀 クイックスタート

### インストール

```bash
# wasm-packをインストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# リポジトリをクローン
git clone https://github.com/oga5/datagrid5.git
cd datagrid5

# プロジェクトをビルド（便利なスクリプトを使用）
./build.sh

# または手動でビルド
wasm-pack build --target web --release

# 開発サーバーを起動
./serve.sh

# または手動でサーバーを起動
python3 -m http.server 8080
```

ブラウザを開いて以下にアクセス：
- メインデモ: http://localhost:8080/www/
- サンプル: http://localhost:8080/examples/

### ビルドスクリプトオプション

```bash
# 開発ビルド（高速、ファイルサイズ大）
./build.sh --dev

# リリースビルド（最適化、ファイルサイズ小）[デフォルト]
./build.sh --release

# クリーンビルド（以前の成果物を削除）
./build.sh --clean

# ヘルプを表示
./build.sh --help
```

### 基本的な使い方

#### オプション1: DataGridWrapperを使用（推奨）⭐

約80%少ないコードで始める最も簡単な方法：

```html
<!DOCTYPE html>
<html>
<head>
    <title>DataGrid5 サンプル</title>
</head>
<body>
    <div id="grid-container" style="width: 100%; height: 600px;"></div>

    <script type="module">
        import init from './pkg/datagrid5.js';
        import { DataGridWrapper } from './www/datagrid5-wrapper.js';

        // WebAssemblyを初期化
        const wasmModule = await init();

        // ラッパーでグリッドを作成 - すべて自動処理！
        const gridWrapper = new DataGridWrapper('grid-container', wasmModule, {
            rows: 100,
            cols: 10,
            enableEditing: true,
            enableVirtualScroll: true
        });

        // データ操作用のグリッドにアクセス
        const grid = gridWrapper.getGrid();
        grid.update_cell_value(0, 0, "商品");
        grid.update_cell_value(0, 1, "価格");
        grid.update_cell_value(1, 0, "ノートPC");
        grid.update_cell_value(1, 1, "¥99,999");

        // 編集イベントをリッスン（オプション）
        document.getElementById('grid-container').addEventListener('celleditend', (e) => {
            console.log(`セル (${e.detail.row}, ${e.detail.col}) が変更: ${e.detail.newValue}`);
        });
    </script>
</body>
</html>
```

**DataGridWrapperが提供するもの:**
- ✅ 自動キャンバス設定とイベント処理
- ✅ キーボードナビゲーション付き組み込みセルエディタ
- ✅ クリップボードサポート（Ctrl+C/X/V）
- ✅ レンダーループ管理
- ✅ 仮想スクロール設定
- ✅ リサイズ処理
- ✅ 統合用カスタムイベント

詳細は[DataGridWrapperガイド](./www/README.md)を参照してください。

#### オプション2: 直接API使用（完全な制御）

```html
<!DOCTYPE html>
<html>
<head>
    <title>DataGrid5 サンプル</title>
</head>
<body>
    <div id="grid-container" style="width: 100%; height: 600px;"></div>

    <script type="module">
        import init, { DataGrid } from './pkg/datagrid5.js';

        // WebAssemblyを初期化
        await init();

        // シンプルな設定でグリッドを作成
        const grid = DataGrid.from_container('grid-container', JSON.stringify({
            rows: 100,
            cols: 10,
            width: 800,
            height: 600
        }));

        // データをロード
        const data = [
            { row: 0, col: 0, value: "商品" },
            { row: 0, col: 1, value: "価格" },
            { row: 1, col: 0, value: "ノートPC" },
            { row: 1, col: 1, value: 99999 }
        ];
        grid.load_data_json(JSON.stringify(data));

        // レンダリング
        grid.render();
    </script>
</body>
</html>
```

### カスタム列ヘッダー

行0に値を設定してスタイルを適用することで、列ヘッダーをカスタマイズできます：

```javascript
// カスタム列ヘッダーを定義
const columnHeaders = [
    "従業員ID", "氏名", "メールアドレス", "部署",
    "給与", "入社日", "ステータス", "上長"
];

// スタイル付きでヘッダーを設定
for (let col = 0; col < columnHeaders.length; col++) {
    grid.update_cell_value(0, col, columnHeaders[col]);

    // ヘッダー行をスタイル設定
    grid.set_cell_bg_color(0, col, 0x667eeaFF);  // 青い背景
    grid.set_cell_fg_color(0, col, 0xFFFFFFFF);  // 白いテキスト
    grid.set_cell_font_style(0, col, true, false);  // 太字
}

// データ行を埋める
for (let row = 1; row <= 100; row++) {
    grid.update_cell_value(row, 0, `EMP${1000 + row}`);
    grid.update_cell_value(row, 1, `社員 ${row}`);
    grid.update_cell_value(row, 2, `employee${row}@company.co.jp`);
    grid.update_cell_value(row, 3, "エンジニアリング");
    grid.update_cell_value(row, 4, `¥${5000000 + row * 100000}`);
    grid.update_cell_value(row, 5, "2020-01-15");
    grid.update_cell_value(row, 6, "在職中");
    grid.update_cell_value(row, 7, "上長名");
}
```

### 列のグループ化（複数レベルのヘッダー）

DataGrid5は複数レベルの階層列ヘッダーをサポートし、視覚的に列をグループ化できます：

```javascript
// 2レベルの例: 地域別に列をグループ化
grid.add_column_group("東京", 0, 3, 0);     // 東京グループの列0-3
grid.add_column_group("大阪", 4, 9, 0);     // 大阪グループの列4-9
grid.add_column_group("その他", 10, 19, 0);  // その他グループの列10-19

// 3レベルの例: 地方 > 都市 > 店舗
grid.add_column_group("関東地方", 0, 7, 0);     // トップレベル
grid.add_column_group("関西地方", 8, 15, 0);

grid.add_column_group("東京", 0, 3, 1);            // 第2レベル
grid.add_column_group("神奈川", 4, 7, 1);
grid.add_column_group("大阪", 8, 11, 1);
grid.add_column_group("京都", 12, 15, 1);

// 必要に応じてヘッダー行の高さを調整
grid.set_header_row_height(35);  // デフォルトは30px

// シンプルなヘッダーに戻すにはすべてのグループをクリア
grid.clear_column_groups();
```

**パラメータ:**
- `label`: グループラベルテキスト
- `start_col`: 最初の列インデックス（0ベース）
- `end_col`: 最後の列インデックス（0ベース、含む）
- `level`: ヘッダーレベル（0 = トップ、1 = 第2、など）

グリッドはレベル数に基づいて総ヘッダー高さを自動計算します。

### 入力検証（列ベースのルール）

正規表現を使用して各列に検証ルールを設定：

```javascript
// 従業員ID列の検証を設定
grid.set_column_validation(
    0,                              // 列インデックス
    "^EMP[0-9]{4}$",               // 正規表現パターン
    '"EMP"の後に4桁の数字が必要です'  // エラーメッセージ
);

// メールの検証
grid.set_column_validation(
    2,
    "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
    "正しいメールアドレス形式で入力してください"
);

// 電話番号の検証
grid.set_column_validation(
    3,
    "^0\\d{1,4}-\\d{1,4}-\\d{4}$",
    "電話番号はハイフン区切りで入力してください"
);

// 年齢の検証（1-99）
grid.set_column_validation(
    4,
    "^[1-9][0-9]?$",
    "年齢は1〜99の数字で入力してください"
);

// 列の検証ルールを取得
const validationJson = grid.get_column_validation(0);
if (validationJson) {
    const { pattern, message } = JSON.parse(validationJson);
    const regex = new RegExp(pattern);
    if (!regex.test(inputValue)) {
        alert(message);
    }
}

// 列の検証をクリア
grid.clear_column_validation(0);
```

**一般的なパターン:**
- メール: `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
- 電話（日本）: `^0\d{1,4}-\d{1,4}-\d{4}$`
- 郵便番号（日本）: `^\d{3}-\d{4}$`
- 日付（YYYY/MM/DD）: `^\d{4}/\d{2}/\d{2}$`
- 数字のみ: `^[0-9]+$`
- 日本語テキスト: `^[\u4E00-\u9FFF\u3040-\u309F]+$`

### 読み取り専用列

列ごとにどの列を編集可能にするかを制御：

```javascript
// 特定の列を読み取り専用に設定
grid.set_column_editable(0, false);  // ID列 - 読み取り専用
grid.set_column_editable(5, false);  // 作成日 - 読み取り専用
grid.set_column_editable(6, false);  // 更新日 - 読み取り専用

// 列を編集可能に設定
grid.set_column_editable(1, true);   // 名前列 - 編集可能
grid.set_column_editable(2, true);   // メール列 - 編集可能

// 列が編集可能かどうかを確認
const isEditable = grid.is_column_editable(0);
console.log(`列0は${isEditable ? '編集可能' : '読み取り専用'}です`);

// すべての列の編集可能ステータスを取得
const statusArray = JSON.parse(grid.get_all_column_editable_status());
// 戻り値: [false, true, true, true, true, false, false, true]

// 読み取り専用列を編集しようとすると、何も起こりません
// ユーザーインタラクションを許可する前に列のステータスを確認できます
```

**一般的な使用例:**
- 自動生成ID（読み取り専用）
- システムタイムスタンプ（created_at、updated_at）
- 計算フィールド（合計、計算値）
- 監査フィールド（created_by、modified_by）
- ワークフローで管理されるステータスフィールド

### コンテキストメニュー編集

DataGrid5は、挿入、削除（単一および一括）、完全な元に戻す/やり直しサポートを含む、行でのコンテキストメニュー操作用のAPIを提供します：

```javascript
// 現在のアクティブセルの下に新しい行を挿入
const insertIndex = activeRow + 1;
grid.insert_row(insertIndex);

// 特定の行を削除
grid.delete_row(activeRow);

// 現在の選択から一意の行インデックスを取得
const selectedRowsJson = grid.get_selected_row_indices();
const selectedRows = JSON.parse(selectedRowsJson);
console.log(`選択された行: ${selectedRows.join(', ')}`);

// 複数行を一括削除（元に戻すサポート付き）
grid.delete_rows(selectedRowsJson);

// 最後の操作を元に戻す
const undoSuccess = grid.undo();
if (undoSuccess) {
    console.log('元に戻す成功');
}

// 最後に元に戻した操作をやり直す
const redoSuccess = grid.redo();
if (redoSuccess) {
    console.log('やり直し成功');
}

// 元に戻す/やり直し操作が可能かどうかを確認
const canUndo = grid.can_undo();
const canRedo = grid.can_redo();
```

**コンテキストメニューの実装:**

```javascript
// 右クリックでコンテキストメニューを表示
canvas.addEventListener('contextmenu', (event) => {
    event.preventDefault();

    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    // アクティブセルの位置を取得
    const activeCell = grid.get_active_cell();
    if (activeCell) {
        const { row, col } = JSON.parse(activeCell);
        showContextMenu(event.clientX, event.clientY, row);
    }
});

function showContextMenu(x, y, row) {
    const menu = document.getElementById('context-menu');
    menu.style.left = x + 'px';
    menu.style.top = y + 'px';
    menu.style.display = 'block';

    // 後で使用するために行を保存
    contextMenuRow = row;

    // 選択に基づいてメニュー項目を更新
    const selectedRowsJson = grid.get_selected_row_indices();
    const selectedRows = JSON.parse(selectedRowsJson);

    if (selectedRows.length > 1) {
        // 一括削除オプションを表示
        document.getElementById('menu-delete-selected').style.display = 'block';
        document.getElementById('selected-count').textContent = selectedRows.length;
    }
}

// メニュー項目のクリックを処理
document.getElementById('menu-insert-row').addEventListener('click', () => {
    grid.insert_row(contextMenuRow + 1);
    hideContextMenu();
});

document.getElementById('menu-delete-row').addEventListener('click', () => {
    grid.delete_row(contextMenuRow);
    hideContextMenu();
});

document.getElementById('menu-delete-selected').addEventListener('click', () => {
    const selectedRowsJson = grid.get_selected_row_indices();
    grid.delete_rows(selectedRowsJson);
    hideContextMenu();
});
```

**元に戻す/やり直しのキーボードショートカット:**

```javascript
document.addEventListener('keydown', (event) => {
    if ((event.ctrlKey || event.metaKey) && !event.shiftKey && event.key === 'z') {
        event.preventDefault();
        grid.undo();
    } else if ((event.ctrlKey || event.metaKey) && (event.shiftKey && event.key === 'z' || event.key === 'y')) {
        event.preventDefault();
        grid.redo();
    }
});
```

**機能:**
- 任意の位置に行を挿入
- 単一行または複数選択行を削除
- すべての行操作の完全な元に戻す/やり直しサポート
- セル選択から一意の行インデックスを取得
- 操作は編集履歴に記録されます

### Excel互換クリップボード操作

DataGrid5は、TSV（タブ区切り値）形式を使用したExcel互換性のある完全なクリップボードサポートを提供します：

```javascript
// 選択されたセルをコピー（Ctrl+C）
const tsvData = grid.copy_selected_cells();
if (tsvData) {
    navigator.clipboard.writeText(tsvData);
    console.log('クリップボードにコピーしました');
}

// 選択されたセルをカット（Ctrl+X）
const cutData = grid.cut_selected_cells();
if (cutData) {
    navigator.clipboard.writeText(cutData);
    console.log('クリップボードにカット - セルをクリア');
}

// クリップボードからペースト（Ctrl+V）
navigator.clipboard.readText().then(tsvData => {
    const success = grid.paste_cells(tsvData);
    if (success) {
        console.log('クリップボードからペーストしました');
    }
});
```

**DataGridWrapperを使用する場合:**

クリップボード操作はCtrl+C/X/Vキーボードショートカットで自動的に処理されます：

```javascript
const gridWrapper = new DataGridWrapper('grid-container', wasmModule, {
    rows: 100,
    cols: 26,
    enableEditing: true
});

// クリップボードイベントをリッスン
const container = document.getElementById('grid-container');

container.addEventListener('gridcopy', (e) => {
    console.log('コピー:', e.detail.data);
});

container.addEventListener('gridcut', (e) => {
    console.log('カット:', e.detail.data);
});

container.addEventListener('gridpaste', (e) => {
    console.log('ペースト:', e.detail.data);
});

// またはプログラムで実行
gridWrapper.copy();  // Ctrl+Cと同じ
gridWrapper.cut();   // Ctrl+Xと同じ
gridWrapper.paste(); // Ctrl+Vと同じ
```

**機能:**
- ExcelやGoogle Sheetsと互換性のあるTSV形式
- システムクリップボード統合
- 複数セル範囲サポート
- ペースト時の自動拡張
- DataGrid5と他のアプリケーション間でのコピー
- システムクリップボードが利用できない場合の内部クリップボードへのフォールバック

## 🎨 高度な設定

### データ型を持つ列定義

```javascript
const options = {
    rows: 100,
    cols: 5,
    columns: [
        {
            display_name: "従業員ID",
            internal_name: "emp_id",
            width: 80,
            data_type: "number",
            editable: false
        },
        {
            display_name: "氏名",
            internal_name: "name",
            width: 150,
            data_type: "text"
        },
        {
            display_name: "入社日",
            internal_name: "hire_date",
            width: 110,
            data_type: "date"
        },
        {
            display_name: "給与",
            internal_name: "salary",
            width: 100,
            data_type: "number"
        },
        {
            display_name: "在職中",
            internal_name: "is_active",
            width: 70,
            data_type: "boolean"
        }
    ],
    frozen_rows: 1,      // ヘッダー行を固定
    frozen_cols: 1,      // 最初の列を固定
    readonly: false,
    show_headers: true,
    alternate_row_colors: true
};

const grid = DataGrid.from_container('my-grid', JSON.stringify(options));
```

## 📖 ドキュメント

- **[APIリファレンス（英語）](./docs/API_REFERENCE.md)** - 完全なAPIドキュメント
- **[APIリファレンス（日本語）](./docs/API_REFERENCE.ja.md)** - APIリファレンス（日本語版）
- **[サンプルガイド（英語）](./docs/EXAMPLES.md)** - 使用例とチュートリアル
- **[サンプルガイド（日本語）](./docs/EXAMPLES.ja.md)** - 使用例とチュートリアル
- **[タスク進捗](./TASK.md)** - 開発ロードマップと機能追跡

## 🌐 ライブデモ

ブラウザでDataGrid5を試してみましょう：

**🚀 [ライブデモサイト](https://oga5.github.io/datagrid5/)**

- [サンプルギャラリー](https://oga5.github.io/datagrid5/examples/) - インタラクティブデモ
- [メインデモ](https://oga5.github.io/datagrid5/www/) - フル機能グリッド

## 📦 サンプル

`examples/`ディレクトリにはDataGridWrapperを使用した包括的なサンプルが含まれています：

- **[simple-usage-v2.html](./examples/simple-usage-v2.html)** - 最小限のコードでの基本グリッド
- **[clipboard-example-v2.html](./examples/clipboard-example-v2.html)** - Excelライクなコピー/ペースト
- **[context-menu-example-v2.html](./examples/context-menu-example-v2.html)** - 右クリックメニュー
- **[validation-example-v2.html](./examples/validation-example-v2.html)** - リアルタイム検証
- **[column-grouping-example-v2.html](./examples/column-grouping-example-v2.html)** - 複数レベルのヘッダー
- **[readonly-columns-example-v2.html](./examples/readonly-columns-example-v2.html)** - 列の権限
- **[advanced-config-example-v2.html](./examples/advanced-config-example-v2.html)** - 高度な設定
- **[sales-analysis-example-v2.html](./examples/sales-analysis-example-v2.html)** - 分析ダッシュボード
- **[responsive-resize-example-v2.html](./examples/responsive-resize-example-v2.html)** - 自動リサイズサポート

完全なショーケースは **[examples/index.html](./examples/index.html)** を参照してください。

## 🎯 使用例

- **ビジネスアプリケーション**: ERP、CRM、在庫管理
- **データ分析**: 大規模データセットの可視化と編集
- **金融ソフトウェア**: トレーディングプラットフォーム、会計システム
- **科学アプリケーション**: 研究データ管理
- **データベースツール**: SQLクエリ結果の表示と編集

## 🔧 開発

### ビルド

```bash
# 開発ビルド
wasm-pack build --target web --dev

# リリースビルド
wasm-pack build --target web --release
```

### テスト

```bash
# Rustテストを実行
cargo test

# コードフォーマット
cargo fmt

# Lint
cargo clippy
```

### サンプルをサーブ

```bash
# シンプルなHTTPサーバー
python3 -m http.server 8080

# http://localhost:8080/examples/ を開く
```

## 🤝 貢献

貢献を歓迎します！プルリクエストをお気軽に送信してください。

1. リポジトリをフォーク
2. 機能ブランチを作成（`git checkout -b feature/amazing-feature`）
3. 変更をコミット（`git commit -m 'Add amazing feature'`）
4. ブランチにプッシュ（`git push origin feature/amazing-feature`）
5. プルリクエストを開く

## 📄 ライセンス

MITライセンス - 詳細は[LICENSE](./LICENSE)を参照してください

すべての依存関係はMIT互換であり、プロジェクトへの統合が簡単です。

## 🙏 謝辞

このプロジェクトは、[psqledit_psqlgrid](https://github.com/oga5/psqledit_psqlgrid)のC++ GridControlの最新WebAssembly移植版です。

移植する包括的な機能セットを提供してくれた元のC++実装に特別な感謝を捧げます。

## 📞 サポート

- **Issues**: [GitHub Issues](https://github.com/oga5/datagrid5/issues)
- **ドキュメント**: [docs/](./docs/)
- **サンプル**: [examples/](./examples/)

---

**RustとWebAssemblyで❤️を込めて作られました**
