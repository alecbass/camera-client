export class WebRTCServer {
  private connection: RTCPeerConnection;

  constructor() {
    this.connection = new RTCPeerConnection();
  }
}
