#include<stdio.h>
#include<string.h>

#define SUNNY 'S'
#define RAINY 'R'

int main()
{
    char weather[4];
    int curret_rain_days = 0;
    int max_rain_days = 0;

    scanf("%s", weather);

    for (int i = 0; i<3; i++)
    {
        if (weather[i] == RAINY)
        {
            curret_rain_days ++;

            if (curret_rain_days > max_rain_days)
            {
                max_rain_days = curret_rain_days;
            }
        } else {
            curret_rain_days = 0;
        }
    }

    printf("%d", max_rain_days);

    return 0;
}