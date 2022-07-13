#include <wintoastlib.h>

using namespace WinToastLib;

static bool IsCompatible()
{
    return WinToast::isCompatible();
}

struct ToastHandler : public IWinToastHandler
{
    virtual void toastActivated();
};