#include<stdio.h>
#include<string.h>
#include<math.h>

#define DECIMAL_POINT 9
#define UPPER_DECIMAL 0
#define UNDER_DECIMAL -1


long long int array[200001][2];

int main()
{
    int n = 0;
    long long int t;
    int d_p, d_c;
    long long int mul = 0;
    long long int answer = 0;
    char num_arrray[20];

    long long int OFFSET = 1;

    for (int i = 0; i<9; i++)
    {
        OFFSET *= 10;
    }

    scanf("%d", &n);

    for (int i = 0; i<n; i++)
    {
        scanf("%s", num_arrray);

        d_p = DECIMAL_POINT;
        d_c = UPPER_DECIMAL;
        t = 0;

        for (int j = 0; j<strlen(num_arrray); j++)
        {
            int current_num = 0;

            if (num_arrray[j] == '.')
            {
                array[i][0] = t;
                d_c = UNDER_DECIMAL;
                t = 0;
                continue;
            }

            current_num = num_arrray[j] - '0';

            if (d_c == UPPER_DECIMAL)
            {
                t = t*10 + current_num;
            } else {
                t = t*10 + current_num;
                d_p += UNDER_DECIMAL;
            }

        }

        if (d_c == UPPER_DECIMAL)
        {
            array[i][0] = t;
            array[i][1] = 0;
        } else {
            for (int k = 0; k<d_p; k++)
            {
                t *= 10;
            }

            array[i][1] = t;

        }

    }

    for (int i = 0; i<(n - 1); i++)
    {
        for (int j = (i + 1); j<n; j++)
        {
            mul = array[i][1]*array[j][1];
            if ((mul%OFFSET) == 0)
            {
                mul = mul/OFFSET;
                mul += array[i][0]*array[j][1] + array[i][1]*array[j][0];
                mul = mul%OFFSET;
                if (mul == 0)
                    answer ++;
            }
        }
    }

    printf("%lld", answer);

    return 0;
}