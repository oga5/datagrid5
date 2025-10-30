# DataGrid5 APIリファレンス

DataGrid5 WebAssemblyグリッドコントロールの完全なAPIドキュメント

[English version](./API_REFERENCE.md)

## 目次

- [DataGridWrapperでのクイックスタート](#datagridwrapperでのクイックスタート)
- [DataGridWrapper API](#datagridwrapper-api)
- [低レベル DataGrid API](#低レベル-datagrid-api)
  - [初期化](#初期化)
  - [グリッド設定](#グリッド設定)
  - [データ管理](#データ管理)
  - [レンダリング](#レンダリング)
  - [イベント処理](#イベント処理)
  - [編集](#編集)
  - [選択](#選択)
  - [検索と置換](#検索と置換)
  - [ソートとフィルタ](#ソートとフィルタ)
  - [スタイリング](#スタイリング)
  - [元に戻す/やり直し](#元に戻すやり直し)
  - [パフォーマンス](#パフォーマンス)
  - [ワーカースレッド対応](#ワーカースレッド対応)
  - [コンテキストメニュー](#コンテキストメニュー)

---

## DataGridWrapperでのクイックスタート

**DataGridWrapper** は推奨される高レベルAPIで、DataGrid5の使用を50-80%少ないコードで簡素化します：

```javascript
import init, { DataGrid } from './pkg/datagrid5.js';

// WASMを初期化
await init();

// 最小限の設定でラッパーを作成
const wrapper = new DataGridWrapper('grid-container', DataGrid, {
    rows: 100,
    cols: 10,
    enableEditing: true  // オプション: セル編集を有効化
});

// データをロード
const data = [
    { row: 0, col: 0, value: "Hello" },
    { row: 0, col: 1, value: "World" }
];
wrapper.loadData(data);

// これだけ！ラッパーが以下を処理します：
// - キャンバスのセットアップとDOM構造
// - イベントハンドラー（マウス、キーボード、ホイール）
// - 仮想スクロール
// - リサイズ処理
// - クリップボード操作（Ctrl+C/V/X）
// - 必要に応じたレンダリング
```

**主な利点：**
- ✅ キャンバスとDOMの自動セットアップ
- ✅ 組み込みイベント処理
- ✅ すぐに使える仮想スクロール
- ✅ Excelライクなキーボードショートカット
- ✅ レスポンシブなリサイズ対応
- ✅ 手動レンダリングループ不要

---

## DataGridWrapper API

### コンストラクター

```javascript
new DataGridWrapper(containerId, DataGrid, options)
```

**パラメータ:**
- `containerId: string` - コンテナdivのID
- `DataGrid: class` - WASMからのDataGridクラス
- `options: object` - 設定オプション

**オプション:**
```typescript
interface WrapperOptions {
    rows: number;              // 行数
    cols: number;              // 列数
    width?: number;            // 初期幅（デフォルト: コンテナ幅）
    height?: number;           // 初期高さ（デフォルト: コンテナ高さ）
    enableEditing?: boolean;   // セル編集を有効化（デフォルト: false）
    columns?: ColumnConfig[];  // 列設定
    frozen_rows?: number;      // 固定行（デフォルト: 0）
    frozen_cols?: number;      // 固定列（デフォルト: 0）
}
```

### メソッド

#### `loadData(data)`
グリッドデータをロード
```javascript
wrapper.loadData([
    { row: 0, col: 0, value: "テキスト" },
    { row: 0, col: 1, value: 123 }
]);
```

#### `setCellValue(row, col, value)`
個別のセル値を設定
```javascript
wrapper.setCellValue(0, 0, "新しい値");
```

#### `getCellValue(row, col)`
セル値を取得
```javascript
const value = wrapper.getCellValue(0, 0);
```

#### `resize(width, height)`
グリッドをリサイズ
```javascript
wrapper.resize(1000, 600);
```

#### `setZebraColor(color)`
交互の行のゼブラストライプ色を設定
```javascript
wrapper.setZebraColor(0xF5F5F5FF); // ライトグレー
```

#### `destroy()`
リソースをクリーンアップ
```javascript
wrapper.destroy();
```

### 組み込み機能

**キーボードショートカット:**
- `Ctrl+C` - 選択したセルをコピー
- `Ctrl+X` - 選択したセルをカット
- `Ctrl+V` - クリップボードから貼り付け
- 矢印キー - セル間を移動
- `Shift+矢印` - 選択範囲を拡張
- `Enter` - 編集開始（有効化されている場合）
- `Escape` - 編集をキャンセル

**マウス操作:**
- クリック - セルを選択
- ドラッグ - 範囲選択
- Shift+クリック - 選択範囲を拡張
- Ctrl+クリック - 複数選択
- ダブルクリック - 編集開始（有効化されている場合）
- ホイール - グリッドをスクロール
- 列/行の境界をドラッグ - サイズ変更

**クリップボード:**
- TSV（タブ区切り値）形式
- Excel/Googleスプレッドシートと互換
- 範囲のコピー/貼り付けをサポート

---

## 低レベル DataGrid API

高度な使用例では、低レベルDataGrid APIを直接使用できます。**注意:** ほとんどのユーザーはDataGridWrapperを使用すべきです。

### 初期化

#### `DataGrid.from_container(container_id, options_json)`

コンテナdivから新しいDataGridインスタンスを作成します。

**パラメータ:**
- `container_id: string` - コンテナdiv要素のID
- `options_json: string` - グリッド設定を含むJSON文字列

**戻り値:** `DataGrid` インスタンス

**例:**
```javascript
const options = {
    rows: 100,
    cols: 10,
    width: 800,
    height: 600,
    columns: [
        {
            display_name: "ID",
            internal_name: "id",
            width: 60,
            data_type: "number",
            editable: false
        }
    ],
    frozen_rows: 1,
    frozen_cols: 0,
    readonly: false
};

const grid = DataGrid.from_container('my-grid', JSON.stringify(options));
```

#### `new DataGrid(webgl_canvas_id, text_canvas_id, rows, cols)`

明示的なキャンバスIDで新しいDataGridインスタンスを作成します（レガシーメソッド）。

**パラメータ:**
- `webgl_canvas_id: string` - WebGLキャンバス要素のID
- `text_canvas_id: string` - テキストオーバーレイキャンバス要素のID
- `rows: number` - 行数
- `cols: number` - 列数

**戻り値:** `DataGrid` インスタンス

---

### グリッド設定

#### 列設定オプション

```typescript
interface ColumnConfig {
    display_name: string;      // ヘッダーの表示名
    internal_name: string;     // 一意の内部識別子
    width: number;             // 列幅（ピクセル）
    data_type: "text" | "number" | "date" | "boolean";
    editable: boolean;         // セルを編集可能か
    visible: boolean;          // 列が表示されるか
    sortable: boolean;         // 列をソート可能か
    filterable: boolean;       // 列をフィルタ可能か
}
```

#### グリッドオプション

```typescript
interface GridOptions {
    rows: number;              // 行数
    cols: number;              // 列数
    width: number;             // グリッド幅（ピクセル）
    height: number;            // グリッド高さ（ピクセル）
    columns?: ColumnConfig[];  // 列設定

    // 固定ペイン
    frozen_rows?: number;      // 固定行数（デフォルト: 0）
    frozen_cols?: number;      // 固定列数（デフォルト: 0）

    // 表示オプション
    show_headers?: boolean;    // 行/列ヘッダーを表示（デフォルト: true）
    show_grid_lines?: boolean; // グリッド線を表示（デフォルト: true）
    alternate_row_colors?: boolean; // 交互の行色（デフォルト: false）

    // インタラクション
    readonly?: boolean;        // 読み取り専用モード（デフォルト: false）
    enable_context_menu?: boolean; // コンテキストメニューを有効化（デフォルト: true）
    enable_row_selection?: boolean; // 行選択を有効化（デフォルト: true）
    enable_col_selection?: boolean; // 列選択を有効化（デフォルト: true）

    // ヘッダー寸法
    row_header_width?: number; // 行ヘッダー幅（デフォルト: 60）
    col_header_height?: number; // 列ヘッダー高さ（デフォルト: 30）
}
```

---

### データ管理

#### `load_data_json(data_json)`

JSONからグリッドデータをロード

**パラメータ:**
- `data_json: string` - セルデータのJSON配列

**形式:**
```javascript
const data = [
    { row: 0, col: 0, value: "テキスト" },
    { row: 0, col: 1, value: 123.45 },
    { row: 0, col: 2, value: "2024-01-15" }, // 日付
    { row: 0, col: 3, value: true }          // 真偽値
];

grid.load_data_json(JSON.stringify(data));
```

#### `set_cell_value(row, col, value)`

単一セルの値を設定

**パラメータ:**
- `row: number` - 行インデックス（0ベース）
- `col: number` - 列インデックス（0ベース）
- `value: string` - セル値（列タイプに基づいて自動変換）

#### `get_cell_value(row, col)`

セルの値を取得

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス

**戻り値:** `string` - セル値

#### `get_dimensions()`

グリッドの寸法を取得

**戻り値:** `[number, number]` - [行数, 列数]

#### `clear_all()`

すべてのセルデータをクリア

---

### レンダリング

#### `render()`

グリッドをレンダリングします。DataGridWrapperを使用していない場合、データや設定を変更した後に呼び出してください。

**例:**
```javascript
// 変更を加える
grid.set_cell_value(0, 0, "更新済み");

// 変更をレンダリング
grid.render();
```

**注意:** DataGridWrapperは自動的にレンダリングを処理します - ラッパーを使用する場合はこのメソッドを呼び出す必要はありません。

#### `resize(width, height)`

グリッドをリサイズ

**パラメータ:**
- `width: number` - 新しい幅（ピクセル）
- `height: number` - 新しい高さ（ピクセル）

---

### イベント処理

**注意:** DataGridWrapperはすべてのイベントを自動的に処理します。これらのメソッドは低レベルの使用のみを対象としています。

#### `handle_wheel(delta_x, delta_y)`

スクロール用のマウスホイールイベントを処理

**パラメータ:**
- `delta_x: number` - 水平スクロールデルタ
- `delta_y: number` - 垂直スクロールデルタ

#### `handle_mouse_down_at_with_modifiers(x, y, shift, ctrl)`

修飾キー付きのマウスダウンイベントを処理

**パラメータ:**
- `x: number` - X座標
- `y: number` - Y座標
- `shift: boolean` - Shiftキーが押されている
- `ctrl: boolean` - Ctrl/Cmdキーが押されている

#### `handle_mouse_up(x, y)`

マウスアップイベントを処理

#### `handle_mouse_move(event)`

マウス移動イベントを処理

#### `handle_keyboard_with_modifiers_key(key, ctrl, shift)`

修飾キー付きのキーボードイベントを処理

**パラメータ:**
- `key: string` - キー文字列（例: "ArrowDown", "Enter"）
- `ctrl: boolean` - Ctrl/Cmdキーが押されている
- `shift: boolean` - Shiftキーが押されている

**サポートされているキー:**
- 矢印キー: ナビゲーション
- Shift+矢印: 範囲選択
- Delete: セルをクリア
- Enter: 編集開始
- Escape: 編集をキャンセル
- Page Up/Down: ページナビゲーション
- Home/End: 開始/終了にジャンプ

#### `handle_context_menu(event)`

コンテキストメニュー（右クリック）イベントを処理

**パラメータ:**
- `event: MouseEvent` - マウスイベント

**戻り値:** `string` - コンテキスト情報を含むJSON

---

### 編集

#### `start_edit(row, col)`

セルの編集を開始

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス

#### `end_edit()`

編集を停止して変更を保存

#### `is_editing()`

現在編集中かどうかをチェック

**戻り値:** `boolean`

---

### 選択

#### `select_cell(row, col)`

単一セルを選択

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス

#### `get_selected_range()`

選択されたセル範囲を取得

**戻り値:** `[number, number, number, number]` - [start_row, start_col, end_row, end_col]

---

### 検索と置換

#### `search(query, case_sensitive, whole_word, use_regex)`

グリッド内のテキストを検索

**パラメータ:**
- `query: string` - 検索クエリ
- `case_sensitive: boolean` - 大文字小文字を区別
- `whole_word: boolean` - 単語全体のみマッチ
- `use_regex: boolean` - 正規表現を使用

**戻り値:** `number` - 見つかったマッチの数

#### `find_next()`

次の検索結果に移動

**戻り値:** `boolean` - マッチが見つかった場合true

#### `find_previous()`

前の検索結果に移動

**戻り値:** `boolean` - マッチが見つかった場合true

---

### ソートとフィルタ

#### `sort_by_column(col, ascending)`

単一列でソート

**パラメータ:**
- `col: number` - 列インデックス
- `ascending: boolean` - ソート方向

#### `filter_by_column(col, predicate)`

列の値で行をフィルタ

**パラメータ:**
- `col: number` - 列インデックス
- `predicate: string` - フィルタ述語（テキストマッチ）

---

### スタイリング

#### `set_cell_bg_color(row, col, color)`

セルの背景色を設定

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス
- `color: number` - u32としてのRGBA色（0xRRGGBBAA）

**例:**
```javascript
// ライトブルーの背景を設定
grid.set_cell_bg_color(0, 0, 0xADD8E6FF);
```

#### `set_cell_fg_color(row, col, color)`

セルの前景（テキスト）色を設定

#### `set_cell_font_style(row, col, bold, italic)`

セルのフォントスタイルを設定

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス
- `bold: boolean` - 太字テキスト
- `italic: boolean` - イタリックテキスト

---

### 元に戻す/やり直し

#### `undo()`

最後のアクションを元に戻す

**戻り値:** `boolean` - 元に戻すが実行された場合true

#### `redo()`

最後に元に戻したアクションをやり直す

**戻り値:** `boolean` - やり直しが実行された場合true

---

### パフォーマンス

#### `get_render_time()`

最後のフレームのレンダリング時間を取得

**戻り値:** `number` - レンダリング時間（ミリ秒）

#### `reserve_capacity(expected_cells)`

パフォーマンス向上のためメモリ容量を予約

**パラメータ:**
- `expected_cells: number` - 予想されるセル数

---

### ワーカースレッド対応

#### `export_grid_data_json()`

ワーカー処理用にすべてのグリッドデータをJSONとしてエクスポート

**戻り値:** `string` - セルデータのJSON配列

#### `import_worker_result(result_json)`

ワーカーから処理されたデータをインポート

**パラメータ:**
- `result_json: string` - ワーカーからのJSON配列

**戻り値:** `number` - 更新されたセルの数

---

### コンテキストメニュー

#### `execute_row_operation(operation, row)`

行のコンテキストメニュー操作を実行

**パラメータ:**
- `operation: string` - 操作名
- `row: number` - 行インデックス

**操作:**
- `"insert_row_above"` - 上に行を挿入
- `"insert_row_below"` - 下に行を挿入
- `"delete_row"` - 行を削除
- `"copy_row"` - 行をクリップボードにコピー
- `"cut_row"` - 行をクリップボードにカット

---

### その他のメソッド

#### 行/列操作

- `insert_row(index)` - 新しい行を挿入
- `delete_row(index)` - 行を削除
- `insert_column(index)` - 新しい列を挿入
- `delete_column(index)` - 列を削除
- `set_col_width(col, width)` - 列幅を設定
- `set_row_height(row, height)` - 行高を設定

#### 固定

- `freeze_rows(count)` - 上部の行を固定
- `freeze_columns(count)` - 左側の列を固定

#### ビューポート

- `scroll_to(row, col)` - セルにスクロール
- `set_scroll(x, y)` - スクロール位置を設定
- `get_viewport_info_array()` - ビューポート情報を取得

---

実用的な例については、[examples](../examples/)ディレクトリを参照してください。
