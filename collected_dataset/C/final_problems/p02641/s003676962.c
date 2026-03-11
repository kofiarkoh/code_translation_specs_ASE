#include<stdio.h>
#include<stdlib.h>
int cmp(const void *a, const void *b){return *(int*)a - *(int*)b;}
int min(int a, int b){return a < b ? a : b;}
int main(){
    int x, n;
    scanf("%d%d",&x,&n);
    if(n == 0){
        printf("%d\n",x);
        return 0;
    }
    int p[n];
    for(int i = 0;  i < n; i++)
        scanf("%d",&p[i]);
    int q[101];
    for(int i = 0; i <= 100; i++)
        q[i] = 0;
    qsort(p,n,4,cmp);
    for(int i = 0; i < n; i++)
        q[p[i]] = 1;
    int di = 11110;
    int ans = 0;
    if(x < p[0] || x > p[n - 1])
        ans = x;
    else{
        for(int i = 0; i < 102; i++){
            if(!q[i]){
                int d = abs(i - x);
                if(di > d){
                    di = d;
                    ans = i;
                }
            }
        }
    }
    printf("%d\n",ans);
    return 0;
}