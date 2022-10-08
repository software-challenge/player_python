from setuptools import setup

with open('requirements.txt') as f:
    required = f.read().splitlines()

setup(
    name='socha',
    version='0.9.6',
    packages=['socha', 'socha.api', 'socha.api.plugin', 'socha.api.protocol',
              'socha.api.networking'],
    url='https://github.com/FalconsSky/Software-Challenge-Python-Client',
    license='GNU Lesser General Public License v3 (LGPLv3)',
    author='FalconsSky',
    author_email='stu222782@mail.uni-kiel.de',
    description='This is the package for the Software-Challenge Germany 2023. This Season the game will be \'Hey, '
                'danke für den Fisch\' a.k.a. \'Penguins\' in short. ',
    install_requires=required
)
