FROM rust:latest
RUN apt-get update && apt-get install -y git sudo wget p7zip-full
WORKDIR /app
CMD ["bash"]
