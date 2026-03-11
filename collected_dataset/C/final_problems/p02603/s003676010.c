#include <stdio.h>
#include <stdlib.h>

int main(void){
    int n;
    int *a;

    int money = 1000;

    int i;



    scanf("%d", &n);


    a = (int*)malloc(sizeof(int) * n);

    for(i = 0; i < n; i++){
        scanf("%d", a + i);
    }


    for(i = 0; i < n - 1; i++){
        if(a[i] < a[i + 1]){
            int number = money / a[i];
            money += (a[i + 1] - a[i]) * number;

        }
    }
    printf("%d\n", money);


    free(a);

    return 0;
}
