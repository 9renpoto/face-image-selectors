import { useState } from "preact/hooks";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [imageUrl, setImage] = useState("");

  const onClick = () =>
    invoke<number>("on_trigger").then((response) => {
      setImage(
        window.URL.createObjectURL(
          new Blob([new Uint8Array(response)], { type: "image/png" }),
        ),
      );
    });
  const onload = () => window.URL.revokeObjectURL(imageUrl);

  return (
      <>
        <img src={imageUrl} alt="" style="max-width: 100%;" onLoad={onload} />
        <button onClick={onClick}>
          画像取得
        </button>
      </>
  );
}

export default App;
