import { For } from "solid-js";

export function Navbar(props: {
  labels: Array<{
    label: string;
    path: string;
  }>;
}) {
  return (
    <nav class="navbar" role="navigation" aria-label="main navigation">
      <div class="navbar-brand">
        <a class="navbar-item" href="/">
          <img src="../../public/icon.png" />
        </a>
      </div>

      <div id="navbarBasicExample" class="navbar-menu">
        <div class="navbar-start">
          <For each={props.labels}>
            {(label, i) => (
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
