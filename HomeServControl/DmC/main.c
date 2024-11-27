#include<windows.h>
#include<stdlib.h>
#include<shellapi.h>
#include<stdio.h>
int main() {
    // Open the default browser with Instagram URL
    // system("xdg-open https://www.instagram.com"); // For Linux
    // system("start https://www.instagram.com"); // For Windows
    // system("open https://www.instagram.com"); // For macOS

    // Move mouse to position (x=500, y=300)
    SetCursorPos(500, 300);
system("\"C:\\Program Files\\LibreWolf\\librewolf.exe\" --width 400 --height 600 https://www.instagram.com");
    // Simulate left mouse button click
    mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
    mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    //ShellExecute(0, "open", "librewolf.exe", "https://www.instagram.com", 0, SW_SHOW);

    // Wait for a moment to allow the browser to open
    Sleep(3000);

    // Find the browser window
    HWND hwnd = FindWindow(NULL, "Instagram");
    if (hwnd) {
        // Resize the window to 800x600
        MoveWindow(hwnd, 100, 100, 800, 600, TRUE);
    } else {
        printf("Window not found.\n");
    }
    return 0;
}
