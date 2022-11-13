# skyline-react-template
This is a template/example application for skyline plugins using a React + TypeScript frontend! Uses `nx-request-api` and `nx-request-handler` to simplify backend calls, which can be expanded on when necessary.

The primary frontend file for the application is [App.tsx](https://github.com/techyCoder81/skyline-react-template/blob/main/web/src/App.tsx).

![react](https://user-images.githubusercontent.com/42820193/201547733-cc50bd44-5f44-489a-be71-ead80adbc9ae.png)

# Setup
First, ensure that you have `npm` installed. Then, in the `switch` directory, 
```bash
cargo skyline build --release
```
The plugin build process will run `npm install` and `npm build` automatically and package your web files. Currently, this will be rebuilt every time the plugin is built. However, you can also navigate to the `web` directory and run commands such as `npm start` to streamline frontend development using a local web browser,and only rebuild the plugin when necessary.
