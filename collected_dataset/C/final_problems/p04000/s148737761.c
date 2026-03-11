#include<stdio.h>
#include<math.h>
#include<stdlib.h>
long int a[900000][2];
int l(long int x1,long int x2,long int y1,long int y2){
  return x1==y1?x2<y2:x1<y1;
}
void q(long int s,long int t){
  long int b[2];
  long int i=s,j=t;
  b[0]=a[s][0];b[1]=a[s][1];
  if(s<t){
    while(1){
      while(l(a[i][0],a[i][1],b[0],b[1]))i++;
      while(l(b[0],b[1],a[j][0],a[j][1]))j--;
      if(i>=j)break;
      a[i][0]^=a[j][0]^=a[i][0]^=a[j][0];
      a[i][1]^=a[j][1]^=a[i][1]^=a[j][1];
      i++;j--;
    }
    q(s,i-1);q(j+1,t);
  }
}
int main(){
  long int h,w,n,n2=0,i,j,x,y,k,x2,y2;
  long int b[2],z[10];
  for(i=0;i<10;i++)z[i]=0;
  scanf("%ld %ld %ld",&h,&w,&n);
  for(i=0;i<n;i++){
    scanf("%ld %ld",&y,&x);
    for(j=0;j<3;j++)for(k=0;k<3;k++){
      x2=x-k;y2=y-j;
      if(x2>0&&x2<w-1&&y2>0&&y2<h-1){a[n2][0]=x2;a[n2][1]=y2;n2++;}
    }
  }
  q(0,n2-1);
  b[0]=a[0][0];b[1]=a[0][1];
  k=0;
  for(i=1;i<n2;i++){
    k++;
    if(a[i-1][0]!=a[i][0]||a[i-1][1]!=a[i][1]){z[k]++;k=0;}
  }
  if(n2)z[k+1]++;
  z[0]=(h-2)*(w-2);
  for(i=1;i<10;i++)z[0]-=z[i];
  for(i=0;i<10;i++)printf("%ld\n",z[i]);

}
