# DataGrid5 APIリファレンス

DataGrid5 WebAssemblyグリッドコントロールの完全なAPIドキュメントです。

[English version](./API_REFERENCE.md)

## 目次

- [初期化](#初期化)
- [グリッド設定](#グリッド設定)
- [データ管理](#データ管理)
- [レンダリング](#レンダリング)
- [イベント処理](#イベント処理)
- [編集](#編集)
- [選択](#選択)
- [検索・置換](#検索置換)
- [ソート・フィルタ](#ソートフィルタ)
- [スタイル](#スタイル)
- [元に戻す・やり直し](#元に戻すやり直し)
- [パフォーマンス](#パフォーマンス)
- [ワーカースレッドサポート](#ワーカースレッドサポート)
- [コンテキストメニュー](#コンテキストメニュー)

---

## 初期化

### `DataGrid.from_container(container_id, options_json)`

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

---

## グリッド設定

### カラム設定オプション

```typescript
interface ColumnConfig {
    display_name: string;      // ヘッダーに表示される名前
    internal_name: string;     // 一意の内部識別子
    width: number;             // カラム幅（ピクセル）
    data_type: "text" | "number" | "date" | "boolean";  // データ型
    editable: boolean;         // セルを編集可能か
    visible: boolean;          // カラムを表示するか
    sortable: boolean;         // ソート可能か
    filterable: boolean;       // フィルタ可能か
}
```

### グリッドオプション

```typescript
interface GridOptions {
    rows: number;              // 行数
    cols: number;              // 列数
    width: number;             // グリッド幅（ピクセル）
    height: number;            // グリッド高さ（ピクセル）
    columns?: ColumnConfig[];  // カラム設定

    // 固定表示
    frozen_rows?: number;      // 固定行数（デフォルト: 0）
    frozen_cols?: number;      // 固定列数（デフォルト: 0）

    // 表示オプション
    show_headers?: boolean;    // 行/列ヘッダーを表示（デフォルト: true）
    show_grid_lines?: boolean; // グリッド線を表示（デフォルト: true）
    alternate_row_colors?: boolean; // 交互行の色（デフォルト: false）

    // インタラクション
    readonly?: boolean;        // 読み取り専用モード（デフォルト: false）
    enable_context_menu?: boolean; // コンテキストメニューを有効化（デフォルト: true）
    enable_row_selection?: boolean; // 行選択を有効化（デフォルト: true）
    enable_col_selection?: boolean; // 列選択を有効化（デフォルト: true）

    // ヘッダーのサイズ
    row_header_width?: number; // 行ヘッダー幅（デフォルト: 60）
    col_header_height?: number; // 列ヘッダー高さ（デフォルト: 30）
}
```

---

## データ管理

### `load_data_json(data_json)`

JSONからグリッドデータを読み込みます。

**パラメータ:**
- `data_json: string` - セルデータのJSON配列

**フォーマット:**
```javascript
const data = [
    { row: 0, col: 0, value: "テキスト" },
    { row: 0, col: 1, value: 123.45 },
    { row: 0, col: 2, value: "2024-01-15" }, // 日付
    { row: 0, col: 3, value: true }          // 真偽値
];

grid.load_data_json(JSON.stringify(data));
```

### `set_cell_value(row, col, value)`

単一セルの値を設定します。

**パラメータ:**
- `row: number` - 行インデックス（0始まり）
- `col: number` - 列インデックス（0始まり）
- `value: string` - セル値（カラムの型に基づいて自動変換）

### `get_cell_value(row, col)`

セルの値を取得します。

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス

**戻り値:** `string` - セル値

### `get_dimensions()`

グリッドの寸法を取得します。

**戻り値:** `[number, number]` - [rows, cols]

### `clear_all()`

全てのセルデータをクリアします。

---

## レンダリング

### `render()`

グリッドをレンダリングします。データや設定を変更した後に呼び出してください。

**例:**
```javascript
grid.render(); // 単一レンダリング

// または連続レンダリング用にアニメーションフレームを使用
function renderLoop() {
    grid.render();
    requestAnimationFrame(renderLoop);
}
renderLoop();
```

### `resize(width, height)`

グリッドをリサイズします。

**パラメータ:**
- `width: number` - 新しい幅（ピクセル）
- `height: number` - 新しい高さ（ピクセル）

---

## イベント処理

### `handle_wheel(event)`

マウスホイールイベントを処理してスクロールします。

**パラメータ:**
- `event: WheelEvent` - マウスホイールイベント

**例:**
```javascript
canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    grid.handle_wheel(e);
});
```

### `handle_mouse_down(event)` / `handle_mouse_up(event)` / `handle_mouse_move(event)`

マウスイベントを処理します。

### `handle_keyboard(event)`

キーボードイベントを処理します。

**サポートされているキー:**
- 矢印キー: ナビゲーション
- Ctrl+C: コピー
- Ctrl+V: 貼り付け
- Ctrl+X: カット
- Ctrl+Z: 元に戻す
- Ctrl+Y: やり直し
- Delete: セルをクリア
- Enter: 編集開始
- Escape: 編集キャンセル

---

## 編集

### `start_editing(row, col)`

セルの編集を開始します。

### `stop_editing()`

編集を停止して変更を保存します。

### `cancel_editing()`

編集をキャンセルします（保存しない）。

---

## 選択

### `select_cell(row, col)`

単一セルを選択します。

### `select_range(start_row, start_col, end_row, end_col)`

セル範囲を選択します。

### `select_all()`

全てのセルを選択します。

### `select_row(row)`

行全体を選択します。

### `select_column(col)`

列全体を選択します。

### `get_selected_cells()`

選択されたセル座標の配列を取得します。

**戻り値:** `Array<[number, number]>` - [row, col]のペアの配列

### `clear_selection()`

現在の選択をクリアします。

---

## 検索・置換

### `search(query, case_sensitive, whole_word, use_regex)`

グリッド内のテキストを検索します。

**パラメータ:**
- `query: string` - 検索クエリ
- `case_sensitive: boolean` - 大文字小文字を区別
- `whole_word: boolean` - 単語全体のみマッチ
- `use_regex: boolean` - 正規表現を使用

**戻り値:** `number` - 見つかったマッチ数

### `find_next()` / `find_previous()`

次/前の検索結果に移動します。

**戻り値:** `boolean` - マッチが見つかった場合true

### `replace(replacement)`

現在のマッチを置換します。

### `replace_all(query, replacement, case_sensitive)`

全てのマッチを置換します。

**戻り値:** `number` - 置換された数

---

## ソート・フィルタ

### `sort_by_column(col, ascending)`

単一カラムでソートします。

**パラメータ:**
- `col: number` - 列インデックス
- `ascending: boolean` - ソート方向

### `sort_by_columns(columns)`

複数カラムでソートします。

**パラメータ:**
- `columns: Array<[number, boolean]>` - [col, ascending]のペアの配列

### `filter_by_column(col, predicate)`

カラム値で行をフィルタします。

### `clear_filters()`

全てのフィルタをクリアします。

---

## スタイル

### `set_cell_bg_color(row, col, color)`

セルの背景色を設定します。

**パラメータ:**
- `row: number` - 行インデックス
- `col: number` - 列インデックス
- `color: number` - RGBA色（u32形式: 0xRRGGBBAA）

### `set_cell_fg_color(row, col, color)`

セルの前景色（テキスト色）を設定します。

### `set_cell_font_style(row, col, bold, italic)`

セルのフォントスタイルを設定します。

**パラメータ:**
- `bold: boolean` - 太字
- `italic: boolean` - 斜体

### `set_cell_border(row, col, side, color, width)`

セルのボーダーを設定します。

**パラメータ:**
- `side: string` - "top" | "right" | "bottom" | "left"
- `color: number` - RGBA色
- `width: number` - ボーダー幅（ピクセル）

---

## 元に戻す・やり直し

### `undo()` / `redo()`

最後のアクションを元に戻す/やり直します。

**戻り値:** `boolean` - 実行された場合true

### `can_undo()` / `can_redo()`

元に戻す/やり直しが可能かチェックします。

**戻り値:** `boolean`

---

## パフォーマンス

### `get_current_fps()`

現在のレンダリングFPSを取得します。

**戻り値:** `number` - FPS値

### `get_render_time()`

最後のフレームのレンダリング時間を取得します。

**戻り値:** `number` - レンダリング時間（ミリ秒）

### `get_memory_usage()`

おおよそのメモリ使用量を取得します。

**戻り値:** `number` - メモリ使用量（バイト）

### `reserve_capacity(expected_cells)`

メモリ容量を予約します。

### `compact_memory()`

メモリ使用量を圧縮します。

---

## ワーカースレッドサポート

### `export_grid_data_json()`

ワーカー処理用に全グリッドデータをJSONとしてエクスポートします。

**戻り値:** `string` - セルデータのJSON配列

### `export_range_json(start_row, end_row, start_col, end_col)`

特定範囲をJSONとしてエクスポートします。

### `import_worker_result(result_json)`

ワーカーから処理されたデータをインポートします。

**戻り値:** `number` - 更新されたセル数

### `apply_sorted_indices(indices_json)`

ワーカーからのソート済み行順序を適用します。

---

## コンテキストメニュー

### `get_row_context_operations(row)`

行コンテキストメニューで利用可能な操作を取得します。

**戻り値:** `Array<string>` - 操作名の配列

### `execute_row_operation(operation, row)`

行コンテキストメニュー操作を実行します。

**操作:**
- `"insert_row_above"` - 上に行を挿入
- `"insert_row_below"` - 下に行を挿入
- `"delete_row"` - 行を削除
- `"copy_row"` - 行をクリップボードにコピー
- `"cut_row"` - 行をクリップボードにカット
- `"move_row_up"` - 行を上に移動
- `"move_row_down"` - 行を下に移動

---

## 追加メソッド

### 行/列操作

- `insert_row(index)` - 新しい行を挿入
- `delete_row(index)` - 行を削除
- `insert_column(index)` - 新しい列を挿入
- `delete_column(index)` - 列を削除

### 固定表示

- `freeze_rows(count)` - 上部の行を固定
- `freeze_columns(count)` - 左側の列を固定

### ビューポート

- `scroll_to(row, col)` - セルまでスクロール
- `get_visible_range()` - 表示中のセル範囲を取得

---

詳しい例については、[使用例ガイド](./EXAMPLES.ja.md)を参照してください。
