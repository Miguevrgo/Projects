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
    time::Duration,
};
//todo: net info

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
fn cpu_info() -> Result<HashMap<String, u64>, std::io::Error> {
    //TODO:Jk:writeln!(j, "")
    let content = std::fs::read_to_string("/proc/stat")?;
    //HACK: Now stat does not include the % of usage, instead a times windows allows us to
    // calculate the usage in a certain time.
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

fn render_loop(stdout: &mut StdoutLock) -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    execute!(stdout, Hide)?;

    loop {
        if event::poll(Duration::from_secs(1))? && event::read()?.is_key_press() {
            break;
        }

        let mem_values = mem_info()?;
        let str = format!(
            "{}MiB/{}MiB",
            mem_values["MemFree:"] / 1024,
            mem_values["MemTotal:"] / 1024
        );
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All), Print(str))?;
    }

    execute!(stdout, Show)?;
    disable_raw_mode()
}

fn main() {
    render_loop(&mut stdout().lock()).unwrap();
}
