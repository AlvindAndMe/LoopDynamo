#pragma once
#include "node.hpp"
#include "context.hpp"
#include <string>

class PrintNode : public node {
    public:
    std::string text;

    PrintNode(const std::string& t) : text(t) {}

    void run(Context& ctx) override {
        ctx.log(text);
    }
};