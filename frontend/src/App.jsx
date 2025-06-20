// import React, { useState } from 'react';
// import { invoke } from '@tauri-apps/api/core';

// function App() {
//   const [logContent, setLogContent] = useState('');
//   const [fileName, setFileName] = useState('');
//   const [statusMessage, setStatusMessage] = useState('');

//   const handleFileChange = async (event) => {
//     const file = event.target.files[0];
//     if (!file) return;

//     const reader = new FileReader();
//     reader.onload = async () => {
//       const contents = reader.result;
//       setLogContent(contents);
//       setFileName(file.name);

//       try {
//         const result = await invoke('process_log', {
//           fileContent: contents,
//           fileName: file.name,
//         });
//         setStatusMessage(`✅ ${result}`);
//       } catch (error) {
//         setStatusMessage(`❌ Error: ${error}`);
//       }
//     };
//     reader.readAsText(file);
//   };

//   return (
//     <div className="p-6 max-w-3xl mx-auto">
//       <h1 className="text-2xl font-bold mb-4">Log File Viewer</h1>
//       <input
//         type="file"
//         accept=".log,.txt"
//         onChange={handleFileChange}
//         className="mb-4"
//       />
//       {fileName && (
//         <div className="mb-2 font-medium text-gray-700">Selected File: {fileName}</div>
//       )}
//       {statusMessage && (
//         <div className="mb-2 font-semibold text-sm text-blue-700">{statusMessage}</div>
//       )}
//       <textarea
//         className="w-full h-96 p-4 border border-gray-300 rounded-md"
//         value={logContent}
//         readOnly
//       />
//     </div>
//   );
// }

// export default App;



import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

export default function App() {
  const [logFilePath, setLogFilePath] = useState("");
  const [status, setStatus] = useState("");

  const handleSelectLogFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "Log Files", extensions: ["log"] }]
      });

      if (selected) {
        setLogFilePath(selected);
        setStatus("File selected: " + selected);
      }
    } catch (error) {
      setStatus("Error selecting file: " + error);
    }
  };

  const handleRunParser = async () => {
    if (!logFilePath) {
      setStatus("No log file selected.");
      return;
    }

    try {
      setStatus("Running parser...");
      const result = await invoke("run_parser", { filePath: logFilePath });
      setStatus("Parser output: " + result);
    } catch (error) {
      setStatus("Parser error: " + error);
    }
  };

  return (
    <div style={{ padding: "2rem" }}>
      <h1>Telemetry Log Parser</h1>
      <button onClick={handleSelectLogFile}>Select Log File</button>
      <button onClick={handleRunParser} style={{ marginLeft: "1rem" }}>
        Run Parser
      </button>
      <p>{status}</p>
    </div>
  );
}
