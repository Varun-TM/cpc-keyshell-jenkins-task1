use decimal::d128;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnitType {
  NoUnit,
  Time,
  Length,
  Area,
  Volume,
  Mass,
  DigitalStorage,
  Energy,
  Power,
  Pressure,
  Speed,
  Temperature,
}
use UnitType::*;

macro_rules! create_units {
  ( $( $variant:ident : $properties:expr ),*, ) => {
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Unit {
      $($variant),*
    }
    use Unit::*;

    impl Unit {
      pub fn category(&self) -> UnitType {
        match self {
          $(
            Unit::$variant => $properties.0
          ),*
        }
      }
      pub fn weight(&self) -> d128 {
        match self {
          $(
            Unit::$variant => $properties.1
          ),*
        }
      }
    }
  }
}

create_units!(
  NoUnit:             (UnitType::NoUnit, d128!(1)),

  Nanosecond:         (Time, d128!(1)),
  Microsecond:        (Time, d128!(1000)),
  Millisecond:        (Time, d128!(1000000)),
  Second:             (Time, d128!(1000000000)),
  Minute:             (Time, d128!(60000000000)),
  Hour:               (Time, d128!(3600000000000)),
  Day:                (Time, d128!(86400000000000)),
  Week:               (Time, d128!(604800000000000)),
  Month:              (Time, d128!(2629746000000000)),
  Quarter:            (Time, d128!(7889238000000000)),
  Year:               (Time, d128!(31556952000000000)),
  Decade:             (Time, d128!(315569520000000000)),
  Century:            (Time, d128!(3155695200000000000)),
  Millenium:          (Time, d128!(31556952000000000000)),

  Millimeter:         (Length, d128!(1)),
  Centimeter:         (Length, d128!(10)),
  Decimeter:          (Length, d128!(100)),
  Meter:              (Length, d128!(1000)),
  Kilometer:          (Length, d128!(1000000)),
  Inch:               (Length, d128!(25.4)),
  Foot:               (Length, d128!(304.8)),
  Yard:               (Length, d128!(914.4)),
  Mile:               (Length, d128!(1609344)),
  // 1-dimensional only:
  NauticalMile:       (Length, d128!(1852000)),
  LightYear:          (Length, d128!(9460730472580800000)),
  LightSecond:        (Length, d128!(299792458000)),

  SquareMillimeter:   (Area, d128!(1)),
  SquareCentimeter:   (Area, d128!(100)),
  SquareDecimeter:    (Area, d128!(10000)),
  SquareMeter:        (Area, d128!(1000000)),
  SquareKilometer:    (Area, d128!(1000000000000)),
  SquareInch:         (Area, d128!(645.16)),
  SquareFoot:         (Area, d128!(92903.04)),
  SquareYard:         (Area, d128!(836127.36)),
  SquareMile:         (Area, d128!(2589988110336.00)),
  // 2-dimensional only
  Are:                (Area, d128!(100000000)),
  Decare:             (Area, d128!(1000000000)),
  Hectare:            (Area, d128!(10000000000)),
  Acre:               (Area, d128!(4046856422.40)),

  CubicMillimeter:    (Volume, d128!(1)),
  CubicCentimeter:    (Volume, d128!(1000)),
  CubicDecimeter:     (Volume, d128!(1000000)),
  CubicMeter:         (Volume, d128!(1000000000)),
  CubicKilometer:     (Volume, d128!(1000000000000000000)),
  CubicInch:          (Volume, d128!(16387.064)),
  CubicFoot:          (Volume, d128!(28316846.592)),
  CubicYard:          (Volume, d128!(764554857.984)),
  CubicMile:          (Volume, d128!(4168181825440579584)),
  // 3-dimensional only
  Milliliter:         (Volume, d128!(1000)),
  Centiliter:         (Volume, d128!(10000)),
  Deciliter:          (Volume, d128!(100000)),
  Liter:              (Volume, d128!(1000000)),
  Teaspoon:           (Volume, d128!(4928.92159375)),
  Tablespoon:         (Volume, d128!(14786.76478125)),
  FluidOunce:         (Volume, d128!(29573.5295625)),
  Cup:                (Volume, d128!(236588.2365)),
  Pint:               (Volume, d128!(473176.473)),
  Quart:              (Volume, d128!(946352.946)),
  Gallon:             (Volume, d128!(3785411.784)),
  OilBarrel:          (Volume, d128!(158987294.928)),

  Milligram:          (Mass, d128!(0.001)),
  Gram:               (Mass, d128!(1)),
  Hectogram:          (Mass, d128!(100)),
  Kilogram:           (Mass, d128!(1000)),
  MetricTon:          (Mass, d128!(1000000)),
  Ounce:              (Mass, d128!(28.349523125)),
  Pound:              (Mass, d128!(453.59237)),
  ShortTon:           (Mass, d128!(907184.74)),
  LongTon:            (Mass, d128!(1016046.9088)),

  Bit:                (DigitalStorage, d128!(1)),
  Kilobit:            (DigitalStorage, d128!(1000)),
  Megabit:            (DigitalStorage, d128!(1000000)),
  Gigabit:            (DigitalStorage, d128!(1000000000)),
  Terabit:            (DigitalStorage, d128!(1000000000000)),
  Petabit:            (DigitalStorage, d128!(1000000000000000)),
  Exabit:             (DigitalStorage, d128!(1000000000000000000)),
  Zettabit:           (DigitalStorage, d128!(1000000000000000000000)),
  Yottabit:           (DigitalStorage, d128!(1000000000000000000000000)),
  Kibibit:            (DigitalStorage, d128!(1024)),
  Mebibit:            (DigitalStorage, d128!(1048576)),
  Gibibit:            (DigitalStorage, d128!(1073741824)),
  Tebibit:            (DigitalStorage, d128!(1099511627776)),
  Pebibit:            (DigitalStorage, d128!(1125899906842624)),
  Exbibit:            (DigitalStorage, d128!(1152921504606846976)),
  Zebibit:            (DigitalStorage, d128!(1180591620717411303424)),
  Yobibit:            (DigitalStorage, d128!(1208925819614629174706176)),
  Byte:               (DigitalStorage, d128!(8)),
  Kilobyte:           (DigitalStorage, d128!(8000)),
  Megabyte:           (DigitalStorage, d128!(8000000)),
  Gigabyte:           (DigitalStorage, d128!(8000000000)),
  Terabyte:           (DigitalStorage, d128!(8000000000000)),
  Petabyte:           (DigitalStorage, d128!(8000000000000000)),
  Exabyte:            (DigitalStorage, d128!(8000000000000000000)),
  Zettabyte:          (DigitalStorage, d128!(8000000000000000000000)),
  Yottabyte:          (DigitalStorage, d128!(8000000000000000000000000)),
  Kibibyte:           (DigitalStorage, d128!(8192)),
  Mebibyte:           (DigitalStorage, d128!(8388608)),
  Gibibyte:           (DigitalStorage, d128!(8589934592)),
  Tebibyte:           (DigitalStorage, d128!(8796093022208)),
  Pebibyte:           (DigitalStorage, d128!(9007199254740992)),
  Exbibyte:           (DigitalStorage, d128!(9223372036854775808)),
  Zebibyte:           (DigitalStorage, d128!(9444732965739290427392)),
  Yobibyte:           (DigitalStorage, d128!(9671406556917033397649408)),

  Millijoule:         (Energy, d128!(0.001)),
  Joule:              (Energy, d128!(1)),
  NewtonMeter:        (Energy, d128!(1)),
  Kilojoule:          (Energy, d128!(1000)),
  Megajoule:          (Energy, d128!(1000000)),
  Gigajoule:          (Energy, d128!(1000000000)),
  Terajoule:          (Energy, d128!(1000000000000)),
  Calorie:            (Energy, d128!(4.1868)),
  KiloCalorie:        (Energy, d128!(4186.8)),
  BritishThermalUnit: (Energy, d128!(1055.05585262)),
  WattHour:           (Energy, d128!(3600)),
  KilowattHour:       (Energy, d128!(3600000)),
  MegawattHour:       (Energy, d128!(3600000000)),
  GigawattHour:       (Energy, d128!(3600000000000)),
  TerawattHour:       (Energy, d128!(3600000000000000)),
  PetawattHour:       (Energy, d128!(3600000000000000000)),

  Milliwatt:                    (Power, d128!(0.001)),
  Watt:                         (Power, d128!(1)),
  Kilowatt:                     (Power, d128!(1000)),
  Megawatt:                     (Power, d128!(1000000)),
  Gigawatt:                     (Power, d128!(1000000000)),
  Terawatt:                     (Power, d128!(1000000000000)),
  Petawatt:                     (Power, d128!(1000000000000000)),
  BritishThermalUnitsPerMinute: (Power, d128!(0.0568690272188)), // probably inexact
  BritishThermalUnitsPerHour:   (Power, d128!(3.412141633128)), // probably inexact
  Horsepower:                   (Power, d128!(745.69987158227022)), // exact according to wikipedia
  MetricHorsepower:             (Power, d128!(735.49875)),

  Pascal:                       (Pressure, d128!(1)),
  Kilopascal:                   (Pressure, d128!(1000)),
  Atmosphere:                   (Pressure, d128!(101325)),
  Millibar:                     (Pressure, d128!(100)),
  Bar:                          (Pressure, d128!(100000)),
  InchOfMercury:                (Pressure, d128!(3386.389)),
  PoundsPerSquareInch:          (Pressure, d128!(6894.757293168361)), // inexact
  Torr:                         (Pressure, d128!(162.12)),

  KilometersPerHour:  (Speed, d128!(1)),
  MetersPerSecond:    (Speed, d128!(3.6)),
  MilesPerHour:       (Speed, d128!(1.609344)),
  FeetPerSecond:      (Speed, d128!(1.09728)),
  Knot:               (Speed, d128!(1.852)),

  Kelvin:             (Temperature, d128!(0)),
  Celcius:            (Temperature, d128!(0)),
  Fahrenheit:         (Temperature, d128!(0)),
);

