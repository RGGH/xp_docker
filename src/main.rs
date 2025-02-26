use bollard::container::{CreateContainerOptions, StartContainerOptions, Config, AttachContainerOptions, StopContainerOptions};
use bollard::models::HostConfig;
use bollard::Docker;
use bollard::image::BuildImageOptions;
use futures_util::stream::StreamExt;
use anyhow::Result;
use tar::Builder;
use std::fs::File;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    println!("Building Docker image...");

    // Create a tar file containing only the Dockerfile
    let tar_file = File::create("context.tar")?;
    let mut tar_builder = Builder::new(tar_file);

    if Path::new("Dockerfile").exists() {
        println!("Dockerfile found, adding to tar...");
        tar_builder.append_path("Dockerfile")?;
    } else {
        println!("Error: Dockerfile not found!");
        return Ok(());
    }

    tar_builder.finish()?;

    // Build options
    let build_opts = BuildImageOptions {
        t: "my-python-app:latest", // Explicitly tag as latest
        dockerfile: "Dockerfile",
        rm: true,
        forcerm: true,
        ..Default::default()
    };

    // Read the tar file
    let context = std::fs::read("context.tar")?;

    // Build the image
    let mut build_stream = docker.build_image(build_opts, None, Some(context.into()));

    // Stream the build output
    while let Some(build_result) = build_stream.next().await {
        match build_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    // Clean up the temporary tar file
    std::fs::remove_file("context.tar")?;

    println!("Docker build completed.");

    // Run the built image
    run_container(&docker, "my-python-app").await?;

    Ok(())
}

// Function to create and start the container
async fn run_container(docker: &Docker, image_name: &str) -> Result<()> {
    let container_name = "python-container";

    // Check if the container already exists
    match docker.inspect_container(container_name, None).await {
        Ok(container_info) => {
            if container_info.state.unwrap().running.unwrap() {
                println!("Stopping existing container '{}'...", container_name);
                let _ = docker.stop_container(container_name, None).await;
            }
            println!("Removing existing container '{}'...", container_name);
            let _ = docker.remove_container(container_name, None).await;
        }
        Err(_) => {
            println!("No existing container found. Proceeding...");
        }
    }

    let config = Config {
        image: Some(image_name),
        cmd: Some(vec!["/bin/sh", "-c", "python3 hello_world.py && while true; do sleep 1000; done"]), // Long-running command
        tty: Some(true), // Allocate a pseudo-TTY
        open_stdin: Some(true), // Keep stdin open for interaction
        host_config: Some(HostConfig {
            auto_remove: Some(true), // Automatically remove container after exit
            publish_all_ports: Some(true),
            ..Default::default()
        }),
        env: Some(vec!["MY_ENV_VAR=example"]), // Example environment variable
        ..Default::default()
    };

    // Create the container
    match docker.create_container(
        Some(CreateContainerOptions {
            name: container_name,
            platform: None, // Explicitly set platform to None
        }),
        config,
    ).await {
        Ok(_) => println!("Container created successfully."),
        Err(e) => {
            println!("Failed to create container: {}", e);
            return Ok(());
        }
    }

    // Start the container
    match docker.start_container(container_name, None::<StartContainerOptions<String>>).await {
        Ok(_) => println!("Container started successfully."),
        Err(e) => {
            println!("Failed to start container: {}", e);
            return Ok(());
        }
    }

    // Attach to the container's stdout and stdin for interactive shell
    let attach_opts = AttachContainerOptions::<String> {
        stdout: Some(true),
        stdin: Some(true),
        stream: Some(true), // Use stream instead of tty
        logs: Some(false),
        ..Default::default() // Set other necessary defaults
    };

    match docker.attach_container(container_name, Some(attach_opts)).await {
        Ok(_) => println!("Attached to container."),
        Err(e) => {
            println!("Failed to attach to container: {}", e);
            return Ok(());
        }
    }

    Ok(())
}

