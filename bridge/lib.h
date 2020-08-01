#include <openvdb/openvdb.h>

namespace openvdb_sys {
    using Coord = int32_t[3];
    using UInt16Tree = openvdb::tree::Tree4<uint16_t, 5, 4, 3>::Type;
    using UInt16Grid = openvdb::Grid<UInt16Tree>;
    using UInt16GridAccessor = UInt16Grid::Accessor;

    UInt16Grid* create_u16_grid(uint16_t background);
    void delete_u16_grid(UInt16Grid* grid);

    UInt16Grid::Accessor* u16_grid_get_accessor(UInt16Grid &grid);
    void u16_grid_delete_accessor(UInt16Grid::Accessor* accessor);

    void u16_grid_accessor_set_value_on(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value);
    void u16_grid_accessor_set_value_off(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value);
    void u16_grid_accessor_set_value_only(UInt16Grid::Accessor& accessor, const Coord &coords, uint16_t value);
    void u16_grid_accessor_set_active_state(UInt16Grid::Accessor& accessor, const Coord &coords, bool active);
    uint16_t u16_grid_accessor_get_value(UInt16Grid::Accessor& accessor, const Coord &coords);
}

