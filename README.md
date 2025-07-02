## TrainTrack

Source code for http://traintrack.nyc

Note: This is not an official Google project.

## Secrets Management

This project uses a `secrets.sh` file to manage API keys and other secrets. This file is not checked into version control. To get started, you will need to create your own `secrets.sh` file.

1.  Copy the `secrets.sh.template` file to `secrets.sh`:

    ```bash
    cp secrets.sh.template secrets.sh
    ```

2.  Edit the `secrets.sh` file and replace the placeholder values with your own secrets.

3.  The `frontend.sh` and `proxy.sh` scripts will automatically source the `secrets.sh` file and use the values you have provided.

## Running the Project

Before you can run the project, make sure you have set up your `secrets.sh` file as described in the "Secrets Management" section.

The application runs as two separate processes: the feed proxy and the frontend. You will need to run them in separate terminal windows.

1.  **Start the Feed Proxy:**

    In your first terminal, run the following command to start the feed proxy, which fetches real-time data from the MTA. You can optionally provide a port number (defaults to `3839`) using `--port`, and a directory for logs using `--log_dir` (defaults to the current directory):

    ```bash
    ./proxy.sh [--port=PORT] [--log_dir=LOG_DIR]
    ```

    Example:
    ```bash
    ./proxy.sh --port=8080 --log_dir=/tmp/traintrack_logs
    ```

2.  **Start the Frontend:**

    In your second terminal, run the following command to start the main web application. If you specified a custom port for the proxy, you must provide the same port number here using `--proxy_port`. You can also specify the frontend's listening port using `--port` (defaults to `3838`), and a directory for logs using `--log_dir` (defaults to the current directory):

    ```bash
    ./frontend.sh [--proxy_port=PROXY_PORT] [--port=FRONTEND_PORT] [--log_dir=LOG_DIR]
    ```

    Example:
    ```bash
    ./frontend.sh --proxy_port=8080 --port=8081 --log_dir=/tmp/traintrack_logs
    ```

Once both processes are running, you should be able to access the application in your web browser.