#[derive(Clone, Debug)]
pub struct Number {
  pub value: d128,
  pub unit: Unit,
}

impl Number {
  pub fn new(value: d128, unit: Unit) -> Number {
    Number {
      value: value,
      unit: unit,
    }
  }
}

fn get_convertion_factor(unit: Unit, to_unit: Unit) -> d128 {
  return unit.weight() / to_unit.weight();
}

pub fn convert(number: Number, to_unit: Unit) -> Result<Number, String> {
  if number.unit.category() != to_unit.category() {
    return Err(format!("Cannot convert from {:?} to {:?}", number.unit, to_unit));
  }
  let value = number.value;
  let ok = |new_value| {
    Ok(Number::new(new_value, to_unit))
  };
  if number.unit.category() == UnitType::Temperature {
    match (number.unit, to_unit) {
      (Kelvin, Kelvin)         => ok(value),
      (Kelvin, Celcius)        => ok(value-d128!(273.15)),
      (Kelvin, Fahrenheit)     => ok(value*d128!(1.8)-d128!(459.67)),
      (Celcius, Celcius)       => ok(value),
      (Celcius, Kelvin)        => ok(value+d128!(273.15)),
      (Celcius, Fahrenheit)    => ok(value*d128!(1.8)+d128!(32)),
      (Fahrenheit, Fahrenheit) => ok(value),
      (Fahrenheit, Kelvin)     => ok((value+d128!(459.67))*d128!(5)/d128!(9)),
      (Fahrenheit, Celcius)    => ok((value-d128!(32))/d128!(1.8)),
      _ => Err(format!("Error converting temperature {:?} to {:?}", number.unit, to_unit)),
    }
  } else {
    let convertion_factor = get_convertion_factor(number.unit, to_unit);
    ok(number.value * convertion_factor)
  }
}

