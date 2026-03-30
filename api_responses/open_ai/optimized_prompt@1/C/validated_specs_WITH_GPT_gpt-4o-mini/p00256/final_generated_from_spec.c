#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BASE_YEAR 2001
#define MAX_INPUT_SIZE 20

// Function to check if a year is a leap year
int isLeapYear(int year) {
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

// Function to calculate the total number of days from the base date (01-01-2001)
int calculateDaysFromBase(int year, int month, int day) {
    int totalDays = 0;
    for (int y = BASE_YEAR; y < year; y++) {
        totalDays += isLeapYear(y) ? 366 : 365;
    }
    // Days in the current year
    int daysInMonth[] = {0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    for (int m = 1; m < month; m++) {
        totalDays += daysInMonth[m];
    }
    // Adjust for leap year if the month is after February
    if (month > 2 && isLeapYear(year)) {
        totalDays++;
    }
    totalDays += day;
    return totalDays;
}

// Function to convert total days back to year, month, day
void convertDaysToDate(int totalDays, int *year, int *month, int *day) {
    int currentYear = BASE_YEAR;
    while (totalDays >= (isLeapYear(currentYear) ? 366 : 365)) {
        totalDays -= (isLeapYear(currentYear) ? 366 : 365);
        currentYear++;
    }
    *year = currentYear;

    // Calculate month and day
    int daysInMonth[] = {0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    *month = 1;
    while (totalDays >= daysInMonth[*month]) {
        totalDays -= daysInMonth[*month];
        (*month)++;
    }
    // Adjust for leap year if the month is after February
    if (*month > 2 && isLeapYear(*year)) {
        if (totalDays == 28) {
            totalDays++;
        }
    }
    *day = totalDays + 1; // Days are 1-indexed
}

int main() {
    char input[MAX_INPUT_SIZE];
    fgets(input, sizeof(input), stdin);

    // Check for termination condition
    if (input[0] == '#') {
        return 0; // Terminate without processing
    }

    // Check if input is in string format
    if (strchr(input, '.') != NULL) {
        int year, month, day;
        sscanf(input, "%d.%d.%d", &year, &month, &day);
        int totalDays = calculateDaysFromBase(year, month, day);
        // Output in the format A.B.C.D.E
        printf("%d.%d.%d.%d.%d\n", totalDays, year, month, day, 0); // E is set to 0 as per spec
    } else {
        // Input is in numerical format
        int year, month, day, unused1, unused2;
        sscanf(input, "%d.%d.%d.%d.%d", &year, &month, &day, &unused1, &unused2);
        int totalDays = calculateDaysFromBase(year, month, day);
        convertDaysToDate(totalDays, &year, &month, &day);
        // Output in the format A.B.C.D.E
        printf("%d.%d.%d.%d.%d\n", totalDays, year, month, day, 0); // E is set to 0 as per spec
    }

    return 0;
}
