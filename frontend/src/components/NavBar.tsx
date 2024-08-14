import "./NavBar.css";

export default function () {
  return (
    <div class="top-bar">
      <style></style>
      <div class="top-bar-left">
        <ul class="dropdown menu" data-dropdown-menu>
          <img src="../icon.png" width="50" />
          <li class="menu-text">
            Fresh meat
          </li>
          <li>
            <a href="/">Today</a>
          </li>
          {
            // <li>
            //   <a href="#">Two</a>
            // </li>
          }
        </ul>
      </div>

      <div class="top-bar-right">
        <ul class="menu">
          <li>
            <input type="search" placeholder="Search"></input>
          </li>
          <li>
            <button type="button" class="button">
              Search
            </button>
          </li>
        </ul>
      </div>
    </div>
  );
}
