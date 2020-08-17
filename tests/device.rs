use saleae::device::{ConnectedDevice, DeviceID};
use std::str::FromStr;

// Sample from C# API
//1, Logic 8, LOGIC_8_DEVICE, 0x2dc9, ACTIVE
//2, Logic Pro 8, LOGIC_PRO_8_DEVICE, 0x7243
//3, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0x673f
//4, Logic 4, LOGIC_4_DEVICE, 0x6709
#[test]
fn test_from_str() {
    // test whether from_str performs as expected
    assert_eq!(
        ConnectedDevice::from_str(
            "1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3, ACTIVE"
        )
        .unwrap(),
        ConnectedDevice {
            d_type: "1".to_string(),
            name: "Logic Pro 16".to_string(),
            device_id: DeviceID::LOGIC_PRO_16_DEVICE,
            index: "0xdf03c43d1f3aa2f3".to_string(),
            is_active: true,
        }
    );
    assert_eq!(
        ConnectedDevice::from_str("2, Logic 8, LOGIC_8_DEVICE, 0xffffffffff, ACTIVE").unwrap(),
        ConnectedDevice {
            d_type: "2".to_string(),
            name: "Logic 8".to_string(),
            device_id: DeviceID::LOGIC_8_DEVICE,
            index: "0xffffffffff".to_string(),
            is_active: true,
        }
    );
    assert_eq!(
        ConnectedDevice::from_str("1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3")
            .unwrap(),
        ConnectedDevice {
            d_type: "1".to_string(),
            name: "Logic Pro 16".to_string(),
            device_id: DeviceID::LOGIC_PRO_16_DEVICE,
            index: "0xdf03c43d1f3aa2f3".to_string(),
            is_active: false,
        }
    );
}
