use std::collections::HashMap;

extern crate shiplift;
use shiplift::{ContainerOptions, Docker};

fn image_labels(image_name: &str) -> Result<HashMap<String, String>, shiplift::Error> {
    let docker = Docker::new();
    let image = try!(docker.images()
        .get(image_name)
        .inspect());

    return Ok(image.Config.Labels);
}

fn validate_stdout_logging(image_name: &str) -> Result<bool, shiplift::Error> {
    let docker = Docker::new();
    let image = docker
        .images()
        .get(image_name)
        .inspect()?;
    let containers = docker.containers();

    let options = ContainerOptions::builder(image_name).build();
    let r = containers.get(&containers.create(&options)?.Id).start()?;

    println!("{:?}", r);


    // let container = containers.create(image_name);
    // println!("huh? {:?}", container.inspect());
    return Ok(true);
}

fn validate_healthcheck_presence(image_name: &str) -> Result<bool, shiplift::Error> {
    let docker = Docker::new();
    let image = docker
        .images()
        .get(image_name)
        .inspect()?;
    match image.Config.Healthcheck {
        Some(healthcheck) => {
            match healthcheck.Test[0].as_ref() {
                "NONE" => Ok(false),
                _ => Ok(true),
            }
        },
        None => Ok(false),
    }
}


fn main() {
    println!("{:?}", image_labels("foo").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn build_fixture(name: &str) {
        Docker::new()
            .images()
            .build(
                &shiplift::BuildOptions::builder("fixtures")
                .dockerfile(name)
                .tag(name)
                .build()
            ).unwrap();

        return ();
    }

    #[test]
    fn it_detects_a_missing_healthcheck() {
        build_fixture("dockerfile_no_healthcheck");
        assert!(!validate_healthcheck_presence("dockerfile_no_healthcheck").unwrap());
    }

    #[test]
    fn it_detects_a_none_healthcheck() {
        build_fixture("dockerfile_none_healthcheck");
        assert!(!validate_healthcheck_presence("dockerfile_none_healthcheck").unwrap());
    }

    #[test]
    fn it_detects_a_healthcheck() {
        build_fixture("dockerfile_passing_healthcheck");
        assert!(validate_healthcheck_presence("dockerfile_passing_healthcheck").unwrap());
    }

    // I don't think the build method blocks until the build finishes, causing races

    // validate healthcheck works

    // #[test]
    // fn it_validates_for_stdout_logging() {
           // validate logging during health check
    //     assert!(validate_stdout_logging("boo").unwrap());
    // }
}
