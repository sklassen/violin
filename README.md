# Interactive Violin Plot Demo

This is an interactive web application that demonstrates the use of violin plots to visualize different types of time series data. The application is built with Svelte for the frontend and Rust compiled to WebAssembly (WASM) for the data generation and analysis.

## Features

*   **Interactive Controls:** Adjust the parameters of each distribution and see the plots update in real-time.
*   **Multiple Distributions:** The application supports the following distributions:
    *   Uniform
    *   Normal
    *   Skewed
    *   Bimodal
    *   Fractal (Fractional Brownian Motion)
*   **Multiple Plot Types:** For each distribution, the application displays:
    *   A violin plot of the raw data.
    *   A time series plot of the cumulative sum of the data.
    *   A point and figure chart of the cumulative sum.

## Running the Application

To run the application locally, you will need to have Node.js and Rust installed.

1.  **Install Dependencies:**
    ```bash
    npm install
    ```
2.  **Build the WebAssembly Module:**
    ```bash
    npm run wasm
    ```
3.  **Run the Development Server:**
    ```bash
    npm run dev
    ```
The application will be available at `http://localhost:5173`.