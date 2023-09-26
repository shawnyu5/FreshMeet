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
   document.addEventListener("DOMContentLoaded", () => {
      // Get all "navbar-burger" elements
      const navbarBurgers = Array.prototype.slice.call(
         document.querySelectorAll(".navbar-burger"),
         0
      );

      // __AUTO_GENERATED_PRINT_VAR_START__
      console.log("Navbar#(anon) navbarBurgers: %s", navbarBurgers); // __AUTO_GENERATED_PRINT_VAR_END__

      // Add a click event on each of them
      navbarBurgers.forEach((el) => {
         el.addEventListener("click", () => {
            // Get the target from the "data-target" attribute
            const target = el.dataset.target;
            const $target = document.getElementById(target);

            // Toggle the "is-active" class on both the "navbar-burger" and the "navbar-menu"
            el.classList.toggle("is-active");
            $target.classList.toggle("is-active");
         });
      });
   });

   return (
      <nav class="navbar" role="navigation" aria-label="main navigation">
         <div class="navbar-brand">
            <a class="navbar-item" href="/">
               <img src="../icon.png" />
            </a>
            <a
               role="button"
               class="navbar-burger"
               aria-label="menu"
               aria-expanded="false"
               data-target="topNavbar"
            >
               <span aria-hidden="true"></span>
               <span aria-hidden="true"></span>
               <span aria-hidden="true"></span>
            </a>
         </div>

         <div id="topNavbar" class="navbar-menu">
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
