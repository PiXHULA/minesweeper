use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use bevy::prelude::Component;
// use bevy_inspector_egui::{prelude::*, reflect_inspector};

// #[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)] // lv - add derives on demand
#[derive(Component)]
pub struct Coordinates {
    pub coord_y: u16,
    pub coord_x: u16,
}

impl Add for Coordinates {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            coord_x: self.coord_x + rhs.coord_x,
            coord_y: self.coord_y + rhs.coord_y,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (rhs_x, rhs_y): (i8,i8)) -> Self::Output {
        let x = ((self.coord_x as i16) + rhs_x as i16) as u16;
        let y = ((self.coord_y as i16) + rhs_y as i16) as u16;
        Self { coord_x: x, coord_y: y}
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub (self, rhs: Self) -> Self::Output {
        Self {
            coord_x: self.coord_x.saturating_sub(rhs.coord_x),
            coord_y: self.coord_y.saturating_sub(rhs.coord_y)
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.coord_x, self.coord_y)
    }
}

