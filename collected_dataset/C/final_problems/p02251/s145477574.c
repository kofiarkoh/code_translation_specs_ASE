#include <stdio.h>

int
main(void)
{
        int amount;
        int coin_25 = 0;
        int coin_10 = 0;
        int coin_5 = 0;
        int coin_1 = 0;

        scanf("%d", &amount);

        coin_25 = amount / 25;
        amount = amount % 25;
        coin_10 = amount / 10;
        amount = amount % 10;
        coin_5 = amount / 5;
        amount = amount % 5;
        coin_1 = amount;

        printf("%d\n", coin_25 + coin_10 + coin_5 + coin_1);

        return 0;
}

