# Visualizing PlantUML Diagrams

This document provides instructions on how to visualize the PlantUML diagrams included in the SC2IR documentation.

## PlantUML Files

The following PlantUML files are available in the project:

- `high_level_architecture.puml`: High-level overview of the system architecture
- `trait_structure.puml`: Structure of traits and their implementations
- `conversion_process.puml`: Activity diagram for the conversion process
- `data_model.puml`: Class diagram for SC and IR data models

## Online Visualization

You can use the PlantUML online server to visualize these diagrams:

1. Visit [PlantUML Online Server](http://www.plantuml.com/plantuml/uml/)
2. Copy-paste the contents of a `.puml` file into the editor
3. The diagram will be rendered automatically

## Local Visualization

### Using VS Code Extension

1. Install the [PlantUML extension](https://marketplace.visualstudio.com/items?itemName=jebbs.plantuml) for VS Code
2. Open any `.puml` file in VS Code
3. Press `Alt+D` (or use the command "PlantUML: Preview Current Diagram") to preview the diagram

### Using the PlantUML JAR

1. Download the PlantUML JAR from [PlantUML website](http://plantuml.com/download)
2. Run the JAR with a `.puml` file as input:
   ```sh
   java -jar plantuml.jar high_level_architecture.puml
   ```
3. This will generate a PNG image in the same directory

### Using Graphviz (for more advanced layouts)

1. Install [Graphviz](https://graphviz.org/download/)
2. Use PlantUML with Graphviz for improved layouts:
   ```sh
   java -jar plantuml.jar -graphvizdot /path/to/dot high_level_architecture.puml
   ```

## Exporting Diagrams

Once you've visualized a diagram, you can export it to various formats:

- PNG (default)
- SVG
- PDF
- LaTeX

Using the PlantUML JAR:

```sh
# Export to SVG
java -jar plantuml.jar -tsvg high_level_architecture.puml

# Export to PDF
java -jar plantuml.jar -tpdf high_level_architecture.puml
```

## Embedding in Documentation

The diagrams are already embedded in the architecture documentation using Markdown code blocks with the `plantuml` language specifier:

````markdown
```plantuml
@startuml
...
@enduml
```
````

When rendered by documentation systems that support PlantUML, these code blocks will be converted to images.
