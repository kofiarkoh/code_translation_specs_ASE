#include<stdio.h>
int main(){
  int n,m,t,r,a,b,i,j;
  int q[400001];
  while(scanf("%d %d",&n,&m),n||m){
    t=r=0;
    for(i=0;i<n;i++){
      scanf("%d %d",&a,&b);
      if(a==0){
	q[r++]=b;
	if(r-t>m)t++;
      }
      if(a==1){
	r--;
	for(j=b+t-1;j<r;j++)q[j]=q[j+1];
      }
      if(a==2)printf("%d\n",q[b+t-1]);
      if(a==3){
	for(j=t;q[j]!=b;j++);
	for(r--;j<r;j++)q[j]=q[j+1];
      }
      //for(j=t;j<r;j++)printf("%d",q[j]);printf("\n");
    }
    printf("end\n");
  }
  return 0;
}