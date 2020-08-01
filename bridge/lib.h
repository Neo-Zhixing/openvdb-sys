#include <openvdb/openvdb.h>

namespace openvdb {
namespace bridge {
    using Coord = openvdb::Coord;
    using UInt16Tree = openvdb::tree::Tree4<uint16_t, 5, 4, 3>::Type;
    using UInt16Grid = openvdb::Grid<UInt16Tree>;
    using UInt16GridAccessor = UInt16Grid::Accessor;

    UInt16Grid* create_u16_grid(uint16_t background);
    void delete_u16_grid(UInt16Grid* grid);

    UInt16Grid::Accessor* u16_grid_get_accessor(UInt16Grid* grid);
    void u16_grid_delete_accessor(UInt16Grid::Accessor* accessor);

}
}
