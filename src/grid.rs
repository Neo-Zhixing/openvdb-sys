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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let grid = Grid::new();
        let accessor = grid.accessor();
    }
}
