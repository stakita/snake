use futures_util::io::AsyncWriteExt;
use rustyline_async::Readline;
use tokio::{select, time::sleep};

use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, mut wr) = Readline::new("> ".to_string())?;
    let mut wr2 = wr.clone();
    let event_generator = async move {
        let mut counter = 1;
        loop {
            wr2.write_all(format!("Event #{counter}\n").as_bytes())
                .await?;
            counter += 1;
            sleep(Duration::from_millis(1000)).await;
        }
        #[allow(unreachable_code)]
        Ok::<(), Box<dyn Error>>(())
    };
    let input_loop = async move {
        loop {
            let line = rl.readline().await?;
            wr.write_all(format!("Got line: {line}\n").as_bytes())
                .await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), Box<dyn Error>>(())
    };
    select! {
        Err(err) = event_generator => { Err(err)? },
        Err(err) = input_loop => { Err(err)? },
    }
    Ok(())
}
