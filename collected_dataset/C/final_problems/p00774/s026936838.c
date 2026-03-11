#include <stdio.h>
#include <string.h>
int main(void){
    int n,mv[10][6],map[10][6],i,o,sign,sum=1,start,ans,t,r;
    for(;ans=0,memset(mv,0,sizeof(mv)),memset(map,0,sizeof(map)),scanf("%d",&n),n;){
        for(i=n-1;i>=0;i--)
            for(o=0;o<5;o++)
                scanf("%d",&map[i][o]);
        for(sign=1;memset(mv,0,sizeof(mv)),sum=1,sign;){
            sign=0;
            for(i=0;i<n;i++){
                for(o=0;o<5;o++){
                    if(map[i][o] != 0 && map[i][o] == map[i][o+1]){
                        if(sum == 1)
                            start=o;
                        sum++;
                    }
                    else if(sum >= 3){
                        ans+=map[i][o]*sum;
                        sign=1;
                        for(t=sum;t;t--)
                            map[i][start+t-1]=0;
                        sum=1;
                    }
                    else
                        sum=1;
                }

            }
            for(i=1;i<n;i++){
                for(o=0;o<5;o++){
                    if(map[i-1][o] == 0)
                        mv[i][o]=mv[i-1][o]+1;
                    else
                        mv[i][o]=mv[i-1][o];
                }
            } 
            for(i=1;i<n;i++){
                for(o=0;o<5;o++){
                    if(map[i][o] !=  0 && mv[i][o] != 0 ){
                        map[i-mv[i][o]][o]=map[i][o];
                        map[i][o]=0;
                    }
                }
            }
        }
        printf("%d\n",ans);
    }
    return 0;
}