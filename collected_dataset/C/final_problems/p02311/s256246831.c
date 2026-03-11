#include<stdio.h>
#include<math.h>
double x[4],y[4],eps=1e-6;
int f(int *a,int *b){
  int i;
  for(i=0;i<2;i++)b[i]-=a[i];
  double l=hypot(b[0],b[1]);
  double c=acos((a[2]-b[2])/l);//printf("%lf %lf\n",l,c);
  if(a[2]>l+b[2]+eps||b[2]>l+a[2]+eps)return 0;
  //printf("%lf " ,(b[0]*cos( c)-b[1]*sin( c))*a[2]/l+a[0]     );
  x[0]=(b[0]*cos( c)-b[1]*sin( c))*a[2]/l+a[0];
  //printf("%lf " ,(b[0]*sin( c)+b[1]*cos( c))*a[2]/l+a[1]     );
  y[0]=(b[0]*sin( c)+b[1]*cos( c))*a[2]/l+a[1];
  //printf("%lf " ,(b[0]*cos( c)-b[1]*sin( c))*b[2]/l+a[0]+b[0]);
  //printf("%lf\n",(b[0]*sin( c)+b[1]*cos( c))*b[2]/l+a[1]+b[1]);
  if(a[2]+eps>l+b[2]||b[2]+eps>l+a[2])return 1;
  //printf("%lf " ,(b[0]*cos(-c)-b[1]*sin(-c))*a[2]/l+a[0]     );
  x[1]=(b[0]*cos(-c)-b[1]*sin(-c))*a[2]/l+a[0];
  //printf("%lf " ,(b[0]*sin(-c)+b[1]*cos(-c))*a[2]/l+a[1]     );
  y[1]=(b[0]*sin(-c)+b[1]*cos(-c))*a[2]/l+a[1];
  //printf("%lf " ,(b[0]*cos(-c)-b[1]*sin(-c))*b[2]/l+a[0]+b[0]);
  //printf("%lf\n",(b[0]*sin(-c)+b[1]*cos(-c))*b[2]/l+a[1]+b[1]);
  if(l+eps<a[2]+b[2])return 2;
  c=acos((a[2]+b[2])/l);//printf("%lf %lf\n",l,c);
  //printf("%lf " ,(b[0]*cos( c)-b[1]*sin( c))*a[2]/l+a[0]     );
  x[2]=(b[0]*cos( c)-b[1]*sin( c))*a[2]/l+a[0];
  //printf("%lf " ,(b[0]*sin( c)+b[1]*cos( c))*a[2]/l+a[1]     );
  y[2]=(b[0]*sin( c)+b[1]*cos( c))*a[2]/l+a[1];
  //printf("%lf " ,(b[0]*cos( c)-b[1]*sin( c))*b[2]/l+a[0]+b[0]);
  //printf("%lf\n",(b[0]*sin( c)+b[1]*cos( c))*b[2]/l+a[1]+b[1]);
  if(l<a[2]+b[2]+eps)return 3;
  //printf("%lf " ,(b[0]*cos(-c)-b[1]*sin(-c))*a[2]/l+a[0]     );
  x[3]=(b[0]*cos(-c)-b[1]*sin(-c))*a[2]/l+a[0];
  //printf("%lf " ,(b[0]*sin(-c)+b[1]*cos(-c))*a[2]/l+a[1]     );
  y[3]=(b[0]*sin(-c)+b[1]*cos(-c))*a[2]/l+a[1];
  //printf("%lf " ,(b[0]*cos(-c)-b[1]*sin(-c))*b[2]/l+a[0]+b[0]);
  //printf("%lf\n",(b[0]*sin(-c)+b[1]*cos(-c))*b[2]/l+a[1]+b[1]);
  return 4;
}

int main(){
  int p[2][3],i,j,n,r[4];//xyr
  for(i=0;i<6;i++)scanf("%d",&p[i/3][i%3]);
  n=f(p[0],p[1]);
  for(i=0;i<n;i++){
    for(j=i;j;j--){
      if(x[r[j-1]]+eps<x[i]||(fabs(x[r[j-1]]-x[i])<eps&&y[r[j-1]]+eps<y[i]))break;
      r[j]=r[j-1];
    }
    r[j]=i;
  }
  for(i=0;i<n;i++)printf("%.10lf %.10lf\n",x[r[i]],y[r[i]]);
  return 0;
}

