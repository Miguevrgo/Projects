use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self},
    execute,
    style::Print,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    collections::HashMap,
    io::{StdoutLock, stdout},
    time::{Duration, Instant},
    vec,
};

// mem info
fn mem_info() -> Result<HashMap<String, u64>, std::io::Error> {
    let content = std::fs::read_to_string("/proc/meminfo")?;
    Ok(content
        .lines()
        .map(|line| {
            let mut values = line.split_whitespace();
            (
                values.next().unwrap().to_owned(),
                values.next().unwrap().trim().parse::<u64>().unwrap(),
            )
        })
        .collect::<HashMap<String, u64>>())
}

// cpu info
fn cpu_info() -> Result<HashMap<String, (u64, u64)>, std::io::Error> {
    let content = std::fs::read_to_string("/proc/stat")?;
    Ok(content
        .lines()
        .filter(|line| line.starts_with("cpu"))
        .map(|line| {
            let mut values = line.split_whitespace();
            let name = values.next().unwrap().to_owned();
            let fields: Vec<u64> = values.take(8).map(|v| v.parse().unwrap()).collect();
            let idle = fields[3] + fields[4];
            (name, (idle, fields.iter().sum()))
        })
        .collect::<HashMap<String, (u64, u64)>>())
}

// net info
fn net_info() -> Result<(u64, u64), std::io::Error> {
    let mut devices = Vec::new();
    for entry in std::fs::read_dir("/sys/class/net/")? {
        let name = entry?.file_name().into_string().unwrap();
        if name != "lo" {
            devices.push(name);
        }
    }

    let (mut rx, mut tx) = (0, 0);
    for device in devices {
        rx += std::fs::read_to_string(format!("/sys/class/net/{device}/statistics/rx_bytes"))?
            .trim()
            .parse::<u64>()
            .unwrap();
        tx += std::fs::read_to_string(format!("/sys/class/net/{device}/statistics/tx_bytes"))?
            .trim()
            .parse::<u64>()
            .unwrap()
    }

    Ok((rx, tx))
}

// disk info
fn disk_info() -> Result<(u64, u64), std::io::Error> {
    let content = std::fs::read_to_string("/proc/diskstats")?;
    let (mut read, mut write) = (0, 0);

    for line in content.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if !std::path::Path::new(&format!("/sys/block/{}/device", fields[2])).exists() {
            continue;
        }
        read += fields[5].parse::<u64>().unwrap() * 512;
        write += fields[9].parse::<u64>().unwrap() * 512;
    }
    Ok((read, write))
}

// process info
fn process_info() -> Result<HashMap<u64, Vec<String>>, std::io::Error> {
    let mut pids = Vec::new();

    for entry in std::fs::read_dir("/proc")? {
        if let Ok(pid) = entry?.file_name().into_string()
            && let Ok(n_pid) = pid.parse::<u64>()
        {
            pids.push(n_pid)
        }
    }

    let mut map = HashMap::new();
    for pid in pids {
        let content = std::fs::read_to_string(format!("/proc/{pid}/stat"))?;
        let vec = content.split_whitespace().collect::<Vec<_>>();
        map.insert(
            pid,
            vec![vec[1][1..vec[1].len() - 1].to_string(), vec[23].to_string()],
        );
    }

    Ok(map)
}

fn render_loop(stdout: &mut StdoutLock) -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    let mut prev = cpu_info()?;
    let mut prev_net = net_info()?;
    let mut prev_disk = disk_info()?;
    let mut last = Instant::now();

    loop {
        if event::poll(Duration::from_secs(1))? && event::read()?.is_key_press() {
            break;
        }

        let mem_values = mem_info()?;
        let now = cpu_info()?;

        let mut str = format!(
            "MEM {}MiB/{}MiB\r\nCPU   {:5.1}%\r\n",
            mem_values["MemFree:"] / 1024,
            mem_values["MemTotal:"] / 1024,
            ((now["cpu"].1 - prev["cpu"].1) - (now["cpu"].0 - prev["cpu"].0)) as f64
                / (now["cpu"].1 - prev["cpu"].1) as f64
                * 100.0
        );

        for i in 0..now.len() - 1 {
            let k = format!("cpu{i}");
            str += &format!(
                "cpu{i:<3}{:5.1}%\r\n",
                ((now[&k].1 - prev[&k].1) - (now[&k].0 - prev[&k].0)) as f64
                    / (now[&k].1 - prev[&k].1) as f64
                    * 100.0
            );
        }

        let net = net_info()?;
        let secs = last.elapsed().as_secs_f64();
        last = Instant::now();
        let net_str = format!(
            "In: {:.1} KiB/s | Out: {:.1} KiB/s\r\n",
            net.0.saturating_sub(prev_net.0) as f64 / 1024.0 / secs,
            net.1.saturating_sub(prev_net.1) as f64 / 1024.0 / secs
        );

        let disk = disk_info()?;
        let disk_str = format!(
            "Read: {:.1} KiB/s | Write: {:.1} KiB/s\r\n",
            disk.0.saturating_sub(prev_disk.0) as f64 / 1024.0 / secs,
            disk.1.saturating_sub(prev_disk.1) as f64 / 1024.0 / secs
        );

        let process = process_info()?;
        execute!(stdout, Clear(ClearType::All))?;
        for (i, (_, proc)) in process.iter().take(20).clone().enumerate() {
            let proc_str = format!(
                "{:<18} | RSS: {}",
                proc[0].chars().take(16).collect::<String>(),
                proc[1]
            );
            execute!(stdout, MoveTo(45, i as u16), Print(proc_str))?;
        }

        prev_disk = disk;
        prev_net = net;
        prev = now;
        execute!(
            stdout,
            MoveTo(0, 0),
            Print(str),
            Print(net_str),
            Print(disk_str),
        )?;
    }

    execute!(stdout, Show)?;
    disable_raw_mode()
}

fn main() {
    render_loop(&mut stdout().lock()).unwrap();
}
