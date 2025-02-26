use bollard::container::{Config, CreateContainerOptions, StartContainerOptions};
use bollard::image::{CreateImageOptions};
use bollard::Docker;
use futures_util::StreamExt;  // Correct import
use bollard::models::{HostConfig};  // Correct import for HostConfig
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::connect_with_socket_defaults()?;

    // Define the image and container name
    let image_name = "my-python-app"; // Your built image name
    let container_name = "python-container-new";

    // Pull the image if necessary
    pull_image(&docker, image_name).await?;

    // Run the container
    run_container(&docker, image_name, container_name).await?;

    Ok(())
}

async fn pull_image(docker: &Docker, image_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Pulling image '{}'...", image_name);

    // Create image options
    let opts = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };

    let mut stream = docker.create_image(Some(opts), None, None); // Wrap in Some()

    // Handle image pull progress
    while let Some(status) = stream.next().await {
        match status {
            Ok(info) => {
                if let Some(status_str) = info.status {
                    println!("{}", status_str); // Changed to status instead of stream
                }
            }
            Err(e) => eprintln!("Error pulling image: {}", e),
        }
    }

    println!("Image '{}' pulled successfully.", image_name);
    Ok(())
}

async fn run_container(docker: &Docker, image_name: &str, container_name: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    // Define the container configuration
    let config = Config {
        image: Some(image_name),
        cmd: Some(vec!["/bin/sh", "-c", "python3 btc_price.py && while true; do sleep 1000; done"]), // Example command to run Python script
        tty: Some(true),
        open_stdin: Some(true),
        host_config: Some(HostConfig {
            auto_remove: Some(true),
            publish_all_ports: Some(true),
            ..Default::default()
        }),
        env: Some(vec!["MY_ENV_VAR=example"]),
        ..Default::default()
    };

    // Create the container
    match docker.create_container(
        Some(CreateContainerOptions {
            name: container_name,
            platform: None, // Platform is set to None (default)
        }),
        config,
    ).await {
        Ok(_) => println!("Container '{}' created successfully.", container_name),
        Err(e) => {
            println!("Failed to create container: {}", e);
            return Ok(());
        }
    }

    // Start the container
    match docker.start_container(container_name, None::<StartContainerOptions<String>>).await {
        Ok(_) => println!("Container '{}' started successfully.", container_name),
        Err(e) => {
            println!("Failed to start container: {}", e);
            return Ok(());
        }
    }

    // Attach to the container's stdout and stdin for interactive shell
    let attach_opts = bollard::container::AttachContainerOptions::<String> {
        stdout: Some(true),
        stdin: Some(true),
        stream: Some(true), // Use stream for interactivity
        logs: Some(false),
        ..Default::default()
    };

    match docker.attach_container(container_name, Some(attach_opts)).await {
        Ok(_) => println!("Attached to container '{}'.", container_name),
        Err(e) => {
            println!("Failed to attach to container: {}", e);
            return Ok(());
        }
    }

    Ok(())
}

