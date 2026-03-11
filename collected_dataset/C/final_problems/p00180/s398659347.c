#include<stdio.h>
int main(){
  int n,m,a[11000],b[11000],c[11000],r[11000],i,j,k,s,t;
  while(scanf("%d %d",&n,&m),n||m){
    int p[110]={0};
    for(i=s=0;i<m;i++){
      scanf("%d %d %d",&a[i],&b[i],&c[i]);
      for(j=r[i]=i;j;j--){
	if(c[r[j-1]]>c[r[j]]){
	  t=r[j-1];
	  r[j-1]=r[j];
	  r[j]=t;
	}
      }
    }
    for(i=0;i<m;i++){//printf("%d\n",c[r[i]]);
      for(j=a[r[i]]+1;p[j];j=p[j]);
      for(k=b[r[i]]+1;p[k];k=p[k]);
      if(j-k){
	p[j]=k;
	s+=c[r[i]];
      }
    }
    printf("%d\n",s);
  }
  return 0;
}