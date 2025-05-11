import { JSX } from "solid-js";

export function PaginationButton(props: {
  text: string;
  onClick: () =>
    | JSX.EventHandlerUnion<
        HTMLAnchorElement,
        MouseEvent,
        JSX.EventHandler<HTMLAnchorElement, MouseEvent>
      >
    | undefined;
}) {
  return (
    <a class="button" onClick={props.onClick()}>
      {props.text}
    </a>
  );
}
