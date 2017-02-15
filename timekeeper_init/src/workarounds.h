#pragma once

// CDT does not know _Static_assert yet
#ifdef __CDT_PARSER__
#define _Static_assert(ASSERTION, MESSAGE)
#endif
