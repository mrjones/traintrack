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
