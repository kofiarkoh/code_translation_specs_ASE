#include <stdio.h>
#include <memory.h>
typedef long long ll;

#define mx 100001
#define MX 240

int n,dp[MX+1][mx];

int min(int a,int b){
    if(a>b) return b;
    return a;
}

int fn(int pos,int rem){
    if(rem==0 && pos){
        return 0;
    }
    if(pos==MX) return mx;
    int ans = dp[pos][rem];
    if(ans!=-1) return ans;
    ans = mx;

    for(int i = 0; i<=9; i++){
        if(pos==0 && i==0)  continue;
        ans = min(ans, i + fn(pos+1,(rem*10 + i)%n ) );
    }
    return dp[pos][rem] = ans;
}


int main(){
//read;
//IO;
scanf("%d",&n);
int p = n,q = 0;
while(p){
    q+=p%10;
    p/=10;
}
if(p==1){
    puts("1");
    return 0;
}
memset(dp,-1,sizeof dp);
int ans = fn(0,0);
printf("%d",ans);


return 0;
}
