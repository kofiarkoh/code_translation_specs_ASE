#include <stdio.h>

int past[1024];
int p = 0;


int check_past(int x)
{
    int i;

    for(i = 0; i < p; i++){
        if(past[i] == x)
            return 1;
    }

    return 0;
}


int main(void)
{
    int q, n, i, j;

    scanf("%d", &q);

    for(i = 0; i < q; i++){
        int base, count;

        p = count = 0;

        scanf("%d", &n);

        while(1){
            int max = 0;

            if(n / 10 == 0)
                break;

            count++;

            for(base = 10; base < 1000000; base = base * 10){
                int quo, rem;
                quo = n / base;
                rem = n % base;

                if(quo * rem > max)
                    max = quo * rem;

            }

            if(check_past(max)){
                count = -1;
                break;
            }

            past[p++] = max;
            n = max;
        }

        printf("%d\n", count);
    }

    return 0;
}