use std::{
  sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
  },
  time::Duration,
};

use notify_debouncer_mini::{DebounceEventResult, new_debouncer};
use pugit::git::Git;

#[derive(Default)]
pub struct WatchSignals {
  head_changed: AtomicBool,
  orig_head_changed: AtomicBool,
  config_changed: AtomicBool,
  index_changed: AtomicBool,
  packed_refs_changed: AtomicBool,
  refs_head_changed: AtomicBool,
  refs_tags_changed: AtomicBool,
  refs_remotes_changed: AtomicBool,
}

impl WatchSignals {
  pub fn spawn() -> anyhow::Result<Arc<WatchSignals>> {
    let signals = Arc::new(WatchSignals::default());
    let (tx, rx) = std::sync::mpsc::channel::<anyhow::Result<()>>();

    {
      let signals = signals.clone();
      std::thread::spawn(move || {
        let result = (|| -> anyhow::Result<()> {
          let repo = Git::string_to_path("~/impl/rust/pugit/.git/")?;

          let mut debouncer = new_debouncer(Duration::from_millis(500), {
            let signals = signals.clone();
            let path = repo.clone();
            move |res: DebounceEventResult| {
              let Ok(events) = res else { return };
              for e in events {
                if e.path.starts_with(path.join("refs/heads/")) {
                  signals
                    .refs_head_changed
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                } else if e.path.starts_with(path.join("refs/tags/")) {
                  signals
                    .refs_tags_changed
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                } else if e.path.starts_with(path.join("refs/remotes/")) {
                  signals
                    .refs_remotes_changed
                    .store(true, std::sync::atomic::Ordering::Relaxed);
                }

                match e.path.file_name().and_then(|p| p.to_str()) {
                  Some("HEAD") => signals
                    .head_changed
                    .store(true, std::sync::atomic::Ordering::Relaxed),
                  Some("ORIG_HEAD") => signals.orig_head_changed.store(true, Ordering::Relaxed),
                  Some("config") => signals.config_changed.store(true, Ordering::Relaxed),
                  Some("index") => signals.index_changed.store(true, Ordering::Relaxed),
                  Some("packed-refs") => signals.packed_refs_changed.store(true, Ordering::Relaxed),
                  _ => {}
                }
              }
            }
          })?;

          for i in ["HEAD", "ORIG_HEAD", "config", "index", "packed-refs"] {
            if repo.join(i).try_exists()? {
              debouncer
                .watcher()
                .watch(&repo.join(i), notify::RecursiveMode::NonRecursive)?;
            }
          }
          for j in ["refs/heads/", "refs/tags/", "refs/remotes"] {
            if repo.join(j).try_exists()? {
              debouncer
                .watcher()
                .watch(&repo.join(j), notify::RecursiveMode::Recursive)?;
            }
          }

          tx.send(Ok(())).ok();
          // parks it in bg
          std::thread::park(); 
          Ok(())
        })();

        if let Err(e) = result {
          tx.send(Err(e)).ok(); // only reached if setup failed before park()
        }
      });
    }

    rx.recv()??; // wait for setup to finish, propagate any setup error to the caller
    Ok(signals)
  }
}
