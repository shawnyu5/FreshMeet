import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { createEffect, createSignal, onMount } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { useSearchParams } from "@solidjs/router";
import { dateToMeetupDate } from "~/utils";
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
      log.info(`Found existing start / end date in query param`);
      startDateParam = startDateParam.slice(0, startDateParam.indexOf("["));
      endDateParam = endDateParam.slice(0, endDateParam.indexOf("["));
      setDatePickerValue([new Date(startDateParam), new Date(endDateParam)]);
    } else {
      setDatePickerValue([new Date(), new Date()]);
    }
  });

  // On every date picker selection, update the query param with the new selected value
  createEffect(() => {
    const [startDate, endDate] = datePickerValue();
    if (
      searchParams.startDate !== dateToMeetupDate(startDate, false) ||
      searchParams.endDate !== dateToMeetupDate(endDate, true)
    ) {
      log.info(
        `Setting query params: ${dateToMeetupDate(startDate, false)}, ${dateToMeetupDate(endDate, true)}`,
      );
      setSearchParams({
        startDate: dateToMeetupDate(startDate, false),
        endDate: dateToMeetupDate(endDate, true),
      });
    }
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
