#include "lib.h"

namespace openvdb_sys {

UInt16Grid* create_u16_grid(uint16_t background) {
    return new UInt16Grid(background);
}

void delete_u16_grid(UInt16Grid* grid) {
    delete grid;
}

UInt16Grid::Accessor* u16_grid_get_accessor(UInt16Grid &grid) {
    return new UInt16Grid::Accessor(grid.tree());
}


void u16_grid_delete_accessor(UInt16Grid::Accessor* accessor) {
    delete accessor;
}

void u16_grid_accessor_set_value_on(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value) {
    accessor.setValue(openvdb::Coord(coords), value);
}
void u16_grid_accessor_set_value_off(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value) {
    accessor.setValueOff(openvdb::Coord(coords), value);
}
void u16_grid_accessor_set_value_only(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value) {
    accessor.setValueOnly(openvdb::Coord(coords), value);
}
void u16_grid_accessor_set_active_state(UInt16Grid::Accessor& accessor, const Coord &coords, bool active) {
    accessor.setActiveState(openvdb::Coord(coords), active);
}

uint16_t u16_grid_accessor_get_value(UInt16Grid::Accessor& accessor, const Coord &coords) {
    return accessor.getValue(openvdb::Coord(coords));
}

}
