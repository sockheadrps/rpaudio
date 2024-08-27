## Development Setup

To contribute to this project, you'll need to set up your development environment and ensure you can build and test the project locally. Follow the instructions below to get started.

### Prerequisites

- **Python**: Make sure Python 3.8 is installed on your system. You can download it from [python.org](https://www.python.org/downloads/).
- **Rust**: Install Rust using [rustup](https://rustup.rs/). This is required for building the Rust components of the project.
- **Python Package Manager**: Ensure you have `pip` installed. This is typically included with Python.

### Setting Up the Environment

1. **Fork the Repository**:
   - Go to the repository on GitHub and click the **Fork** button in the top right corner. This will create a copy of the repository in your own GitHub account.

2. **Clone Your Fork**:
   - Clone the repository from your fork to your local machine:
     ```bash
     git clone https://github.com/your-username/repository.git
     cd repository
     ```

3. **Create and Activate a Virtual Environment**:
   - Create a virtual environment:
     ```bash
     python -m venv .venv
     ```
   - Activate the virtual environment:
     - **Windows**:
       ```bash
       .\.venv\Scripts\activate
       ```
     - **Linux/macOS**:
       ```bash
       source .venv/bin/activate
       ```

4. **Install Python Dependencies**:
   - Install the required Python packages:
     ```bash
     pip install -r requirements.txt
     ```

5. **Install Rust Toolchain**:
   - Ensure you have the required Rust toolchain installed. You can use [Rustup](https://www.rust-lang.org/tools/install) to install the stable toolchain:
   - Install Rust dependencies
```bash
cd src
cargo fetch
```


6. **Build the Project**:
   - If you need to build the Rust components manually, you can use the following command:
```bash
maturin develop
```

7. **Verify Installation**:
   - To ensure that the Python package can be imported correctly, install the built wheel locally and test it:
```bash
python -c "import rpaudio"
```
If the package is installed correctly, this command will complete without any output or errors. Otherwise it will error something like:
```
ModuleNotFoundError: No module named 'rpaudio'
```

### Making Changes and Testing

1. **Fork the Repository**:
   - Go to the repository on GitHub and click the **Fork** button in the top right corner. This creates a copy of the repository in your own GitHub account.

2. **Clone Your Fork**:
   - Clone the repository from your fork to your local machine:
```bash
git clone https://github.com/your-username/repository.git
cd repository
```

3. **Checkout the Experimental Branch**:
   - Create and switch to a new branch for your changes based on the `experimental` branch in your fork:
```bash
git fetch origin
git checkout -b feature/your-feature origin/experimental
```

4. **Make Your Changes**:
   - Edit or add files as needed for your feature or fix.

5. **Build and Test Locally**:
   - Build the project and run tests to ensure everything works as expected. This may include running specific build commands and test scripts relevant to your project.

6. **Push Your Changes to Your Fork**:
   - Push your feature branch to your fork:
```bash
git add .
git commit -m "Describe your changes"
git push origin feature/your-feature
```

7. **Create a feature Branch**:
   - Create a test branch in your fork to test the changes before creating a pull request:
```bash
git checkout -b feature/your-feature
git push origin feature/your-feature
```


9. **Merge into Experimental Branch**:
   - Merge your feature branch into the `experimental` branch in your fork to run the test workflows to build the python ab3i wheels. This is done so you can test your code locally before ensuring it builds with the workflows, which can sometimes take a few minutes.
     ```bash
     git checkout experimental
     git merge feature/your-feature
     git push origin experimental
     ```
     [Please also update the docs with the changes you made.](docs\source)

10. **Create a Pull Request**:
    - If your builds all succeed open a pull request from your `experimental` branch in your fork to the `experimental` branch of the original repository on GitHub. Provide a clear description of your changes and any relevant details. 

    - To do this:
      - Go to the original repository on GitHub.
      - Click on **Pull Requests**.
      - Click the **New Pull Request** button.
      - Set the base branch to `experimental` and the compare branch to `experimental` in your fork.
      - Review your changes and submit the pull request.

### Additional Notes

- **Dependencies**: If your changes require additional dependencies, update the `requirements.txt` or relevant configuration files and ensure they are included in your pull request.
- **Documentation**: Update any relevant documentation to reflect your changes.
- **Workflow**: The GitHub Actions workflow will automatically build and test the project. Ensure your changes do not break the build or tests before submitting the pull request.

For any issues or questions, feel free to open an issue or ask for help in the project's discussions.