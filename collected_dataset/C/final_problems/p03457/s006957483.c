#include <stdio.h>
#include <stdlib.h>

int cango(int t1, int t2, int x1, int x2, int y1, int y2){
    if((int)(abs(x2-x1)+abs(y2-y1))%2==0){
        if(t2-t1>=(int)(abs(x2-x1)+abs(y2-y1))&&(t2-t1)%2==0) return 1;
        else return 0;
    } else {
        if(t2-t1>=(int)(abs(x2-x1)+abs(y2-y1))&&(t2-t1)%2==1) return 1;
        else return 0;
    }
}

int main()
{
    int N;
    scanf("%d", &N);
    int t[N+1], x[N+1], y[N+1];
    int i;
    for(i=1;i<=N;i++) scanf("%d %d %d", &t[i], &x[i], &y[i]);
    
    int judge = 1;
    judge *= cango(0, t[1], 0, x[1], 0, y[1]);
    for(i=1;i<N;i++){
        judge *= cango(t[i], t[i+1], x[i], x[i+1], y[i], y[i+1]);
    }
    
    if(judge==1) printf("Yes");
    else printf("No");

    return 0;
}