pub fn convert_to_lowest(left: Number, right: Number) -> Result<(Number, Number), String> {
  if left.unit.weight() == right.unit.weight() {
    Ok((left, right))
  } else if left.unit.weight() > right.unit.weight() {
    let left_converted = convert(left, right.unit)?;
    Ok((left_converted, right))
  } else {
    let right_converted = convert(right, left.unit)?;
    Ok((left, right_converted))
  }
}

pub fn add(left: Number, right: Number) -> Result<Number, String> {
  if left.unit.category() == right.unit.category() && left.unit.category() != Temperature {
    let (left, right) = convert_to_lowest(left, right)?;
    Ok(Number::new(left.value + right.value, left.unit))
  } else {
    return Err(format!("Cannot add {:?} and {:?}", left.unit, right.unit))
  }
}

pub fn subtract(left: Number, right: Number) -> Result<Number, String> {
  if left.unit.category() == right.unit.category() && left.unit.category() != Temperature {
    let (left, right) = convert_to_lowest(left, right)?;
    Ok(Number::new(left.value - right.value, left.unit))
  } else {
    return Err(format!("Cannot subtract {:?} by {:?}", left.unit, right.unit))
  }
}

pub fn multiply(left: Number, right: Number) -> Result<Number, String> {
  if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
    // 3 * 2
    return Ok(Number::new(left.value * right.value, left.unit))
  } else if left.unit.category() == Temperature || right.unit.category() == Temperature {
    // if temperature
    return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
  } else if left.unit == Unit::NoUnit && right.unit != Unit::NoUnit {
    // 3 * 1 km
    return Ok(Number::new(left.value * right.value, right.unit))
  } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
    // 1 km * 3
    return Ok(Number::new(left.value * right.value, left.unit))
  } else {
    return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
  }
}

