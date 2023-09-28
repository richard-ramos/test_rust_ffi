#include <stdio.h>
#include "myfunc.h"

void waku_version(WakuCallBack onOkCb, void *user_data)
{
  const char *str = "v0.1.0";
  onOkCb(str, 6, user_data);
}

