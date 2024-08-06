#[cfg(test)]
mod test {
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread::{spawn, JoinHandle};

    #[test]
    fn tests_concurrency() {
        let file_mutex: Arc<Mutex<File>> = Arc::new(Mutex::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("increments.txt")
                .unwrap(),
        ));

        let mut handles: Vec<JoinHandle<()>> = vec![];

        for i in 0..10 {
            let file_mutex: Arc<Mutex<File>> = Arc::clone(&file_mutex);
            let handle: JoinHandle<()> = spawn(move || {
                let mut file: MutexGuard<File> = file_mutex.lock().unwrap();
                writeln!(file, "{}", i).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()
        }
    }
}
