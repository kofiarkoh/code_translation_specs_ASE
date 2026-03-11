#include <stdio.h>

int main()
{
    unsigned long n;
    unsigned long long T, total, tt, mint = -1;
    typedef struct
    {
        unsigned long long a;
        unsigned long long b;
        int c;
    } taiki;
    taiki time[200000];
    int i, j, k, mini, count;

    total = 0;
    tt = 0;

    scanf("%ld %lld", &n, &T);

    for (i = 0; i < n; i++)
    {
        scanf("%lld %lld", &time[i].a, &time[i].b);
        time[i].c = 0;
    }

    count = 0;
    for (tt = 1; tt < T + 0.5;)
    {
        mint = -1;
        for (i = 0; i < n; i++)
        {
            if (time[i].c != 1)
            {
                if (mint == -1)
                {
                    mint = time[i].a * tt + time[i].b;
                    mini = i;
                }
                else
                {
                    if (mint > time[i].a * tt + time[i].b)
                    {
                        mint = time[i].a * tt + time[i].b;
                        mini = i;
                    }
                }
            }
        }
        if(mint == -1){
            break;
        }
        if ((tt + mint) < T + 0.5)
        {
            time[mini].c = 1;
            count++;
        }
        tt += (mint + 1);

    }

    printf("%d\n", count);

    return 0;
}
