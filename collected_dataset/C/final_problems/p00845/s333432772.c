#include <stdio.h>
#include <math.h>

double sx[500], sy[500], sz[500];
double lx[50], ly[50], lz[50], angs[50];

double getArg(int i, int j){
    return acos((sx[i]*lx[j] + sy[i]*ly[j] + sz[i]*lz[j])
                 /(sqrt(sx[i]*sx[i]+sy[i]*sy[i]+sz[i]*sz[i])*
                   sqrt(lx[j]*lx[j]+ly[j]*ly[j]+lz[j]*lz[j])));
}

int main(){
    int n, m, i, j, ans;
    while(scanf("%d",&n),n){
        for(i=0;i<n;i++) scanf("%lf%lf%lf",sx+i,sy+i,sz+i);
        scanf("%d",&m);
        for(i=0;i<m;i++) scanf("%lf%lf%lf%lf",lx+i,ly+i,lz+i,angs+i);

        ans = 0;
        for(i=0;i<n;i++){
            for(j=0;j<m;j++){
                if( getArg(i,j) < angs[j] ){
                    ans++;
                    break;
                }
            }
        }

        printf("%d\n",ans);
    }
}