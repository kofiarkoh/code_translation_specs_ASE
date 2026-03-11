#include <stdio.h>

int main(void)
{
    int a, b;
    int i;

    while (~scanf("%d %d", &a, &b)){
        int edge[128] = {0};
        int flag = 0;

        do {
            edge[a]++;
            edge[b]++;
            scanf("%d %d", &a, &b);
        } while (a);

        for (i = 3; i < 128; i++){
            flag |= edge[i] % 2;
        }
        puts(flag ? "NG" : "OK");
    }

    return 0;
}