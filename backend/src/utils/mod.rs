use chrono::{DateTime, FixedOffset, Utc};
use tracing::debug;

/// Get the current time stamp in EST
///
/// Time stamp is formatted in a way meetup API accepts
pub fn now() -> String {
    // Define the Eastern Standard Time (EST) offset
    let est_offset = FixedOffset::west_opt(4 * 3600).expect("Invalid time zone conversion"); // EST is UTC-5
    let utc_now: DateTime<Utc> = Utc::now();
    // Convert UTC to EST
    let est_now = utc_now
        .with_timezone(&est_offset)
        .format("%Y-%m-%dT%H:%M:%S-04:00[US/Eastern]")
        .to_string();

    debug!("Current date time: {est_now}");
    return est_now;
}

// /// Get the time stamp for end of today in EST
// ///
// /// Time stamp is formatted in a way meetup API accepts
// pub fn eod() -> String {
//     // Define the Eastern Standard Time (EST) offset
//     let est_offset = FixedOffset::west_opt(4 * 3600).expect("Invalid time zone conversion"); // EST is UTC-5
//     let utc_now: DateTime<Utc> = Utc::now();
//     // Convert UTC to EST
//     let est_eod = utc_now
//         .with_timezone(&est_offset)
//         .format("%Y-%m-%dT23:59:59-04:00[US/Eastern]")
//         .to_string();
//     // let eod = Local::now().format("%Y-%m-%dT23:59:59-04:00").to_string();
//     debug!("End of date time: {est_eod}");
//     return est_eod;
// }
