import ast
import logging
import os
import shutil
import subprocess
import sys
import zipfile


class SochaPackageBuilder:

    def __init__(self, package_name):
        self.package_name = package_name
        self.dependencies_dir = 'dependencies'
        self.packages_dir = 'packages'
        self.cache_dir = '.pip_cache'

    def _download_dependencies(self):
        current_dir = os.getcwd()

        req_file = os.path.join(current_dir, 'requirements.txt')
        try:
            with open(req_file, 'r') as f:
                requirements = f.read().splitlines()
        except Exception as e:
            logging.error(f"Error reading requirements file: {str(e)}")
            sys.exit(1)

        logging.info(f'Downloading the following packages: {requirements}')

        # Download all dependencies to the dependencies directory
        try:
            subprocess.check_call(
                [sys.executable, '-m', 'pip', 'download', '--only-binary=:all:', '-d',
                 f'{self.package_name}/{self.dependencies_dir}'] + requirements)
        except subprocess.CalledProcessError as e:
            logging.error(f"Error downloading dependencies: {str(e)}")
            sys.exit(1)

    def _create_directory_structure(self):
        try:
            if not os.path.exists(self.package_name):
                logging.info(f'Creating directory {self.package_name}')
                os.mkdir(self.package_name)

            logging.info(f'Creating directory {self.dependencies_dir}')
            if not os.path.exists(f'{self.package_name}/{self.dependencies_dir}'):
                os.mkdir(f'{self.package_name}/{self.dependencies_dir}')

            logging.info(f'Creating directory {self.packages_dir}')
            if not os.path.exists(f'{self.package_name}/{self.packages_dir}'):
                os.mkdir(f'{self.package_name}/{self.packages_dir}')

            logging.info(f'Creating directory {self.cache_dir}')
            if not os.path.exists(f'{self.package_name}/{self.cache_dir}'):
                os.mkdir(f'{self.package_name}/{self.cache_dir}')

        except OSError as e:
            logging.error(f"Error creating directory: {e}")
            sys.exit(1)

    def _copy_scripts(self):
        """
        Recursively searches for the given python file in the current working directory and its subdirectories,
        and copies all python files with their directory structure to the target_folder.
        """
        logging.info(f'Copying python files to {self.package_name}')
        source_folder = os.getcwd()
        for root, dirs, files in os.walk(source_folder):
            for file in files:
                if file.endswith('.py'):
                    source_file_path = os.path.join(root, file)
                    target_file_path = os.path.join(self.package_name, os.path.relpath(source_file_path, source_folder))
                    os.makedirs(os.path.dirname(target_file_path), exist_ok=True)
                    shutil.copy2(source_file_path, target_file_path)
                    logging.info(f'Copying {source_file_path} to {target_file_path}')

    def _create_shell_script(self):
        logging.info(f'Creating shell script {self.package_name}/start.sh')
        with open(f'{self.package_name}/start.sh', 'w', newline='\n') as f:
            f.write('#!/bin/sh\n')
            f.write('\n')
            f.write('# Exit immediately if any command fails\n')
            f.write('set -e\n')
            f.write('\n')
            f.write(f'# Sets the environment variable, which specifies the location for pip to store its cache files\n')
            f.write(f'export XDG_CACHE_HOME=./{self.package_name}/.pip_cache\n')
            f.write('\n')
            f.write(
                f'# Sets the environment variable, which adds the directory to the list of paths that Python searches '
                f'for modules and packages when they are imported.\n')
            f.write(f'export PYTHONPATH=./{self.package_name}/packages:$PYTHONPATH\n')
            f.write('\n')
            f.write(f'# Install the package socha and dependencies\n')
            f.write(
                f'pip install --no-index --find-links=./{self.package_name}/{self.dependencies_dir}/ --target=./'
                f'{self.package_name}/{self.packages_dir}/ --cache-dir=./{self.package_name}/{self.cache_dir} ')

            # Add all downloaded packages to the pip install command
            for package in os.listdir(f'{self.package_name}/{self.dependencies_dir}'):
                f.write(f'./{self.package_name}/{self.dependencies_dir}/{package} ')

            f.write('\n\n')
            f.write(f'# Run the {os.path.basename(sys.argv[0])} script with start arguments\n')
            f.write(f'python3 ./{self.package_name}/{os.path.basename(sys.argv[0])} "$@"\n')

    def _zipdir(self):
        logging.info(f'Zipping directory {self.package_name}')
        try:
            zipf = zipfile.ZipFile(f'{self.package_name}.zip', 'w', zipfile.ZIP_DEFLATED)
            for root, dirs, files in os.walk(self.package_name):
                for file in files:
                    zipf.write(os.path.join(root, file))
                for _dir in dirs:
                    zipf.write(os.path.join(root, _dir))
            zipf.close()
            logging.info(f'{self.package_name}.zip successfully created!')
        except Exception as e:
            logging.error(f'Error creating {self.package_name}.zip: {str(e)}')
            sys.exit(1)

    def build_package(self):
        logging.info('Building package...')

        # Create the directory structure
        self._create_directory_structure()

        # Copy the scripts
        self._copy_scripts()

        # Download all dependencies
        self._download_dependencies()

        # Create the shell script
        self._create_shell_script()

        # Zip the directory
        self._zipdir()

        # Log a success message
        logging.info(f'{self.package_name} package successfully built!')
