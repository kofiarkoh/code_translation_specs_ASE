#include<stdio.h>
int main(){
  int n,m,i,j,k,u,o;
  int h[1001],r[1001],p[2001];
  while(1){
    h[0]=-1;
    r[0]=-1;
    p[0]=0;

    scanf("%d",&n);
    if(n==0){break;}

    for(i=1;i<=n;i++){
      scanf("%d%d",&h[i],&r[i]);
    }
    scanf("%d",&m);
    for(j=i;j<=n+m;j++){  
      scanf("%d%d",&h[j],&r[j]);
    }
    
    /*hツづーsort*/
    for(i=1;i<n+m;i++){
      for(j=i+1;j<=n+m;j++){
	if(h[i]>h[j]){
	  u=h[i];
	  h[i]=h[j];
	  h[j]=u;
	  o=r[i];
	  r[i]=r[j];
	  r[j]=o; 
	}
      }
  }

  /*LIS*/
    for(i=1;i<=n+m;i++){
      p[i]=0;
      for(j=i;j>=0;j--){
	if(r[i]>r[j] && p[i]<=p[j])
	  {
	    p[i]=p[j]+1;  
	  }  
      }
    }
    
    k=0;
    for(i=1;i<=n+m;i++){
      if(k<p[i]){
	k=p[i];
      }
    }
    printf("%d\n",k);
	
  }

  return 0;
}