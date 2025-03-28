import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {

  async function readNFC() {
    try {
      const uid = await invoke('read_nfc');
      console.log('Card UID =>', uid);
    } catch (error) {
      console.error('Error reading NFC:', error);
    }
  }

  return (
    <main className="container">

      <div>
        <h1>Read NFC!</h1>

        <button id="read-nfc-btn" onClick={readNFC}>read NFC</button>
      </div>

    </main>
  );
}

export default App;
