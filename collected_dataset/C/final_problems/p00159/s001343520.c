#include<stdio.h>
int main(){
int n,a[1001],b[1001],c[1001],i,k=1001;
double x,y,z,d[1001],w=10000;
while(1){
scanf("%d\n",&n);
if(n==0){
break;
}
for(i=0;i<n;i++){
scanf("%d %d %d\n",&a[i],&b[i],&c[i]);
x=b[i],y=c[i];
x=x/100;
z=y/(x*x);
if(z>22){
z=z-22;
}
else{
z=22-z;
}
d[i]=z;
if((z<w) || (z==w && k>a[i])){
w=z;
k=a[i];
}
}
for(i=0;i<n;i++){
if(w==d[i]){
printf("%d\n",a[i]);
break;
}
}
w=10000,k=1001;
}
return 0;
}
