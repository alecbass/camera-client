import { useCallback, useEffect, useRef, useState } from "react";
import { downloadFile } from "./utils";

export function Video() {
  const videoRef = useRef<HTMLVideoElement | null>;

  const [isRecording, setIsRecording] = useState(false);
  const chunks = useRef<Blob[]>([]);
  const mediaRecorder = useRef<MediaRecorder | null>(null);

  useEffect(() => {
    async function setup() {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      mediaRecorder.current = new MediaRecorder(stream);

      mediaRecorder.current.ondataavailable = (e) => {
        chunks.current.push(e.data);
      };

      mediaRecorder.current.onstop = () => {
        const blob = new Blob(chunks.current, { type: "video/mp4" });
        console.debug("Stopped recording:", blob);
        const file = new File([blob], "video.mp4");
        downloadFile(file, file.name);
        chunks.current = [];
      };
    }

    setup();
  }, []);

  const startRecording = useCallback(() => {
    setIsRecording(true);
    if (mediaRecorder.current) {
      mediaRecorder.current.start();
    }
  }, []);

  const stopRecording = useCallback(() => {
    setIsRecording(false);
    if (mediaRecorder.current) {
      mediaRecorder.current.stop();
    }
  }, []);

  return (
    <div>
      Video
      <button disabled={isRecording} onClick={startRecording}>
        Start
      </button>
      <button disabled={!isRecording} onClick={stopRecording}>
        Stop
      </button>
      <video ref={videoRef} />
    </div>
  );
}
