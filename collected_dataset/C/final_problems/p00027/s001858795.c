#include <stdio.h>
 
int dayOfWeek(int year, int month, int day)
{
    if (month == 1 || month == 2) {
        year--;
        month += 12;
    }
    return (year + year / 4 - year / 100 + year / 400
        + (13 * month + 8) / 5 + day) % 7;
}
 
int main()
{
    int m, d;
    char *day[7] = {
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"
    };
     
    while (1) {
        scanf("%d%d", &m, &d);
        if (m == 0 || d == 0) break;
        puts(day[dayOfWeek(2004, m, d)]);
    }
    return 0;
}

