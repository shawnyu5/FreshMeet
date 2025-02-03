import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { createEffect, createSignal, onMount } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { useSearchParams } from "@solidjs/router";
import { SearchBar } from "./SearchBar";
import log from "~/logger";

export default function () {
  const [searchParams, setSearchParams] = useSearchParams();
  // The selected dates of the date picker. The first date is the beginning, second is the end
  const [datePickerValue, setDatePickerValue] = createSignal<[Date, Date]>([
    new Date(),
    new Date(),
  ]);

  // Get dates from query param to set as initial value for picker.
  // This is so a refresh will not reset the search date
  onMount(() => {
    let startDateParam = searchParams.startDate as string;
    let endDateParam = searchParams.endDate as string;

    if (startDateParam || endDateParam) {
      log.info(`Found existing start / end date in query param:`);
      log.info(`Start date: ${startDateParam}`);
      log.info(`End date: ${endDateParam}`);
      setDatePickerValue([new Date(startDateParam), new Date(endDateParam)]);
    } else {
      log.info(
        "No query params found. Setting initial date picker value to current date:",
      );
      const startDate = new Date();
      const endDate = new Date();
      startDate.setHours(1, 0);
      endDate.setHours(23, 59);

      log.info(`Start date: ${startDate}`);
      log.info(`End date: ${endDate}`);
      setDatePickerValue([startDate, endDate]);
    }
  });

  // On every date picker selection, update the query param with the new selected value
  createEffect(() => {
    const [startDate, endDate] = datePickerValue();
    const startDateParam = startDate.toDateString();
    const endDateParam = endDate.toDateString();
    log.info(
      `Setting query params from date picker update: ${startDateParam}, ${endDateParam}`,
    );

    setSearchParams({
      startDate: startDateParam,
      endDate: endDateParam,
    });
  });

  return (
    <div class="top-bar">
      <style></style>
      <div class="top-bar-left">
        <ul class="dropdown menu" data-dropdown-menu>
          <img src="../icon.png" width="50" />
          <li class="menu-text">Fresh meet</li>
          {
            // TODO: Is this button really needed?
            // <li>
            //   <a href="/">Today</a>
            // </li>
          }
          <li>
            <DatePicker value={datePickerValue} setValue={setDatePickerValue} />
          </li>
        </ul>
      </div>

      <div class="top-bar-right">
        {
          // TODO: fix this. this component needs to accept the correct type as props
          <SearchBar />
        }
      </div>
    </div>
  );
}
