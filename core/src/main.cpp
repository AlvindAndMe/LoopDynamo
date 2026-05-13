#include "flow.hpp"
#include "context.hpp"
#include "print_node.hpp"
#include <iostream>

int main() {
    Flow f;
    Context ctx;
    f.nodes.push_back(new PrintNode("Hello, World!"));
    f.nodes.push_back(new PrintNode("loopdynamo is running!"));
    f.nodes.push_back(new PrintNode("core up"));
    f.nodes.push_back(new PrintNode("SIX SEVEN"));
    f.run(ctx);

    for (const auto& l : ctx.logs) {
        std::cout << l << "\n";
    }
}