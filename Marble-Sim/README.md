# Marble-Sim

Marble-Sim is a marble simulator based on OpenGL that demonstrates concepts of 3D graphics, basic lighting, and physics. It utilizes GLFW for window management and input handling, GLEW for OpenGL extensions loading, and OpenGL for rendering.
Features

- Marble Rendering: Simulates the visual representation of marbles in 3D.
- Basic Lighting: Implements simple lighting effects for the marbles.
- Shaders: Uses GLSL shaders for visual effects.
- Basic Interaction: Handles user input for interacting with the simulation.

## Requirements

Before building the project, ensure you have the following dependencies installed on your system:

- GLFW: A library for window management and input handling.
- GLEW: A library for loading OpenGL extensions.
- OpenGL: API for 3D rendering.

### Debian (and derivatives)
you can install these dependencies with the following commands:
```
sudo apt-get update
sudo apt-get install libglfw3-dev libglew-dev libgl1-mesa-dev
```
## Installation and Build
Clone the repository:
```
git clone https://github.com/your-username/Marble-Sim.git
cd Marble-Sim
```
Create a build directory and navigate into it:
```
mkdir -p build
cd build
```
Configure the project with CMake and build the project:
```
cmake ..
make
```
Run the simulator:
```
./Marble-Sim
```
## Usage

When you run the simulator, a window will open displaying the 3D marble simulation. Use the keyboard to interact with the simulation. Currently, interactive features are limited, but you can extend them based on your needs.
Project Structure

- src/: Contains the source code files (.cpp).
- CMakeLists.txt: CMake configuration file for building the project.

## Contributing
If you want to contribute to Marble-Sim, please fork the repository and submit a pull request with your changes. For major changes, please open an issue first to discuss what you would like to change.
