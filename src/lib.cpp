#include <wintoastlib.h>

using namespace WinToastLib;

static int IsCompatible()
{
    return WinToast::isCompatible() ? 0 : 1;
}

struct CustomHandler : public IWinToastHandler
{
};