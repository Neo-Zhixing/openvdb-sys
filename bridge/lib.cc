#include "lib.h"

namespace openvdb {
namespace bridge {

UInt16Grid* create_u16_grid(uint16_t background) {
    return new UInt16Grid(background);
}

void delete_u16_grid(UInt16Grid* grid) {
    delete grid;
}

UInt16Grid::Accessor* u16_grid_get_accessor(UInt16Grid* grid) {
    return new UInt16Grid::Accessor(grid->tree());
}


void u16_grid_delete_accessor(UInt16Grid::Accessor* accessor) {
    delete accessor;
}

}
}
