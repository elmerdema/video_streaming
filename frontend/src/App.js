import React from "react";
import VideoInput from "./VideoInput";
import "./styles.css";

export default function App() {
  return (
    <div className="App">
      <h1>Video upload</h1>
      <VideoInput width={400} height={300} />
    </div>
  );
}
