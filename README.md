# Oxwalk

OxwalkはRust製の歩行者群衆シミュレータです。歩行者モデルにはHelbingらのSocial Force Model (SFM) [^1]を使用しています。

## 機能

- [x] 基礎的なSFMの実装
- [x] 任意の形状の障害物/目的地をサポート
- [x] シミュレーション結果のGIFアニメーション出力

## 使い方

### 0. 前提条件

Oxwalkの実行にはRustのコンパイラのインストールが必要です。https://www.rust-lang.org/ja/tools/install の手順に従ってインストールしてください。

### 1. 実行

次のコマンドをレポジトリのディレクトリで実行してください。

```sh
cargo run -r
# -r（リリースビルド）オプションを有効にしてください。
# オプションを指定しない場合、パフォーマンスが低下します。
```

シミュレーション結果のGIFアニメーションは `/output` ディレクトリ内に作成されます。

### 2. シナリオの編集

デフォルトでは、双方向の歩行者が直線状の通路を通過するシナリオが用意されています。シナリオを変更するには、`src/main.rs` を編集します。デフォルトのシナリオは次のようになっています。

```rust
fn main() {
    // ...

    // シミュレータの状態を作成
    let mut state = State::default();

    // 1つ目の目的地を追加 (ID: 0)
    // 追加した順にIDが割り当てられます。
    state.add_destination(Destination::new(vec![dvec2(1.0, 1.0), dvec2(1.0, 3.0)]));

    // 2つ目の目的地を追加 (ID: 1)
    state.add_destination(Destination::new(vec![dvec2(9.0, 1.0), dvec2(9.0, 3.0)]));

    // 障害物の追加
    state.add_obstacle(Obstacle::new(vec![dvec2(0.0, 0.0), dvec2(10.0, 0.0)]));
    state.add_obstacle(Obstacle::new(vec![dvec2(0.0, 4.0), dvec2(10.0, 4.0)]));
    state.add_obstacle(Obstacle::new(vec![
        dvec2(5.0, 1.0),
        dvec2(6.0, 2.0),
        dvec2(5.0, 3.0),
        dvec2(4.0, 2.0),
        dvec2(5.0, 1.0),
    ]));

    // メインの更新処理
    for step in 0..1000 {
        // ポワソン分布に従って歩行者を発生させる
        for _ in 0..util::poisson(0.1) {
            // 左端から右（目的地1）に移動する歩行者を追加
            state.spawn_pedestrian(Pedestrian {
                position: dvec2(1.0, 1.0 + fastrand::f64() * 2.0),
                destination_id: 1,
                ..Default::default()
            });
        }
        for _ in 0..util::poisson(0.1) {
            // 右端から左（目的地0）に移動する歩行者を追加
            state.spawn_pedestrian(Pedestrian {
                position: dvec2(9.0, 1.0 + fastrand::f64() * 2.0),
                destination_id: 0,
                ..Default::default()
            });
        }

        // 歩行者を移動
        state.tick();

        // 2ステップおきにGIFフレームを追加
        if step % 2 == 0 {
            visualizer.render(step, &state);
        }
        // 100ステップおきに進捗を表示
        if step % 100 == 0 {
            let active_pedestrians = state.pedestrians.iter().filter(|p| p.active).count();
            println!("Step {step}: {active_pedestrians} pedestrians");
        }
    }
}
```

### 3. モデルの編集

必要に応じて、既存の歩行者モデルに機能を追加したり、別のモデルを使用したりすることができます。歩行者モデルは `src/model/mod.rs` に実装されているため、必要に応じて編集してください。

## ファイルの説明
- `src/main.rs`: メイン関数が含まれるファイルで、シナリオが記述されています。
- `src/model/mod.rs`: 歩行者モデルが実装されています。
- `src/visualizer.rs`: アニメーションの描画に関するコードが記述されています。
- `src/util.rs`: 確率や座標計算に関する関数群が実装されています。

[^1]: D. Helbing and P. Molnár, “Social force model for pedestrian dynamics,” Phys. Rev. E, vol. 51, no. 5, pp. 4282–4286, May 1995, https://arxiv.org/pdf/cond-mat/9805244. 