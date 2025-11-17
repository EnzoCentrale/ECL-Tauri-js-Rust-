"""
Basic ManimGL Example
A simple scene demonstrating basic animations with ManimGL.
"""

from manimlib import *


class BasicScene(Scene):
    """A basic scene with text and shape animations."""
    
    def construct(self):
        # Create a text object
        title = Text("Welcome to ManimGL!")
        title.scale(1.5)
        self.play(Write(title))
        self.wait()
        
        # Transform the title
        self.play(title.animate.scale(0.7).to_edge(UP))
        self.wait()
        
        # Create a circle
        circle = Circle(radius=2, color=BLUE)
        self.play(ShowCreation(circle))
        self.wait()
        
        # Create a square
        square = Square(side_length=4, color=RED)
        self.play(Transform(circle, square))
        self.wait()
        
        # Fade out everything
        self.play(
            FadeOut(circle),
            FadeOut(title)
        )
        self.wait()


class MathExample(Scene):
    """A scene demonstrating mathematical equations."""
    
    def construct(self):
        # Create a mathematical equation
        equation = Tex(r"E = mc^2")
        equation.scale(2)
        self.play(Write(equation))
        self.wait()
        
        # Transform to another equation
        new_equation = Tex(r"\int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2}")
        self.play(Transform(equation, new_equation))
        self.wait(2)
        
        self.play(FadeOut(equation))
        self.wait()


class GraphExample(Scene):
    """A scene demonstrating graph plotting."""
    
    def construct(self):
        # Create axes
        axes = Axes(
            x_range=[-3, 3, 1],
            y_range=[-3, 3, 1],
            height=6,
            width=6,
        )
        axes.add_coordinate_labels()
        
        self.play(ShowCreation(axes))
        self.wait()
        
        # Plot a function
        graph = axes.get_graph(
            lambda x: x**2,
            color=YELLOW,
        )
        graph_label = axes.get_graph_label(graph, "f(x) = x^2")
        
        self.play(ShowCreation(graph))
        self.play(Write(graph_label))
        self.wait(2)
        
        # Cleanup
        self.play(
            FadeOut(axes),
            FadeOut(graph),
            FadeOut(graph_label),
        )
        self.wait()


if __name__ == "__main__":
    # You can run specific scenes directly
    # python basic_example.py BasicScene
    pass
