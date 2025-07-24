use embedded_hal::i2c::{I2c, Operation};

use crate::regs::RegisterTrait;

pub struct BQ24296<I2C: I2c> {
    bus: I2C,
}

impl<I2C: I2c> BQ24296<I2C> {
    const ADDRESS: u8 = 0x6b;

    pub fn new(i2c: I2C) -> Self {
        BQ24296 { bus: i2c }
    }

    pub fn read<Register: RegisterTrait<N>, const N: usize>(&mut self) -> Result<Register, I2C::Error> {
        let mut v = [0u8; N];
        self.bus.write_read(Self::ADDRESS, &[Register::ADDRESS], &mut v)?;
        Ok(Register::from(v))
    }

    pub fn write<Register: RegisterTrait<N>, const N: usize>(&mut self, register: Register) -> Result<(), I2C::Error> {
        self.bus.transaction(Self::ADDRESS, &mut [
            Operation::Write(&[Register::ADDRESS]),
            Operation::Write(&register.into())
        ])
    }

    pub fn transact<Register: RegisterTrait<N>, const N: usize>(&mut self, f: impl FnOnce(&mut Register)) -> Result<(), I2C::Error> {
        let mut register = self.read()?;
        f(&mut register);
        self.write(register)
    }

}

#[cfg(test)]
mod test {
    use embedded_hal_mock::eh1::i2c::Mock as MockedI2C;
    use embedded_hal_mock::eh1::i2c::Transaction as I2cTransaction;

    use crate::ConfigurationRegisters;
    use crate::InputSourceControlRegister;

    use super::BQ24296;

    fn test_single_register() {
        let expectations = [];
        let i2c = MockedI2C::new(expectations);

        let mut device = BQ24296::new(i2c);
        let r: InputSourceControlRegister = device.read().unwrap();
    }

    fn test_register_sets() {
        let expectations = [];
        let i2c = MockedI2C::new(expectations);

        let mut device = BQ24296::new(i2c);
        let r: ConfigurationRegisters = device.read().unwrap();
    }
}
