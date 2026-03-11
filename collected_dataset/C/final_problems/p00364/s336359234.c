#include<stdio.h>
int main(){
int a[1001],b[1001],c,d,i,j;
double x,y,z,w=0;
scanf("%d %d\n",&c,&d);
for(i=0;i<c;i++){
scanf("%d %d\n",&a[i],&b[i]);
}
for(i=0;i<c;i++){
x=a[i],y=b[i];
z=y/x;
if(w<z){
w=z;
}
}
printf("%lf\n",d*w);
return 0;
}
