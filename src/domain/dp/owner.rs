#[allow(dead_code)]
struct Owner(String);

pub trait OwnerT {
    #[allow(dead_code)]
    fn owner(&self) -> String;
}

impl OwnerT for Owner {
    fn owner(&self) -> String {
        self.0.clone().to_string()
    }
}

#[allow(dead_code)]
pub fn new(o: String) -> Box<dyn OwnerT> {
    Box::new(Owner(o))
}
