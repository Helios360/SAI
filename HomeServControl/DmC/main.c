#include <windows.h>
#include <stdlib.h>
#include <stdio.h>

void SimulateMouseClick(int x, int y) {
    // Move the mouse to the specified position
    SetCursorPos(x, y);

    // Create the mouse input event
    INPUT Inputs[2] = {0};
    Inputs[0].type = INPUT_MOUSE;
    Inputs[0].mi.dwFlags = MOUSEEVENTF_LEFTDOWN;
    Inputs[1].type = INPUT_MOUSE;
    Inputs[1].mi.dwFlags = MOUSEEVENTF_LEFTUP;

    // Send the mouse input event
    SendInput(2, Inputs, sizeof(INPUT));
}

int main() {
    // Open the default browser with Instagram URL
    if (system("start https://www.instagram.com") != 0) {
        printf("Failed to open the browser.\n");
        return 1;
    }

    // Simulate left mouse button click at position (x=500, y=300)
    SimulateMouseClick(500, 300);

    // Wait for a moment to allow the browser to open
    Sleep(3000);

    // Find the browser window
    HWND hwnd = FindWindow(NULL, "Instagram - firefox"); // Adjust the window title as needed
    if (hwnd) {
        // Resize the window to 800x600
        MoveWindow(hwnd, 100, 100, 800, 600, TRUE);
    } else {
        printf("Window not found.\n");
    }

    return 0;
}
