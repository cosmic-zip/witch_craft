# Use a Rust base image
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the entire mono repo into the container
COPY . /app

# Run your build script
RUN apt update && apt install -y sudo
RUN chmod +x build.sh
RUN ./build.sh
