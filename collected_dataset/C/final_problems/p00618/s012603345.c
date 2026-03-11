#include<stdio.h>

int cnt(int n){
  int ret = 0;
  while(n){
    ret++;
    n ^= (n & (-n));
  }
  return ret;
}

int main(){
  int c[20],k[20],r[20][20];
  int n,u;
  int i,j,l;
  while(scanf("%d%d",&n,&u), n+u){
    int ans = n;
    for(i=0; i<n; i++){
      scanf("%d%d",&c[i],&k[i]);
      for(j=0; j<k[i]; j++)
	scanf("%d",&r[i][j]);
    }
    for(i=0; i<(1<<n); i++){
      int tni = 0;
      int ok  = 1;
      int cc  = cnt(i);
      if(cc >= ans) continue;
      for(j=0; ok && j<n; j++)
	if(i & (1<<j))
	  tni += c[j];
      if(tni < u) continue;
      for(j=0; ok && j<n; j++)
	if(i & (1<<j))
	  for(l=0; ok && l<k[j]; l++)
	    if(!(i & (1<<r[j][l])))
	      ok = 0;

      if(ok)
	ans = cc;
    }
    printf("%d\n",ans);
  }
}