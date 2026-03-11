#include <stdio.h>

int e[101];

int main(void)
{
    int e1, e2, e3, e4;
    int i;

    scanf("%d%d%d%d", &e1, &e2, &e3, &e4);
    
    e[e1]++;
    e[e2]++;
    e[e3]++;
    e[e4]++;

    for (i = 1; i <= 100; i++){
        if (e[i] != 0 && e[i] != 2 && e[i] != 4){
            printf("no\n");
            return (0);
        }
    }

    printf("yes\n");
    return (0);
}
    

