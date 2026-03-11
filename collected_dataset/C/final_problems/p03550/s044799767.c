#include <stdio.h>
#include <stdlib.h>

#define MIN(a,b) ((a)<(b))?(a):(b)
#define MAX(a,b) ((a)>(b))?(a):(b)

int N, Z, W;
int a[2000];
int s[2][2000];
int INF=2000000000;

int searchScore(int teban, int i,int x, int y){
    //printf("teban=%d, i=%d, x=%d, y=%d ",teban,i,x,y);
    if(i==N)return abs(x-y);
    //printf("a");
    if(s[teban][i]>=0)return s[teban][i];
    //printf("b");
    int j;
    int ret;
    int score;
    if(teban){
        ret=INF;
        for(j=i+1;j<=N;j++){
            //printf("c1");
            //printf("y%d\n",a[j-1]);
            score=searchScore(0,j,x,a[j-1]);
            ret=MIN(ret,score);
            //printf("d1");
        }
    }else{
        ret=0;
        for(j=i+1;j<=N;j++){
            //printf("c2");
            //printf("x%d\n",a[j-1]);
            score=searchScore(1,j,a[j-1],y);
            ret=MAX(ret,score);
            //printf("d2");
        }
    }
    //printf("ret=%d\n",ret);
    return s[teban][i]=ret;
}

int main()
{
    scanf("%d %d %d",&N,&Z,&W);
    int i,j;
    for(i=0;i<N;i++)scanf("%d",&a[i]);

    for(i=0;i<2;i++)for(j=0;j<N;j++)s[i][j]=-1;
    int ans=searchScore(0,0,Z,W);
    printf("%d\n",ans);

    return 0;
}
