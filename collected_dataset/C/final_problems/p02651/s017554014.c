#include<stdio.h>
int main(){
  int t;
  scanf("%d",&t);
  while(t--){
    long long n,i,j,k,r=1,x,a[210],d[110],g[110]={};
    scanf("%lld",&n);
    for(i=0;i<n;i++)scanf("%lld",&d[i]);
    for(i=0;i<n;i++)scanf("%1lld",&a[i]);
    
    for(i=n;i--;){
      if(a[i]){
	for(j=0;j<64;j++){
	  if(d[i]&(1LL<<j)){
	    if(g[j]==0)goto END;
	    for(k=0;k<64;k++){
	      if(d[i]&(1LL<<k))continue;
	      if(g[j]==g[k])goto END;
	    }
	  }
	}
      }
      else{
	for(j=0;j<64;j++){
	  if(d[i]&(1LL<<j)){
	    x=g[j];
	    for(k=0;k<64;k++){
	      if(d[i]&(1LL<<k)){
		if(g[k]==x)g[k]=r;
	      }
	    }
	    r++;
	  }
	}
      }
      //for(j=0;j<4;j++)printf("%lld ",g[j]);printf("\n");
    }
    printf("0\n");
    continue;
  END:;
    printf("1\n");
  }
  return 0;
}
