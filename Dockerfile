# Use Ubuntu as the base image
FROM ubuntu:latest

# Install required packages, including Chromium
RUN apt-get update && apt-get install -y git sudo npm
RUN npx @puppeteer/browsers install chrome@stable

# Set the working directory
WORKDIR /app

# Clone the repository directly into the working directory
RUN git clone https://github.com/cosmic-zip/witch_craft --branch=trunk --depth=1

# Change the working directory to the cloned repository
WORKDIR /app/witch_craft

# Ensure the build.sh script has execute permissions
RUN chmod +x build.sh

# Run the build.sh script
RUN ./build.sh

# Default command
CMD ["bash"]
