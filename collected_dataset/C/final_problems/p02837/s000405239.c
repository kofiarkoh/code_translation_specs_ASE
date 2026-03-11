#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>
int n,a[16],x[16][16],y[16][16],i,j;

int max(int a,int b){
  return (a>b)?a:b;
}

int counter(int x) {
    if(x == 0) return 0;
    return counter(x >> 1) + (x & 1);
}

void solve(){
  scanf("%d",&n );
  for(i=0;i<n;i++){
    scanf("%d",&a[i] );
    for (j = 0; j < a[i]; j++) {
      scanf("%d %d",&x[i][j],&y[i][j]);
    }
  }
  int ans = 0;
  for(int bits=1;bits<(1<<n);bits++){
    bool ok=true;
    for (i = 1; i <= n; i++) {
      if(!(bits&(1<<(i-1)))) continue;
      for(j=1;j<=a[i];j++){
        if(((bits>>(x[i][j]-1))&1)^y[i][j]) ok=false;
      }
    }
    if(ok) ans=max(ans,counter(bits));
  }
  printf("%d\n",ans-1 );
  return;
}

int main(){
  solve();
  return 0;
}
