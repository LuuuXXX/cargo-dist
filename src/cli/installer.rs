use installer::Generator;

#[derive(Default)]
pub struct InstallerGenerator {
        /// The name of the product, for display, Usually is the name of the tarball.
        /// The tarball name can be overided by the `package_name`.
        product_name: String,
        /// The name of the component file, distinct from other installed components
        component_name: String,
        /// The name of the packaged tarball name, like：${package_name}.tar.gz
        package_name: String,
        /// The directory containing the installation medium
        image_dir: String,
        /// The directory to do temporary work
        work_dir: String,
        /// The location to put the final image and tarball
        output_dir: String,
}

impl InstallerGenerator {
    pub fn project_name(mut self, project_name: String) -> Self {
        self.product_name = project_name;
        self
    }

    pub fn component_name(mut self, component_name: String) -> Self {
        self.component_name = component_name;
        self
    }

    pub fn package_name(mut self, package_name: String) -> Self {
        self.package_name = package_name;
        self
    }

    pub fn image_dir(mut self, image_dir: String) -> Self {
        self.image_dir = image_dir;
        self
    }

    pub fn work_dir(mut self, work_dir: String) -> Self {
        self.work_dir = work_dir;
        self
    }

    pub fn output_dir(mut self, output_dir: String) -> Self {
        self.output_dir = output_dir;
        self
    }

    // Using rust-installer to generate
    pub fn generate(self) {
        let mut generator = Generator::default();
        generator
            .product_name(self.product_name)
            .package_name(self.package_name)
            .image_dir(self.image_dir)
            .component_name(self.component_name)
            .work_dir(self.work_dir)
            .output_dir(self.output_dir);

        match generator.run() {
            Ok(_) => {
                "Dist Completed"
            },
            Err(_) => {
                panic!("failed to generate package");
            },
        };
    }
}