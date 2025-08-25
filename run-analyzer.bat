@echo off
echo 🐝 Instagram Matrix Analyzer 🐝
echo.
echo This will analyze your Instagram data to find non-mutual follows
echo.

REM Check if data files exist
if not exist "data-followers\followers_and_following\followers_1.json" (
    echo ❌ Error: followers_1.json not found in data-followers\followers_and_following\
    echo.
    echo Please make sure you have:
    echo 1. Downloaded your Instagram data from Meta
    echo 2. Placed followers_1.json and following.json in data-followers\followers_and_following\
    echo.
    pause
    exit /b 1
)

if not exist "data-followers\followers_and_following\following.json" (
    echo ❌ Error: following.json not found in data-followers\followers_and_following\
    echo.
    echo Please make sure you have:
    echo 1. Downloaded your Instagram data from Meta
    echo 2. Placed followers_1.json and following.json in data-followers\followers_and_following\
    echo.
    pause
    exit /b 1
)

echo ✅ Found Instagram data files
echo.
echo Starting analysis...
echo.

REM Run the analyzer
target\release\instagram-matrix-analyzer.exe -f "data-followers\followers_and_following\followers_1.json" -g "data-followers\followers_and_following\following.json"

echo.
echo Analysis complete! Press any key to exit.
pause >nul