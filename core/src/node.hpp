#pragma once
#include <string>
#include <vector>

class Context;

class node {
public:
    virtual ~node() {}
    virtual void run(Context& ctx) = 0;
};