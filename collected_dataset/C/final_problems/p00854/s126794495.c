#include <stdio.h>
#include <stdbool.h>

int n, k, m;
int i, j;
int tmp;
int game[20000] = {0};

int main(void){
    while(true){
        scanf("%d %d %d", &n, &k, &m);
        if(n == 0 && k == 0 && m == 0) break;
        for(i = 0; i < n; i++){
            game[i] = i + 1;
        }

        tmp = m - 1;
        while(true){
            if(n == 1) break;
            for(i = tmp; i < n - 1; i++){
                game[i] = game[i + 1];
            }
            n--;
            tmp = (tmp + k - 1) % n;
        }
        printf("%d\n", game[0]);
    }

    return 0;
}

