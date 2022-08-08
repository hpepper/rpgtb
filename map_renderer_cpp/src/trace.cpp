
#include "trace.h"

#include <cstring>
#include <execinfo.h> // for backtrace
#include <dlfcn.h>    // for dladdr
#include <cxxabi.h>   // for __cxa_demangle

#include <string>
#include <sstream>

// A C++ function that will produce a stack trace with demangled function and method names.
// see: https://gist.github.com/fmela/591333/0e8f9f123c87c1f234cd3050b2dac9c76185bdf1
std::string Backtrace(int skip)
{
    void *callstack[128];
    const int nMaxFrames = sizeof(callstack) / sizeof(callstack[0]);
    char buf[1024];
    int nFrames = backtrace(callstack, nMaxFrames);
    std::ostringstream trace_buf;
    for (int i = skip; i < nFrames; i++)
    {
        Dl_info info;
        if (dladdr(callstack[i], &info))
        {
            char *demangled = NULL;
            int status;
            demangled = abi::__cxa_demangle(info.dli_sname, NULL, 0, &status);
            snprintf(buf, sizeof(buf), "demangled: %-3d %p %s + %zd\n",
                     i,
                     callstack[i],
                     status == 0 ? demangled : info.dli_sname,
                     (char *)callstack[i] - (char *)info.dli_saddr);
            free(demangled);
        }
        else
        {
            snprintf(buf, sizeof(buf), "NO DEMANGLE: %-3d %ld %p\n",
                     i, 2 + sizeof(void *) * 2, callstack[i]);
        }
        trace_buf << buf;
    }
    if (nFrames == nMaxFrames)
        trace_buf << "  [truncated]\n";
    return trace_buf.str();
}


//****************************************************************
/**
  @memo Support function for TRACE_SUBSYSF() macro :
 
  @doc Environment variables :
   TRACE_LEVEL=n           Sets debug level
   TRACE_SUBSYS=sys1,sys2  Defines which subsystems will
                           be traced.
 
  If TRACE_LEVEL is defined, but TRACE_SUBSYS is not, then
  all subsystems will be traced at the appropriate level.
 */
//****************************************************************
int DebugSubsystem(int nLevel,const char *szSubSystem) {
    char *cp;                     /* Work pointer */
    char vbuf[128];               /* Buffer for variable */
    static short nTraceLevel = -1;/* Trace level after init */
    static char *pszTraceSubSys = 0;/* Pointer to environment val*/

    /*
     * One time initialization : Test for the presence
     * of the environment variables NTRACELEVEL and
     * PSZTRACESUBSYS.
     */
    if ( nTraceLevel == -1 ) {
        pszTraceSubSys = getenv("TRACE_SUBSYS"); /* Get variable */
        if ( (cp = getenv("TRACE_LEVEL")) != 0 )
            nTraceLevel = atoi(cp);         /* Trace level */
        else
            nTraceLevel = 0;                /* No trace */
    }

    /*
     * If the NTRACELEVEL is lower than this macro
     * call, then return false :
     */
    if ( nTraceLevel < nLevel )    /* Tracing at lower lvl? */
        return 0;                 /* Yes, No trace required */

    /*
     * Return TRUE if no TRACE_SUBSYS environment
     * value is defined :
     */
    if ( !pszTraceSubSys )          /* TRACE_SUBSYS defined? */
        return 1;                 /* No, Trace ALL subsystems */

    /*
     * Copy string so we don't modify env. variable :
     */
    strncpy(vbuf,pszTraceSubSys,sizeof vbuf);
    vbuf[sizeof vbuf - 1] = 0;    /* Enforce nul byte */

    // Scan if we have a matching subsystem token :
    for ( cp=strtok(vbuf,","); cp != 0; cp=strtok(NULL,",") ) {
        if ( !strcmp(szSubSystem,cp) ) { /* Compare strings? */
            return 1;             /* Yes, trace this call */
	} 
    } // next cp
    return 0;                     /* Not in trace list */
}

