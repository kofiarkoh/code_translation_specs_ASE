#include <stdio.h>
#include <memory.h>
int main(void){
    int sign,num,q,map[101],i,o,temp,n;
    for(;sign=0,scanf("%d %d",&num,&q),num;){
        memset(map,0,sizeof(map));
        for(i=0;i<num;i++){
            scanf("%d",&n);
            for(o=0;o<n;o++){
                scanf("%d",&temp);
                map[temp-1]++;
                if(map[temp-1] == q && sign == 0){
                    sign=temp;
                }
            }
        }
        printf("%d\n",sign);
    }
    return 0;
}