use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use std::{sync::mpsc, time::Duration};
fn main() {
    for root in [std::env::temp_dir(), std::path::PathBuf::from(std::env::args().nth(1).expect("private root"))] {
        let dir = tempfile::tempdir_in(&root).unwrap();
        let file = dir.path().join("synthetic.svg");
        std::fs::write(&file, "before").unwrap();
        let (raw_tx, raw_rx) = mpsc::channel();
        let mut raw = notify::recommended_watcher(move |e| { let _ = raw_tx.send(e); }).unwrap();
        raw.watch(dir.path(), RecursiveMode::NonRecursive).unwrap();
        let (poll_tx, poll_rx) = mpsc::channel();
        let mut poll = PollWatcher::new(move |e| { let _ = poll_tx.send(e); }, Config::default().with_poll_interval(Duration::from_millis(50)).with_compare_contents(true)).unwrap();
        poll.watch(dir.path(), RecursiveMode::NonRecursive).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        for n in 0..5 { std::fs::write(&file, format!("after-{n}")).unwrap(); std::thread::sleep(Duration::from_millis(20)); }
        std::thread::sleep(Duration::from_secs(2));
        let raw: Vec<_> = raw_rx.try_iter().collect();
        let polled: Vec<_> = poll_rx.try_iter().collect();
        println!("root={} raw_callbacks={} poll_callbacks={} raw={raw:?} polled={polled:?}", root.display(), raw.len(), polled.len());
    }
}
