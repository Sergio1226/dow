use indicatif::{ProgressBar, ProgressState, ProgressStyle}; 
use std::fmt::Write;

pub struct DownloadProgress {
    pb: ProgressBar,
}

impl DownloadProgress {
    pub fn new(total: u64) -> Self {
        let pb = ProgressBar::new(total);

        pb.set_style(ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:70.bright.green}] {pos}/{len} {msg}"
           )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
        );
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        DownloadProgress { pb }
    }

    pub fn inc(&self,i:u64){
        self.pb.inc(i);
    } 

    pub fn close(&self, msg:String){
        self.pb.finish_with_message(msg);
    }
}
