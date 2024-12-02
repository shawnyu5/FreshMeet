import { PickerValue } from '@rnwonder/solid-date-picker'
import { expect, it, suite, test } from 'vitest'
import { pickerValueToDate } from './DatePicker'
import { NormalizedDate } from '~/utils'

suite("pickerValueToDate", () => {
   it("should return the correct date obj in the middle of the year", () => {
      let pickerValue: PickerValue = {
         label: "test",
         value: {
            startDateObject: {
               year: 2024,
               month: 10,
               day: 21
            },
            endDateObject: {
               year: 2024,
               month: 10,
               day: 21
            }
         }
      }

      let date = pickerValueToDate(pickerValue)
      let expectedDate = [new NormalizedDate(2024, 9, 21), new NormalizedDate(2024, 9, 21)]
      expect(date).toStrictEqual(expectedDate)
   })
   it("should return the correct date for the last day of the year", () => {
      let pickerValue: PickerValue = {
         label: "test",
         value: {
            startDateObject: {
               year: 2024,
               month: 12,
               day: 31
            },
            endDateObject: {
               year: 2024,
               month: 12,
               day: 31
            }
         }
      }

      let date = pickerValueToDate(pickerValue)
      let expectedDate = [new NormalizedDate(2024, 11, 31), new NormalizedDate(2024, 11, 31)]
      expect(date).toStrictEqual(expectedDate)
   })
})
