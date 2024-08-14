/**
 * Next page pagination button
 * @param nextPageCallback - callback for next page
 * @param disableNextBtn - if next page button should be disabled
 */
export default function Pagination(props: {
  nextPageCallback: (
    e: MouseEvent & {
      currentTarget: HTMLButtonElement;
      target: Element;
    }
  ) => void;
  disableNextBtn: boolean;
}) {
  return (
    <div>
      <button onClick={props.nextPageCallback} disabled={props.disableNextBtn}>
        Next
      </button>
    </div>
  );
}
