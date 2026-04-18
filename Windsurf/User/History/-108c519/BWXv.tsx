import { Route, Routes } from "react-router-dom";
import Home from "./Home";
import Opener from "./Opener";
import Settings from "./Settings";

function ProjectRoutes() {
  return (
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path="/specification-opener" element={<Opener />} />
      <Route path="/settings" element={<Settings />} />
    </Routes>
  );
}

export default ProjectRoutes;
