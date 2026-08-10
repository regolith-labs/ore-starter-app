use steel::Pubkey;

pub fn format_whole_number(amount_string: String) -> String {
    // Remove any decimal portion
    let whole_part = amount_string.split('.').next().unwrap_or(&amount_string);

    // Convert to numeric to remove any existing commas
    if let Ok(num) = whole_part.replace(',', "").parse::<u64>() {
        if num >= 1_000_000 {
            format!(
                "{},{:03},{:03}",
                num / 1_000_000,
                (num % 1_000_000) / 1000,
                num % 1000
            )
        } else if num >= 1_000 {
            format!("{},{:03}", num / 1000, num % 1000)
        } else {
            num.to_string()
        }
    } else {
        amount_string
    }
}

pub fn format_time_since(timestamp: u64) -> String {
    let now = crate::utils::time::SystemTime::now()
        .duration_since(crate::utils::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let time_since = now.saturating_sub(timestamp);
    let time_until = timestamp.saturating_sub(now);
    let diff = time_since.max(time_until);

    if diff < 60 {
        format!("{} sec", diff)
    } else if diff < 3600 {
        format!("{} min", diff / 60)
    } else if diff < 86400 {
        let hours = diff / 3600;
        if hours == 1 {
            format!("1 hour")
        } else {
            format!("{} hours", hours)
        }
    } else {
        let days = diff / 86400;
        if days == 1 {
            format!("1 day")
        } else {
            format!("{} days", days)
        }
    }
}

pub fn format_abbreviated_pubkey(pubkey: Pubkey) -> String {
    let pubkey_str = pubkey.to_string();
    format!(
        "{}...{}",
        &pubkey_str[..4],
        &pubkey_str[pubkey_str.len() - 4..]
    )
}

/// Generate a deterministic gradient based on pubkey
pub fn generate_gradient_colors(pubkey: &Pubkey) -> (String, String) {
    let bytes = pubkey.to_bytes();

    // Use first 2 bytes for color hues, convert to 0-360 range
    // Use u32 to avoid overflow: max value is 255 * 360 = 91,800
    let hue1 = (bytes[0] as u32 * 360) / 255;
    let hue2 = (bytes[1] as u32 * 360) / 255;

    // Keep saturation and lightness consistent for nice looking gradients
    let color1 = format!("hsl({}, 70%, 60%)", hue1);
    let color2 = format!("hsl({}, 70%, 60%)", hue2);

    (color1, color2)
}
