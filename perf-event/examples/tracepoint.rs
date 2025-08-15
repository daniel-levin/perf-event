use perf_event::events::Tracepoint;
use perf_event::Builder;

fn main() -> std::io::Result<()> {
    let mut counter = Builder::new(Tracepoint::with_name("net/net_dev_xmit")?)
        .any_pid()
        .one_cpu(0)
        .build()?
        .sampled(8192)?;

    counter.enable()?;

    for _ in 0..10 {
        let x = counter.next_blocking(Some(std::time::Duration::from_millis(2500)));

        if let Some(y) = x {
            dbg!(y.len());
        } else {
            println!("timeout");
        }
    }

    Ok(())
}
