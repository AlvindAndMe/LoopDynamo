#pragma once
#include "node.hpp"
#include <vector>

class Flow {
public:
std::vector<node*> nodes;
void run(class Context& ctx);
};