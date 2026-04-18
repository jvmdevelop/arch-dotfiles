import { Link } from "react-router-dom";

function Navigation() {
  return (
    <nav className="navigation">
      <Link to="/">Storage</Link>
      <Link to="/specification-opener">Opener</Link>
      <Link to="/settings">Settings</Link>
    </nav>
  );
}

export default Navigation;
