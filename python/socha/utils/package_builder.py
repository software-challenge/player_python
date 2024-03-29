import gc
import inspect
import logging
import os
import shutil

# trunk-ignore(bandit/B404)
import subprocess
import sys
import time
import types
import zipfile


class SochaPackageBuilder:

    def __init__(self, package_name, architecture):
        self.package_name = package_name
        self.architecture = architecture
        self.dependencies_dir = "dependencies"
        self.packages_dir = "packages"
        self.cache_dir = ".pip_cache"
        self.start_time = time.time_ns()
        self.build_dir = self._create_build_directory()

    def _download_dependencies(self):
        current_dir = os.getcwd()

        req_file = os.path.join(current_dir, "requirements.txt")
        try:
            with open(req_file, "r") as f:
                requirements = f.read().splitlines()
        except Exception as e:
            logging.error(f"Error reading requirements file: {str(e)}")
            logging.info(
                "Please create a 'requirements.txt' in the same folder as your logic."
            )
            sys.exit(1)

        logging.info(f"Downloading the following packages: {requirements}")

        # Download all dependencies to the dependencies directory
        try:
            # trunk-ignore(bandit/B603)
            subprocess.check_call(
                [
                    sys.executable,
                    "-m",
                    "pip",
                    "download",
                    f"--platform={self.architecture}",
                    "--python-version 310",
                    "--only-binary=:all:",
                    "-d",
                    f"{self.build_dir}/{self.package_name}/{self.dependencies_dir}",
                ]
                + requirements
            )
        except subprocess.CalledProcessError as e:
            logging.error(f"Error downloading dependencies: {str(e)}")
            sys.exit(1)

    @staticmethod
    def _create_build_directory():
        current_time = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
        build_dir = os.path.join("socha_builds", current_time)
        os.makedirs(build_dir, exist_ok=True)
        return build_dir

    def _create_directory_structure(self):
        try:
            if not os.path.exists(f"{self.build_dir}/{self.package_name}"):
                logging.info(f"Creating directory {self.package_name}")
                os.mkdir(f"{self.build_dir}/{self.package_name}")

            logging.info(f"Creating directory {self.dependencies_dir}")
            if not os.path.exists(
                f"{self.build_dir}/{self.package_name}/{self.dependencies_dir}"
            ):
                os.mkdir(
                    f"{self.build_dir}/{self.package_name}/{self.dependencies_dir}"
                )

            logging.info(f"Creating directory {self.packages_dir}")
            if not os.path.exists(
                f"{self.build_dir}/{self.package_name}/{self.packages_dir}"
            ):
                os.mkdir(f"{self.build_dir}/{self.package_name}/{self.packages_dir}")

            logging.info(f"Creating directory {self.cache_dir}")
            if not os.path.exists(
                f"{self.build_dir}/{self.package_name}/{self.cache_dir}"
            ):
                os.mkdir(f"{self.build_dir}/{self.package_name}/{self.cache_dir}")

        except OSError as e:
            logging.error(f"Error creating directory: {e}")
            sys.exit(1)

    @staticmethod
    def _get_modules():
        # Get the directory of the main script
        frame = inspect.currentframe()
        while frame.f_back:
            frame = frame.f_back
        main_module = inspect.getmodule(frame)
        main_dir = os.path.dirname(os.path.abspath(main_module.__file__))

        # Get the set of module names that were imported in the main script
        # and are in the same directory or a subdirectory
        main_modules = set(sys.modules) - set(globals())
        if main_module:
            main_modules |= set(
                obj.__name__
                for obj in gc.get_objects()
                if isinstance(obj, types.ModuleType)
                and obj.__name__.startswith(main_module.__name__)
            )
        main_modules = {
            name
            for name in main_modules
            if hasattr(sys.modules[name], "__file__")
            and sys.modules[name].__file__ is not None
            and (
                os.path.abspath(os.path.dirname(sys.modules[name].__file__)) == main_dir
                or os.path.abspath(
                    os.path.dirname(sys.modules[name].__file__)
                ).startswith(main_dir + os.path.sep)
            )
        }

        module_paths = set()
        # Print the path of each module
        for name in main_modules:
            module = sys.modules[name]
            filepath = getattr(module, "__file__", None)
            module_paths.add(filepath)

        return module_paths

    def _copy_modules(self):
        """
        Recursively searches for the given python file in the current working directory and its subdirectories,
        and copies all python files with their directory structure to the target_folder.
        """
        logging.info(f"Copying python files to {self.package_name}")
        source_folder = os.getcwd()
        main_modules = self._get_modules()
        for root, dirs, files in os.walk(source_folder):
            if "socha_builds" in dirs:
                dirs.remove("socha_builds")
            for file in files:
                if file.endswith(".py"):
                    source_file_path = os.path.join(root, file)
                    target_file_path = os.path.join(
                        self.build_dir,
                        self.package_name,
                        os.path.relpath(source_file_path, source_folder),
                    )
                    if file in sys.argv[0]:
                        os.makedirs(os.path.dirname(target_file_path), exist_ok=True)
                        shutil.copy2(source_file_path, target_file_path)
                        logging.info(
                            f"Copying {source_file_path} to {target_file_path}"
                        )
                    if source_file_path in main_modules:
                        os.makedirs(os.path.dirname(target_file_path), exist_ok=True)
                        shutil.copy2(source_file_path, target_file_path)
                        logging.info(
                            f"Copying {source_file_path} to {target_file_path}"
                        )

    def _create_shell_script(self):
        logging.info(f"Creating shell script {self.package_name}/start.sh")
        with open(
            f"{self.build_dir}/{self.package_name}/start.sh", "w", newline="\n"
        ) as f:
            f.write("#!/bin/sh\n")
            f.write("\n")
            f.write("# Exit immediately if any command fails\n")
            f.write("set -e\n")
            f.write("\n")
            f.write(
                "# Sets the environment variable, which specifies the location for pip to store its cache files\n"
            )
            f.write(f"export XDG_CACHE_HOME=./{self.package_name}/.pip_cache\n")
            f.write("\n")
            f.write(
                "# Sets the environment variable, which adds the directory to the list of paths that Python searches "
                "for modules and packages when they are imported.\n"
            )
            f.write(f"export PYTHONPATH=./{self.package_name}/packages:$PYTHONPATH\n")
            f.write("\n")
            f.write("# Install the package socha and dependencies\n")
            f.write(
                f"pip install --no-index --find-links=./{self.package_name}/{self.dependencies_dir}/ --target=./"
                f"{self.package_name}/{self.packages_dir}/ --cache-dir=./{self.package_name}/{self.cache_dir} "
            )

            # Add all downloaded packages to the pip install command
            for package in os.listdir(
                f"{self.build_dir}/{self.package_name}/{self.dependencies_dir}"
            ):
                f.write(f"./{self.package_name}/{self.dependencies_dir}/{package} ")

            f.write("\n\n")
            f.write(
                f"# Run the {os.path.basename(sys.argv[0])} script with start arguments\n"
            )
            f.write(
                f'python3 ./{self.package_name}/{os.path.basename(sys.argv[0])} "$@"\n'
            )

    def _zipdir(self):
        logging.info(f"Zipping directory {self.package_name}")
        try:
            zipf = zipfile.ZipFile(
                f"{self.build_dir}/{self.package_name}.zip", "w", zipfile.ZIP_DEFLATED
            )
            for root, dirs, files in os.walk(f"{self.build_dir}/{self.package_name}"):
                for file in files:
                    file_path = os.path.join(root, file)
                    arc_name = os.path.relpath(file_path, self.build_dir)
                    zipf.write(file_path, arcname=arc_name)
                for _dir in dirs:
                    dir_path = os.path.join(root, _dir)
                    arc_name = os.path.relpath(dir_path, self.build_dir)
                    zipf.write(dir_path, arcname=arc_name)
            zipf.close()
            logging.info(f"{self.package_name}.zip successfully created!")
        except Exception as e:
            logging.error(f"Error creating {self.package_name}.zip: {str(e)}")
            sys.exit(1)

    def build_package(self):
        logging.info("Building package...")

        # Create the directory structure
        self._create_directory_structure()

        # Copy the scripts
        self._copy_modules()

        # Download all dependencies
        self._download_dependencies()

        # Create the shell script
        self._create_shell_script()

        # Zip the directory
        self._zipdir()

        # Log a success message
        logging.info(f"{self.package_name} package successfully built!")
