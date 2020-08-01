use crate::ffi;

struct Grid {
    ptr: *mut ffi::UInt16Grid,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            ptr: unsafe { ffi::create_u16_grid(Default::default()) }
        }
    }

    fn new_with_background(background: u16) -> Grid {
        Grid {
            ptr: unsafe { ffi::create_u16_grid(background.into()) },
        }
    }

    fn accessor(&self) -> Accessor {
        Accessor {
            ptr: unsafe { ffi::u16_grid_get_accessor(self.ptr) },
        }
    }
}

impl Drop for Grid {
    fn drop(&mut self) {
        unsafe {
            ffi::delete_u16_grid(self.ptr)
        }
    }
}

struct Accessor {
    ptr: *mut ffi::UInt16GridAccessor,
}

impl Drop for Accessor {
    fn drop(&mut self) {
        unsafe {
            ffi::u16_grid_delete_accessor(self.ptr)
        }
    }
}

impl Accessor {
    #[inline]
    fn set<T>(&mut self, coords: T, value: u16)
        where T: Into<[i32; 3]> {
        unsafe {
            ffi::u16_grid_accessor_set_value_on(self.ptr, &coords.into(), value);
        }
    }

    #[inline]
    fn set_on<T>(&mut self, coords: T, value: u16)
        where T: Into<[i32; 3]> {
        self.set(coords, value);
    }

    #[inline]
    fn set_off<T>(&mut self, coords: T, value: u16)
        where T: Into<[i32; 3]> {
        unsafe {
            ffi::u16_grid_accessor_set_value_off(self.ptr, &coords.into(), value);
        }
    }

    #[inline]
    fn set_active_state<T>(&mut self, coords: T, active: bool)
        where T: Into<[i32; 3]> {
        unsafe {
            ffi::u16_grid_accessor_set_active_state(self.ptr, &coords.into(), active);
        }
    }

    #[inline]
    fn get<T>(&mut self, coords: T) -> u16
        where T: Into<[i32; 3]> {
        unsafe {
            ffi::u16_grid_accessor_get_value(self.ptr, &coords.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let grid = Grid::new();
        let mut accessor = grid.accessor();

        let coords: [i32; 3] = [1, 2, 10];
        accessor.set(coords, 7);
        assert_eq!(accessor.get(coords), 7);

        let coords: [i32; 3] = [std::i32::MAX, std::i32::MAX, std::i32::MAX];
        accessor.set(coords, 19);
        assert_eq!(accessor.get(coords), 19);


        assert_eq!(accessor.get([9, 1, 1]), 0);
    }
}
