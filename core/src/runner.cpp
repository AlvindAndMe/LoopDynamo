#include "runner.hpp"
#include "flow.hpp"
#include "context.hpp"

std::string run_flow_json(const std::string& json) {
    Context ctx;

    ctx.log("Flow executed (stub)");

    std::string out;
    for (auto& l : ctx.logs) {
        out += l + "\n";
    }
    return out;
};