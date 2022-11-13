# skyline-react-template
This is a template/example application for skyline plugins using a React + TypeScript frontend! Uses `nx-request-api` and `nx-request-handler` to simplify backend calls, which can be expanded on when necessary.

The primary frontend file for the application is [App.tsx](https://github.com/techyCoder81/skyline-react-template/blob/main/web/src/App.tsx).

![react](https://user-images.githubusercontent.com/42820193/201546415-aa54216a-bc7a-4713-90c7-4fbef44ee196.png)

# Setup
First, ensure that you have `npm` installed. Then, in the `switch` directory, 
```bash
cargo skyline build --release
```
The plugin build process will run `npm install` and `npm build` automatically and package your web files. However, you can also navigate to the `web` directory and run commands such as `npm start` to streamline frontend development using a local web browser,and only rebuild the plugin when necessary.
