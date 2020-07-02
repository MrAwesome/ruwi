use super::{
    BluetoothDevice, BluetoothDeviceAddress, BluetoothDeviceName, BluetoothKnownDeviceIdentifier,
    BluetoothService,
};
use crate::prelude::*;

use std::convert::TryFrom;
use std::thread;
use std::time::Duration;

use blurz::bluetooth_adapter::BluetoothAdapter as BlurzBluetoothAdapter;
use blurz::bluetooth_device::BluetoothDevice as BlurzBluetoothDevice;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as BlurzBluetoothDiscoverySession;
use blurz::bluetooth_session::BluetoothSession as BlurzBluetoothSession;

pub(crate) struct BlurzController<'a, O: Global> {
    opts: &'a O,
    session: &'static BlurzBluetoothSession,
    adapter: &'static BlurzBluetoothAdapter<'static>,
}

impl<'a, O: Global> BlurzController<'a, O> {
    pub(crate) fn init(opts: &'a O) -> Result<Self, RuwiError> {
        // TODO: clean up these Box::leak usages
        let session = BlurzBluetoothSession::create_session(None).map_err(
            |e| rerr!(
                RuwiErrorKind::FailedToStartBlurzSession,
                "Failed to initialize bluetooth session using blurz! Is the bluetooth service running?",
                "blurz_err" => e
            ))?;
        let stat_session: &'static BlurzBluetoothSession = Box::leak(Box::new(session));

        let adapter = BlurzBluetoothAdapter::init(stat_session).map_err(
            |e| rerr!(
                RuwiErrorKind::FailedToStartBlurzAdapter,
                "Failed to initialize bluetooth adapter using blurz! Is the bluetooth service running?",
                "blurz_err" => e
            ))?;
        let stat_adapter: &'static BlurzBluetoothAdapter = Box::leak(Box::new(adapter));

        Ok(Self {
            opts,
            session: stat_session,
            adapter: stat_adapter,
        })
    }
}

impl<'a, O: Global> BluetoothService for BlurzController<'a, O> {
    type Opts = O;

    fn get_opts(&self) -> &O {
        self.opts
    }

    fn list_devices(&self) -> Result<Vec<BluetoothDevice>, RuwiError> {
        let blurz_device_strings = self.adapter.get_device_list().map_err(|e| {
            rerr!(
                RuwiErrorKind::FailedToListDevicesWithBlurz,
                "Failed to list devices using blurz! Is the bluetooth service running?",
                "blurz_err" => e
            )
        })?;

        let mut devices = vec![];
        for d in blurz_device_strings {
            let blurz_device = BlurzBluetoothDevice::new(&self.session, d.clone());
            let device_res = BluetoothDevice::try_from(&blurz_device);
            match device_res {
                Ok(device) => devices.push(device),
                Err(err) => eprintln!(
                    "Error \"{:?}\" encountered trying to use blurz device \"{:?}\"",
                    err, blurz_device
                ),
            }
        }

        Ok(devices)
    }

    fn power_on(&self) -> Result<(), RuwiError> {
        self.adapter.set_powered(true).map_err(|e| {
            rerr!(
                RuwiErrorKind::FailedToPowerOnBluetoothAdapterWithBlurz,
                "Failed to power on bluetooth adapter with blurz. Are you running as root?",
                "blurz_err" => e
            )
        })
    }
    fn power_off(&self) -> Result<(), RuwiError> {
        self.adapter.set_powered(false).map_err(|e| {
            rerr!(
                RuwiErrorKind::FailedToPowerOffBluetoothAdapterWithBlurz,
                "Failed to power off bluetooth adapter with blurz. Are you running as root?",
                "blurz_err" => e
            )
        })
    }
    fn scan(&self, scan_secs: usize) -> Result<(), RuwiError> {
        let session = BlurzBluetoothDiscoverySession::create_session(&self.session, self.adapter.get_id()).map_err(
            |e| rerr!(
                RuwiErrorKind::FailedToStartBlurzDiscoverySession, 
                "Failed to initialize bluetooth discovery session using blurz! Is the bluetooth service running?",
                "blurz_err" => e
                ))
            ?;
        thread::sleep(Duration::from_millis(200));
        session.start_discovery().map_err(|e| {
            rerr!(
                RuwiErrorKind::FailedToStartBlurzDiscovery,
                "Failed to discover devices using blurz! Is the bluetooth service running?",
                "blurz_err" => e
            )
        })?;
        thread::sleep(Duration::from_secs(scan_secs as u64));
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) enum BlurzDeviceParseError {
    NoAddrFound,
}

impl<'a> TryFrom<&BlurzBluetoothDevice<'a>> for BluetoothDevice {
    type Error = BlurzDeviceParseError;

    fn try_from(blurz_device: &BlurzBluetoothDevice) -> Result<Self, Self::Error> {
        let name = blurz_device
            .get_name()
            .unwrap_or_else(|_| "<no name found for device>".to_string());
        let addr = blurz_device
            .get_address()
            .map_err(|_| BlurzDeviceParseError::NoAddrFound)?;
        let known_identifier = BluetoothKnownDeviceIdentifier::Blurz(blurz_device.get_id());

        Ok(BluetoothDevice::builder()
            .name(BluetoothDeviceName(name))
            .addr(BluetoothDeviceAddress(addr))
            .known_identifier(known_identifier)
            .build())
    }
}
