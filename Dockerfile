# Use the official Rust image as the base image.
# This image contains the Rust toolchain needed to compile the project.
FROM rust

# Set the working directory inside the container.
WORKDIR /usr/src/event_ray

# Copy the entire project directory into the container's working directory.
COPY . .

# Compile the application in release mode.
# This creates an optimized executable binary.
RUN cargo build --release

# Inform Docker that the container listens on port 8081 at runtime.
EXPOSE 8081

# Define the default command to run when the container starts.
# This executes the compiled binary located in the target/release directory.
CMD ["./target/release/event_ray"]
