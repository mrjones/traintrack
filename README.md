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

    In your first terminal, run the following command to start the feed proxy, which fetches real-time data from the MTA:

    ```bash
    ./proxy.sh
    ```

2.  **Start the Frontend:**

    In your second terminal, run the following command to start the main web application:

    ```bash
    ./frontend.sh
    ```

Once both processes are running, you should be able to access the application in your web browser.
