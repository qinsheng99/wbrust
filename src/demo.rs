use {
    std::arch::asm,
    wbrust::{hm, make_answer, seac, show_streams, MyDerive},
};

fn main() {
    let o: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) o);
    }

    assert_eq!(o, 5);

    make_answer!();

    println!("{}", answer());
    println!("{}", answer1());
    A::hello_macro();

    let m = Model {
        icd: "".to_string(),
    };
    println!("{}", m.icd);
}

pub trait HM {
    fn hello_macro();
}

#[allow(dead_code)]
#[derive(MyDerive, hm)]
struct A {}
#[allow(dead_code)]
#[show_streams]
pub fn v1() {}
#[allow(dead_code)]
#[show_streams(2)]
fn v2() {}
#[allow(dead_code)]
#[derive(seac)]
#[seac_f(comment = "Macto")]
struct Model {
    #[seac_f(p = 1)]
    pub icd: String,
}
