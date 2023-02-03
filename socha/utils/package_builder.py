import logging
import os
import subprocess
import sys
import zipfile


class SochaPackageBuilder:

    def __init__(self, package_name):
        self.package_name = package_name
        self.dependencies_dir = 'dependencies'
        self.packages_dir = 'packages'
        self.cache_dir = '.pip_cache'

    def download_dependencies(self):
        current_dir = os.getcwd()

        # Try to find requirements.txt in the current directory if it not exists in the same directory as the script,
        # than quit the program with an error message
        req_file = os.path.join(current_dir, 'requirements.txt')
        try:
            with open(req_file, 'r') as f:
                requirements = f.read().splitlines()
        except Exception as e:
            logging.error(f"Error reading requirements file: {str(e)}")
            sys.exit(1)

        # Log the packages that will be downloaded
        logging.info(f'Downloading the following packages: {requirements}')

        # Download all dependencies to the dependencies directory
        try:
            subprocess.check_call(
                [sys.executable, '-m', 'pip', 'download', '--only-binary=:all:', '-d',
                 f'{self.package_name}/{self.dependencies_dir}'] + requirements)
        except subprocess.CalledProcessError as e:
            logging.error(f"Error downloading dependencies: {str(e)}")
            sys.exit(1)

    def create_directory_structure(self):
        try:
            # Create the package directory if it does not exist
            if not os.path.exists(self.package_name):
                logging.info(f'Creating directory {self.package_name}')
                os.mkdir(self.package_name)

            logging.info(f'Creating directory {self.dependencies_dir}')
            # Create the dependencies directory if it does not exist
            if not os.path.exists(f'{self.package_name}/{self.dependencies_dir}'):
                os.mkdir(f'{self.package_name}/{self.dependencies_dir}')

            logging.info(f'Creating directory {self.packages_dir}')
            # Create the packages directory if it does not exist
            if not os.path.exists(f'{self.package_name}/{self.packages_dir}'):
                os.mkdir(f'{self.package_name}/{self.packages_dir}')

            logging.info(f'Creating directory {self.cache_dir}')
            # Create the pip cache directory if it does not exist
            if not os.path.exists(f'{self.package_name}/{self.cache_dir}'):
                os.mkdir(f'{self.package_name}/{self.cache_dir}')

        except OSError as e:
            logging.error(f"Error creating directory: {e}")
            sys.exit(1)

    def create_shell_script(self):
        logging.info(f'Creating shell script {self.package_name}/start.sh')
        # Create the shell script
        with open(f'{self.package_name}/start.sh', 'w') as f:
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

    def zipdir(self):
        logging.info(f'Zipping directory {self.package_name}')
        try:
            zipf = zipfile.ZipFile(f'{self.package_name}.zip', 'w', zipfile.ZIP_DEFLATED)
            for root, dirs, files in os.walk(self.package_name):
                for file in files:
                    file_path = os.path.join(root, file)
                    # get the relative path of the file with respect to the parent directory
                    relative_path = os.path.relpath(file_path, self.package_name)
                    # add the file to the zip file with the relative path
                    zipf.write(file_path, arcname=relative_path)
                for _dir in dirs:
                    dir_path = os.path.join(root, _dir)
                    # get the relative path of the directory with respect to the parent directory
                    relative_path = os.path.relpath(dir_path, self.package_name)
                    # add the directory to the zip file with the relative path
                    zipf.write(dir_path, arcname=relative_path)
            zipf.close()
            logging.info(f'{self.package_name}.zip successfully created!')
        except Exception as e:
            logging.error(f'Error creating {self.package_name}.zip: {str(e)}')
            sys.exit(1)

    def build_package(self):
        logging.info('Building package...')

        # Create the directory structure
        self.create_directory_structure()

        # Download all dependencies
        self.download_dependencies()

        # Create the shell script
        self.create_shell_script()

        # Zip the directory
        self.zipdir()

        # Log a success message
        logging.info(f'{self.package_name} package successfully built!')
