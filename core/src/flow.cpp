#include "flow.hpp"
#include "context.hpp"

void Flow::run(Context& ctx) {
    for (auto* n : nodes) {
        n->run(ctx);
    }
}
