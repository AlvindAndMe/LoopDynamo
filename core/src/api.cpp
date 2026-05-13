#include "api.hpp"
#include "runner.hpp"
#include <string>

static std::string LAST_RESULT;

extern "C" {
    const char* ld_run_flow(const char * json) {
        LAST_RESULT = run_flow_json(json);
        return LAST_RESULT.c_str();
    }

    }
