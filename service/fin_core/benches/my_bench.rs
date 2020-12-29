use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fin_core::algo::*;
use fin_core::portfolio::*;
use fin_core::tic_id;

fn bench_algo(c: &mut Criterion) {
    let mut group = c.benchmark_group("Algo group");

    let mut metas = Vec::new();
    for i in 1..10 {
        let mo = i % 2;
        let action = match mo {
            0 => TickerAction::Buy,
            1 => TickerAction::Sell,
            _ => TickerAction::Hold,
        };
        metas.push(TickerMeta {
            id: tic_id!(i),
            action: action,
            ticker_value: 0.0,
            ticker_percent: 0.0,
        });
    }
    let metas: Vec<&TickerMeta> = metas.iter().collect();

    group.bench_function("filter_buys", |b| {
        b.iter(|| BuyNext::filter_buys(black_box(metas.clone())))
    });
    group.finish();
}

criterion_group!(bb, bench_algo);
criterion_main!(bb);
