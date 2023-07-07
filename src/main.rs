use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;
use xml_builder::{XMLBuilder, XMLVersion, XMLElement};

/// Create a mvn project from cli
#[derive(Parser)]
struct Cli {
    /// The project root to build the mvn project
    project_root: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.project_root);

    let mut pom_xml = XMLBuilder::new()
        .version(XMLVersion::XML1_1)
        .encoding("UTF-8".into())
        .build();

    let mut project = XMLElement::new("project");
    project.add_attribute("xmlns","http://maven.apache.org/POM/4.0.0");
    project.add_attribute("xmlns:xsi","http://www.w3.org/2001/XMLSchema-instance");
    project.add_attribute("xsi:schemaLocation", "http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd");

    let model_version = XMLElement::new("ModelVersion");
    model_version.add_text("4.0.0".to_string());
    project.add_child(model_version);

 let name = XMLElement::new("name");
    name.add_text("user input name".to_string());
    project.add_child(name);

    let group_id = XMLElement::new("groupId");
    group_id.add_text("dbonhard".to_string());
    project.add_child(group_id);

    let artifact_id = XMLElement::new("artifactId");
    artifact_id.add_text("test-project".to_string());
    project.add_child(artifact_id);

    let version = XMLElement::new("version");
    version.add_text("1.0-SNAPSHOT".to_string());
    project.add_child(version);

 let properties = XMLElement::new("properties");
 let properties_src_encoding = XMLElement::new("project.build.sourceEncoding");
 properties_src_encoding.add_text("UTF-8".to_string());
 properties.add_child(properties_src_encoding);

 let compiler_src = XMLElement::new("maven.compiler.source");
 compiler_src.add_text("20".to_string());
 properties.add_child(compiler_src);

let compiler_target = XMLElement::new("maven.compiler.target");
compiler_target.add_text("20".to_string());
properties.add_child(compiler_target);

project.add_child(properties);

 let model_version = XMLElement::new("ModelVersion");
    model_version.add_text("4.0.0".to_string());
    project.add_child(model_version);



    let model_version = XMLElement::new("ModelVersion");
    model_version.add_text("4.0.0".to_string());
    project.add_child(model_version);




        <dependencyManagement>
                <dependencies>
                        <dependency>
                                <groupId>org.junit</groupId>
                                <artifact_id>junit-bom</artifact_id>
                                <version>5.9.3</version>
                                <type>pom</type>
                                <scope>import</scope>
                        </dependency>
                </dependencies>
        </dependencyManagement>

        <dependencies>
                <dependencies>
	<dependency>
		<groupId>org.projectlombok</groupId>
		<artifact_id>lombok</artifact_id>
		<version>1.18.28</version>
		<scope>provided</scope>
	</dependency>
                <dependency>
                        <groupId>org.junit.jupiter</groupId>
                        <artifact_id>junit-jupiter</artifact_id>
                        <scope>test</scope>
                </dependency>
        </dependencies>

        <build>
                <plugins>
                        <plugin>
                                <artifact_id>maven-surefire-plugin</artifact_id>
                                <version>3.0.0</version>
                        </plugin>
                </plugins>
        </build>





    for i in 1..=2 {
        let mut room = XMLElement::new("room");
        room.add_attribute("number", &i.to_string());
        match room.add_text(format!("This is room number {}", i)) {
            Err(e) => panic!("unable to add text due to error:{}", e),
            Ok(_) => println!("Added element"),
        }

        project.add_child(room).unwrap();
    }

    pom_xml.set_root_element(project);

        let mut writer: Vec<u8> = Vec::new();
    match pom_xml.generate(&mut writer) {
        Err(e) => panic!("unable to write pom.xml"),
        Ok(_) => println!("created pom.xml file"),
    }
    match fs::create_dir_all(&args.project_root) {
        Err(e) => panic!("couldn't create dir with error {}", e),
        Ok(_) => println!("created dir with name {}", args.project_root),
    }

    let project_root = args.project_root + "/pom.xml";
    let target_path = Path::new(&project_root);
    let mut file = match File::create(&target_path) {
        Err(e) => panic!("couldn't create file with error {}", e),
        Ok(file) => file,
    };

    match file.write_all("hello world!".as_bytes()) {
        Err(e) => panic!("couldn't write to file with error {}", e),
        Ok(_) => println!("wrote to file successfully!"),
    };
}

fn add_depedency(root: XMLElement) -> XMLElement {
    let dependency = XMLElement::new("dependency");
    dependency.add_child(
                            <dependency>
                                <groupId>org.junit</groupId>
                                <artifact_id>junit-bom</artifact_id>
                                <version>5.9.3</version>
                                <type>pom</type>
                                <scope>import</scope>
                        </dependency>

    return root;
}
