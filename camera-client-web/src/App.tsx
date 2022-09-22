import React, { useCallback, useMemo, useState } from "react";
import logo from "./logo.svg";
import "./App.css";

import { PersistentWebSocket } from "api/websocket";
import { AudioRecorder } from "components/Audio";

const API_URL = "ws://127.0.0.1:8000";

function createAudioUrl(data: Blob) {
  const blob = new Blob([data], { type: "audio/wav" });
  const url = URL.createObjectURL(blob);
  return url;
}

function App() {
  const [audioUrl, setAudioUrl] = useState<string | null>(null);

  const handleWsReceive = useCallback(async (data: Blob) => {
    // setAudioUrl(createAudioUrl(data));
    const peerAddresses = (await data.text()).split("\n");
    console.debug(peerAddresses);
    for (const address of peerAddresses) {
      // https://levelup.gitconnected.com/what-powers-google-meet-and-microsoft-teams-webrtc-demystified-step-by-step-tutorial-e0cb422010f7
      new RTCPeerConnection();
    }
  }, []);

  const websocket = useMemo(() => {
    const websocket = new PersistentWebSocket(API_URL, {
      onMessage: handleWsReceive,
    });
    return websocket;
  }, []);

  const handleStream = useCallback(
    async (data: Blob) => {
      const buffer = await data.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      console.debug(`Sending ${bytes.length}`);
      // TODO: Send this data via RTC
      // websocket.send(bytes);
    },
    [websocket]
  );

  return (
    <div className="App">
      <AudioRecorder onData={handleStream} />
      {audioUrl && (
        <audio preload="auto" controls>
          <source src={audioUrl} type="audio/wav" />
        </audio>
      )}
      <button onClick={() => websocket.send("hi!!!")}>Send data</button>
    </div>
  );
}

export default App;
