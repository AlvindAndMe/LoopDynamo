#pragma once
#include <string>
#include <vector>

class Context {
public:
    std::vector<std::string> logs;

    void log(const std::string& message) {
        logs.push_back(message);
    }
};