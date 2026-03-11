#include<stdio.h>
int f(const int d[30],int s){
  int min=s,c[30],r,z,i,k,l,ny,nx;
  int Y[]={0,1,1,1};
  int X[]={1,1,0,-1};//printf("%d\n",min);
  if(s==0)return 0;
  for(i=0;i<s;i++){
    for(k=0;k<4;k++){
      ny=i/4+Y[k];
      nx=i%4+X[k];
      if(4<ny||nx<0||3<nx||ny*4+nx>=s||d[i]-d[ny*4+nx])continue;
      for(l=z=0;l<s;l++){
	c[l-z]=d[l];
	if(l==i||l==ny*4+nx)z++;
      }//for(l=0;l<s-2;l++)printf("%d ",c[l]);printf("\n");//return 0;
      r=f(c,s-2);
      if(min>r)min=r;
      if(min==0)return 0;
    }
  }
  return min;
}
int main(){
  int n,d[30],i;
  scanf("%d",&n);
  while(n--){
    for(i=0;i<20;i++)scanf("%d",&d[i]);
    printf("%d\n",f(d,20));
  }
  return 0;
}