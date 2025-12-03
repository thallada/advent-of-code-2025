use color_eyre::Result;
use tracing::info;

macro_rules! days {
    ($($day_num:literal => $day_mod:ident),* $(,)?) => {
        pub fn run_days(day: Option<u8>, part: Option<u8>) -> Result<()> {
            match day {
                $(
                    Some($day_num) => $crate::runner::run_day($day_num, part, $day_mod::part1, $day_mod::part2, $day_mod::INPUT)?,
                )*
                Some(d) => color_eyre::eyre::bail!("Day {} is not yet implemented", d),
                None => {
                    $(
                        $crate::runner::run_day($day_num, None, $day_mod::part1, $day_mod::part2, $day_mod::INPUT)?;
                    )*
                }
            }
            Ok(())
        }
    };
}

pub(crate) use days;

pub fn run_day<T1, T2>(
    day: u8,
    part: Option<u8>,
    part1_fn: fn(&str) -> Result<T1>,
    part2_fn: fn(&str) -> Result<T2>,
    input: &str,
) -> Result<()>
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    info!("Day {}", day);
    let day_name = format!("{:02}", day);
    let _span = tracing::info_span!("day", day = %day_name).entered();

    if part.is_none() || part == Some(1) {
        let result = part1_fn(input)?;
        info!("Part 1: {}", result);
    }

    if part.is_none() || part == Some(2) {
        let result = part2_fn(input)?;
        info!("Part 2: {}", result);
    }

    if let Some(p) = part {
        if p != 1 && p != 2 {
            color_eyre::eyre::bail!("Part {} is invalid. Must be 1 or 2.", p);
        }
    }

    Ok(())
}
