#include <wintoastlib.h>

using namespace WinToastLib;

static int IsCompatible()
{
    return WinToast::isCompatible() ? 0 : 1;
}

struct ToastHandler : public IWinToastHandler
{
    virtual void toastActivated();
};