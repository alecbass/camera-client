import { useCallback, useEffect, useRef, useState } from "react";
import { PersistentWebSocket } from "api/websocket";
import { downloadFile } from "./utils";

interface Props {
  onData: (data: Blob) => void;
}

export function AudioRecorder({ onData }: Props) {
  const [isRecording, setIsRecording] = useState(false);
  const chunks = useRef<Blob[]>([]);
  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const mediaInterval = useRef<NodeJS.Timer | null>(null);

  const [recordingUrl, setRecordingUrl] = useState<string | null>(null);

  useEffect(() => {
    async function setup() {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder.current = new MediaRecorder(stream);

      mediaRecorder.current.ondataavailable = (e) => {
        chunks.current.push(e.data);
        onData(e.data);
      };

      mediaRecorder.current.onstop = () => {
        const blob = new Blob(chunks.current, { type: "audio/wav" });
        const file = new File([blob], "audio.wav");
        // downloadFile(file, file.name);
        chunks.current = [];

        const recordingUrl = URL.createObjectURL(file);
        setRecordingUrl(recordingUrl);

        // Play it lol
        // const audio = new Audio(recordingUrl);
        // audio.play();
      };
    }

    setup();

    return () => {};
  }, [onData]);

  const startRecording = useCallback(() => {
    setIsRecording(true);
    if (mediaRecorder.current) {
      mediaRecorder.current.start();
    }

    mediaInterval.current = setInterval(() => {
      if (mediaRecorder.current) {
        mediaRecorder.current.requestData();
      }
    }, 100);
  }, []);

  const stopRecording = useCallback(() => {
    setIsRecording(false);
    if (mediaRecorder.current) {
      mediaRecorder.current.stop();
    }

    if (mediaInterval.current) {
      clearInterval(mediaInterval.current);
      mediaInterval.current = null;
    }
  }, []);

  return (
    <div>
      Audio
      <button disabled={isRecording} onClick={startRecording}>
        Start
      </button>
      <button disabled={!isRecording} onClick={stopRecording}>
        Stop
      </button>
      {/* {recordingUrl && <audio src={recordingUrl} controls />} */}
    </div>
  );
}
