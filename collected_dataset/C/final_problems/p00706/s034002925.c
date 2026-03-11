#include <stdio.h>
#include <memory.h>
int main(void){
    int N,W,H,i,x,y,map[100][100],S,T,o,sum,sum1,max,j;
    for(;sum=0,max=0,memset(map,0,sizeof(map)),scanf("%d",&N),N;){
        int W,H;
        scanf("%d %d",&W,&H);
        for(i=0;i<N;i++){
            scanf("%d %d",&x,&y);
            map[y][x]=1;
        }
        scanf("%d %d",&S,&T);
        for(i=1;i<=T;i++)
            for(o=1;o<=S;o++)
                sum+=map[i][o];
        sum1=max=sum;
        for(i=1;;max < sum ?(max=sum):(max=max)){
            for(o=2;o+S-1<=W;o++){
                for(j=0;j<T;j++){
                    sum-=map[i+j][o-1];
                    sum+=map[i+j][o+S-1];
                }
                if(max<sum)
                    max=sum;
            }
            i++;
            if(i+T-1> H)
                break;
            for(o=1,j=0;j<S;j++){
                sum1-=map[i-1][o+j];
                sum1+=map[i+T-1][o+j];
            }
            sum=sum1;
        }
        printf("%d\n",max);
    }
    return 0;
}