pub fn divide(left: Number, right: Number) -> Result<Number, String> {
  if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
    // 3 / 2
    Ok(Number::new(left.value / right.value, left.unit))
  } else if left.unit.category() == Temperature || right.unit.category() == Temperature {
    // if temperature
    return Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
  } else if left.unit != Unit::NoUnit && right.unit == Unit::NoUnit {
    // 1 km / 2
    Ok(Number::new(left.value / right.value, right.unit))
  } else if left.unit.category() == right.unit.category() {
    // 1 km / 1 km
    let (left, right) = convert_to_lowest(left, right)?;
    Ok(Number::new(left.value * right.value, Unit::NoUnit))
  } else {
    Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
  }
}

pub fn modulo(left: Number, right: Number) -> Result<Number, String> {
  if left.unit.category() == Temperature || right.unit.category() == Temperature {
    // if temperature
    return Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
  } else if left.unit.category() == right.unit.category() {
    // 5 km % 3 m
    let (left, right) = convert_to_lowest(left, right)?;
    return Ok(Number::new(left.value % right.value, left.unit))
  } else {
    return Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
  }
}

pub fn pow(left: Number, right: Number) -> Result<Number, String> {
  if left.unit == Unit::NoUnit && right.unit == Unit::NoUnit {
    // 3 ^ 2
    return Ok(Number::new(left.value.pow(right.value), left.unit))
  } else if right.unit == Unit::NoUnit && left.unit != Unit::NoUnit {
    // 1 km ^ 3
    return Ok(Number::new(left.value.pow(right.value), left.unit))
  } else {
    return Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert() {
    pub fn convert_test(value: f64, unit: Unit, to_unit: Unit) -> f64 {
      use std::str::FromStr;

      let value_string = &value.to_string();
      let value_d128 = d128::from_str(value_string).unwrap();
      let number = Number::new(value_d128, unit);
      
      let result = convert(number, to_unit);
      let string_result = &result.unwrap().value.to_string();
      let float_result = f64::from_str(string_result).unwrap();

      return float_result;
    }
    assert_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
    assert_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
    assert_eq!(convert_test(1000.0, Microsecond, Millisecond), 1.0);
    assert_eq!(convert_test(1000.0, Millisecond, Second), 1.0);
    assert_eq!(convert_test(60.0, Second, Minute), 1.0);
    assert_eq!(convert_test(60.0, Minute, Hour), 1.0);
    assert_eq!(convert_test(24.0, Hour, Day), 1.0);
    assert_eq!(convert_test(7.0, Day, Week), 1.0);
    assert_eq!(convert_test(30.436875, Day, Month), 1.0);
    assert_eq!(convert_test(3.0, Month, Quarter), 1.0);
    assert_eq!(convert_test(4.0, Quarter, Year), 1.0);
    assert_eq!(convert_test(10.0, Year, Decade), 1.0);
    assert_eq!(convert_test(10.0, Decade, Century), 1.0);
    assert_eq!(convert_test(10.0, Century, Millenium), 1.0);

    assert_eq!(convert_test(10.0, Millimeter, Centimeter), 1.0);
    assert_eq!(convert_test(10.0, Centimeter, Decimeter), 1.0);
    assert_eq!(convert_test(10.0, Decimeter, Meter), 1.0);
    assert_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
    assert_eq!(convert_test(2.54, Centimeter, Inch), 1.0);
    assert_eq!(convert_test(12.0, Inch, Foot), 1.0);
    assert_eq!(convert_test(3.0, Foot, Yard), 1.0);
    assert_eq!(convert_test(1760.0, Yard, Mile), 1.0);
    assert_eq!(convert_test(1852.0, Meter, NauticalMile), 1.0);
    assert_eq!(convert_test(9460730472580800.0, Meter, LightYear), 1.0);
    assert_eq!(convert_test(299792458.0, Meter, LightSecond), 1.0);
    
    assert_eq!(convert_test(100.0, SquareMillimeter, SquareCentimeter), 1.0);
    assert_eq!(convert_test(100.0, SquareCentimeter, SquareDecimeter), 1.0);
    assert_eq!(convert_test(100.0, SquareDecimeter, SquareMeter), 1.0);
    assert_eq!(convert_test(1000000.0, SquareMeter, SquareKilometer), 1.0);
    assert_eq!(convert_test(645.16, SquareMillimeter, SquareInch), 1.0);
    assert_eq!(convert_test(144.0, SquareInch, SquareFoot), 1.0);
    assert_eq!(convert_test(9.0, SquareFoot, SquareYard), 1.0);
    assert_eq!(convert_test(3097600.0, SquareYard, SquareMile), 1.0);
    assert_eq!(convert_test(100.0, SquareMeter, Are), 1.0);
    assert_eq!(convert_test(10.0, Are, Decare), 1.0);
    assert_eq!(convert_test(10.0, Decare, Hectare), 1.0);
    assert_eq!(convert_test(640.0, Acre, SquareMile), 1.0);

    assert_eq!(convert_test(1000.0, CubicMillimeter, CubicCentimeter), 1.0);
    assert_eq!(convert_test(1000.0, CubicCentimeter, CubicDecimeter), 1.0);
    assert_eq!(convert_test(1000.0, CubicDecimeter, CubicMeter), 1.0);
    assert_eq!(convert_test(1000000000.0, CubicMeter, CubicKilometer), 1.0);
    assert_eq!(convert_test(1728.0, CubicInch, CubicFoot), 1.0);
    assert_eq!(convert_test(27.0, CubicFoot, CubicYard), 1.0);
    assert_eq!(convert_test(5451776000.0, CubicYard, CubicMile), 1.0);
    assert_eq!(convert_test(1.0, Milliliter, CubicCentimeter), 1.0);
    assert_eq!(convert_test(10.0, Milliliter, Centiliter), 1.0);
    assert_eq!(convert_test(10.0, Centiliter, Deciliter), 1.0);
    assert_eq!(convert_test(10.0, Deciliter, Liter), 1.0);
    assert_eq!(convert_test(4.92892159375, Milliliter, Teaspoon), 1.0);
    assert_eq!(convert_test(3.0, Teaspoon, Tablespoon), 1.0);
    assert_eq!(convert_test(2.0, Tablespoon, FluidOunce), 1.0);
    assert_eq!(convert_test(8.0, FluidOunce, Cup), 1.0);
    assert_eq!(convert_test(2.0, Cup, Pint), 1.0);
    assert_eq!(convert_test(2.0, Pint, Quart), 1.0);
    assert_eq!(convert_test(4.0, Quart, Gallon), 1.0);
    assert_eq!(convert_test(42.0, Gallon, OilBarrel), 1.0);

    assert_eq!(convert_test(1000.0, Milligram, Gram), 1.0);
    assert_eq!(convert_test(100.0, Gram, Hectogram), 1.0);
    assert_eq!(convert_test(1000.0, Gram, Kilogram), 1.0);
    assert_eq!(convert_test(1000.0, Kilogram, MetricTon), 1.0);
    assert_eq!(convert_test(0.45359237, Kilogram, Pound), 1.0);
    assert_eq!(convert_test(16.0, Ounce, Pound), 1.0);
    assert_eq!(convert_test(2000.0, Pound, ShortTon), 1.0);
    assert_eq!(convert_test(2240.0, Pound, LongTon), 1.0);

    assert_eq!(convert_test(1000.0, Bit, Kilobit), 1.0);
    assert_eq!(convert_test(1000.0, Kilobit, Megabit), 1.0);
    assert_eq!(convert_test(1000.0, Megabit, Gigabit), 1.0);
    assert_eq!(convert_test(1000.0, Gigabit, Terabit), 1.0);
    assert_eq!(convert_test(1000.0, Terabit, Petabit), 1.0);
    assert_eq!(convert_test(1000.0, Petabit, Exabit), 1.0);
    assert_eq!(convert_test(1000.0, Exabit, Zettabit), 1.0);
    assert_eq!(convert_test(1000.0, Zettabit, Yottabit), 1.0);
    assert_eq!(convert_test(1024.0, Bit, Kibibit), 1.0);
    assert_eq!(convert_test(1024.0, Kibibit, Mebibit), 1.0);
    assert_eq!(convert_test(1024.0, Mebibit, Gibibit), 1.0);
    assert_eq!(convert_test(1024.0, Gibibit, Tebibit), 1.0);
    assert_eq!(convert_test(1024.0, Tebibit, Pebibit), 1.0);
    assert_eq!(convert_test(1024.0, Pebibit, Exbibit), 1.0);
    assert_eq!(convert_test(1024.0, Exbibit, Zebibit), 1.0);
    assert_eq!(convert_test(1024.0, Zebibit, Yobibit), 1.0);
    assert_eq!(convert_test(8.0, Bit, Byte), 1.0);
    assert_eq!(convert_test(1000.0, Byte, Kilobyte), 1.0);
    assert_eq!(convert_test(1000.0, Kilobyte, Megabyte), 1.0);
    assert_eq!(convert_test(1000.0, Megabyte, Gigabyte), 1.0);
    assert_eq!(convert_test(1000.0, Gigabyte, Terabyte), 1.0);
    assert_eq!(convert_test(1000.0, Terabyte, Petabyte), 1.0);
    assert_eq!(convert_test(1000.0, Petabyte, Exabyte), 1.0);
    assert_eq!(convert_test(1000.0, Exabyte, Zettabyte), 1.0);
    assert_eq!(convert_test(1000.0, Zettabyte, Yottabyte), 1.0);
    assert_eq!(convert_test(1024.0, Kibibyte, Mebibyte), 1.0);
    assert_eq!(convert_test(1024.0, Mebibyte, Gibibyte), 1.0);
    assert_eq!(convert_test(1024.0, Gibibyte, Tebibyte), 1.0);
    assert_eq!(convert_test(1024.0, Tebibyte, Pebibyte), 1.0);
    assert_eq!(convert_test(1024.0, Pebibyte, Exbibyte), 1.0);
    assert_eq!(convert_test(1024.0, Exbibyte, Zebibyte), 1.0);
    assert_eq!(convert_test(1024.0, Zebibyte, Yobibyte), 1.0);


    assert_eq!(convert_test(1000.0, Millijoule, Joule), 1.0);
    assert_eq!(convert_test(1000.0, Joule, Kilojoule), 1.0);
    assert_eq!(convert_test(1.0, NewtonMeter, Joule), 1.0);
    assert_eq!(convert_test(1000.0, Kilojoule, Megajoule), 1.0);
    assert_eq!(convert_test(1000.0, Megajoule, Gigajoule), 1.0);
    assert_eq!(convert_test(1000.0, Gigajoule, Terajoule), 1.0);
    assert_eq!(convert_test(4.1868, Joule, Calorie), 1.0);
    assert_eq!(convert_test(1000.0, Calorie, KiloCalorie), 1.0);
    assert_eq!(convert_test(1055.05585262, Joule, BritishThermalUnit), 1.0);
    assert_eq!(convert_test(3600.0, Joule, WattHour), 1.0);
    assert_eq!(convert_test(1000.0, WattHour, KilowattHour), 1.0);
    assert_eq!(convert_test(1000.0, KilowattHour, MegawattHour), 1.0);
    assert_eq!(convert_test(1000.0, MegawattHour, GigawattHour), 1.0);
    assert_eq!(convert_test(1000.0, GigawattHour, TerawattHour), 1.0);
    assert_eq!(convert_test(1000.0, TerawattHour, PetawattHour), 1.0);

    assert_eq!(convert_test(1000.0, Watt, Kilowatt), 1.0);
    assert_eq!(convert_test(1000.0, Kilowatt, Megawatt), 1.0);
    assert_eq!(convert_test(1000.0, Megawatt, Gigawatt), 1.0);
    assert_eq!(convert_test(1000.0, Gigawatt, Terawatt), 1.0);
    assert_eq!(convert_test(1000.0, Terawatt, Petawatt), 1.0);
    assert_eq!(convert_test(0.0568690272188, Watt, BritishThermalUnitsPerMinute), 1.0);
    assert_eq!(convert_test(60.0, BritishThermalUnitsPerMinute, BritishThermalUnitsPerHour), 1.0);
    assert_eq!(convert_test(745.6998715822702, Watt, Horsepower), 1.0);
    assert_eq!(convert_test(735.49875, Watt, MetricHorsepower), 1.0);

    assert_eq!(convert_test(1000.0, Pascal, Kilopascal), 1.0);
    assert_eq!(convert_test(101325.0, Pascal, Atmosphere), 1.0);
    assert_eq!(convert_test(100.0, Pascal, Millibar), 1.0);
    assert_eq!(convert_test(1000.0, Millibar, Bar), 1.0);
    assert_eq!(convert_test(3386.389, Pascal, InchOfMercury), 1.0);
    assert_eq!(convert_test(6894.757293168361, Pascal, PoundsPerSquareInch), 1.0);
    assert_eq!(convert_test(162.12, Pascal, Torr), 1.0);

    assert_eq!(convert_test(3.6, KilometersPerHour, MetersPerSecond), 1.0);
    assert_eq!(convert_test(0.3048, MetersPerSecond, FeetPerSecond), 1.0);
    assert_eq!(convert_test(1.609344, KilometersPerHour, MilesPerHour), 1.0);
    assert_eq!(convert_test(1.852, KilometersPerHour, Knot), 1.0);

    assert_eq!(convert_test(274.15, Kelvin, Celcius), 1.0);
    assert_eq!(convert_test(300.0, Kelvin, Fahrenheit), 80.33);
    assert_eq!(convert_test(-272.15, Celcius, Kelvin), 1.0);
    assert_eq!(convert_test(-15.0, Celcius, Fahrenheit), 5.0);
    assert_eq!(convert_test(80.33, Fahrenheit, Kelvin), 300.0);
    assert_eq!(convert_test(5.0, Fahrenheit, Celcius), -15.0);
  }
}
