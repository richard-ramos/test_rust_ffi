#include <stdlib.h>

typedef void (*WakuCallBack) (const char* msg,
                              size_t len_0,
                              void* user_data);

void waku_version(WakuCallBack onOkCb, void *user_data);
