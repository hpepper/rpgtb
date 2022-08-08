#ifndef TRACE_H
#define TRACE_H

#include <iostream>
#include <string>


std::string Backtrace(int skip = 1);

int DebugSubsystem(int nLevel,const char *szSubsystem);

// https://thesoftwarecondition.com/blog/2010/08/01/cc-assertions-and-defining-your-own-assert-macro/
#define ABORT_IF_FALSE( truth, explanation ) { \
    if ( ! ( truth ) ) {\
        std::string backTrace = Backtrace(1); \
        std::cerr << "!!! Assertion failed at " << __FILE__ << ":" << __LINE__; \
        std::cerr << " inside " << __FUNCTION__ << std::endl; \
        std::cerr << "Condition:   " << #truth << std::endl; \
        std::cerr << "Explanation: " << explanation << std::endl; \
        std::cerr << backTrace << std::endl; \
        abort(); \
    } \
}

#define ABORT_IF_TRUE( truth, explanation ) { \
    if (  truth  ) {\
        std::string backTrace = Backtrace(1); \
        std::cerr << "!!! Assertion failed at " << __FILE__ << ":" << __LINE__; \
        std::cerr << " inside " << __FUNCTION__ << std::endl; \
        std::cerr << "Condition:   " << #truth << std::endl; \
        std::cerr << "Explanation: " << explanation << std::endl; \
        std::cerr << backTrace << std::endl; \
        abort(); \
    } \
}

// There should be some more code, like adding current time.
/// Dump filename, line number and error message to the stdout.
#define ERROR_TRACE_DUMP(msg) { \
        printf("*** %s @ %u : ",__FILE__,__LINE__); \
        printf msg; \
        fflush(stdout); \
}


/**************************************************************
****************************************************************/
/// Trace by TRACE_LEVEL & TRACE_SUBSYS macro :
#define TRACE_SUBSYSF(level,subsys,msg) { \
    if ( DebugSubsystem(level,subsys) ) { \
        printf("--  %s @ %6u : ",__FILE__,__LINE__); \
        printf msg; \
        fflush(stdout); \
    } \
}


#endif
