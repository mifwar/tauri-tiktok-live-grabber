import { listen } from "@tauri-apps/api/event";
import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [isStarted, setIsStarted] = useState(false);
  const [message, setMessage] = useState("");
  const ref = useRef<HTMLInputElement>(null);

  const handleConnect = () => {
    setIsStarted(true);
    invoke("start_thread", { username: ref.current?.value });
  };

  const handleDisconnect = () => {
    //TODO: create invoke stop_thread
    setIsStarted(false);
  };

  useEffect(() => {
    const unlisten = listen<string>("tiktok-live-event", (event) => {
      console.log("Received event:", event.payload);
      setMessage(JSON.stringify(event.payload));
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="flex flex-col gap-4 p-4">
      <h1>Welcome to Tauri Tiktok Live Grabber!</h1>
      <div className="flex flex-row w-full gap-4 items-center justify-center mx-auto">
        <input
          id="username"
          ref={ref}
          placeholder="Enter a tiktok username..."
          className="flex-grow"
        />
        {!isStarted && (
          <button
            disabled={ref.current?.value === ""}
            className={ref.current?.value === "" ? "cursor-not-allowed" : ""}
            onClick={handleConnect}
          >
            {"Connect"}
          </button>
        )}
        {isStarted && (
          <button onClick={handleDisconnect}>{"Disconnect"}</button>
        )}
      </div>
      <p className="text-center">
        {message ? message : "Live comment will be shown here"}
      </p>
    </div>
  );
}

export default App;
