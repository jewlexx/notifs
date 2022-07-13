#include <wintoastlib.h>

using namespace WinToastLib;

static int IsCompatible()
{
    return WinToast::isCompatible();
}

class CustomHandler : public IWinToastHandler
{
};