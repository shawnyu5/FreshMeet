/**
 * pagination buttons
 * @param nextPageCallback - callback for next page
 * @param previousPageCallback - callback for previous page
 */
export default function Pagination(props: {
  nextPageCallback: (
    e: MouseEvent & {
      currentTarget: HTMLButtonElement;
      target: Element;
    }
  ) => void;
  previousPageCallback: (
    e: MouseEvent & {
      currentTarget: HTMLButtonElement;
      target: Element;
    }
  ) => void;
}) {
  return (
    <div>
      <button onClick={props.previousPageCallback}>Previous</button>
      <button onClick={props.nextPageCallback}>Next</button>
    </div>
  );
}
