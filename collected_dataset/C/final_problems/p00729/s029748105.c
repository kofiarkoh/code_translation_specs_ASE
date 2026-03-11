#include<stdio.h>
 
struct data{
    int t,n,m,s;
}d[1001];
struct QUESTION{
    int s,e,m;//* start,end,m
}q[51];
 
int t[1261][10001];
 
int main(){
    int i,j,k,l,n,m,r,qq;
    int ans;
    while(scanf("%d%d",&n,&m),n+m){
        scanf("%d",&r);
        for(i = 0;i < r;i++)
            scanf("%d%d%d%d",&d[i].t,&d[i].n,&d[i].m,&d[i].s);
        scanf("%d",&qq);
        for(i = 0;i < qq;i++)
            scanf("%d%d%d",&q[i].s,&q[i].e,&q[i].m);
 
        for(i = 0;i < 1261;i++)for(j = 1;j <= m;j++)t[i][j] = 0;
 
        for(i = 0;i < r;i++){
            t[d[i].t][d[i].m] = d[i].s - (d[i].s == 0);
        }
 
        for(j = 1;j <= m;j++){
            for(i = 1;i < 1261;i++){
                t[i][j] += t[i-1][j];
            }
        }
 
        for(i = 0;i < qq;i++){
            ans = 0;
            for(j = q[i].s;j < q[i].e;j++){
                if(t[j][q[i].m])ans++;
            }
            printf("%d\n",ans);
        }
    }
    return 0;
}