#include <stdio.h>
int schedule[101];

int main(void){
    int N,Q,i,j,M,date,max;
    while(scanf("%d %d",&N,&Q),N||Q){
        for(i=0;i<101;i++)schedule[i]=0;
        for(i=0;i<N;i++){
            scanf("%d",&M);
            for(j=0;j<M;j++){
                scanf("%d", &date);
                schedule[date]++;
                //printf("date:%d %d\n",date,schedule[date]);
            }
        }
        max=0;
        for(i=100;i>0;i--){
            if(max<=schedule[i]){
                max=schedule[i];
                date=i;
            }
        }
        if(max>=Q){
            printf("%d\n",date);
        }else{
            printf("%d\n",0);
        }
    }
    return 0;
}