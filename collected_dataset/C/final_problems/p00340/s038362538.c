#include <stdio.h>
#include <stdlib.h>

int compar(const int *val1,const int *val2){
    return *val1-*val2;
}
int main(void){
    int map[4]={0},i,sign=0,sum=0;
    for(i=0;i<4;i++)
        scanf("%d",&map[i]);
    qsort(map,4,sizeof(int),(int(*)(const void *,const void *))compar);
    for(i=0;i<3;i++){
        if(map[i] != map[i+1]){
            sum++;
            sign=i;
        }
    }
    if((sign == 0 || sign == 1) && (sum == 0 || sum == 1))
        printf("yes\n");
    else
        printf("no\n");
    return 0;
}