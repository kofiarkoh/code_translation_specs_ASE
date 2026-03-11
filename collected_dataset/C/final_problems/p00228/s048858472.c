#include <stdio.h>
#include <string.h>
int main(void) {
    int len,i,num,j;
    char seg[12][9]={"0111111","0000110","1011011","1001111","1100110","1101101","1101101","1111101","1111111","1101111"},dis[9];
    while (scanf("%d",&len)!=EOF&&len!=-1) {
        strcpy(dis,"0000000");
        for (i=0; i<len; i++) {
            scanf("%d",&num);
            for (j=0; j<7; j++) {
                if (dis[j]==seg[num][j]) {
                    printf("0");
                } else {
                    printf("1");
                    dis[j]=seg[num][j];
                }
            }
            printf("\n");
        }
    }
    return 0;
}