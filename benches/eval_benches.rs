use divan::{counter::BytesCount, Bencher};
use std::fmt;

fn generate_expr(depth: u32) -> String {
    #[derive(Copy, Clone, PartialEq, Debug)]
    enum Op {
        Add,
        Sub,
        Mul,
        Div,
    }

    impl fmt::Display for Op {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match *self {
                Op::Add => "+",
                Op::Sub => "-",
                Op::Mul => "*",
                Op::Div => "/",
            };

            write!(f, "{}", s)
        }
    }

    impl Op {
        fn next(&self) -> Op {
            match self {
                Op::Add => Op::Mul,
                Op::Sub => Op::Div,
                Op::Mul => Op::Sub,
                Op::Div => Op::Add,
            }
        }
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
            buf.push_str(&op.to_string());
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

    let mut buf = String::new();
    generate(&mut buf, depth, 1, Op::Add);
    buf
}

#[divan::bench(args = [5, 10])]
fn bench_eval(bencher: Bencher, depth: u32) {
    bencher
        .with_inputs(|| generate_expr(depth))
        .input_counter(BytesCount::of_str)
        .bench_values(|expr| sari::eval(&expr));
}

fn main() {
    divan::main();
}
