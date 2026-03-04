use super::ParsedError;

pub fn parse(input: &str) -> Option<ParsedError> {
    if input.contains("Cannot connect to the Docker daemon") {
        return Some(ParsedError {
            error_type: "Docker Daemon Not Running".to_string(),
            message: input.to_string(),
            suggestion: "Start Docker with `sudo systemctl start docker` or open Docker Desktop."
                .to_string(),
        });
    }

    if input.contains("permission denied while trying to connect to the Docker daemon") {
        return Some(ParsedError {
            error_type: "Docker Permission Denied".to_string(),
            message: input.to_string(),
            suggestion: "Add your user to the docker group: `sudo usermod -aG docker $USER` then logout and login again.".to_string(),
        });
    }

    if input.contains("port is already allocated") || input.contains("address already in use") {
        return Some(ParsedError {
            error_type: "Docker Port Conflict".to_string(),
            message: input.to_string(),
            suggestion: "The port is already in use. Change the port mapping in your docker-compose.yml or stop the conflicting container with `docker ps` then `docker stop <id>`.".to_string(),
        });
    }

    if input.contains("No such image") || input.contains("pull access denied") {
        return Some(ParsedError {
            error_type: "Docker Image Not Found".to_string(),
            message: input.to_string(),
            suggestion: "Check the image name and tag. Run `docker pull <image>` or verify the image exists on hub.docker.com.".to_string(),
        });
    }

    if input.contains("no space left on device") {
        return Some(ParsedError {
            error_type: "Docker Disk Full".to_string(),
            message: input.to_string(),
            suggestion: "Clean up Docker resources: `docker system prune -a` to remove unused images, containers and volumes.".to_string(),
        });
    }

    if input.contains("OCI runtime exec failed") {
        return Some(ParsedError {
            error_type: "Docker Exec Failed".to_string(),
            message: input.to_string(),
            suggestion: "The command doesn't exist in the container. Check the container OS and use the correct shell (`bash` or `sh`).".to_string(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemon_not_running() {
        let input = "Cannot connect to the Docker daemon at unix:///var/run/docker.sock";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Docker Daemon Not Running");
        assert!(result.suggestion.contains("systemctl start docker"));
    }

    #[test]
    fn test_permission_denied() {
        let input = "permission denied while trying to connect to the Docker daemon socket";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Docker Permission Denied");
        assert!(result.suggestion.contains("usermod"));
    }

    #[test]
    fn test_port_conflict() {
        let input = "port is already allocated on 0.0.0.0:3000";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Docker Port Conflict");
    }

    #[test]
    fn test_image_not_found() {
        let input = "No such image: myapp:latest";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Docker Image Not Found");
    }

    #[test]
    fn test_disk_full() {
        let input = "no space left on device";
        let result = parse(input).unwrap();
        assert_eq!(result.error_type, "Docker Disk Full");
        assert!(result.suggestion.contains("docker system prune"));
    }

    #[test]
    fn test_no_match() {
        let input = "Container started successfully";
        assert!(parse(input).is_none());
    }
}
