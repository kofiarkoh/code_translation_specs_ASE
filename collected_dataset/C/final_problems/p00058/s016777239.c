#include<stdio.h>
int main(){
double a[8];
while(scanf("%lf %lf %lf %lf %lf %lf %lf %lf",&a[0],&a[1],&a[2],&a[3],&a[4],&a[5],&a[6],&a[7])!=EOF){
if((a[1]-a[3])*(a[5]-a[7])+(a[0]-a[2])*(a[4]-a[6])==0)
printf("YES\n");
else printf("NO\n");
}
return 0;
}