#include <stdio.h>

int main(void){
    int n,m,a,x,map[1000][3],i,h,o;
    for(scanf("%d %d %d",&n,&m,&a);a;scanf("%d %d %d",&n,&m,&a)){
        for(i=0;i<m;i++){
            for(o=0;o<3;o++){
                scanf("%d",&map[i][o]);
            }
        }
        for(h=n,x=a;h != 0;h--){
            for(i=0;i<m;i++){
                if(h == map[i][0] && x == map[i][1]){
                    x=map[i][2];
                    break;
                }
                else if(h == map[i][0] && x == map[i][2]){
                    x=map[i][1];
                    break;
                }
            }
        }
        printf("%d\n",x);
    }
    return 0;
}