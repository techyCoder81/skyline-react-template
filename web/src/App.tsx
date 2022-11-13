import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import {DefaultMessenger} from "nx-request-api";

var messenger: DefaultMessenger;
try {
  messenger = new DefaultMessenger();
} catch {
  console.error("Messenger is not available!");
}

function App() {
  const [buttonInfo, setButtonInfo] = useState("");
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          This is a sample React application running on a skyline plugin!
        </p>
        <div className='inline'>
          <button className="sample-buttons" 
            onClick={async () => messenger.exitSession()}
            autoFocus
            onFocus={() => setButtonInfo("This will close the session and start smash.")}>
              Play Smash
          </button>
          <button className="sample-buttons" 
            onClick={async () => messenger.println("This is a statement from the frontend!")}
            onFocus={() => setButtonInfo("This will send a log statement to the skyline logger.")}>
              Send Log
          </button>
          <button className="sample-buttons" 
            onClick={async () => messenger.customRequest("custom1", null)}
            onFocus={() => setButtonInfo("This will invoke the 'custom1' backend handler.")}>
              Custom1
          </button>
          <button className="sample-buttons" 
            onClick={async () => messenger.exitApplication()}
            onFocus={() => setButtonInfo("This will close the application.")}>
              Exit Game
          </button>
        </div>
        <p>{buttonInfo}</p>
      </header>
    </div>
  );
}

export default App;
