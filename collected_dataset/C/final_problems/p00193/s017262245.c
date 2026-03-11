#include<stdio.h>
int main(){
  int m[101][201]={0};
  int c[101][201]={0};
  int n,k,a,b,t,r,max,i;
  int x[10000],y[10000];;
  int X[]={ 1, 2, 1,-1,-2,-1};
  int Y[]={ 1, 0,-1,-1, 0, 1};
  while(scanf("%d %d",&n,&k),n||k){
    scanf("%d",&a);
    for(r=0;r<a;r++){
      scanf("%d %d",&x[r],&y[r]);
      x[r]=x[r]*2-y[r]%2;
      m[y[r]][x[r]]=1;
    }
    for(t=0;r-t;t++){
      for(i=0;i<6;i++){
	if(x[t]+X[i]<1||k*2<=x[t]+X[i]||
	   y[t]+Y[i]<1||n  <=y[t]+Y[i])continue;
	if(m[y[t]+Y[i]][x[t]+X[i]]==0||
	   m[y[t]+Y[i]][x[t]+X[i]]>m[y[t]][x[t]]+1){
	  m[y[t]+Y[i]][x[t]+X[i]]=m[y[t]][x[t]]+1;
	  x[r  ]=x[t]+X[i];
	  y[r++]=y[t]+Y[i];
	}
      }
    }
    /*for(i=0;i<=n+1;i++){
      for(a=0;a<=k*2+1;a++)printf("%2d",m[i][a]);
      printf("\n");
      }*/
    scanf("%d",&a);
    max=0;
    while(a--){
      scanf("%d %d",&x[t=0],&y[0]);
      c[y[t]][x[t]=x[t]*2-y[t]%2]=1;
      for(r=1;r-t;t++){
	for(i=0;i<6;i++){
	  if(x[t]+X[i]<1||k*2<x[t]+X[i]||
	     y[t]+Y[i]<1||n  <y[t]+Y[i])continue;
	  if(m[y[t]+Y[i]][x[t]+X[i]]>c[y[t]][x[t]]+1){
	    c[y[t]+Y[i]][x[t]+X[i]]=c[y[t]][x[t]]+1;
	    x[r  ]=x[t]+X[i];
	    y[r++]=y[t]+Y[i];
	  }
	}
      }
      if(max<r)max=r;
    }
    printf("%d\n",max);
  }
  return 0;
}