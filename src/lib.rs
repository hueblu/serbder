use anyhow::*;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Server> {
        Ok(Server {
            listener: TcpListener::bind("127.0.0.1:6379").await?,
        })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (stream, _) = self.listener.accept().await?;
            process(stream).await?;
        }
    }
}

async fn process(mut stream: TcpStream) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = tokio_stream::io::LineStream::new(buf_reader.lines())
        .map(|result| result?)
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{}", data);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}