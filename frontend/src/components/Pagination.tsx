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
  disableNextBtn: boolean;
  disablePrevBtn: boolean;
}) {
  return (
    <div>
      <button
        onClick={props.previousPageCallback}
        disabled={props.disablePrevBtn}
      >
        Previous
      </button>
      <button onClick={props.nextPageCallback} disabled={props.disableNextBtn}>
        Next
      </button>
    </div>
  );
}
