use std::error::Error;

#[derive(clap::Parser)]
struct Args {}

trait Notify {
    fn notify(&self) -> bool;
}

pub struct FakeInstance {}

impl Notify for FakeInstance {
    fn notify(&self) -> bool {
        todo!()
    }
}

fn mock_call1(notify: &impl Notify) {}

fn mock_call2<T: Notify>(notify: &T) {}

fn mock_call3<T>(notify: &T)
where
    T: Notify,
{
}

fn mock_call4() -> Box<dyn Notify> {
    Box::new(FakeInstance {})
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.google.com";

    let resp = reqwest::get(url).await?.text().await?;

    println!("resp is {}", resp);

    Ok(())
}
