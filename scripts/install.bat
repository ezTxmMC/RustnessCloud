@echo off

IF "%EUID%" NEQ "0" (
  echo "Please run as root"
  exit "1"
)
echo "Do you want to download RustnessCloud? (y/n)"
read "wantInstall"
IF "%wantInstall%" NEQ "y" (
  echo "Aborting..."
  exit "1"
)
echo "Downloading RustnessCloud..."
sleep "0.5"
curl "https://github.com/RustServices/RustnessCloud/releases/latest/download/rustnesscloud.zip"
echo "Unzipping RustnessCloud..."
sleep "0.5"
Expand-Archive -Force 'rustnesscloud.zip' '.'
echo "RustnessCloud installation completed successfully."
echo "Be sure to have installed Java 22 or higher."
echo "Do you want to start RustnessCloud? (y/n)"
read "wantStart"
IF "%wantStart%" "=" "y" (
  call RustnessCloud.exe
) ELSE (
  echo "Exiting."
)

EXIT /B %ERRORLEVEL%
EXIT /B 0