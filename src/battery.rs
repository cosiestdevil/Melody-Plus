extern crate ina219;
extern crate linux_embedded_hal as hal;

use hal::I2cdev;
use ina219::{SyncIna219};
use ina219::address::Address;
use ina219::calibration::{Calibration, IntCalibration, MicroAmpere};
use ina219::configuration::{BusVoltageRange, Configuration, MeasuredSignals, OperatingMode, Reset, Resolution, ShuntVoltageRange};

fn get_calibration()->IntCalibration {IntCalibration::new(MicroAmpere(100), 100_000).unwrap()}
pub fn setup() -> anyhow::Result<SyncIna219<I2cdev,IntCalibration>> {
    let device = I2cdev::new("/dev/i2c-1")?;
    let mut ina = SyncIna219::new_calibrated(device, Address::from_byte(0x42)?, get_calibration())?;
    ina.set_configuration(Configuration {
        // Be extra precise, but take some extra time
        bus_resolution: Resolution::Avg128,
        shunt_resolution: Resolution::Avg128,

        // We only care about low voltage bus and shunt, values larger are truncated to the max
        bus_voltage_range: BusVoltageRange::Fsr32v,
        shunt_voltage_range: ShuntVoltageRange::Fsr320mv,

        // Measure both signals continuously (default)
        operating_mode: OperatingMode::Continous(MeasuredSignals::ShutAndBusVoltage),
        reset: Reset::Run
    })?;
    Ok(ina)
}

pub fn get_percent(ina: &mut SyncIna219<I2cdev,IntCalibration>) -> Option<crate::BatteryData> {
    let voltage = ina.bus_voltage().ok()?.voltage_mv() as f64;
    let current = get_calibration().current_from_register(ina.current_raw().ok()?);
    let charge = (voltage-6000.0)/24.0;
    Some(crate::BatteryData{
        charge,
        charging:current.0>0
    })
}
