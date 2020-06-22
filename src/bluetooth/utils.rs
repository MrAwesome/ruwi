use crate::prelude::*;
use super::BluetoothDevice;

pub(crate) fn get_first_matching_device<'a>(
    devs: &'a [BluetoothDevice],
    prefix: &str,
) -> Result<&'a BluetoothDevice, RuwiError> {
    let matching_devs = devs
        .iter()
        .filter(|dev| dev.get_name().starts_with(prefix))
        .collect::<Vec<_>>();

    if matching_devs.is_empty() {
        Err(rerr!(
            RuwiErrorKind::NoMatchingBluetoothDeviceFoundForPrefix,
            format!("No matching devices found for prefix \"{}\"!", prefix),
        ))
    } else if matching_devs.len() == 1 {
        eprintln!("[NOTE]: Found device matching prefix \"{}\": {}", prefix, matching_devs[0]);
        Ok(matching_devs[0])
    } else {
        eprintln!(
            "[NOTE]: Multiple matching devices found for prefix \"{}\"! Using the first: {}",
            prefix, matching_devs[0]
        );
        Ok(matching_devs[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_devices() -> Vec<BluetoothDevice> {
        vec![
            BluetoothDevice::builder()
                .name("DOOK DEVICE")
                .addr("AB:13")
                .build(),
            BluetoothDevice::builder()
                .name("DOOK DEVICE 2")
                .addr("AB:14")
                .build(),
            BluetoothDevice::builder()
                .name("UNIQ DEVICE")
                .addr("AB:15")
                .build(),
            BluetoothDevice::builder()
                .name("*@(!#&@!(*@ WEIRD DEVICE !&^(!%#@!'\"9)()")
                .addr("AB:16")
                .build()
        ]
    }

    #[test]
    fn check_failure_to_find_matching_device() {
        let devs = get_devices();
        let prefix = "JLKFDJSLKJFDS";
        let first_dev = get_first_matching_device(&devs, prefix);
        assert![first_dev.is_err()];
    }

    #[test]
    fn check_first_result() {
        let devs = get_devices();
        let prefix = "DOOK";
        let first_dev = get_first_matching_device(&devs, prefix);
        assert_eq![first_dev.unwrap().get_addr(), "AB:13"];
    }

    #[test]
    fn check_uniq() {
        let devs = get_devices();
        let prefix = "UNIQ";
        let first_dev = get_first_matching_device(&devs, prefix);
        assert_eq![first_dev.unwrap().get_addr(), "AB:15"];
    }

    #[test]
    fn check_weird() {
        let devs = get_devices();
        let prefix = "*@(!";
        let first_dev = get_first_matching_device(&devs, prefix);
        assert_eq![first_dev.unwrap().get_addr(), "AB:16"];
    }
}
