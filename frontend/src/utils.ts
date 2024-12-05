import log from "./logger";

/**
 * @param date - a date object
* @param eod - if the EOD time should be used. Defaults to using the current time
 * @returns the date object in a format for the meetup API to consume, in the format "2024-11-10T19:04:01-04:00\[US/Eastern\]"
 */
export function dateToMeetupDate(date: NormalizedDate, eod = false): string {
   const year = date.getFullYear();
   const month = date.getMonth()
   const day = date.getDate()
   let time;

   if (eod) {
      time = `23:59:59-04:00[US/Eastern]`
   } else {
      time = `01:00:00-04:00[US/Eastern]`;
   }
   return `${year}-${month.toString().padStart(2, "0")}-${day.toString().padStart(2, "0")}T${time}`;
}

/**
 * Checks if a Date represents today's date
 *
 * @param date - a date to check
 * @returns if `date` represents today's date
 */
function isToday(date: NormalizedDate) {
   const now = new NormalizedDate();
   log.info(`Is today passed in date: ${date}`)
   // logger.info(
   //    `is today: ${date.getFullYear() === now.getFullYear() &&
   //    date.getMonth() === now.getMonth() &&
   //    date.getUTCDate() === now.getDate()
   //    }`,
   // );
   // logger.info(`Full year: ${date.getFullYear()} : ${now.getFullYear()}`);
   // logger.info(`month: ${date.getMonth()} : ${now.getMonth()}`);
   // logger.info(`date: ${date.getUTCDate()} : ${now.getDate()}`);
   return (
      date.getFullYear() === now.getFullYear() &&
      date.getMonth() === now.getMonth() &&
      date.getDate() === now.getDate()
   );
}

/**
 * A date class where month is 1 indexed instead of 0
 */
export class NormalizedDate extends Date {
   /**
    * Returns 1 base indexed month
    */
   getUTCMonth() {
      return super.getUTCMonth() + 1;
   }
   /**
    * Returns 1 base indexed month
    */
   getMonth() {
      return super.getUTCMonth() + 1;
   }
}
