#[cfg(unix)]

pub mod ipc {
    use std::io::{Read, Write};
    use std::os::unix::net::{UnixListener, UnixStream};

    pub struct Server {
        listener: UnixListener,
    }

    impl Server {
        pub fn bind(path: &str) -> std::io::Result<Self> {
            let _ = fs::remove_file(path);
            let listener = UnixListener::bind(path)?;
            Ok(Server { listener })
        }

        pub fn accept(&self) -> std::io::Result<UnixStream> {
            let (stream, _addr) = self.listener.accept()?;
            stream.write_all("Kneel Down Peasents")?;
            println!("Peasant Connected !");
            Ok(stream)
        }
    }

    pub type Client = UnixStream;

    pub fn connect(path: &str) -> std::io::Result<Client> {
        let stream = UnixStream::connect(path)?;
        println!("Connected");
        Ok(stream)
    }
}

#[cfg(windows)]
pub mod ipc {
    use std::fs::OpenOptions;
    use std::os::windows::fs::OpenOptionsExt;
    use std::thread;
    use std::time::Duration;

    pub struct Server {
        path: String,
    }

    impl Server {
        pub fn bind(path: &str) -> std::io::Result<Self> {
            Ok(Server {
                path: path.to_string(),
            })
        }

        pub fn accept(&self) -> std::io::Result<std::fs::File> {
            loop {
                if let Ok(file) = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .custom_flags(0)
                    .open(&self.path)
                {
                    println!("Peasant Connected !");
                    return Ok(file);
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    pub type Client = std::fs::File;

    pub fn connect(path: &str) -> std::io::Result<Client> {
        loop {
            if let Ok(file) = OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(0)
                .open(path)
            {
                println!("Connected");
                return Ok(file);
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}
