1. **Install `socat`**: If not already installed, you can install `socat` on most Linux distributions using the package manager. For example, on Ubuntu or Debian-based systems, you can use:

   ```bash
   sudo apt-get update
   sudo apt-get install socat
   ```

2. **Create a Virtual Serial Port Pair**: Use `socat` to create a pair of linked virtual serial ports. For example:

   ```bash
   socat -d -d pty,raw,echo=0 pty,raw,echo=0
   ```

   This command will output something like:

   ```
   2023/01/01 12:34:56 socat[12345] N PTY is /dev/pts/2
   2023/01/01 12:34:56 socat[12345] N PTY is /dev/pts/3
   2023/01/01 12:34:56 socat[12345] N starting data transfer loop with FDs [5,5] and [7,7]
   ```

   Here, `/dev/pts/2` and `/dev/pts/3` are the ends of your virtual serial port pair. Your application would connect to one end (e.g., `/dev/pts/3`), and you can write data to the other end (e.g., `/dev/pts/2`) to simulate the GPS device.

3. **Simulate GPS Data**: You can now simulate GPS data by writing to one of the virtual serial ports. A simple way to send a "Hello World" message with a timestamp to the port is by using a bash loop. For example, to write to `/dev/pts/2`:

   ```bash
   while true; do
     echo "Hello World $(date)" > /dev/pts/2
     sleep 1
   done
   ```

This setup will continuously send a "Hello World" message followed by the current timestamp to the virtual serial port every second. Your application, connected to the other end of the virtual serial port, should be able to read these messages, allowing you to test your serial communication handling without needing a real GPS device.

This method is convenient for testing and development purposes, as it doesn't require any custom coding beyond your application logic and can be easily set up and torn down.