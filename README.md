# AtCoder 競技プログラミング記録

- **開始日**: 2026年1月31日
- **主な使用言語**: Rust

## 使い方メモ
### 1. 新しいコンテストに参加する
```bash
cargo compete new abcXXX
cd abcXXX
```

## 2. テストと提出
```bash
# A問題をテスト
cargo compete test a

# A問題を提出
cargo compete submit a
```

## 3. トラブルシューティング
### 補完が効かない場合:
- ルートの Cargo.toml の workspace.members に abcXXX が含まれているか確認。
- VS Codeで rust-analyzer: Restart server を実行。

### バージョンエラーが出る場合:
- rust-toolchain.toml が存在し、1.89.0 になっているか確認。