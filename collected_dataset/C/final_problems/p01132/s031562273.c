#include <stdio.h>

int main(void)
{
    int first = 1;
    while(1){
        int pay;
        int coins[4];
        int val[4] = {10, 50, 100, 500};
        scanf("%d", &pay);
        if(pay == 0)
            break;

        int i;
        for(i = 0; i < 4; i++)
            scanf("%d", coins + i);
        if(first)
            first = 0;
        else
            printf("\n");

        int min_coins[4] = {0};
        int change_coins[4] = {0};
        int min = 100000;

        for(i = 0; i <= coins[0]; i++){
            int j;
            for(j = 0; j <= coins[1]; j++){
                int k;
                for(k = 0; k <= coins[2]; k++){
                    int l;
                    for(l = 0; l <= coins[3]; l++){
                        int money = 10 * i + 50 * j + 100 * k + 500 * l;
                        if(pay <= money){
                            int change = money - pay;
                            change_coins[3] = change / 500;
                            change %= 500;
                            change_coins[2] = change / 100;
                            change %= 100;
                            change_coins[1] = change / 50;
                            change %= 50;
                            change_coins[0] = change / 10;

                            if(change_coins[0] * i != 0 || change_coins[1] * j != 0 || change_coins[2] * k != 0 || change_coins[3] * l != 0)
                                continue;

                            int total = 0;
                            total += coins[0] - i + change_coins[0];
                            total += coins[1] - j + change_coins[1];
                            total += coins[2] - k + change_coins[2];
                            total += coins[3] - l + change_coins[3];
                            if(min > total){
                                min_coins[0] = i;
                                min_coins[1] = j;
                                min_coins[2] = k;
                                min_coins[3] = l;
                                min = total;
                            }
                        }
                    }
                }
            }
        }

        for(i = 0; i < 4; i++)
            if(min_coins[i] != 0)
                printf("%d %d\n", val[i], min_coins[i]);


    }
    return 0;
}