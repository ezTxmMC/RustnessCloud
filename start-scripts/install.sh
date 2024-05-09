if [ "$EUID" -ne 0 ]; then
    echo "Please run as root"
    exit 1
fi

echo "Do you want to download RustnessCloud? (y/n)"
read wantInstall

if [ "$wantInstall" != "y" ]; then
    echo "Aborting..."
    exit 1
fi

# Function to install package if missing
install_package_if_missing() {
    local package="$1"
    hash "$package" 2>/dev/null || {
        echo >&2 "You need to install $package. Do you want to install it? (y/n)";
        read wantInstall
        if [ "$wantInstall" = "y" ]; then
            sudo apt install "$package" -y
        else
            echo "Aborting!"
            exit 1
        fi
    }
}

# Check and install required packages
install_package_if_missing "wget"
install_package_if_missing "unzip"
install_package_if_missing "screen"

# Check and install Java
hash java 2>/dev/null || {
    echo >&2 "You need to install Java. Do you want to install it? (y/n)";
    read wantInstall
    if [ "$wantInstall" = "y" ]; then
        sudo apt install -y software-properties-common
        sudo echo "deb [arch=amd64] https://some.repository.url focal main" | sudo tee /etc/apt/sources.list.d/adoptium.list > /dev/null
        sudo apt install -y wget apt-transport-https gpg
        sudo wget -qO - https://packages.adoptium.net/artifactory/api/gpg/key/public | sudo gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/adoptium.gpg > /dev/null
        sudo echo "deb https://packages.adoptium.net/artifactory/deb $(awk -F= '/^VERSION_CODENAME/{print$2}' /etc/os-release) main" | sudo tee /etc/apt/sources.list.d/adoptium.list
        sudo apt install -y temurin-22-jdk
    else
        echo "Aborting..."
        exit 1
    fi
}
hash java 2>/dev/null || {
    echo >&2 "You need to install Java. Do you want to install it? (y/n)";
    read wantInstall
    if [ "$wantInstall" = "y" ]; then
        curl https://sh.rustup.rs -sSf | sh
    else
        echo "Aborting..."
        exit 1
    fi
}

# Download and install RustnessCloud
echo "Downloading RustnessCloud..."
sleep 0.5

wget https://github.com/RustServices/RustnessCloud/releases/latest/download/rustnesscloud.zip

echo "Unzipping RustnessCloud..."
sleep 0.5

unzip rustnesscloud.zip
chmod a+x start.sh

echo "RustnessCloud installation completed successfully."

# Prompt to start RustnessCloud
echo "Do you want to start RustnessCloud? (y/n)"
read wantStart

if [ "$wantStart" = "y" ]; then
    bash start.sh
else
    echo "Exiting."
fi