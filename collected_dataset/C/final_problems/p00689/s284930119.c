#include<stdio.h>
#include<math.h>
int main(){
  int n,i,j,ax,ay,bx,by,a,b,c,d,x[400],y[400],mi;
  double s,max,m;
  while(scanf("%d",&n),n){
    for(i=ax=by=bx=0;i<n;i++)scanf("%d %d",&x[i],&y[i]);
    ay=-1;
    int f[400]={0};
    for(i=s=0;i<n;i++){
      for(j=max=0;j<n;j++){
	if(f[j])continue;
	a=bx-ax;
	b=by-ay;
	c=x[j]-bx;
	d=y[j]-by;
	m=(a*c+b*d)/hypot(a,b)/hypot(c,d);
	if(a*d>b*c)m=1-m;
	else       m+=3;
	if(max<m){
	  max=m;
	  mi=j;
	}
      }
      f[mi]=1;
      ax=bx;
      ay=by;
      bx=x[mi];
      by=y[mi];//printf("%d %d\n",bx,by);
      s+=hypot(bx-ax,by-ay);  
    }
    printf("%.1f\n",s);
  }
  return 0;
}