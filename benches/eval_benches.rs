use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn as_str(&self) -> &'static str {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        }
    }

    fn next(&self) -> Op {
        match self {
            Op::Add => Op::Mul,
            Op::Sub => Op::Div,
            Op::Mul => Op::Sub,
            Op::Div => Op::Add,
        }
    }
}

fn generate_expr(depth: u32) -> String {
    let mut buf = String::new();
    generate(&mut buf, depth, 1, Op::Add);
    buf
}

fn generate(buf: &mut String, depth: u32, start: u32, op: Op) {
    if depth == 0 {
        buf.push_str(&start.to_string());
    } else {
        let child_op = op.next();
        let child_depth = depth - 1;

        let use_parens = child_depth > 0 && (child_op == Op::Add || child_op == Op::Sub);

        let start_left = start;
        let start_right = start + 2u32.pow(child_depth);

        if use_parens {
            buf.push('(');
        }
        generate(buf, child_depth, start_left, child_op);
        if use_parens {
            buf.push(')');
        }

        buf.push(' ');
        buf.push_str(op.as_str());
        buf.push(' ');

        if use_parens {
            buf.push('(');
        }
        generate(buf, child_depth, start_right, child_op);
        if use_parens {
            buf.push(')');
        }
    }
}

fn bench_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("eval");
    for depth in [5, 10] {
        let expr = generate_expr(depth);

        group.throughput(Throughput::Bytes(expr.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(depth), &expr, |b, expr| {
            b.iter(|| sari::eval(expr));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_eval);
criterion_main!(benches);
