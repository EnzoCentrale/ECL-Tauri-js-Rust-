# ManimGL Examples

This directory contains example scripts demonstrating the usage of ManimGL for creating mathematical animations.

## Running Examples

Make sure you have installed the dependencies first (see [PYTHON_SETUP.md](../PYTHON_SETUP.md)).

### Basic Example

The `basic_example.py` file contains three scenes:

1. **BasicScene**: Demonstrates basic text and shape animations
   ```bash
   manimgl examples/basic_example.py BasicScene
   ```

2. **MathExample**: Shows mathematical equation rendering
   ```bash
   manimgl examples/basic_example.py MathExample
   ```

3. **GraphExample**: Plots a mathematical function
   ```bash
   manimgl examples/basic_example.py GraphExample
   ```

## Creating Your Own Animations

To create a new animation:

1. Create a new Python file in this directory
2. Import from manimlib: `from manimlib import *`
3. Create a class that inherits from `Scene`
4. Implement the `construct` method with your animation logic
5. Run it with: `manimgl your_file.py YourSceneClass`

## Resources

- [ManimGL Documentation](https://3b1b.github.io/manim/)
- [ManimGL GitHub Repository](https://github.com/3b1b/manim)
- [Video tutorials by 3Blue1Brown](https://www.youtube.com/c/3blue1brown)
