import { For } from "solid-js";

/**
 * A Navbar component
 * @param props.sections An array of sections in the navbar
 */
export function Navbar(props: {
  sections: Array<{
    label: string;
    path: string;
  }>;
}) {
  /* TODO: idk why this hamburger menu doesnt work
   <a
     role="button"
     class="navbar-burger"
     aria-label="menu"
     aria-expanded="false"
     data-target="top-navbar"
   >
     <span aria-hidden="true"></span>
     <span aria-hidden="true"></span>
     <span aria-hidden="true"></span>
   </a>
   */
  return (
    <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="/">
          <img src="../icon.png" />
        </a>
      </div>
      <div id="top-navbar" class="navbar-menu">
        <div class="navbar-start">
          <For each={props.sections}>
            {(label, _) => (
              <a class="navbar-item" href={label.path}>
                {label.label}
              </a>
            )}
          </For>
        </div>
      </div>
    </nav>
  );
}
