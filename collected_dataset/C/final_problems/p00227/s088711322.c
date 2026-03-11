#include <stdio.h>

int main(){
  int i,j,n,m,work,p[10001],sum=0,k;
  
  while(1){
    scanf("%d %d",&n,&m);
    if(n==0&&m==0) break;
    
    for(i=0;i<n;i++){
      scanf("%d",&p[i]);
      sum+=p[i];
    }
    
    for(i=0;i<n;i++){
      for(j=0;j<n-1;j++){
	if(p[j]<p[j+1]){
	  work = p[j];
	  p[j] = p[j+1];
	  p[j+1] = work;
	}
      }
    }
    
    for(i=1,k=1;i<=n;i++){
      if(i==k*m){
	k++;
	sum-=p[i-1];
      }
    }
    
    printf("%d\n",sum);
    sum = 0;
  }
    
  

  return 0;
}