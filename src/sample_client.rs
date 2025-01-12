pub trait Client: Send + Sync {
    fn invoke(&self);
}

pub struct SampleClient {}

impl Client for SampleClient {
    fn invoke(&self) {}
}

impl SampleClient {
    pub fn new() -> Self {
        SampleClient {}
    }
}
