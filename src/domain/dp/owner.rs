struct Owner(String);

pub trait OwnerT {
    fn owner(&self) -> String;
}

impl OwnerT for Owner {
    fn owner(&self) -> String {
        self.0.clone().to_string()
    }
}

pub fn new(o: String) -> Box<dyn OwnerT> {
    Box::new(Owner(o))
}
