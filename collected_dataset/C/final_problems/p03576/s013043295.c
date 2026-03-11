#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define max(p,q)((p)>(q)?(p):(q))
#define min(p,q)((p)>(q)?(q):(p))
int i,j,k,l,m;
int xa,xb,ya,yb;
int maxx,minx,maxy,miny;
int x,y;
int cnt;
int N,K;
int point[51][2];
long long int menseki;
long long int answer=5000000000000000000;


int main(){
    scanf("%d%d",&N,&K);
    //printf("%lld\n",answer);
    for (i=0;i<N;i++){
        scanf("%d%d",&x,&y);
        point[i][0]=x;
        point[i][1]=y;
    }
    /*
    for (i=0;i<N;i++){
        //printf("%d %d\n",point[i][0],point[i][1]);
    }
     */
    for (i=0;i<N;i++){
        for (j=i;j<N;j++){
            xa=point[i][0];
            xb=point[j][0];
            minx=min(xa,xb);
            maxx=max(xa,xb);
            for (k=0;k<N;k++){
                for (l=k;l<N;l++){
                    ya=point[k][1];
                    yb=point[l][1];
                    miny=min(ya,yb);
                    maxy=max(ya,yb);
                    cnt=0;
                    menseki=(long long int)(maxx-minx)*(maxy-miny);
                    //printf("%d %d %d %d %lld\n",maxx,minx,maxy,miny,menseki);
                    for (m=0;m<N;m++){
                        x=point[m][0];
                        y=point[m][1];
                        if ((minx<=x)&&(x<=maxx)&&(miny<=y)&&(y<=maxy)) cnt++;
                        
                    }
                    //printf("%d %d\n",cnt,K);
                    if (cnt>=K) answer=min(answer,menseki);
                    //printf("%lld\n",answer);
                }
            }
        }
    }
    printf("%lld",answer);
    return 0;
}
