#![allow(non_snake_case, non_camel_case_types)]
use bitfields::bitfield;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputCurrentLimit {
    mA_100 = 0,
    mA_150 = 1,
    mA_500 = 2,
    mA_900 = 3,
    mA_1000 = 4,
    mA_1500 = 5,
    mA_2000 = 6,
    mA_3000 = 7,
}

impl InputCurrentLimit {
    pub fn as_mA(self) -> u32 {
        match self {
            InputCurrentLimit::mA_100 => 100,
            InputCurrentLimit::mA_150 => 150,
            InputCurrentLimit::mA_500 => 500,
            InputCurrentLimit::mA_900 => 900,
            InputCurrentLimit::mA_1000 => 1000,
            InputCurrentLimit::mA_1500 => 1500,
            InputCurrentLimit::mA_2000 => 2000,
            InputCurrentLimit::mA_3000 => 3000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VbusStatus {
    Unknown = 0,
    UsbHost = 1,
    AdapterPort = 2,
    Otg = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeStatus {
    NotCharging = 0,
    PreCharge = 1,
    FastCharging = 2,
    ChargeDone = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeFaultStatus {
    Normal = 0,
    InputFault = 1,
    ThermalShutdown = 2,
    ChargeTimerExpired = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtcFaultStatus {
    Normal = 0,
    Cold = 1,
    Hot = 2,
    ColdAndHot = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogTimer {
    Disabled = 0,
    Seconds40 = 1,
    Seconds80 = 2,
    Seconds160 = 3,
}

impl WatchdogTimer {
    pub fn as_seconds(self) -> u32 {
        match self {
            WatchdogTimer::Disabled => 0,
            WatchdogTimer::Seconds40 => 40,
            WatchdogTimer::Seconds80 => 80,
            WatchdogTimer::Seconds160 => 160,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeTimer {
    Hours5 = 0,
    Hours8 = 1,
    Hours12 = 2,
    Hours20 = 3,
}

impl ChargeTimer {
    pub fn as_hours(self) -> u32 {
        match self {
            ChargeTimer::Hours5 => 5,
            ChargeTimer::Hours8 => 8,
            ChargeTimer::Hours12 => 12,
            ChargeTimer::Hours20 => 20,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalRegulationThreshold {
    Celsius60 = 0,
    Celsius80 = 1,
    Celsius100 = 2,
    Celsius120 = 3,
}

impl ThermalRegulationThreshold {
    pub fn as_celsius(self) -> u32 {
        match self {
            ThermalRegulationThreshold::Celsius60 => 60,
            ThermalRegulationThreshold::Celsius80 => 80,
            ThermalRegulationThreshold::Celsius100 => 100,
            ThermalRegulationThreshold::Celsius120 => 120,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoostHotThreshold {
    Celsius55 = 0, // Vbhot1
    Celsius60 = 1, // Vbhot0
    Celsius65 = 2, // Vbhot2
    Disabled = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoostCurrentLimit {
    mA_1000 = 0,
    mA_1500 = 1,
}

impl BoostCurrentLimit {
    pub fn as_mA(self) -> u32 {
        match self {
            BoostCurrentLimit::mA_1000 => 1000,
            BoostCurrentLimit::mA_1500 => 1500,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryLowVoltageThreshold {
    mV_2800 = 0,
    mV_3000 = 1,
}

impl BatteryLowVoltageThreshold {
    pub fn as_mV(self) -> u32 {
        match self {
            BatteryLowVoltageThreshold::mV_2800 => 2800,
            BatteryLowVoltageThreshold::mV_3000 => 3000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryRechargeThreshold {
    mV_100 = 0,
    mV_300 = 1,
}

impl BatteryRechargeThreshold {
    pub fn as_mV(self) -> u32 {
        match self {
            BatteryRechargeThreshold::mV_100 => 100,
            BatteryRechargeThreshold::mV_300 => 300,
        }
    }
}

pub trait RegisterTrait<const N: usize>: RegisterAddress + From<[u8; N]> + Into<[u8; N]> {}

pub trait RegisterAddress {
    const ADDRESS: u8;
}

macro_rules! impl_simple_register_traits {
    ($ty:ident, $addr:expr) => {
        impl RegisterAddress for $ty {
            const ADDRESS: u8 = $addr;
        }
        impl From<[u8; 1]> for $ty {
            fn from(value: [u8; 1]) -> Self {
                Self::from(value[0])
            }
        }
        impl Into<[u8; 1]> for $ty {
            fn into(self) -> [u8; 1] {
                [self.into()]
            }
        }
        impl RegisterTrait<1> for $ty {}
    };
}

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct InputSourceControlRegister {
    #[bits(3)]
    IINLIM: u8,

    #[bits(4)]
    VINDPM: u8,

    #[bits(1)]
    HIZ: u8,
}

impl InputSourceControlRegister {
    pub fn is_hiz_enabled(&self) -> bool {
        self.HIZ() != 0
    }

    pub fn set_hiz_enabled(&mut self, enabled: bool) {
        self.set_HIZ(enabled.into());
    }

    pub fn get_input_voltage_dpm_mV(&self) -> u32 {
        self.VINDPM() as u32 * 80 + 3880
    }

    pub fn set_input_voltage_dpm_mV(&mut self, voltage: u32) {
        self.set_VINDPM((voltage.saturating_sub(3880) / 80) as u8);
    }

    pub fn get_input_current_limit(&self) -> InputCurrentLimit {
        match self.IINLIM() {
            0 => InputCurrentLimit::mA_100,
            1 => InputCurrentLimit::mA_150,
            2 => InputCurrentLimit::mA_500,
            3 => InputCurrentLimit::mA_900,
            4 => InputCurrentLimit::mA_1000,
            5 => InputCurrentLimit::mA_1500,
            6 => InputCurrentLimit::mA_2000,
            _ => InputCurrentLimit::mA_3000,
        }
    }

    pub fn set_input_current_limit(&mut self, limit: InputCurrentLimit) {
        self.set_IINLIM(limit as u8);
    }

    pub fn get_input_current_limit_mA(&self) -> u32 {
        self.get_input_current_limit().as_mA()
    }
}

impl_simple_register_traits!(InputSourceControlRegister, 0);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct PowerOnConfigurationRegister {
    #[bits(1)]
    BOOST_LIM: u8,

    #[bits(3)]
    SYS_MIN: u8,

    #[bits(1)]
    CHG: u8,

    #[bits(1)]
    OTG: u8,

    #[bits(1)]
    I2C_WATCHDOG_RESET: u8,

    #[bits(1)]
    REGISTER_RESET: u8,
}

impl PowerOnConfigurationRegister {
    pub fn get_boost_current_limit(&self) -> BoostCurrentLimit {
        match self.BOOST_LIM() {
            0 => BoostCurrentLimit::mA_1000,
            _ => BoostCurrentLimit::mA_1500,
        }
    }

    pub fn set_boost_current_limit(&mut self, limit: BoostCurrentLimit) {
        self.set_BOOST_LIM(limit as u8);
    }

    pub fn get_boost_current_limit_mA(&self) -> u32 {
        self.get_boost_current_limit().as_mA()
    }

    pub fn get_system_min_bus_voltage_mV(&self) -> u32 {
        self.SYS_MIN() as u32 * 100 + 3000
    }

    pub fn set_system_min_bus_voltage_mV(&mut self, voltage: u32) {
        self.set_SYS_MIN((voltage.saturating_sub(3000) / 100).min(7) as u8);
    }

    pub fn enable_otg(&mut self) {
        self.disable_charging();
        self.set_OTG(1);
    }

    pub fn enable_charging(&mut self) {
        self.disable_otg();
        self.set_CHG(1);
    }

    pub fn disable_otg(&mut self) {
        self.set_OTG(0);
    }

    pub fn disable_charging(&mut self) {
        self.set_CHG(0);
    }

    pub fn is_otg_enabled(&self) -> bool {
        self.OTG() != 0
    }

    pub fn is_charging_enabled(&self) -> bool {
        self.CHG() != 0
    }

    pub fn reset_i2c_watchdog(&mut self) {
        self.set_I2C_WATCHDOG_RESET(1);
    }

    pub fn reset_register(&mut self) {
        self.set_REGISTER_RESET(1);
    }
}

impl_simple_register_traits!(PowerOnConfigurationRegister, 1);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct ChargeCurrentControlRegister {
    #[bits(1)]
    FORCE_20PCT: u8,

    #[bits(1)]
    BCOLD: u8,

    #[bits(6)]
    ICHG: u8,
}

impl ChargeCurrentControlRegister {
    pub fn is_force_20pct_enabled(&self) -> bool {
        self.FORCE_20PCT() != 0
    }

    pub fn enable_force_20pct(&mut self) {
        self.set_FORCE_20PCT(1);
    }

    pub fn set_boost_converter_low_temp_to_m20(&mut self) {
        self.set_BCOLD(1);
    }

    pub fn set_boost_converter_low_temp_to_m10(&mut self) {
        self.set_BCOLD(0);
    }

    pub fn is_boost_convert_low_temp_m20(&self) -> bool {
        self.BCOLD() == 1
    }

    pub fn get_charge_current_limit_mA(&self) -> u32 {
        self.ICHG() as u32 * 64 + 512
    }

    pub fn set_charge_current_limit_mA(&mut self, current: u32) {
        let value = ((current.saturating_sub(512)) / 64).min(63) as u8;
        self.set_ICHG(value);
    }
}

impl_simple_register_traits!(ChargeCurrentControlRegister, 2);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct PreChargeTerminationCurrentControlRegister {
    #[bits(3)]
    ITERM: u8,

    #[bits(1, default = 0)]
    _reserved: u8,

    #[bits(4)]
    IPRECHG: u8,
}

impl PreChargeTerminationCurrentControlRegister {
    pub fn get_termination_current_mA(&self) -> u32 {
        self.ITERM() as u32 * 128
    }

    pub fn set_termination_current_mA(&mut self, current: u32) {
        let value = (current / 128).min(7) as u8;
        self.set_ITERM(value);
    }

    pub fn get_precharge_current_mA(&self) -> u32 {
        (self.IPRECHG() as u32 * 128).min(128)
    }

    pub fn set_precharge_current_mA(&mut self, current: u32) {
        let value = (current / 128).min(15) as u8;
        self.set_IPRECHG(value);
    }
}

impl_simple_register_traits!(PreChargeTerminationCurrentControlRegister, 3);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct ChargeVoltageControlRegister {
    #[bits(1)]
    VRECHG: u8,

    #[bits(1)]
    BATLOW: u8,

    #[bits(6)]
    VREG: u8,
}

impl ChargeVoltageControlRegister {
    pub fn get_charge_voltage_limit_mV(&self) -> u32 {
        self.VREG() as u32 * 16 + 3504
    }

    pub fn set_charge_voltage_limit_mV(&mut self, voltage: u32) {
        let value = ((voltage.saturating_sub(3504)) / 16).min(63) as u8;
        self.set_VREG(value);
    }

    pub fn get_battery_low_voltage_threshold(&self) -> BatteryLowVoltageThreshold {
        match self.BATLOW() {
            0 => BatteryLowVoltageThreshold::mV_2800,
            _ => BatteryLowVoltageThreshold::mV_3000,
        }
    }

    pub fn set_battery_low_voltage_threshold(&mut self, threshold: BatteryLowVoltageThreshold) {
        self.set_BATLOW(threshold as u8);
    }

    pub fn get_battery_low_voltage_threshold_mV(&self) -> u32 {
        self.get_battery_low_voltage_threshold().as_mV()
    }

    pub fn get_battery_recharge_threshold(&self) -> BatteryRechargeThreshold {
        match self.VRECHG() {
            0 => BatteryRechargeThreshold::mV_100,
            _ => BatteryRechargeThreshold::mV_300,
        }
    }

    pub fn set_battery_recharge_threshold(&mut self, threshold: BatteryRechargeThreshold) {
        self.set_VRECHG(threshold as u8);
    }

    pub fn get_battery_recharge_threshold_mV(&self) -> u32 {
        self.get_battery_recharge_threshold().as_mV()
    }
}

impl_simple_register_traits!(ChargeVoltageControlRegister, 4);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct ChargeTerminationTimerControlRegister {
    #[bits(1, default = 0)]
    _reserved: u8,

    #[bits(2)]
    CHG_TIMER: u8,

    #[bits(1)]
    EN_TIMER: u8,

    #[bits(2)]
    WATCHDOG: u8,

    #[bits(1, default = 0)]
    _reserved: u8,

    #[bits(1)]
    EN_TERM: u8,
}

impl ChargeTerminationTimerControlRegister {
    pub fn is_termination_enabled(&self) -> bool {
        self.EN_TERM() != 0
    }

    pub fn enable_termination(&mut self) {
        self.set_EN_TERM(1);
    }

    pub fn disable_termination(&mut self) {
        self.set_EN_TERM(0);
    }

    pub fn get_watchdog_timer(&self) -> WatchdogTimer {
        match self.WATCHDOG() {
            0 => WatchdogTimer::Disabled,
            1 => WatchdogTimer::Seconds40,
            2 => WatchdogTimer::Seconds80,
            _ => WatchdogTimer::Seconds160,
        }
    }

    pub fn set_watchdog_timer(&mut self, timer: WatchdogTimer) {
        self.set_WATCHDOG(timer as u8);
    }

    pub fn get_watchdog_timer_seconds(&self) -> u32 {
        self.get_watchdog_timer().as_seconds()
    }

    pub fn is_safety_timer_enabled(&self) -> bool {
        self.EN_TIMER() != 0
    }

    pub fn enable_safety_timer(&mut self) {
        self.set_EN_TIMER(1);
    }

    pub fn disable_safety_timer(&mut self) {
        self.set_EN_TIMER(0);
    }

    pub fn get_charge_timer(&self) -> ChargeTimer {
        match self.CHG_TIMER() {
            0 => ChargeTimer::Hours5,
            1 => ChargeTimer::Hours8,
            2 => ChargeTimer::Hours12,
            _ => ChargeTimer::Hours20,
        }
    }

    pub fn set_charge_timer(&mut self, timer: ChargeTimer) {
        self.set_CHG_TIMER(timer as u8);
    }

    pub fn get_charge_timer_hours(&self) -> u32 {
        self.get_charge_timer().as_hours()
    }
}

impl_simple_register_traits!(ChargeTerminationTimerControlRegister, 5);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct BoostVoltageThermalRegulationRegister {
    #[bits(2)]
    TREG: u8,

    #[bits(2)]
    BHOT: u8,

    #[bits(4)]
    BOOSTV: u8,
}

impl BoostVoltageThermalRegulationRegister {
    pub fn get_boost_voltage_mV(&self) -> u32 {
        self.BOOSTV() as u32 * 64 + 4550
    }

    pub fn set_boost_voltage_mV(&mut self, voltage: u32) {
        let value = ((voltage.saturating_sub(4550)) / 64).min(15) as u8;
        self.set_BOOSTV(value);
    }

    pub fn get_boost_hot_temperature_threshold(&self) -> BoostHotThreshold {
        match self.BHOT() {
            0 => BoostHotThreshold::Celsius55,
            1 => BoostHotThreshold::Celsius60,
            2 => BoostHotThreshold::Celsius65,
            _ => BoostHotThreshold::Disabled,
        }
    }

    pub fn set_boost_hot_temperature_threshold(&mut self, threshold: BoostHotThreshold) {
        self.set_BHOT(threshold as u8);
    }

    pub fn get_thermal_regulation_threshold(&self) -> ThermalRegulationThreshold {
        match self.TREG() {
            0 => ThermalRegulationThreshold::Celsius60,
            1 => ThermalRegulationThreshold::Celsius80,
            2 => ThermalRegulationThreshold::Celsius100,
            _ => ThermalRegulationThreshold::Celsius120,
        }
    }

    pub fn set_thermal_regulation_threshold(&mut self, threshold: ThermalRegulationThreshold) {
        self.set_TREG(threshold as u8);
    }

    pub fn get_thermal_regulation_threshold_celsius(&self) -> u32 {
        self.get_thermal_regulation_threshold().as_celsius()
    }
}

impl_simple_register_traits!(BoostVoltageThermalRegulationRegister, 6);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct MiscOperationControlRegister {
    #[bits(2)]
    INT_MASK: u8,

    #[bits(3, default = 0)]
    _reserved: u8,

    #[bits(1)]
    BATFET_DISABLE: u8,

    #[bits(1)]
    TMR2X_EN: u8,

    #[bits(1)]
    DPDM_EN: u8,
}

impl MiscOperationControlRegister {
    pub fn is_dpdm_detection_enabled(&self) -> bool {
        self.DPDM_EN() != 0
    }

    pub fn enable_dpdm_detection(&mut self) {
        self.set_DPDM_EN(1);
    }

    pub fn disable_dpdm_detection(&mut self) {
        self.set_DPDM_EN(0);
    }

    pub fn is_timer_2x_enabled(&self) -> bool {
        self.TMR2X_EN() != 0
    }

    pub fn enable_timer_2x(&mut self) {
        self.set_TMR2X_EN(1);
    }

    pub fn disable_timer_2x(&mut self) {
        self.set_TMR2X_EN(0);
    }

    pub fn is_batfet_disabled(&self) -> bool {
        self.BATFET_DISABLE() != 0
    }

    pub fn disable_batfet(&mut self) {
        self.set_BATFET_DISABLE(1);
    }

    pub fn enable_batfet(&mut self) {
        self.set_BATFET_DISABLE(0);
    }

    pub fn get_interrupt_mask(&self) -> u8 {
        self.INT_MASK()
    }

    pub fn set_interrupt_mask(&mut self, mask: u8) {
        self.set_INT_MASK(mask & 0x3);
    }

    pub fn is_charge_fault_interrupt_enabled(&self) -> bool {
        (self.INT_MASK() & 0x2) != 0
    }

    pub fn is_battery_fault_interrupt_enabled(&self) -> bool {
        (self.INT_MASK() & 0x1) != 0
    }
}

impl_simple_register_traits!(MiscOperationControlRegister, 7);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct SystemStatusRegister {
    #[bits(1)]
    VSYS_STAT: u8,

    #[bits(1)]
    THERM_STAT: u8,

    #[bits(1)]
    PG_STAT: u8,

    #[bits(1)]
    DPM_STAT: u8,

    #[bits(2)]
    CHRG_STAT: u8,

    #[bits(2)]
    VBUS_STAT: u8,
}

impl SystemStatusRegister {
    pub fn get_vbus_status(&self) -> VbusStatus {
        match self.VBUS_STAT() {
            0 => VbusStatus::Unknown,
            1 => VbusStatus::UsbHost,
            2 => VbusStatus::AdapterPort,
            _ => VbusStatus::Otg,
        }
    }

    pub fn get_charge_status(&self) -> ChargeStatus {
        match self.CHRG_STAT() {
            0 => ChargeStatus::NotCharging,
            1 => ChargeStatus::PreCharge,
            2 => ChargeStatus::FastCharging,
            _ => ChargeStatus::ChargeDone,
        }
    }

    pub fn is_dpm_active(&self) -> bool {
        self.DPM_STAT() != 0
    }

    pub fn is_power_good(&self) -> bool {
        self.PG_STAT() != 0
    }

    pub fn is_thermal_regulation_active(&self) -> bool {
        self.THERM_STAT() != 0
    }

    pub fn is_vsys_regulation_active(&self) -> bool {
        self.VSYS_STAT() != 0
    }
}

impl_simple_register_traits!(SystemStatusRegister, 8);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct NewFaultRegister {
    #[bits(2)]
    NTC_FAULT: u8,

    #[bits(1, default = 0)]
    _reserved: u8,

    #[bits(1)]
    BAT_FAULT: u8,

    #[bits(2)]
    CHARGER_FAULT: u8,

    #[bits(1)]
    OTG_FAULT: u8,

    #[bits(1)]
    WATCHDOG_FAULT: u8,
}

impl NewFaultRegister {
    pub fn is_watchdog_fault(&self) -> bool {
        self.WATCHDOG_FAULT() != 0
    }

    pub fn is_otg_fault(&self) -> bool {
        self.OTG_FAULT() != 0
    }

    pub fn get_charge_fault_status(&self) -> ChargeFaultStatus {
        match self.CHARGER_FAULT() {
            0 => ChargeFaultStatus::Normal,
            1 => ChargeFaultStatus::InputFault,
            2 => ChargeFaultStatus::ThermalShutdown,
            _ => ChargeFaultStatus::ChargeTimerExpired,
        }
    }

    pub fn is_battery_fault(&self) -> bool {
        self.BAT_FAULT() != 0
    }

    pub fn get_ntc_fault_status(&self) -> NtcFaultStatus {
        match self.NTC_FAULT() {
            0 => NtcFaultStatus::Normal,
            1 => NtcFaultStatus::Cold,
            2 => NtcFaultStatus::Hot,
            _ => NtcFaultStatus::ColdAndHot,
        }
    }

    pub fn is_ntc_cold_fault(&self) -> bool {
        (self.NTC_FAULT() & 0x1) != 0
    }

    pub fn is_ntc_hot_fault(&self) -> bool {
        (self.NTC_FAULT() & 0x2) != 0
    }
}

impl_simple_register_traits!(NewFaultRegister, 9);

#[bitfield(u8, from = true, debug = true)]
#[derive(Clone, Copy)]
pub struct VendorPartRevisionRegister {
    #[bits(3)]
    REV: u8,

    #[bits(2, default = 0)]
    _reserved: u8,

    #[bits(3)]
    PN: u8,
}

impl VendorPartRevisionRegister {
    pub fn get_part_number(&self) -> u8 {
        self.PN()
    }

    pub fn get_part_name(&self) -> &'static str {
        match self.PN() {
            1 => "bq24296M",
            _ => "Unknown",
        }
    }

    pub fn get_revision(&self) -> u8 {
        self.REV()
    }

    pub fn is_bq24296m(&self) -> bool {
        self.PN() == 1
    }
}

impl_simple_register_traits!(VendorPartRevisionRegister, 10);

macro_rules! count {
    () => {
        0usize
    };

    ($x:ident) => {
        1usize
    };

    ($x:ident, $($rest:ident),*) => {
        1 + count!($($rest),*)
    };
}

macro_rules! impl_register_traits_for_sets {
    ($ty:ident, $addr:expr, $($field:ident),*) => {
        impl RegisterAddress for $ty { const ADDRESS: u8 = $addr; }
        impl From<[u8; count!($($field),*)]> for $ty {
            fn from(value: [u8; count!($($field),*)]) -> $ty {
                let mut iter = value.into_iter();

                $ty {
                    $($field: (iter.next().unwrap()).into()),*
                }
            }
        }
        impl Into<[u8; count!($($field),*)]> for $ty {
            fn into(self) -> [u8; count!($($field),*)] {
                [
                    $(self.$field.into()),*
                ]
            }
        }
        impl RegisterTrait<{ count!($($field),*) }> for $ty {}
    };
}

#[derive(Debug)]
pub struct ConfigurationRegisters {
    pub ISCR: InputSourceControlRegister,
    pub POCR: PowerOnConfigurationRegister,
    pub CCCR: ChargeCurrentControlRegister,
    pub PCTCCR: PreChargeTerminationCurrentControlRegister,
    pub CVCR: ChargeVoltageControlRegister,
    pub CTTCR: ChargeTerminationTimerControlRegister,
    pub BVTRR: BoostVoltageThermalRegulationRegister,
    pub MOCR: MiscOperationControlRegister,
}

impl_register_traits_for_sets!(
    ConfigurationRegisters,
    0,
    ISCR,
    POCR,
    CCCR,
    PCTCCR,
    CVCR,
    CTTCR,
    BVTRR,
    MOCR
);

#[derive(Debug)]
pub struct StatusRegisters {
    pub SSR: SystemStatusRegister,
    pub NFR: NewFaultRegister,
    pub VPRR: VendorPartRevisionRegister,
}

impl_register_traits_for_sets!(StatusRegisters, 8, SSR, NFR, VPRR);
