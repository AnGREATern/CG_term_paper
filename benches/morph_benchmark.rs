use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Morphing::color::Color;
use Morphing::figure::merged_object::MergedObject;
use Morphing::figure::object::Object;
use Morphing::figure::projection::Projection;

pub fn criterion_benchmark(c: &mut Criterion) {
    let color = Color::new([0; 4]);
    let radius = 100.0;
    let mut group = c.benchmark_group("Морфинг");
    group.sample_size(10);

    // let st = Object::load("./models/cube.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/cube.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Куб", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });

    // let st = Object::load("./models/cylinder.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/cylinder.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Цилиндр", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });

    // let st = Object::load("./models/head.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/head.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Голова", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });

    // let st = Object::load("./models/cube.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/cylinder.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Куб+цилиндр", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });

    // let st = Object::load("./models/cube.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/radish.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Куб+редиска", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });

    let st = Object::load("./models/cylinder.obj", color.clone()).unwrap();
    let st = Projection::new(st, radius);

    let en = Object::load("./models/radish.obj", color.clone()).unwrap();
    let en = Projection::new(en, radius);

    group.bench_with_input("Цилиндр+редиска", &(st, en), |b, (st, en)| {
        b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    });

    // let st = Object::load("./models/cylinder.obj", color.clone()).unwrap();
    // let st = Projection::new(st, radius);

    // let en = Object::load("./models/head.obj", color.clone()).unwrap();
    // let en = Projection::new(en, radius);

    // group.bench_with_input("Цилиндр+голова", &(st, en), |b, (st, en)| {
    //     b.iter(|| MergedObject::new(black_box(st.clone()), black_box(en.clone())));
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
